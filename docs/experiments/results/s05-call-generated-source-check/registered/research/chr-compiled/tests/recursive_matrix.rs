//! Independent bounded cross-product for the sealed recursive certificate.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent, recursive::Prepared};
use chr_syntax::{Answer, Query, Rule, Var, and, atom, c, eq, t, v};

fn source(accumulate: bool) -> Vec<Rule> {
    let body = if accumulate {
        and(vec![
            eq(v(3), t("wrap", [v(1)])),
            c("relation", [v(0), v(3), v(2)]).into(),
        ])
    } else {
        and(vec![
            eq(v(2), t("wrap", [v(3)])),
            c("relation", [v(0), v(1), v(3)]).into(),
        ])
    };
    vec![
        Rule::simplify(
            "finish",
            [c("relation", [atom("done"), v(1), v(2)])],
            eq(v(1), v(2)),
        ),
        Rule::simplify(
            "continue",
            [c("relation", [t("more", [v(0)]), v(1), v(2)])],
            body,
        ),
    ]
}
fn compiled(p: &PreparedRuleset, q: Query) -> Vec<Answer> {
    let mut e = p
        .start(q, Policy::Global, Access::Indexed)
        .unwrap()
        .into_search();
    let mut answers = Vec::new();
    for _ in 0..100_000 {
        match e.tick() {
            SearchEvent::Complete(mut b) => answers.push(b.engine.observe().unwrap()),
            SearchEvent::Exhausted => return answers,
            _ => (),
        }
    }
    panic!("finite correspondence matrix cutoff");
}
#[test]
fn alias_and_constructor_cross_product_preserves_full_raw_observations() {
    let payloads = [
        v(10),
        atom("a"),
        t("pair", [v(10), v(10)]),
        t("pair", [v(10), v(11)]),
    ];
    let results = [
        v(11),
        v(10),
        atom("a"),
        atom("b"),
        t("wrap", [v(10)]),
        t("pair", [v(10), v(11)]),
        t("wrap", [t("wrap", [v(10)])]),
    ];
    let mut cases = 0;
    for accumulate in [false, true] {
        let rules = source(accumulate);
        let direct = Prepared::new(rules.clone()).unwrap();
        let current = PreparedRuleset::new(rules.clone(), None)
            .unwrap()
            .specialize_inferred();
        for depth in [0, 1, 3] {
            let control = (0..depth).fold(atom("done"), |x, _| t("more", [x]));
            for payload in &payloads {
                for result in &results {
                    let q = Query {
                        constraints: vec![c(
                            "relation",
                            [control.clone(), payload.clone(), result.clone()],
                        )],
                        outputs: vec![
                            ("x".into(), Var(10)),
                            ("y".into(), Var(11)),
                            ("unused".into(), Var(99)),
                        ],
                    };
                    let expected = scalar::run(&rules, &q, 100_000);
                    let actual: Vec<_> = direct.execute(q.clone()).unwrap().into_iter().collect();
                    scalar::same_raw(actual.clone(), expected);
                    scalar::same_raw(actual, compiled(&current, q));
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 168);
}

#[test]
fn mathematical_spine_and_alias_expectations() {
    for accumulate in [false, true] {
        let p = Prepared::new(source(accumulate)).unwrap();
        for depth in [0, 1, 3, 12] {
            let control = (0..depth).fold(atom("done"), |x, _| t("more", [x]));
            let input = |payload, result| Query {
                constraints: vec![c("relation", [control.clone(), payload, result])],
                outputs: vec![
                    ("x".into(), Var(10)),
                    ("y".into(), Var(11)),
                    ("unused".into(), Var(99)),
                ],
            };
            let answer = p.execute(input(v(10), v(11))).unwrap().unwrap();
            let expected = Answer {
                outputs: vec![
                    ("x".into(), v(0)),
                    ("y".into(), (0..depth).fold(v(0), |x, _| t("wrap", [x]))),
                    ("unused".into(), v(1)),
                ],
                residual: vec![],
            };
            scalar::same_raw(vec![answer], vec![expected]);
            assert_eq!(
                p.execute(input(v(10), v(10))).unwrap().is_some(),
                depth == 0
            );
            assert!(p.execute(input(atom("a"), atom("b"))).unwrap().is_none());
            let result = (0..depth).fold(atom("a"), |x, _| t("wrap", [x]));
            assert!(p.execute(input(atom("a"), result)).unwrap().is_some());
        }
    }
}
