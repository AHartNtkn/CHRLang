#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
// Reuse the source constructors and independent observers of the semantic gate.
#[allow(dead_code, unused_imports)]
mod gate {
    include!("../tests/finite_learning_gate.rs");
    #[cfg(feature = "alloc-meter")]
    use super::meter;
    use std::time::Instant;
    struct Phase {
        name: &'static str,
        query: usize,
        ns: u128,
        #[cfg(feature = "alloc-meter")]
        reading: meter::Reading,
    }
    fn mark<T>(
        records: &mut Vec<Phase>,
        query: usize,
        name: &'static str,
        f: impl FnOnce() -> T,
    ) -> T {
        assert!(records.len() < records.capacity());
        #[cfg(feature = "alloc-meter")]
        let memory_start = meter::begin();
        let start = Instant::now();
        let value = f();
        let ns = start.elapsed().as_nanos();
        #[cfg(feature = "alloc-meter")]
        let reading = meter::end(memory_start);
        records.push(Phase {
            name,
            query,
            ns,
            #[cfg(feature = "alloc-meter")]
            reading,
        });
        value
    }
    pub fn run() {
        let args: Vec<String> = std::env::args().collect();
        if args.get(1).map(String::as_str) == Some("meter-check") {
            #[cfg(feature = "alloc-meter")]
            {
                meter::self_check().unwrap();
                println!("meter-check passed");
                return;
            }
            #[cfg(not(feature = "alloc-meter"))]
            panic!("meter not enabled");
        }
        assert_eq!(args.len(), 7);
        let cancel = args[6].as_str();
        assert!(["none", "step1"].contains(&cancel));
        if chr_compiled::COLLECT_METRICS
            || chr_observe::COLLECT_METRICS
            || cfg!(feature = "metrics")
        {
            panic!("measurement requires engine metrics disabled");
        }
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
        let mut first_answers = Vec::with_capacity(count + 1);
        #[cfg(feature = "alloc-meter")]
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
                phase::learning::Learner::<{ cfg!(feature = "learning-diagnostics") }>::new(
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
            let query_start = Instant::now();
            let mut first = None;
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
            let retained_before = learner.as_ref().map_or(0, |l| l.retained());
            let installed_before = learner.as_ref().map_or(0, |l| l.stats().learned_regions);
            let report = if let Some(learner) = learner.as_mut() {
                let mut session = mark(&mut records, index, "query_setup", || {
                    learner.start(&q, Limits::default()).unwrap()
                });
                let report = mark(&mut records, index, "finite_service", || {
                    if index == 1 && cancel == "step1" {
                        assert!(matches!(session.advance().unwrap(), phase::Event::Progress));
                        None
                    } else {
                        loop {
                            match session.advance().unwrap() {
                                phase::Event::Progress => (),
                                phase::Event::Complete(r) => break Some(r),
                                phase::Event::Exhausted => panic!("missing result"),
                            }
                        }
                    }
                });
                mark(&mut records, index, "session_drop", || drop(session));
                report
            } else {
                let mut machine = mark(&mut records, index, "query_setup", || {
                    prepared.start(&q, Limits::default()).unwrap()
                });
                let report = mark(&mut records, index, "finite_service", || {
                    if index == 1 && cancel == "step1" {
                        assert!(matches!(machine.advance().unwrap(), phase::Event::Progress));
                        None
                    } else {
                        loop {
                            match machine.advance().unwrap() {
                                phase::Event::Progress => (),
                                phase::Event::Complete(r) => break Some(r),
                                phase::Event::Exhausted => panic!("missing result"),
                            }
                        }
                    }
                });
                mark(&mut records, index, "session_drop", || drop(machine));
                report
            };
            if index == 1 && cancel == "step1" {
                assert_eq!(
                    learner.as_ref().map_or(0, |l| l.stats().learned_regions),
                    installed_before
                );
                assert_eq!(
                    learner.as_ref().map_or(0, |l| l.retained()),
                    retained_before
                );
            }
            let answers = mark(
                &mut records,
                index,
                "caller_transport_execute_observe_drop",
                || {
                    let mut answers = Vec::new();
                    let mut steps = 0;
                    for solution in report.into_iter().flat_map(|r| r.solutions) {
                        let (query, weight) = caller.transport(solution);
                        let mut run = caller.start(query, weight);
                        loop {
                            steps += 1;
                            assert!(steps <= 200000);
                            match run.step() {
                                bridge::Event::Progress => (),
                                bridge::Event::Answer(a) => {
                                    answers.push(a);
                                    if first.is_none() {
                                        first = Some(query_start.elapsed().as_nanos());
                                    }
                                }
                                bridge::Event::Exhausted => break,
                            }
                        }
                    }
                    answers
                },
            );
            outputs.push(answers);
            first_answers.push(first);
            mark(&mut records, index, "input_drop", || drop(q));
        }
        let retained = learner.as_ref().map_or(0, |l| l.retained());
        mark(&mut records, 0, "learner_drop", || drop(learner));
        mark(&mut records, 0, "prepared_drop", || drop(prepared));
        mark(&mut records, 0, "caller_prepared_drop", || drop(caller));
        // Independent observation validation is outside allocation intervals.
        #[cfg(feature = "alloc-meter")]
        let validation_live = meter::end(meter::begin()).live_end;
        for (index, answers) in outputs.iter().enumerate() {
            if index == 1 && cancel == "step1" {
                assert!(answers.is_empty());
                continue;
            }
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
        #[cfg(feature = "alloc-meter")]
        assert_eq!(meter::end(meter::begin()).live_end, validation_live);
        let answer_count: usize = outputs.iter().map(Vec::len).sum();
        // Keep the reserved outer consumer storage outside the measured ownership.
        mark(&mut records, 0, "consumer_drop", || outputs.clear());
        mark(&mut records, 0, "source_drop", || drop(source));
        #[cfg(feature = "alloc-meter")]
        assert_eq!(
            meter::end(meter::begin()).live_end,
            baseline,
            "task-owned heap remains"
        );
        #[cfg(feature = "alloc-meter")]
        let baseline = baseline.to_string();
        #[cfg(not(feature = "alloc-meter"))]
        let baseline = "null";
        let first_json = first_answers
            .iter()
            .map(|t| t.map_or("null".into(), |n| n.to_string()))
            .collect::<Vec<_>>()
            .join(",");
        println!(
            "{{\"mode\":\"{mode}\",\"accepted\":{accepted},\"weight\":{weight},\"capacity\":{capacity},\"queries\":{},\"cancel\":\"{cancel}\",\"answers\":{answer_count},\"retained_regions\":{retained},\"diagnostics\":{},\"allocation_meter\":{},\"baseline\":{baseline},\"first_owned_ns\":[{first_json}],\"phases\":[{}]}}",
            count + 1,
            cfg!(feature = "learning-diagnostics"),
            cfg!(feature = "alloc-meter"),
            records
                .iter()
                .map(|r| {
                    #[cfg(feature = "alloc-meter")]
                    let memory = r.reading.json();
                    #[cfg(not(feature = "alloc-meter"))]
                    let memory = "null";
                    format!(
                        "{{\"name\":\"{}\",\"query\":{},\"ns\":{},\"memory\":{memory}}}",
                        r.name, r.query, r.ns
                    )
                })
                .collect::<Vec<_>>()
                .join(",")
        );
    }
}
fn main() {
    gate::run();
}
