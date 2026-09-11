#[allow(dead_code)]
mod runtime_support;
use chr_direct_choice::demand::{Event, Prepared};
use chr_syntax::{and, atom, c, eq, or, t, v, Answer, Goal, Query, Rule, Var};
fn collect(rules: Vec<Rule>, query: Query, expected: Vec<Answer>) {
    runtime_support::same_raw(
        runtime_support::run(&rules, &query, 200_000),
        expected.clone(),
    );
    let prepared = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
        let mut search = prepared
            .start_search(query.clone(), chr_compiled::Policy::Global, access)
            .unwrap();
        let mut actual = vec![];
        let mut exhausted = false;
        for _ in 0..100_000 {
            match search.tick() {
                chr_compiled::SearchEvent::Complete(mut b) => {
                    actual.push(b.engine.observe().unwrap())
                }
                chr_compiled::SearchEvent::Exhausted => {
                    exhausted = true;
                    break;
                }
                _ => {}
            }
        }
        assert!(exhausted, "compiled control cutoff");
        runtime_support::same_raw(actual, expected.clone());
    }
    let mut graph = chr_direct_choice::engine::PreparedRuleset::new(rules.clone())
        .unwrap()
        .start(query.clone())
        .unwrap();
    let mut actual = vec![];
    let mut exhausted = false;
    for _ in 0..100_000 {
        match graph.tick() {
            chr_direct_choice::engine::Event::Answer(a) => actual.push(a),
            chr_direct_choice::engine::Event::Exhausted => {
                exhausted = true;
                break;
            }
            chr_direct_choice::engine::Event::Progress => {}
        }
    }
    assert!(exhausted, "direct graph control cutoff");
    runtime_support::same_raw(actual, expected.clone());
    runtime_support::same_raw(
        demand_with_policy(
            rules.clone(),
            query.clone(),
            chr_direct_choice::demand::Reuse::CurrentContext,
        ),
        expected.clone(),
    );
    for policy in [
        chr_direct_choice::demand::Reuse::CurrentContext,
        chr_direct_choice::demand::Reuse::StaticBirth,
    ] {
        runtime_support::same_raw(
            demand_strategy(rules.clone(), query.clone(), policy, true),
            expected.clone(),
        );
    }
    for pull in [false, true] {
        runtime_support::same_raw(
            demand_strategy(
                rules.clone(),
                query.clone(),
                chr_direct_choice::demand::Reuse::MatchDependencies,
                pull,
            ),
            expected.clone(),
        );
    }
    for policy in [
        chr_direct_choice::demand::Reuse::CurrentContext,
        chr_direct_choice::demand::Reuse::StaticBirth,
        chr_direct_choice::demand::Reuse::MatchDependencies,
    ] {
        runtime_support::same_raw(
            demand_prepared(
                Prepared::with_reuse(rules.clone(), policy)
                    .unwrap()
                    .with_derivation_templates(),
                query.clone(),
            ),
            expected.clone(),
        );
    }
    runtime_support::same_raw(
        demand_prepared(
            Prepared::new(rules.clone()).unwrap().with_template_follow_limit(0),
            query.clone(),
        ),
        expected.clone(),
    );
    runtime_support::same_raw(demand_answers(rules, query), expected);
}
fn demand_answers(rules: Vec<Rule>, query: Query) -> Vec<Answer> {
    demand_with_policy(rules, query, chr_direct_choice::demand::Reuse::StaticBirth)
}
fn demand_with_policy(
    rules: Vec<Rule>,
    query: Query,
    policy: chr_direct_choice::demand::Reuse,
) -> Vec<Answer> {
    demand_strategy(rules, query, policy, false)
}
fn demand_strategy(
    rules: Vec<Rule>,
    query: Query,
    policy: chr_direct_choice::demand::Reuse,
    pull_tabs: bool,
) -> Vec<Answer> {
    let prepared = Prepared::with_reuse(rules, policy).unwrap();
    let prepared = if pull_tabs {
        prepared.with_pull_tabs()
    } else {
        prepared
    };
    demand_prepared(prepared, query)
}
fn demand_prepared(prepared: Prepared, query: Query) -> Vec<Answer> {
    let mut run = prepared.start(query).unwrap();
    let mut answers = vec![];
    for _ in 0..100_000 {
        match run.tick() {
            Event::Progress => {}
            Event::Answer(a) => answers.push(a),
            Event::Exhausted => return answers,
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
fn finite_sibling_is_serviced_and_pure_propagation_is_rejected() {
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
    assert!(Prepared::new(vec![Rule::propagate(
        "effect",
        [c("p", [v(0)])],
        eq(v(0), atom("a"))
    )])
    .is_err());
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
    let Event::Answer(answer) = run.tick() else {
        panic!("quiescent nonmatch must publish its residual")
    };
    runtime_support::same_raw(
        vec![answer],
        vec![Answer {
            outputs: vec![("x".into(), v(901))],
            residual: vec![c("only", [v(900), v(901)])],
        }],
    );
    assert!(matches!(run.tick(), Event::Exhausted));
    assert!(prepared
        .start(Query {
            constraints: vec![
                c("only", [atom("a"), v(100)]),
                c("only", [atom("a"), v(100)])
            ],
            outputs: vec![]
        })
        .is_err());
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

#[test]
fn residual_calls_keep_joint_aliases_and_occurrence_multiplicity() {
    let rules = vec![Rule::simplify(
        "only",
        [c("only", [atom("a"), v(0)])],
        eq(v(0), atom("ok")),
    )];
    for input in [v(200), atom("b")] {
        let residual_input = if input == v(200) {
            v(900)
        } else {
            input.clone()
        };
        collect(
            rules.clone(),
            Query {
                constraints: vec![
                    c("only", [input.clone(), v(100)]),
                    c("only", [input, v(101)]),
                ],
                outputs: vec![("x".into(), Var(100)), ("y".into(), Var(101))],
            },
            vec![Answer {
                outputs: vec![("x".into(), v(901)), ("y".into(), v(902))],
                residual: vec![
                    c("only", [residual_input.clone(), v(901)]),
                    c("only", [residual_input, v(902)]),
                ],
            }],
        );
    }
}
#[test]
fn tail_replacement_and_branch_local_residuals_preserve_outputs() {
    let rules = vec![
        Rule::simplify("only", [c("only", [atom("a"), v(0)])], eq(v(0), atom("ok"))),
        Rule::simplify(
            "forward",
            [c("forward", [v(0), v(1)])],
            c("only", [v(0), v(1)]).into(),
        ),
        Rule::simplify(
            "choose",
            [c("choose", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
    ];
    collect(
        rules.clone(),
        Query {
            constraints: vec![c("forward", [v(100), v(101)])],
            outputs: vec![("x".into(), Var(100)), ("y".into(), Var(101))],
        },
        vec![Answer {
            outputs: vec![("x".into(), v(900)), ("y".into(), v(901))],
            residual: vec![c("only", [v(900), v(901)])],
        }],
    );
    collect(
        rules,
        Query {
            constraints: vec![c("forward", [v(100), v(101)]), c("choose", [v(100)])],
            outputs: vec![("x".into(), Var(100)), ("y".into(), Var(101))],
        },
        vec![
            Answer {
                outputs: vec![("x".into(), atom("a")), ("y".into(), atom("ok"))],
                residual: vec![],
            },
            Answer {
                outputs: vec![("x".into(), atom("b")), ("y".into(), v(900))],
                residual: vec![c("only", [atom("b"), v(900)])],
            },
        ],
    );
}
#[test]
fn residual_does_not_hide_later_producer_or_independent_failure() {
    let only = Rule::simplify("only", [c("only", [atom("a"), v(0)])], eq(v(0), atom("ok")));
    collect(
        vec![
            only.clone(),
            Rule::simplify("make", [c("make", [v(0)])], eq(v(0), atom("a"))),
        ],
        Query {
            constraints: vec![c("only", [v(100), v(101)]), c("make", [v(100)])],
            outputs: vec![("x".into(), Var(101))],
        },
        vec![Answer {
            outputs: vec![("x".into(), atom("ok"))],
            residual: vec![],
        }],
    );
    collect(
        vec![only, Rule::simplify("bad", [c("bad", [v(0)])], Goal::Fail)],
        Query {
            constraints: vec![c("only", [atom("b"), v(100)]), c("bad", [v(101)])],
            outputs: vec![("x".into(), Var(100))],
        },
        vec![],
    );
}

#[test]
fn producer_residual_has_its_own_output_and_one_occurrence() {
    let rules = vec![
        Rule::simplify("only", [c("only", [atom("a"), v(0)])], eq(v(0), atom("ok"))),
        Rule::simplify(
            "outer",
            [c("outer", [v(0)])],
            and([
                c("only", [atom("b"), v(1)]).into(),
                eq(v(0), t("box", [v(1)])),
            ]),
        ),
    ];
    collect(
        rules,
        Query {
            constraints: vec![c("outer", [v(100)])],
            outputs: vec![("x".into(), Var(100))],
        },
        vec![Answer {
            outputs: vec![("x".into(), t("box", [v(900)]))],
            residual: vec![c("only", [atom("b"), v(900)])],
        }],
    );
}

#[test]
fn inactive_choice_arm_calls_do_not_leak_into_residuals() {
    let rules = vec![
        Rule::simplify("only", [c("only", [atom("a"), v(0)])], eq(v(0), atom("ok"))),
        Rule::simplify(
            "outer",
            [c("outer", [v(0)])],
            or(
                c("only", [atom("a"), v(0)]).into(),
                c("only", [atom("b"), v(0)]).into(),
            ),
        ),
    ];
    collect(
        rules,
        Query {
            constraints: vec![c("outer", [v(100)])],
            outputs: vec![("x".into(), Var(100))],
        },
        vec![
            Answer {
                outputs: vec![("x".into(), atom("ok"))],
                residual: vec![],
            },
            Answer {
                outputs: vec![("x".into(), v(900))],
                residual: vec![c("only", [atom("b"), v(900)])],
            },
        ],
    );
}

#[test]
fn shared_resource_claims_preserve_distinct_requests_and_tokens() {
    let rules = vec![Rule::simplify(
        "take",
        [c("take", [v(0), v(1)]), c("token", [v(0)])],
        eq(v(1), atom("ok")),
    )];
    for tokens in [1, 2] {
        let mut constraints = vec![
            c("take", [atom("k"), v(100)]),
            c("take", [atom("k"), v(101)]),
        ];
        constraints.extend(vec![c("token", [atom("k")]); tokens]);
        let expected = Answer {
            outputs: vec![
                ("x".into(), atom("ok")),
                ("again".into(), atom("ok")),
                ("y".into(), if tokens == 2 { atom("ok") } else { v(900) }),
            ],
            residual: if tokens == 2 {
                vec![]
            } else {
                vec![c("take", [atom("k"), v(900)])]
            },
        };
        collect(
            rules.clone(),
            Query {
                constraints,
                outputs: vec![
                    ("x".into(), Var(100)),
                    ("again".into(), Var(100)),
                    ("y".into(), Var(101)),
                ],
            },
            vec![expected],
        );
    }
}
#[test]
fn resource_tuple_is_atomic_and_kept_heads_survive() {
    let rules = vec![Rule {
        name: "take".into(),
        kept: vec![c("permit", [v(0)])],
        removed: vec![
            c("take", [v(0), v(1)]),
            c("token", [v(0)]),
            c("token", [v(0)]),
        ],
        guards: vec![],
        body: eq(v(1), atom("ok")),
    }];
    for tokens in [1, 2] {
        let mut constraints = vec![c("take", [atom("k"), v(100)]), c("permit", [atom("k")])];
        constraints.extend(vec![c("token", [atom("k")]); tokens]);
        let mut residual = vec![c("permit", [atom("k")])];
        if tokens == 1 {
            residual.extend([c("take", [atom("k"), v(900)]), c("token", [atom("k")])]);
        }
        collect(
            rules.clone(),
            Query {
                constraints,
                outputs: vec![("x".into(), Var(100))],
            },
            vec![Answer {
                outputs: vec![("x".into(), if tokens == 2 { atom("ok") } else { v(900) })],
                residual,
            }],
        );
    }
}

#[test]
fn conditional_claims_and_late_posts_follow_source_choices() {
    let take = Rule::simplify(
        "take",
        [c("take", [v(0), v(1)]), c("token", [v(0)])],
        eq(v(1), atom("ok")),
    );
    let supply = Rule::simplify(
        "supply",
        [c("supply", [v(0)])],
        or(
            and([c("token", [atom("a")]).into(), eq(v(0), atom("a"))]),
            eq(v(0), atom("b")),
        ),
    );
    collect(
        vec![take, supply],
        Query {
            constraints: vec![c("take", [atom("a"), v(100)]), c("supply", [v(101)])],
            outputs: vec![("x".into(), Var(100)), ("arm".into(), Var(101))],
        },
        vec![
            Answer {
                outputs: vec![("x".into(), atom("ok")), ("arm".into(), atom("a"))],
                residual: vec![],
            },
            Answer {
                outputs: vec![("x".into(), v(900)), ("arm".into(), atom("b"))],
                residual: vec![c("take", [atom("a"), v(900)])],
            },
        ],
    );
    let rules = vec![Rule::simplify(
        "take",
        [c("take", [atom("k"), v(0)]), c("token", [atom("k")])],
        or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
    )];
    collect(
        rules,
        Query {
            constraints: vec![
                c("take", [atom("k"), v(100)]),
                c("take", [atom("k"), v(101)]),
                c("token", [atom("k")]),
            ],
            outputs: vec![("first".into(), Var(100)), ("second".into(), Var(101))],
        },
        ["a", "b"]
            .into_iter()
            .map(|a| Answer {
                outputs: vec![("first".into(), atom(a)), ("second".into(), v(900))],
                residual: vec![c("take", [atom("k"), v(900)])],
            })
            .collect(),
    );
}
#[test]
fn resource_keys_are_nonbinding_and_can_become_available() {
    let take = Rule::simplify(
        "take",
        [c("take", [v(0), v(1)]), c("token", [v(0)])],
        eq(v(1), atom("ok")),
    );
    for bind in [false, true] {
        let rules = vec![
            Rule::simplify("make", [c("make", [v(0)])], eq(v(0), atom("a"))),
            take.clone(),
        ];
        let mut constraints = vec![c("take", [v(100), v(101)]), c("token", [atom("a")])];
        if bind {
            constraints.push(c("make", [v(100)]));
        }
        collect(
            rules,
            Query {
                constraints,
                outputs: vec![("key".into(), Var(100)), ("out".into(), Var(101))],
            },
            vec![if bind {
                Answer {
                    outputs: vec![("key".into(), atom("a")), ("out".into(), atom("ok"))],
                    residual: vec![],
                }
            } else {
                Answer {
                    outputs: vec![("key".into(), v(900)), ("out".into(), v(901))],
                    residual: vec![c("take", [v(900), v(901)]), c("token", [atom("a")])],
                }
            }],
        );
    }
}

#[test]
fn nonconfluent_resource_competition_has_an_explicit_committed_policy() {
    let rules = vec![
        Rule::simplify("a", [c("a", [v(0)]), c("token", [])], eq(v(0), atom("a"))),
        Rule::simplify("b", [c("b", [v(0)]), c("token", [])], eq(v(0), atom("b"))),
    ];
    let query = Query {
        constraints: vec![c("b", [v(101)]), c("a", [v(100)]), c("token", [])],
        outputs: vec![("a".into(), Var(100)), ("b".into(), Var(101))],
    };
    let a_wins = Answer {
        outputs: vec![("a".into(), atom("a")), ("b".into(), v(900))],
        residual: vec![c("b", [v(900)])],
    };
    let b_wins = Answer {
        outputs: vec![("a".into(), v(900)), ("b".into(), atom("b"))],
        residual: vec![c("a", [v(900)])],
    };
    // A commits first under the scalar declaration-priority policy.
    runtime_support::same_raw(
        runtime_support::run(&rules, &query, 200_000),
        vec![a_wins.clone()],
    );
    // B's request is the first serviced demand. Its two heads are available,
    // so this is a valid committed step, not an implicit alternative to A.
    runtime_support::same_raw(
        demand_answers(rules.clone(), query.clone()),
        vec![b_wins.clone()],
    );
    let mut alternate_policy = rules;
    alternate_policy.reverse();
    runtime_support::same_raw(
        runtime_support::run(&alternate_policy, &query, 200_000),
        vec![b_wins.clone()],
    );
    assert!(!chr_observe::equivalent(
        &a_wins,
        &b_wins,
        &mut Default::default()
    ));
    println!(
        "nonconfluent probe: declaration priority gives a; demand service gives b; alternate scalar priority gives b; these endpoints are not interchangeable timing controls"
    );
}
#[test]
fn self_dependent_resource_waits_and_nonground_posts_preserve_bindings() {
    let take = Rule::simplify(
        "take",
        [c("take", [v(0), v(1)]), c("token", [v(0)])],
        eq(v(1), atom("ok")),
    );
    let query = Query {
        constraints: vec![c("take", [atom("a"), v(100)]), c("token", [v(100)])],
        outputs: vec![("out".into(), Var(100))],
    };
    collect(
        vec![take.clone()],
        query.clone(),
        vec![Answer {
            outputs: vec![("out".into(), v(100))],
            residual: query.constraints,
        }],
    );
    let supply = Rule::simplify(
        "supply",
        [c("supply", [v(0), v(1)])],
        and([c("token", [v(0)]).into(), eq(v(1), atom("ok"))]),
    );
    collect(vec![take, supply], Query {
        constraints: vec![c("supply", [atom("a"), v(100)]), c("take", [atom("a"), v(101)])],
        outputs: vec![("supply".into(), Var(100)), ("take".into(), Var(101))],
    }, vec![Answer {
        outputs: vec![("supply".into(), atom("ok")), ("take".into(), atom("ok"))],
        residual: vec![],
    }]);
}

#[test]
fn a_sibling_claim_cannot_consume_the_other_siblings_resource() {
    let rules = vec![
        Rule::simplify(
            "take",
            [c("take", [atom("k"), v(0)]), c("token", [atom("k")])],
            eq(v(0), atom("ok")),
        ),
        Rule::simplify(
            "outer",
            [c("outer", [v(0)])],
            or(c("take", [atom("k"), v(0)]).into(), eq(v(0), atom("pass"))),
        ),
    ];
    collect(
        rules,
        Query {
            constraints: vec![c("outer", [v(100)]), c("token", [atom("k")])],
            outputs: vec![("x".into(), Var(100))],
        },
        vec![
            Answer {
                outputs: vec![("x".into(), atom("ok"))],
                residual: vec![],
            },
            Answer {
                outputs: vec![("x".into(), atom("pass"))],
                residual: vec![c("token", [atom("k")])],
            },
        ],
    );
}
#[test]
fn unknown_resource_handles_preserve_aliases_without_binding() {
    let rules = vec![Rule {
        name: "take".into(),
        kept: vec![c("permit", [v(1)])],
        removed: vec![c("take", [v(0)]), c("token", [v(1)])],
        guards: vec![],
        body: eq(v(0), t("box", [v(1)])),
    }];
    for same in [false, true] {
        let expected = if same {
            Answer {
                outputs: vec![("x".into(), t("box", [v(900)]))],
                residual: vec![c("permit", [v(900)])],
            }
        } else {
            Answer {
                outputs: vec![("x".into(), v(902))],
                residual: vec![
                    c("take", [v(902)]),
                    c("permit", [v(900)]),
                    c("token", [v(901)]),
                ],
            }
        };
        collect(
            rules.clone(),
            Query {
                constraints: vec![
                    c("take", [v(100)]),
                    c("permit", [v(200)]),
                    c("token", [v(if same { 200 } else { 201 })]),
                ],
                outputs: vec![("x".into(), Var(100))],
            },
            vec![expected],
        );
    }
}

#[test]
fn pulled_demand_preserves_finite_service_and_off_output_failure() {
    let rules = vec![
        Rule::simplify(
            "make",
            [c("make", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify(
            "a",
            [c("take", [atom("a"), v(0)])],
            c("loop", [v(0)]).into(),
        ),
        Rule::simplify("b", [c("take", [atom("b"), v(0)])], eq(v(0), atom("ok"))),
        Rule::simplify("loop", [c("loop", [v(0)])], c("loop", [v(0)]).into()),
        Rule::simplify("bad", [c("bad", [v(0)])], Goal::Fail),
    ];
    for templates in [false, true] {
        for policy in [
            chr_direct_choice::demand::Reuse::StaticBirth,
            chr_direct_choice::demand::Reuse::MatchDependencies,
        ] {
            for pull_tabs in [false, true] {
                for failing in [false, true] {
                    let mut constraints = vec![c("make", [v(100)]), c("take", [v(100), v(101)])];
                    if failing {
                        constraints.push(c("bad", [v(102)]));
                    }
                    let prepared = Prepared::with_reuse(rules.clone(), policy).unwrap();
                    let prepared = if pull_tabs {
                        prepared.with_pull_tabs()
                    } else {
                        prepared
                    };
                    let prepared = if templates {
                        prepared.with_derivation_templates()
                    } else {
                        prepared
                    };
                    let mut run = prepared
                        .start(Query {
                            constraints,
                            outputs: vec![("x".into(), Var(101))],
                        })
                        .unwrap();
                    let mut answers = vec![];
                    let mut exhausted = false;
                    for _ in 0..128 {
                        match run.tick() {
                            Event::Progress => {}
                            Event::Answer(a) => answers.push(a),
                            Event::Exhausted => {
                                exhausted = true;
                                break;
                            }
                        }
                    }
                    if failing {
                        assert!(
                            exhausted && answers.is_empty(),
                            "off-output failure must terminate"
                        );
                    } else {
                        assert!(!exhausted, "loop must remain unfinished");
                        runtime_support::same_raw(
                            answers,
                            vec![Answer {
                                outputs: vec![("x".into(), atom("ok"))],
                                residual: vec![],
                            }],
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn pulled_nonmatching_arm_preserves_one_residual_occurrence() {
    collect(
        vec![
            Rule::simplify(
                "make",
                [c("make", [v(0)])],
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
            ),
            Rule::simplify("take", [c("take", [atom("a"), v(0)])], eq(v(0), atom("ok"))),
        ],
        Query {
            constraints: vec![c("make", [v(100)]), c("take", [v(100), v(101)])],
            outputs: vec![("x".into(), Var(101))],
        },
        vec![
            Answer {
                outputs: vec![("x".into(), atom("ok"))],
                residual: vec![],
            },
            Answer {
                outputs: vec![("x".into(), v(900))],
                residual: vec![c("take", [atom("b"), v(900)])],
            },
        ],
    );
}

#[test]
fn pulled_calls_keep_correlation_and_consume_distinct_tokens() {
    let rules = vec![
        Rule::simplify(
            "make",
            [c("make", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify(
            "a",
            [c("take", [atom("a"), v(0)]), c("token", [])],
            eq(v(0), atom("a")),
        ),
        Rule::simplify(
            "b",
            [c("take", [atom("b"), v(0)]), c("token", [])],
            eq(v(0), atom("b")),
        ),
    ];
    collect(
        rules,
        Query {
            constraints: vec![
                c("make", [v(100)]),
                c("take", [v(100), v(101)]),
                c("take", [v(100), v(102)]),
                c("token", []),
                c("token", []),
            ],
            outputs: vec![("x".into(), Var(101)), ("y".into(), Var(102))],
        },
        ["a", "b"]
            .into_iter()
            .map(|value| Answer {
                outputs: vec![("x".into(), atom(value)), ("y".into(), atom(value))],
                residual: vec![],
            })
            .collect(),
    );
}

#[test]
fn pull_tab_resource_competition_matches_demand_control() {
    let rules = vec![
        Rule::simplify(
            "make",
            [c("make", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify(
            "a",
            [c("take", [atom("a"), v(0)]), c("token", [])],
            eq(v(0), atom("a")),
        ),
        Rule::simplify(
            "b",
            [c("take", [atom("b"), v(0)]), c("token", [])],
            eq(v(0), atom("b")),
        ),
    ];
    let query = Query {
        constraints: vec![
            c("make", [v(100)]),
            c("take", [v(100), v(101)]),
            c("take", [v(100), v(102)]),
            c("token", []),
        ],
        outputs: vec![("x".into(), Var(101)), ("y".into(), Var(102))],
    };
    let policy = chr_direct_choice::demand::Reuse::StaticBirth;
    runtime_support::same_raw(
        demand_strategy(rules.clone(), query.clone(), policy, true),
        demand_strategy(rules, query, policy, false),
    );
}

#[test]
fn fresh_derivation_templates_preserve_unknowns_choices_and_call_identity() {
    for choice in [false, true] {
        let a = eq(v(0), t("box", [v(99)]));
        let body = if choice {
            or(a, eq(v(0), t("other", [v(99)])))
        } else {
            a
        };
        let rules = vec![
            Rule::simplify("base", [c("build", [atom("z"), v(0)])], body),
            Rule::simplify(
                "step",
                [c("build", [t("s", [v(0)]), v(1)])],
                c("build", [v(0), v(1)]).into(),
            ),
        ];
        for depth in [0, 1, 8, 65] {
            for separate in [false, true] {
                let input = (0..depth).fold(atom("z"), |x, _| t("s", [x]));
                let mut constraints = vec![c("build", [input.clone(), v(100)])];
                if separate {
                    constraints.push(c("build", [input, v(101)]));
                }
                let values = if choice {
                    vec!["box", "other"]
                } else {
                    vec!["box"]
                };
                let mut expected = vec![];
                for x in &values {
                    for y in &values {
                        if !separate && x != y {
                            continue;
                        }
                        expected.push(Answer {
                            outputs: vec![
                                ("x".into(), t(x, [v(900)])),
                                ("y".into(), t(y, [v(if separate { 901 } else { 900 })])),
                            ],
                            residual: vec![],
                        });
                    }
                }
                collect(
                    rules.clone(),
                    Query {
                        constraints,
                        outputs: vec![
                            ("x".into(), Var(100)),
                            ("y".into(), Var(if separate { 101 } else { 100 })),
                        ],
                    },
                    expected,
                );
            }
        }
    }
}

#[test]
fn derivation_templates_retain_resource_claims_and_off_output_failure() {
    let mut rules = vec![
        Rule::simplify(
            "base",
            [c("build", [atom("z"), v(0)])],
            c("take", [v(0)]).into(),
        ),
        Rule::simplify(
            "step",
            [c("build", [t("s", [v(0)]), v(1)])],
            c("build", [v(0), v(1)]).into(),
        ),
        Rule::simplify(
            "take",
            [c("take", [v(0)]), c("token", [])],
            eq(v(0), t("box", [v(99)])),
        ),
    ];
    let input = (0..8).fold(atom("z"), |x, _| t("s", [x]));
    let query = Query {
        constraints: vec![
            c("build", [input.clone(), v(100)]),
            c("build", [input, v(101)]),
            c("token", []),
            c("token", []),
        ],
        outputs: vec![("x".into(), Var(100)), ("y".into(), Var(101))],
    };
    collect(
        rules.clone(),
        query.clone(),
        vec![Answer {
            outputs: vec![
                ("x".into(), t("box", [v(900)])),
                ("y".into(), t("box", [v(901)])),
            ],
            residual: vec![],
        }],
    );
    rules[0].body = and([c("bad", [v(1)]).into(), eq(v(0), atom("ok"))]);
    rules.push(Rule::simplify("bad", [c("bad", [v(0)])], Goal::Fail));
    collect(rules, query, vec![]);
}

#[test]
fn fresh_derivation_size_bound_preserves_complete_source_result() {
    let rules = vec![
        Rule::simplify("base", [c("grow", [atom("z"), v(0), v(1)])], eq(v(1), v(0))),
        Rule::simplify(
            "step",
            [c("grow", [t("s", [v(0)]), v(1), v(2)])],
            c("grow", [v(0), t("pair", [v(1), v(1)]), v(2)]).into(),
        ),
    ];
    let depth = (0..12).fold(atom("z"), |x, _| t("s", [x]));
    let expected = (0..12).fold(atom("leaf"), |x, _| t("pair", [x.clone(), x]));
    collect(
        rules,
        Query {
            constraints: vec![c("grow", [depth, atom("leaf"), v(100)])],
            outputs: vec![("x".into(), Var(100))],
        },
        vec![Answer {
            outputs: vec![("x".into(), expected)],
            residual: vec![],
        }],
    );
}

#[test]
fn fresh_derivation_boundary_preserves_residual_occurrences() {
    let rules = vec![
        Rule::simplify(
            "base",
            [c("build", [atom("z"), v(0)])],
            c("take", [v(0)]).into(),
        ),
        Rule::simplify(
            "step",
            [c("build", [t("s", [v(0)]), v(1)])],
            c("build", [v(0), v(1)]).into(),
        ),
        Rule::simplify(
            "take",
            [c("take", [v(0)]), c("token", [])],
            eq(v(0), atom("done")),
        ),
    ];
    let depth = (0..8).fold(atom("z"), |x, _| t("s", [x]));
    collect(
        rules,
        Query {
            constraints: vec![
                c("build", [depth.clone(), v(100)]),
                c("build", [depth, v(101)]),
            ],
            outputs: vec![("x".into(), Var(100)), ("y".into(), Var(101))],
        },
        vec![Answer {
            outputs: vec![("x".into(), v(900)), ("y".into(), v(901))],
            residual: vec![c("take", [v(900)]), c("take", [v(901)])],
        }],
    );
}

#[test]
fn derivation_contraction_exposes_a_resource_scheduling_difference() {
    let rules = vec![
        Rule::simplify(
            "base",
            [c("build", [atom("z"), v(0)])],
            c("take", [v(0)]).into(),
        ),
        Rule::simplify(
            "step",
            [c("build", [t("s", [v(0)]), v(1)])],
            c("build", [v(0), v(1)]).into(),
        ),
        Rule::simplify(
            "take",
            [c("take", [v(0)]), c("token", [])],
            eq(v(0), atom("done")),
        ),
    ];
    let depth = (0..8).fold(atom("z"), |x, _| t("s", [x]));
    let query = Query {
        constraints: vec![
            c("build", [depth, v(100)]),
            c("build", [atom("z"), v(101)]),
            c("token", []),
        ],
        outputs: vec![("x".into(), Var(100)), ("y".into(), Var(101))],
    };
    let long_wins = Answer {
        outputs: vec![("x".into(), atom("done")), ("y".into(), v(901))],
        residual: vec![c("take", [v(901)])],
    };
    let short_wins = Answer {
        outputs: vec![("x".into(), v(900)), ("y".into(), atom("done"))],
        residual: vec![c("take", [v(900)])],
    };
    runtime_support::same_raw(
        runtime_support::run(&rules, &query, 200_000),
        vec![short_wins.clone()],
    );
    runtime_support::same_raw(
        demand_answers(rules.clone(), query.clone()),
        vec![short_wins.clone()],
    );
    runtime_support::same_raw(
        demand_prepared(
            Prepared::new(rules.clone())
                .unwrap()
                .with_template_follow_limit(0),
            query.clone(),
        ),
        vec![short_wins.clone()],
    );
    runtime_support::same_raw(
        demand_prepared(
            Prepared::new(rules).unwrap().with_derivation_templates(),
            query,
        ),
        vec![long_wins.clone()],
    );
    assert!(!chr_observe::equivalent(
        &short_wins,
        &long_wins,
        &mut Default::default()
    ));
}
