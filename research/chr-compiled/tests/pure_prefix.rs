#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_compiled::pure_prefix::Program;
use chr_syntax::{Query, Rule, Var, atom, c, eq, or, t, v};
#[test]
fn source_derived_prefix_preserves_consumption_choices_and_fresh_aliases() {
    for offset in [0, 70] {
        for name in ["paint", "decorate"] {
            for color in ["red", "blue"] {
                let v = |n| v(n + offset);
                let rules = vec![
                    Rule::simplify(
                        name,
                        [c(name, [v(0), v(1)])],
                        c("inner", [v(0), v(1)]).into(),
                    ),
                    Rule::simplify(
                        "inner",
                        [c("inner", [v(0), v(1)])],
                        or(
                            eq(v(1), t(color, [v(0), v(2), v(2)])),
                            eq(v(1), t(color, [v(0), v(2), v(2)])),
                        ),
                    ),
                    Rule::simplify(
                        "use",
                        [c("ready", [v(0)]), c("token", [v(0)])],
                        c("done", [v(0)]).into(),
                    ),
                ];
                let query = Query {
                    constraints: vec![
                        c(name, [atom("key"), v(10)]),
                        c("ready", [atom("key")]),
                        c("token", [atom("key")]),
                    ],
                    outputs: vec![("result".into(), Var(10 + offset))],
                };
                let program = Program::new(&rules).unwrap();
                assert_eq!(program.eliminated_predicates().len(), 2);
                let (lowered, q) = program.lower(&query).unwrap();
                oracle::same_raw(
                    oracle::run(&lowered, &q, 10000),
                    oracle::run(&rules, &query, 10000),
                );
                for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                    oracle::same_raw(
                        compiled(&lowered, &q, access),
                        oracle::run(&rules, &query, 10000),
                    );
                    oracle::same_raw(
                        compiled(&rules, &query, access),
                        oracle::run(&rules, &query, 10000),
                    );
                }
            }
        }
    }
}

fn compiled(
    rules: &[Rule],
    query: &Query,
    access: chr_compiled::Access,
) -> Vec<chr_syntax::Answer> {
    let mut e = chr_compiled::PreparedRuleset::new(rules.to_vec(), None)
        .unwrap()
        .start_search(query.clone(), chr_compiled::Policy::Global, access)
        .unwrap();
    let mut answers = vec![];
    for _ in 0..10000 {
        match e.tick() {
            chr_compiled::SearchEvent::Complete(mut b) => answers.push(b.engine.observe().unwrap()),
            chr_compiled::SearchEvent::Exhausted => return answers,
            _ => (),
        }
    }
    panic!("finite compiled bound");
}
#[test]
fn resource_priority_is_a_required_condition() {
    let rules = vec![
        Rule::simplify(
            "high",
            [c("p", [atom("a")]), c("token", [])],
            c("high", []).into(),
        ),
        Rule::simplify("low", [c("p", [v(0)]), c("token", [])], c("low", []).into()),
        Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
    ];
    let q = Query {
        constraints: vec![c("p", [v(10)]), c("token", []), c("bind", [v(10)])],
        outputs: vec![("x".into(), Var(10))],
    };
    assert!(Program::new(&rules).is_err());
    let original = oracle::run(&rules, &q, 10000);
    let reordered = oracle::run(
        &[rules[2].clone(), rules[0].clone(), rules[1].clone()],
        &q,
        10000,
    );
    assert_eq!(original[0].residual, vec![c("low", [])]);
    assert_eq!(reordered[0].residual, vec![c("high", [])]);
}
#[test]
fn observers_late_heads_and_recursive_prefixes_are_not_eliminated() {
    let pure = Rule::simplify("pure", [c("pure", [v(0)])], eq(v(0), atom("a")));
    let observer = Rule::simplify(
        "observe",
        [c("pure", [v(0)]), c("watch", [])],
        c("seen", []).into(),
    );
    assert!(Program::new(&[pure, observer]).is_err());
    assert!(
        Program::new(&[Rule::simplify(
            "late",
            [c("late", [atom("a"), v(0)])],
            eq(v(0), atom("yes"))
        )])
        .is_err()
    );
    assert!(
        Program::new(&[Rule::simplify(
            "cycle",
            [c("cycle", [v(0)])],
            c("cycle", [v(0)]).into()
        )])
        .is_err()
    );
}
#[test]
fn body_calls_and_finite_sibling_survive_ordinary_recursion() {
    let rules = vec![
        Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(c("loop", []).into(), c("bind", [v(0)]).into()),
        ),
        Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
    ];
    let q = Query {
        constraints: vec![c("start", [v(10)])],
        outputs: vec![("x".into(), Var(10))],
    };
    let p = Program::new(&rules).unwrap();
    assert_eq!(p.eliminated_predicates(), vec![("bind".into(), 1)]);
    let (lowered, q) = p.lower(&q).unwrap();
    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
        let mut e = chr_compiled::PreparedRuleset::new(lowered.clone(), None)
            .unwrap()
            .start_search(q.clone(), chr_compiled::Policy::Global, access)
            .unwrap();
        let mut answer = None;
        for _ in 0..500 {
            if let chr_compiled::SearchEvent::Complete(mut b) = e.tick() {
                answer = Some(b.engine.observe().unwrap());
                break;
            }
        }
        assert_eq!(
            answer,
            Some(chr_syntax::Answer {
                outputs: vec![("x".into(), atom("a"))],
                residual: vec![]
            })
        );
        for _ in 0..32 {
            assert!(!matches!(e.tick(), chr_compiled::SearchEvent::Exhausted));
        }
    }
}

#[test]
fn independent_calls_can_bind_competing_resource_consumers() {
    let rules = vec![
        Rule::simplify(
            "pick",
            [c("pick", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify(
            "take",
            [c("ready", [v(0)]), c("token", [v(0)])],
            c("done", [v(0)]).into(),
        ),
    ];
    let p = Program::new(&rules).unwrap();
    for alias in [false, true] {
        for reverse in [false, true] {
            let y = if alias { v(10) } else { v(11) };
            let mut constraints = vec![
                c("ready", [v(10)]),
                c("ready", [y.clone()]),
                c("pick", [v(10)]),
                c("pick", [y]),
                c("token", [atom("a")]),
            ];
            if reverse {
                constraints.reverse();
            }
            let q = Query {
                constraints,
                outputs: vec![("x".into(), Var(10)), ("y".into(), Var(11))],
            };
            let expected = oracle::run(&rules, &q, 10000);
            assert_eq!(expected.len(), if alias { 2 } else { 4 });
            let (lowered, input) = p.lower(&q).unwrap();
            oracle::same_raw(oracle::run(&lowered, &input, 10000), expected.clone());
            for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                oracle::same_raw(compiled(&lowered, &input, access), expected.clone());
            }
        }
    }
}
#[test]
fn fresh_variables_from_distinct_calls_remain_distinct() {
    let rules = vec![Rule::simplify(
        "fresh",
        [c("fresh", [v(0)])],
        eq(v(0), t("pair", [v(1), v(1)])),
    )];
    let q = Query {
        constraints: vec![c("fresh", [v(10)]), c("fresh", [v(11)])],
        outputs: vec![("x".into(), Var(10)), ("y".into(), Var(11))],
    };
    let (p, input) = Program::new(&rules).unwrap().lower(&q).unwrap();
    let expected = oracle::run(&rules, &q, 10000);
    assert_ne!(expected[0].outputs[0].1, expected[0].outputs[1].1);
    oracle::same_raw(oracle::run(&p, &input, 10000), expected.clone());
    oracle::same_raw(compiled(&p, &input, chr_compiled::Access::Scan), expected);
}

#[test]
fn emitted_program_eliminates_private_applications_in_actual_execution() {
    let rules = vec![
        Rule::simplify("p", [c("p", [v(0)])], c("q", [v(0)]).into()),
        Rule::simplify("q", [c("q", [v(0)])], c("r", [v(0)]).into()),
        Rule::simplify("r", [c("r", [v(0)])], eq(v(0), atom("a"))),
        Rule::simplify(
            "use",
            [c("ready", [v(0)]), c("token", [v(0)])],
            c("done", [v(0)]).into(),
        ),
    ];
    let query = Query {
        constraints: vec![
            c("p", [v(10)]),
            c("ready", [v(10)]),
            c("token", [atom("a")]),
        ],
        outputs: vec![("x".into(), Var(10))],
    };
    let (lowered, input) = Program::new(&rules).unwrap().lower(&query).unwrap();
    for (source, q, wanted) in [(&rules, &query, 4), (&lowered, &input, 2)] {
        let mut e = chr_compiled::PreparedRuleset::new(source.clone(), None)
            .unwrap()
            .start_search(
                q.clone(),
                chr_compiled::Policy::Global,
                chr_compiled::Access::Scan,
            )
            .unwrap();
        e.enable_trace();
        let mut checked = false;
        for _ in 0..1000 {
            if let chr_compiled::SearchEvent::Complete(mut b) = e.tick() {
                assert_eq!(b.engine.trace().len(), wanted);
                oracle::same_raw(
                    vec![b.engine.observe().unwrap()],
                    oracle::run(&rules, &query, 10000),
                );
                checked = true;
                break;
            }
        }
        assert!(checked);
    }
}

#[test]
fn prefix_selection_agrees_with_bounded_dependency_graphs() {
    // Each rule is an equation, one of three calls, or a foreign residual.
    for encoding in 0..125 {
        let mut n = encoding;
        let mut edges = vec![];
        for _ in 0..3 {
            edges.push(n % 5);
            n /= 5;
        }
        fn valid(i: usize, k: usize, edges: &[usize], path: &mut Vec<usize>) -> bool {
            if i >= k || path.contains(&i) {
                return false;
            }
            path.push(i);
            let ok = match edges[i] {
                0 => true,
                1..=3 => valid(edges[i] - 1, k, edges, path),
                _ => false,
            };
            path.pop();
            ok
        }
        let expected = (1..=3)
            .filter(|k| (0..*k).all(|i| valid(i, *k, &edges, &mut vec![])))
            .max();
        let rules = (0..3)
            .map(|i| {
                Rule::simplify(
                    &format!("p{i}"),
                    [c(&format!("p{i}"), [v(0)])],
                    match edges[i] {
                        0 => eq(v(0), atom("a")),
                        1..=3 => c(&format!("p{}", edges[i] - 1), [v(0)]).into(),
                        _ => c("external", [v(0)]).into(),
                    },
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            Program::new(&rules)
                .ok()
                .map(|p| p.eliminated_predicates().len()),
            expected,
            "graph {edges:?}"
        );
    }
}
