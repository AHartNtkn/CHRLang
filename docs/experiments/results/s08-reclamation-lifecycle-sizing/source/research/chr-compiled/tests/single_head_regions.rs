//! Full source/trace gate for scheduling-preserving single-head specialization.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
use chr_syntax::{Answer, Constraint, Goal, Guard, Query, Rule, Var, and, atom, c, eq, or, t, v};
type Observation = (Answer, Vec<(usize, Vec<u64>)>);
fn input(cs: Vec<Constraint>) -> Query {
    Query {
        constraints: cs,
        outputs: vec![("x".into(), Var(0)), ("y".into(), Var(1))],
    }
}
fn collect(p: &PreparedRuleset, q: Query) -> Vec<Observation> {
    let mut e = p
        .start(q, Policy::Global, Access::Indexed)
        .unwrap()
        .into_search();
    e.enable_trace();
    let mut answers = vec![];
    for _ in 0..1_000_000 {
        match e.tick() {
            SearchEvent::Complete(mut b) => {
                answers.push((b.engine.observe().unwrap(), b.engine.trace().to_vec()))
            }
            SearchEvent::Exhausted => return answers,
            _ => (),
        }
    }
    panic!("finite region gate cutoff");
}
fn check(rules: Vec<Rule>, q: Query) {
    let expected = scalar::run_traced(&rules, &q, 100_000);
    let ordinary = collect(
        &PreparedRuleset::new(rules.clone(), None).unwrap(),
        q.clone(),
    );
    let specialized = collect(&specialize(rules), q);
    scalar::same_raw(
        specialized.iter().map(|(a, _)| a.clone()).collect(),
        expected.iter().map(|(a, _)| a.clone()).collect(),
    );
    let mut actual_traces: Vec<_> = ordinary
        .iter()
        .map(|(_, t)| t.iter().map(|(r, _)| *r).collect::<Vec<_>>())
        .collect();
    let mut expected_traces: Vec<_> = expected.into_iter().map(|(_, trace)| trace).collect();
    actual_traces.sort();
    expected_traces.sort();
    assert_eq!(actual_traces, expected_traces);
    let mut remaining = ordinary;
    for (answer, trace) in specialized {
        let i = remaining
            .iter()
            .position(|(a, t)| {
                *t == trace && chr_observe::equivalent(a, &answer, &mut Default::default())
            })
            .expect("full answer and occurrence trace differ");
        remaining.swap_remove(i);
    }
    assert!(remaining.is_empty());
}
fn specialize(rules: Vec<Rule>) -> PreparedRuleset {
    PreparedRuleset::new(rules, None)
        .unwrap()
        .specialize_inferred()
}
#[test]
fn mixed_context_and_nonground_source_cases() {
    let cases = vec![
        (
            vec![
                Rule::simplify(
                    "qa",
                    [c("q", [atom("a")]), c("token", [])],
                    c("A", []).into(),
                ),
                Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
                Rule::simplify("qb", [c("q", [v(0)]), c("token", [])], c("B", []).into()),
            ],
            input(vec![c("q", [v(0)]), c("bind", [v(0)]), c("token", [])]),
        ),
        (
            vec![
                Rule::simplify(
                    "constructor",
                    [c("p", [t("f", [v(0)])])],
                    c("f", [v(0)]).into(),
                ),
                Rule::simplify("opaque", [c("p", [v(0)])], c("opaque", [v(0)]).into()),
            ],
            input(vec![c("p", [v(0)])]),
        ),
        (
            vec![
                Rule::simplify("constructor", [c("p", [atom("a")])], c("hit", []).into()),
                Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
            ],
            input(vec![c("p", [v(0)]), c("p", [atom("a")]), c("bind", [v(0)])]),
        ),
        (
            vec![Rule::simplify(
                "fresh",
                [c("p", [v(0)])],
                c("out", [v(0), v(1)]).into(),
            )],
            input(vec![c("p", [atom("a")]), c("p", [atom("a")])]),
        ),
        (
            vec![
                Rule::simplify("repeated", [c("p", [v(0), v(0)])], c("out", [v(0)]).into()),
                Rule::simplify("alias", [c("alias", [v(0), v(1)])], eq(v(0), v(1))),
            ],
            input(vec![c("p", [v(0), v(1)]), c("alias", [v(0), v(1)])]),
        ),
        (
            vec![
                Rule::propagate("observe", [c("p", [v(0)])], c("seen", [v(0)]).into()),
                Rule::simplify("consume", [c("p", [v(0)])], c("out", [v(0)]).into()),
                Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
            ],
            input(vec![c("p", [v(0)]), c("bind", [v(0)])]),
        ),
        (
            vec![Rule::simplify(
                "choose",
                [c("p", [v(0)])],
                and(vec![
                    or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
                    c("out", [v(0), v(1)]).into(),
                ]),
            )],
            input(vec![c("p", [v(0)]), c("p", [v(1)])]),
        ),
        (
            vec![Rule::simplify(
                "fail",
                [c("p", [v(0)])],
                or(
                    and(vec![eq(v(0), atom("a")), eq(atom("a"), atom("b"))]),
                    eq(v(0), atom("b")),
                ),
            )],
            input(vec![c("p", [v(0)])]),
        ),
    ];
    for (rules, q) in cases {
        check(rules, q);
    }
}
#[test]
fn guard_and_constructor_crossing_corpus() {
    let values = [
        v(0),
        v(1),
        atom("a"),
        atom("b"),
        t("f", [v(0)]),
        t("f", [atom("a")]),
    ];
    for left in &values {
        for right in &values {
            for reversed in [false, true] {
                let mut r = Rule::simplify(
                    "guard",
                    [c("p", [v(0), v(1)])],
                    c("out", [v(0), v(1), v(2)]).into(),
                );
                r.guards = vec![Guard::Equal(v(0), v(1))];
                let mut rules = vec![
                    r,
                    Rule::simplify(
                        "constructor",
                        [c("p", [t("f", [v(0)]), v(1)])],
                        c("struct", [v(0), v(1)]).into(),
                    ),
                    Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
                ];
                if reversed {
                    rules.reverse();
                }
                check(
                    rules,
                    input(vec![
                        c("p", [left.clone(), right.clone()]),
                        c("bind", [v(0)]),
                    ]),
                );
            }
        }
    }
}
#[test]
fn eligibility_checks_every_head_use_and_arity() {
    let rules = vec![
        Rule::simplify("p1", [c("p", [v(0)])], Goal::True),
        Rule::simplify("p2", [c("p", [v(0), v(1)]), c("token", [])], Goal::True),
        Rule::simplify("q", [c("q", [])], Goal::True),
        Rule::propagate("observe", [c("q", [])], Goal::True),
    ];
    let p = PreparedRuleset::new(rules, None).unwrap();
    let reports = p.region_eligibility();
    assert!(
        reports
            .iter()
            .find(|r| r.predicate == ("p".into(), 1))
            .unwrap()
            .eligible
    );
    for pred in [("p".into(), 2), ("q".into(), 0), ("token".into(), 0)] {
        assert!(
            !reports
                .iter()
                .find(|r| r.predicate == pred)
                .unwrap()
                .eligible
        );
        assert!(p.specialize_checked(&[pred]).is_err());
    }
    assert!(p.specialize_checked(&[("absent".into(), 1)]).is_err());
    let chosen = p.specialize_checked(&[("p".into(), 1)]).unwrap();
    assert!(
        chosen
            .start(input(vec![]), Policy::Active, Access::Indexed)
            .is_err()
    );
}
#[test]
fn selected_calls_bypass_tuple_cursor_and_history() {
    let p = specialize(vec![Rule::simplify("erase", [c("p", [v(0)])], Goal::True)]);
    for _ in 0..3 {
        let mut e = p
            .start(
                input(vec![c("p", [v(0)]), c("p", [v(0)])]),
                Policy::Global,
                Access::Indexed,
            )
            .unwrap();
        assert!(e.advance(10_000).exhausted);
        assert!(e.observe().is_some());
        assert_eq!(e.retention().history, 0);
        assert_eq!(e.stats().cursor_steps, 0);
        assert_eq!(e.stats().history_checks, 0);
        if cfg!(feature = "metrics") {
            assert_eq!(e.stats().specialized_applications, 2);
            assert!(e.stats().specialized_candidates >= 2);
        }
    }
}
#[test]
fn recursive_choices_and_guard_local_identity() {
    let rules = vec![
        Rule::simplify(
            "base",
            [c("build", [atom("z"), v(0)])],
            eq(v(0), atom("nil")),
        ),
        Rule::simplify(
            "step",
            [c("build", [t("s", [v(0)]), v(1)])],
            and(vec![
                or(
                    eq(v(1), t("pair", [atom("a"), v(2)])),
                    eq(v(1), t("pair", [atom("b"), v(2)])),
                ),
                c("build", [v(0), v(2)]).into(),
            ]),
        ),
    ];
    for depth in [0, 1, 3, 2, 4] {
        let n = (0..depth).fold(atom("z"), |x, _| t("s", [x]));
        check(rules.clone(), input(vec![c("build", [n, v(0)])]));
    }
    for equal in [false, true] {
        let mut r = Rule::simplify("local", [c("p", [])], c("out", [v(2), v(3)]).into());
        r.guards = vec![Guard::Equal(v(2), if equal { v(2) } else { v(3) })];
        check(vec![r], input(vec![c("p", []), c("p", [])]));
    }
}
#[test]
fn generated_mixed_plans_and_prepared_query_leases() {
    for id in 0..chr_compiled::fixtures::ANALYTIC_PROGRAMS {
        let q = chr_compiled::fixtures::case(id, 3).query;
        let p = PreparedRuleset::bundled(id, chr_compiled::Execution::Generated).unwrap();
        assert_eq!(collect(&p, q.clone()), collect(&p.specialize_inferred(), q));
    }
    let p = specialize(vec![Rule::simplify("erase", [c("p", [v(0)])], Goal::True)]);
    let template = p
        .prepare_query(input(vec![]), Policy::Global, Access::Indexed)
        .unwrap();
    drop(p);
    for _ in 0..3 {
        let mut search = template.start(vec![c("p", [v(0)])]).unwrap();
        let mut complete = 0;
        for _ in 0..1000 {
            match search.tick() {
                SearchEvent::Complete(b) => {
                    complete += 1;
                    assert_eq!(b.engine.retention().history, 0);
                    assert_eq!(b.engine.stats().cursor_steps, 0)
                }
                SearchEvent::Exhausted => break,
                _ => (),
            }
        }
        assert_eq!(complete, 1);
    }
}
