#[path = "../experiments/subscription_join.rs"]
mod join;
#[path = "../experiments/subscription_low_yield.rs"]
mod low_yield;
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../experiments/subscription_runtime.rs"]
mod runtime;
#[path = "../experiments/subscription_source.rs"]
mod source;
use chr_syntax::{Answer, Constraint, Query, Term, atom, c, t, v};
use source::{query, source_rules};
fn open() -> Term {
    t("open", [atom("k"), atom("yes"), atom("d")])
}
fn ask(round: &str) -> Term {
    t("ask", [atom("d"), atom(round)])
}
fn close() -> Term {
    t("close", [atom("d")])
}
fn rows() -> Vec<Constraint> {
    vec![
        c("left", [atom("k"), t("f", [atom("a")])]),
        c("middle", [atom("a"), t("g", [atom("b")])]),
        c("right", [atom("b"), t("h", [atom("yes")])]),
    ]
}
fn receipt(round: &str) -> Constraint {
    c(
        "receipt",
        [
            atom("d"),
            atom(round),
            t("f", [atom("a")]),
            t("g", [atom("b")]),
            t("h", [atom("yes")]),
        ],
    )
}
fn check(q: &Query, consuming: bool, expected: Vec<Constraint>) -> Answer {
    let rules = source_rules(consuming);
    let mut answers = oracle::run(&rules, q, 200_000);
    assert_eq!(answers.len(), 1);
    let answer = answers.pop().unwrap();
    let mut actual: Vec<_> = answer
        .residual
        .iter()
        .filter(|x| x.name == "receipt")
        .cloned()
        .collect();
    let mut expected = expected;
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
    let prepared = runtime::Prepared::new(&rules).unwrap();
    for mode in [
        join::Mode::Indexed,
        join::Mode::Eager,
        join::Mode::Subscribed,
    ] {
        for quantum in [1, 37] {
            let mut e = prepared.start(q.clone(), mode).unwrap();
            assert!(!e.advance(0));
            assert!(e.observe().is_err());
            for _ in 0..200_000 / quantum {
                if e.advance(quantum) {
                    break;
                }
            }
            oracle::same_raw(e.observe().unwrap(), vec![answer.clone()]);
            let _ = (e.stats(), e.retained());
        }
    }
    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
        let mut traces = vec![];
        for code in [
            None,
            Some(chr_compiled::access_subscription_bundled(usize::from(
                consuming,
            ))),
        ] {
            let p = chr_compiled::PreparedRuleset::new(rules.clone(), code).unwrap();
            let mut engine = p
                .start(q.clone(), chr_compiled::Policy::Global, access)
                .unwrap();
            engine.enable_trace();
            let end = engine.advance(200_000);
            assert!(end.exhausted && !end.failed);
            oracle::same_raw(vec![engine.observe().unwrap()], vec![answer.clone()]);
            traces.push(engine.trace().to_vec());
        }
        assert_eq!(
            traces[0], traces[1],
            "generated access changed source occurrence competition"
        );
    }
    answer
}
#[test]
fn live_demand_repeated_pulses_retirement_and_reopening() {
    for first in [false, true] {
        let q = query(
            rows(),
            vec![
                ask("absent"),
                open(),
                ask("one"),
                ask("two"),
                close(),
                ask("retired"),
                open(),
                ask("three"),
                close(),
            ],
            first,
        );
        let answer = check(
            &q,
            false,
            vec![receipt("one"), receipt("two"), receipt("three")],
        );
        let mut expected = rows();
        expected.extend([
            receipt("one"),
            receipt("two"),
            receipt("three"),
            c("done", []),
        ]);
        oracle::same_raw(
            vec![answer],
            vec![Answer {
                outputs: vec![("x".into(), v(50)), ("y".into(), v(51))],
                residual: expected,
            }],
        );
    }
}
#[test]
fn occurrence_multiplicity_selectivity_and_consuming_shared_partners() {
    for nl in [1, 3] {
        for nm in [1, 2] {
            for nr in [1, 3] {
                for consuming in [false, true] {
                    for first in [false, true] {
                        let base = rows();
                        let mut data = vec![base[0].clone(); nl];
                        data.extend(vec![base[1].clone(); nm]);
                        data.extend(vec![base[2].clone(); nr]);
                        data.push(c("right", [atom("b"), t("h", [atom("no")])]));
                        data.push(c("middle", [atom("a"), t("g", [atom("disconnected")])]));
                        let q = query(data, vec![open(), ask("one"), ask("two"), close()], first);
                        let expected = if consuming {
                            vec![receipt("one"); nr]
                        } else {
                            let mut r = vec![receipt("one"); nl * nm * nr];
                            r.extend(vec![receipt("two"); nl * nm * nr]);
                            r
                        };
                        let answer = check(&q, consuming, expected);
                        assert_eq!(
                            answer.residual.iter().filter(|c| c.name == "right").count(),
                            if consuming { 1 } else { nr + 1 }
                        );
                    }
                }
            }
        }
    }
}
#[test]
fn updates_to_each_relation_and_demand_retirement() {
    for relation in ["left", "middle", "right"] {
        for first in [false, true] {
            let base = rows();
            let r = base.iter().find(|r| r.name == relation).unwrap();
            let q = query(
                base.clone(),
                vec![
                    open(),
                    ask("before"),
                    t(&format!("remove_{relation}"), r.args.clone()),
                    ask("missing"),
                    t(&format!("insert_{relation}"), r.args.clone()),
                    ask("restored"),
                    close(),
                    t(&format!("insert_{relation}"), r.args.clone()),
                    ask("retired"),
                ],
                first,
            );
            check(&q, false, vec![receipt("before"), receipt("restored")]);
        }
    }
}
#[test]
fn aliases_and_structural_binding_enable_matches_without_binding_during_match() {
    for first in [false, true] {
        let mut data = rows();
        data[0].args[1] = v(50);
        let q = query(
            data,
            vec![
                open(),
                ask("unknown"),
                t("bind", [v(50), v(51)]),
                ask("alias"),
                t("bind", [v(51), t("f", [atom("a")])]),
                ask("enabled"),
                close(),
            ],
            first,
        );
        let answer = check(&q, false, vec![receipt("enabled")]);
        assert_eq!(
            answer.outputs,
            vec![
                ("x".into(), t("f", [atom("a")])),
                ("y".into(), t("f", [atom("a")]))
            ]
        );
    }
}
#[test]
fn duplicate_demand_occurrences_have_independent_history_and_retirement() {
    let q = query(
        rows(),
        vec![
            open(),
            open(),
            ask("both"),
            close(),
            ask("one"),
            close(),
            ask("none"),
        ],
        false,
    );
    check(
        &q,
        false,
        vec![receipt("both"), receipt("both"), receipt("one")],
    );
}
#[test]
fn unmatched_resource_update_blocks_driver_with_exact_residual() {
    let ops = vec![
        t("remove_right", [atom("missing"), t("h", [atom("yes")])]),
        open(),
        ask("unreached"),
    ];
    let q = query(rows(), ops, false);
    let a = check(&q, false, vec![]);
    oracle::same_raw(
        vec![a],
        vec![Answer {
            outputs: vec![("x".into(), v(50)), ("y".into(), v(51))],
            residual: q.constraints,
        }],
    );
}

#[test]
fn binding_changes_demand_endpoints_shared_rows_and_late_receipts() {
    for consuming in [false, true] {
        for first in [false, true] {
            let data = vec![
                c("left", [v(50), t("f", [atom("a")])]),
                c("left", [atom("k"), t("f", [atom("a")])]),
                c("middle", [atom("a"), t("g", [atom("b")])]),
                c("right", [atom("b"), t("h", [v(51)])]),
            ];
            let q = query(
                data,
                vec![
                    t("open", [v(50), v(51), atom("d")]),
                    ask("before"),
                    t("bind", [v(50), atom("k")]),
                    t("bind", [v(51), atom("yes")]),
                    ask("after"),
                    close(),
                ],
                first,
            );
            let mut before = receipt("before");
            before.args[4] = t("h", [atom("yes")]);
            let mut expected = vec![before];
            if !consuming {
                expected.extend([receipt("after"), receipt("after")]);
            }
            check(&q, consuming, expected);
        }
    }
}

#[test]
fn consuming_source_priority_is_preserved_across_distinct_matching_demands() {
    let mut data = rows();
    data.push(c("left", [atom("other"), t("f", [atom("other_a")])]));
    data.push(c("middle", [atom("other_a"), t("g", [atom("b")])]));
    let q = query(
        data,
        vec![
            t("open", [atom("other"), atom("yes"), atom("d")]),
            open(),
            ask("priority"),
            close(),
            ask("empty"),
        ],
        false,
    );
    let a = check(&q, true, vec![receipt("priority")]);
    assert_eq!(a.residual.iter().filter(|c| c.name == "demand").count(), 1);
}

#[test]
fn changed_prepared_queries_cancellation_failure_and_certification() {
    let rules = source_rules(false);
    let p = runtime::Prepared::new(&rules).unwrap();
    let mut changed = rules.clone();
    changed.swap(0, 1);
    assert!(runtime::Prepared::new(&changed).is_err());
    for mode in [
        join::Mode::Indexed,
        join::Mode::Eager,
        join::Mode::Subscribed,
    ] {
        for round in ["a", "b", "a"] {
            let q = query(rows(), vec![open(), ask(round), close()], false);
            let expected = oracle::run(&rules, &q, 200_000);
            for prefix in [0, 1, 2, 3, 4] {
                let mut canceled = p.start(q.clone(), mode).unwrap();
                canceled.advance(prefix);
                drop(canceled);
            }
            let mut e = p.start(q, mode).unwrap();
            assert!(e.advance(100));
            oracle::same_raw(e.observe().unwrap(), expected);
        }
        for equations in [
            vec![t("bind", [v(50), t("cycle", [v(50)])])],
            vec![t("bind", [v(50), atom("a")]), t("bind", [v(50), atom("b")])],
        ] {
            let q = query(rows(), equations, false);
            assert!(oracle::run(&rules, &q, 200_000).is_empty());
            let mut e = p.start(q, mode).unwrap();
            assert!(e.advance(100));
            assert!(e.observe().unwrap().is_empty());
        }
    }
}

#[test]
fn existing_single_head_specialization_has_no_eligible_region_here() {
    for consuming in [false, true] {
        let rules = source_rules(consuming);
        let p = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
        let eligibility = p.region_eligibility();
        assert!(eligibility.iter().all(|e| !e.eligible));
        for head in rules.iter().flat_map(|r| r.kept.iter().chain(&r.removed)) {
            assert!(
                p.specialize_checked(&[(head.name.clone(), head.args.len())])
                    .is_err()
            );
        }
        let q = query(rows(), vec![open(), ask("one"), close()], false);
        let mut e = p
            .specialize_inferred()
            .start(
                q.clone(),
                chr_compiled::Policy::Global,
                chr_compiled::Access::Indexed,
            )
            .unwrap();
        assert!(e.advance(200_000).exhausted);
        assert_eq!(e.stats().specialized_candidates, 0);
        oracle::same_raw(vec![e.observe().unwrap()], oracle::run(&rules, &q, 200_000));
        println!("S01_SUBSCRIPTION_ELIGIBILITY consuming={consuming} {eligibility:?}");
    }
}

#[test]
fn low_yield_sources_preserve_all_dead_rows_and_exact_bridge_receipts() {
    for f in low_yield::FAMILIES {
        for n in [4, 8] {
            for rounds in [2, 16] {
                let q = low_yield::query(f, n, rounds, 7);
                let expected = (0..rounds)
                    .map(|r| {
                        c(
                            "receipt",
                            [
                                atom("d"),
                                atom(&format!("r{r}")),
                                t("f", [atom("a0")]),
                                t("g", [atom("b0")]),
                                t("h", [atom("value")]),
                            ],
                        )
                    })
                    .collect();
                let a = check(&q, false, expected);
                assert_eq!(
                    a.residual.iter().filter(|r| r.name == "middle").count(),
                    2 * n * n + 1
                );
                assert_eq!(a.residual.iter().filter(|r| r.name == "left").count(), n);
                assert_eq!(a.residual.iter().filter(|r| r.name == "right").count(), n);
            }
        }
    }
}
