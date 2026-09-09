//! Open parents retain binding dependencies around immutable closed children.
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
use chr_syntax::{Answer, Query, Rule, and, atom, c, eq, or, t, v};
fn unary(n: usize) -> chr_syntax::Term {
    (0..n).fold(atom("z"), |tail, _| t("s", [tail]))
}
#[test]
fn open_parent_follows_new_alias_targets_across_forks_and_failed_equations() {
    for policy in [Policy::Global, Policy::Active] {
        for access in [Access::Scan, Access::Indexed] {
            for specialized in [false, true] {
                if specialized && policy == Policy::Active {
                    continue;
                }
                let closed = unary(16);
                let mut rules = ["a", "b"]
                    .into_iter()
                    .map(|name| {
                        Rule::simplify(
                            name,
                            [c(
                                "p",
                                [t("g", [t("f", [atom(name)]), closed.clone()]), v(0)],
                            )],
                            c("done", [atom(name), v(0), v(0)]).into(),
                        )
                    })
                    .collect::<Vec<_>>();
                rules.push(Rule::simplify(
                    "wrap",
                    [c("wrap", [v(0), v(1)])],
                    eq(v(0), t("f", [v(1)])),
                ));
                // One branch first binds a variable during a multi-pair equation,
                // then clashes; its sibling bindings and index repair must survive.
                let clash = eq(
                    t("pair", [atom("a"), v(0)]),
                    t("pair", [atom("b"), atom("a")]),
                );
                rules.push(Rule::simplify(
                    "choose",
                    [c("choose", [v(0)])],
                    or(clash, or(eq(v(0), atom("a")), eq(v(0), atom("b")))),
                ));
                let query = Query {
                    constraints: vec![
                        c("p", [t("g", [v(0), closed]), v(2)]),
                        c("wrap", [v(0), v(1)]),
                        c("choose", [v(1)]),
                    ],
                    outputs: vec![],
                };
                let prepared = PreparedRuleset::new(rules, None).unwrap();
                let prepared = if specialized {
                    prepared.specialize_inferred()
                } else {
                    prepared
                };
                let mut search = prepared.start_search(query, policy, access).unwrap();
                let mut answers = Vec::new();
                let mut failed = 0;
                let mut exhausted = false;
                for _ in 0..100000 {
                    match search.tick() {
                        SearchEvent::Complete(mut b) => answers.push(b.engine.observe().unwrap()),
                        SearchEvent::Failed(_) => failed += 1,
                        SearchEvent::Exhausted => {
                            exhausted = true;
                            break;
                        }
                        SearchEvent::Progress | SearchEvent::Split { .. } => (),
                    }
                }
                assert!(exhausted);
                assert_eq!(failed, 1);
                assert_eq!(answers.len(), 2);
                for name in ["a", "b"] {
                    let expected = Answer {
                        outputs: vec![],
                        residual: vec![c("done", [atom(name), v(9), v(9)])],
                    };
                    assert!(answers.iter().any(|a| chr_observe::equivalent(
                        a,
                        &expected,
                        &mut Default::default()
                    )));
                }
            }
        }
    }
}
#[test]
fn syntactically_open_bound_variable_still_reactivates_after_alias_target_changes() {
    let rules = vec![
        Rule::simplify("hit", [c("p", [t("f", [atom("a")])])], c("done", []).into()),
        Rule::simplify(
            "bind",
            [c("go", [v(0), v(1)])],
            and(vec![eq(v(0), t("f", [v(1)])), eq(v(1), atom("a"))]),
        ),
    ];
    for policy in [Policy::Global, Policy::Active] {
        let mut e = PreparedRuleset::new(rules.clone(), None)
            .unwrap()
            .start_search(
                Query {
                    constraints: vec![c("p", [v(0)]), c("go", [v(0), v(1)])],
                    outputs: vec![],
                },
                policy,
                Access::Indexed,
            )
            .unwrap();
        let mut seen = 0;
        let mut exhausted = false;
        for _ in 0..10000 {
            match e.tick() {
                SearchEvent::Complete(mut b) => {
                    assert_eq!(
                        b.engine.observe().unwrap(),
                        Answer {
                            outputs: vec![],
                            residual: vec![c("done", [])]
                        }
                    );
                    seen += 1;
                }
                SearchEvent::Exhausted => {
                    assert_eq!(seen, 1);
                    exhausted = true;
                    break;
                }
                SearchEvent::Progress => (),
                _ => panic!("unexpected source choice or failure"),
            }
        }
        assert_eq!(seen, 1);
        assert!(exhausted);
    }
}
