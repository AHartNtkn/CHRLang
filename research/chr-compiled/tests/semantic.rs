use chr_compiled::{Access, Engine, Execution, Policy, PreparedRuleset, fixtures};
fn prepared_engine(
    id: usize,
    q: chr_syntax::Query,
    p: Policy,
    e: Execution,
) -> Result<Engine, String> {
    let rules = PreparedRuleset::bundled(id, e)?;
    let mut engine = rules.start(q, p, Access::Scan)?;
    engine.enable_trace();
    Ok(engine)
}
#[test]
fn integrated_hand_cases_have_independent_complete_answers() {
    for (id, _) in fixtures::programs()
        .iter()
        .take(fixtures::ANALYTIC_PROGRAMS)
        .enumerate()
    {
        let case = fixtures::case(id, 3);
        for policy in [Policy::Global, Policy::Active] {
            let mut traces = vec![];
            for execution in [Execution::Generic, Execution::Generated] {
                let mut engine =
                    prepared_engine(id, case.query.clone(), policy, execution).unwrap();
                let result = engine.advance(100_000);
                assert!(result.exhausted, "{id} {policy:?} {execution:?}");
                assert_eq!(result.failed, case.failed, "{id} {policy:?} {execution:?}");
                match (&engine.observe(), &case.expected) {
                    (Some(actual), Some(expected)) => assert!(
                        chr_observe::equivalent(actual, expected, &mut Default::default()),
                        "{id} {policy:?} {execution:?}: {actual:?}"
                    ),
                    (None, None) => (),
                    _ => panic!("wrong completion {id}"),
                }
                traces.push(engine.trace().to_vec());
            }
            assert_eq!(traces[0], traces[1], "within-policy trace {id} {policy:?}");
        }
    }
}

#[test]
fn equation_change_report_is_commit_only() {
    use chr_persistent::{
        Stats,
        kernel::{Arena, Bindings, Term},
    };
    let mut arena = Arena::default();
    let mut stats = Stats::default();
    let mut bindings = Bindings::default();
    let a = arena.make("a", vec![], &mut stats);
    let b = arena.make("b", vec![], &mut stats);
    let left = arena.make("pair", vec![a, Term::Var(0)], &mut stats);
    let right = arena.make("pair", vec![b, a], &mut stats);
    let mut changed = vec![99];
    assert!(!arena.unify_record(left, right, &mut bindings, &mut stats, &mut changed));
    assert_eq!(changed, vec![99]);
    assert!(arena.unify_record(Term::Var(0), a, &mut bindings, &mut stats, &mut changed));
    assert_eq!(changed, vec![99, 0]);
}

mod support;
#[test]
fn independent_source_checker_validates_commits_and_quiescence() {
    for (id, rules) in fixtures::programs()
        .iter()
        .take(fixtures::ANALYTIC_PROGRAMS)
        .enumerate()
    {
        for policy in [Policy::Global, Policy::Active] {
            for execution in [Execution::Generic, Execution::Generated] {
                let case = fixtures::case(id, 2);
                let mut engine = prepared_engine(id, case.query, policy, execution).unwrap();
                engine.enable_audit();
                let result = engine.advance(10000);
                assert!(result.exhausted);
                for commit in engine.audit() {
                    support::check_commit(rules, commit)
                }
                if !result.failed {
                    assert!(
                        support::terminal(rules, &engine.view()),
                        "missed work {id} {policy:?} {execution:?}"
                    )
                }
            }
        }
    }
}

#[test]
fn active_changes_history_and_runtime_queries() {
    for id in 8..12 {
        for policy in [Policy::Global, Policy::Active] {
            for execution in [Execution::Generic, Execution::Generated] {
                let case = fixtures::case(id, 2);
                let mut engine = prepared_engine(id, case.query, policy, execution).unwrap();
                engine.enable_audit();
                let result = engine.advance(10000);
                assert!(result.exhausted && !result.failed);
                assert!(
                    chr_observe::equivalent(
                        engine.observe().as_ref().unwrap(),
                        case.expected.as_ref().unwrap(),
                        &mut Default::default()
                    ),
                    "{id} {policy:?} {execution:?}"
                );
                let rules = &fixtures::programs()[id];
                for commit in engine.audit() {
                    support::check_commit(rules, commit)
                }
                assert!(support::terminal(rules, &engine.view()));
            }
        }
    }
    // The same native ruleset handles runtime depth that is absent from generation.
    for depth in [0, 1, 9] {
        let case = fixtures::case(0, depth);
        let mut engine =
            prepared_engine(0, case.query, Policy::Active, Execution::Generated).unwrap();
        let result = engine.advance(10000);
        assert!(result.exhausted);
        assert!(chr_observe::equivalent(
            engine.observe().as_ref().unwrap(),
            case.expected.as_ref().unwrap(),
            &mut Default::default()
        ));
    }
}

#[test]
fn nonconfluent_policies_may_choose_different_legal_schedules() {
    use chr_syntax::{Query, c};
    let query = Query {
        constraints: vec![c("seed", []), c("p", [])],
        outputs: vec![],
    };
    let mut policy_answers = vec![];
    for policy in [Policy::Global, Policy::Active] {
        let mut traces = vec![];
        for execution in [Execution::Generic, Execution::Generated] {
            let mut engine = prepared_engine(12, query.clone(), policy, execution).unwrap();
            engine.enable_audit();
            let result = engine.advance(1000);
            assert!(result.exhausted && !result.failed);
            let rules = &fixtures::programs()[12];
            for commit in engine.audit() {
                support::check_commit(rules, commit)
            }
            assert!(support::terminal(rules, &engine.view()));
            traces.push(engine.trace().to_vec());
            if matches!(execution, Execution::Generic) {
                policy_answers.push(engine.observe().unwrap());
            }
        }
        assert_eq!(traces[0], traces[1]);
    }
    assert_ne!(policy_answers[0], policy_answers[1]);
}

#[test]
fn partner_traversal_resumes_at_candidate_boundaries() {
    use chr_syntax::{Query, atom, c};
    for policy in [Policy::Global, Policy::Active] {
        for execution in [Execution::Generic, Execution::Generated] {
            let q = Query {
                constraints: vec![
                    c("p", [atom("a")]),
                    c("p", [atom("b")]),
                    c("p", [atom("c")]),
                ],
                outputs: vec![],
            };
            let mut engine = prepared_engine(4, q, policy, execution).unwrap();
            for _ in 0..100 {
                let before = engine.stats().candidate_visits;
                engine.step();
                if chr_compiled::COLLECT_METRICS {
                    assert!(
                        engine.stats().candidate_visits - before <= 1,
                        "{policy:?} {execution:?} did not yield between candidates"
                    );
                }
            }
            assert!(engine.advance(1000).exhausted);
        }
    }
}

#[test]
fn source_effects_and_matching_do_not_publish_invalid_answers() {
    use chr_syntax::{Goal, Query, Rule, atom, c, or, v};
    for policy in [Policy::Global, Policy::Active] {
        for execution in [Execution::Generic, Execution::Generated] {
            let q = Query {
                constraints: vec![c("skip", []), c("late", [v(0)])],
                outputs: vec![("x".into(), chr_syntax::Var(0))],
            };
            let mut late = prepared_engine(6, q, policy, execution).unwrap();
            let result = late.advance(10000);
            assert!(result.exhausted && result.failed);
            assert!(late.observe().is_none());
            // A guard must not equate two distinct free query variables.
            let q = Query {
                constraints: vec![c("p", [v(0), v(1)])],
                outputs: vec![],
            };
            let mut guard = prepared_engine(3, q, policy, execution).unwrap();
            let result = guard.advance(1000);
            assert!(result.exhausted);
            assert_eq!(
                guard.observe().unwrap().residual,
                vec![c("p", [v(0), v(1)])]
            );
            // One occurrence cannot fill both repeated head positions.
            let q = Query {
                constraints: vec![c("p", [atom("a")])],
                outputs: vec![],
            };
            let mut distinct = prepared_engine(4, q, policy, execution).unwrap();
            assert_eq!(
                {
                    assert!(distinct.advance(1000).exhausted);
                    distinct.observe().unwrap().residual
                },
                vec![c("p", [atom("a")])]
            );
        }
    }
    let rules = vec![Rule::simplify(
        "choice",
        [c("p", [])],
        or(Goal::True, Goal::Fail),
    )];
    assert!(chr_compiled::generate::emit("choice", &rules).is_ok());
    let prepared = PreparedRuleset::new(rules, None).unwrap();
    let mut branch = prepared
        .start(
            Query {
                constraints: vec![c("p", [])],
                outputs: vec![],
            },
            Policy::Global,
            Access::Scan,
        )
        .unwrap();
    assert!(branch.advance(1000).pending_split);
    assert!(branch.observe().is_none());
}

#[test]
fn native_rules_do_not_call_generic_template_walkers() {
    for policy in [Policy::Global, Policy::Active] {
        for execution in [Execution::Generic, Execution::Generated] {
            let case = fixtures::case(0, 3);
            let mut engine = prepared_engine(0, case.query, policy, execution).unwrap();
            assert!(engine.advance(10000).exhausted);
            if chr_compiled::COLLECT_METRICS {
                assert!(engine.stats().structural_tests > 0);
            }
            if chr_compiled::COLLECT_METRICS {
                assert_eq!(
                    engine.stats().generic_ast_visits == 0,
                    matches!(execution, Execution::Generated)
                );
            }
        }
    }
}
