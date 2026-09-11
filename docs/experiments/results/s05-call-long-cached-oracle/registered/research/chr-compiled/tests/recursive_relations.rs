#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent, recursive::Prepared};
use chr_syntax::{Answer, Query, Rule, Var, and, atom, c, eq, t, v};
fn rules() -> Vec<Rule> {
    vec![
        Rule::simplify(
            "base",
            [c("walk", [atom("end"), v(9), v(2)])],
            eq(v(2), v(9)),
        ),
        Rule::simplify(
            "step",
            [c("walk", [t("next", [v(7)]), v(9), v(2)])],
            and(vec![
                eq(v(3), t("pair", [v(9), v(4)])),
                c("walk", [v(7), v(3), v(2)]).into(),
            ]),
        ),
    ]
}
fn query(n: usize) -> Query {
    let control = (0..n).fold(atom("end"), |x, _| t("next", [x]));
    Query {
        constraints: vec![c("walk", [control, v(10), v(11)])],
        outputs: vec![
            ("x".into(), Var(10)),
            ("y".into(), Var(11)),
            ("unused".into(), Var(99)),
        ],
    }
}
fn check(rules: Vec<Rule>, q: Query, direct: Option<Answer>) {
    let expected = scalar::run_traced(&rules, &q, 100_000)
        .into_iter()
        .map(|x| x.0)
        .collect();
    scalar::same_raw(direct.clone().into_iter().collect(), expected);
    let mut e = PreparedRuleset::new(rules, None)
        .unwrap()
        .specialize_inferred()
        .start(q, Policy::Global, Access::Indexed)
        .unwrap()
        .into_search();
    let mut actual = vec![];
    for _ in 0..100_000 {
        match e.tick() {
            SearchEvent::Complete(mut b) => actual.push(b.engine.observe().unwrap()),
            SearchEvent::Exhausted => {
                scalar::same_raw(actual, direct.into_iter().collect());
                return;
            }
            _ => (),
        }
    }
    panic!("cutoff")
}
#[test]
fn reused_equations_preserve_fresh_joint_aliases() {
    let prepared = Prepared::new(rules()).unwrap();
    for n in [0, 1, 2, 8, 16, 1] {
        let q = query(n);
        let answer = prepared.execute(q.clone()).unwrap().unwrap();
        assert!(answer.residual.is_empty());
        if n == 0 {
            assert_eq!(answer.outputs[0].1, answer.outputs[1].1);
        }
        assert_ne!(answer.outputs[0].1, answer.outputs[2].1);
        check(rules(), q, Some(answer));
    }
}
#[test]
fn conflicts_and_occurs_fail_without_unsupported_fallback() {
    for body in [
        eq(v(9), t("self", [v(9)])),
        and(vec![eq(v(9), atom("a")), eq(v(9), atom("b"))]),
    ] {
        let mut rs = rules();
        rs[0].body = body;
        let p = Prepared::new(rs.clone()).unwrap();
        let q = query(2);
        assert_eq!(p.execute(q.clone()).unwrap(), None);
        check(rs, q, None);
    }
}
#[test]
fn unknown_control_and_context_are_explicitly_unsupported() {
    let p = Prepared::new(rules()).unwrap();
    for control in [
        v(88),
        t("next", [v(88)]),
        atom("other"),
        t("end", [atom("end")]),
    ] {
        let mut q = query(0);
        q.constraints[0].args[0] = control;
        assert!(p.execute(q).is_err());
    }
    let mut q = query(0);
    q.constraints.push(c("extra", []));
    assert!(p.execute(q).is_err());
}
#[test]
fn control_equations_are_executed_and_entire_query_checked_first() {
    let mut rs = rules();
    rs[1].body = and(vec![
        eq(v(7), atom("end")),
        c("walk", [v(7), v(9), v(2)]).into(),
    ]);
    let prepared = Prepared::new(rs.clone()).unwrap();
    for n in [0, 1, 2, 5] {
        let q = query(n);
        let answer = prepared.execute(q.clone()).unwrap();
        assert_eq!(answer.is_some(), n <= 1);
        check(rs.clone(), q, answer);
    }
    let mut q = query(0);
    q.constraints[0].args[0] = t("next", [t("next", [v(55)])]);
    assert!(prepared.execute(q).is_err());
}
#[test]
fn inferred_names_positions_order_and_different_bodies() {
    for position in 0..3 {
        let permute = |mut args: Vec<chr_syntax::Term>| {
            args.swap(0, position);
            args
        };
        let rs = vec![
            Rule::simplify(
                "unwind",
                [c(
                    "relation",
                    permute(vec![t("branch", [v(400)]), v(401), v(402)]),
                )],
                and(vec![
                    eq(v(403), t("cell", [v(404), v(401)])),
                    c("relation", permute(vec![v(400), v(403), v(402)])).into(),
                ]),
            ),
            Rule::simplify(
                "finish",
                [c("relation", permute(vec![atom("leaf"), v(1), v(2)]))],
                and(vec![eq(v(3), v(1)), eq(v(2), t("wrap", [v(3), v(3)]))]),
            ),
        ];
        let prepared = Prepared::new(rs.clone()).unwrap();
        assert_eq!(prepared.control_position(), position);
        assert_eq!(prepared.predicate(), ("relation", 3));
        for n in [0, 1, 4, 9] {
            let spine = (0..n).fold(atom("leaf"), |x, _| t("branch", [x]));
            let q = Query {
                constraints: vec![c("relation", permute(vec![spine, v(0), v(1)]))],
                outputs: vec![
                    ("a".into(), Var(0)),
                    ("b".into(), Var(1)),
                    ("fresh".into(), Var(403)),
                ],
            };
            let answer = prepared.execute(q.clone()).unwrap().unwrap();
            let chr_syntax::Term::App(name, children) = &answer.outputs[1].1 else {
                panic!("wrapped result")
            };
            assert_eq!(name, "wrap");
            assert_eq!(children[0], children[1]);
            check(rs.clone(), q, Some(answer));
        }
    }
}
#[test]
fn reject_source_and_query_context_outside_certificate() {
    use chr_syntax::{Goal, Guard};
    let mut invalid = vec![];
    let mut rs = rules();
    rs.push(Rule::propagate(
        "observer",
        [c("walk", [v(0), v(1), v(2)])],
        Goal::True,
    ));
    invalid.push(rs);
    let mut rs = rules();
    rs[1].name = rs[0].name.clone();
    invalid.push(rs);
    let mut rs = rules();
    rs[0].guards.push(Guard::Equal(v(9), v(9)));
    invalid.push(rs);
    let mut rs = rules();
    rs[0].kept.push(c("token", []));
    invalid.push(rs);
    let mut rs = rules();
    rs[0].removed[0].args[2] = v(9);
    invalid.push(rs);
    let mut rs = rules();
    rs[1].removed[0].args[2] = v(7);
    invalid.push(rs);
    let mut rs = rules();
    rs[0].body = c("walk", [atom("end"), v(9), v(2)]).into();
    invalid.push(rs);
    let mut rs = rules();
    rs[0].body = Goal::Fail;
    invalid.push(rs);
    let mut rs = rules();
    rs[1].body = chr_syntax::or(Goal::True, Goal::True);
    invalid.push(rs);
    let mut rs = rules();
    rs[1].body = and(vec![c("walk", [v(7), v(9), v(2)]).into(), eq(v(2), v(9))]);
    invalid.push(rs);
    let mut rs = rules();
    rs[1].body = c("walk", [v(9), v(9), v(2)]).into();
    invalid.push(rs);
    for rs in invalid {
        assert!(Prepared::new(rs).is_err());
    }
    let p = Prepared::new(rules()).unwrap();
    let mut q = query(0);
    q.outputs.push(q.outputs[0].clone());
    assert!(p.execute(q).is_err());
    let mut q = query(0);
    q.constraints[0].name = "foreign".into();
    assert!(p.execute(q).is_err());
}
