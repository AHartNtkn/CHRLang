use chr_compiled::{
    Access, Execution, Policy, PreparedRuleset, SearchEngine, SearchEvent, search_bundled,
    search_fixtures,
};
use chr_syntax::{Answer, Query, Var, atom, c, v};

type Observation = (Answer, Vec<(usize, Vec<u64>)>);
fn answers(mut search: SearchEngine) -> Vec<Observation> {
    search.enable_trace();
    let mut answers = vec![];
    for _ in 0..100_000 {
        match search.tick() {
            SearchEvent::Complete(mut branch) => answers.push((
                branch.engine.observe().unwrap(),
                branch.engine.trace().to_vec(),
            )),
            SearchEvent::Exhausted => {
                answers.sort();
                return answers;
            }
            _ => (),
        }
    }
    panic!("finite search exceeded test bound");
}
#[test]
fn prepared_topology_preserves_changed_givens_raw_answers_and_early_drop() {
    for execution in [Execution::Generic, Execution::Generated] {
        let rules = search_fixtures::finite_rules(0);
        let code = matches!(execution, Execution::Generated)
            .then(|| search_bundled(search_fixtures::FINITE_START));
        let prepared = PreparedRuleset::new(rules, code).unwrap();
        let query = Query {
            constraints: vec![
                c("choose", [v(4)]),
                c("choose", [v(8)]),
                c("marker", [v(4), v(8)]),
                c("marker", [v(4), v(8)]),
            ],
            outputs: vec![("shown".into(), Var(4))],
        };
        let template = prepared
            .prepare_query(query.clone(), Policy::Active, Access::Indexed)
            .unwrap();
        assert_eq!(template.retention().occurrences, 4);
        assert_eq!(template.retention().pending, 0);
        assert_eq!(template.retention().history, 0);
        assert_eq!(template.stats().applications, 0);
        for extra in [
            vec![],
            vec![c("given", [v(4), atom("a1")])],
            vec![
                c("given", [v(4), atom("a0")]),
                c("given", [v(4), atom("a2")]),
            ],
            vec![
                c("given", [v(8), atom("a2")]),
                c("given", [v(8), atom("a2")]),
            ],
        ] {
            let mut fresh = query.clone();
            fresh.constraints.extend(extra.clone());
            let expected = answers(
                prepared
                    .start_search(fresh, Policy::Active, Access::Indexed)
                    .unwrap(),
            );
            let mut abandoned = template.start(extra.clone()).unwrap();
            for _ in 0..4 {
                let _ = abandoned.tick();
            }
            drop(abandoned);
            assert_eq!(answers(template.start(extra).unwrap()), expected);
        }
        // Neither assignment choices nor source simplification ran in preparation.
        assert_eq!(answers(template.start(vec![]).unwrap()).len(), 9);
        assert!(
            template
                .start(vec![c("given", [v(99), atom("a0")])])
                .is_err()
        );
        assert_eq!(answers(template.start(vec![]).unwrap()).len(), 9);
    }
}

#[test]
fn prepared_prefix_preserves_ground_join_choices_and_owned_answers() {
    use chr_syntax::Rule;
    for family in ["selective", "neutral", "duplicate", "broad"] {
        for n in [8_usize, 32, 128] {
            for policy in [Policy::Global, Policy::Active] {
                for access in [Access::Scan, Access::Indexed] {
                    let duplicate = family == "duplicate";
                    let changing = duplicate || family == "broad";
                    let neutral = family == "neutral";
                    let key = |name: &str, i| atom(&format!("{name}{i}"));
                    let mut kept = vec![c("left", [v(0), v(1)]), c("right", [v(1), v(2)])];
                    if !neutral {
                        kept.insert(0, c("request", [v(2)]));
                    }
                    let prepared = PreparedRuleset::new(
                        vec![Rule {
                            name: "join".into(),
                            kept,
                            removed: if neutral {
                                vec![c("request", [v(2)])]
                            } else {
                                vec![c("token", [])]
                            },
                            guards: vec![],
                            body: c("receipt", [v(0), v(1), v(2)]).into(),
                        }],
                        None,
                    )
                    .unwrap();
                    let left = (0..n)
                        .map(|i| {
                            c(
                                "left",
                                [key("a", i), key("x", if duplicate { 0 } else { i })],
                            )
                        })
                        .collect::<Vec<_>>();
                    let right = |endpoint| {
                        (0..if duplicate { n - 1 } else { n })
                            .map(|i| {
                                c(
                                    "right",
                                    [
                                        key("x", if duplicate { 0 } else { i }),
                                        key("b", if changing { endpoint } else { i }),
                                    ],
                                )
                            })
                            .collect::<Vec<_>>()
                    };
                    let mut prefix = left.clone();
                    if !changing {
                        prefix.extend(right(0));
                    }
                    let template = prepared
                        .prepare_query(
                            Query {
                                constraints: prefix.clone(),
                                outputs: vec![],
                            },
                            policy,
                            access,
                        )
                        .unwrap();
                    let mut retained = vec![];
                    for endpoint in [0, n - 1] {
                        let mut suffix = if changing { right(endpoint) } else { vec![] };
                        suffix.push(c("request", [key("b", endpoint)]));
                        if !neutral {
                            suffix.push(c("token", []));
                        }
                        let mut facts = prefix.clone();
                        facts.extend(suffix.clone());
                        let chosen = if changing { 0 } else { endpoint };
                        let m = if duplicate { n - 1 } else { n };
                        let request = (n + m) as u64;
                        let mut tuple = vec![chosen as u64, (n + chosen) as u64];
                        if neutral {
                            tuple.push(request);
                        } else {
                            tuple.insert(0, request);
                            tuple.push(request + 1);
                        }
                        // Ground relational oracle: all table occurrences survive, exactly one receipt.
                        let mut residual = left.clone();
                        residual.extend(right(endpoint));
                        if !neutral {
                            residual.push(c("request", [key("b", endpoint)]));
                        }
                        residual.push(c(
                            "receipt",
                            [key("a", chosen), key("x", chosen), key("b", endpoint)],
                        ));
                        residual.sort();
                        if endpoint == 0 {
                            let mut cancelled = template.start(suffix.clone()).unwrap();
                            assert!(matches!(cancelled.tick(), SearchEvent::Progress));
                            drop(cancelled);
                        }
                        for search in [
                            prepared
                                .start_search(
                                    Query {
                                        constraints: facts,
                                        outputs: vec![],
                                    },
                                    policy,
                                    access,
                                )
                                .unwrap(),
                            template.start(suffix).unwrap(),
                        ] {
                            let mut observed = answers(search);
                            for (answer, _) in &mut observed {
                                answer.residual.sort();
                            }
                            assert_eq!(observed.len(), 1, "{family} {n} {policy:?} {access:?}");
                            let (answer, trace) = &observed[0];
                            assert!(answer.outputs.is_empty());
                            assert_eq!(
                                answer.residual, residual,
                                "{family} {n} {policy:?} {access:?}"
                            );
                            assert_eq!(
                                trace,
                                &vec![(0, tuple.clone())],
                                "{family} {n} {policy:?} {access:?}"
                            );
                            retained.push((observed, residual.clone()));
                        }
                    }
                    drop(template);
                    drop(prepared);
                    for (observed, expected) in retained {
                        assert_eq!(observed[0].0.residual, expected);
                    }
                }
            }
        }
    }
}
