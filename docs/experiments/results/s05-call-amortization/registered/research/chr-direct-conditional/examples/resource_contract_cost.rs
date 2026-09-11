//! Separate counter-free allocation and ordinary timing processes.
#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
#[path = "support/post_choice_source.rs"]
#[allow(dead_code)]
mod source;
use chr_compiled::resource_contract::{Admission, Declaration, Predicate, PreparedContract};
use chr_compiled::{Access, Policy};
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

fn prepare(r: &[Rule], declared: bool) -> PreparedContract {
    let declaration = declared.then(|| Declaration {
        resource: Predicate {
            name: "fuel".into(),
            arity: 0,
        },
        access_rules: vec![1],
        entry: Predicate {
            name: "start".into(),
            arity: 3,
        },
        ground_argument: 0,
    });
    PreparedContract::new(r.to_vec(), declaration, Admission::Optional, true).unwrap()
}
fn query(f: &str, k: usize, d: usize, i: usize, schedule: &str) -> Query {
    let mut q = input(f, k, d, i, schedule == "missing-permit");
    if schedule == "unknown" && i % 2 == 1 {
        q.constraints
            .iter_mut()
            .find(|c| c.name == "start")
            .unwrap()
            .args[0] = chr_syntax::v(99998);
    }
    q
}
fn answers(f: &str, k: usize, d: usize, i: usize, schedule: &str) -> Vec<Answer> {
    let mut out = expected(f, k, d, i, schedule == "missing-permit");
    if schedule == "unknown" && i % 2 == 1 {
        use chr_syntax::{c, v};
        for a in &mut out {
            a.outputs[0].1 = v(99999);
            a.residual.retain(|c| c.name != "fresh");
            a.residual.push(c("run", [v(99998), v(99999)]));
            a.residual.extend((0..d).map(|_| c("fuel", [])));
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
    assert!(["inferred", "declared"].contains(&mode.as_str()));
    assert!(["common", "independent"].contains(&family.as_str()));
    assert!([0, 3].contains(&k) && [0, 16].contains(&d) && [1, 4].contains(&reuse));
    assert!(["eligible", "missing-permit", "unknown"].contains(&schedule.as_str()));
    let declared = mode == "declared";
    let rules = source::rules(family, k);
    let expected = (0..reuse)
        .map(|i| answers(family, k, d, i, schedule))
        .collect::<Vec<_>>();
    let warm = prepare(&rules, declared);
    for (i, a) in expected.iter().enumerate() {
        let q = query(family, k, d, i, schedule);
        same(&scalar::run(&rules, &q, 500000), a);
        let e = warm.start(q, Policy::Global, Access::Scan);
        assert_eq!(e.is_err(), declared && schedule == "unknown" && i % 2 == 1);
        if let Ok(e) = e {
            let mut e = Engine::Compiled(e);
            let mut budget = 500000;
            let first = next(&mut e, &mut budget);
            same(&finish(&mut e, first, &mut budget), a);
        }
    }
    drop(warm);
    let mut phases = Vec::with_capacity(7 + 7 * reuse);
    let mut rejected = 0;
    let (prepared, preparation) = phase(|| prepare(&rules, declared));
    phases.push(("prepare", preparation));
    for (i, a) in expected.iter().enumerate() {
        let (q, m) = phase(|| query(family, k, d, i, schedule));
        phases.push(("input", m));
        let (mut search, m) = phase(|| {
            prepared
                .start(q.clone(), Policy::Global, Access::Scan)
                .map(Engine::Compiled)
        });
        phases.push(("admit-lower-setup", m));
        let reject = declared && schedule == "unknown" && i % 2 == 1;
        assert_eq!(search.is_err(), reject);
        rejected += usize::from(reject);
        let mut budget = 500000;
        let (first, m) = phase(|| search.as_mut().ok().and_then(|e| next(e, &mut budget)));
        phases.push(("first-delivery", m));
        let (out, m) = phase(|| match search.as_mut() {
            Ok(e) => finish(e, first, &mut budget),
            Err(_) => vec![],
        });
        phases.push(("remaining-delivery", m));
        if !reject {
            same(&out, a);
        } else {
            assert!(out.is_empty());
        }
        let (_, m) = phase(|| drop(search));
        phases.push(("search-or-error-drop", m));
        let (_, m) = phase(|| drop(out));
        phases.push(("answers-drop", m));
        let (_, m) = phase(|| drop(q));
        phases.push(("input-drop", m));
        #[cfg(feature = "alloc-meter")]
        assert_eq!(m.memory.live_end, preparation.memory.live_end);
    }
    let i = usize::from(schedule != "eligible");
    let (q, m) = phase(|| query(family, k, d, i, schedule));
    phases.push(("cancel-input", m));
    let (mut search, m) = phase(|| {
        prepared
            .start(q.clone(), Policy::Global, Access::Scan)
            .map(Engine::Compiled)
    });
    phases.push(("cancel-admit-lower-setup", m));
    let reject = declared && schedule == "unknown";
    assert_eq!(search.is_err(), reject);
    rejected += usize::from(reject);
    let (_, m) = phase(|| {
        if let Ok(e) = &mut search {
            drop(std::hint::black_box(e.step()));
        }
    });
    phases.push(("cancel-unit", m));
    let (_, m) = phase(|| drop(search));
    phases.push(("cancel-search-or-error-drop", m));
    let (_, m) = phase(|| drop(q));
    phases.push(("cancel-input-drop", m));
    #[cfg(feature = "alloc-meter")]
    assert_eq!(m.memory.live_end, preparation.memory.live_end);
    let (_, m) = phase(|| drop(prepared));
    phases.push(("prepared-drop", m));
    #[cfg(feature = "alloc-meter")]
    {
        assert_eq!(m.memory.live_end, preparation.memory.live_start);
        assert!(
            phases
                .windows(2)
                .all(|x| x[0].1.memory.live_end == x[1].1.memory.live_start)
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
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"choices\":{k},\"depth\":{d},\"reuse\":{reuse},\"schedule\":\"{schedule}\",\"rejected\":{rejected},\"meter\":{},\"phases\":[{records}]}}",
        cfg!(feature = "alloc-meter")
    );
}
