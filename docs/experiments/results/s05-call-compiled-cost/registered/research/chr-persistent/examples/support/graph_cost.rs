//! Full synchronous lifecycle; validation excluded, ordinary deliveries retained.
use crate::graph_session::{Policy, Session, Snapshot};
use std::time::Instant;
#[derive(Clone, Copy, Default)]
pub struct Reading {
    pub calls: u64,
    pub requested: u64,
    pub live: u64,
    pub peak: u64,
}
pub trait Metering {
    const ENABLED: bool;
    fn start() -> Reading;
    fn read() -> Reading;
}
const SOURCE: [&str; 19] = [
    "steps",
    "applications",
    "introductions",
    "equations",
    "pairs",
    "dereferences",
    "occurs_visits",
    "head_candidates",
    "splits",
    "failed",
    "completed",
    "duplicates",
    "machine_frontier",
    "term_nodes",
    "term_requests",
    "pending_allocations",
    "map_visits",
    "map_allocations",
    "snapshot_copies",
];
const OBS: [&str; 6] = [
    "term_pairs",
    "occurrence_scans",
    "occurrence_candidates",
    "backtracks",
    "dereferences",
    "binding_visits",
];
fn header() {
    print!("case\tpolicy\tmetered\texpected_answers\tvalidated");
    for name in [
        "prepare_ns",
        "first_ns",
        "cold_ns",
        "index_drop_ns",
        "engine_drop_ns",
        "validation_ns",
        "output_drop_ns",
    ] {
        print!("\t{name}");
    }
    for phase in [
        "before",
        "prepared",
        "first",
        "done",
        "index_dropped",
        "engine_dropped",
        "output_drop_before",
        "output_dropped",
    ] {
        for field in ["calls", "requested", "live", "peak"] {
            print!("\t{phase}_{field}");
        }
    }
    for phase in ["prepared", "first", "done"] {
        for name in SOURCE {
            print!("\t{phase}_source_{name}");
        }
        for name in ["answers", "dereferences", "map_visits"] {
            print!("\t{phase}_eager_export_{name}");
        }
        for name in ["snapshots", "residual_occurrences", "store_visits"] {
            print!("\t{phase}_capture_{name}");
        }
        for name in &OBS[..4] {
            print!("\t{phase}_eager_compare_{name}");
        }
        for kind in ["graph_compare", "graph_export"] {
            for name in OBS {
                print!("\t{phase}_{kind}_{name}");
            }
        }
        for name in [
            "raw",
            "exports",
            "deliveries",
            "index_keys",
            "frontier",
            "queue_peak",
        ] {
            print!("\t{phase}_{name}");
        }
    }
    println!();
}
fn snapshot(s: &Snapshot) {
    for values in [
        &s.source[..],
        &s.eager_export[..],
        &s.capture[..],
        &s.eager_compare[..],
        &s.graph_compare[..],
        &s.graph_export[..],
    ] {
        for value in values {
            print!("\t{value}");
        }
    }
    for value in [
        s.raw_completions,
        s.exports,
        s.deliveries as u64,
        s.index_keys as u64,
        s.frontier as u64,
        s.queue_peak as u64,
    ] {
        print!("\t{value}");
    }
}
pub fn main<M: Metering>() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    assert_eq!(args.len(), 2, "case policy required");
    let id = &args[0];
    let policy = Policy::parse(&args[1]).unwrap();
    assert!(crate::graph_cost_cases::ids().contains(id));
    let expected = crate::graph_cost_cases::expected(id);
    let mut memory = [Reading::default(); 8];
    memory[0] = M::start();
    let start = Instant::now();
    let (rules, query) = crate::graph_cost_cases::build_source(id);
    let mut session = Session::new(rules, query, policy).unwrap();
    let prepared_ns = start.elapsed().as_nanos();
    let prepared = session.snapshot();
    memory[1] = M::read();
    let mut first = None;
    for _ in 0..100_000 {
        if session.exhausted() {
            break;
        }
        let delivered = session.step().unwrap();
        if delivered && first.is_none() {
            let when = start.elapsed().as_nanos();
            let snap = session.snapshot();
            memory[2] = M::read();
            first = Some((when, snap));
        }
    }
    let cold_ns = start.elapsed().as_nanos();
    let done = session.snapshot();
    memory[3] = M::read();
    assert!(session.exhausted(), "registered source budget reached");
    let (first_ns, first) = first.expect("fixture must deliver");
    let Session {
        engine,
        index,
        deliveries,
        counters: _,
    } = session;
    let drop_start = Instant::now();
    drop(index);
    let index_ns = drop_start.elapsed().as_nanos();
    memory[4] = M::read();
    let drop_start = Instant::now();
    drop(engine);
    let engine_ns = drop_start.elapsed().as_nanos();
    memory[5] = M::read();
    let validation_start = Instant::now();
    assert_eq!(done.raw_completions, 32);
    assert_eq!(done.deliveries, expected.len());
    assert_eq!(deliveries.len(), expected.len());
    for wanted in &expected {
        assert!(deliveries.iter().any(|got| chr_observe::equivalent(
            got,
            wanted,
            &mut chr_observe::Stats::default()
        )));
    }
    assert_eq!(
        done.exports,
        if policy == Policy::GraphCompare {
            expected.len() as u64
        } else {
            32
        }
    );
    assert_eq!(
        done.index_keys,
        if matches!(policy, Policy::EagerCompare | Policy::EagerGraphCompare) {
            0
        } else {
            expected.len()
        }
    );
    let validation_ns = validation_start.elapsed().as_nanos();
    memory[6] = M::read();
    let drop_start = Instant::now();
    drop(deliveries);
    let output_ns = drop_start.elapsed().as_nanos();
    memory[7] = M::read();
    if M::ENABLED {
        assert_eq!(
            memory[7].live, memory[0].live,
            "all measured source/index/output ownership must be released"
        );
        assert_eq!(
            memory[5].live, memory[6].live,
            "oracle temporaries must not remain retained"
        );
    }
    header();
    print!(
        "{id}\t{}\t{}\t{}\t1",
        args[1],
        u8::from(M::ENABLED),
        expected.len()
    );
    for value in [
        prepared_ns,
        first_ns,
        cold_ns,
        index_ns,
        engine_ns,
        validation_ns,
        output_ns,
    ] {
        print!("\t{value}");
    }
    for reading in memory {
        for value in [reading.calls, reading.requested, reading.live, reading.peak] {
            print!("\t{value}");
        }
    }
    for s in [prepared, first, done] {
        snapshot(&s);
    }
    println!();
}
