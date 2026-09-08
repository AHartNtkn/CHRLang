use chr_compiled::{Engine, Execution, Policy, fixtures};
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
                let mut engine = Engine::new(id, case.query.clone(), policy, execution).unwrap();
                let result = engine.run(100_000);
                assert!(result.exhausted, "{id} {policy:?} {execution:?}");
                assert_eq!(result.failed, case.failed, "{id} {policy:?} {execution:?}");
                match (&result.answer, &case.expected) {
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
                let mut engine = Engine::new(id, case.query, policy, execution).unwrap();
                engine.enable_audit();
                let result = engine.run(10000);
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
                let mut engine = Engine::new(id, case.query, policy, execution).unwrap();
                engine.enable_audit();
                let result = engine.run(10000);
                assert!(result.exhausted && !result.failed);
                assert!(
                    chr_observe::equivalent(
                        result.answer.as_ref().unwrap(),
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
        let mut engine = Engine::new(0, case.query, Policy::Active, Execution::Generated).unwrap();
        let result = engine.run(10000);
        assert!(chr_observe::equivalent(
            result.answer.as_ref().unwrap(),
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
            let mut engine = Engine::new(12, query.clone(), policy, execution).unwrap();
            engine.enable_audit();
            let result = engine.run(1000);
            assert!(result.exhausted && !result.failed);
            let rules = &fixtures::programs()[12];
            for commit in engine.audit() {
                support::check_commit(rules, commit)
            }
            assert!(support::terminal(rules, &engine.view()));
            traces.push(engine.trace().to_vec());
            if matches!(execution, Execution::Generic) {
                policy_answers.push(result.answer.unwrap());
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
            let mut engine = Engine::new(4, q, policy, execution).unwrap();
            for _ in 0..100 {
                let before = engine.stats().candidate_visits;
                engine.step();
                assert!(
                    engine.stats().candidate_visits - before <= 1,
                    "{policy:?} {execution:?} did not yield between candidates"
                );
            }
            assert!(engine.run(1000).exhausted);
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
            let mut late = Engine::new(6, q, policy, execution).unwrap();
            let result = late.run(10000);
            assert!(result.exhausted && result.failed);
            assert!(result.answer.is_none());
            // A guard must not equate two distinct free query variables.
            let q = Query {
                constraints: vec![c("p", [v(0), v(1)])],
                outputs: vec![],
            };
            let mut guard = Engine::new(3, q, policy, execution).unwrap();
            let result = guard.run(1000);
            assert_eq!(result.answer.unwrap().residual, vec![c("p", [v(0), v(1)])]);
            // One occurrence cannot fill both repeated head positions.
            let q = Query {
                constraints: vec![c("p", [atom("a")])],
                outputs: vec![],
            };
            let mut distinct = Engine::new(4, q, policy, execution).unwrap();
            assert_eq!(
                distinct.run(1000).answer.unwrap().residual,
                vec![c("p", [atom("a")])]
            );
        }
    }
    let rules = vec![Rule::simplify(
        "choice",
        [c("p", [])],
        or(Goal::True, Goal::Fail),
    )];
    assert!(chr_compiled::generate::emit("unsupported", &rules).is_err());
    assert!(
        Engine::with_program(
            rules,
            Query {
                constraints: vec![],
                outputs: vec![]
            },
            Policy::Global,
            None
        )
        .is_err()
    );
}

#[test]
fn native_rules_do_not_call_generic_template_walkers() {
    for policy in [Policy::Global, Policy::Active] {
        for execution in [Execution::Generic, Execution::Generated] {
            let case = fixtures::case(0, 3);
            let mut engine = Engine::new(0, case.query, policy, execution).unwrap();
            assert!(engine.run(10000).exhausted);
            assert!(engine.stats().structural_tests > 0);
            assert_eq!(
                engine.stats().generic_ast_visits == 0,
                matches!(execution, Execution::Generated)
            );
        }
    }
}
