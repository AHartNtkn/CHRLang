#[path = "support/chr_forest.rs"]
mod forest;
#[path = "support/chr_constructors.rs"]
mod kernel;
#[path = "support/local_ports.rs"]
mod local;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_syntax::{Answer, Query, Rule, Term, Var, and, atom, c, eq, t, v};
fn take_plan() -> std::sync::Arc<local::Plan> {
    local::Plan::compile(&Rule::simplify(
        "take",
        [c("take", [t("f", [v(0)]), v(1)]), c("token", [])],
        eq(v(1), v(0)),
    ))
    .unwrap()
}
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
            let mut local = local::Run::default();
            let values = (0..n).map(|_| local.value()).collect::<Vec<_>>();
            for (i, &kind) in kinds.iter().enumerate() {
                match kind {
                    0 => (),
                    1 | 2 => local.describe(values[i], if kind == 1 { "a" } else { "b" }, vec![]),
                    _ => local.describe(values[i], "f", vec![values[kind - 3]]),
                }
            }
            if union < 9 {
                local.equate(values[union / 3], values[union % 3]);
            }
            local.settle();
            oracle::same_raw(
                local.answer(&values).into_iter().collect(),
                expected.clone(),
            );

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
        let mut local = local::Run::default();
        let values = (0..3).map(|_| local.value()).collect::<Vec<_>>();
        local.post(&take_plan(), values[0], values[1]);
        local.post(&take_plan(), values[1], values[2]);
        for _ in 0..tokens {
            local.token();
        }
        local.settle(); // Both requests suspend before constructor information.
        assert!(local.consumed_tokens.is_empty());
        let mid = local.value();
        let leaf = local.value();
        local.describe(leaf, "a", vec![]);
        local.describe(mid, "f", vec![leaf]);
        local.describe(values[0], "f", vec![mid]);
        local.settle();
        assert_eq!(local.consumed_tokens.len(), tokens.min(2));
        oracle::same_raw(
            local.answer(&values).into_iter().collect(),
            expected.clone(),
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
            assert!(
                trace[takes[0] + 1..takes[1]]
                    .iter()
                    .any(|&i| rules[i].name == "link-roots")
            );
            assert!(
                trace[takes[0] + 1..takes[1]]
                    .iter()
                    .any(|&i| rules[i].name == "repair-take-0")
            );
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
        let mut local = local::Run::default();
        let values = (0..6).map(|_| local.value()).collect::<Vec<_>>();
        local.describe(values[4], "a", vec![]);
        local.describe(values[5], "b", vec![]);
        local.describe(values[0], "f", vec![values[4]]);
        local.describe(values[2], "f", vec![values[5]]);
        for i in if reverse_requests { [2, 0] } else { [0, 2] } {
            local.post(&take_plan(), values[i], values[i + 1]);
        }
        local.token();
        local.settle();
        let mut answer = local.answer(&[values[1], values[3]]).unwrap();
        answer.outputs[0].0 = "v1".into();
        answer.outputs[1].0 = "v3".into();
        oracle::same_raw(vec![answer], expected.clone());
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

#[test]
fn local_handle_repair_wakes_only_the_affected_region() {
    for unrelated in [0, 8, 64] {
        let mut run = local::Run::default();
        let target = run.value();
        let result = run.value();
        run.post(&take_plan(), target, result);
        for _ in 0..unrelated {
            let a = run.value();
            let b = run.value();
            run.post(&take_plan(), a, b);
        }
        run.token();
        run.settle();
        let before = run.visited_requests;
        let leaf = run.value();
        run.describe(leaf, "a", vec![]);
        run.describe(target, "f", vec![leaf]);
        run.settle();
        assert_eq!(
            run.visited_requests - before,
            1,
            "unrelated requests were revisited"
        );
        assert_eq!(run.consumed_tokens, vec![0]);
        let answer = run.answer(&[result]).unwrap();
        assert_eq!(answer.outputs[0].1, atom("a"));
        assert_eq!(answer.residual.len(), unrelated);
    }
}

#[test]
fn local_forks_preserve_fresh_handles_and_isolate_hidden_failure() {
    let mut base = local::Run::default();
    let x = base.value();
    let y = base.value();
    base.post(&take_plan(), x, y);
    base.token();
    base.settle();
    let mut good = base.clone();
    let a = good.value();
    good.describe(a, "a", vec![]);
    good.describe(x, "f", vec![a]);
    good.settle();
    let mut bad = base.clone();
    let hidden = bad.value();
    bad.describe(hidden, "f", vec![hidden]);
    bad.settle();
    assert!(bad.answer(&[x, y]).is_none());
    assert!(base.consumed_tokens.is_empty());
    assert_eq!(good.consumed_tokens, vec![0]);
    let expected = Answer {
        outputs: vec![("v0".into(), t("f", [atom("a")])), ("v1".into(), atom("a"))],
        residual: vec![],
    };
    oracle::same_raw(good.answer(&[x, y]).into_iter().collect(), vec![expected]);
    let untouched = Answer {
        outputs: vec![("v0".into(), v(0)), ("v1".into(), v(1))],
        residual: vec![c("take", [v(0), v(1)]), c("token", [])],
    };
    oracle::same_raw(base.answer(&[x, y]).into_iter().collect(), vec![untouched]);
}

#[test]
fn local_broad_merge_repairs_every_alias_and_services_all_consumers() {
    for mode in [
        local::DependencyMode::Endpoint,
        local::DependencyMode::Filtered,
        local::DependencyMode::Indexed,
    ] {
        for width in [1, 8, 64, 256] {
            let mut run = local::Run::with_dependencies(mode);
            let values = (0..width).map(|_| run.value()).collect::<Vec<_>>();
            let outputs = (0..width).map(|_| run.value()).collect::<Vec<_>>();
            for (&input, &output) in values.iter().zip(&outputs) {
                run.post(&take_plan(), input, output);
                run.token();
            }
            for &other in &values[1..] {
                run.equate(values[0], other);
            }
            run.settle();
            run.assert_dependency_integrity();
            assert_eq!(run.repaired_handles, width - 1);
            assert!(run.consumed_tokens.is_empty());
            assert_eq!(
                run.visited_requests, width,
                "descriptor-free aliases cannot enable a constructor match"
            );
            let leaf = run.value();
            run.describe(leaf, "a", vec![]);
            run.describe(values[0], "f", vec![leaf]);
            run.settle();
            run.assert_dependency_integrity();
            let answer = run.answer(&outputs).unwrap();
            assert!(answer.outputs.iter().all(|(_, term)| *term == atom("a")));
            assert!(answer.residual.is_empty());
            assert_eq!(run.visited_requests, 2 * width); // registration and descriptor activation only
            println!(
                "constructor width={width}: inspections={}",
                run.visited_requests
            );
            assert_eq!(run.consumed_tokens, (0..width).collect::<Vec<_>>());
        }
    }
}

#[test]
fn local_decomposition_preserves_cross_child_aliases_and_arity() {
    for clash in [false, true] {
        let mut run = local::Run::default();
        let x = run.value();
        let a = run.value();
        let b = run.value();
        let out = run.value();
        run.describe(a, "a", vec![]);
        run.describe(b, if clash { "b" } else { "a" }, vec![]);
        run.describe(out, "pair", vec![x, x]);
        run.describe(out, "pair", vec![a, b]);
        run.settle();
        let source = vec![Rule::simplify(
            "start",
            [c("start", [v(0), v(1)])],
            and(vec![
                eq(v(1), t("pair", [v(0), v(0)])),
                eq(
                    v(1),
                    t("pair", [atom("a"), atom(if clash { "b" } else { "a" })]),
                ),
            ]),
        )];
        let expected = oracle::run(
            &source,
            &Query {
                constraints: vec![c("start", [v(0), v(1)])],
                outputs: vec![("v0".into(), Var(0)), ("v1".into(), Var(1))],
            },
            200_000,
        );
        oracle::same_raw(run.answer(&[x, out]).into_iter().collect(), expected);
    }
    let mut run = local::Run::default();
    let x = run.value();
    let child = run.value();
    run.describe(x, "f", vec![child]);
    run.describe(x, "f", vec![child, child]);
    run.settle();
    assert!(run.answer(&[x]).is_none(), "different arities cannot merge");
}

#[test]
fn registering_a_known_input_does_not_revisit_existing_requests() {
    let mut run = local::Run::default();
    let x = run.value();
    let leaf = run.value();
    run.describe(leaf, "a", vec![]);
    run.describe(x, "f", vec![leaf]);
    run.settle();
    let mut outputs = vec![];
    for _ in 0..64 {
        let out = run.value();
        outputs.push(out);
        run.post(&take_plan(), x, out);
        run.token();
    }
    assert_eq!(run.visited_requests, 64);
    run.settle();
    let answer = run.answer(&outputs).unwrap();
    assert!(answer.outputs.iter().all(|(_, value)| *value == atom("a")));
    assert!(answer.residual.is_empty());
}

#[test]
fn nested_source_pattern_waits_for_inner_information() {
    let rule = Rule::simplify(
        "nested",
        [c("take", [t("f", [t("g", [v(0)])]), v(1)]), c("token", [])],
        eq(v(1), v(0)),
    );
    let plan = local::Plan::compile(&rule).unwrap();
    let mut run = local::Run::default();
    let x = run.value();
    let y = run.value();
    let inner = run.value();
    let leaf = run.value();
    run.post(&plan, x, y);
    run.token();
    run.describe(x, "f", vec![inner]);
    run.settle();
    assert!(
        run.consumed_tokens.is_empty(),
        "outer constructor is insufficient"
    );
    run.describe(leaf, "a", vec![]);
    run.describe(inner, "g", vec![leaf]);
    run.settle();
    let ordinary = vec![
        rule,
        Rule::simplify(
            "supply",
            [c("supply", [v(0)])],
            eq(v(0), t("f", [t("g", [atom("a")])])),
        ),
    ];
    let expected = oracle::run(
        &ordinary,
        &Query {
            constraints: vec![c("take", [v(0), v(1)]), c("token", []), c("supply", [v(0)])],
            outputs: vec![("v0".into(), Var(0)), ("v1".into(), Var(1))],
        },
        200_000,
    );
    oracle::same_raw(run.answer(&[x, y]).into_iter().collect(), expected);
}

#[test]
fn source_patterns_preserve_nonbinding_repeated_variables_and_late_aliases() {
    for mode in [
        local::DependencyMode::Endpoint,
        local::DependencyMode::Filtered,
        local::DependencyMode::Indexed,
    ] {
        fn embed(run: &mut local::Run, term: &Term, vars: &[usize]) -> usize {
            match term {
                Term::Var(Var(v)) => vars[*v as usize],
                Term::App(name, args) => {
                    let children = args.iter().map(|t| embed(run, t, vars)).collect();
                    let h = run.value();
                    run.describe(h, name, children);
                    h
                }
            }
        }
        let terms = [atom("a"), atom("b"), v(2), t("f", [v(2)]), t("f", [v(3)])];
        for repeated in [false, true] {
            let pattern = t("pair", [v(100), v(if repeated { 100 } else { 101 })]);
            let rule = Rule::simplify(
                "capture",
                [c("take", [pattern, v(102)]), c("token", [])],
                eq(v(102), v(100)),
            );
            let plan = local::Plan::compile(&rule).unwrap();
            for a in &terms {
                for b in &terms {
                    for late in 0..4 {
                        let input = t("pair", [a.clone(), b.clone()]);
                        let change = match late {
                            0 => chr_syntax::Goal::True,
                            1 => eq(v(2), v(3)),
                            2 => eq(v(2), atom("a")),
                            _ => eq(v(3), atom("b")),
                        };
                        let source = vec![
                            rule.clone(),
                            Rule::simplify("supply", [c("supply", [v(2), v(3)])], change),
                        ];
                        let expected = oracle::run(
                            &source,
                            &Query {
                                constraints: vec![
                                    c("take", [input.clone(), v(0)]),
                                    c("token", []),
                                    c("supply", [v(2), v(3)]),
                                ],
                                outputs: vec![
                                    ("v0".into(), Var(0)),
                                    ("v1".into(), Var(2)),
                                    ("v2".into(), Var(3)),
                                ],
                            },
                            200_000,
                        );
                        let mut run = local::Run::with_dependencies(mode);
                        let vars = (0..4).map(|_| run.value()).collect::<Vec<_>>();
                        let input = embed(&mut run, &input, &vars);
                        run.post(&plan, input, vars[0]);
                        run.token();
                        run.settle();
                        run.assert_dependency_integrity();
                        match late {
                            0 => (),
                            1 => run.equate(vars[2], vars[3]),
                            2 => run.describe(vars[2], "a", vec![]),
                            _ => run.describe(vars[3], "b", vec![]),
                        }
                        run.settle();
                        run.assert_dependency_integrity();
                        oracle::same_raw(
                            run.answer(&[vars[0], vars[2], vars[3]])
                                .into_iter()
                                .collect(),
                            expected,
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn source_plan_rejects_unimplemented_effects_and_ownership() {
    let good = Rule::simplify(
        "capture",
        [c("take", [t("f", [v(0)]), v(1)]), c("token", [])],
        eq(v(1), v(0)),
    );
    assert!(local::Plan::compile(&good).is_ok());
    let mut body = good.clone();
    body.body = chr_syntax::Goal::Or(
        Box::new(chr_syntax::Goal::True),
        Box::new(chr_syntax::Goal::True),
    );
    let mut kept = good.clone();
    kept.kept.push(c("extra", []));
    let mut resource = good.clone();
    resource.removed[1] = c("token", [v(9)]);
    let mut fresh = good.clone();
    fresh.guards.push(chr_syntax::Guard::Equal(v(0), v(9)));
    let mut bound_output = good.clone();
    bound_output.removed[0].args[0] = t("f", [v(1)]);
    for rejected in [body, kept, resource, fresh, bound_output] {
        assert!(local::Plan::compile(&rejected).is_err());
    }
}

#[test]
fn descriptor_waits_follow_both_merge_directions_and_nested_information() {
    for mode in [
        local::DependencyMode::Endpoint,
        local::DependencyMode::Filtered,
        local::DependencyMode::Indexed,
    ] {
        let rule = Rule::simplify(
            "nested",
            [c("take", [t("f", [t("g", [v(0)])]), v(1)]), c("token", [])],
            eq(v(1), v(0)),
        );
        let plan = local::Plan::compile(&rule).unwrap();
        for large_input in [false, true] {
            for large_inner in [false, true] {
                let mut run = local::Run::with_dependencies(mode);
                let input = run.value();
                let output = run.value();
                let described = run.value();
                let inner = run.value();
                let inner_alias = run.value();
                let leaf = run.value();
                for _ in 0..8 {
                    let h = run.value();
                    run.equate(if large_input { input } else { described }, h);
                    let h = run.value();
                    run.equate(if large_inner { inner } else { inner_alias }, h);
                }
                run.settle();
                run.assert_dependency_integrity();
                run.post(&plan, input, output);
                run.token();
                run.describe(described, "f", vec![inner]);
                run.settle();
                run.assert_dependency_integrity();
                assert_eq!(run.visited_requests, 1);
                run.equate(input, described);
                run.settle();
                run.assert_dependency_integrity();
                assert_eq!(run.visited_requests, 2);
                assert!(run.consumed_tokens.is_empty());
                run.equate(inner, inner_alias);
                run.settle();
                run.assert_dependency_integrity();
                assert_eq!(run.visited_requests, 2);
                run.describe(leaf, "a", vec![]);
                run.describe(inner_alias, "g", vec![leaf]);
                run.settle();
                run.assert_dependency_integrity();
                assert_eq!(run.visited_requests, 3);
                assert_eq!(
                    run.subscriptions(),
                    0,
                    "consumed nested request retains subscriptions"
                );
                assert_eq!(run.consumed_tokens, vec![0]);
                let source = vec![
                    rule.clone(),
                    Rule::simplify(
                        "supply",
                        [c("supply", [v(0)])],
                        eq(v(0), t("f", [t("g", [atom("a")])])),
                    ),
                ];
                let q = Query {
                    constraints: vec![c("take", [v(0), v(1)]), c("token", []), c("supply", [v(0)])],
                    outputs: vec![("v0".into(), Var(0)), ("v1".into(), Var(1))],
                };
                oracle::same_raw(
                    run.answer(&[input, output]).into_iter().collect(),
                    oracle::run(&source, &q, 200_000),
                );
                // Neither a consumed request nor its former nested subscriptions
                // may be revisited by later endpoint repair.
                let extra = run.value();
                run.equate(inner, extra);
                run.settle();
                run.assert_dependency_integrity();
                assert_eq!(run.visited_requests, 3);
            }
        }
    }
}

#[test]
fn unresolved_equality_observes_descendants_and_releases_consumed_dependencies() {
    for mode in [
        local::DependencyMode::Endpoint,
        local::DependencyMode::Filtered,
        local::DependencyMode::Indexed,
    ] {
        fn wrapped(depth: usize, leaf: Term) -> Term {
            (0..depth).fold(leaf, |child, _| t("f", [child]))
        }
        fn graph_wrap(run: &mut local::Run, depth: usize, leaf: usize) -> usize {
            (0..depth).fold(leaf, |child, _| {
                let h = run.value();
                run.describe(h, "f", vec![child]);
                h
            })
        }
        let rule = Rule::simplify(
            "repeat",
            [
                c("take", [t("pair", [v(10), v(10)]), v(11)]),
                c("token", []),
            ],
            eq(v(11), v(10)),
        );
        let plan = local::Plan::compile(&rule).unwrap();
        for depth in [0, 1, 3] {
            for late in 0..3 {
                for reverse in [false, true] {
                    let mut run = local::Run::with_dependencies(mode);
                    let out = run.value();
                    let a = run.value();
                    let b = run.value();
                    let left = graph_wrap(&mut run, depth, a);
                    let right = graph_wrap(&mut run, depth, b);
                    let input = run.value();
                    run.describe(input, "pair", vec![left, right]);
                    run.settle();
                    run.assert_dependency_integrity();
                    run.post(&plan, input, out);
                    run.token();
                    run.settle();
                    run.assert_dependency_integrity();
                    assert!(run.consumed_tokens.is_empty());
                    // Endpoint aliases can relocate subscriptions before the useful change.
                    for _ in 0..4 {
                        let h = run.value();
                        run.equate(if reverse { b } else { a }, h);
                    }
                    run.settle();
                    run.assert_dependency_integrity();
                    assert!(run.consumed_tokens.is_empty());
                    let change = match late {
                        0 => {
                            run.equate(if reverse { b } else { a }, if reverse { a } else { b });
                            eq(v(1), v(2))
                        }
                        1 => {
                            run.describe(a, "z", vec![]);
                            run.describe(b, "z", vec![]);
                            and([eq(v(1), atom("z")), eq(v(2), atom("z"))])
                        }
                        _ => {
                            run.describe(a, "z", vec![]);
                            run.describe(b, "w", vec![]);
                            and([eq(v(1), atom("z")), eq(v(2), atom("w"))])
                        }
                    };
                    run.settle();
                    run.assert_dependency_integrity();
                    let source = vec![
                        rule.clone(),
                        Rule::simplify("supply", [c("supply", [v(1), v(2)])], change),
                    ];
                    let q = Query {
                        constraints: vec![
                            c(
                                "take",
                                [
                                    t("pair", [wrapped(depth, v(1)), wrapped(depth, v(2))]),
                                    v(0),
                                ],
                            ),
                            c("token", []),
                            c("supply", [v(1), v(2)]),
                        ],
                        outputs: vec![
                            ("v0".into(), Var(0)),
                            ("v1".into(), Var(1)),
                            ("v2".into(), Var(2)),
                        ],
                    };
                    oracle::same_raw(
                        run.answer(&[out, a, b]).into_iter().collect(),
                        oracle::run(&source, &q, 200_000),
                    );
                    assert_eq!(run.consumed_tokens.len(), usize::from(late != 2));
                    assert_eq!(
                        run.subscriptions(),
                        0,
                        "resolved request retains subscriptions"
                    );
                    let inspections = run.visited_requests;
                    let extra = run.value();
                    run.equate(a, extra);
                    run.settle();
                    run.assert_dependency_integrity();
                    assert_eq!(
                        run.visited_requests, inspections,
                        "resolved equality or mismatch retained dependencies"
                    );
                    if late == 0 {
                        run.describe(a, "z", vec![]);
                        run.settle();
                        run.assert_dependency_integrity();
                        assert_eq!(run.visited_requests, inspections);
                        assert_eq!(
                            run.answer(&[out]).unwrap().outputs[0].1,
                            wrapped(depth, atom("z"))
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn equality_alias_overhead_is_explicit_and_both_forks_preserve_source_answers() {
    for mode in [
        local::DependencyMode::Endpoint,
        local::DependencyMode::Filtered,
        local::DependencyMode::Indexed,
    ] {
        let rule = Rule::simplify(
            "repeat",
            [
                c("take", [t("pair", [v(100), v(100)]), v(101)]),
                c("token", []),
            ],
            eq(v(101), v(100)),
        );
        let plan = local::Plan::compile(&rule).unwrap();
        for width in [8, 64] {
            let mut run = local::Run::with_dependencies(mode);
            let common = run.value();
            let others = (0..width).map(|_| run.value()).collect::<Vec<_>>();
            let outputs = (0..width).map(|_| run.value()).collect::<Vec<_>>();
            for (&other, &output) in others.iter().zip(&outputs) {
                let pair = run.value();
                run.describe(pair, "pair", vec![common, other]);
                run.settle();
                run.assert_dependency_integrity();
                run.post(&plan, pair, output);
                run.token();
            }
            for _ in 0..width {
                let h = run.value();
                run.equate(common, h);
            }
            run.settle();
            run.assert_dependency_integrity();
            assert_eq!(
                run.visited_requests,
                if mode == local::DependencyMode::Endpoint {
                    width + width * width
                } else {
                    width
                }
            );
            assert_eq!(
                run.dependency_work.notifications,
                if mode == local::DependencyMode::Indexed {
                    0
                } else {
                    width * width
                }
            );
            println!("WORK stable {mode:?} {width} {}", run.work_json());
            assert!(run.consumed_tokens.is_empty());
            println!(
                "equality width={width}: inspections before useful equality={}",
                run.visited_requests
            );
            let untouched = run.clone();
            let mut constraints = Vec::new();
            for i in 0..width {
                constraints.push(c(
                    "take",
                    [
                        t("pair", [v(0), v((i + 1) as u64)]),
                        v((width + 1 + i) as u64),
                    ],
                ));
                constraints.push(c("token", []));
            }
            let q = Query {
                constraints,
                outputs: (0..width)
                    .map(|i| (format!("v{i}"), Var((width + 1 + i) as u64)))
                    .collect(),
            };
            oracle::same_raw(
                untouched.answer(&outputs).into_iter().collect(),
                oracle::run(std::slice::from_ref(&rule), &q, 200_000),
            );
            for &h in &others {
                run.equate(common, h);
            }
            run.settle();
            run.assert_dependency_integrity();
            assert_eq!(run.consumed_tokens, (0..width).collect::<Vec<_>>());
            assert_eq!(run.subscriptions(), 0);
            println!("WORK supplied {mode:?} {width} {}", run.work_json());
            assert!(untouched.subscriptions() > 0);
            let arguments = (0..=width).map(|i| v(i as u64)).collect::<Vec<_>>();
            let mut supplied = q.clone();
            supplied.constraints.push(c("supply", arguments.clone()));
            let source = vec![
                rule.clone(),
                Rule::simplify(
                    "supply",
                    [c("supply", arguments)],
                    and((0..width)
                        .map(|i| eq(v(0), v((i + 1) as u64)))
                        .collect::<Vec<_>>()),
                ),
            ];
            oracle::same_raw(
                run.answer(&outputs).into_iter().collect(),
                oracle::run(&source, &supplied, 200_000),
            );
            assert!(untouched.consumed_tokens.is_empty());
            oracle::same_raw(
                untouched.answer(&outputs).into_iter().collect(),
                oracle::run(std::slice::from_ref(&rule), &q, 200_000),
            );
        }
    }
}

#[test]
fn indexed_equality_must_deliver_identity_notifications_and_release_relations() {
    let rule = Rule::simplify(
        "repeat",
        [
            c("take", [t("pair", [v(10), v(10)]), v(11)]),
            c("token", []),
        ],
        eq(v(11), v(10)),
    );
    let plan = local::Plan::compile(&rule).unwrap();
    let mut run = local::Run::with_dependencies(local::DependencyMode::Indexed);
    let a = run.value();
    let b = run.value();
    let out = run.value();
    let input = run.value();
    run.describe(input, "pair", vec![a, b]);
    run.settle();
    run.post(&plan, input, out);
    run.token();
    run.settle();
    assert!(run.consumed_tokens.is_empty());
    run.equate(a, b);
    run.settle();
    assert_eq!(run.consumed_tokens, vec![0]);
    assert_eq!(run.subscriptions(), 0);
    let source = vec![
        rule,
        Rule::simplify("supply", [c("supply", [v(0), v(1)])], eq(v(0), v(1))),
    ];
    let q = Query {
        constraints: vec![
            c("take", [t("pair", [v(0), v(1)]), v(2)]),
            c("token", []),
            c("supply", [v(0), v(1)]),
        ],
        outputs: vec![
            ("v0".into(), Var(0)),
            ("v1".into(), Var(1)),
            ("v2".into(), Var(2)),
        ],
    };
    oracle::same_raw(
        run.answer(&[a, b, out]).into_iter().collect(),
        oracle::run(&source, &q, 200_000),
    );
}

#[test]
fn equality_index_charges_relocation_coalescence_and_descendant_progress() {
    let rule = Rule::simplify(
        "repeat",
        [
            c("take", [t("pair", [v(1000), v(1000)]), v(1001)]),
            c("token", []),
        ],
        eq(v(1001), v(1000)),
    );
    let plan = local::Plan::compile(&rule).unwrap();
    for mode in [
        local::DependencyMode::Endpoint,
        local::DependencyMode::Filtered,
        local::DependencyMode::Indexed,
    ] {
        for width in [8, 64] {
            let mut run = local::Run::with_dependencies(mode);
            let common = run.value();
            let others = (0..width).map(|_| run.value()).collect::<Vec<_>>();
            let child_a = run.value();
            let child_b = run.value();
            let outputs = (0..width).map(|_| run.value()).collect::<Vec<_>>();
            for (&other, &output) in others.iter().zip(&outputs) {
                let input = run.value();
                run.describe(input, "pair", vec![common, other]);
                run.settle();
                run.post(&plan, input, output);
                run.token();
            }
            let bigger = run.value();
            for _ in 0..8 {
                let h = run.value();
                run.equate(bigger, h);
            }
            run.settle();
            run.equate(common, bigger);
            run.settle();
            run.assert_dependency_integrity();
            assert!(run.consumed_tokens.is_empty());
            if mode == local::DependencyMode::Indexed {
                assert_eq!(run.dependency_work.moved_relations, width);
                assert_eq!(run.dependency_work.moved_subscribers, width);
            }
            println!("WORK relocated {mode:?} {width} {}", run.work_json());
            for &other in &others[1..] {
                run.equate(others[0], other);
            }
            run.settle();
            run.assert_dependency_integrity();
            println!("WORK coalesced {mode:?} {width} {}", run.work_json());
            run.describe(common, "f", vec![child_a]);
            run.settle();
            run.assert_dependency_integrity();
            assert!(run.consumed_tokens.is_empty());
            run.describe(others[0], "f", vec![child_b]);
            run.settle();
            run.assert_dependency_integrity();
            assert!(run.consumed_tokens.is_empty());
            println!("WORK described {mode:?} {width} {}", run.work_json());
            let untouched = run.clone();
            let mut constraints = Vec::new();
            for i in 0..width {
                constraints.push(c(
                    "take",
                    [
                        t("pair", [v(0), v((i + 1) as u64)]),
                        v((width + 3 + i) as u64),
                    ],
                ));
                constraints.push(c("token", []));
            }
            let args = (0..width + 3).map(|i| v(i as u64)).collect::<Vec<_>>();
            constraints.push(c("supply", args.clone()));
            let q = Query {
                constraints,
                outputs: (0..width)
                    .map(|i| (format!("v{i}"), Var((width + 3 + i) as u64)))
                    .collect(),
            };
            let mut effects = (2..=width)
                .map(|i| eq(v(1), v(i as u64)))
                .collect::<Vec<_>>();
            effects.extend([
                eq(v(0), t("f", [v((width + 1) as u64)])),
                eq(v(1), t("f", [v((width + 2) as u64)])),
            ]);
            let source = vec![
                rule.clone(),
                Rule::simplify("supply", [c("supply", args.clone())], and(effects.clone())),
            ];
            oracle::same_raw(
                untouched.answer(&outputs).into_iter().collect(),
                oracle::run(&source, &q, 200_000),
            );
            run.equate(child_a, child_b);
            run.settle();
            run.assert_dependency_integrity();
            assert_eq!(run.consumed_tokens, (0..width).collect::<Vec<_>>());
            assert_eq!(run.subscriptions(), 0);
            effects.push(eq(v((width + 1) as u64), v((width + 2) as u64)));
            let source = vec![
                rule.clone(),
                Rule::simplify("supply", [c("supply", args)], and(effects)),
            ];
            oracle::same_raw(
                run.answer(&outputs).into_iter().collect(),
                oracle::run(&source, &q, 200_000),
            );
            untouched.assert_dependency_integrity();
            assert!(untouched.subscriptions() > 0);
            println!("WORK completed {mode:?} {width} {}", run.work_json());
        }
    }
}

#[test]
fn described_winner_awakens_relocated_pairs_without_binding_a_match() {
    let rule = Rule::simplify(
        "repeat",
        [
            c("take", [t("pair", [v(10), v(10)]), v(11)]),
            c("token", []),
        ],
        eq(v(11), v(10)),
    );
    let plan = local::Plan::compile(&rule).unwrap();
    for mode in [
        local::DependencyMode::Endpoint,
        local::DependencyMode::Filtered,
        local::DependencyMode::Indexed,
    ] {
        let mut run = local::Run::with_dependencies(mode);
        let x = run.value();
        let y = run.value();
        let a = run.value();
        let b = run.value();
        let out = run.value();
        let known = run.value();
        run.describe(known, "f", vec![a]);
        run.describe(y, "f", vec![b]);
        for _ in 0..8 {
            let h = run.value();
            run.equate(known, h);
        }
        let input = run.value();
        run.describe(input, "pair", vec![x, y]);
        run.settle();
        run.post(&plan, input, out);
        run.token();
        run.settle();
        run.equate(x, known);
        run.settle();
        run.assert_dependency_integrity();
        assert_eq!(run.visited_requests, 2);
        assert!(run.consumed_tokens.is_empty());
        run.equate(a, b);
        run.settle();
        run.assert_dependency_integrity();
        assert_eq!(run.consumed_tokens, vec![0]);
        assert_eq!(run.subscriptions(), 0);
        let source = vec![
            rule.clone(),
            Rule::simplify(
                "supply",
                [c("supply", [v(0), v(1), v(2), v(3)])],
                and([
                    eq(v(0), t("f", [v(2)])),
                    eq(v(1), t("f", [v(3)])),
                    eq(v(2), v(3)),
                ]),
            ),
        ];
        let q = Query {
            constraints: vec![
                c("take", [t("pair", [v(0), v(1)]), v(4)]),
                c("token", []),
                c("supply", [v(0), v(1), v(2), v(3)]),
            ],
            outputs: vec![("v0".into(), Var(4))],
        };
        oracle::same_raw(
            run.answer(&[out]).into_iter().collect(),
            oracle::run(&source, &q, 200_000),
        );
    }
}

#[test]
fn source_body_constructor_equation_is_an_executable_plan() {
    let bodies = vec![
        chr_syntax::Goal::True,
        chr_syntax::Goal::Fail,
        eq(v(1), t("g", [v(0), v(2), v(2)])),
        and([
            eq(v(1), t("g", [v(0), v(2), v(2)])),
            c("note", [v(1), v(2)]).into(),
            c("note", [v(1), v(2)]).into(),
        ]),
        and([eq(v(1), v(2)), eq(v(2), t("cycle", [v(2)]))]),
        eq(atom("a"), atom("b")),
        and([
            chr_syntax::Goal::True,
            and([
                eq(v(1), v(0)),
                c("take", [v(0), v(1), v(2)]).into(),
                c("token", [v(2)]).into(),
            ]),
        ]),
    ];
    let values = [
        atom("a"),
        atom("b"),
        v(50),
        t("h", [v(50), v(50)]),
        t("h", [v(50), v(51)]),
    ];
    for (body_id, body) in bodies.into_iter().enumerate() {
        let rule = Rule::simplify(
            "body",
            [c("take", [t("f", [v(0)]), v(1)]), c("token", [])],
            body,
        );
        let plan = local::Plan::compile(&rule).unwrap();
        let prepared = chr_compiled::PreparedRuleset::new(vec![rule.clone()], None).unwrap();
        let eligibility = prepared.region_eligibility();
        for (name, arity) in [("take", 2), ("token", 0)] {
            let e = eligibility
                .iter()
                .find(|e| e.predicate.0 == name && e.predicate.1 == arity)
                .unwrap();
            assert!(!e.eligible);
            println!(
                "BODY_ELIGIBILITY {body_id} {name}/{arity}: {}",
                e.reason.as_deref().unwrap()
            );
        }
        for value in &values {
            for calls in [1, 3] {
                let mut constraints = Vec::new();
                let mut outputs = Vec::new();
                for i in 0..calls {
                    constraints.extend([
                        c("take", [t("f", [value.clone()]), v(100 + i)]),
                        c("token", []),
                    ]);
                    outputs.push((format!("result{i}"), Var(100 + i)));
                }
                outputs.extend([
                    ("input".into(), Var(50)),
                    ("other".into(), Var(51)),
                    ("unused".into(), Var(9999)),
                ]);
                let q = Query {
                    constraints,
                    outputs,
                };
                for alias_output in [false, true] {
                    let mut q = q.clone();
                    if alias_output {
                        for c in &mut q.constraints {
                            if c.name == "take" {
                                c.args[1] = v(50);
                            }
                        }
                        for (name, var) in &mut q.outputs {
                            if name.starts_with("result") {
                                *var = Var(50);
                            }
                        }
                    }
                    let expected = oracle::run(std::slice::from_ref(&rule), &q, 200_000);
                    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                        let mut engine = prepared
                            .start_search(q.clone(), chr_compiled::Policy::Global, access)
                            .unwrap();
                        let mut answers = vec![];
                        let mut ended = false;
                        for _ in 0..200_000 {
                            match engine.tick() {
                                chr_compiled::SearchEvent::Complete(mut b) => {
                                    answers.push(b.engine.observe().unwrap())
                                }
                                chr_compiled::SearchEvent::Exhausted => {
                                    ended = true;
                                    break;
                                }
                                _ => (),
                            }
                        }
                        assert!(ended);
                        oracle::same_raw(answers, expected.clone());
                    }
                    for mode in [
                        local::DependencyMode::Endpoint,
                        local::DependencyMode::Filtered,
                        local::DependencyMode::Indexed,
                    ] {
                        let mut engine = plan.start(&q, mode);
                        engine.settle();
                        assert!(!engine.body_pending());
                        oracle::same_raw(engine.observe().into_iter().collect(), expected.clone());
                        if engine.observe().is_some() {
                            engine.assert_dependency_integrity();
                        }
                    }
                    assert_eq!(
                        std::sync::Arc::strong_count(&plan),
                        1,
                        "query retained prepared state after disposal"
                    );
                }
            }
        }
    }
}

#[test]
fn source_bodies_post_chained_consumers_and_preserve_fresh_residual_aliases() {
    let rule = Rule::simplify(
        "chain",
        [c("take", [t("f", [v(0)]), v(1)]), c("token", [])],
        and([
            eq(v(1), t("g", [v(0)])),
            c("take", [v(0), v(2)]).into(),
            c("token", []).into(),
            c("link", [v(1), v(2), v(2)]).into(),
        ]),
    );
    let plan = local::Plan::compile(&rule).unwrap();
    for depth in [0, 1, 4, 12] {
        for leaf in [atom("a"), v(90)] {
            let input = (0..depth).fold(leaf, |x, _| t("f", [x]));
            let q = Query {
                constraints: vec![c("take", [input, v(100)]), c("token", [])],
                outputs: vec![("result".into(), Var(100)), ("leaf".into(), Var(90))],
            };
            let expected = oracle::run(std::slice::from_ref(&rule), &q, 200_000);
            for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                oracle::same_raw(run(vec![rule.clone()], q.clone(), access), expected.clone());
            }
            for mode in [
                local::DependencyMode::Endpoint,
                local::DependencyMode::Filtered,
                local::DependencyMode::Indexed,
            ] {
                let mut e = plan.start(&q, mode);
                e.settle();
                e.assert_dependency_integrity();
                assert!(!e.body_pending());
                assert_eq!(e.consumed_tokens.len(), depth);
                oracle::same_raw(e.observe().into_iter().collect(), expected.clone());
                assert_eq!(
                    e.observe()
                        .unwrap()
                        .residual
                        .iter()
                        .filter(|c| c.name == "link")
                        .count(),
                    depth
                );
            }
        }
    }
}

#[test]
fn body_barrier_preserves_the_older_consumer_that_becomes_ready_late() {
    let rule = Rule::simplify(
        "body-order",
        [
            c("take", [t("go", [v(0), v(1), v(2)]), v(3)]),
            c("token", []),
        ],
        and([
            c("take", [v(0), v(4)]).into(),
            eq(v(1), v(2)),
            eq(v(3), v(2)),
            c("record", [v(3), v(4)]).into(),
        ]),
    );
    let older = t("go", [atom("stop"), v(50), atom("older")]);
    let newer = t("go", [atom("stop"), v(50), atom("newer")]);
    let trigger = t("go", [newer, v(51), older]);
    let q = Query {
        constraints: vec![
            c("take", [v(51), v(52)]),
            c("take", [trigger, v(53)]),
            c("token", []),
            c("token", []),
        ],
        outputs: vec![
            ("winner".into(), Var(50)),
            ("older-output".into(), Var(52)),
            ("trigger-output".into(), Var(53)),
        ],
    };
    let expected = oracle::run(std::slice::from_ref(&rule), &q, 200_000);
    assert_eq!(expected[0].outputs[0].1, atom("older"));
    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
        oracle::same_raw(run(vec![rule.clone()], q.clone(), access), expected.clone());
    }
    let plan = local::Plan::compile(&rule).unwrap();
    for mode in [
        local::DependencyMode::Endpoint,
        local::DependencyMode::Filtered,
        local::DependencyMode::Indexed,
    ] {
        let mut e = plan.start(&q, mode);
        e.settle();
        e.assert_dependency_integrity();
        assert!(!e.body_pending());
        assert_eq!(e.consumed_tokens, vec![0, 1]);
        oracle::same_raw(e.observe().into_iter().collect(), expected.clone());
    }
    // Exposing an ordinary continuation constraint between body phases makes
    // the emitted request compete before the late equation. It changes meaning.
    let split = Rule::simplify(
        "split-body",
        rule.removed.clone(),
        and([
            c("take", [v(0), v(4)]).into(),
            c("resume", [v(1), v(2), v(3), v(4)]).into(),
        ]),
    );
    let resume = Rule::simplify(
        "resume",
        [c("resume", [v(1), v(2), v(3), v(4)])],
        and([
            eq(v(1), v(2)),
            eq(v(3), v(2)),
            c("record", [v(3), v(4)]).into(),
        ]),
    );
    let split_answer = oracle::run(&[split, resume], &q, 200_000);
    assert_eq!(split_answer[0].outputs[0].1, atom("newer"));
    println!("BODY_ORDER complete=older exposed-continuation=newer");
    assert!(!chr_observe::equivalent(
        &expected[0],
        &split_answer[0],
        &mut Default::default()
    ));
}

#[test]
fn prepared_body_ownership_and_forked_queries_are_independent() {
    let rule = Rule::simplify(
        "fresh",
        [c("take", [t("f", [v(0)]), v(1)]), c("token", [])],
        and([
            eq(v(1), t("g", [v(0), v(2)])),
            c("fresh", [v(2), v(2)]).into(),
        ]),
    );
    for mode in [
        local::DependencyMode::Endpoint,
        local::DependencyMode::Filtered,
        local::DependencyMode::Indexed,
    ] {
        let plan = local::Plan::compile(&rule).unwrap();
        let weak = std::sync::Arc::downgrade(&plan);
        let q = Query {
            constraints: vec![c("take", [t("f", [atom("a")]), v(20)]), c("token", [])],
            outputs: vec![("result".into(), Var(20))],
        };
        let mut one = plan.start(&q, mode);
        let mut two = one.clone();
        drop(plan);
        assert!(weak.upgrade().is_some());
        one.settle();
        assert!(!one.body_pending());
        assert!(two.consumed_tokens.is_empty());
        let hidden = two.value();
        two.describe(hidden, "loop", vec![hidden]);
        two.settle();
        assert!(two.observe().is_none());
        assert!(!two.body_pending());
        oracle::same_raw(
            one.observe().into_iter().collect(),
            oracle::run(std::slice::from_ref(&rule), &q, 200_000),
        );
        drop(one);
        assert!(weak.upgrade().is_some());
        drop(two);
        assert!(weak.upgrade().is_none());
    }
}
