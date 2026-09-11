//! Separate counter-free allocation and ordinary timing processes.
#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
#[path = "support/resumable_source.rs"]
mod source;
use chr_compiled::{Access, Policy, PreparedRuleset};
use chr_syntax::{Answer, Query, Rule};
#[path = "../tests/composition_support/mod.rs"]
#[allow(dead_code)]
mod engines;
use engines::{Engine, Event};
use std::time::Instant;
#[repr(C)]
struct Timespec {
    seconds: i64,
    nanos: i64,
}
unsafe extern "C" {
    fn clock_gettime(clock: i32, time: *mut Timespec) -> i32;
}
fn cpu() -> u128 {
    let mut t = Timespec {
        seconds: 0,
        nanos: 0,
    };
    assert_eq!(unsafe { clock_gettime(2, &mut t) }, 0);
    t.seconds as u128 * 1_000_000_000 + t.nanos as u128
}
#[derive(Clone, Copy)]
struct Reading {
    ns: u128,
    cpu_ns: u128,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn phase<T>(f: impl FnOnce() -> T) -> (T, Reading) {
    #[cfg(feature = "alloc-meter")]
    let start = meter::begin();
    let c = cpu();
    let t = Instant::now();
    let x = std::hint::black_box(f());
    let ns = t.elapsed().as_nanos();
    let cpu_ns = cpu() - c;
    #[cfg(feature = "alloc-meter")]
    let memory = meter::end(start);
    (
        x,
        Reading {
            ns,
            cpu_ns,
            #[cfg(feature = "alloc-meter")]
            memory,
        },
    )
}
fn next(search: &mut Engine, budget: &mut usize) -> Option<Answer> {
    while *budget > 0 {
        *budget -= 1;
        match search.step() {
            Event::Answer(a) => return Some(a),
            Event::Exhausted => return None,
            Event::Progress => (),
        }
    }
    panic!("unfinished service bound")
}
fn finish(search: &mut Engine, first: Option<Answer>, budget: &mut usize) -> Vec<Answer> {
    let mut out = first.into_iter().collect::<Vec<_>>();
    if out.is_empty() {
        return out;
    }
    while let Some(a) = next(search, budget) {
        out.push(a);
    }
    out
}
fn same(a: &[Answer], b: &[Answer]) {
    assert_eq!(a.len(), b.len());
    let mut used = vec![false; b.len()];
    for a in a {
        let i = b
            .iter()
            .enumerate()
            .position(|(i, b)| !used[i] && chr_observe::equivalent(a, b, &mut Default::default()))
            .expect("raw answer mismatch");
        used[i] = true;
    }
}
enum Prepared {
    Compiled(PreparedRuleset, Access),
    Contextual(
        std::sync::Arc<chr_relational::contextual_execute::Prepared>,
        bool,
    ),
    Conditional(chr_direct_conditional::engine::PreparedRuleset),
    Demand(std::sync::Arc<chr_relational::contextual_execute::Prepared>),
    Resumable(std::sync::Arc<chr_relational::contextual_execute::Prepared>),
}
impl Prepared {
    fn start(&self, q: &Query) -> Engine {
        match self {
            Self::Compiled(p, access) => {
                Engine::Compiled(p.start_search(q.clone(), Policy::Global, *access).unwrap())
            }
            Self::Contextual(p, shared) => Engine::Contextual(if *shared {
                p.start_persistent_equality(q, true)
            } else {
                p.start(q)
            }),
            Self::Demand(p) => Engine::Contextual(p.start_demand(q)),
            Self::Resumable(p) => Engine::Contextual(p.start_resumable(q)),
            Self::Conditional(p) => Engine::Conditional(p.start(q.clone()).unwrap()),
        }
    }
}
fn prepare(r: &[Rule], mode: &str) -> Prepared {
    match mode {
        "scan" | "indexed" | "special" => {
            let p = PreparedRuleset::new(r.to_vec(), None).unwrap();
            Prepared::Compiled(
                if mode == "special" {
                    p.specialize_inferred()
                } else {
                    p
                },
                if mode == "indexed" {
                    Access::Indexed
                } else {
                    Access::Scan
                },
            )
        }
        "resumable" => {
            Prepared::Resumable(chr_relational::contextual_execute::Prepared::new(r).unwrap())
        }
        "demand" => Prepared::Demand(chr_relational::contextual_execute::Prepared::new(r).unwrap()),
        "contextual" | "shared" => Prepared::Contextual(
            chr_relational::contextual_execute::Prepared::new(r).unwrap(),
            mode == "shared",
        ),
        "conditional" => Prepared::Conditional(
            chr_direct_conditional::engine::PreparedRuleset::new(r.to_vec()).unwrap(),
        ),
        _ => panic!("unknown mode"),
    }
}
fn main() {
    assert!(!std::hint::black_box(cfg!(feature = "metrics")));
    assert!(std::hint::black_box(cfg!(feature = "selective-discovery")));
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    assert_eq!(args.len(), 5);
    let mode = &args[0];
    let family = &args[1];
    let n: usize = args[2].parse().unwrap();
    let reuse: usize = args[3].parse().unwrap();
    let consumer = &args[4];
    assert!(
        [
            "scan",
            "indexed",
            "special",
            "contextual",
            "demand",
            "resumable",
            "shared",
            "conditional"
        ]
        .contains(&mode.as_str())
    );
    assert!(source::FAMILIES.contains(&family.as_str()));
    assert!([0, 1, 3].contains(&n) && [1, 4].contains(&reuse));
    assert!(["query", "prepared"].contains(&consumer.as_str()));
    let rules = source::rules(family);
    let expected = (0..reuse)
        .map(|i| source::expected(family, n, i))
        .collect::<Vec<_>>();
    let warm = prepare(&rules, mode);
    for (i, answer) in expected.iter().enumerate() {
        let q = source::query(family, n, i);
        same(&scalar::run(&rules, &q, 500000), answer);
        let mut e = warm.start(&q);
        let mut budget = 500000;
        let first = next(&mut e, &mut budget);
        same(&finish(&mut e, first, &mut budget), answer);
        if let Engine::Contextual(e) = &e {
            assert_eq!(e.discovery_stats().offered, 0);
        }
    }
    drop(warm);
    let mut phases = Vec::with_capacity(9 + reuse * 7);
    let mut held: Vec<Vec<Answer>> = Vec::with_capacity(reuse);
    #[cfg(feature = "alloc-meter")]
    let mut held_bytes = 0;
    let (prepared, preparation) = phase(|| prepare(&rules, mode));
    phases.push(("prepare", preparation));
    for (i, expected) in expected.iter().enumerate() {
        let (q, m) = phase(|| source::query(family, n, i));
        #[cfg(feature = "alloc-meter")]
        let input_bytes = m.memory.live_end - m.memory.live_start;
        phases.push(("input", m));
        let (mut search, m) = phase(|| prepared.start(&q));
        phases.push(("setup", m));
        let mut budget = 500000;
        let (first, m) = phase(|| next(&mut search, &mut budget));
        phases.push(("first-delivery-or-exhaustion", m));
        let (answers, m) = phase(|| finish(&mut search, first, &mut budget));
        phases.push(("remaining-delivery", m));
        same(&answers, expected);
        let (_, m) = phase(|| drop(search));
        phases.push(("search-drop", m));
        #[cfg(feature = "alloc-meter")]
        let answer_bytes =
            m.memory.live_end - preparation.memory.live_end - held_bytes - input_bytes;
        let (_, m) = phase(|| {
            if consumer == "prepared" {
                held.push(answers)
            } else {
                drop(answers)
            }
        });
        phases.push(("answer-policy", m));
        #[cfg(feature = "alloc-meter")]
        if consumer == "prepared" {
            held_bytes += answer_bytes;
        }
        let (_, m) = phase(|| drop(q));
        phases.push(("input-drop", m));
        #[cfg(feature = "alloc-meter")]
        assert_eq!(m.memory.live_end, preparation.memory.live_end + held_bytes);
    }
    let (q, m) = phase(|| source::query(family, n, 0));
    phases.push(("cancel-input", m));
    let (mut search, m) = phase(|| prepared.start(&q));
    phases.push(("cancel-setup", m));
    let (_, m) = phase(|| drop(std::hint::black_box(search.step())));
    phases.push(("cancel-unit", m));
    let (_, m) = phase(|| drop(search));
    phases.push(("cancel-search-drop", m));
    let (_, m) = phase(|| drop(q));
    phases.push(("cancel-input-drop", m));
    #[cfg(feature = "alloc-meter")]
    assert_eq!(m.memory.live_end, preparation.memory.live_end + held_bytes);
    let (_, m) = phase(|| drop(prepared));
    phases.push(("prepared-drop", m));
    #[cfg(feature = "alloc-meter")]
    assert_eq!(
        m.memory.live_end,
        preparation.memory.live_start + held_bytes
    );
    for (actual, expected) in held.iter().zip(&expected) {
        same(actual, expected);
    }
    let (_, m) = phase(|| held.clear());
    phases.push(("retained-answers-drop", m));
    #[cfg(feature = "alloc-meter")]
    {
        assert_eq!(m.memory.live_end, preparation.memory.live_start);
        assert!(
            phases
                .windows(2)
                .all(|xs| xs[0].1.memory.live_end == xs[1].1.memory.live_start)
        );
    }
    let records = phases
        .iter()
        .map(|(name, m)| {
            #[cfg(feature = "alloc-meter")]
            let memory = m.memory.json();
            #[cfg(not(feature = "alloc-meter"))]
            let memory = "null".to_string();
            format!(
                "{{\"phase\":\"{name}\",\"ns\":{},\"cpu_ns\":{},\"memory\":{memory}}}",
                m.ns, m.cpu_ns
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"n\":{n},\"reuse\":{reuse},\"consumer\":\"{consumer}\",\"meter\":{},\"phases\":[{records}]}}",
        cfg!(feature = "alloc-meter")
    );
}
