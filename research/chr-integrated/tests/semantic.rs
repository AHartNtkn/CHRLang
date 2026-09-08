mod support;
use chr_integrated::{PreparedRuleset, Step};
use chr_syntax::{Answer, Goal, Guard, Query, Rule, Var, and, atom, c, eq, t, v};
use support::Replay;

fn verify(rules: &[Rule], query: &Query, expected: Option<&Answer>) {
    let prepared = PreparedRuleset::new(rules).unwrap();
    let mut engine = prepared.start(query);
    engine.enable_trace();
    assert!(engine.answer().is_none());
    let mut result = Step::Progress;
    for _ in 0..100_000 {
        result = engine.run(1);
        if !matches!(result, Step::Progress) {
            break;
        }
        assert!(
            engine.answer().is_none(),
            "a cutoff is not a completed answer"
        );
    }
    assert!(
        matches!(result, Step::Complete),
        "bounded successful source did not complete: {result:?}"
    );
    verify_complete(rules, query, expected, &engine);
}
fn verify_complete(
    rules: &[Rule],
    query: &Query,
    expected: Option<&Answer>,
    engine: &chr_integrated::Engine,
) {
    let answer = engine.answer().expect("completed successful answer");
    let audit = engine.audit();
    assert!(audit.complete && audit.indexes_valid && !audit.inconsistent);
    assert_eq!(
        (
            audit.pending_equalities,
            audit.pending_repairs,
            audit.pending_source,
            audit.pending_bodies,
            audit.enabled_applications
        ),
        (0, 0, 0, 0, 0)
    );
    let mut replay = Replay::new(query);
    for commit in engine.trace() {
        replay
            .fire(commit.rule, &rules[commit.rule], &commit.ids)
            .expect("successful integrated trace must serialize");
    }
    assert!(
        replay.terminal(rules),
        "independent replay finds outstanding source work"
    );
    assert!(
        chr_observe::equivalent(&answer, &replay.answer(), &mut Default::default()),
        "candidate and independently reconstructed answer disagree"
    );
    let view = engine.view().expect("completed view");
    assert_eq!(
        view.live.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
        replay.live_ids()
    );
    assert_eq!(view.history, replay.history());
    if let Some(expected) = expected {
        assert!(
            chr_observe::equivalent(&answer, expected, &mut Default::default()),
            "analytic expectation disagrees: {answer:?}"
        );
    }
}
fn binding_rules() -> Vec<Rule> {
    vec![
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
        Rule::simplify(
            "open",
            [c("open", [v(0), t("f", [v(1)])]), c("ticket", [v(0)])],
            c("result", [v(0), v(1)]).into(),
        ),
        Rule::propagate(
            "seen",
            [c("result", [v(0), v(1)])],
            c("seen", [v(0), v(1)]).into(),
        ),
    ]
}
#[test]
fn equality_activates_consumption_without_fusing_resources() {
    let rules = binding_rules();
    for n in [1, 2, 3] {
        for indirect in [false, true] {
            let mut constraints = vec![];
            for _ in 0..n {
                constraints.extend([c("open", [atom("k"), v(0)]), c("ticket", [atom("k")])]);
            }
            if indirect {
                constraints.extend([
                    c("bind", [v(0), v(1)]),
                    c("bind", [v(1), t("f", [atom("a")])]),
                ]);
            } else {
                constraints.push(c("bind", [v(0), t("f", [atom("a")])]));
            }
            let expected = Answer {
                outputs: vec![("x".into(), t("f", [atom("a")]))],
                residual: (0..n)
                    .flat_map(|_| {
                        [
                            c("result", [atom("k"), atom("a")]),
                            c("seen", [atom("k"), atom("a")]),
                        ]
                    })
                    .collect(),
            };
            for shift in 0..constraints.len() {
                let mut input = constraints.clone();
                input.rotate_left(shift);
                verify(
                    &rules,
                    &Query {
                        constraints: input,
                        outputs: vec![("x".into(), Var(0))],
                    },
                    Some(&expected),
                );
            }
        }
    }
}
#[test]
fn unknown_heads_and_joint_aliases_remain_most_general() {
    let rules = binding_rules();
    let q = Query {
        constraints: vec![c("open", [atom("k"), v(0)]), c("ticket", [atom("k")])],
        outputs: vec![("x".into(), Var(0))],
    };
    verify(
        &rules,
        &q,
        Some(&Answer {
            outputs: vec![("x".into(), v(0))],
            residual: q.constraints.clone(),
        }),
    );
    let q = Query {
        constraints: vec![
            c("r", [v(0)]),
            c("s", [v(1), v(2)]),
            c("bind", [v(0), v(1)]),
            c("bind", [v(1), v(0)]),
        ],
        outputs: vec![
            ("x".into(), Var(0)),
            ("y".into(), Var(1)),
            ("z".into(), Var(2)),
        ],
    };
    verify(
        &rules,
        &q,
        Some(&Answer {
            outputs: vec![("x".into(), v(0)), ("y".into(), v(0)), ("z".into(), v(2))],
            residual: vec![c("r", [v(0)]), c("s", [v(0), v(2)])],
        }),
    );
}
#[test]
fn repeated_slots_and_positive_guards_wait_for_equality() {
    for guard in [false, true] {
        let mut rules = vec![binding_rules()[0].clone()];
        rules.push(if guard {
            Rule {
                name: "guard".into(),
                kept: vec![],
                removed: vec![c("p", [v(0), v(1)])],
                guards: vec![Guard::Equal(v(0), v(1))],
                body: c("hit", []).into(),
            }
        } else {
            Rule::simplify("repeat", [c("p", [v(0), v(0)])], c("hit", []).into())
        });
        for bound in [false, true] {
            let mut input = vec![c("p", [v(0), v(1)])];
            if bound {
                input.push(c("bind", [v(0), v(1)]));
            }
            verify(
                &rules,
                &Query {
                    constraints: input,
                    outputs: vec![],
                },
                Some(&Answer {
                    outputs: vec![],
                    residual: if bound {
                        vec![c("hit", [])]
                    } else {
                        vec![c("p", [v(0), v(1)])]
                    },
                }),
            );
        }
    }
}
#[test]
fn fresh_locals_and_propagation_are_occurrence_sensitive() {
    let rules = vec![Rule::simplify(
        "fresh",
        [c("seed", [])],
        c("pair", [v(0), v(0)]).into(),
    )];
    verify(
        &rules,
        &Query {
            constraints: vec![c("seed", []), c("seed", [])],
            outputs: vec![],
        },
        Some(&Answer {
            outputs: vec![],
            residual: vec![c("pair", [v(0), v(0)]), c("pair", [v(1), v(1)])],
        }),
    );
    let rules = vec![
        Rule::propagate("once", [c("p", [v(0)])], c("seen", [v(0)]).into()),
        binding_rules()[0].clone(),
    ];
    verify(
        &rules,
        &Query {
            constraints: vec![
                c("p", [v(0)]),
                c("p", [v(1)]),
                c("bind", [v(0), v(1)]),
                c("bind", [v(1), atom("a")]),
            ],
            outputs: vec![],
        },
        Some(&Answer {
            outputs: vec![],
            residual: vec![
                c("p", [atom("a")]),
                c("p", [atom("a")]),
                c("seen", [atom("a")]),
                c("seen", [atom("a")]),
            ],
        }),
    );
    let rules = vec![Rule::propagate(
        "ordered",
        [c("p", [v(0)]), c("p", [v(0)])],
        c("hit", []).into(),
    )];
    verify(
        &rules,
        &Query {
            constraints: vec![c("p", [atom("a")]), c("p", [atom("a")])],
            outputs: vec![],
        },
        Some(&Answer {
            outputs: vec![],
            residual: vec![
                c("p", [atom("a")]),
                c("p", [atom("a")]),
                c("hit", []),
                c("hit", []),
            ],
        }),
    );
}
#[test]
fn legal_source_competition_is_not_implicit_search() {
    let rules = vec![
        Rule::simplify("one", [c("p", [])], c("one", []).into()),
        Rule::simplify("two", [c("p", [])], c("two", []).into()),
    ];
    verify(
        &rules,
        &Query {
            constraints: vec![c("p", [])],
            outputs: vec![],
        },
        None,
    );
}
fn interleaving(failed: bool, recursive: bool) -> (Vec<Rule>, Query) {
    let mut left = vec![v(0)];
    let mut right = vec![atom("b")];
    for i in 1..=64 {
        left.push(v(i));
        right.push(atom("a"));
    }
    if failed {
        left.push(atom("a"));
        right.push(atom("c"));
    }
    (
        vec![
            Rule::simplify(
                "start",
                [c("start", [v(0)])],
                eq(t("tuple", left), t("tuple", right)),
            ),
            Rule::simplify(
                "use",
                [c("p", [atom("b")])],
                if recursive {
                    c("p", [atom("b")]).into()
                } else {
                    c("hit", []).into()
                },
            ),
        ],
        Query {
            constraints: vec![c("start", [v(0)]), c("p", [v(0)])],
            outputs: vec![("x".into(), Var(0))],
        },
    )
}
#[test]
fn source_effects_run_during_pending_equality_but_never_publish_failure() {
    for failed in [false, true] {
        let (rules, q) = interleaving(failed, false);
        let p = PreparedRuleset::new(&rules).unwrap();
        let mut e = p.start(&q);
        e.enable_trace();
        let result = e.run(100_000);
        assert!(
            e.trace()
                .iter()
                .any(|c| c.rule == 1 && c.pending_equalities > 0),
            "requires a source application while consistency deductions remain pending"
        );
        if failed {
            assert!(matches!(result, Step::Failed));
            assert!(e.answer().is_none());
        } else {
            assert!(matches!(result, Step::Complete));
            verify_complete(
                &rules,
                &q,
                Some(&Answer {
                    outputs: vec![("x".into(), atom("b"))],
                    residual: vec![c("hit", [])],
                }),
                &e,
            );
        }
    }
}
#[test]
fn pending_contradiction_receives_service_beside_recursive_source_work() {
    let (rules, q) = interleaving(true, true);
    let p = PreparedRuleset::new(&rules).unwrap();
    let mut e = p.start(&q);
    e.enable_trace();
    assert!(matches!(e.run(100_000), Step::Failed));
    assert!(e.answer().is_none());
    assert!(
        e.trace()
            .iter()
            .any(|c| c.rule == 1 && c.pending_equalities > 0)
    );
}
#[test]
fn finite_tree_and_complete_body_obligations_prevent_publication() {
    for body in [
        eq(v(0), t("f", [v(0)])),
        and(vec![eq(v(0), t("f", [v(1)])), eq(v(1), t("g", [v(0)]))]),
        eq(t("f", [atom("a")]), t("f", [atom("a"), atom("b")])),
        eq(atom("a"), atom("b")),
        and(vec![
            c("hit", []).into(),
            and(vec![Goal::True, eq(atom("a"), atom("b"))]),
        ]),
    ] {
        let rules = vec![Rule::simplify("start", [c("start", [v(0)])], body)];
        let p = PreparedRuleset::new(&rules).unwrap();
        let mut e = p.start(&Query {
            constraints: vec![c("start", [v(0)])],
            outputs: vec![],
        });
        assert!(matches!(e.run(100_000), Step::Failed));
        assert!(e.answer().is_none());
    }
}
#[test]
fn preparation_reuse_has_independent_query_lifetimes() {
    let rules = binding_rules();
    let p = PreparedRuleset::new(&rules).unwrap();
    let a = Query {
        constraints: vec![c("bind", [v(0), atom("a")])],
        outputs: vec![("x".into(), Var(0))],
    };
    let b = Query {
        constraints: vec![c("bind", [v(0), atom("b")])],
        outputs: a.outputs.clone(),
    };
    let mut one = p.start(&a);
    let mut two = p.start(&b);
    assert!(matches!(one.run(1000), Step::Complete));
    assert!(matches!(two.run(1000), Step::Complete));
    assert_eq!(one.answer().unwrap().outputs[0].1, atom("a"));
    assert_eq!(two.answer().unwrap().outputs[0].1, atom("b"));
}

#[test]
fn nested_congruence_and_constructor_guards_repair_access() {
    let rules = vec![
        binding_rules()[0].clone(),
        Rule::simplify(
            "join",
            [c("p", [v(0)]), c("q", [v(0)])],
            c("hit", []).into(),
        ),
    ];
    let input = vec![
        c("p", [t("f", [t("g", [v(0)])])]),
        c("q", [t("f", [t("g", [v(1)])])]),
        c("bind", [v(0), v(1)]),
    ];
    for shift in 0..input.len() {
        let mut constraints = input.clone();
        constraints.rotate_left(shift);
        verify(
            &rules,
            &Query {
                constraints,
                outputs: vec![],
            },
            Some(&Answer {
                outputs: vec![],
                residual: vec![c("hit", [])],
            }),
        );
    }
    let rules = vec![
        binding_rules()[0].clone(),
        Rule {
            name: "guard".into(),
            kept: vec![],
            removed: vec![c("p", [v(0), v(1)])],
            guards: vec![Guard::Equal(v(0), t("f", [v(1)]))],
            body: c("hit", []).into(),
        },
    ];
    verify(
        &rules,
        &Query {
            constraints: vec![c("p", [v(0), v(1)]), c("bind", [v(0), t("f", [v(1)])])],
            outputs: vec![],
        },
        Some(&Answer {
            outputs: vec![],
            residual: vec![c("hit", [])],
        }),
    );
}
#[test]
fn mixed_heads_keep_one_resource_across_two_consumptions() {
    let rules = vec![Rule {
        name: "mixed".into(),
        kept: vec![c("keep", [v(0)])],
        removed: vec![c("take", [v(0)])],
        guards: vec![],
        body: c("hit", [v(0)]).into(),
    }];
    verify(
        &rules,
        &Query {
            constraints: vec![
                c("keep", [atom("a")]),
                c("take", [atom("a")]),
                c("take", [atom("a")]),
            ],
            outputs: vec![],
        },
        Some(&Answer {
            outputs: vec![],
            residual: vec![
                c("keep", [atom("a")]),
                c("hit", [atom("a")]),
                c("hit", [atom("a")]),
            ],
        }),
    );
}

#[allow(dead_code)]
#[path = "../experiments/workloads.rs"]
mod workloads;
#[cfg(feature = "experiment")]
include!(concat!(env!("OUT_DIR"), "/generated.rs"));
#[test]
fn registered_cost_workloads_have_independent_complete_observations() {
    use workloads::Family;
    assert!(Family::parse("unknown").is_none());
    for family in [
        Family::Independent,
        Family::Fanout,
        Family::Repair,
        Family::Build,
    ] {
        for depth in [1, 3] {
            for size in [2, 3] {
                let rules = workloads::programs()[family.program()].clone();
                let (query, expected) = workloads::case(family, size, depth);
                verify(&rules, &query, Some(&expected));
                #[cfg(feature = "experiment")]
                for generated in [false, true] {
                    let p = chr_compiled::PreparedRuleset::new(
                        rules.clone(),
                        generated.then(|| code(family.program())),
                    )
                    .unwrap();
                    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                        let mut e = p
                            .start(query.clone(), chr_compiled::Policy::Active, access)
                            .unwrap();
                        let result = e.advance(100_000);
                        assert!(result.exhausted && !result.failed);
                        assert!(chr_observe::equivalent(
                            &e.observe().unwrap(),
                            &expected,
                            &mut Default::default()
                        ));
                    }
                }
            }
        }
    }
}
