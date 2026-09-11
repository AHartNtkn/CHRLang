#[path = "../examples/support/ordered_hidden.rs"]
#[allow(dead_code)]
mod fixture;
#[path = "../examples/support/completed_caller.rs"]
mod runtime;
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
#[allow(dead_code)]
mod scalar;
use chr_syntax::{Goal, Query, Rule, Var, atom, c, eq, or, t, v};
#[test]
fn complete_keys_preserve_context_multiplicity_fresh_aliases_and_ownership() {
    let mut cases = 0;
    for priority in 0..3 {
        for family in 0..4 {
            let mut rules = fixture::host(eq(v(0), v(1)), vec![v(0), v(1)], priority, 0);
            match family {
                0 => (),
                1 => rules.push(Rule::simplify(
                    "fresh",
                    [c("request", [v(0)])],
                    eq(v(0), t("pair", [v(1), v(1)])),
                )),
                2 => rules.push(Rule::simplify(
                    "duplicate",
                    [c("request", [v(0)])],
                    or(eq(v(0), atom("x")), eq(v(0), atom("x"))),
                )),
                3 => rules.push(Rule::simplify("fail", [c("request", [v(0)])], Goal::Fail)),
                _ => unreachable!(),
            }
            let mut caller = runtime::Caller::new(rules.clone()).unwrap();
            for tokens in 0..3 {
                for name in ["a", "b"] {
                    for extra in [false, true] {
                        for alias in [false, true] {
                            for offset in [10, 1000] {
                                let mut q = Query {
                                    constraints: vec![
                                        c("start", [v(offset), atom(name)]),
                                        c("watch", [v(offset)]),
                                    ],
                                    outputs: vec![("out".into(), Var(offset))],
                                };
                                q.constraints.extend((0..tokens).map(|_| c("token", [])));
                                if extra {
                                    q.constraints.push(c("context", [atom("extra")]));
                                }
                                if alias {
                                    q.outputs.push(("alias".into(), Var(offset)));
                                }
                                if family > 0 {
                                    q.constraints.push(c("request", [v(offset + 1)]));
                                    q.outputs.push(("fresh".into(), Var(offset + 1)));
                                }
                                let direct = caller.run(q.clone(), false, 10000).unwrap();
                                scalar::same_raw(direct.clone(), scalar::run(&rules, &q, 10000));
                                let mut reference =
                                    chr_reference::Search::new(rules.clone(), q.clone()).unwrap();
                                let batch = reference.advance(10000);
                                assert!(batch.exhausted);
                                let mut unique = chr_observe::AnswerSet::default();
                                for a in &direct {
                                    unique.insert(a.clone());
                                }
                                scalar::same_raw(unique.into_answers(), batch.answers);
                                // A hit is independent of the execution bound; a miss must complete.
                                assert_eq!(caller.run(q.clone(), true, 10000).unwrap(), direct);
                                assert_eq!(caller.run(q.clone(), true, 0).unwrap(), direct);
                                let mut changed = caller.run(q.clone(), true, 0).unwrap();
                                if let Some(a) = changed.first_mut() {
                                    a.outputs.clear();
                                    a.residual.clear();
                                }
                                changed.clear();
                                drop(changed);
                                assert_eq!(caller.run(q, true, 0).unwrap(), direct);
                                cases += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(cases, 576);
    eprintln!("complete_caller_contexts={cases}");
}
#[test]
fn bounded_failures_do_not_install_partial_results_and_owners_are_separate() {
    let q = Query {
        constraints: vec![
            c("start", [v(10), atom("a")]),
            c("watch", [v(10)]),
            c("token", []),
        ],
        outputs: vec![("out".into(), Var(10))],
    };
    let mut a =
        runtime::Caller::new(fixture::host(eq(v(0), v(1)), vec![v(0), v(1)], 0, 0)).unwrap();
    let mut b =
        runtime::Caller::new(fixture::host(eq(v(0), v(1)), vec![v(0), v(1)], 2, 0)).unwrap();
    assert!(a.run(q.clone(), true, 1).is_err());
    assert!(a.run(q.clone(), true, 0).is_err());
    let first = a.run(q.clone(), true, 10000).unwrap();
    assert!(b.run(q.clone(), true, 0).is_err());
    let second = b.run(q.clone(), true, 10000).unwrap();
    assert_ne!(first, second);
    assert_eq!(a.run(q.clone(), true, 0).unwrap(), first);
    assert_eq!(b.run(q, true, 0).unwrap(), second);
}
