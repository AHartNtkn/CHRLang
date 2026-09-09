#[path = "support/chr_forest.rs"]
mod forest;
#[path = "support/chr_constructors.rs"]
mod kernel;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_syntax::{and, atom, c, eq, t, v, Answer, Query, Rule, Term, Var};
fn node(i: usize) -> Term {
    atom(&format!("n{i}"))
}
fn run(rules: Vec<Rule>, query: Query, access: chr_compiled::Access) -> Vec<Answer> {
    let prepared = chr_compiled::PreparedRuleset::new(rules, None).unwrap();
    let mut search = prepared
        .start_search(query, chr_compiled::Policy::Global, access)
        .unwrap();
    let mut answers = vec![];
    for _ in 0..200_000 {
        match search.tick() {
            chr_compiled::SearchEvent::Complete(mut b) => answers.push(b.engine.observe().unwrap()),
            chr_compiled::SearchEvent::Exhausted => return answers,
            _ => (),
        }
    }
    panic!("unfinished constructor query");
}
fn decode(answer: Answer, n: usize) -> Answer {
    fn root(x: &Term, facts: &[chr_syntax::Constraint], fuel: usize) -> Term {
        assert!(fuel > 0, "parent cycle");
        match facts.iter().find(|c| c.name == "edge" && c.args[0] == *x) {
            Some(e) => root(&e.args[1], facts, fuel - 1),
            None => {
                assert!(facts.iter().any(|c| c.name == "root" && c.args[0] == *x));
                x.clone()
            }
        }
    }
    fn term(x: &Term, facts: &[chr_syntax::Constraint], n: usize, fuel: usize) -> Term {
        assert!(fuel > 0, "constructor cycle escaped kernel");
        let r = root(x, facts, n + 1);
        let ds = facts
            .iter()
            .filter(|c| c.name.starts_with("d_") && c.args[0] == r)
            .collect::<Vec<_>>();
        assert!(ds.len() <= 1, "unreconciled descriptors");
        match ds.first() {
            Some(d) => t(
                d.name.strip_prefix("d_").unwrap(),
                d.args[1..]
                    .iter()
                    .map(|x| term(x, facts, n, fuel - 1))
                    .collect::<Vec<_>>(),
            ),
            None => v((0..n).find(|&i| node(i) == r).unwrap() as u64),
        }
    }
    assert!(answer.residual.iter().all(|c| matches!(
        c.name.as_str(),
        "root" | "edge" | "below" | "d_a" | "d_b" | "d_f" | "token" | "take"
    )));
    for i in 0..n {
        assert_eq!(
            answer
                .residual
                .iter()
                .filter(|c| matches!(c.name.as_str(), "root" | "edge") && c.args[0] == node(i))
                .count(),
            1,
            "forest ownership"
        );
    }
    for fact in &answer.residual {
        if fact.name.starts_with("d_") || matches!(fact.name.as_str(), "below" | "take") {
            for arg in &fact.args {
                assert_eq!(
                    *arg,
                    root(arg, &answer.residual, n + 1),
                    "stale exposed reference"
                );
            }
        }
    }
    let outputs = (0..n)
        .map(|i| (format!("v{i}"), term(&node(i), &answer.residual, n, n + 1)))
        .collect();
    let residual = answer
        .residual
        .iter()
        .filter(|c| matches!(c.name.as_str(), "token" | "take"))
        .map(|fact| {
            c(
                &fact.name,
                fact.args
                    .iter()
                    .map(|x| term(x, &answer.residual, n, n + 1))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    Answer { outputs, residual }
}
#[test]
fn constructor_classes_agree_with_independent_finite_tree_substitution() {
    let n = 3;
    // Each node is unknown, a, b, or f(one of the three nodes).
    for code in 0..216 {
        let kinds = [code % 6, (code / 6) % 6, (code / 36) % 6];
        for union in 0..10 {
            let mut facts = (0..n).map(|i| c("root", [node(i)])).collect::<Vec<_>>();
            let mut equations = vec![];
            for (i, &kind) in kinds.iter().enumerate() {
                match kind {
                    0 => (),
                    1 | 2 => {
                        let name = if kind == 1 { "a" } else { "b" };
                        facts.push(c(&format!("d_{name}"), [node(i)]));
                        equations.push(eq(v(i as u64), atom(name)));
                    }
                    _ => {
                        facts.push(c("d_f", [node(i), node(kind - 3)]));
                        equations.push(eq(v(i as u64), t("f", [v((kind - 3) as u64)])));
                    }
                }
            }
            if union < 9 {
                facts.push(c("union", [node(union / 3), node(union % 3)]));
                equations.push(eq(v((union / 3) as u64), v((union % 3) as u64)));
            }
            let source = vec![Rule::simplify(
                "equations",
                [c("start", [v(0), v(1), v(2)])],
                and(equations),
            )];
            let query = Query {
                constraints: vec![c("start", [v(0), v(1), v(2)])],
                outputs: (0..n).map(|i| (format!("v{i}"), Var(i as u64))).collect(),
            };
            let expected = oracle::run(&source, &query, 200_000);
            for reverse in [false, true] {
                let mut facts = facts.clone();
                if reverse {
                    facts.reverse();
                }
                for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                    let actual = run(
                        kernel::rules(),
                        Query {
                            constraints: facts.clone(),
                            outputs: vec![],
                        },
                        access,
                    )
                    .into_iter()
                    .map(|a| decode(a, n))
                    .collect();
                    oracle::same_raw(actual, expected.clone());
                }
            }
        }
    }
}
#[test]
fn equality_enables_consumption_which_enables_another_consumption() {
    for tokens in 0..=3 {
        let source = vec![
            Rule {
                name: "take".into(),
                kept: vec![],
                removed: vec![c("take", [t("f", [v(0)]), v(1)]), c("token", [])],
                guards: vec![],
                body: eq(v(1), v(0)),
            },
            Rule::simplify(
                "supply",
                [c("supply", [v(0)])],
                eq(v(0), t("f", [t("f", [atom("a")])])),
            ),
        ];
        let mut ordinary = vec![
            c("take", [v(0), v(1)]),
            c("take", [v(1), v(2)]),
            c("supply", [v(0)]),
        ];
        ordinary.extend((0..tokens).map(|_| c("token", [])));
        let expected = oracle::run(
            &source,
            &Query {
                constraints: ordinary,
                outputs: (0..3).map(|i| (format!("v{i}"), Var(i))).collect(),
            },
            200_000,
        );
        let mut facts = (0..6).map(|i| c("root", [node(i)])).collect::<Vec<_>>();
        facts.extend([
            c("d_f", [node(3), node(4)]),
            c("d_f", [node(4), node(5)]),
            c("d_a", [node(5)]),
            c("union", [node(0), node(3)]),
            c("take", [node(0), node(1)]),
            c("take", [node(1), node(2)]),
        ]);
        facts.extend((0..tokens).map(|_| c("token", [])));
        let rules = kernel::rules();
        let traces = oracle::run_traced(
            &rules,
            &Query {
                constraints: facts.clone(),
                outputs: vec![],
            },
            200_000,
        );
        assert_eq!(traces.len(), 1);
        let trace = &traces[0].1;
        let takes = trace
            .iter()
            .enumerate()
            .filter(|(_, i)| rules[**i].name == "take-f")
            .map(|(p, _)| p)
            .collect::<Vec<_>>();
        assert_eq!(takes.len(), tokens.min(2));
        if takes.len() == 2 {
            assert!(trace[takes[0] + 1..takes[1]]
                .iter()
                .any(|&i| rules[i].name == "link-roots"));
            assert!(trace[takes[0] + 1..takes[1]]
                .iter()
                .any(|&i| rules[i].name == "repair-take-0"));
        }
        for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
            let actual = run(
                kernel::rules(),
                Query {
                    constraints: facts.clone(),
                    outputs: vec![],
                },
                access,
            )
            .into_iter()
            .map(|a| {
                let mut d = decode(a, 6);
                d.outputs.truncate(3);
                d
            })
            .collect();
            oracle::same_raw(actual, expected.clone());
        }
    }
}

#[test]
fn choice_keeps_consumption_local_and_checks_cycles() {
    use chr_syntax::Goal;
    for cyclic in [false, true] {
        for tokens in 0..=2 {
            let take = Rule::simplify(
                "take",
                [c("take", [t("f", [v(0)]), v(1)]), c("token", [])],
                eq(v(1), v(0)),
            );
            let alternative = if cyclic { t("f", [v(0)]) } else { atom("b") };
            let source = vec![
                take,
                Rule::simplify(
                    "choose",
                    [c("choose", [v(0)])],
                    Goal::Or(
                        Box::new(eq(v(0), t("f", [atom("a")]))),
                        Box::new(eq(v(0), alternative)),
                    ),
                ),
            ];
            let mut ordinary = vec![c("choose", [v(0)]), c("take", [v(0), v(1)])];
            ordinary.extend((0..tokens).map(|_| c("token", [])));
            let expected = oracle::run(
                &source,
                &Query {
                    constraints: ordinary,
                    outputs: vec![("v0".into(), Var(0)), ("v1".into(), Var(1))],
                },
                200_000,
            );
            let mut rules = kernel::rules();
            rules.push(Rule::simplify(
                "choose",
                [c("choose", [v(0), v(1)])],
                Goal::Or(
                    Box::new(c("d_f", [v(0), v(1)]).into()),
                    Box::new(if cyclic {
                        c("d_f", [v(0), v(0)]).into()
                    } else {
                        c("d_b", [v(0)]).into()
                    }),
                ),
            ));
            let mut facts = (0..3).map(|i| c("root", [node(i)])).collect::<Vec<_>>();
            facts.extend([
                c("choose", [node(0), node(2)]),
                c("d_a", [node(2)]),
                c("take", [node(0), node(1)]),
            ]);
            facts.extend((0..tokens).map(|_| c("token", [])));
            for reverse in [false, true] {
                let mut facts = facts.clone();
                if reverse {
                    facts.reverse();
                }
                for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                    let actual = run(
                        rules.clone(),
                        Query {
                            constraints: facts.clone(),
                            outputs: vec![],
                        },
                        access,
                    )
                    .into_iter()
                    .map(|a| {
                        let mut a = decode(a, 3);
                        a.outputs.truncate(2);
                        a
                    })
                    .collect();
                    oracle::same_raw(actual, expected.clone());
                }
            }
        }
    }
}

#[test]
fn competing_consumers_expose_descriptor_order_as_a_scheduling_choice() {
    let source = vec![Rule::simplify(
        "take",
        [c("take", [t("f", [v(0)]), v(1)]), c("token", [])],
        eq(v(1), v(0)),
    )];
    for reverse_requests in [false, true] {
        let mut requests = vec![
            c("take", [t("f", [atom("a")]), v(0)]),
            c("take", [t("f", [atom("b")]), v(1)]),
        ];
        if reverse_requests {
            requests.reverse();
        }
        requests.push(c("token", []));
        let expected = oracle::run(
            &source,
            &Query {
                constraints: requests,
                outputs: vec![("v1".into(), Var(0)), ("v3".into(), Var(1))],
            },
            200_000,
        );
        let mut facts = (0..6).map(|i| c("root", [node(i)])).collect::<Vec<_>>();
        facts.extend([
            c("d_f", [node(0), node(4)]),
            c("d_f", [node(2), node(5)]),
            c("d_a", [node(4)]),
            c("d_b", [node(5)]),
        ]);
        let mut requests = vec![c("take", [node(0), node(1)]), c("take", [node(2), node(3)])];
        if reverse_requests {
            requests.reverse();
        }
        facts.extend(requests);
        facts.push(c("token", []));
        for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
            let actual = run(
                kernel::rules(),
                Query {
                    constraints: facts.clone(),
                    outputs: vec![],
                },
                access,
            )
            .into_iter()
            .map(|a| {
                let mut d = decode(a, 6);
                d.outputs = vec![d.outputs[1].clone(), d.outputs[3].clone()];
                d
            })
            .collect::<Vec<_>>();
            assert_eq!(actual.len(), 1);
            println!(
                "reverse={reverse_requests} access={access:?} source={:?} kernel={:?}",
                expected[0].outputs, actual[0].outputs
            );
            let first_wins = Answer {
                outputs: vec![("v1".into(), atom("a")), ("v3".into(), v(1))],
                residual: vec![c("take", [t("f", [atom("b")]), v(1)])],
            };
            let second_wins = Answer {
                outputs: vec![("v1".into(), v(0)), ("v3".into(), atom("b"))],
                residual: vec![c("take", [t("f", [atom("a")]), v(0)])],
            };
            oracle::same_raw(
                expected.clone(),
                vec![if reverse_requests {
                    second_wins
                } else {
                    first_wins.clone()
                }],
            );
            // Exact counterexample, not an arbitrary permitted-winner check.
            assert_eq!(
                chr_observe::equivalent(&actual[0], &expected[0], &mut Default::default()),
                !reverse_requests
            );
            oracle::same_raw(actual, vec![first_wins]);
        }
    }
}

#[test]
fn a_hidden_constructor_cycle_cannot_publish_or_poison_its_sibling() {
    use chr_syntax::Goal;
    let source = vec![Rule::simplify(
        "choose",
        [c("choose", [])],
        Goal::Or(Box::new(Goal::True), Box::new(eq(v(0), t("f", [v(0)])))),
    )];
    let expected = oracle::run(
        &source,
        &Query {
            constraints: vec![c("choose", []), c("token", [])],
            outputs: vec![],
        },
        200_000,
    );
    assert_eq!(expected.len(), 1);
    let mut rules = kernel::rules();
    rules.push(Rule::simplify(
        "choose",
        [c("choose", [v(0)])],
        Goal::Or(
            Box::new(Goal::True),
            Box::new(c("d_f", [v(0), v(0)]).into()),
        ),
    ));
    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
        let actual = run(
            rules.clone(),
            Query {
                constraints: vec![c("root", [node(0)]), c("choose", [node(0)]), c("token", [])],
                outputs: vec![],
            },
            access,
        )
        .into_iter()
        .map(|a| {
            let mut a = decode(a, 1);
            a.outputs.clear();
            a
        })
        .collect();
        oracle::same_raw(actual, expected.clone());
    }
}
