//! Semantic probes for selecting a recursive-lowering boundary, not a compiler.
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_syntax::{Answer, Query, Rule, Var, atom, c, eq, t, v};

fn chain(depth: usize, tail: chr_syntax::Term) -> chr_syntax::Term {
    (0..depth).fold(tail, |n, _| t("s", [n]))
}
fn run(rules: &[Rule], query: &Query) -> Vec<Answer> {
    let expected = oracle::run(rules, query, 10000);
    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
        let mut engine = chr_compiled::PreparedRuleset::new(rules.to_vec(), None)
            .unwrap()
            .start_search(query.clone(), chr_compiled::Policy::Global, access)
            .unwrap();
        let mut answers = vec![];
        let mut exhausted = false;
        for _ in 0..10000 {
            match engine.tick() {
                chr_compiled::SearchEvent::Complete(mut b) => {
                    answers.push(b.engine.observe().unwrap())
                }
                chr_compiled::SearchEvent::Exhausted => {
                    exhausted = true;
                    break;
                }
                _ => (),
            }
        }
        assert!(exhausted);
        oracle::same_raw(answers, expected.clone());
    }
    expected
}

#[test]
fn ground_recursive_completion_effects_can_change_the_resource_winner() {
    for name in ["walk", "traverse"] {
        let rules = vec![
            Rule::simplify(
                "base",
                [c(name, [atom("z"), v(0)])],
                c("claim", [v(0)]).into(),
            ),
            Rule::simplify(
                "step",
                [c(name, [t("s", [v(0)]), v(1)])],
                c(name, [v(0), v(1)]).into(),
            ),
            Rule::simplify(
                "take",
                [c("claim", [v(0)]), c("token", [])],
                c("won", [v(0)]).into(),
            ),
        ];
        let input = |a, b| Query {
            constraints: vec![
                c(name, [chain(a, atom("z")), atom("a")]),
                c(name, [chain(b, atom("z")), atom("b")]),
                c("token", []),
            ],
            outputs: vec![],
        };
        let original = run(&rules, &input(2, 1));
        let contracted = run(&rules, &input(0, 0));
        assert_eq!(original.len(), 1);
        assert_eq!(contracted.len(), 1);
        assert!(original[0].residual.contains(&c("won", [atom("b")])));
        assert!(original[0].residual.contains(&c("claim", [atom("a")])));
        assert!(contracted[0].residual.contains(&c("won", [atom("a")])));
        assert!(contracted[0].residual.contains(&c("claim", [atom("b")])));
    }
}

#[test]
fn pure_recursive_results_preserve_the_bounded_consuming_observations() {
    let rules = vec![
        Rule::simplify("base", [c("walk", [atom("z"), v(0), v(1)])], eq(v(1), v(0))),
        Rule::simplify(
            "step",
            [c("walk", [t("s", [v(0)]), v(1), v(2)])],
            c("walk", [v(0), v(1), v(2)]).into(),
        ),
        Rule::simplify(
            "a",
            [c("ready", [atom("a")]), c("token", [])],
            c("won", [atom("a")]).into(),
        ),
        Rule::simplify(
            "b",
            [c("ready", [atom("b")]), c("token", [])],
            c("won", [atom("b")]).into(),
        ),
    ];
    for a in 0..5 {
        for b in 0..5 {
            for reverse in [false, true] {
                let query = |a, b| {
                    let mut constraints = vec![
                        c("walk", [chain(a, atom("z")), atom("a"), v(70)]),
                        c("walk", [chain(b, atom("z")), atom("b"), v(71)]),
                        c("ready", [v(70)]),
                        c("ready", [v(71)]),
                        c("token", []),
                    ];
                    if reverse {
                        constraints.reverse();
                    }
                    Query {
                        constraints,
                        outputs: vec![("x".into(), Var(70)), ("y".into(), Var(71))],
                    }
                };
                let original = run(&rules, &query(a, b));
                assert!(original[0].residual.contains(&c("won", [atom("a")])));
                oracle::same_raw(run(&rules, &query(0, 0)), original);
            }
        }
    }
}

#[test]
fn an_unknown_recursive_tail_is_suspended_and_later_binding_resumes_it() {
    let rules = vec![
        Rule::simplify("base", [c("walk", [atom("z"), v(0)])], eq(v(0), atom("a"))),
        Rule::simplify(
            "step",
            [c("walk", [t("s", [v(0)]), v(1)])],
            c("walk", [v(0), v(1)]).into(),
        ),
        Rule::simplify(
            "take",
            [c("ready", [atom("a")]), c("token", [])],
            c("won", []).into(),
        ),
        Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("z"))),
    ];
    for depth in 0..5 {
        for bind in [false, true] {
            let mut constraints = vec![
                c("walk", [chain(depth, v(90)), v(91)]),
                c("ready", [v(91)]),
                c("token", []),
            ];
            if bind {
                constraints.push(c("bind", [v(90)]));
            }
            let q = Query {
                constraints,
                outputs: vec![("input".into(), Var(90)), ("result".into(), Var(91))],
            };
            let answers = run(&rules, &q);
            assert_eq!(answers.len(), 1);
            if bind {
                assert_eq!(
                    answers[0].outputs,
                    vec![("input".into(), atom("z")), ("result".into(), atom("a"))]
                );
                assert_eq!(answers[0].residual, vec![c("won", [])]);
            } else {
                let x = answers[0].outputs[0].1.clone();
                let y = answers[0].outputs[1].1.clone();
                assert!(
                    matches!(x, chr_syntax::Term::Var(_))
                        && matches!(y, chr_syntax::Term::Var(_))
                        && x != y
                );
                assert_eq!(answers[0].residual.len(), 3);
                assert!(answers[0].residual.contains(&c("walk", [x, y.clone()])));
                assert!(answers[0].residual.contains(&c("ready", [y])));
                assert!(answers[0].residual.contains(&c("token", [])));
            }
        }
    }
}

#[test]
fn hoisting_a_late_recursive_input_binding_changes_pure_result_consumption() {
    let rules = vec![
        Rule::simplify("base", [c("walk", [atom("z"), v(0), v(1)])], eq(v(1), v(0))),
        Rule::simplify(
            "step",
            [c("walk", [t("s", [v(0)]), v(1), v(2)])],
            c("walk", [v(0), v(1), v(2)]).into(),
        ),
        Rule::simplify(
            "a",
            [c("ready", [atom("a")]), c("token", [])],
            c("won", [atom("a")]).into(),
        ),
        Rule::simplify(
            "b",
            [c("ready", [atom("b")]), c("token", [])],
            c("won", [atom("b")]).into(),
        ),
        Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("z"))),
    ];
    let query = |tail| Query {
        constraints: vec![
            c("walk", [chain(1, tail), atom("a"), v(70)]),
            c("walk", [atom("z"), atom("b"), v(71)]),
            c("ready", [v(70)]),
            c("ready", [v(71)]),
            c("token", []),
            c("bind", [v(90)]),
        ],
        outputs: vec![("x".into(), Var(70)), ("y".into(), Var(71))],
    };
    let original = run(&rules, &query(v(90)));
    let premature = run(&rules, &query(atom("z")));
    assert_eq!(original[0].outputs, premature[0].outputs);
    assert!(original[0].residual.contains(&c("won", [atom("b")])));
    assert!(premature[0].residual.contains(&c("won", [atom("a")])));
    assert!(original[0].residual.contains(&c("ready", [atom("a")])));
    assert!(premature[0].residual.contains(&c("ready", [atom("b")])));
}
