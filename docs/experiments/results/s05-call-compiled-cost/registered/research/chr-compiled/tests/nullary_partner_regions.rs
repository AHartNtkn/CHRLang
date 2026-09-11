//! Source qualification of direct selection with a distinct nullary consumable.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
use chr_syntax::{Goal, Query, Rule, Var, c, eq, t, v};
#[test]
fn inferred_pair_consumes_both_occurrences_and_executes_direct_selection() {
    let rule = Rule::simplify(
        "take",
        [c("job", [t("f", [v(0)]), v(1)]), c("permit", [])],
        eq(v(1), t("g", [v(0)])),
    );
    let prepared = PreparedRuleset::new(vec![rule.clone()], None).unwrap();
    assert!(
        prepared
            .region_eligibility()
            .iter()
            .find(|e| e.predicate == ("job".into(), 2))
            .unwrap()
            .eligible
    );
    let q = Query {
        constraints: vec![c("job", [t("f", [v(3)]), v(4)]), c("permit", [])],
        outputs: vec![("out".into(), Var(4))],
    };
    for access in [Access::Scan, Access::Indexed] {
        let mut e = prepared
            .specialize_inferred()
            .start(q.clone(), Policy::Global, access)
            .unwrap();
        e.enable_trace();
        assert!(e.advance(200_000).exhausted);
        assert_eq!(e.trace(), &[(0, vec![0, 1])]);
        if chr_compiled::COLLECT_METRICS {
            assert_eq!(e.stats().specialized_applications, 1);
            assert!(e.stats().specialized_head_instructions > 0);
            assert_eq!(e.stats().cursor_steps, 0);
        }
        scalar::same_raw(
            e.observe().into_iter().collect(),
            scalar::run(std::slice::from_ref(&rule), &q, 200_000),
        );
    }
}

type Seen = (chr_syntax::Answer, Vec<(usize, Vec<u64>)>);
fn collect(p: &PreparedRuleset, q: &Query, access: Access) -> Vec<Seen> {
    let mut e = p.start_search(q.clone(), Policy::Global, access).unwrap();
    e.enable_trace();
    let mut seen = vec![];
    for _ in 0..200_000 {
        match e.tick() {
            SearchEvent::Complete(mut b) => {
                seen.push((b.engine.observe().unwrap(), b.engine.trace().to_vec()))
            }
            SearchEvent::Exhausted => return seen,
            _ => (),
        }
    }
    panic!("nullary partner source gate cutoff");
}
fn check(rules: Vec<Rule>, q: Query) {
    let expected = scalar::run_traced(&rules, &q, 200_000);
    let p = PreparedRuleset::new(rules, None).unwrap();
    let specialized = p.specialize_inferred();
    for access in [Access::Scan, Access::Indexed] {
        let ordinary = collect(&p, &q, access);
        let actual = collect(&specialized, &q, access);
        scalar::same_raw(
            ordinary.iter().map(|(a, _)| a.clone()).collect(),
            expected.iter().map(|(a, _)| a.clone()).collect(),
        );
        scalar::same_raw(
            actual.iter().map(|(a, _)| a.clone()).collect(),
            expected.iter().map(|(a, _)| a.clone()).collect(),
        );
        let mut rules = actual
            .iter()
            .map(|(_, t)| t.iter().map(|(r, _)| *r).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut expected_rules = expected.iter().map(|(_, t)| t.clone()).collect::<Vec<_>>();
        rules.sort();
        expected_rules.sort();
        assert_eq!(rules, expected_rules);
        let mut ordinary = ordinary;
        for (answer, trace) in actual {
            let i = ordinary
                .iter()
                .position(|(a, t)| {
                    *t == trace && chr_observe::equivalent(a, &answer, &mut Default::default())
                })
                .expect("selected occurrences differ");
            ordinary.swap_remove(i);
        }
        assert!(ordinary.is_empty());
    }
}
#[test]
fn source_shapes_guards_aliases_and_duplicate_resources_preserve_full_traces() {
    use chr_syntax::{Guard, and, atom, or};
    for (job, permit) in [("job", "permit"), ("renamed", "ticket"), ("same", "same")] {
        for repeat in [false, true] {
            for guarded in [false, true] {
                for tokens in [0, 1, 3] {
                    for reverse in [false, true] {
                        let mut rule = Rule::simplify(
                            "consume",
                            [
                                c(
                                    job,
                                    [t("pair", [v(0), v(if repeat { 0 } else { 1 })]), v(2)],
                                ),
                                c(permit, []),
                            ],
                            and([
                                eq(v(2), t("out", [v(0), v(3), v(3)])),
                                c("record", [v(3), v(3)]).into(),
                            ]),
                        );
                        if guarded {
                            rule.guards.push(Guard::Equal(v(0), atom("a")));
                        }
                        let mut facts = vec![
                            c(job, [t("pair", [atom("a"), atom("a")]), v(20)]),
                            c(job, [t("pair", [v(40), atom("b")]), v(21)]),
                            c(job, [t("pair", [atom("b"), atom("b")]), v(22)]),
                        ];
                        if reverse {
                            facts.reverse();
                        }
                        facts.extend((0..tokens).map(|_| c(permit, [])));
                        // Same name with another arity is not an available resource.
                        facts.push(c(permit, [atom("wrong-arity")]));
                        check(
                            vec![rule],
                            Query {
                                constraints: facts,
                                outputs: vec![
                                    ("a".into(), Var(20)),
                                    ("b".into(), Var(21)),
                                    ("c".into(), Var(22)),
                                    ("partial".into(), Var(40)),
                                ],
                            },
                        );
                    }
                }
            }
        }
    }
    for body in [
        Goal::Fail,
        or(Goal::True, Goal::True),
        and([c("permit", []).into(), eq(v(1), t("loop", [v(1)]))]),
    ] {
        check(
            vec![Rule::simplify(
                "branch",
                [c("job", [v(0), v(1)]), c("permit", [])],
                body,
            )],
            Query {
                constraints: vec![c("job", [v(1), v(1)]), c("permit", [])],
                outputs: vec![("out".into(), Var(1))],
            },
        );
    }
    check(
        vec![Rule::simplify(
            "nullary",
            [c("go", []), c("permit", [])],
            c("done", []).into(),
        )],
        Query {
            constraints: vec![c("permit", []), c("go", []), c("permit", [])],
            outputs: vec![],
        },
    );
}
#[test]
fn absence_late_supply_and_mixed_rule_order_do_not_skip_source_work() {
    use chr_syntax::{and, atom};
    let consume = Rule::simplify(
        "consume",
        [c("job", [t("f", [v(0)]), v(1)]), c("permit", [])],
        eq(v(1), v(0)),
    );
    let q = Query {
        constraints: vec![c("job", [t("f", [atom("a")]), v(0)])],
        outputs: vec![("out".into(), Var(0))],
    };
    for access in [Access::Scan, Access::Indexed] {
        let p = PreparedRuleset::new(vec![consume.clone()], None)
            .unwrap()
            .specialize_inferred();
        let mut e = p.start(q.clone(), Policy::Global, access).unwrap();
        assert!(e.advance(200_000).exhausted);
        assert_eq!(e.stats().specialized_candidates, 0);
        assert_eq!(e.stats().cursor_steps, 0);
        assert_eq!(e.stats().specialized_head_instructions, 0);
        scalar::same_raw(
            e.observe().into_iter().collect(),
            scalar::run(std::slice::from_ref(&consume), &q, 200_000),
        );
    }
    for reverse in [false, true] {
        let supply = Rule::simplify(
            "supply",
            [c("supply", [v(0)])],
            and([eq(v(0), t("f", [atom("late")])), c("permit", []).into()]),
        );
        let mut rules = vec![consume.clone(), supply];
        if reverse {
            rules.reverse();
        }
        check(
            rules,
            Query {
                constraints: vec![c("job", [v(0), v(1)]), c("supply", [v(0)])],
                outputs: vec![("out".into(), Var(1))],
            },
        );
    }
    let broad = Rule::simplify(
        "broad",
        [c("job", [v(0), v(1)]), c("permit", [])],
        eq(v(1), atom("broad")),
    );
    check(
        vec![consume, broad],
        Query {
            constraints: vec![
                c("job", [atom("opaque"), v(0)]),
                c("job", [t("f", [atom("specific")]), v(1)]),
                c("permit", []),
            ],
            outputs: vec![("first".into(), Var(0)), ("second".into(), Var(1))],
        },
    );
}
#[test]
fn unsupported_partners_and_other_head_uses_are_rejected_explicitly() {
    use chr_syntax::atom;
    let cases = vec![
        vec![Rule::simplify(
            "argument",
            [c("p", [v(0)]), c("resource", [v(1)])],
            Goal::True,
        )],
        vec![Rule::simplify("same", [c("p", []), c("p", [])], Goal::True)],
        vec![Rule::simplify(
            "three",
            [c("p", [v(0)]), c("r", []), c("s", [])],
            Goal::True,
        )],
        vec![Rule::propagate(
            "kept",
            [c("p", [v(0)]), c("r", [])],
            Goal::True,
        )],
        vec![
            Rule::simplify("leading", [c("p", []), c("r", [])], Goal::True),
            Rule::simplify("partner", [c("q", [atom("a")]), c("p", [])], Goal::True),
        ],
    ];
    for rules in cases {
        let arity = rules[0]
            .kept
            .first()
            .or(rules[0].removed.first())
            .unwrap()
            .args
            .len();
        let p = PreparedRuleset::new(rules, None).unwrap();
        assert!(p.specialize_checked(&[("p".into(), arity)]).is_err());
    }
    let p = PreparedRuleset::new(
        vec![Rule::simplify(
            "ok",
            [c("p", [v(0)]), c("r", [])],
            Goal::True,
        )],
        None,
    )
    .unwrap()
    .specialize_inferred();
    assert!(
        p.start(
            Query {
                constraints: vec![],
                outputs: vec![]
            },
            Policy::Active,
            Access::Scan
        )
        .is_err()
    );
}
