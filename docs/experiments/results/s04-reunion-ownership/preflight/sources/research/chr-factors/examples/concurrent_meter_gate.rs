//! Concurrent regional accounting diagnostic; no timing comparisons.
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
use cases::{expected, query, source};
fn begin(phased: bool) -> Option<meter::Start> {
    phased.then(meter::begin)
}
fn finish(
    start: Option<meter::Start>,
    phase: &'static str,
    cycle: usize,
    query: usize,
    records: &mut Vec<(&'static str, usize, usize, meter::Reading)>,
) {
    if let Some(start) = start {
        records.push((phase, cycle, query, meter::end(start)));
    }
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let n: usize = args[1].parse().unwrap();
    let quantum: usize = args[2].parse().unwrap();
    let phased = args[3] == "phased";
    let mode = if n == 0 {
        regions::Mode::Inline
    } else {
        regions::Mode::Threads(n)
    };
    let rules = source();
    let cases = [
        (4, 32, false, false),
        (4, 128, false, true),
        (4, 8, true, false),
        (1, 0, false, false),
        (2, 8, false, false),
    ]
    .map(|(n, d, s, cancel)| (query(n, d, s), expected(n), cancel, n));
    let mut records = Vec::with_capacity(4 * 30);
    let mut totals = Vec::with_capacity(4);
    println!("{{\"workers\":{n},\"quantum\":{quantum},\"phased\":{phased}}}");
    let init = meter::begin();
    let (sender, receiver) = std::sync::mpsc::sync_channel::<()>(1);
    assert!(
        receiver
            .recv_timeout(std::time::Duration::from_millis(1))
            .is_err()
    );
    drop(receiver);
    drop(sender);
    let context = meter::end(init);
    for cycle in 0..4 {
        let total = meter::begin();
        let start = begin(phased);
        let mut runtime = regions::Runtime::new(rules.clone(), mode, quantum, 4).unwrap();
        finish(start, "prepare", cycle, 0, &mut records);
        for (i, (q, want, cancel, count)) in cases.iter().enumerate() {
            let start = begin(phased);
            let mut session = runtime.start(q.clone()).unwrap();
            finish(start, "setup", cycle, i, &mut records);
            assert_eq!(session.factor_count(), *count);
            let _ = &session.certificate;
            let start = begin(phased);
            let mut actual = Vec::new();
            for _ in 0..if *cancel { 1 } else { 100_000 } {
                let batch = session.advance(1).unwrap();
                actual.extend(batch.answers);
                if batch.exhausted {
                    break;
                }
            }
            finish(start, "execute-observe", cycle, i, &mut records);
            if *cancel {
                assert!(!session.exhausted());
                assert_eq!(session.raw_count(), None);
                assert!(actual.is_empty());
            } else {
                assert!(session.exhausted());
                assert_eq!(session.raw_count(), Some(want.len() as u128));
                actual.sort_unstable_by(|a, b| a.outputs.cmp(&b.outputs));
                assert_eq!(&actual, want);
            }
            let start = begin(phased);
            session.close().unwrap();
            finish(start, "close", cycle, i, &mut records);
            let start = begin(phased);
            drop(session);
            finish(start, "session-drop", cycle, i, &mut records);
            let start = begin(phased);
            drop(actual);
            finish(start, "answer-drop", cycle, i, &mut records);
        }
        let start = begin(phased);
        runtime.shutdown().unwrap();
        finish(start, "shutdown", cycle, 0, &mut records);
        let start = begin(phased);
        drop(runtime);
        finish(start, "runtime-drop", cycle, 0, &mut records);
        let reading = meter::end(total);
        assert_eq!(reading.live_start, reading.live_end);
        totals.push(reading);
    }
    println!("{{\"context\":{}}}", context.json());
    for (cycle, total) in totals.into_iter().enumerate() {
        println!("{{\"cycle\":{cycle},\"total\":{}}}", total.json());
    }
    for (phase, cycle, query, reading) in records {
        println!(
            "{{\"phase\":\"{phase}\",\"cycle\":{cycle},\"query\":{query},\"reading\":{}}}",
            reading.json()
        );
    }
}
