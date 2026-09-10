//! Counter-free lifecycle phases over the qualified general matching adapters.
#[allow(dead_code)]
#[path = "support/local_ports.rs"]
mod local;
#[cfg(feature = "alloc-meter")]
#[allow(dead_code, unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
#[path = "support/multihead_source.rs"]
mod source;
use chr_syntax::{Answer, Query, Rule};
use std::sync::Arc;
trait Backend {
    type State;
    fn prepare(r: &[Rule]) -> Self;
    fn setup(&self, q: &Query) -> Self::State;
    fn advance(s: &mut Self::State) -> bool;
    fn observe(s: &mut Self::State) -> Option<Answer>;
}
struct Local<const MODE: usize>(Arc<local::multihead::Program>);
impl<const MODE: usize> Backend for Local<MODE> {
    type State = local::multihead::Execution<false>;
    fn prepare(r: &[Rule]) -> Self {
        Self(local::multihead::Program::compile(r).unwrap())
    }
    fn setup(&self, q: &Query) -> Self::State {
        if MODE == 2 {
            self.0.start_partial(q)
        } else {
            self.0.start_mode(q, MODE == 1)
        }
    }
    fn advance(s: &mut Self::State) -> bool {
        s.advance()
    }
    fn observe(s: &mut Self::State) -> Option<Answer> {
        s.answer()
    }
}
struct Compiled<const INDEX: bool, const SPECIAL: bool>(chr_compiled::PreparedRuleset);
impl<const INDEX: bool, const SPECIAL: bool> Backend for Compiled<INDEX, SPECIAL> {
    type State = chr_compiled::Engine;
    fn prepare(r: &[Rule]) -> Self {
        let p = chr_compiled::PreparedRuleset::new(r.to_vec(), None).unwrap();
        Self(if SPECIAL { p.specialize_inferred() } else { p })
    }
    fn setup(&self, q: &Query) -> Self::State {
        self.0
            .start(
                q.clone(),
                chr_compiled::Policy::Global,
                if INDEX {
                    chr_compiled::Access::Indexed
                } else {
                    chr_compiled::Access::Scan
                },
            )
            .unwrap()
    }
    fn advance(s: &mut Self::State) -> bool {
        s.advance(1).exhausted
    }
    fn observe(s: &mut Self::State) -> Option<Answer> {
        s.observe()
    }
}
fn finish<B: Backend>(s: &mut B::State) {
    for _ in 0..200_000 {
        if B::advance(s) {
            return;
        }
    }
    panic!("unfinished service bound");
}
fn same(a: &Option<Answer>, expected: &[Answer]) {
    assert_eq!(usize::from(a.is_some()), expected.len());
    if let Some(a) = a {
        assert!(chr_observe::equivalent(
            a,
            &expected[0],
            &mut Default::default()
        ));
    }
}
#[derive(Clone, Copy)]
struct Phase {
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    heap: meter::Reading,
}
fn measure<T>(f: impl FnOnce() -> T) -> (T, Phase) {
    #[cfg(feature = "alloc-meter")]
    let start = meter::begin();
    let clock = std::time::Instant::now();
    let value = std::hint::black_box(f());
    let ns = clock.elapsed().as_nanos();
    #[cfg(feature = "alloc-meter")]
    let heap = meter::end(start);
    (
        value,
        Phase {
            ns,
            #[cfg(feature = "alloc-meter")]
            heap,
        },
    )
}
impl Phase {
    fn json(self) -> String {
        #[cfg(feature = "alloc-meter")]
        return format!("{{\"ns\":{},\"memory\":{}}}", self.ns, self.heap.json());
        #[cfg(not(feature = "alloc-meter"))]
        format!("{{\"ns\":{}}}", self.ns)
    }
}
fn run<B: Backend>(mode: &str, family: &str, width: usize, reuse: usize) {
    let rules = source::rules(family);
    let expected = (0..reuse)
        .map(|i| scalar::run(&rules, &source::query(family, width, i), 200_000))
        .collect::<Vec<_>>();
    let warm = B::prepare(&rules);
    for (i, expected) in expected.iter().enumerate() {
        let mut s = warm.setup(&source::query(family, width, i));
        finish::<B>(&mut s);
        same(&B::observe(&mut s), expected);
    }
    drop(warm);
    drop(rules);
    let mut phases = Vec::with_capacity(4 + reuse * 7 + 10);
    let (rules, source_build) = measure(|| source::rules(family));
    phases.push(("source-build", source_build));
    let (p, prep) = measure(|| B::prepare(&rules));
    phases.push(("prepare", prep));
    for (i, expected) in expected.iter().enumerate() {
        let (q, m) = measure(|| source::query(family, width, i));
        phases.push(("input", m));
        let (mut s, m) = measure(|| p.setup(&q));
        phases.push(("setup", m));
        let (_, m) = measure(|| finish::<B>(&mut s));
        phases.push(("execute", m));
        let (answer, m) = measure(|| B::observe(&mut s));
        phases.push(("observe", m));
        same(&answer, expected);
        let (_, m) = measure(|| drop(s));
        phases.push(("engine-drop", m));
        let (_, m) = measure(|| drop(answer));
        phases.push(("answer-drop", m));
        let (_, m) = measure(|| drop(q));
        phases.push(("input-drop", m));
        #[cfg(feature = "alloc-meter")]
        assert_eq!(m.heap.live_end, prep.heap.live_end);
    }
    for advance in [false, true] {
        let (q, m) = measure(|| source::query(family, width, 0));
        phases.push(("cancel-input", m));
        let (mut s, m) = measure(|| p.setup(&q));
        phases.push(("cancel-setup", m));
        let (_, m) = measure(|| {
            if advance {
                std::hint::black_box(B::advance(&mut s));
            }
        });
        phases.push(("cancel-advance", m));
        let (_, m) = measure(|| drop(s));
        phases.push(("cancel-engine-drop", m));
        let (_, m) = measure(|| drop(q));
        phases.push(("cancel-input-drop", m));
        #[cfg(feature = "alloc-meter")]
        assert_eq!(m.heap.live_end, prep.heap.live_end);
    }
    let (_, m) = measure(|| drop(p));
    phases.push(("prepared-drop", m));
    #[cfg(feature = "alloc-meter")]
    assert_eq!(m.heap.live_end, prep.heap.live_start);
    let (_, m) = measure(|| drop(rules));
    phases.push(("source-drop", m));
    #[cfg(feature = "alloc-meter")]
    assert_eq!(m.heap.live_end, source_build.heap.live_start);
    #[cfg(feature = "alloc-meter")]
    assert!(
        phases
            .windows(2)
            .all(|xs| xs[0].1.heap.live_end == xs[1].1.heap.live_start)
    );
    let records = phases
        .iter()
        .map(|(name, m)| format!("{{\"phase\":\"{name}\",\"measurement\":{}}}", m.json()))
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"width\":{width},\"reuse\":{reuse},\"state_bytes\":{},\"phases\":[{records}]}}",
        std::mem::size_of::<B::State>()
    );
}
fn smoke<B: Backend>(family: &str) {
    let rules = source::rules(family);
    let q = source::query(family, 4, 0);
    let expected = scalar::run(&rules, &q, 200_000);
    let p = B::prepare(&rules);
    let mut s = p.setup(&q);
    finish::<B>(&mut s);
    same(&B::observe(&mut s), &expected);
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.as_slice() == ["clock-check"] {
        let mut samples = Vec::with_capacity(10000);
        for _ in 0..10000 {
            let (_, p) = measure(|| std::hint::black_box(()));
            samples.push(p.ns);
        }
        samples.sort();
        println!(
            "{{\"samples\":10000,\"min_ns\":{},\"median_ns\":{},\"p99_ns\":{}}}",
            samples[0], samples[5000], samples[9900]
        );
        return;
    }
    if args.is_empty() {
        for family in ["sparse", "broad", "nested", "cold", "dense", "three"] {
            smoke::<Local<0>>(family);
            smoke::<Local<1>>(family);
            smoke::<Local<2>>(family);
            smoke::<Compiled<false, false>>(family);
            smoke::<Compiled<true, false>>(family);
            smoke::<Compiled<false, true>>(family);
            smoke::<Compiled<true, true>>(family);
        }
        println!("42 independent source smoke comparisons passed");
        return;
    }
    assert!(
        !std::hint::black_box(
            cfg!(feature = "local-work")
                || cfg!(feature = "compiled-work")
                || chr_compiled::COLLECT_METRICS
                || chr_compiled::COLLECT_KERNEL_METRICS
                || chr_persistent::COLLECT_METRICS
                || chr_persistent::COLLECT_KERNEL_METRICS
                || chr_observe::COLLECT_METRICS
        ),
        "engine and kernel counters forbidden in lifecycle measurements"
    );
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    assert_eq!(args.len(), 4);
    let (mode, family) = (&args[0], &args[1]);
    let width = args[2].parse().unwrap();
    let reuse = args[3].parse().unwrap();
    assert!(["sparse", "broad", "nested", "cold", "dense", "three"].contains(&family.as_str()));
    assert!([4, 16, 64].contains(&width) && [1, 4].contains(&reuse));
    match mode.as_str() {
        "local-scan" => run::<Local<0>>(mode, family, width, reuse),
        "tuples" => run::<Local<1>>(mode, family, width, reuse),
        "partial" => run::<Local<2>>(mode, family, width, reuse),
        "scan" => run::<Compiled<false, false>>(mode, family, width, reuse),
        "indexed" => run::<Compiled<true, false>>(mode, family, width, reuse),
        "special-scan" => run::<Compiled<false, true>>(mode, family, width, reuse),
        "special-indexed" => run::<Compiled<true, true>>(mode, family, width, reuse),
        _ => panic!("unknown mode"),
    }
}
