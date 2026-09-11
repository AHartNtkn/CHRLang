//! Complete serial source organizations for comparison with the worker pilot.
#[path = "worker_cases.rs"]
mod cases;
use chr_compiled::{Access, Policy, PreparedRuleset, search::SearchEvent};
use std::time::Instant;
pub trait Meter {
    type Start;
    fn begin() -> Self::Start;
    fn end(start: Self::Start) -> Option<String>;
}
pub fn run<M: Meter>() {
    assert!(!std::hint::black_box(chr_compiled::COLLECT_METRICS));
    let args = std::env::args().collect::<Vec<_>>();
    let mode = args[1].as_str();
    assert!(matches!(mode, "specialized" | "contracted"));
    let depth: usize = args[2].parse().unwrap();
    let queries: usize = args[3].parse().unwrap();
    let family = args[4].as_str();
    assert!(matches!(family, "balanced" | "skew" | "tiny"));
    let count = if family == "tiny" { 1 } else { 4 };
    let expected = cases::expected(count);
    let mut samples = Vec::with_capacity(queries);
    println!(
        "{{\"mode\":\"{mode}\",\"depth\":{depth},\"queries\":{queries},\"family\":\"{family}\"}}"
    );
    let allocation = M::begin();
    let start = Instant::now();
    let source = PreparedRuleset::new(cases::source(), None).unwrap();
    let specialized = source.specialize_inferred();
    let program = if mode == "contracted" {
        let program = specialized.contract_carriers_inferred().unwrap();
        drop(specialized);
        program
    } else {
        specialized
    };
    drop(source);
    let prepare = start.elapsed().as_nanos();
    for query in 0..queries {
        let start = Instant::now();
        let mut search = program
            .start_search(
                cases::query(count, depth + query % 3, family == "skew"),
                Policy::Global,
                Access::Scan,
            )
            .unwrap();
        let setup = start.elapsed().as_nanos();
        let start = Instant::now();
        let mut actual = Vec::new();
        let mut first = None;
        let mut exhausted = false;
        for _ in 0..10_000_000 {
            match search.tick() {
                SearchEvent::Complete(mut branch) => {
                    let answer = branch.engine.observe().unwrap();
                    if first.is_none() {
                        first = Some(start.elapsed().as_nanos());
                    }
                    actual.push(answer);
                }
                SearchEvent::Exhausted => {
                    exhausted = true;
                    break;
                }
                SearchEvent::Failed(_) => panic!("unexpected failed branch"),
                _ => {}
            }
        }
        let execute_observe = start.elapsed().as_nanos();
        assert!(exhausted);
        // Every raw branch contributes exactly one distinct expected tuple here.
        actual.sort_unstable_by(|a, b| a.outputs.cmp(&b.outputs));
        assert_eq!(actual, expected);
        let start = Instant::now();
        drop(search);
        drop(actual);
        let dispose = start.elapsed().as_nanos();
        samples.push((setup, execute_observe, first.unwrap(), dispose));
    }
    let start = Instant::now();
    drop(program);
    let shutdown = start.elapsed().as_nanos();
    let allocation = M::end(allocation);
    let lifecycle = prepare + shutdown + samples.iter().map(|s| s.0 + s.1 + s.3).sum::<u128>();
    println!(
        "{{\"prepare_ns\":{prepare},\"shutdown_ns\":{shutdown},\"lifecycle_ns\":{lifecycle}}}"
    );
    for (query, (setup, execute_observe, first, dispose)) in samples.into_iter().enumerate() {
        println!(
            "{{\"query\":{query},\"setup_ns\":{setup},\"execute_observe_ns\":{execute_observe},\"first_observation_ns\":{first},\"dispose_ns\":{dispose}}}"
        );
    }
    if let Some(allocation) = allocation {
        println!("{{\"allocation\":{allocation}}}");
    }
}
