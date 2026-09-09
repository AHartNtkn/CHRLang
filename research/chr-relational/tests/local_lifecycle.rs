//! Isolated lifecycle sizing. Static adapters avoid boxing or a shared state layout.
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
use chr_syntax::{Answer, Query, Rule, Var, and, atom, c, eq, t, v};
use std::{hint::black_box, sync::Arc, time::Instant};
const WORK: bool = cfg!(feature = "local-work");
#[derive(Clone, Copy)]
struct Phase {
    ns: u128,
    cpu_ns: u128,
    #[cfg(feature = "alloc-meter")]
    heap: meter::Reading,
}
// The registered runner targets Linux; process CPU time excludes descheduling.
#[repr(C)]
struct Timespec {
    seconds: i64,
    nanos: i64,
}
unsafe extern "C" {
    fn clock_gettime(clock: i32, time: *mut Timespec) -> i32;
}
fn cpu_time() -> u128 {
    let mut time = Timespec {
        seconds: 0,
        nanos: 0,
    };
    assert_eq!(unsafe { clock_gettime(2, &mut time) }, 0);
    time.seconds as u128 * 1_000_000_000 + time.nanos as u128
}
fn phase<T>(f: impl FnOnce() -> T) -> (T, Phase) {
    #[cfg(feature = "alloc-meter")]
    let start = meter::begin();
    let cpu = cpu_time();
    let clock = Instant::now();
    let result = black_box(f());
    let ns = clock.elapsed().as_nanos();
    let cpu_ns = cpu_time() - cpu;
    #[cfg(feature = "alloc-meter")]
    let heap = meter::end(start);
    (
        result,
        Phase {
            ns,
            cpu_ns,
            #[cfg(feature = "alloc-meter")]
            heap,
        },
    )
}
impl Phase {
    fn json(self) -> String {
        #[cfg(feature = "alloc-meter")]
        return format!(
            "{{\"ns\":{},\"cpu_ns\":{},\"heap\":{}}}",
            self.ns,
            self.cpu_ns,
            self.heap.json()
        );
        #[cfg(not(feature = "alloc-meter"))]
        format!("{{\"ns\":{},\"cpu_ns\":{}}}", self.ns, self.cpu_ns)
    }
}
trait Backend: Sized {
    type State;
    fn prepare(rule: &Rule) -> Self;
    fn setup(&self, q: &Query) -> Self::State;
    fn advance(state: &mut Self::State, budget: usize) -> bool;
    fn observe(state: &mut Self::State) -> Option<Answer>;
    fn work(state: &Self::State) -> String;
}
struct Local<const MODE: usize>(Arc<local::Plan>);
impl<const MODE: usize> Backend for Local<MODE> {
    type State = local::Run<WORK>;
    fn prepare(rule: &Rule) -> Self {
        Self(local::Plan::compile(rule).unwrap())
    }
    fn setup(&self, q: &Query) -> Self::State {
        self.0.start_with_metrics::<WORK>(
            q,
            [
                local::DependencyMode::Endpoint,
                local::DependencyMode::Filtered,
                local::DependencyMode::Indexed,
            ][MODE],
        )
    }
    fn advance(s: &mut Self::State, budget: usize) -> bool {
        s.advance(budget)
    }
    fn observe(s: &mut Self::State) -> Option<Answer> {
        s.observe()
    }
    fn work(s: &Self::State) -> String {
        format!(
            "{{\"applications\":{},\"dependency\":{}}}",
            s.consumed_tokens.len(),
            s.work_json()
        )
    }
}
struct Compiled<const INDEX: bool, const SPECIAL: bool>(chr_compiled::PreparedRuleset);
impl<const INDEX: bool, const SPECIAL: bool> Backend for Compiled<INDEX, SPECIAL> {
    type State = chr_compiled::Engine;
    fn prepare(rule: &Rule) -> Self {
        let p = chr_compiled::PreparedRuleset::new(vec![rule.clone()], None).unwrap();
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
    fn advance(s: &mut Self::State, budget: usize) -> bool {
        s.advance(budget).exhausted
    }
    fn observe(s: &mut Self::State) -> Option<Answer> {
        s.observe()
    }
    fn work(s: &Self::State) -> String {
        let w = s.stats();
        format!(
            "{{\"applications\":{},\"specialized\":{},\"candidates\":{},\"cursor_steps\":{}}}",
            w.applications, w.specialized_applications, w.candidate_visits, w.cursor_steps
        )
    }
}
fn source(family: &str, n: usize, query: usize) -> (Rule, Query) {
    let (family, updates) = match family.strip_prefix("aliases-") {
        Some(count) => (
            "lowyield",
            count.parse::<usize>().expect("alias update count"),
        ),
        None => (family, 1),
    };
    assert!([1, 8, 32].contains(&updates));
    if family == "chain" {
        let rule = Rule::simplify(
            "chain",
            [c("take", [t("f", [v(0)]), v(1)]), c("token", [])],
            and([
                eq(v(1), t("g", [v(0)])),
                c("take", [v(0), v(2)]).into(),
                c("token", []).into(),
                c("link", [v(1), v(2), v(2)]).into(),
            ]),
        );
        return (
            rule,
            Query {
                constraints: vec![
                    c(
                        "take",
                        [
                            (0..n).fold(atom(&format!("leaf{query}")), |x, _| t("f", [x])),
                            v(900),
                        ],
                    ),
                    c("token", []),
                ],
                outputs: vec![("out".into(), Var(900))],
            },
        );
    }
    let rule = Rule::simplify(
        "pair",
        [c("take", [t("pair", [v(0), v(0)]), v(1)]), c("token", [])],
        eq(v(1), v(0)),
    );
    let mut constraints = vec![];
    let mut outputs = vec![];
    let label = atom(&format!("a{query}"));
    let base = 1000 + query as u64 * 10000;
    for i in 0..n as u64 {
        let (a, b) = match family {
            "flat" => (label.clone(), label.clone()),
            "shared" => (v(base), label.clone()),
            "lowyield" => (v(base), v(base + 10 + i)),
            _ => panic!("unknown family"),
        };
        constraints.extend([
            c("take", [t("pair", [a, b]), v(base + 1000 + i)]),
            c("token", []),
        ]);
        outputs.push((format!("out{i}"), Var(base + 1000 + i)));
        if family == "lowyield" {
            outputs.push((format!("other{i}"), Var(base + 10 + i)));
        }
    }
    if family != "flat" {
        outputs.push(("shared".into(), Var(base)));
        outputs.push(("seed".into(), Var(base + 1)));
        for j in 0..updates as u64 {
            let seed = if j == 0 { base + 1 } else { base + 2000 + j };
            let x = if family == "shared" {
                label.clone()
            } else {
                v(seed)
            };
            constraints.extend([
                c("take", [t("pair", [x.clone(), x]), v(base)]),
                c("token", []),
            ]);
            if j > 0 {
                outputs.push((format!("seed{j}"), Var(seed)));
            }
        }
    }
    (
        rule,
        Query {
            constraints,
            outputs,
        },
    )
}
fn measure<B: Backend>(family: &str, n: usize, reuse: usize, stop: &str) {
    let cancel = stop != "complete";
    let (rule, _) = source(family, n, 0);
    let queries = (0..reuse)
        .map(|i| source(family, n, i).1)
        .collect::<Vec<_>>();
    let expected = queries
        .iter()
        .map(|q| scalar::run(std::slice::from_ref(&rule), q, 200_000))
        .collect::<Vec<_>>();
    let mut records = Vec::with_capacity(reuse * 5 + 2);
    let mut work = Vec::with_capacity(if WORK { reuse } else { 0 });
    #[cfg(feature = "alloc-meter")]
    let baseline = meter::begin();
    let (prepared, p) = phase(|| B::prepare(&rule));
    records.push(p);
    for (i, q) in queries.iter().enumerate() {
        let (mut state, p) = phase(|| prepared.setup(q));
        records.push(p);
        let (ended, p) = phase(|| {
            B::advance(
                &mut state,
                if stop == "setup" {
                    0
                } else if cancel {
                    1
                } else {
                    200_000
                },
            )
        });
        records.push(p);
        assert!(cancel || ended, "execution cutoff");
        let (answer, p) = phase(|| if cancel { None } else { B::observe(&mut state) });
        records.push(p);
        // Allocate diagnostic serialization outside phases, then release it before restoration.
        if WORK {
            work.push(B::work(&state));
        }
        let (_, p) = phase(|| drop(state));
        records.push(p);
        if !cancel {
            match (answer.as_ref(), expected[i].as_slice()) {
                (None, []) => (),
                (Some(actual), [expected]) => assert!(chr_observe::equivalent(
                    actual,
                    expected,
                    &mut Default::default()
                )),
                _ => panic!("raw answer cardinality differs"),
            }
        }
        let (_, p) = phase(|| drop(answer));
        records.push(p);
    }
    let (_, p) = phase(|| drop(prepared));
    records.push(p);
    let work_json = if WORK {
        format!("[{}]", work.join(","))
    } else {
        "[]".into()
    };
    drop(work);
    // Work JSON is a harness-owned allocation: release before checking baseline.
    #[cfg(feature = "alloc-meter")]
    {
        drop(work_json);
        let r = meter::end(baseline);
        assert_eq!(r.live_start, r.live_end);
    }
    print!(
        "{{\"family\":\"{family}\",\"size\":{n},\"reuse\":{reuse},\"stop\":\"{stop}\",\"metrics\":{WORK},\"meter\":{},\"phases\":[",
        cfg!(feature = "alloc-meter")
    );
    for (i, p) in records.into_iter().enumerate() {
        if i > 0 {
            print!(",");
        }
        print!("{}", p.json());
    }
    #[cfg(not(feature = "alloc-meter"))]
    println!("],\"work\":{work_json}}}");
    #[cfg(feature = "alloc-meter")]
    println!("],\"work\":[],\"restored\":true}}");
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        smoke();
        return;
    }
    assert_eq!(
        args.len(),
        6,
        "mode family size reuse complete|setup|cancel"
    );
    if cfg!(feature = "alloc-meter") && WORK {
        panic!("measurement requires separate allocation and work builds");
    }
    assert_eq!(WORK, chr_compiled::COLLECT_METRICS);
    let mode = &args[1];
    let family = &args[2];
    let n = args[3].parse().unwrap();
    let reuse = args[4].parse().unwrap();
    let stop = &args[5];
    assert!(["complete", "setup", "cancel"].contains(&stop.as_str()));
    match mode.as_str() {
        "endpoint" => measure::<Local<0>>(family, n, reuse, stop),
        "filtered" => measure::<Local<1>>(family, n, reuse, stop),
        "local-indexed" => measure::<Local<2>>(family, n, reuse, stop),
        "scan" => measure::<Compiled<false, false>>(family, n, reuse, stop),
        "indexed" => measure::<Compiled<true, false>>(family, n, reuse, stop),
        "special-scan" => measure::<Compiled<false, true>>(family, n, reuse, stop),
        "special-indexed" => measure::<Compiled<true, true>>(family, n, reuse, stop),
        _ => panic!("unknown mode"),
    }
}

fn check<B: Backend>(rule: &Rule, q: &Query, expected: &[Answer]) {
    let p = B::prepare(rule);
    let mut s = p.setup(q);
    assert!(B::advance(&mut s, 200_000));
    scalar::same_raw(B::observe(&mut s).into_iter().collect(), expected.to_vec());
}
fn smoke() {
    for family in [
        "chain",
        "flat",
        "shared",
        "lowyield",
        "aliases-1",
        "aliases-8",
        "aliases-32",
    ] {
        for n in [0, 1, 4] {
            let (rule, q) = source(family, n, 2);
            let expected = scalar::run(std::slice::from_ref(&rule), &q, 200_000);
            check::<Local<0>>(&rule, &q, &expected);
            check::<Local<1>>(&rule, &q, &expected);
            check::<Local<2>>(&rule, &q, &expected);
            check::<Compiled<false, false>>(&rule, &q, &expected);
            check::<Compiled<true, false>>(&rule, &q, &expected);
            check::<Compiled<false, true>>(&rule, &q, &expected);
            check::<Compiled<true, true>>(&rule, &q, &expected);
        }
    }
    println!("147 independent source comparisons pass");
}
