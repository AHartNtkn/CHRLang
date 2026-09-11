#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_direct_relation::{Prepared, Step};
use chr_syntax::{Answer, Goal, Guard, Query, Rule, Term, Var, and, atom, c, eq, or, t, v};
fn table(name: &str, rows: &[Vec<Term>], arity: usize) -> Rule {
    let branches = rows.iter().map(|row| {
        and(row
            .iter()
            .enumerate()
            .map(|(i, x)| eq(v(i as u64), x.clone()))
            .collect::<Vec<_>>())
    });
    let body = branches.reduce(or).unwrap_or(Goal::Fail);
    Rule::simplify(
        name,
        [c(name, (0..arity).map(|i| v(i as u64)).collect::<Vec<_>>())],
        body,
    )
}
fn request() -> Rule {
    Rule::simplify(
        "request",
        [c("request", [v(0), v(1), v(2), v(3)])],
        and(vec![
            c("ab", [v(0), v(1)]).into(),
            c("bc", [v(1), v(2)]).into(),
        ]),
    )
}
fn run(p: &std::sync::Arc<Prepared>, q: &Query) -> Vec<Answer> {
    let mut e = p.start(q).unwrap();
    let mut answers = Vec::new();
    for _ in 0..100000 {
        match e.advance() {
            Step::Answer(a) => answers.push(a),
            Step::Exhausted => return answers,
            Step::Progress => (),
        }
    }
    panic!("finite relation gate cutoff");
}
fn control(rules: &[Rule], q: &Query, access: chr_compiled::Access) -> Vec<Answer> {
    let prepared = chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap();
    let mut engine = prepared
        .start_search(q.clone(), chr_compiled::Policy::Global, access)
        .unwrap();
    let mut answers = Vec::new();
    for _ in 0..100000 {
        match engine.tick() {
            chr_compiled::SearchEvent::Complete(mut branch) => {
                answers.push(branch.engine.observe().unwrap())
            }
            chr_compiled::SearchEvent::Exhausted => return answers,
            _ => (),
        }
    }
    panic!("dedicated source control cutoff");
}

#[test]
fn binary_bag_joins_match_product_oracle_and_source() {
    let base = [
        vec![atom("a"), atom("a")],
        vec![atom("a"), atom("b")],
        vec![atom("b"), atom("a")],
        vec![atom("b"), atom("b")],
    ];
    let mut configurations = 0;
    for left_mask in 0..16 {
        for right_mask in 0..16 {
            for duplicate in [false, true] {
                let mut left = base
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| left_mask & (1 << i) != 0)
                    .map(|(_, r)| r.clone())
                    .collect::<Vec<_>>();
                if duplicate && !left.is_empty() {
                    left.push(left[0].clone());
                }
                let right = base
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| right_mask & (1 << i) != 0)
                    .map(|(_, r)| r.clone())
                    .collect::<Vec<_>>();
                let rules = [request(), table("ab", &left, 2), table("bc", &right, 2)];
                let variants = [
                    [0, 1, 2],
                    [0, 2, 1],
                    [1, 0, 2],
                    [1, 2, 0],
                    [2, 0, 1],
                    [2, 1, 0],
                ]
                .map(|order| {
                    let reordered = order.map(|i| rules[i].clone()).to_vec();
                    let prepared = Prepared::new(&reordered, ("request", 4)).unwrap();
                    (reordered, prepared)
                });
                for repeated in [false, true] {
                    let q = Query {
                        constraints: vec![
                            c(
                                "request",
                                [v(10), v(11), if repeated { v(10) } else { v(12) }, v(13)],
                            ),
                            c("carry", [v(13), v(13)]),
                        ],
                        outputs: vec![
                            ("x".into(), Var(10)),
                            ("y".into(), Var(11)),
                            ("z".into(), if repeated { Var(10) } else { Var(12) }),
                            ("u".into(), Var(13)),
                        ],
                    };
                    // Explicit row-index product, independent of indexes and candidate binding.
                    let mut expected = Vec::new();
                    for a in &left {
                        for b in &right {
                            if a[1] == b[0] && (!repeated || a[0] == b[1]) {
                                expected.push(Answer {
                                    outputs: vec![
                                        ("x".into(), a[0].clone()),
                                        ("y".into(), a[1].clone()),
                                        ("z".into(), b[1].clone()),
                                        ("u".into(), v(13)),
                                    ],
                                    residual: vec![c("carry", [v(13), v(13)])],
                                });
                            }
                        }
                    }
                    for (source, prepared) in &variants {
                        oracle::same_raw(oracle::run(source, &q, 100000), expected.clone());
                        oracle::same_raw(run(prepared, &q), expected.clone());
                        for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                            oracle::same_raw(control(source, &q, access), expected.clone());
                        }
                    }
                    configurations += 1;
                }
            }
        }
    }
    assert_eq!(configurations, 1024);
}
#[test]
fn constructors_givens_locals_multiple_calls_and_passive_aliases() {
    let rows = vec![
        vec![t("f", [atom("a")]), atom("a")],
        vec![t("f", [atom("b")]), atom("b")],
        vec![t("f", [atom("a")]), atom("a")],
    ];
    let entry = Rule::simplify(
        "request",
        [c("request", [v(0)])],
        and(vec![
            c("tab", [v(0), v(1)]).into(),
            c("tab", [v(2), v(1)]).into(),
        ]),
    );
    let rules = vec![entry, table("tab", &rows, 2)];
    let p = Prepared::new(&rules, ("request", 1)).unwrap();
    for constraints in [
        vec![
            c("request", [t("f", [v(10)])]),
            c("carry", [v(10), v(11), v(11)]),
        ],
        vec![c("request", [t("f", [atom("a")])])],
        vec![c("request", [atom("clash")])],
        vec![c("request", [v(10)]), c("request", [v(11)])],
        vec![c("request", [v(10)]), c("request", [v(10)])],
        vec![c("tab", [t("f", [v(10)]), v(10)])],
        vec![c("tab", [v(10), v(10)])],
        vec![c("request", [v(10)]), c("tab", [atom("clash"), v(11)])],
        vec![c("request", [])],
        vec![],
    ] {
        let q = Query {
            constraints,
            outputs: vec![("x".into(), Var(10)), ("y".into(), Var(11))],
        };
        oracle::same_raw(run(&p, &q), oracle::run(&rules, &q, 100000));
    }
    // Hidden independent zero-arity choices retain their derivation product.
    let rules = vec![
        Rule::simplify(
            "request",
            [c("request", [])],
            and(vec![c("coin", []).into(), c("coin", []).into()]),
        ),
        table("coin", &[vec![], vec![]], 0),
    ];
    let q = Query {
        constraints: vec![c("request", [])],
        outputs: vec![],
    };
    let expected = vec![
        Answer {
            outputs: vec![],
            residual: vec![]
        };
        4
    ];
    oracle::same_raw(
        run(&Prepared::new(&rules, ("request", 0)).unwrap(), &q),
        expected.clone(),
    );
    oracle::same_raw(oracle::run(&rules, &q, 100000), expected);
}
#[test]
fn certificate_rejects_near_misses_in_complete_ruleset() {
    let good = vec![
        request(),
        table("ab", &[vec![atom("a"), atom("b")]], 2),
        table("bc", &[vec![atom("b"), atom("a")]], 2),
    ];
    let mut bad = Vec::new();
    let mut x = good.clone();
    x.push(Rule::propagate(
        "observer",
        [c("ab", [v(0), v(1)])],
        Goal::True,
    ));
    bad.push(x);
    let mut x = good.clone();
    x.push(good[1].clone());
    bad.push(x);
    let mut x = good.clone();
    x[1].guards.push(Guard::Equal(v(0), v(1)));
    bad.push(x);
    let mut x = good.clone();
    x[1].removed[0].args = vec![v(0), v(0)];
    bad.push(x);
    let mut x = good.clone();
    x[1].removed[0].args[0] = atom("a");
    bad.push(x);
    let mut x = good.clone();
    x[1].body = eq(v(0), atom("a"));
    bad.push(x);
    let mut x = good.clone();
    x[1].body = and(vec![eq(v(0), v(2)), eq(v(1), atom("a"))]);
    bad.push(x);
    let mut x = good.clone();
    x[0].body = c("request", [v(0), v(1), v(2), v(3)]).into();
    bad.push(x);
    let mut x = good.clone();
    x[0].body = eq(v(0), atom("a"));
    bad.push(x);
    let mut x = good.clone();
    x[1].body = and(vec![
        eq(v(0), atom("a")),
        eq(v(0), atom("a")),
        eq(v(1), atom("b")),
    ]);
    bad.push(x);
    let mut x = good.clone();
    x[1].body = c("effect", []).into();
    bad.push(x);
    assert_eq!(bad.len(), 11);
    for rules in bad {
        assert!(Prepared::new(&rules, ("request", 4)).is_err());
    }
    assert!(Prepared::new(&good, ("missing", 4)).is_err());
}
#[test]
fn first_answer_is_lazy_over_large_disconnected_product() {
    let rows = (0..20)
        .map(|i| vec![atom(&format!("k{i}"))])
        .collect::<Vec<_>>();
    let calls = (0..10).map(|i| c("tab", [v(i)]).into()).collect::<Vec<_>>();
    let rules = vec![
        Rule::simplify("request", [c("request", [])], and(calls)),
        table("tab", &rows, 1),
    ];
    let p = Prepared::new(&rules, ("request", 0)).unwrap();
    let mut engine = p
        .start(&Query {
            constraints: vec![c("request", [])],
            outputs: vec![],
        })
        .unwrap();
    let mut found = false;
    for _ in 0..25 {
        match engine.advance() {
            Step::Answer(a) => {
                assert!(a.residual.is_empty());
                found = true;
                break;
            }
            Step::Exhausted => panic!("product exhausted"),
            _ => (),
        }
    }
    assert!(found, "first answer must not enumerate 20^10 row products");
    assert!(!matches!(engine.advance(), Step::Exhausted));
}
