#[allow(dead_code)]
mod runtime_support;
use chr_direct_choice::demand::{Event, Prepared};
use chr_syntax::{Answer, Goal, Query, Rule, Var, and, atom, c, eq, or, t, v};
fn collect(rules: Vec<Rule>, query: Query, expected: Vec<Answer>) {
    runtime_support::same_raw(
        runtime_support::run(&rules, &query, 200_000),
        expected.clone(),
    );
    let mut run = Prepared::new(rules).unwrap().start(query).unwrap();
    let mut answers = vec![];
    for _ in 0..100_000 {
        match run.tick() {
            Event::Progress => {}
            Event::Answer(a) => answers.push(a),
            Event::Exhausted => {
                runtime_support::same_raw(answers, expected);
                return;
            }
            Event::Stuck => panic!("finite source unexpectedly stuck"),
        }
    }
    panic!("suspended source gate cutoff");
}
#[test]
fn opaque_recursive_calls_preserve_shared_and_independent_births() {
    let rules = vec![
        Rule::simplify(
            "make",
            [c("make", [atom("k"), v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify("base", [c("work", [atom("z"), v(0), v(1)])], eq(v(1), v(0))),
        Rule::simplify(
            "step",
            [c("work", [t("s", [v(0)]), v(1), v(2)])],
            c("work", [v(0), v(1), v(2)]).into(),
        ),
    ];
    for n in [0, 1, 8] {
        for independent in [false, true] {
            let depth = (0..n).fold(atom("z"), |x, _| t("s", [x]));
            let mut constraints = vec![
                c("make", [atom("k"), v(100)]),
                c("work", [depth, v(100), v(101)]),
            ];
            if independent {
                constraints.push(c("make", [atom("k"), v(102)]));
            }
            let expected = ["a", "b"]
                .into_iter()
                .flat_map(|a| {
                    ["a", "b"]
                        .into_iter()
                        .filter(move |b| independent || a == *b)
                        .map(move |b| Answer {
                            outputs: vec![("x".into(), atom(a)), ("y".into(), atom(b))],
                            residual: vec![],
                        })
                })
                .collect();
            collect(
                rules.clone(),
                Query {
                    constraints,
                    outputs: vec![
                        ("x".into(), Var(101)),
                        ("y".into(), Var(if independent { 102 } else { 101 })),
                    ],
                },
                expected,
            );
        }
    }
}
#[test]
fn fresh_locals_and_unused_failure_are_source_obligations() {
    let make = Rule::simplify(
        "make",
        [c("make", [atom("k"), v(0)])],
        eq(v(0), t("box", [v(1)])),
    );
    collect(
        vec![make],
        Query {
            constraints: vec![
                c("make", [atom("k"), v(100)]),
                c("make", [atom("k"), v(101)]),
            ],
            outputs: vec![("x".into(), Var(100)), ("y".into(), Var(101))],
        },
        vec![Answer {
            outputs: vec![
                ("x".into(), t("box", [v(900)])),
                ("y".into(), t("box", [v(901)])),
            ],
            residual: vec![],
        }],
    );
    let rules = vec![
        Rule::simplify("bad", [c("bad", [v(0)])], Goal::Fail),
        Rule::simplify(
            "outer",
            [c("outer", [v(0)])],
            and([c("bad", [v(1)]).into(), eq(v(0), atom("ok"))]),
        ),
    ];
    collect(
        rules,
        Query {
            constraints: vec![c("outer", [v(100)])],
            outputs: vec![("x".into(), Var(100))],
        },
        vec![],
    );
}
#[test]
fn finite_sibling_is_serviced_and_effectful_sources_are_rejected() {
    let rules = vec![
        Rule::simplify("loop", [c("loop", [v(0)])], c("loop", [v(0)]).into()),
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(c("loop", [v(0)]).into(), eq(v(0), atom("ok"))),
        ),
    ];
    let mut run = Prepared::new(rules)
        .unwrap()
        .start(Query {
            constraints: vec![c("start", [v(100)])],
            outputs: vec![("x".into(), Var(100))],
        })
        .unwrap();
    let mut answers = 0;
    for _ in 0..128 {
        match run.tick() {
            Event::Progress => {}
            Event::Answer(a) => {
                assert_eq!(a.outputs, vec![("x".into(), atom("ok"))]);
                answers += 1
            }
            _ => panic!("continuing source cannot finish"),
        }
    }
    assert_eq!(answers, 1);
    assert!(
        Prepared::new(vec![Rule::propagate(
            "effect",
            [c("p", [v(0)])],
            eq(v(0), atom("a"))
        )])
        .is_err()
    );
}

#[test]
fn branch_local_producers_do_not_overwrite_sibling_results() {
    let rules = vec![
        Rule::simplify("a", [c("a", [v(0)])], eq(v(0), atom("a"))),
        Rule::simplify("b", [c("b", [v(0)])], eq(v(0), atom("b"))),
        Rule::simplify(
            "outer",
            [c("outer", [v(0)])],
            or(
                and([c("a", [v(1)]).into(), eq(v(0), v(1))]),
                and([c("b", [v(1)]).into(), eq(v(0), v(1))]),
            ),
        ),
    ];
    collect(
        rules,
        Query {
            constraints: vec![c("outer", [v(100)])],
            outputs: vec![("x".into(), Var(100))],
        },
        ["a", "b"]
            .into_iter()
            .map(|a| Answer {
                outputs: vec![("x".into(), atom(a))],
                residual: vec![],
            })
            .collect(),
    );
}
#[test]
fn demand_must_not_enable_a_rule_ahead_of_an_already_enabled_competitor() {
    let rules = vec![
        Rule::simplify(
            "specific",
            [c("read", [atom("a"), v(0)])],
            eq(v(0), atom("specific")),
        ),
        Rule::simplify(
            "fallback",
            [c("read", [v(0), v(1)])],
            eq(v(1), atom("fallback")),
        ),
        Rule::simplify("make", [c("make", [v(0)])], eq(v(0), atom("a"))),
    ];
    let query = Query {
        constraints: vec![c("read", [v(100), v(101)]), c("make", [v(100)])],
        outputs: vec![("x".into(), Var(101))],
    };
    runtime_support::same_raw(
        runtime_support::run(&rules, &query, 200_000),
        vec![Answer {
            outputs: vec![("x".into(), atom("fallback"))],
            residual: vec![],
        }],
    );
    assert!(
        Prepared::new(rules).is_err(),
        "this compiler has no proof allowing demand to change rule competition"
    );
}
#[test]
fn nested_duplicate_choices_and_unresolved_demands_remain_honest() {
    let rules = vec![Rule::simplify(
        "make",
        [c("make", [v(0)])],
        or(
            eq(v(0), atom("a")),
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
    )];
    collect(
        rules,
        Query {
            constraints: vec![c("make", [v(100)])],
            outputs: vec![],
        },
        vec![
            Answer {
                outputs: vec![],
                residual: vec![]
            };
            3
        ],
    );
    let prepared = Prepared::new(vec![Rule::simplify(
        "only",
        [c("only", [atom("a"), v(0)])],
        eq(v(0), atom("ok")),
    )])
    .unwrap();
    let mut run = prepared
        .start(Query {
            constraints: vec![c("only", [v(100), v(101)])],
            outputs: vec![("x".into(), Var(101))],
        })
        .unwrap();
    assert!(matches!(run.tick(), Event::Stuck));
    assert!(matches!(run.tick(), Event::Stuck));
    assert!(
        prepared
            .start(Query {
                constraints: vec![
                    c("only", [atom("a"), v(100)]),
                    c("only", [atom("a"), v(100)])
                ],
                outputs: vec![]
            })
            .is_err()
    );
}

#[test]
fn independent_failure_is_serviced_beside_an_ongoing_output() {
    let rules = vec![
        Rule::simplify("loop", [c("loop", [v(0)])], c("loop", [v(0)]).into()),
        Rule::simplify("bad", [c("bad", [v(0)])], Goal::Fail),
    ];
    let mut run = Prepared::new(rules)
        .unwrap()
        .start(Query {
            constraints: vec![c("loop", [v(100)]), c("bad", [v(101)])],
            outputs: vec![("x".into(), Var(100))],
        })
        .unwrap();
    for _ in 0..128 {
        match run.tick() {
            Event::Exhausted => return,
            Event::Progress => {}
            _ => panic!("failing query cannot publish"),
        }
    }
    panic!("independent source failure was starved");
}

#[test]
fn cyclic_producer_graph_is_rejected_and_nested_failure_is_serviced() {
    let id = Rule::simplify("id", [c("id", [v(0), v(1)])], eq(v(1), v(0)));
    let cyclic = Rule::simplify(
        "outer",
        [c("outer", [v(0)])],
        and([
            c("id", [v(2), v(1)]).into(),
            c("id", [v(1), v(2)]).into(),
            eq(v(0), atom("ok")),
        ]),
    );
    assert!(Prepared::new(vec![id, cyclic]).is_err());
    let rules = vec![
        Rule::simplify("loop", [c("loop", [v(0)])], c("loop", [v(0)]).into()),
        Rule::simplify("bad", [c("bad", [v(0)])], Goal::Fail),
        Rule::simplify(
            "outer",
            [c("outer", [v(0)])],
            and([
                c("loop", [v(1)]).into(),
                c("bad", [v(2)]).into(),
                eq(v(0), atom("ok")),
            ]),
        ),
    ];
    let mut run = Prepared::new(rules)
        .unwrap()
        .start(Query {
            constraints: vec![c("outer", [v(100)])],
            outputs: vec![("x".into(), Var(100))],
        })
        .unwrap();
    for _ in 0..128 {
        match run.tick() {
            Event::Exhausted => return,
            Event::Progress => {}
            _ => panic!("nested failing producer cannot publish"),
        }
    }
    panic!("nested source failure was starved");
}

#[test]
fn constructor_demand_can_refute_before_unrelated_producer_finishes() {
    let rules = vec![
        Rule::simplify(
            "reject",
            [c("reject", [t("box", [v(0)]), v(1)])],
            Goal::Fail,
        ),
        Rule::simplify(
            "build",
            [c("build", [v(0)])],
            and([c("loop", [v(1)]).into(), eq(v(0), t("box", [v(1)]))]),
        ),
        Rule::simplify("loop", [c("loop", [v(0)])], c("loop", [v(0)]).into()),
    ];
    let query = Query {
        constraints: vec![c("build", [v(100)]), c("reject", [v(100), v(101)])],
        outputs: vec![("x".into(), Var(101))],
    };
    runtime_support::same_raw(runtime_support::run(&rules, &query, 200_000), vec![]);
    let mut run = Prepared::new(rules).unwrap().start(query).unwrap();
    for _ in 0..128 {
        match run.tick() {
            Event::Exhausted => return,
            Event::Progress => {}
            _ => panic!("refuted computation cannot publish"),
        }
    }
    panic!("constructor demand waited for an unrelated producer");
}
