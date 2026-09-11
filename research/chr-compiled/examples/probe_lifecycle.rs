//! Full native lifecycle, with independent ground-answer expectations.
#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use chr_compiled::{Access, Policy, PreparedRuleset};
use chr_syntax::{Constraint, Query, Rule, atom, c, v};
use std::time::Instant;
#[cfg(feature = "lifecycle-cpu-clock")]
fn thread_cpu() -> u128 {
    #[repr(C)]
    struct Timespec {
        sec: std::ffi::c_long,
        nsec: std::ffi::c_long,
    }
    unsafe extern "C" {
        fn clock_gettime(clock: std::ffi::c_int, out: *mut Timespec) -> std::ffi::c_int;
    }
    let mut out = Timespec { sec: 0, nsec: 0 };
    assert_eq!(unsafe { clock_gettime(3, &mut out) }, 0);
    out.sec as u128 * 1_000_000_000 + out.nsec as u128
}
struct Reading {
    phase: &'static str,
    q: usize,
    ns: u128,
    #[cfg(feature = "lifecycle-cpu-clock")]
    cpu_ns: u128,
    #[cfg(feature = "alloc-meter")]
    heap: meter::Reading,
}
fn measure<T>(rows: &mut Vec<Reading>, phase: &'static str, q: usize, f: impl FnOnce() -> T) -> T {
    #[cfg(feature = "alloc-meter")]
    let start = meter::begin();
    let clock = Instant::now();
    #[cfg(feature = "lifecycle-cpu-clock")]
    let cpu_start = thread_cpu();
    let result = f();
    #[cfg(feature = "lifecycle-cpu-clock")]
    let cpu_ns = thread_cpu() - cpu_start;
    let ns = clock.elapsed().as_nanos();
    #[cfg(feature = "alloc-meter")]
    let heap = meter::end(start);
    rows.push(Reading {
        phase,
        q,
        ns,
        #[cfg(feature = "lifecycle-cpu-clock")]
        cpu_ns,
        #[cfg(feature = "alloc-meter")]
        heap,
    });
    result
}
fn rules(family: &str) -> Vec<Rule> {
    let mut kept = vec![c("left", [v(0), v(1)]), c("right", [v(1), v(2)])];
    let removed = if family == "neutral" {
        vec![c("request", [v(2)])]
    } else {
        kept.insert(0, c("request", [v(2)]));
        vec![c("token", [])]
    };
    vec![Rule {
        name: "join".into(),
        kept,
        removed,
        guards: vec![],
        body: c("receipt", [v(0), v(1), v(2)]).into(),
    }]
}
fn input(family: &str, n: usize, q: usize) -> Query {
    let endpoint = if q == 0 { 0 } else { n - 1 };
    let mut facts = vec![];
    let shared = family == "duplicate";
    let broad = family == "broad";
    let key = |prefix: &str, i: usize| atom(&format!("{prefix}{i}"));
    facts.push(c("request", [key("b", endpoint)]));
    for i in 0..n {
        facts.push(c(
            "left",
            [key("a", i), key("x", if shared { 0 } else { i })],
        ));
    }
    for i in 0..if shared { n - 1 } else { n } {
        facts.push(c(
            "right",
            [
                key("x", if shared { 0 } else { i }),
                key("b", if shared || broad { endpoint } else { i }),
            ],
        ));
    }
    if family != "neutral" {
        facts.push(c("token", []));
    }
    Query {
        constraints: facts,
        outputs: vec![],
    }
}
fn expected(family: &str, n: usize, q: usize) -> Vec<Constraint> {
    let query = input(family, n, q);
    let endpoint = if q == 0 { 0 } else { n - 1 };
    let chosen = if family == "duplicate" || family == "broad" {
        0
    } else {
        endpoint
    };
    let mut result = query
        .constraints
        .into_iter()
        .filter(|r| {
            r.name
                != if family == "neutral" {
                    "request"
                } else {
                    "token"
                }
        })
        .collect::<Vec<_>>();
    result.push(c(
        "receipt",
        [
            atom(&format!("a{chosen}")),
            atom(&format!("x{chosen}")),
            atom(&format!("b{endpoint}")),
        ],
    ));
    result.sort();
    result
}
fn session(args: &[String]) {
    let family = &args[1];
    assert!(["selective", "neutral", "duplicate", "broad"].contains(&family.as_str()));
    let n: usize = args[2].parse().unwrap();
    assert!(n >= 2);
    let policy = match args[3].as_str() {
        "global" => Policy::Global,
        "active" => Policy::Active,
        _ => panic!("policy"),
    };
    let access = match args[4].as_str() {
        "scan" => Access::Scan,
        "indexed" => Access::Indexed,
        _ => panic!("access"),
    };
    let keep = args[5] == "true";
    let cancel = args[6] == "true";
    if chr_compiled::COLLECT_METRICS
        || chr_compiled::COLLECT_KERNEL_METRICS
        || chr_observe::COLLECT_METRICS
    {
        panic!("lifecycle measurements require engine, kernel and observer counters off");
    }
    let expectations = [expected(family, n, 0), expected(family, n, 1)];
    let mut rows = Vec::with_capacity(32);
    let mut retained = Vec::with_capacity(2);
    #[cfg(feature = "alloc-meter")]
    let root = meter::end(meter::begin()).live_end;
    let prepared = measure(&mut rows, "prepare", 0, || {
        PreparedRuleset::new(rules(family), None).unwrap()
    });
    for (q, want) in expectations.iter().enumerate() {
        let mut engine = measure(&mut rows, "setup", q, || {
            prepared.start(input(family, n, q), policy, access).unwrap()
        });
        let stop = cancel && q == 0;
        let status = measure(&mut rows, "execute", q, || {
            engine.advance(if stop { n / 2 } else { 2_000_000 })
        });
        let answer = if stop {
            assert!(!status.exhausted);
            None
        } else {
            assert!(status.exhausted && !status.failed && !status.pending_split);
            let mut a = measure(&mut rows, "observe", q, || engine.observe().unwrap());
            assert!(a.outputs.is_empty());
            a.residual.sort();
            assert_eq!(&a.residual, want);
            Some(a)
        };
        measure(&mut rows, "engine_drop", q, || drop(engine));
        if keep {
            if let Some(a) = answer {
                retained.push(a);
            }
            measure(&mut rows, "answer_drop", q, || ());
        } else {
            measure(&mut rows, "answer_drop", q, || drop(answer));
        }
    }
    measure(&mut rows, "prepared_drop", 0, || drop(prepared));
    // Retained answers must remain readable after both producer and preparation disposal.
    if keep {
        for (a, q) in retained
            .iter()
            .zip(if cancel { vec![1] } else { vec![0, 1] })
        {
            assert_eq!(a.residual, expectations[q]);
        }
    }
    measure(&mut rows, "retained_drop", 0, || retained.clear());
    #[cfg(feature = "alloc-meter")]
    {
        assert_eq!(meter::end(meter::begin()).live_end, root);
    }
    print!(
        "{{\"validated\":true,\"feature\":{},\"cancel\":{cancel},\"records\":[",
        cfg!(feature = "selective-probe")
    );
    for (i, r) in rows.iter().enumerate() {
        if i > 0 {
            print!(",");
        }
        #[cfg(feature = "alloc-meter")]
        let heap = r.heap.json();
        #[cfg(not(feature = "alloc-meter"))]
        let heap = "null";
        #[cfg(feature = "lifecycle-cpu-clock")]
        let cpu = r.cpu_ns.to_string();
        #[cfg(not(feature = "lifecycle-cpu-clock"))]
        let cpu = "null";
        print!(
            "{{\"phase\":\"{}\",\"q\":{},\"ns\":{},\"heap\":{heap},\"cpu_ns\":{cpu}}}",
            r.phase, r.q, r.ns
        );
    }
    println!("]}}");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let repetitions = args.get(7).map_or(1, |s| s.parse::<usize>().unwrap());
    assert!(repetitions > 0);
    for _ in 0..repetitions {
        session(&args);
    }
}
