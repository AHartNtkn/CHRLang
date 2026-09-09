use chr_compiled::{Access, Execution, Policy, PreparedRuleset, fixtures};

#[test]
fn indexed_and_scanned_access_preserve_full_observations_and_traces() {
    for id in 0..fixtures::ANALYTIC_PROGRAMS {
        let case = fixtures::case(id, 3);
        for execution in [Execution::Generic, Execution::Generated] {
            let prepared = PreparedRuleset::bundled(id, execution).unwrap();
            for policy in [Policy::Global, Policy::Active] {
                let mut traces = vec![];
                for access in [Access::Scan, Access::Indexed] {
                    let mut engine = prepared.start(case.query.clone(), policy, access).unwrap();
                    engine.enable_audit();
                    let result = engine.advance(100000);
                    assert!(result.exhausted);
                    assert_eq!(result.failed, case.failed);
                    if let Some(expected) = &case.expected {
                        assert!(
                            chr_observe::equivalent(
                                engine.observe().as_ref().unwrap(),
                                expected,
                                &mut Default::default()
                            ),
                            "{id} {policy:?} {execution:?} {access:?}"
                        )
                    }
                    let rules = &fixtures::programs()[id];
                    for commit in engine.audit() {
                        support::check_commit(rules, commit)
                    }
                    if !result.failed {
                        assert!(support::terminal(rules, &engine.view()));
                    }
                    traces.push(engine.trace().to_vec());
                }
                assert_eq!(traces[0], traces[1], "{id} {policy:?} {execution:?}");
            }
        }
    }
}

#[test]
fn prepared_rules_and_queries_have_independent_lifetimes() {
    for execution in [Execution::Generic, Execution::Generated] {
        let prepared = PreparedRuleset::bundled(7, execution).unwrap();
        let case = fixtures::case(7, 0);
        let mut one = prepared
            .start(case.query.clone(), Policy::Active, Access::Indexed)
            .unwrap();
        let mut two = prepared
            .start(case.query, Policy::Global, Access::Scan)
            .unwrap();
        one.step();
        drop(prepared);
        for engine in [&mut one, &mut two] {
            let result = engine.advance(10000);
            assert!(result.exhausted);
            assert!(chr_observe::equivalent(
                engine.observe().as_ref().unwrap(),
                case.expected.as_ref().unwrap(),
                &mut Default::default()
            ));
            assert!(engine.trace().is_empty());
            assert!(engine.audit().is_empty());
        }
    }
}

#[test]
fn different_queries_share_only_static_preparation() {
    for execution in [Execution::Generic, Execution::Generated] {
        let prepared = PreparedRuleset::bundled(0, execution).unwrap();
        let a = fixtures::case(0, 1);
        let b = fixtures::case(0, 4);
        let mut one = prepared
            .start(a.query, Policy::Active, Access::Indexed)
            .unwrap();
        let mut two = prepared
            .start(b.query, Policy::Active, Access::Indexed)
            .unwrap();
        one.step();
        two.step();
        assert!(chr_observe::equivalent(
            {
                assert!(one.advance(10000).exhausted);
                one.observe()
            }
            .as_ref()
            .unwrap(),
            a.expected.as_ref().unwrap(),
            &mut Default::default()
        ));
        drop(one);
        drop(prepared);
        assert!(chr_observe::equivalent(
            {
                assert!(two.advance(10000).exhausted);
                two.observe()
            }
            .as_ref()
            .unwrap(),
            b.expected.as_ref().unwrap(),
            &mut Default::default()
        ));
    }
}

#[path = "support/mod.rs"]
mod support;
#[test]
fn flat_keys_collisions_and_low_yield_repairs_have_independent_checks() {
    let cases = vec![
        (1, fixtures::flat_chain_case(3, false)),
        (2, fixtures::flat_chain_case(3, true)),
        (13, fixtures::collision_case(3)),
        (11, fixtures::repair_case(3)),
    ];
    for (id, case) in cases {
        for execution in [Execution::Generic, Execution::Generated] {
            let rules = fixtures::programs()[id].clone();
            let prepared = PreparedRuleset::bundled(id, execution).unwrap();
            for policy in [Policy::Global, Policy::Active] {
                let mut traces = vec![];
                for access in [Access::Scan, Access::Indexed] {
                    let mut engine = prepared.start(case.query.clone(), policy, access).unwrap();
                    engine.enable_audit();
                    let result = engine.advance(100000);
                    assert!(result.exhausted && !result.failed);
                    assert!(chr_observe::equivalent(
                        engine.observe().as_ref().unwrap(),
                        case.expected.as_ref().unwrap(),
                        &mut Default::default()
                    ));
                    if id == 13 && access == Access::Indexed && chr_compiled::COLLECT_METRICS {
                        assert!(
                            engine.stats().index_bucket_entries > 0,
                            "fixture must exercise a bound-key bucket"
                        );
                    }
                    for commit in engine.audit() {
                        support::check_commit(&rules, commit)
                    }
                    assert!(support::terminal(&rules, &engine.view()));
                    traces.push(engine.trace().to_vec());
                }
                assert_eq!(traces[0], traces[1]);
            }
        }
    }
}

#[test]
fn execution_observation_and_teardown_are_separate_lifetimes() {
    let prepared = PreparedRuleset::bundled(0, Execution::Generated).unwrap();
    let case = fixtures::case(0, 3);
    let mut engine = prepared
        .start(case.query, Policy::Active, Access::Indexed)
        .unwrap();
    assert!(!engine.advance(2).exhausted);
    assert!(engine.observe().is_none());
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(engine.stats().observation_visits, 0);
    }
    assert!(engine.advance(10000).exhausted);
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(engine.stats().observation_visits, 0);
    }
    let steps = engine.stats().source_steps;
    let first = engine.observe().unwrap();
    let second = engine.observe().unwrap();
    assert_eq!(first, second);
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(engine.stats().source_steps, steps);
    }
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(engine.stats().observation_visits, 2);
    }
    assert!(chr_observe::equivalent(
        &first,
        case.expected.as_ref().unwrap(),
        &mut Default::default()
    ));
    drop(engine);
    assert_eq!(prepared.stats().rules, 2);
}

#[test]
fn ground_intermediate_output_is_not_a_completed_observation() {
    use chr_syntax::{Query, Var, atom, c, v};
    for execution in [Execution::Generic, Execution::Generated] {
        let prepared = PreparedRuleset::bundled(6, execution).unwrap();
        let query = Query {
            constraints: vec![c("late", [v(0)])],
            outputs: vec![("x".into(), Var(0))],
        };
        let mut engine = prepared
            .start(query, Policy::Active, Access::Indexed)
            .unwrap();
        let mut saw_ground_pending = false;
        for _ in 0..100 {
            let status = engine.advance(1);
            if !status.exhausted && engine.view().outputs == vec![("x".into(), atom("a"))] {
                saw_ground_pending = true;
                assert!(engine.observe().is_none());
            }
            if status.exhausted {
                assert!(status.failed);
                break;
            }
        }
        assert!(saw_ground_pending);
        assert!(engine.status().failed);
        assert!(engine.observe().is_none());
        if chr_compiled::COLLECT_METRICS {
            assert_eq!(engine.stats().observation_visits, 0);
        }
    }
}

#[test]
fn no_head_residuals_keep_aliases_without_matcher_maintenance() {
    use chr_syntax::{Answer, Query, Var, atom, c, t, v};
    for execution in [Execution::Generic, Execution::Generated] {
        let prepared = PreparedRuleset::bundled(11, execution).unwrap();
        for policy in [Policy::Global, Policy::Active] {
            for access in [Access::Scan, Access::Indexed] {
                let query = Query {
                    constraints: vec![
                        c("residual", [v(0)]),
                        c("bind", [v(0), t("f", [v(1)])]),
                        c("bind", [v(1), atom("a")]),
                    ],
                    outputs: vec![("x".into(), Var(0)), ("y".into(), Var(1))],
                };
                let mut engine = prepared.start(query, policy, access).unwrap();
                engine.step();
                assert_eq!(engine.retention().occurrences, 1);
                assert_eq!(engine.retention().dependency_edges, 0);
                assert_eq!(engine.retention().index_reverse_records, 0);
                if chr_compiled::COLLECT_METRICS {
                    assert_eq!(engine.stats().dependency_refreshes, 0);
                }
                if chr_compiled::COLLECT_METRICS {
                    assert_eq!(engine.stats().activation_pushes, 0);
                }
                assert!(engine.advance(10000).exhausted);
                let expected = Answer {
                    outputs: vec![("x".into(), t("f", [atom("a")])), ("y".into(), atom("a"))],
                    residual: vec![c("residual", [t("f", [atom("a")])])],
                };
                assert_eq!(engine.observe().unwrap(), expected);
                assert_eq!(engine.retention().dependency_edges, 0);
                assert_eq!(engine.retention().index_reverse_records, 0);
                assert_eq!(engine.retention().index_entries, 0);
            }
        }
        let case = fixtures::case(7, 0);
        let fresh = PreparedRuleset::bundled(7, execution).unwrap();
        let mut engine = fresh
            .start(case.query, Policy::Active, Access::Indexed)
            .unwrap();
        assert!(engine.advance(10000).exhausted);
        assert!(chr_observe::equivalent(
            engine.observe().as_ref().unwrap(),
            case.expected.as_ref().unwrap(),
            &mut Default::default()
        ));
        assert_eq!(engine.retention().dependency_edges, 0);
        assert_eq!(engine.retention().index_reverse_records, 0);
    }
}

#[test]
fn metric_availability_does_not_change_execution_or_retention() {
    let p = PreparedRuleset::bundled(11, Execution::Generated).unwrap();
    let case = fixtures::repair_case(3);
    let mut e = p
        .start(case.query, Policy::Active, Access::Indexed)
        .unwrap();
    assert!(e.advance(10000).exhausted);
    assert!(chr_observe::equivalent(
        e.observe().as_ref().unwrap(),
        case.expected.as_ref().unwrap(),
        &mut Default::default()
    ));
    assert!(e.retention().index_entries > 0);
    if !chr_compiled::COLLECT_METRICS {
        assert_eq!(e.stats().candidate_visits, 0);
        assert_eq!(e.stats().key_visits, 0);
    }
    if !chr_persistent::COLLECT_KERNEL_METRICS {
        assert_eq!(e.stats().kernel.dereferences, 0);
        assert_eq!(e.stats().kernel.storage.visits, 0);
    }
}

#[test]
fn active_known_keys_bound_partner_discovery_on_flat_chains() {
    for n in [8, 32] {
        for execution in [Execution::Generic, Execution::Generated] {
            let prepared = PreparedRuleset::bundled(1, execution).unwrap();
            let case = fixtures::flat_chain_case(n, false);
            let mut engine = prepared
                .start(case.query, Policy::Active, Access::Indexed)
                .unwrap();
            // Repeated bounded calls exercise suspended matching, not only one large advance.
            let mut complete = false;
            for _ in 0..10000 {
                if engine.advance(7).exhausted {
                    complete = true;
                    break;
                }
            }
            assert!(complete);
            assert!(chr_observe::equivalent(
                engine.observe().as_ref().unwrap(),
                case.expected.as_ref().unwrap(),
                &mut Default::default()
            ));
            if chr_compiled::COLLECT_METRICS {
                assert!(
                    engine.stats().candidate_visits <= 30 * (n as u64 + 1),
                    "known active keys should bound partner discovery: n={n}, visits={}",
                    engine.stats().candidate_visits
                );
            }
        }
    }
}

#[test]
fn later_anchor_preserves_unknown_structure_repeated_slots_and_resources() {
    use chr_syntax::{Answer, Query, Var, atom, c, t, v};
    for execution in [Execution::Generic, Execution::Generated] {
        let prepared = PreparedRuleset::bundled(14, execution).unwrap();
        for right in [
            c("right", [v(1), v(0)]),
            c("right", [t("f", [v(0)]), v(0)]),
            c("right", [t("f", [atom("a")]), atom("b")]),
            c("right", [t("f", [atom("a")]), atom("a")]),
        ] {
            let succeeds = right == c("right", [t("f", [atom("a")]), atom("a")]);
            for reverse in [false, true] {
                let mut input = vec![
                    c("left", [atom("a")]),
                    c("left", [atom("a")]),
                    right.clone(),
                ];
                if reverse {
                    input.reverse();
                }
                let expected = Answer {
                    outputs: vec![("x".into(), v(0)), ("y".into(), v(1))],
                    residual: if succeeds {
                        vec![c("left", [atom("a")]), c("hit", [])]
                    } else {
                        input.clone()
                    },
                };
                let mut traces = vec![];
                for access in [Access::Scan, Access::Indexed] {
                    let query = Query {
                        constraints: input.clone(),
                        outputs: vec![("x".into(), Var(0)), ("y".into(), Var(1))],
                    };
                    let mut engine = prepared.start(query, Policy::Active, access).unwrap();
                    engine.enable_audit();
                    let mut complete = false;
                    for _ in 0..1000 {
                        if engine.advance(1).exhausted {
                            complete = true;
                            break;
                        }
                    }
                    assert!(complete);
                    assert!(chr_observe::equivalent(
                        engine.observe().as_ref().unwrap(),
                        &expected,
                        &mut Default::default()
                    ));
                    for commit in engine.audit() {
                        support::check_commit(&fixtures::programs()[14], commit);
                    }
                    assert!(support::terminal(&fixtures::programs()[14], &engine.view()));
                    traces.push(engine.trace().to_vec());
                }
                assert_eq!(traces[0], traces[1]);
            }
        }
    }
}
