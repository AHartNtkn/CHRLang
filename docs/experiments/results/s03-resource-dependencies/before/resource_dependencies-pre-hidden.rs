#[allow(dead_code)]
mod runtime_support;
use chr_direct_choice::demand::{Event, Prepared, Reuse};
use chr_syntax::{Answer, Goal, Query, Rule, Var, atom, c, eq, or, t, v};
fn check(rules: Vec<Rule>, q: Query) -> Vec<Answer> {
    let expected = runtime_support::run(&rules, &q, 200_000);
    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
        let p = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
        let mut s = p
            .start_search(q.clone(), chr_compiled::Policy::Global, access)
            .unwrap();
        let mut a = vec![];
        let mut done = false;
        for _ in 0..200_000 {
            match s.tick() {
                chr_compiled::SearchEvent::Complete(mut b) => a.push(b.engine.observe().unwrap()),
                chr_compiled::SearchEvent::Exhausted => {
                    done = true;
                    break;
                }
                _ => (),
            }
        }
        assert!(done);
        runtime_support::same_raw(a, expected.clone());
    }
    for reuse in [
        Reuse::CurrentContext,
        Reuse::StaticBirth,
        Reuse::MatchDependencies,
    ] {
        let p = Prepared::with_reuse(rules.clone(), reuse).unwrap();
        let mut s = p.start(q.clone()).unwrap();
        let mut a = vec![];
        let mut done = false;
        for _ in 0..200_000 {
            match s.tick() {
                Event::Answer(x) => a.push(x),
                Event::Exhausted => {
                    done = true;
                    break;
                }
                _ => (),
            }
        }
        assert!(done);
        runtime_support::same_raw(a, expected.clone());
    }
    expected
}
#[test]
fn delayed_resource_keys_match_complete_source_answers() {
    let mut count = 0;
    for structured in [false, true] {
        for choice in [false, true] {
            for resources in 0..3 {
                for reverse in [false, true] {
                    for fail in [false, true] {
                        let value = if structured {
                            t("box", [atom("a")])
                        } else {
                            atom("a")
                        };
                        let body = if fail {
                            Goal::Fail
                        } else if choice {
                            or(eq(v(0), value.clone()), eq(v(0), value.clone()))
                        } else {
                            eq(v(0), value.clone())
                        };
                        let rules = vec![
                            Rule::simplify("make", [c("make", [v(0)])], body),
                            Rule::simplify(
                                "take",
                                [c("take", [v(0)]), c("token", [value])],
                                eq(v(0), atom("done")),
                            ),
                        ];
                        let mut constraints = vec![c("make", [v(10)]), c("take", [v(11)])];
                        constraints.extend((0..resources).map(|_| c("token", [v(10)])));
                        if reverse {
                            constraints.reverse();
                        }
                        check(
                            rules,
                            Query {
                                constraints,
                                outputs: vec![("key".into(), Var(10)), ("out".into(), Var(11))],
                            },
                        );
                        count += 1;
                    }
                }
            }
        }
    }
    assert_eq!(count, 48);
    println!("resource_dependency_cases={count}");
}
#[test]
fn resource_mediated_alias_and_constructor_cycles_have_finite_tree_meaning() {
    for constructor in [false, true] {
        let rules = vec![
            Rule::simplify(
                "f",
                [c("f", [v(0)]), c("r", [v(1)])],
                eq(v(0), if constructor { t("box", [v(1)]) } else { v(1) }),
            ),
            Rule::simplify("g", [c("g", [v(0)]), c("s", [v(1)])], eq(v(0), v(1))),
        ];
        let q = Query {
            constraints: vec![
                c("f", [v(10)]),
                c("g", [v(11)]),
                c("r", [v(11)]),
                c("s", [v(10)]),
            ],
            outputs: vec![("x".into(), Var(10)), ("y".into(), Var(11))],
        };
        let a = check(rules, q);
        if constructor {
            assert!(a.is_empty())
        } else {
            assert_eq!(a.len(), 1);
            assert_eq!(a[0].outputs[0].1, a[0].outputs[1].1);
        }
    }
}
#[test]
fn recursive_waits_and_nested_claims_preserve_resources() {
    let waiting = vec![
        Rule::simplify(
            "f",
            [c("f", [v(0)]), c("r", [atom("ready")])],
            eq(v(0), atom("ready")),
        ),
        Rule::simplify(
            "g",
            [c("g", [v(0)]), c("s", [atom("ready")])],
            eq(v(0), atom("ready")),
        ),
    ];
    let q = Query {
        constraints: vec![
            c("f", [v(10)]),
            c("g", [v(11)]),
            c("r", [v(11)]),
            c("s", [v(10)]),
        ],
        outputs: vec![("x".into(), Var(10)), ("y".into(), Var(11))],
    };
    let a = check(waiting, q);
    assert_eq!(a.len(), 1);
    assert_ne!(a[0].outputs[0].1, a[0].outputs[1].1);
    assert_eq!(a[0].residual.len(), 4);
    let rules = vec![
        Rule::simplify(
            "f",
            [c("f", [v(0)]), c("token", []), c("key", [atom("ready")])],
            eq(v(0), atom("fired")),
        ),
        Rule::simplify(
            "g",
            [c("g", [v(0)]), c("token", [])],
            eq(v(0), atom("ready")),
        ),
    ];
    let q = Query {
        constraints: vec![
            c("f", [v(10)]),
            c("key", [v(11)]),
            c("g", [v(11)]),
            c("token", []),
        ],
        outputs: vec![("f".into(), Var(10)), ("g".into(), Var(11))],
    };
    let a = check(rules, q);
    assert_eq!(a.len(), 1);
    assert!(matches!(a[0].outputs[0].1, chr_syntax::Term::Var(_)));
    assert_eq!(a[0].outputs[1].1, atom("ready"));
    let rules = vec![
        Rule::simplify(
            "make",
            [c("make", [v(0)])],
            or(eq(v(0), atom("ready")), Goal::Fail),
        ),
        Rule::simplify(
            "take",
            [c("take", [v(0)]), c("token", [atom("ready")])],
            eq(v(0), atom("done")),
        ),
    ];
    let a = check(
        rules,
        Query {
            constraints: vec![c("take", [v(10)]), c("token", [v(11)]), c("make", [v(11)])],
            outputs: vec![("out".into(), Var(10))],
        },
    );
    assert_eq!(a.len(), 1);
    assert_eq!(a[0].outputs[0].1, atom("done"));
}
