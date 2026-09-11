#![cfg(feature = "carrier-contraction")]
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[allow(dead_code)]
#[path = "../examples/support/stream_run.rs"]
mod runtime;
#[allow(dead_code)]
#[path = "../examples/support/stream_source.rs"]
mod source;

#[test]
fn source_inferred_carriers_match_existing_stream_controls() {
    let mut cases = 0;
    for family in ["repeated", "distinct", "aliases"] {
        for resource in [false, true] {
            for fail_tail in [false, true] {
                let schema = source::Schema {
                    family,
                    resource,
                    fail_tail,
                    work: 2,
                    payload: 1,
                };
                let rules = schema.rules();
                let base = chr_compiled::PreparedRuleset::new(rules.clone(), None)
                    .unwrap()
                    .specialize_inferred();
                let eligibility = base.carrier_eligibility();
                assert!(
                    eligibility
                        .iter()
                        .any(|e| e.predicate.0 == "wait" && e.eligible)
                );
                assert!(
                    eligibility
                        .iter()
                        .any(|e| e.predicate.0 == "stream" && !e.eligible)
                );
                let contracted = base.contract_carriers_inferred().unwrap();
                let mut controls: Vec<_> = ["direct", "sealed", "dependencies", "templates"]
                    .into_iter()
                    .map(|mode| runtime::Prepared::new(mode, schema, rules.clone()))
                    .collect();
                controls.push(runtime::Prepared::Graph(
                    chr_direct_choice::demand::Prepared::new(rules.clone())
                        .unwrap()
                        .with_template_follow_limit(0),
                ));
                for depth in [0, 8, 32] {
                    for reverse in [false, true] {
                        let query = schema.query(depth, reverse);
                        let expected = oracle::run(&rules, &query, 200_000);
                        oracle::same_raw(expected.clone(), schema.expected(depth));
                        for prepared in &controls {
                            let mut run = prepared.start(query.clone());
                            let mut answers = vec![];
                            let mut done = false;
                            for _ in 0..200_000 {
                                match run.tick() {
                                    runtime::Event::Answer(a) => answers.push(a),
                                    runtime::Event::Done => {
                                        done = true;
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                            assert!(done);
                            drop(run);
                            oracle::same_raw(answers, expected.clone());
                        }
                        let mut run = contracted
                            .start_search(
                                query,
                                chr_compiled::Policy::Global,
                                chr_compiled::Access::Scan,
                            )
                            .unwrap();
                        let mut answers = vec![];
                        let mut done = false;
                        for _ in 0..200_000 {
                            match run.tick() {
                                chr_compiled::SearchEvent::Complete(mut b) => {
                                    answers.push(b.engine.observe().unwrap())
                                }
                                chr_compiled::SearchEvent::Exhausted => {
                                    done = true;
                                    break;
                                }
                                _ => {}
                            }
                        }
                        assert!(done);
                        drop(run);
                        oracle::same_raw(answers, expected);
                        cases += 1;
                    }
                }
            }
        }
    }
    assert_eq!(cases, 72);
}
