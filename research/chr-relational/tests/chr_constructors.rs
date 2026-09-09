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
fn take_plan() -> local::Plan {
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
    for width in [1, 8, 64, 256] {
        let mut run = local::Run::default();
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
                    let mut run = local::Run::default();
                    let vars = (0..4).map(|_| run.value()).collect::<Vec<_>>();
                    let input = embed(&mut run, &input, &vars);
                    run.post(&plan, input, vars[0]);
                    run.token();
                    run.settle();
                    match late {
                        0 => (),
                        1 => run.equate(vars[2], vars[3]),
                        2 => run.describe(vars[2], "a", vec![]),
                        _ => run.describe(vars[3], "b", vec![]),
                    }
                    run.settle();
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

#[test]
fn source_plan_rejects_unimplemented_effects_and_ownership() {
    let good = Rule::simplify(
        "capture",
        [c("take", [t("f", [v(0)]), v(1)]), c("token", [])],
        eq(v(1), v(0)),
    );
    assert!(local::Plan::compile(&good).is_ok());
    let mut body = good.clone();
    body.body = chr_syntax::Goal::True;
    let mut kept = good.clone();
    kept.kept.push(c("extra", []));
    let mut resource = good.clone();
    resource.removed[1] = c("token", [v(9)]);
    let mut fresh = good.clone();
    fresh.body = eq(v(1), v(9));
    let mut bound_output = good.clone();
    bound_output.removed[0].args[0] = t("f", [v(1)]);
    for rejected in [body, kept, resource, fresh, bound_output] {
        assert!(local::Plan::compile(&rejected).is_err());
    }
}

#[test]
fn descriptor_waits_follow_both_merge_directions_and_nested_information() {
    let rule = Rule::simplify(
        "nested",
        [c("take", [t("f", [t("g", [v(0)])]), v(1)]), c("token", [])],
        eq(v(1), v(0)),
    );
    let plan = local::Plan::compile(&rule).unwrap();
    for large_input in [false, true] {
        for large_inner in [false, true] {
            let mut run = local::Run::default();
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
            run.post(&plan, input, output);
            run.token();
            run.describe(described, "f", vec![inner]);
            run.settle();
            assert_eq!(run.visited_requests, 1);
            run.equate(input, described);
            run.settle();
            assert_eq!(run.visited_requests, 2);
            assert!(run.consumed_tokens.is_empty());
            run.equate(inner, inner_alias);
            run.settle();
            assert_eq!(run.visited_requests, 2);
            run.describe(leaf, "a", vec![]);
            run.describe(inner_alias, "g", vec![leaf]);
            run.settle();
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
            assert_eq!(run.visited_requests, 3);
        }
    }
}

#[test]
fn unresolved_equality_observes_descendants_and_releases_consumed_dependencies() {
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
                let mut run = local::Run::default();
                let out = run.value();
                let a = run.value();
                let b = run.value();
                let left = graph_wrap(&mut run, depth, a);
                let right = graph_wrap(&mut run, depth, b);
                let input = run.value();
                run.describe(input, "pair", vec![left, right]);
                run.settle();
                run.post(&plan, input, out);
                run.token();
                run.settle();
                assert!(run.consumed_tokens.is_empty());
                // Endpoint aliases can relocate subscriptions before the useful change.
                for _ in 0..4 {
                    let h = run.value();
                    run.equate(if reverse { b } else { a }, h);
                }
                run.settle();
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
                assert_eq!(
                    run.visited_requests, inspections,
                    "resolved equality or mismatch retained dependencies"
                );
                if late == 0 {
                    run.describe(a, "z", vec![]);
                    run.settle();
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

#[test]
fn equality_alias_overhead_is_explicit_and_both_forks_preserve_source_answers() {
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
        let mut run = local::Run::default();
        let common = run.value();
        let others = (0..width).map(|_| run.value()).collect::<Vec<_>>();
        let outputs = (0..width).map(|_| run.value()).collect::<Vec<_>>();
        for (&other, &output) in others.iter().zip(&outputs) {
            let pair = run.value();
            run.describe(pair, "pair", vec![common, other]);
            run.settle();
            run.post(&plan, pair, output);
            run.token();
        }
        for _ in 0..width {
            let h = run.value();
            run.equate(common, h);
        }
        run.settle();
        assert_eq!(run.visited_requests, width + width * width);
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
        assert_eq!(run.consumed_tokens, (0..width).collect::<Vec<_>>());
        assert_eq!(run.subscriptions(), 0);
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
