#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../examples/support/stream_run.rs"]
mod runtime;
#[path = "../examples/support/stream_source.rs"]
mod source;
use runtime::{Event, MODES, Prepared};
use source::Schema;

#[test]
fn progressive_source_has_independent_complete_answers() {
    for family in ["repeated", "distinct", "aliases"] {
        for resource in [false, true] {
            for fail_tail in [false, true] {
                for n in [0, 1, 4, 16] {
                    for work in [0, 3] {
                        for payload in [0, 4] {
                            let schema = Schema {
                                family,
                                resource,
                                fail_tail,
                                work,
                                payload,
                            };
                            for reverse in [false, true] {
                                let answers = oracle::run(
                                    &schema.rules(),
                                    &schema.query(n, reverse),
                                    200_000,
                                );
                                oracle::same_raw(answers, schema.expected(n));
                                for mode in MODES {
                                    let prepared = Prepared::new(mode, schema, schema.rules());
                                    let mut run = prepared.start(schema.query(n, reverse));
                                    let mut actual = vec![];
                                    let mut done = false;
                                    for _ in 0..200_000 {
                                        match run.tick() {
                                            Event::Answer(a) => actual.push(a),
                                            Event::Done => {
                                                done = true;
                                                break;
                                            }
                                            Event::Progress => (),
                                        }
                                    }
                                    assert!(done, "{mode} {family} {n}");
                                    oracle::same_raw(actual, schema.expected(n));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn consumer_release_and_cancellation_preserve_valid_delivery() {
    for family in ["repeated", "distinct", "aliases"] {
        let schema = Schema {
            family,
            resource: true,
            fail_tail: true,
            work: 3,
            payload: 4,
        };
        let expected = oracle::run(&schema.rules(), &schema.query(16, false), 200_000);
        for mode in MODES {
            let p = Prepared::new(mode, schema, schema.rules());
            for keep in [0, 4, usize::MAX] {
                for stop in [Some(4), None] {
                    let mut run = p.start(schema.query(16, false));
                    let mut remaining = expected.clone();
                    let mut consumer = std::collections::VecDeque::new();
                    let mut count = 0;
                    let mut complete = false;
                    for _ in 0..200_000 {
                        match run.tick() {
                            Event::Answer(a) => {
                                let at = remaining
                                    .iter()
                                    .position(|e| {
                                        chr_observe::equivalent(e, &a, &mut Default::default())
                                    })
                                    .expect("invalid raw answer or multiplicity");
                                remaining.swap_remove(at);
                                count += 1;
                                consumer.push_back(a);
                                if consumer.len() > keep {
                                    consumer.pop_front();
                                }
                                if stop == Some(count) {
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
                    assert_eq!(complete, stop.is_none());
                    assert_eq!(count, stop.unwrap_or(16));
                    assert_eq!(consumer.len(), count.min(keep));
                    drop(consumer);
                    drop(run);
                }
            }
        }
    }
}
