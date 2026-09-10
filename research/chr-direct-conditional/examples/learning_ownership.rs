#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
// Reuse the source constructors and independent observers of the semantic gate.
#[allow(dead_code, unused_imports)]
mod gate {
    include!("../tests/finite_learning_gate.rs");
    use super::meter;
    struct Phase {
        name: &'static str,
        query: usize,
        reading: meter::Reading,
    }
    fn mark<T>(
        records: &mut Vec<Phase>,
        query: usize,
        name: &'static str,
        f: impl FnOnce() -> T,
    ) -> T {
        assert!(records.len() < records.capacity());
        let start = meter::begin();
        let value = f();
        let reading = meter::end(start);
        records.push(Phase {
            name,
            query,
            reading,
        });
        value
    }
    pub fn run() {
        let args: Vec<String> = std::env::args().collect();
        if args.get(1).map(String::as_str) == Some("meter-check") {
            meter::self_check().unwrap();
            println!("meter-check passed");
            return;
        }
        assert_eq!(args.len(), 6);
        let mode = args[1].as_str();
        let accepted: u16 = args[2].parse().unwrap();
        let weight: usize = args[3].parse().unwrap();
        let capacity: usize = args[4].parse().unwrap();
        let count: usize = args[5].parse().unwrap();
        assert!(["recompute", "eager", "covered"].contains(&mode));
        assert!([0, 273, 238, 511].contains(&accepted));
        assert!([1, 2].contains(&weight) && [0, 1, 4].contains(&capacity));
        assert!([1, 4, 16].contains(&count));
        let mut records = Vec::with_capacity(16 + 8 * (count + 1));
        let mut outputs = Vec::with_capacity(count + 1);
        let baseline = meter::end(meter::begin()).live_end;
        let source = mark(&mut records, 0, "source_build", || {
            matrix_rules(accepted, weight)
        });
        let prepared = mark(&mut records, 0, "prepare", || {
            Prepared::new(&source, source.len() - 1).unwrap()
        });
        let caller = mark(&mut records, 0, "caller_prepare", || {
            bridge::Bridge::new(source.clone())
        });
        let mut learner = mark(&mut records, 0, "learner_setup", || {
            (mode != "recompute").then(|| {
                Learner::new(
                    &prepared,
                    capacity,
                    if mode == "eager" {
                        Pruning::Eager
                    } else {
                        Pruning::WhenCovered
                    },
                )
            })
        });
        for index in 0..=count {
            let domain = if index == 0 {
                3
            } else if index % 2 == 1 {
                7
            } else {
                1
            };
            let q = mark(&mut records, index, "input_build", || {
                matrix_query(domain, domain, false, 10 + 10 * index as u64)
            });
            let report = if let Some(learner) = learner.as_mut() {
                let session = mark(&mut records, index, "query_setup", || {
                    learner.start(&q, Limits::default()).unwrap()
                });
                mark(&mut records, index, "execute_and_session_drop", || {
                    session.finish().unwrap()
                })
            } else {
                let machine = mark(&mut records, index, "query_setup", || {
                    prepared.start(&q, Limits::default()).unwrap()
                });
                mark(&mut records, index, "execute_and_session_drop", || {
                    machine.finish().unwrap()
                })
            };
            let answers = mark(
                &mut records,
                index,
                "caller_transport_execute_observe_drop",
                || {
                    let mut answers = Vec::new();
                    let mut steps = 0;
                    for solution in report.solutions {
                        let (query, weight) = caller.transport(solution);
                        let mut run = caller.start(query, weight);
                        loop {
                            steps += 1;
                            assert!(steps <= 200000);
                            match run.step() {
                                bridge::Event::Progress => (),
                                bridge::Event::Answer(a) => answers.push(a),
                                bridge::Event::Exhausted => break,
                            }
                        }
                    }
                    answers
                },
            );
            outputs.push(answers);
            mark(&mut records, index, "input_drop", || drop(q));
        }
        let retained = learner.as_ref().map_or(0, Learner::retained);
        mark(&mut records, 0, "learner_drop", || drop(learner));
        mark(&mut records, 0, "prepared_drop", || drop(prepared));
        mark(&mut records, 0, "caller_prepared_drop", || drop(caller));
        // Independent observation validation is outside allocation intervals.
        let validation_live = meter::end(meter::begin()).live_end;
        for (index, answers) in outputs.iter().enumerate() {
            let domain = if index == 0 {
                3
            } else if index % 2 == 1 {
                7
            } else {
                1
            };
            let q = matrix_query(domain, domain, false, 10 + 10 * index as u64);
            runtime_support::same_raw(answers.clone(), runtime_support::run(&source, &q, 200000));
            let actual = composition_support::Engine::new(0, &source, &q).collect();
            runtime_support::same_raw(answers.clone(), actual);
        }
        assert_eq!(meter::end(meter::begin()).live_end, validation_live);
        let answer_count: usize = outputs.iter().map(Vec::len).sum();
        // Keep the reserved outer consumer storage outside the measured ownership.
        mark(&mut records, 0, "consumer_drop", || outputs.clear());
        mark(&mut records, 0, "source_drop", || drop(source));
        assert_eq!(
            meter::end(meter::begin()).live_end,
            baseline,
            "task-owned heap remains"
        );
        println!(
            "{{\"mode\":\"{mode}\",\"accepted\":{accepted},\"weight\":{weight},\"capacity\":{capacity},\"queries\":{},\"answers\":{answer_count},\"retained_regions\":{retained},\"baseline\":{baseline},\"phases\":[{}]}}",
            count + 1,
            records
                .iter()
                .map(|r| format!(
                    "{{\"name\":\"{}\",\"query\":{},\"memory\":{}}}",
                    r.name,
                    r.query,
                    r.reading.json()
                ))
                .collect::<Vec<_>>()
                .join(",")
        );
    }
}
fn main() {
    gate::run();
}
