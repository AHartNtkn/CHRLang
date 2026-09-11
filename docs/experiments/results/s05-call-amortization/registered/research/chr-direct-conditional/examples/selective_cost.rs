//! Separate counter-free allocation and ordinary timing processes.
#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
#[path = "support/selective_source.rs"]
#[allow(dead_code)]
mod source;
use chr_compiled::resource_count::Program;
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
fn input(family: &str, k: usize, d: usize, i: usize, mixed: bool) -> Query {
    let mut q = source::query(family, k, d, i);
    if mixed && i % 2 == 1 {
        q.constraints.retain(|c| c.name != "permit");
    }
    q
}
fn expected(family: &str, k: usize, d: usize, i: usize, mixed: bool) -> Vec<Answer> {
    use chr_syntax::{Term, atom, c, t, v};
    let mut out = source::expected(family, k, d, i);
    if mixed && i % 2 == 1 {
        for a in &mut out {
            let extra = if family == "independent" {
                a.outputs
                    .iter()
                    .skip(1)
                    .filter(|(_, x)| matches!(x,Term::App(n,xs) if n=="b" && xs.is_empty()))
                    .count()
            } else {
                0
            };
            a.residual
                .retain(|c| !matches!(c.name.as_str(), "permit" | "mark" | "fuel"));
            a.residual.extend(
                (0..d + if family == "independent" { k } else { 0 }).map(|_| c("fuel", [])),
            );
            if d + extra > 0 {
                a.residual.retain(|c| c.name != "fresh");
                a.outputs[0].1 = v(99999);
                let mut depth = atom("z");
                for _ in 0..d + extra {
                    depth = t("s", [depth]);
                }
                a.residual.push(c("run", [depth, v(99999)]));
            }
        }
    }
    out
}
fn main() {
    assert!(!std::hint::black_box(cfg!(feature = "metrics")));
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    assert_eq!(args.len(), 6);
    let mode = &args[0];
    let family = &args[1];
    let k: usize = args[2].parse().unwrap();
    let d: usize = args[3].parse().unwrap();
    let reuse: usize = args[4].parse().unwrap();
    let schedule = &args[5];
    let (form, executor) = mode.split_once('-').unwrap();
    assert!(["original", "counted"].contains(&form));
    assert!(
        [
            "scan",
            "indexed",
            "special",
            "contextual",
            "demand",
            "shared",
            "conditional"
        ]
        .contains(&executor)
    );
    assert!(source::FAMILIES.contains(&family.as_str()));
    assert!([0, 3].contains(&k) && [0, 1, 16].contains(&d) && [1, 4].contains(&reuse));
    assert!(["eligible", "mixed"].contains(&schedule.as_str()));
    let mixed = schedule == "mixed";
    let counted = form == "counted";
    assert!(family != "dense" || (!counted && !mixed && d == 0));
    let rules = source::rules(family, k);
    let expected = (0..reuse)
        .map(|i| expected(family, k, d, i, mixed))
        .collect::<Vec<_>>();
    let warm_program = Program::infer(&rules).ok();
    let warm = prepare(&rules, executor);
    for (i, answer) in expected.iter().enumerate() {
        let q = input(family, k, d, i, mixed);
        same(&scalar::run(&rules, &q, 500000), answer);
        let lowered = warm_program.as_ref().and_then(|p| p.lower(&q).ok());
        assert_eq!(
            lowered.is_some(),
            family != "dense" && !(mixed && i % 2 == 1)
        );
        for chosen in [Some(&q), lowered.as_ref()].into_iter().flatten() {
            let mut e = warm.start(chosen);
            let mut budget = 500000;
            let first = next(&mut e, &mut budget);
            same(&finish(&mut e, first, &mut budget), answer);
        }
    }
    drop(warm);
    drop(warm_program);
    let mut phases = Vec::with_capacity(11 + 9 * reuse);
    let mut admitted = 0;
    let (program, inference) = phase(|| counted.then(|| Program::infer(&rules).unwrap()));
    phases.push(("inference", inference));
    let (prepared, preparation) = phase(|| prepare(&rules, executor));
    phases.push(("prepare", preparation));
    for (i, expected) in expected.iter().enumerate() {
        let (q, m) = phase(|| input(family, k, d, i, mixed));
        phases.push(("input", m));
        let (lowered, m) = phase(|| program.as_ref().and_then(|p| p.lower(&q).ok()));
        phases.push(("transform", m));
        assert_eq!(lowered.is_some(), counted && !(mixed && i % 2 == 1));
        admitted += usize::from(lowered.is_some());
        let (mut search, m) = phase(|| prepared.start(lowered.as_ref().unwrap_or(&q)));
        phases.push(("setup", m));
        let mut budget = 500000;
        let (first, m) = phase(|| next(&mut search, &mut budget));
        phases.push(("first-delivery-or-exhaustion", m));
        let (answers, m) = phase(|| finish(&mut search, first, &mut budget));
        phases.push(("remaining-delivery", m));
        same(&answers, expected);
        let (_, m) = phase(|| drop(search));
        phases.push(("search-drop", m));
        let (_, m) = phase(|| drop(answers));
        phases.push(("answers-drop", m));
        let (_, m) = phase(|| drop(lowered));
        phases.push(("transformed-drop", m));
        let (_, m) = phase(|| drop(q));
        phases.push(("input-drop", m));
        #[cfg(feature = "alloc-meter")]
        assert_eq!(m.memory.live_end, preparation.memory.live_end);
    }
    let cancel_i = usize::from(mixed);
    let (q, m) = phase(|| input(family, k, d, cancel_i, mixed));
    phases.push(("cancel-input", m));
    let (lowered, m) = phase(|| program.as_ref().and_then(|p| p.lower(&q).ok()));
    phases.push(("cancel-transform", m));
    assert_eq!(lowered.is_some(), counted && !mixed);
    admitted += usize::from(lowered.is_some());
    let (mut search, m) = phase(|| prepared.start(lowered.as_ref().unwrap_or(&q)));
    phases.push(("cancel-setup", m));
    let (_, m) = phase(|| drop(std::hint::black_box(search.step())));
    phases.push(("cancel-unit", m));
    let (_, m) = phase(|| drop(search));
    phases.push(("cancel-search-drop", m));
    let (_, m) = phase(|| drop(lowered));
    phases.push(("cancel-transformed-drop", m));
    let (_, m) = phase(|| drop(q));
    phases.push(("cancel-input-drop", m));
    #[cfg(feature = "alloc-meter")]
    assert_eq!(m.memory.live_end, preparation.memory.live_end);
    let (_, m) = phase(|| drop(prepared));
    phases.push(("prepared-drop", m));
    #[cfg(feature = "alloc-meter")]
    assert_eq!(m.memory.live_end, inference.memory.live_end);
    let (_, m) = phase(|| drop(program));
    phases.push(("inference-drop", m));
    #[cfg(feature = "alloc-meter")]
    {
        assert_eq!(m.memory.live_end, inference.memory.live_start);
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
    let selective = cfg!(feature = "selective-discovery");
    println!(
        "{{\"mode\":\"{mode}\",\"selective\":{selective},\"family\":\"{family}\",\"choices\":{k},\"depth\":{d},\"reuse\":{reuse},\"schedule\":\"{schedule}\",\"admitted\":{admitted},\"meter\":{},\"phases\":[{records}]}}",
        cfg!(feature = "alloc-meter")
    );
}
