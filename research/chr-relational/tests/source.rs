#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_relational::execute::{Prepared, Step};
use chr_syntax::{Answer, Goal, Guard, Query, Rule, Var, and, atom, c, eq, or, t, v};
fn run(p: &std::sync::Arc<Prepared>, q: &Query) -> Vec<Answer> {
    let mut engine = p.start(q);
    let mut answers = Vec::new();
    for _ in 0..10000 {
        match engine.advance() {
            Step::Answer(a) => answers.push(a),
            Step::Exhausted => {
                answers.sort();
                let mut candidate = chr_relational::contextual_execute::Prepared::new(p.rules())
                    .unwrap()
                    .start(q);
                let mut contextual = Vec::new();
                for _ in 0..10000 {
                    match candidate.advance() {
                        chr_relational::contextual_execute::Step::Answer(a) => contextual.push(a),
                        chr_relational::contextual_execute::Step::Exhausted => {
                            oracle::same_raw(contextual, answers.clone());
                            return answers;
                        }
                        chr_relational::contextual_execute::Step::Progress => (),
                    }
                }
                panic!("contextual finite source bound");
            }
            Step::Progress => (),
        }
    }
    panic!("finite source bound");
}
#[test]
fn complete_sources_match_independent_owned_semantics() {
    let mut cases = Vec::new();
    for reverse in [false, true] {
        for choice in [false, true] {
            let bind = if choice {
                or(eq(v(0), t("f", [atom("a")])), eq(v(0), t("f", [atom("b")])))
            } else {
                eq(v(0), t("f", [atom("a")]))
            };
            let rules = vec![
                Rule::simplify("bind", [c("bind", [v(0)])], bind),
                Rule::simplify(
                    "join",
                    [c("open", [t("f", [v(0)])]), c("ticket", [v(0)])],
                    c("done", [v(0)]).into(),
                ),
            ];
            let mut constraints = vec![
                c("open", [v(10)]),
                c("ticket", [atom("a")]),
                c("bind", [v(10)]),
            ];
            if reverse {
                constraints.reverse();
            }
            cases.push((
                rules,
                Query {
                    constraints,
                    outputs: vec![("x".into(), Var(10))],
                },
            ));
        }
    }
    let mut guarded = Rule::propagate(
        "guard",
        [c("watch", [v(0), v(1)])],
        c("seen", [v(0)]).into(),
    );
    guarded.guards = vec![Guard::Equal(v(0), v(1))];
    cases.push((
        vec![
            guarded,
            Rule::simplify("alias", [c("alias", [v(0), v(1)])], eq(v(0), v(1))),
        ],
        Query {
            constraints: vec![c("watch", [v(10), v(11)]), c("alias", [v(10), v(11)])],
            outputs: vec![("x".into(), Var(10)), ("y".into(), Var(11))],
        },
    ));
    cases.push((
        vec![Rule {
            name: "keep".into(),
            kept: vec![c("p", [v(0)])],
            removed: vec![c("q", [v(0)])],
            guards: vec![],
            body: c("out", [v(0), v(1), v(1)]).into(),
        }],
        Query {
            constraints: vec![c("p", [atom("a")]), c("q", [atom("a")])],
            outputs: vec![],
        },
    ));
    for body in [
        or(Goal::True, Goal::True),
        or(Goal::Fail, eq(v(0), atom("a"))),
        or(eq(v(0), t("f", [v(0)])), eq(v(0), atom("b"))),
        and(vec![eq(v(0), atom("a")), eq(v(0), atom("b"))]),
    ] {
        cases.push((
            vec![Rule::simplify("branch", [c("start", [v(0)])], body)],
            Query {
                constraints: vec![c("start", [v(10)])],
                outputs: vec![("x".into(), Var(10))],
            },
        ));
    }
    // Read-only matching and guards must not bind unknowns to constructors.
    let mut no_bind = Rule::simplify("guard", [c("p", [v(0)])], Goal::Fail);
    no_bind.guards = vec![Guard::Equal(v(0), atom("a"))];
    cases.push((
        vec![
            no_bind,
            Rule::simplify("pattern", [c("p", [atom("a")])], Goal::Fail),
        ],
        Query {
            constraints: vec![c("p", [v(10)])],
            outputs: vec![("x".into(), Var(10))],
        },
    ));
    // Favorable constructor selectivity and adverse broad/low-yield updates.
    for n in [1, 4, 16] {
        for delayed in [false, true] {
            for dense in [false, true] {
                let rules = vec![
                    Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
                    Rule::simplify(
                        "join",
                        [c("open", [t("f", [atom("a"), v(0)])]), c("ticket", [v(0)])],
                        c("done", [v(0)]).into(),
                    ),
                ];
                let mut constraints = Vec::new();
                for i in 0..n {
                    let key = atom(&format!("key{i}"));
                    let tag = atom(if dense || i == 0 { "a" } else { "b" });
                    let value = t("f", [tag, key.clone()]);
                    if delayed {
                        constraints.push(c("open", [v(100 + i)]));
                        constraints.push(c("bind", [v(100 + i), value]));
                    } else {
                        constraints.push(c("open", [value]));
                    }
                    constraints.push(c("ticket", [key]));
                }
                cases.push((
                    rules,
                    Query {
                        constraints,
                        outputs: vec![],
                    },
                ));
            }
        }
    }
    assert_eq!(cases.len(), 23);
    let mut integrated_checks = 0;
    for (rules, q) in cases {
        let p = Prepared::new(&rules).unwrap();
        // Reused prepared rules, including a changed query with an extra residual.
        for extra in [false, true] {
            let mut q = q.clone();
            if extra {
                q.constraints.push(c("untouched", [atom("z")]));
            }
            let expected = oracle::run(&rules, &q, 10000);
            oracle::same_raw(run(&p, &q), expected.clone());
            if let Ok(control) = chr_integrated::PreparedRuleset::new(&rules) {
                let mut engine = control.start(&q);
                let endpoint = engine.run(10000);
                assert_ne!(
                    endpoint,
                    chr_integrated::Step::Progress,
                    "integrated control cutoff"
                );
                oracle::same_raw(engine.answer().into_iter().collect(), expected);
                integrated_checks += 1;
            }
        }
    }
    assert_eq!(integrated_checks, 36);
}

#[test]
fn finite_sibling_publishes_beside_ongoing_rule_work() {
    let p = Prepared::new(&[
        Rule::simplify(
            "fork",
            [c("start", [v(0)])],
            or(c("loop", []).into(), eq(v(0), atom("answer"))),
        ),
        Rule::simplify("again", [c("loop", [])], c("loop", []).into()),
    ])
    .unwrap();
    let mut e = p.start(&Query {
        constraints: vec![c("start", [v(10)])],
        outputs: vec![("x".into(), Var(10))],
    });
    let mut answers = Vec::new();
    for _ in 0..200 {
        match e.advance() {
            Step::Answer(a) => answers.push(a),
            Step::Progress => (),
            Step::Exhausted => panic!("ongoing branch exhausted"),
        }
    }
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0].outputs, vec![("x".into(), atom("answer"))]);
}

#[test]
fn candidate_reuse_preserves_priority_arrivals_guards_and_consumption() {
    let mut guarded = Rule::simplify("guard-first", [c("p", [v(0)])], c("guarded", [v(0)]).into());
    guarded.guards = vec![Guard::Equal(v(0), atom("a"))];
    let cases = vec![
        // A new higher-priority head must take the shared token before low.
        (
            vec![
                Rule::simplify(
                    "high",
                    [c("p", [v(0)]), c("token", [])],
                    c("high", [v(0)]).into(),
                ),
                Rule::simplify("make", [c("trigger", [])], c("p", [atom("a")]).into()),
                Rule::simplify("low", [c("seed", []), c("token", [])], c("low", []).into()),
            ],
            vec![c("trigger", []), c("seed", []), c("token", [])],
        ),
        // A failed guard must be reconsidered after a lower rule binds its value.
        (
            vec![
                guarded,
                Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
            ],
            vec![c("p", [v(20)]), c("bind", [v(20)])],
        ),
        // A new high-priority partner must win before the next low-priority tuple.
        (
            vec![
                Rule::simplify(
                    "high",
                    [c("p", [v(0)]), c("ticket", [v(0)])],
                    c("high", [v(0)]).into(),
                ),
                Rule::simplify(
                    "low",
                    [c("seed", [v(0)])],
                    and(vec![c("p", [v(0)]).into(), c("ticket", [v(0)]).into()]),
                ),
            ],
            vec![c("seed", [atom("a")]), c("seed", [atom("b")])],
        ),
        // Cached tuples share q; only one can consume that occurrence.
        (
            vec![Rule::simplify(
                "pair",
                [c("p", [v(0)]), c("q", [v(0)])],
                c("done", [v(0)]).into(),
            )],
            vec![
                c("p", [atom("a")]),
                c("p", [atom("a")]),
                c("q", [atom("a")]),
            ],
        ),
        // Both forks inherit a previously ineligible p; equality differs locally.
        (
            vec![
                Rule::simplify("known", [c("p", [atom("a")])], c("yes", []).into()),
                Rule::simplify(
                    "choose",
                    [c("choose", [v(0)])],
                    or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
                ),
            ],
            vec![c("p", [v(20)]), c("choose", [v(20)])],
        ),
        // Previously distinct constructor identities become repeated-variable partners.
        (
            vec![
                Rule::simplify(
                    "same",
                    [c("p", [v(0)]), c("q", [v(0)])],
                    c("same", []).into(),
                ),
                Rule::simplify("alias", [c("alias", [v(0), v(1)])], eq(v(0), v(1))),
            ],
            vec![
                c("p", [t("f", [v(20)])]),
                c("q", [t("f", [v(21)])]),
                c("alias", [v(20), v(21)]),
            ],
        ),
    ];
    for (rules, constraints) in cases {
        let q = Query {
            constraints,
            outputs: vec![("x".into(), Var(20)), ("y".into(), Var(21))],
        };
        oracle::same_raw(
            run(&Prepared::new(&rules).unwrap(), &q),
            oracle::run(&rules, &q, 10000),
        );
    }
}

#[test]
fn equality_cache_adversaries_preserve_raw_sources_under_all_schedules() {
    let alias = Rule::simplify("alias", [c("alias", [v(0), v(1)])], eq(v(0), v(1)));
    let mut guarded = Rule::simplify("guarded", [c("p", [v(0), v(1)])], c("guarded", []).into());
    guarded.guards.push(Guard::Equal(v(0), v(1)));
    let cases = vec![
        (
            vec![guarded, alias.clone()],
            vec![c("p", [v(10), v(11)]), c("alias", [v(10), v(11)])],
        ),
        (
            vec![
                Rule::simplify("repeated", [c("p", [v(0), v(0)])], c("matched", []).into()),
                alias.clone(),
            ],
            vec![c("p", [v(10), v(11)]), c("alias", [v(10), v(11)])],
        ),
        (
            vec![
                Rule::simplify(
                    "constructor",
                    [c("p", [t("f", [v(0)])])],
                    c("matched", [v(0)]).into(),
                ),
                alias.clone(),
            ],
            vec![c("p", [v(10)]), c("alias", [v(10), t("f", [atom("a")])])],
        ),
        // A cached second tuple refers to the representative retired by the first.
        (
            vec![Rule::simplify(
                "merge",
                [c("p", [v(0), v(1)])],
                and([eq(v(0), v(1)), c("seen", [v(0), v(1)]).into()]),
            )],
            vec![c("p", [v(10), v(11)]), c("p", [v(11), atom("a")])],
        ),
        // Shared values become known while later occurrence candidates remain queued.
        (
            vec![Rule::simplify(
                "use",
                [c("p", [v(0)])],
                and([eq(v(0), atom("a")), c("seen", [v(0)]).into()]),
            )],
            vec![
                c("p", [v(10)]),
                c("p", [v(10)]),
                c("p", [v(11)]),
                c("alias", [v(11), v(10)]),
            ],
        ),
        (
            vec![
                Rule::propagate("watch", [c("p", [v(0)])], c("seen", [v(0)]).into()),
                alias.clone(),
            ],
            vec![c("p", [v(10)]), c("alias", [v(10), atom("a")])],
        ),
        (
            vec![
                Rule::simplify(
                    "pair",
                    [c("p", [v(0)]), c("p", [v(1)])],
                    and([eq(v(0), v(1)), c("pair", [v(0), v(1)]).into()]),
                ),
                Rule::simplify("insert", [c("seed", [v(0)])], c("p", [v(0)]).into()),
            ],
            vec![
                c("p", [atom("a")]),
                c("p", [v(10)]),
                c("p", [atom("b")]),
                c("seed", [v(11)]),
            ],
        ),
        (
            vec![Rule::simplify(
                "branch",
                [c("p", [v(0)])],
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
            )],
            vec![c("p", [v(10)]), c("p", [v(10)])],
        ),
    ];
    for (rules, facts) in cases {
        for reverse in [false, true] {
            let mut facts = facts.clone();
            if reverse {
                facts.reverse();
            }
            let query = Query {
                constraints: facts,
                outputs: vec![],
            };
            let expected = oracle::run(&rules, &query, 100_000);
            for mode in 0..3 {
                let prepared = Prepared::new_ready(&rules).unwrap();
                let mut engine = prepared.start(&query);
                let mut actual = vec![];
                let mut exhausted = false;
                for _ in 0..100_000 {
                    let step = match mode {
                        0 => engine.advance(),
                        1 => engine.advance_settled(),
                        _ => engine.advance_ready(8),
                    };
                    match step {
                        Step::Answer(a) => actual.push(a),
                        Step::Exhausted => {
                            exhausted = true;
                            break;
                        }
                        Step::Progress => (),
                    }
                }
                assert!(exhausted);
                oracle::same_raw(actual, expected.clone());
            }
        }
    }
}
