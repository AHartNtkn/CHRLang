#[allow(dead_code)]
mod runtime_support;
use chr_direct_choice::demand::{Event, Prepared, Reuse};
use chr_syntax::{Answer, Goal, Query, Rule, Var, and, atom, c, eq, or, t, v};
fn take() -> Rule {
    Rule::simplify(
        "take",
        [c("take", [v(0), v(1)]), c("token", [v(0)])],
        eq(v(1), atom("taken")),
    )
}
fn check(rules: Vec<Rule>, query: Query, expected: Vec<Answer>) {
    runtime_support::same_raw(
        runtime_support::run(&rules, &query, 200_000),
        expected.clone(),
    );
    let p = chr_compiled::PreparedRuleset::new(rules.clone(), None)
        .unwrap()
        .specialize_inferred();
    let mut run = p
        .start_search(
            query.clone(),
            chr_compiled::Policy::Global,
            chr_compiled::Access::Indexed,
        )
        .unwrap();
    let mut actual = vec![];
    let mut done = false;
    for _ in 0..200_000 {
        match run.tick() {
            chr_compiled::SearchEvent::Complete(mut b) => actual.push(b.engine.observe().unwrap()),
            chr_compiled::SearchEvent::Exhausted => {
                done = true;
                break;
            }
            _ => (),
        }
    }
    assert!(done);
    runtime_support::same_raw(actual, expected.clone());
    for reuse in [
        Reuse::CurrentContext,
        Reuse::StaticBirth,
        Reuse::MatchDependencies,
    ] {
        for flags in 0..8 {
            let mut p = Prepared::with_reuse(rules.clone(), reuse).unwrap();
            if flags & 1 != 0 {
                p = p.with_miss_reuse();
            }
            if flags & 2 != 0 {
                p = p.with_pull_tabs();
            }
            if flags & 4 != 0 {
                p = p.with_derivation_templates();
            }
            let mut cancelled_answers = vec![];
            for steps in [1, 4] {
                let mut partial = p.start(query.clone()).unwrap();
                for _ in 0..steps {
                    match partial.tick() {
                        Event::Answer(a) => cancelled_answers.push(a),
                        Event::Exhausted => break,
                        Event::Progress => (),
                    }
                }
                drop(partial);
            }
            let mut held = vec![];
            for reverse in [false, true] {
                let mut q = query.clone();
                if reverse {
                    q.constraints.reverse();
                }
                let mut r = p.start(q).unwrap();
                let mut a = vec![];
                let mut done = false;
                for _ in 0..200_000 {
                    match r.tick() {
                        Event::Answer(x) => a.push(x),
                        Event::Exhausted => {
                            done = true;
                            break;
                        }
                        _ => (),
                    }
                }
                assert!(done, "cutoff flags={flags} reuse={reuse:?}");
                if flags & 4 != 0
                    && query
                        .constraints
                        .iter()
                        .filter(|c| c.name == "emit")
                        .count()
                        == 2
                {
                    assert!(r.retained_derivation_templates().0 > 0);
                    #[cfg(feature = "work-diagnostics")]
                    assert!(
                        r.work().template_hits > 0,
                        "fresh-call template reuse must execute"
                    );
                }
                drop(r);
                held.push(a);
            }
            drop(p);
            for answer in cancelled_answers {
                assert!(
                    expected.iter().any(|e| chr_observe::equivalent(
                        e,
                        &answer,
                        &mut Default::default()
                    )),
                    "cancelled retained output changed"
                );
            }
            for a in held {
                runtime_support::same_raw(a, expected.clone());
            }
        }
    }
}
#[test]
fn delayed_input_and_forward_producer_posts_are_consumed() {
    for forward in [false, true] {
        let emit = if forward {
            Rule::simplify(
                "emit",
                [c("emit", [v(0), v(1)])],
                and([
                    c("token", [v(2)]).into(),
                    c("value", [v(2)]).into(),
                    eq(v(1), atom("ok")),
                ]),
            )
        } else {
            Rule::simplify(
                "emit",
                [c("emit", [v(0), v(1)])],
                and([c("token", [v(0)]).into(), eq(v(1), atom("ok"))]),
            )
        };
        check(
            vec![
                emit,
                take(),
                Rule::simplify("value", [c("value", [v(0)])], eq(v(0), atom("a"))),
            ],
            Query {
                constraints: vec![
                    c("take", [atom("a"), v(100)]),
                    c("emit", [v(101), v(102)]),
                    c("value", [v(101)]),
                ],
                outputs: vec![("taken".into(), Var(100)), ("emitted".into(), Var(102))],
            },
            vec![Answer {
                outputs: vec![
                    ("taken".into(), atom("taken")),
                    ("emitted".into(), atom("ok")),
                ],
                residual: vec![],
            }],
        );
    }
}
#[test]
fn posted_call_output_tracks_the_result() {
    check(
        vec![
            take(),
            Rule::simplify(
                "emit",
                [c("emit", [v(0)])],
                and([c("token", [v(0)]).into(), eq(v(0), atom("a"))]),
            ),
        ],
        Query {
            constraints: vec![c("take", [atom("a"), v(100)]), c("emit", [v(101)])],
            outputs: vec![("out".into(), Var(101)), ("taken".into(), Var(100))],
        },
        vec![Answer {
            outputs: vec![("out".into(), atom("a")), ("taken".into(), atom("taken"))],
            residual: vec![],
        }],
    );
}
#[test]
fn fresh_post_variables_share_locally_and_remain_distinct_across_calls() {
    let sink = Rule::simplify(
        "sink",
        [c("sink", [v(0)]), c("pair", [v(1), v(2)])],
        eq(v(0), atom("ok")),
    );
    let emit = Rule::simplify(
        "emit",
        [c("emit", [atom("k"), v(7)])],
        and([
            c("pair", [v(42), v(42)]).into(),
            eq(v(7), t("box", [v(42)])),
        ]),
    );
    check(
        vec![sink, emit],
        Query {
            constraints: vec![
                c("emit", [atom("k"), v(100)]),
                c("emit", [atom("k"), v(101)]),
            ],
            outputs: vec![("x".into(), Var(100)), ("y".into(), Var(101))],
        },
        vec![Answer {
            outputs: vec![
                ("x".into(), t("box", [v(200)])),
                ("y".into(), t("box", [v(201)])),
            ],
            residual: vec![c("pair", [v(200), v(200)]), c("pair", [v(201), v(201)])],
        }],
    );
}
#[test]
fn choice_births_and_failed_posts_preserve_complete_answers() {
    let emit = Rule::simplify(
        "emit",
        [c("emit", [v(0)])],
        or(
            and([c("token", [v(1)]).into(), Goal::Fail]),
            and([c("token", [v(0)]).into(), eq(v(0), atom("a"))]),
        ),
    );
    check(
        vec![take(), emit],
        Query {
            constraints: vec![c("emit", [v(100)]), c("take", [atom("a"), v(101)])],
            outputs: vec![("out".into(), Var(100))],
        },
        vec![Answer {
            outputs: vec![("out".into(), atom("a"))],
            residual: vec![],
        }],
    );
}

#[test]
fn competing_consumers_claim_each_posted_occurrence_once() {
    for count in [1, 2] {
        let mut body = vec![c("token", [v(0)]).into(); count];
        body.push(eq(v(0), atom("a")));
        let emit = Rule::simplify("emit", [c("emit", [v(0)])], and(body));
        check(
            vec![take(), emit],
            Query {
                constraints: vec![
                    c("take", [atom("a"), v(100)]),
                    c("emit", [v(102)]),
                    c("take", [atom("a"), v(101)]),
                ],
                outputs: vec![],
            },
            vec![Answer {
                outputs: vec![],
                residual: if count == 1 {
                    vec![c("take", [atom("a"), v(200)])]
                } else {
                    vec![]
                },
            }],
        );
    }
}
#[test]
fn hidden_constructor_failure_through_a_posted_output_is_not_published() {
    let use_rule = Rule::simplify(
        "use",
        [c("use", [v(0)]), c("token", [t("f", [v(1)])])],
        eq(v(0), t("f", [v(1)])),
    );
    let emit = Rule::simplify(
        "emit",
        [c("emit", [v(0)])],
        and([
            c("token", [v(0)]).into(),
            c("use", [v(1)]).into(),
            eq(v(0), t("f", [v(1)])),
        ]),
    );
    check(
        vec![use_rule, emit],
        Query {
            constraints: vec![c("emit", [v(100)])],
            outputs: vec![],
        },
        vec![],
    );
}
#[test]
fn finite_posted_answer_receives_service_beside_recursion() {
    let rules = vec![
        take(),
        Rule::simplify("loop", [c("loop", [v(0)])], c("loop", [v(0)]).into()),
        Rule::simplify(
            "outer",
            [c("outer", [v(0)])],
            or(
                c("loop", [v(0)]).into(),
                and([c("token", [v(0)]).into(), eq(v(0), atom("done"))]),
            ),
        ),
    ];
    let q = Query {
        constraints: vec![c("outer", [v(100)])],
        outputs: vec![("out".into(), Var(100))],
    };
    let expected = vec![Answer {
        outputs: vec![("out".into(), atom("done"))],
        residual: vec![c("token", [atom("done")])],
    }];
    let p = chr_compiled::PreparedRuleset::new(rules.clone(), None)
        .unwrap()
        .specialize_inferred();
    let mut explicit = p
        .start_search(
            q.clone(),
            chr_compiled::Policy::Global,
            chr_compiled::Access::Indexed,
        )
        .unwrap();
    let mut answers = vec![];
    for _ in 0..2000 {
        match explicit.tick() {
            chr_compiled::SearchEvent::Complete(mut b) => answers.push(b.engine.observe().unwrap()),
            chr_compiled::SearchEvent::Exhausted => panic!("continuing branch reported exhausted"),
            _ => (),
        }
    }
    runtime_support::same_raw(answers, expected.clone());
    for flags in 0..8 {
        let mut p = Prepared::new(rules.clone()).unwrap();
        if flags & 1 != 0 {
            p = p.with_miss_reuse();
        }
        if flags & 2 != 0 {
            p = p.with_pull_tabs();
        }
        if flags & 4 != 0 {
            p = p.with_derivation_templates();
        }
        let mut r = p.start(q.clone()).unwrap();
        let mut answers = vec![];
        for _ in 0..2000 {
            match r.tick() {
                Event::Answer(a) => answers.push(a),
                Event::Exhausted => panic!("continuing branch reported exhausted"),
                _ => (),
            }
        }
        drop(r);
        drop(p);
        runtime_support::same_raw(answers, expected.clone());
    }
}

#[test]
fn continuing_calls_without_posts_do_not_exhaust_the_host_stack() {
    let p = Prepared::new(vec![Rule::simplify(
        "loop",
        [c("loop", [v(0)])],
        c("loop", [v(0)]).into(),
    )])
    .unwrap();
    let mut r = p
        .start(Query {
            constraints: vec![c("loop", [v(100)])],
            outputs: vec![],
        })
        .unwrap();
    for _ in 0..2000 {
        assert!(matches!(r.tick(), Event::Progress));
    }
}

#[test]
fn constructed_posts_preserve_shared_delayed_inputs() {
    let emit = Rule::simplify(
        "emit",
        [c("emit", [v(0), v(1)])],
        and([
            c("token", [t("pair", [v(0), v(0)])]).into(),
            eq(v(1), atom("ok")),
        ]),
    );
    let value = Rule::simplify(
        "value",
        [c("value", [v(0)])],
        eq(v(0), t("box", [atom("a")])),
    );
    check(
        vec![take(), emit, value],
        Query {
            constraints: vec![
                c(
                    "take",
                    [
                        t("pair", [t("box", [atom("a")]), t("box", [atom("a")])]),
                        v(100),
                    ],
                ),
                c("emit", [v(101), v(102)]),
                c("value", [v(101)]),
            ],
            outputs: vec![("taken".into(), Var(100))],
        },
        vec![Answer {
            outputs: vec![("taken".into(), atom("taken"))],
            residual: vec![],
        }],
    );
}
