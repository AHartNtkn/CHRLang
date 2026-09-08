//! Matched source/runtime lifecycle, isolated requested-heap diagnostics.
#[path = "../experiments/worker_cases.rs"]
mod cases;
#[allow(dead_code, unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[path = "../experiments/reusable_regions.rs"]
mod regions;
#[allow(dead_code)]
#[path = "../experiments/reusable_workers.rs"]
mod workers;
fn main() {
    assert!(!std::hint::black_box(chr_factors::COLLECT_METRICS));
    assert!(!std::hint::black_box(chr_persistent::COLLECT_METRICS));
    let args = std::env::args().collect::<Vec<_>>();
    let workers: usize = args[1].parse().unwrap();
    let depth: usize = args[2].parse().unwrap();
    let repetitions: usize = args[3].parse().unwrap();
    let quantum: usize = args[4].parse().unwrap();
    let family = args[5].as_str();
    assert!(matches!(family, "balanced" | "skew" | "tiny"));
    let count = if family == "tiny" { 1 } else { 4 };
    let expected = cases::expected(count);
    let mode = if workers == 0 {
        regions::Mode::Inline
    } else {
        regions::Mode::Threads(workers)
    };
    println!(
        "{{\"workers\":{workers},\"depth\":{depth},\"queries\":{repetitions},\"quantum\":{quantum},\"family\":\"{family}\"}}"
    );
    // Isolate known process-local channel context before the runtime window.
    let (tx, rx) = std::sync::mpsc::sync_channel::<()>(1);
    assert!(
        rx.recv_timeout(std::time::Duration::from_millis(1))
            .is_err()
    );
    drop(rx);
    drop(tx);
    let start = meter::begin();
    let mut runtime = regions::Runtime::new(cases::source(), mode, quantum, 4).unwrap();
    for query in 0..repetitions {
        let q = cases::query(count, depth + query % 3, family == "skew");
        let mut session = runtime.start(q).unwrap();
        assert_eq!(session.factor_count(), count);
        let _ = &session.certificate;
        let mut actual = Vec::new();
        for _ in 0..10_000_000 {
            let batch = session.advance(1).unwrap();
            actual.extend(batch.answers);
            if batch.exhausted {
                break;
            }
        }
        assert!(session.exhausted());
        assert_eq!(session.raw_count(), Some(expected.len() as u128));
        actual.sort_unstable_by(|a, b| a.outputs.cmp(&b.outputs));
        assert_eq!(actual, expected);
        session.close().unwrap();
        drop(session);
        drop(actual);
    }
    runtime.shutdown().unwrap();
    drop(runtime);
    let reading = meter::end(start);
    assert_eq!(reading.live_start, reading.live_end);
    println!("{}", reading.json());
}
