//! Allocation-only owner probe; no elapsed-time comparison.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "support/stream_run.rs"]
mod runtime;
#[path = "support/stream_source.rs"]
mod source;
use chr_compiled::experiment::meter;
use runtime::{Event, MODES, Prepared};
use source::Schema;
use std::collections::VecDeque;

struct Snapshot {
    query: usize,
    label: &'static str,
    answers: usize,
    retained: usize,
    baseline: usize,
    reading: meter::Reading,
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.get(1).is_some_and(|x| x == "meter-check") {
        meter::self_check().unwrap();
        println!("meter self-check passed");
        return;
    }
    if chr_reuse::continuations::COLLECT_METRICS
        || chr_direct_choice::demand::COLLECT_WORK_DIAGNOSTICS
        || chr_compiled::COLLECT_METRICS
        || chr_compiled::COLLECT_KERNEL_METRICS
        || chr_observe::COLLECT_METRICS
    {
        panic!("allocation probe requires counters disabled");
    }
    assert_eq!(args.len(), 7, "mode family depth resource keep cancel");
    let mode = args[1].as_str();
    assert!(MODES.contains(&mode));
    let family = match args[2].as_str() {
        "repeated" => "repeated",
        "distinct" => "distinct",
        "aliases" => "aliases",
        _ => panic!("family"),
    };
    let n = args[3].parse::<usize>().unwrap();
    assert!(matches!(n, 16 | 64));
    let resource = match args[4].as_str() {
        "0" => false,
        "1" => true,
        _ => panic!("resource"),
    };
    let keep = match args[5].as_str() {
        "0" => 0,
        "4" => 4,
        "all" => usize::MAX,
        _ => panic!("consumer"),
    };
    let cancel = match args[6].as_str() {
        "0" => false,
        "1" => true,
        _ => panic!("cancel"),
    };
    let schema = Schema {
        family,
        resource,
        fail_tail: false,
        work: 4,
        payload: 8,
    };
    // Complete independent replay ends before the measured owners exist.
    {
        let p = Prepared::new(mode, schema, schema.rules());
        for q in 0..2 {
            let input = schema.query(n + q, q == 1);
            let expected = oracle::run(&schema.rules(), &input, 2_000_000);
            oracle::same_raw(expected.clone(), schema.expected(n + q));
            let mut run = p.start(input);
            let mut answers = vec![];
            let mut done = false;
            for _ in 0..2_000_000 {
                match run.tick() {
                    Event::Answer(a) => answers.push(a),
                    Event::Done => {
                        done = true;
                        break;
                    }
                    Event::Progress => (),
                }
            }
            assert!(done);
            oracle::same_raw(answers, expected);
        }
    }
    // Fixed harness storage is identical for all consumers at this source size.
    let mut consumer = VecDeque::with_capacity(n + 2);
    let mut snapshots = Vec::with_capacity(32);
    println!("{{\"event\":\"start\"}}");
    let root = meter::begin();
    let baseline = meter::end(root).live_end;
    let p = Prepared::new(mode, schema, schema.rules());
    snapshots.push(Snapshot {
        query: 0,
        label: "prepared",
        answers: 0,
        retained: 0,
        baseline,
        reading: meter::end(root),
    });
    for q in 0..2 {
        let baseline = meter::end(root).live_end;
        let mut run = p.start(schema.query(n + q, q == 1));
        snapshots.push(Snapshot {
            query: q,
            label: "setup",
            answers: 0,
            retained: 0,
            baseline,
            reading: meter::end(root),
        });
        let mut answers = 0;
        let mut complete = false;
        for _ in 0..2_000_000 {
            match run.tick() {
                Event::Answer(a) => {
                    answers += 1;
                    consumer.push_back(a);
                    if consumer.len() > keep {
                        consumer.pop_front();
                    }
                    if [1, 4, 16, 64].contains(&answers) {
                        snapshots.push(Snapshot {
                            query: q,
                            label: "delivery",
                            answers,
                            retained: consumer.len(),
                            baseline,
                            reading: meter::end(root),
                        });
                    }
                    if cancel && q == 0 && answers == 4 {
                        break;
                    }
                }
                Event::Done => {
                    complete = true;
                    break;
                }
                Event::Progress => (),
            }
        }
        assert_eq!(complete, !(cancel && q == 0));
        assert_eq!(answers, if cancel && q == 0 { 4 } else { n + q + 1 });
        snapshots.push(Snapshot {
            query: q,
            label: if complete { "exhausted" } else { "cancelled" },
            answers,
            retained: consumer.len(),
            baseline,
            reading: meter::end(root),
        });
        consumer.clear();
        snapshots.push(Snapshot {
            query: q,
            label: "consumer-released",
            answers,
            retained: 0,
            baseline,
            reading: meter::end(root),
        });
        drop(run);
        let reading = meter::end(root);
        assert_eq!(
            reading.live_end, baseline,
            "query owners remain after disposal"
        );
        snapshots.push(Snapshot {
            query: q,
            label: "engine-disposed",
            answers,
            retained: 0,
            baseline,
            reading,
        });
    }
    drop(p);
    let restored = meter::end(root);
    assert_eq!(
        restored.live_start, restored.live_end,
        "prepared owners remain"
    );
    let rows = snapshots.iter().map(|s|format!("{{\"query\":{},\"label\":\"{}\",\"answers\":{},\"retained\":{},\"baseline\":{},\"memory\":{}}}",s.query,s.label,s.answers,s.retained,s.baseline,s.reading.json())).collect::<Vec<_>>().join(",");
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"depth\":{n},\"resource\":{resource},\"keep\":\"{}\",\"cancel\":{cancel},\"counters\":false,\"meter\":true,\"snapshots\":[{rows}],\"restored\":{}}}",
        args[5],
        restored.json()
    );
}
