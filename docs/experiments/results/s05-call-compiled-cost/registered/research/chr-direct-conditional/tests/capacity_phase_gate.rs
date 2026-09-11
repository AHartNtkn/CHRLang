#[allow(dead_code, unused_imports)]
mod gate {
    include!("resource_capacity_entry.rs");
    fn naive(prefix: &[Rule], caller: &[Rule], q: &Query) -> Vec<Answer> {
        capacity::Prepared::<false>::new(prefix)
            .unwrap()
            .solve(q, capacity::Limits::default())
            .unwrap()
            .answers
            .into_iter()
            .flat_map(|a| {
                let next = Query {
                    constraints: a.residual,
                    outputs: vec![],
                };
                checked(caller, &next)
            })
            .collect()
    }
    #[test]
    fn unordered_capacity_answers_do_not_preserve_first_done_consumption() {
        let prefix = source("p_", 1);
        let caller = vec![Rule::simplify(
            "choose",
            [c("go", []), c("p_done", [v(0)])],
            c("chosen", [v(0)]).into(),
        )];
        let mut q = query("p_", &[2, 1], [1, 1], false, 1);
        // pick1 has earlier source priority but the larger variable identity.
        q.outputs.clear();
        q.constraints.push(c("go", []));
        let mut full = prefix.clone();
        full.extend(caller.clone());
        let expected = checked(&full, &q);
        let actual = naive(&prefix, &caller, &q);
        runtime_support::same_raw(resumed(&full, 5, &q), expected.clone());
        assert_eq!(expected.len(), 1);
        assert_eq!(actual.len(), 1);
        assert!(expected[0].residual.contains(&c("chosen", [atom("a")])));
        assert!(actual[0].residual.contains(&c("chosen", [atom("b")])));
    }
    #[test]
    fn unordered_capacity_answers_do_not_preserve_spare_token_order() {
        let prefix = source("p_", 1);
        let caller = vec![Rule::simplify(
            "choose",
            [c("go", []), c("p_token", [v(0)])],
            c("chosen", [v(0)]).into(),
        )];
        let q = Query {
            constraints: vec![
                c("p_token", [atom("b")]),
                c("p_token", [atom("a")]),
                c("go", []),
            ],
            outputs: vec![],
        };
        let mut full = prefix.clone();
        full.extend(caller.clone());
        let expected = checked(&full, &q);
        let actual = naive(&prefix, &caller, &q);
        runtime_support::same_raw(resumed(&full, 5, &q), expected.clone());
        assert_eq!(expected.len(), 1);
        assert_eq!(actual.len(), 1);
        assert!(expected[0].residual.contains(&c("chosen", [atom("b")])));
        assert!(actual[0].residual.contains(&c("chosen", [atom("a")])));
    }
    fn resumed(full: &[Rule], prefix: usize, q: &Query) -> Vec<Answer> {
        let phase = capacity::phase::Prepared::<false>::new(full, prefix).unwrap();
        let continuations = phase.solve(q, capacity::Limits::default()).unwrap();
        let mut rules = vec![Rule::simplify(
            "$bind",
            [c("$bind", [v(0), v(1)])],
            eq(v(0), v(1)),
        )];
        rules.extend_from_slice(full);
        continuations
            .into_iter()
            .flat_map(|mut next| {
                next.query.constraints.extend(
                    next.bindings
                        .into_iter()
                        .map(|(var, value)| c("$bind", [Term::Var(var), value])),
                );
                checked(&rules, &next.query)
            })
            .collect()
    }
    #[test]
    fn ordered_capacity_phase_preserves_complete_callers() {
        let mut cases = 0;
        for token_first in [false, true] {
            for token_reverse in [false, true] {
                for domains in [[1, 2], [2, 1], [3, 3]] {
                    for caps in [[1, 1], [2, 2], [0, 0]] {
                        for alias in [false, true] {
                            for descending in [false, true] {
                                for weight in [1, 2] {
                                    for reverse in [false, true] {
                                        for caller_kind in 0..5 {
                                            let mut full = source("p_", weight);
                                            if token_first {
                                                full[3].removed.swap(0, 1);
                                            }
                                            let mut q = query("p_", &domains, caps, alias, 10);
                                            if token_reverse {
                                                q.constraints[2..2 + caps[0] + caps[1]].reverse();
                                            }

                                            if descending && !alias {
                                                for c in &mut q.constraints {
                                                    for t in &mut c.args {
                                                        if *t == v(10) {
                                                            *t = v(11);
                                                        } else if *t == v(11) {
                                                            *t = v(10);
                                                        }
                                                    }
                                                }
                                                for (_, var) in &mut q.outputs {
                                                    if *var == Var(10) {
                                                        *var = Var(11);
                                                    } else if *var == Var(11) {
                                                        *var = Var(10);
                                                    }
                                                }
                                            }
                                            let shared = q.constraints[0].args[0].clone();
                                            if reverse {
                                                q.constraints.swap(0, 1);
                                            }
                                            q.outputs.retain(|(name, _)| name == "unused");
                                            q.constraints.push(c("go", [shared, v(99)]));
                                            q.constraints.push(c("outside", [v(99)]));
                                            let rule = match caller_kind {
                                                0 => Rule::simplify(
                                                    "caller",
                                                    [c("go", [v(0), v(1)]), c("p_done", [v(2)])],
                                                    c("chosen", [v(2), v(0), v(1)]).into(),
                                                ),
                                                1 => Rule::simplify(
                                                    "caller",
                                                    [c("go", [v(0), v(1)]), c("p_token", [v(2)])],
                                                    c("chosen", [v(2), v(0), v(1)]).into(),
                                                ),
                                                2 => Rule::simplify(
                                                    "caller",
                                                    [c("go", [v(0), v(1)])],
                                                    c("observed", [v(0), v(1)]).into(),
                                                ),
                                                3 => Rule::simplify(
                                                    "caller",
                                                    [c("go", [v(0), v(1)])],
                                                    and(vec![
                                                        eq(v(0), atom("a")),
                                                        c("written", [v(0), v(1)]).into(),
                                                    ]),
                                                ),
                                                _ => Rule::simplify(
                                                    "caller",
                                                    [c("go", [v(0), v(1)])],
                                                    and(vec![
                                                        c("p_pick1", [v(1)]).into(),
                                                        c("p_token", [atom("a")]).into(),
                                                        c("later", [v(0), v(1)]).into(),
                                                    ]),
                                                ),
                                            };
                                            full.push(rule);
                                            runtime_support::same_raw(
                                                resumed(&full, 5, &q),
                                                checked(&full, &q),
                                            );
                                            cases += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(cases, 2880);
    }
    #[test]
    fn capacity_phase_preserves_named_outputs_and_rejects_prior_interference() {
        let prefix = source("p_", 2);
        let mut after = prefix.clone();
        after.push(Rule::simplify(
            "steal",
            [c("go", []), c("p_token", [v(0)])],
            c("stolen", [v(0)]).into(),
        ));
        let mut q = query("p_", &[1], [1, 1], false, 10);
        q.constraints.push(c("go", []));
        let expected = checked(&after, &q);
        assert_eq!(expected.len(), 2);
        assert!(
            expected
                .iter()
                .all(|a| a.residual.contains(&c("stolen", [atom("b")])))
        );
        runtime_support::same_raw(resumed(&after, 5, &q), expected);
        let mut before = vec![after.last().unwrap().clone()];
        before.extend(prefix.clone());
        assert!(checked(&before, &q).is_empty());
        assert!(capacity::phase::Prepared::<false>::new(&before, 6).is_err());
        assert!(capacity::phase::Prepared::<false>::new(&before, 5).is_err());
        let phase = capacity::phase::Prepared::<false>::new(&after, 5).unwrap();
        assert!(
            phase
                .solve(
                    &q,
                    capacity::Limits {
                        states: 0,
                        ..capacity::Limits::default()
                    }
                )
                .is_err()
        );
        assert!(
            phase
                .solve(
                    &q,
                    capacity::Limits {
                        answers: 1,
                        ..capacity::Limits::default()
                    }
                )
                .is_err()
        );
        assert_eq!(
            phase.solve(&q, capacity::Limits::default()).unwrap().len(),
            2
        );
        let mut malformed = q.clone();
        malformed.outputs.push(malformed.outputs[0].clone());
        assert!(
            phase
                .solve(&malformed, capacity::Limits::default())
                .is_err()
        );
        let oversized = Query {
            constraints: vec![c("inert", []); 10001],
            outputs: vec![],
        };
        assert!(
            phase
                .solve(&oversized, capacity::Limits::default())
                .is_err()
        );
        let shared = query("p_", &[3, 3], [2, 2], true, 50);
        runtime_support::same_raw(resumed(&after, 5, &shared), checked(&after, &shared));
    }
    #[test]
    fn capacity_phase_preserves_later_propagation_and_fresh_aliases() {
        for weight in [1, 2] {
            for alias in [false, true] {
                let mut full = source("p_", weight);
                full.push(Rule::propagate(
                    "observe-pair",
                    [c("p_done", [v(0)]), c("p_done", [v(1)])],
                    c("seen", [v(0), v(1), v(99), v(99)]).into(),
                ));
                full.push(Rule::simplify(
                    "take",
                    [c("go", []), c("p_done", [v(0)])],
                    c("chosen", [v(0)]).into(),
                ));
                let mut q = query("p_", &[3, 3], [2, 2], alias, 10);
                q.constraints.insert(0, c("p_done", [atom("initial")]));
                q.constraints.push(c("go", []));
                let expected = checked(&full, &q);
                assert_eq!(expected.len(), if alias { 2 } else { 4 } * weight * weight);
                for answer in &expected {
                    assert_eq!(
                        answer.residual.iter().filter(|c| c.name == "seen").count(),
                        6
                    );
                    assert!(answer.residual.contains(&c("chosen", [atom("initial")])));
                }
                runtime_support::same_raw(resumed(&full, 5, &q), expected);
            }
        }
    }
    #[test]
    fn token_first_consumption_preserves_its_own_source_order() {
        let mut full = source("p_", 1);
        full[3].removed.swap(0, 1);
        full.push(Rule::simplify(
            "choose",
            [c("go", []), c("p_done", [v(0)])],
            c("chosen", [v(0)]).into(),
        ));
        let mut q = query("p_", &[1, 2], [1, 1], false, 10);
        q.constraints.swap(2, 3);
        q.constraints.push(c("go", []));
        let expected = checked(&full, &q);
        assert_eq!(expected.len(), 1);
        assert!(expected[0].residual.contains(&c("chosen", [atom("b")])));
        runtime_support::same_raw(resumed(&full, 5, &q), expected);
    }
}
