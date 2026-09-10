// Measurement reuses the qualified parser and execution types; no new engine.
#[allow(dead_code)]
mod common {
    include!("native_common_source.rs");
    use std::time::Instant;
    fn ns(start: Instant) -> u128 {
        start.elapsed().as_nanos()
    }
    fn append(
        wire: &mut answer_wire::OwnedWire,
        answer: chr_syntax::Answer,
        start: Instant,
        serialization: &mut u128,
        first: &mut Option<u128>,
    ) {
        let t = Instant::now();
        wire.push(answer);
        *serialization += ns(t);
        if first.is_none() {
            *first = Some(ns(start));
        }
    }
    pub fn lifecycle() {
        let mode: usize = std::env::args().nth(1).unwrap().parse().unwrap();
        assert!(mode <= 12);
        let limit: usize = std::env::args()
            .nth(2)
            .map_or(200000, |s| s.parse().unwrap());
        assert!(limit <= 1000000);
        eprintln!(
            "{{\"configuration\":true,\"compiled_metrics\":{},\"observe_metrics\":{},\"conditional_metrics\":{}}}",
            chr_compiled::COLLECT_METRICS,
            chr_observe::COLLECT_METRICS,
            cfg!(feature = "metrics")
        );
        let load_start = Instant::now();
        let mut text = String::new();
        io::stdin().read_to_string(&mut text).unwrap();
        let input_load_ns = ns(load_start);
        let decode_start = Instant::now();
        let mut sections = text.split("\nNEXT\n");
        let mut lines = sections.next().unwrap().lines();
        let first_constraints = constraints(lines.next().unwrap());
        let first_outputs = outputs(lines.next().unwrap());
        let rules: Vec<Rule> = lines
            .enumerate()
            .map(|(i, line)| {
                let parts: Vec<_> = line.split(';').collect();
                assert_eq!(parts.len(), 3);
                Rule {
                    name: format!("r{i}"),
                    kept: self::constraints(parts[0]),
                    removed: self::constraints(parts[1]),
                    guards: vec![],
                    body: goal(parts[2]),
                }
            })
            .collect();

        let mut queries = vec![Query {
            constraints: first_constraints,
            outputs: first_outputs,
        }];
        for section in sections {
            let mut lines = section.lines();
            queries.push(Query {
                constraints: constraints(lines.next().unwrap()),
                outputs: outputs(lines.next().unwrap()),
            });
            assert!(lines.next().is_none());
        }

        let decode_ns = ns(decode_start);
        let t = Instant::now();
        drop(text);
        let input_drop_ns = ns(t);
        let prepare_start = Instant::now();
        let compiled = if mode < 4 {
            let p = PreparedRuleset::new(rules.clone(), None).unwrap();
            Some(if mode >= 2 {
                p.specialize_inferred()
            } else {
                p
            })
        } else {
            None
        };
        let contextual = if (4..=8).contains(&mode) {
            Some(chr_relational::contextual_execute::Prepared::new(&rules).unwrap())
        } else {
            None
        };
        let conditional = if mode == 9 {
            Some(chr_direct_conditional::engine::PreparedRuleset::new(rules.clone()).unwrap())
        } else {
            None
        };
        if mode == 99 {
            for prefix in 1..=rules.len() {
                match finite_phase::Prepared::new(&rules, prefix) {
                    Ok(_) => println!("PREFIX {prefix} admitted"),
                    Err(e) => println!("PREFIX {prefix} {e:?}"),
                }
            }
            return;
        }
        let prefix = if mode == 10 {
            Some(match chr_compiled::pure_prefix::Program::new(&rules) {
                Ok(p) => p,
                Err(e) => {
                    println!("UNSUPPORTED {e}");
                    return;
                }
            })
        } else {
            None
        };
        let finite = if mode == 11 {
            let admitted = (1..=rules.len())
                .rev()
                .find_map(|n| finite_phase::Prepared::new(&rules, n).ok().map(|p| (n, p)));
            match admitted {
                Some((_, p)) => Some((p, finite_bridge::Bridge::new(rules.clone()))),
                None => {
                    println!("UNSUPPORTED no admitted finite phase");
                    return;
                }
            }
        } else {
            None
        };
        let kept = if mode == 12 {
            match finite_kept_read::Prepared::new(&rules, 1) {
                Ok(p) => Some((p, finite_bridge::Bridge::new(rules.clone()))),
                Err(e @ finite_phase::Error::Source(_)) => {
                    println!("UNSUPPORTED {e:?}");
                    return;
                }
                Err(e) => panic!("checked finite preparation incomplete: {e:?}"),
            }
        } else {
            None
        };

        let source_symbols = answer_wire::Symbols::source(&rules);
        let mut artifacts = std::collections::BTreeMap::new();
        let prepare_ns = ns(prepare_start);
        let t = Instant::now();
        let mut held = Vec::with_capacity(queries.len());
        let consumer_setup_ns = ns(t);
        let mut query_sum = 0;
        let mut query_inputs = queries.into_iter().enumerate();
        for (index, input) in query_inputs.by_ref() {
            let setup_start = Instant::now();
            let mut wire = answer_wire::OwnedWire::new(source_symbols.query(&input), vec![]);
            let mut query = Some(input);
            let mut machine = if let Some((phase, _)) = &finite {
                Some(
                    phase
                        .start(query.as_ref().unwrap(), Default::default())
                        .unwrap(),
                )
            } else if let Some((phase, _)) = &kept {
                match phase.start(query.as_ref().unwrap(), Default::default()) {
                    Ok(m) => Some(m),
                    Err(e @ finite_phase::Error::Source(_)) => {
                        println!("UNSUPPORTED {e:?}");
                        return;
                    }
                    Err(e) => panic!("incomplete admission: {e:?}"),
                }
            } else {
                None
            };
            let mut engine = if mode <= 10 {
                let engine = match mode {
                    0..=3 => Engine::Compiled(
                        compiled
                            .as_ref()
                            .unwrap()
                            .start_search(
                                query.take().unwrap(),
                                Policy::Global,
                                if mode.is_multiple_of(2) {
                                    Access::Scan
                                } else {
                                    Access::Indexed
                                },
                            )
                            .unwrap(),
                    ),
                    4 => Engine::Contextual(
                        contextual.as_ref().unwrap().start(query.as_ref().unwrap()),
                    ),
                    5 => Engine::Contextual(
                        contextual
                            .as_ref()
                            .unwrap()
                            .start_shared_deductions(query.as_ref().unwrap()),
                    ),
                    6 => Engine::Contextual(
                        contextual
                            .as_ref()
                            .unwrap()
                            .start_persistent_equality(query.as_ref().unwrap(), false),
                    ),
                    7 => Engine::Contextual(
                        contextual
                            .as_ref()
                            .unwrap()
                            .start_persistent_equality(query.as_ref().unwrap(), true),
                    ),
                    8 => Engine::Contextual(
                        contextual
                            .as_ref()
                            .unwrap()
                            .start_resumable(query.as_ref().unwrap()),
                    ),
                    9 => Engine::Conditional(
                        conditional
                            .as_ref()
                            .unwrap()
                            .start(query.take().unwrap())
                            .unwrap(),
                    ),
                    10 => {
                        let shape: Vec<_> = query
                            .as_ref()
                            .unwrap()
                            .constraints
                            .iter()
                            .map(|c| (c.name.clone(), c.args.len()))
                            .collect();
                        if !artifacts.contains_key(&shape) {
                            artifacts.insert(
                                shape.clone(),
                                prefix.as_ref().unwrap().prepare_shape(&shape).unwrap(),
                            );
                        }
                        Engine::Compiled(
                            artifacts[&shape]
                                .start(query.as_ref().unwrap(), Access::Scan)
                                .unwrap(),
                        )
                    }
                    _ => panic!("unknown mode"),
                };

                Some(engine)
            } else {
                None
            };
            drop(query);
            let setup_ns = ns(setup_start);
            let mut caller: Option<finite_bridge::Caller> = None;
            let mut solutions: Option<std::vec::IntoIter<finite_phase::Solution>> = None;
            let mut exhausted = false;
            let mut calls = 0;
            let mut serialization = 0;
            let mut first = None;
            let service_start = Instant::now();
            while calls < limit && !exhausted {
                calls += 1;
                if let Some(e) = &mut engine {
                    match e.step() {
                        Event::Progress => (),
                        Event::Answer(a) => {
                            append(&mut wire, a, service_start, &mut serialization, &mut first)
                        }
                        Event::Exhausted => exhausted = true,
                    }
                } else if let Some(m) = &mut machine {
                    match m.advance().unwrap() {
                        finite_phase::Event::Progress => (),
                        finite_phase::Event::Complete(r) => {
                            solutions = Some(r.solutions.into_iter());
                            machine = None;
                        }
                        finite_phase::Event::Exhausted => {
                            panic!("phase exhausted without admission")
                        }
                    }
                } else if let Some(c) = &mut caller {
                    match c.step() {
                        finite_bridge::Event::Progress => (),
                        finite_bridge::Event::Answer(a) => {
                            append(&mut wire, a, service_start, &mut serialization, &mut first)
                        }
                        finite_bridge::Event::Exhausted => caller = None,
                    }
                } else if let Some(solution) = solutions.as_mut().unwrap().next() {
                    let bridge = if let Some((_, b)) = &finite {
                        b
                    } else {
                        &kept.as_ref().unwrap().1
                    };
                    let (q, weight) = bridge.transport(solution);
                    caller = Some(bridge.start(q, weight));
                } else {
                    exhausted = true;
                }
            }
            let service_ns = ns(service_start);
            let t = Instant::now();
            drop((engine, machine, caller, solutions));
            let query_drop_ns = ns(t);
            assert!(serialization <= service_ns);
            let total = setup_ns + service_ns + query_drop_ns;
            query_sum += total;
            eprintln!(
                "{{\"query\":{index},\"calls\":{calls},\"exhausted\":{exhausted},\"setup_ns\":{setup_ns},\"service_ns\":{service_ns},\"serialization_ns\":{serialization},\"compute_observe_ns\":{},\"query_drop_ns\":{query_drop_ns},\"query_total_ns\":{total},\"first_observation_ns\":{},\"wire_bytes\":{},\"wire_capacity\":{}}}",
                service_ns - serialization,
                first.map_or("null".into(), |n| n.to_string()),
                wire.bytes.len(),
                wire.bytes.capacity()
            );
            held.push((index, exhausted, wire));
        }
        let t = Instant::now();
        drop(query_inputs);
        let query_batch_drop_ns = ns(t);
        let t = Instant::now();
        drop((
            compiled,
            contextual,
            conditional,
            prefix,
            finite,
            kept,
            artifacts,
            source_symbols,
            rules,
        ));
        let prepared_drop_ns = ns(t);
        let mut consumer_drop_ns = 0;
        for (index, exhausted, wire) in held.drain(..) {
            use std::io::Write;
            println!("WIRE {index} {exhausted} {}", wire.bytes.len());
            println!("PRED {}", wire.symbols.predicates.join(","));
            println!("ATOM {}", wire.symbols.atoms.join(","));
            io::stdout().write_all(&wire.bytes).unwrap();
            let t = Instant::now();
            drop(wire);
            consumer_drop_ns += ns(t);
        }
        let t = Instant::now();
        drop(held);
        consumer_drop_ns += ns(t);
        let total = input_load_ns
            + decode_ns
            + input_drop_ns
            + prepare_ns
            + consumer_setup_ns
            + query_sum
            + query_batch_drop_ns
            + prepared_drop_ns
            + consumer_drop_ns;
        eprintln!(
            "{{\"session_disposed\":true,\"input_load_ns\":{input_load_ns},\"decode_ns\":{decode_ns},\"input_drop_ns\":{input_drop_ns},\"prepare_ns\":{prepare_ns},\"consumer_setup_ns\":{consumer_setup_ns},\"query_batch_drop_ns\":{query_batch_drop_ns},\"prepared_drop_ns\":{prepared_drop_ns},\"consumer_drop_ns\":{consumer_drop_ns},\"lifecycle_ns\":{total}}}"
        );
    }
}
fn main() {
    common::lifecycle();
}
