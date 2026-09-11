//! Separate counter-free allocation and ordinary timing processes.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
#[path = "support/resource_fusion_source.rs"]
mod source;
#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use chr_compiled::{
    Access, Engine, Policy, PreparedRuleset, SearchEngine, SearchEvent, resource_fusion::Program,
};
use chr_syntax::{Answer, Rule};
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
fn finish(e: &mut SearchEngine) -> Vec<Engine> {
    let mut out = vec![];
    for _ in 0..200_000 {
        match e.tick() {
            SearchEvent::Complete(b) => out.push(b.engine),
            SearchEvent::Exhausted => return out,
            _ => (),
        }
    }
    panic!("unfinished service bound")
}
fn observe(states: &mut [Engine]) -> Vec<Answer> {
    states.iter_mut().map(|e| e.observe().unwrap()).collect()
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
fn prepare(r: &[Rule], special: bool) -> PreparedRuleset {
    let p = PreparedRuleset::new(r.to_vec(), None).unwrap();
    if special { p.specialize_inferred() } else { p }
}
fn main() {
    assert!(
        !std::hint::black_box(cfg!(feature = "metrics")),
        "work counters forbidden"
    );
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    assert_eq!(args.len(), 4);
    let mode = &args[0];
    let family = &args[1];
    let n = args[2].parse().unwrap();
    let reuse = args[3].parse().unwrap();
    assert!(
        [
            "original-scan",
            "original-special",
            "original-indexed",
            "fused-scan",
            "fused-special",
            "fused-indexed"
        ]
        .contains(&mode.as_str())
    );
    assert!(["plain", "choices", "duplicates", "shared", "spare"].contains(&family.as_str()));
    assert!([0, 1, 4].contains(&n) && [1, 4].contains(&reuse));
    let fused = mode.starts_with("fused");
    let special = mode.ends_with("special");
    let access = if mode.ends_with("indexed") {
        Access::Indexed
    } else {
        Access::Scan
    };
    let rules = source::rules(family);
    let expected = (0..reuse)
        .map(|i| scalar::run(&rules, &source::query(family, n, i), 200_000))
        .collect::<Vec<_>>();
    let warm = fused.then(|| Program::infer(&rules).unwrap());
    let q = source::query(family, n, 0);
    let warm_p = prepare(
        warm.as_ref().map_or(&rules[..], |p| p.lower(&q).unwrap()),
        special,
    );
    for (i, expected) in expected.iter().enumerate() {
        let q = source::query(family, n, i);
        if let Some(p) = &warm {
            p.lower(&q).unwrap();
        }
        let mut s = warm_p.start_search(q, Policy::Global, access).unwrap();
        same(&observe(&mut finish(&mut s)), expected);
    }
    drop(warm_p);
    drop(warm);
    drop(q);
    let mut phases = Vec::with_capacity(10 + reuse * 9);
    let (inferred, inference) = phase(|| fused.then(|| Program::infer(&rules).unwrap()));
    phases.push(("inference", inference));
    let mut prepared = None;
    #[cfg(feature = "alloc-meter")]
    let mut retained = 0;
    for (i, expected) in expected.iter().enumerate() {
        let (q, m) = phase(|| source::query(family, n, i));
        phases.push(("input", m));
        let (selected, m) = phase(|| {
            inferred
                .as_ref()
                .map_or(&rules[..], |p| p.lower(&q).unwrap())
        });
        phases.push(("certify", m));
        if i == 0 {
            let (p, m) = phase(|| prepare(selected, special));
            #[cfg(feature = "alloc-meter")]
            {
                retained = m.memory.live_end - m.memory.live_start;
            }
            phases.push(("prepare", m));
            prepared = Some(p);
        }
        let (mut search, m) = phase(|| {
            prepared
                .as_ref()
                .unwrap()
                .start_search(q.clone(), Policy::Global, access)
                .unwrap()
        });
        phases.push(("setup", m));
        let (mut complete, m) = phase(|| finish(&mut search));
        phases.push(("execute", m));
        let (answers, m) = phase(|| observe(&mut complete));
        phases.push(("observe", m));
        same(&answers, expected);
        let (_, m) = phase(|| drop(complete));
        phases.push(("completed-drop", m));
        let (_, m) = phase(|| drop(search));
        phases.push(("search-drop", m));
        let (_, m) = phase(|| drop(answers));
        phases.push(("answers-drop", m));
        let (_, m) = phase(|| drop(q));
        phases.push(("input-drop", m));
        #[cfg(feature = "alloc-meter")]
        assert_eq!(m.memory.live_end, inference.memory.live_end + retained);
    }
    let (q, m) = phase(|| source::query(family, n, 0));
    phases.push(("cancel-input", m));
    let (_, m) = phase(|| {
        if let Some(p) = &inferred {
            p.lower(&q).unwrap();
        }
    });
    phases.push(("cancel-certify", m));
    let (mut search, m) = phase(|| {
        prepared
            .as_ref()
            .unwrap()
            .start_search(q.clone(), Policy::Global, access)
            .unwrap()
    });
    phases.push(("cancel-setup", m));
    let (_, m) = phase(|| drop(std::hint::black_box(search.tick())));
    phases.push(("cancel-unit", m));
    let (_, m) = phase(|| drop(search));
    phases.push(("cancel-search-drop", m));
    let (_, m) = phase(|| drop(q));
    phases.push(("cancel-input-drop", m));
    #[cfg(feature = "alloc-meter")]
    assert_eq!(m.memory.live_end, inference.memory.live_end + retained);
    let (_, m) = phase(|| drop(prepared));
    phases.push(("prepared-drop", m));
    #[cfg(feature = "alloc-meter")]
    assert_eq!(m.memory.live_end, inference.memory.live_end);
    let (_, m) = phase(|| drop(inferred));
    phases.push(("inference-drop", m));
    #[cfg(feature = "alloc-meter")]
    {
        assert_eq!(m.memory.live_end, inference.memory.live_start);
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
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"firings\":{n},\"reuse\":{reuse},\"meter\":{},\"phases\":[{records}]}}",
        cfg!(feature = "alloc-meter")
    );
}
