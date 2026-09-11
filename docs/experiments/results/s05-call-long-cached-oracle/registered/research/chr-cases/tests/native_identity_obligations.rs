use chr_reference::Search;
use chr_syntax::{Answer, Constraint, Query, Rule, Var, and, atom, c, eq, v};

fn check(
    rules: Vec<Rule>,
    query: Query,
    outputs: Vec<(String, chr_syntax::Term)>,
    mut residual: Vec<Constraint>,
) {
    let mut search = Search::new(rules, query).unwrap();
    let batch = search.advance(20_000);
    assert!(batch.exhausted);
    assert_eq!(search.stats().completed_branches, 1);
    residual.sort();
    assert_eq!(batch.answers, vec![Answer { outputs, residual }]);
}
fn watch() -> Rule {
    Rule::propagate("watch", [c("p", [v(0)])], c("seen", [v(0)]).into())
}

#[test]
fn equal_value_replacement_is_a_new_propagation_occurrence() {
    for tickets in 0..=3 {
        for reverse in [false, true] {
            let rules = vec![
                watch(),
                Rule::simplify(
                    "replace",
                    [c("ticket", []), c("p", [v(0)])],
                    c("p", [v(0)]).into(),
                ),
            ];
            let mut constraints = vec![c("p", [atom("a")])];
            constraints.extend((0..tickets).map(|_| c("ticket", [])));
            if reverse {
                constraints.reverse();
            }
            let mut expected = vec![c("p", [atom("a")])];
            expected.extend((0..=tickets).map(|_| c("seen", [atom("a")])));
            check(
                rules,
                Query {
                    constraints,
                    outputs: vec![],
                },
                vec![],
                expected,
            );
        }
    }
}

#[test]
fn binding_changes_values_without_creating_a_new_propagation_occurrence() {
    for id in [10, 100, 1000] {
        for reverse in [false, true] {
            let rules = vec![
                watch(),
                Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
            ];
            let mut constraints = vec![c("p", [v(id)]), c("bind", [v(id)])];
            if reverse {
                constraints.reverse();
            }
            check(
                rules,
                Query {
                    constraints,
                    outputs: vec![("x".into(), Var(id))],
                },
                vec![("x".into(), atom("a"))],
                vec![c("p", [atom("a")]), c("seen", [atom("a")])],
            );
        }
    }
}

#[test]
fn replacement_can_introduce_a_fresh_unknown_without_aliasing_the_old_value() {
    for id in [10, 100, 1000] {
        for reverse in [false, true] {
            let rules = vec![
                watch(),
                Rule::simplify(
                    "replace",
                    [c("ticket", []), c("p", [v(0)])],
                    c("p", [v(1)]).into(),
                ),
            ];
            let mut constraints = vec![c("p", [v(id)]), c("ticket", [])];
            if reverse {
                constraints.reverse();
            }
            check(
                rules,
                Query {
                    constraints,
                    outputs: vec![("old".into(), Var(id))],
                },
                vec![("old".into(), v(0))],
                vec![c("seen", [v(0)]), c("seen", [v(1)]), c("p", [v(1)])],
            );
        }
    }
}

#[test]
fn propagation_histories_distinguish_ordered_occurrence_tuples() {
    for count in 0..=3 {
        let rules = vec![Rule::propagate(
            "pairs",
            [c("p", [v(0)]), c("p", [v(1)])],
            c("seen", [v(0), v(1)]).into(),
        )];
        let mut residual = vec![c("p", [atom("a")]); count];
        residual.extend(
            (0..count * (count.saturating_sub(1))).map(|_| c("seen", [atom("a"), atom("a")])),
        );
        check(
            rules,
            Query {
                constraints: vec![c("p", [atom("a")]); count],
                outputs: vec![],
            },
            vec![],
            residual,
        );
    }
    let rules = vec![Rule::propagate(
        "pairs",
        [c("p", [v(0)]), c("p", [v(1)])],
        c("seen", [v(0), v(1)]).into(),
    )];
    check(
        rules,
        Query {
            constraints: vec![c("p", [atom("a")]), c("p", [atom("b")])],
            outputs: vec![],
        },
        vec![],
        vec![
            c("p", [atom("a")]),
            c("p", [atom("b")]),
            c("seen", [atom("a"), atom("b")]),
            c("seen", [atom("b"), atom("a")]),
        ],
    );
}

#[test]
fn equal_final_aliases_do_not_justify_reordering_competing_effects() {
    for id in [10, 100, 1000] {
        for aliased in [false, true] {
            for binder_first in [false, true] {
                for reverse in [false, true] {
                    let join = Rule::simplify(
                        "join",
                        [c("p", [v(0)]), c("q", [v(0)])],
                        c("joined", [v(0)]).into(),
                    );
                    let take = Rule::simplify(
                        "take",
                        [c("p", [v(0)]), c("token", [])],
                        c("taken", [v(0)]).into(),
                    );
                    let bind = Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1)));
                    let rules = if binder_first {
                        vec![bind, join, take]
                    } else {
                        vec![join, take, bind]
                    };
                    let other = if aliased { id } else { id + 1 };
                    let mut constraints = vec![
                        c("p", [v(id)]),
                        c("q", [v(other)]),
                        c("bind", [v(id), v(other)]),
                        c("token", []),
                    ];
                    if reverse {
                        constraints.reverse();
                    }
                    let residual = if aliased || binder_first {
                        vec![c("joined", [v(0)]), c("token", [])]
                    } else {
                        vec![c("taken", [v(0)]), c("q", [v(0)])]
                    };
                    check(
                        rules,
                        Query {
                            constraints,
                            outputs: vec![("x".into(), Var(id)), ("y".into(), Var(other))],
                        },
                        vec![("x".into(), v(0)), ("y".into(), v(0))],
                        residual,
                    );
                }
            }
        }
    }
}

#[test]
fn independent_binding_effects_have_a_commuting_control() {
    for id in [10, 100, 1000] {
        for reverse_rules in [false, true] {
            for reverse_query in [false, true] {
                let mut rules = vec![
                    Rule::simplify(
                        "left",
                        [c("left", [v(0)])],
                        and([eq(v(0), atom("a")), c("doneleft", [v(0)]).into()]),
                    ),
                    Rule::simplify(
                        "right",
                        [c("right", [v(0)])],
                        and([eq(v(0), atom("b")), c("doneright", [v(0)]).into()]),
                    ),
                ];
                if reverse_rules {
                    rules.reverse();
                }
                let mut constraints = vec![c("left", [v(id)]), c("right", [v(id + 1)])];
                if reverse_query {
                    constraints.reverse();
                }
                check(
                    rules,
                    Query {
                        constraints,
                        outputs: vec![("x".into(), Var(id)), ("y".into(), Var(id + 1))],
                    },
                    vec![("x".into(), atom("a")), ("y".into(), atom("b"))],
                    vec![c("doneleft", [atom("a")]), c("doneright", [atom("b")])],
                );
            }
        }
    }
}
