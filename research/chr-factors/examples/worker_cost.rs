//! Ordinary-allocator sizing for the reusable-worker comparison.
#[path = "../experiments/worker_cases.rs"]
mod cases;
#[path = "../experiments/reusable_regions.rs"]
mod regions;
#[allow(dead_code)]
#[path = "../experiments/reusable_workers.rs"]
mod workers;
use std::time::Instant;
fn main() {
    assert!(!std::hint::black_box(chr_factors::COLLECT_METRICS));
    assert!(!std::hint::black_box(chr_persistent::COLLECT_METRICS));
    let args = std::env::args().collect::<Vec<_>>();
    let workers: usize = args[1].parse().unwrap();
    let depth: usize = args[2].parse().unwrap();
    let repetitions: usize = args[3].parse().unwrap();
    let quantum: usize = args[4].parse().unwrap();
    let family = args[5].as_str();
    let count = if family == "tiny" { 1 } else { 4 };
    assert!(matches!(family, "balanced" | "skew" | "tiny"));
    let expected = cases::expected(count);
    let mut samples = Vec::with_capacity(repetitions);
    let mode = if workers == 0 {
        regions::Mode::Inline
    } else {
        regions::Mode::Threads(workers)
    };
    let start = Instant::now();
    let mut runtime = regions::Runtime::new(cases::source(), mode, quantum, 4).unwrap();
    let prepare = start.elapsed().as_nanos();
    for query in 0..repetitions {
        let start = Instant::now();
        let q = cases::query(count, depth + query % 3, family == "skew");
        let mut session = runtime.start(q).unwrap();
        let setup = start.elapsed().as_nanos();
        assert_eq!(session.factor_count(), count);
        let _ = &session.certificate;
        let start = Instant::now();
        let mut actual = Vec::new();
        let mut first = None;
        for _ in 0..10_000_000 {
            let batch = session.advance(1).unwrap();
            if first.is_none() && !batch.answers.is_empty() {
                first = Some(start.elapsed().as_nanos());
            }
            actual.extend(batch.answers);
            if batch.exhausted {
                break;
            }
        }
        let execute_observe = start.elapsed().as_nanos();
        assert!(session.exhausted());
        assert_eq!(session.raw_count(), Some(expected.len() as u128));
        actual.sort_unstable_by(|a, b| a.outputs.cmp(&b.outputs));
        assert_eq!(actual, expected);
        let start = Instant::now();
        session.close().unwrap();
        let close = start.elapsed().as_nanos();
        let start = Instant::now();
        drop(session);
        drop(actual);
        let dispose = start.elapsed().as_nanos();
        samples.push((setup, execute_observe, first.unwrap(), close, dispose));
    }
    let start = Instant::now();
    runtime.shutdown().unwrap();
    let shutdown = start.elapsed().as_nanos();
    let start = Instant::now();
    drop(runtime);
    let runtime_drop = start.elapsed().as_nanos();
    let lifecycle = prepare
        + shutdown
        + runtime_drop
        + samples.iter().map(|s| s.0 + s.1 + s.3 + s.4).sum::<u128>();
    println!(
        "{{\"workers\":{workers},\"depth\":{depth},\"queries\":{repetitions},\"quantum\":{quantum},\"family\":\"{family}\",\"prepare_ns\":{prepare},\"shutdown_ns\":{shutdown},\"runtime_drop_ns\":{runtime_drop},\"lifecycle_ns\":{lifecycle}}}"
    );
    for (query, (setup, execute_observe, first, close, dispose)) in samples.into_iter().enumerate()
    {
        println!(
            "{{\"query\":{query},\"setup_ns\":{setup},\"execute_observe_ns\":{execute_observe},\"first_observation_ns\":{first},\"close_ns\":{close},\"dispose_ns\":{dispose}}}"
        );
    }
}
