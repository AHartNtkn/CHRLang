#![cfg(feature = "effect-contract")]
#[allow(dead_code)]
mod runtime_support;
use chr_direct_conditional::engine::{EffectAdmission, EffectDeclaration, Event, PreparedRuleset};
use chr_syntax::{Goal, Query, Rule, Var, atom, c, eq, v};
fn run(p: &PreparedRuleset, q: Query) -> (Vec<chr_syntax::Answer>, usize, usize, String) {
    let mut e = p.start(q).unwrap();
    e.enable_trace();
    let mut answers = vec![];
    let mut done = false;
    let mut calls = 0;
    for _ in 0..100000 {
        calls += 1;
        if p.has_immutable_bindings() {
            assert!(e.store().changes().is_empty());
        }
        match e.tick() {
            Event::Answer(a) => answers.push(a),
            Event::Exhausted => {
                done = true;
                break;
            }
            Event::Progress => (),
        }
    }
    assert!(done);
    let (_, entries) = e.dependency_retention();
    (answers, entries, calls, format!("{:?}", e.trace()))
}
#[test]
fn immutable_bindings_remove_dependency_retention_and_drain_work() {
    let rules = vec![Rule::simplify("a", [c("p", [atom("a")])], Goal::True)];
    let q = Query {
        constraints: vec![c("p", [v(100)])],
        outputs: vec![("x".into(), Var(100))],
    };
    let baseline = PreparedRuleset::new(rules.clone()).unwrap();
    let inferred = PreparedRuleset::new(rules.clone())
        .unwrap()
        .with_effect_contract(None, EffectAdmission::Optional)
        .unwrap();
    assert!(inferred.has_immutable_bindings());
    let (a, old, steps, trace) = run(&baseline, q.clone());
    let (b, new, fewer, other) = run(&inferred, q.clone());
    runtime_support::same_raw(a.clone(), runtime_support::run(&rules, &q, 100000));
    runtime_support::same_raw(a, b);
    assert!(old > 0);
    assert_eq!(new, 0);
    assert!(fewer < steps);
    assert_eq!(trace, other);
}
#[test]
fn actual_binding_retains_wakes_and_contradicts_the_declaration() {
    let rules = vec![
        Rule::simplify("a", [c("p", [atom("a")])], c("done", []).into()),
        Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
    ];
    let q = Query {
        constraints: vec![c("p", [v(100)]), c("bind", [v(100)])],
        outputs: vec![],
    };
    assert!(
        PreparedRuleset::new(rules.clone())
            .unwrap()
            .with_effect_contract(
                Some(EffectDeclaration::NoBindings),
                EffectAdmission::Optional
            )
            .is_err()
    );
    assert!(
        PreparedRuleset::new(rules.clone())
            .unwrap()
            .with_effect_contract(None, EffectAdmission::Required)
            .is_err()
    );
    let p = PreparedRuleset::new(rules.clone())
        .unwrap()
        .with_effect_contract(None, EffectAdmission::Optional)
        .unwrap();
    assert!(!p.has_immutable_bindings());
    let (a, retained, _, _) = run(&p, q.clone());
    assert!(retained > 0);
    runtime_support::same_raw(a, runtime_support::run(&rules, &q, 100000));
}
fn accepted(rules: Vec<Rule>, q: Query, expected: Vec<chr_syntax::Answer>) {
    runtime_support::same_raw(runtime_support::run(&rules, &q, 100000), expected.clone());
    let base = PreparedRuleset::new(rules).unwrap();
    let (a, _, _, trace) = run(&base, q.clone());
    runtime_support::same_raw(a, expected.clone());
    for (dec, admission) in [
        (None, EffectAdmission::Optional),
        (
            Some(EffectDeclaration::NoBindings),
            EffectAdmission::Optional,
        ),
        (
            Some(EffectDeclaration::NoBindings),
            EffectAdmission::Required,
        ),
    ] {
        let p = base.clone().with_effect_contract(dec, admission).unwrap();
        assert!(p.has_immutable_bindings());
        let (a, retained, _, other) = run(&p, q.clone());
        assert_eq!(retained, 0);
        assert_eq!(trace, other);
        runtime_support::same_raw(a, expected.clone());
    }
}
#[test]
fn no_binding_effect_does_not_mean_consumption_commutes() {
    let read = Rule {
        name: "read".into(),
        kept: vec![c("p", [])],
        removed: vec![c("q", [])],
        guards: vec![],
        body: c("mark", []).into(),
    };
    let take = Rule::simplify("take", [c("p", [])], Goal::True);
    for (rules, residual) in [
        (vec![read.clone(), take.clone()], vec![c("mark", [])]),
        (vec![take, read], vec![c("q", [])]),
    ] {
        accepted(
            rules,
            Query {
                constraints: vec![c("p", []), c("q", [])],
                outputs: vec![],
            },
            vec![chr_syntax::Answer {
                outputs: vec![],
                residual,
            }],
        );
    }
}
#[test]
fn choices_failure_guards_aliases_and_propagation_preserve_observations() {
    use chr_syntax::{Guard, and, or, t};
    for duplicates in 0..4 {
        for fail in [false, true] {
            let rules = vec![
                Rule::simplify(
                    "choose",
                    [c("start", [v(0)])],
                    or(
                        and([
                            c("p", [v(0), v(0)]).into(),
                            c("gate", [t("box", [v(0)])]).into(),
                        ]),
                        if fail {
                            Goal::Fail
                        } else {
                            c("other", [v(0)]).into()
                        },
                    ),
                ),
                Rule {
                    name: "guard".into(),
                    kept: vec![c("p", [v(0), v(1)])],
                    removed: vec![],
                    guards: vec![Guard::Equal(v(0), v(1))],
                    body: c("seen", [v(0)]).into(),
                },
                Rule::simplify("blocked", [c("gate", [t("box", [atom("a")])])], Goal::True),
            ];
            let mut q = Query {
                constraints: vec![c("start", [v(100)])],
                outputs: vec![("x".into(), Var(100))],
            };
            for _ in 0..duplicates {
                q.constraints.push(c("p", [v(100), v(100)]));
            }
            let mut left = vec![
                c("p", [v(0), v(0)]),
                c("gate", [t("box", [v(0)])]),
                c("seen", [v(0)]),
            ];
            for _ in 0..duplicates {
                left.extend([c("p", [v(0), v(0)]), c("seen", [v(0)])]);
            }
            let mut expected = vec![chr_syntax::Answer {
                outputs: vec![("x".into(), v(0))],
                residual: left,
            }];
            if !fail {
                let mut right = vec![c("other", [v(0)])];
                for _ in 0..duplicates {
                    right.extend([c("p", [v(0), v(0)]), c("seen", [v(0)])]);
                }
                expected.push(chr_syntax::Answer {
                    outputs: vec![("x".into(), v(0))],
                    residual: right,
                });
            }
            accepted(rules, q, expected);
        }
    }
}
#[test]
fn nested_and_unreachable_writers_are_conservatively_rejected() {
    use chr_syntax::{and, or};
    for goal in [
        and([Goal::Fail, eq(v(0), atom("a"))]),
        or(Goal::True, eq(v(0), atom("a"))),
        eq(v(0), v(0)),
    ] {
        let p = PreparedRuleset::new(vec![Rule::simplify("write", [c("unused", [v(0)])], goal)])
            .unwrap();
        assert!(
            !p.clone()
                .with_effect_contract(None, EffectAdmission::Optional)
                .unwrap()
                .has_immutable_bindings()
        );
        assert!(
            p.with_effect_contract(
                Some(EffectDeclaration::NoBindings),
                EffectAdmission::Required
            )
            .is_err()
        );
    }
}
#[cfg(feature = "head-dispatch")]
#[test]
fn the_benefit_remains_with_unary_dispatch_and_serial_accounting() {
    use chr_direct_conditional::engine::HeadAdmission;
    let rules = vec![Rule::simplify("blocked", [c("p", [atom("a")])], Goal::True)];
    for count in [1, 4, 16, 64] {
        let q = Query {
            constraints: (0..count).map(|i| c("p", [v(100 + i)])).collect(),
            outputs: vec![],
        };
        let base =
            PreparedRuleset::with_head_contract(rules.clone(), None, HeadAdmission::Optional)
                .unwrap();
        let p = base
            .clone()
            .with_effect_contract(None, EffectAdmission::Optional)
            .unwrap();
        let (a, old, steps, trace) = run(&base, q.clone());
        let (b, new, fewer, other) = run(&p, q.clone());
        runtime_support::same_raw(a.clone(), runtime_support::run(&rules, &q, 100000));
        runtime_support::same_raw(a, b);
        assert_eq!(old, count as usize);
        assert_eq!(new, 0);
        assert_eq!(steps - fewer, count as usize);
        assert_eq!(trace, other);
        println!(
            "count={count} baseline_calls={steps} certified_calls={fewer} baseline_dependencies={old} certified_dependencies={new}"
        );
    }
}
#[test]
fn finite_sibling_remains_available_beside_continuing_work() {
    use chr_syntax::or;
    let rules = vec![
        Rule::simplify(
            "choice",
            [c("start", [])],
            or(c("loop", []).into(), c("done", []).into()),
        ),
        Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
    ];
    let p = PreparedRuleset::new(rules)
        .unwrap()
        .with_effect_contract(None, EffectAdmission::Optional)
        .unwrap();
    let mut e = p
        .start(Query {
            constraints: vec![c("start", [])],
            outputs: vec![],
        })
        .unwrap();
    for _ in 0..10000 {
        if let Event::Answer(a) = e.tick() {
            assert_eq!(a.residual, vec![c("done", [])]);
            return;
        }
    }
    panic!("finite sibling starved");
}
#[test]
fn newly_posted_linking_constraints_still_discover_matches() {
    use chr_syntax::Guard;
    let rules = vec![
        Rule {
            name: "join".into(),
            kept: vec![c("p", [v(0)])],
            removed: vec![c("q", [v(1)])],
            guards: vec![Guard::Equal(v(0), v(1))],
            body: c("done", [v(0)]).into(),
        },
        Rule::simplify("link", [c("seed", [v(0)])], c("q", [v(0)]).into()),
    ];
    for shared in [false, true] {
        let q = Query {
            constraints: vec![
                c("p", [v(100)]),
                c("seed", [v(if shared { 100 } else { 101 })]),
            ],
            outputs: vec![("x".into(), Var(100))],
        };
        let residual = if shared {
            vec![c("p", [v(0)]), c("done", [v(0)])]
        } else {
            vec![c("p", [v(0)]), c("q", [v(1)])]
        };
        accepted(
            rules.clone(),
            q,
            vec![chr_syntax::Answer {
                outputs: vec![("x".into(), v(0))],
                residual,
            }],
        );
    }
}
