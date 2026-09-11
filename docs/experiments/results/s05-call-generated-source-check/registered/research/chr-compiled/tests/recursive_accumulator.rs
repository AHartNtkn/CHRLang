#![cfg(feature = "carrier-contraction")]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_compiled::{Access, Policy, PreparedRuleset};
use chr_syntax::{Query, Rule, Var, atom, c, eq, t, v};

#[test]
fn inferred_recursive_updates_build_results_and_preserve_unknown_tail_wakeups() {
    for name in ["fold", "accumulate"] {
        for constructor in ["f", "wrap"] {
            let rules = vec![
                Rule::simplify(
                    "step",
                    [c(name, [t("s", [v(0)]), v(1), v(2)])],
                    c(name, [v(0), t(constructor, [v(1)]), v(2)]).into(),
                ),
                Rule::simplify("base", [c(name, [atom("z"), v(0), v(1)])], eq(v(1), v(0))),
                Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("z"))),
            ];
            let ordinary = PreparedRuleset::new(rules.clone(), None)
                .unwrap()
                .specialize_inferred();
            let derived = ordinary
                .contract_carriers_checked(&[(name.into(), 3)])
                .unwrap();
            for n in [0usize, 1, 2, 8] {
                for tail_mode in 0..3 {
                    let tail = if tail_mode == 0 { atom("z") } else { v(90) };
                    let input = (0..n).fold(tail, |n, _| t("s", [n]));
                    let mut constraints = vec![c(name, [input, v(91), v(92)])];
                    if tail_mode == 2 {
                        constraints.push(c("bind", [v(90)]));
                    }
                    let q = Query {
                        constraints,
                        outputs: vec![("seed".into(), Var(91)), ("result".into(), Var(92))],
                    };
                    let expected = oracle::run(&rules, &q, 10000);
                    for access in [Access::Scan, Access::Indexed] {
                        for diagnostic in [false, true] {
                            let mut a = ordinary.start(q.clone(), Policy::Global, access).unwrap();
                            let mut b = derived.start(q.clone(), Policy::Global, access).unwrap();
                            if diagnostic {
                                a.enable_trace();
                                a.enable_audit();
                                b.enable_trace();
                                b.enable_audit();
                            }
                            assert!(a.advance(10000).exhausted && b.advance(10000).exhausted);
                            assert_eq!(a.view(), b.view());
                            oracle::same_raw(vec![b.observe().unwrap()], expected.clone());
                            if diagnostic {
                                assert_eq!(a.trace(), b.trace());
                                assert_eq!(a.audit().len(), b.audit().len());
                                for (a, b) in a.audit().iter().zip(b.audit()) {
                                    assert_eq!(
                                        (a.rule, &a.ids, &a.before, &a.after),
                                        (b.rule, &b.ids, &b.before, &b.after)
                                    );
                                }
                            }
                            if chr_compiled::COLLECT_METRICS {
                                assert_eq!(b.stats().carrier_steps, n.saturating_sub(1) as u64);
                                if n >= 2 && !diagnostic {
                                    assert!(
                                        b.stats().specialized_candidates
                                            < a.stats().specialized_candidates
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn execute(p: &PreparedRuleset, q: &Query) -> Vec<chr_syntax::Answer> {
    let mut e = p
        .start_search(q.clone(), Policy::Global, Access::Indexed)
        .unwrap();
    let mut answers = vec![];
    for _ in 0..10000 {
        match e.tick() {
            chr_compiled::SearchEvent::Complete(mut b) => answers.push(b.engine.observe().unwrap()),
            chr_compiled::SearchEvent::Exhausted => return answers,
            _ => (),
        }
    }
    panic!("finite source bound");
}
fn update_source() -> Vec<Rule> {
    vec![
        Rule::simplify(
            "step",
            [c("fold", [t("s", [v(0)]), v(1), v(2)])],
            c("fold", [v(0), t("f", [v(1)]), v(2)]).into(),
        ),
        Rule::simplify("base", [c("fold", [atom("z"), v(0), v(1)])], eq(v(1), v(0))),
    ]
}
#[test]
fn terminal_alternatives_consumption_and_multiple_calls_preserve_complete_answers() {
    use chr_syntax::{and, or};
    let mut rules = update_source();
    rules[1].body = or(
        and(vec![eq(v(1), v(0)), c("claim", [v(0)]).into()]),
        eq(v(1), t("alt", [v(0)])),
    );
    rules.push(Rule::simplify(
        "take",
        [c("claim", [v(0)]), c("token", [])],
        c("won", [v(0)]).into(),
    ));
    let ordinary = PreparedRuleset::new(rules.clone(), None)
        .unwrap()
        .specialize_inferred();
    let derived = ordinary
        .contract_carriers_checked(&[("fold".into(), 3)])
        .unwrap();
    for calls in [1, 2] {
        for fail in [false, true] {
            let mut constraints = vec![c("token", []), c("token", [])];
            let mut outputs = vec![];
            for i in 0..calls {
                let control = (0..(5 - i)).fold(atom("z"), |n, _| t("s", [n]));
                let result = if fail { atom("clash") } else { v(80 + i) };
                constraints.push(c("fold", [control, v(70 + i), result]));
                outputs.push((format!("result{i}"), Var(80 + i)));
            }
            let q = Query {
                constraints,
                outputs,
            };
            let expected = oracle::run(&rules, &q, 10000);
            assert_eq!(expected.len(), if fail { 0 } else { 1 << calls });
            oracle::same_raw(execute(&ordinary, &q), expected.clone());
            oracle::same_raw(execute(&derived, &q), expected);
        }
    }
}
#[test]
fn fresh_step_variables_and_extra_effects_are_rejected() {
    use chr_syntax::and;
    let mut fresh = update_source();
    fresh[0].body = c("fold", [v(0), t("f", [v(9)]), v(2)]).into();
    let mut effects = update_source();
    effects[0].body = and(vec![effects[0].body.clone(), c("claim", []).into()]);
    for rules in [fresh, effects] {
        let p = PreparedRuleset::new(rules, None)
            .unwrap()
            .specialize_inferred();
        assert!(p.contract_carriers_checked(&[("fold".into(), 3)]).is_err());
    }
}
#[test]
fn enabling_trace_during_prefix_inspection_preserves_the_result() {
    if !chr_compiled::COLLECT_METRICS {
        return;
    }
    let rules = update_source();
    let p = PreparedRuleset::new(rules.clone(), None)
        .unwrap()
        .specialize_inferred()
        .contract_carriers_inferred()
        .unwrap();
    let q = Query {
        constraints: vec![c(
            "fold",
            [
                (0..8).fold(atom("z"), |n, _| t("s", [n])),
                atom("seed"),
                v(90),
            ],
        )],
        outputs: vec![("result".into(), Var(90))],
    };
    let mut e = p.start(q.clone(), Policy::Global, Access::Indexed).unwrap();
    for _ in 0..1000 {
        if e.stats().carrier_checks >= 3 {
            break;
        }
        e.step();
    }
    assert_eq!(e.stats().carrier_checks, 3);
    e.enable_trace();
    e.enable_audit();
    assert!(e.advance(10000).exhausted);
    oracle::same_raw(vec![e.observe().unwrap()], oracle::run(&rules, &q, 10000));
}

#[test]
fn nested_updates_preserve_shared_control_tail_aliases() {
    let mut rules = update_source();
    rules[0].body = c(
        "fold",
        [v(0), t("pair", [v(1), t("tail", [v(0), v(0)])]), v(2)],
    )
    .into();
    rules.push(Rule::simplify(
        "bind",
        [c("bind", [v(0)])],
        eq(v(0), atom("z")),
    ));
    let p = PreparedRuleset::new(rules.clone(), None)
        .unwrap()
        .specialize_inferred()
        .contract_carriers_inferred()
        .unwrap();
    for late in [false, true] {
        let mut constraints = vec![c(
            "fold",
            [(0..5).fold(v(90), |n, _| t("s", [n])), v(91), v(92)],
        )];
        if late {
            constraints.push(c("bind", [v(90)]));
        }
        let q = Query {
            constraints,
            outputs: vec![
                ("tail".into(), Var(90)),
                ("seed".into(), Var(91)),
                ("result".into(), Var(92)),
            ],
        };
        oracle::same_raw(execute(&p, &q), oracle::run(&rules, &q, 10000));
    }
}
#[test]
fn updating_recursion_services_finite_siblings_and_survives_cancellation() {
    use chr_syntax::or;
    let mut rules = update_source();
    let input = (0..32).fold(atom("z"), |n, _| t("s", [n]));
    rules.push(Rule::simplify(
        "start",
        [c("start", [v(0)])],
        or(
            c("loop", []).into(),
            c("fold", [input.clone(), atom("seed"), v(0)]).into(),
        ),
    ));
    rules.push(Rule::simplify(
        "loop",
        [c("loop", [])],
        c("loop", []).into(),
    ));
    let p = PreparedRuleset::new(rules.clone(), None)
        .unwrap()
        .specialize_inferred()
        .contract_carriers_inferred()
        .unwrap();
    let q = Query {
        constraints: vec![c("start", [v(90)])],
        outputs: vec![("result".into(), Var(90))],
    };
    let finite = Query {
        constraints: vec![c("fold", [input, atom("seed"), v(90)])],
        outputs: q.outputs.clone(),
    };
    let expected = oracle::run(&rules, &finite, 10000);
    for ticks in [0, 1, 16, 64] {
        let mut abandoned = p
            .start_search(q.clone(), Policy::Global, Access::Indexed)
            .unwrap();
        for _ in 0..ticks {
            abandoned.tick();
        }
        drop(abandoned);
        let mut e = p
            .start_search(q.clone(), Policy::Global, Access::Indexed)
            .unwrap();
        let mut answer = None;
        for _ in 0..4000 {
            if let chr_compiled::SearchEvent::Complete(mut b) = e.tick() {
                if chr_compiled::COLLECT_METRICS {
                    assert_eq!(b.engine.stats().carrier_steps, 31);
                }
                answer = Some(b.engine.observe().unwrap());
                break;
            }
        }
        oracle::same_raw(vec![answer.expect("finite sibling")], expected.clone());
        for _ in 0..32 {
            assert!(!matches!(e.tick(), chr_compiled::SearchEvent::Exhausted));
        }
    }
}

#[test]
fn derived_updates_follow_source_argument_rearrangements_and_constructors() {
    let terms = vec![
        v(0),
        v(1),
        v(2),
        atom("k"),
        t("f", [v(1)]),
        t("pair", [v(0), v(2)]),
    ];
    for accumulator in &terms {
        for result in &terms {
            let mut rules = update_source();
            rules[0].body = c("fold", [v(0), accumulator.clone(), result.clone()]).into();
            let ordinary = PreparedRuleset::new(rules.clone(), None)
                .unwrap()
                .specialize_inferred();
            let derived = ordinary
                .contract_carriers_checked(&[("fold".into(), 3)])
                .unwrap();
            let q = Query {
                constraints: vec![c(
                    "fold",
                    [(0..4).fold(atom("z"), |n, _| t("s", [n])), v(91), v(92)],
                )],
                outputs: vec![("a".into(), Var(91)), ("b".into(), Var(92))],
            };
            let expected = oracle::run(&rules, &q, 10000);
            oracle::same_raw(execute(&ordinary, &q), expected.clone());
            oracle::same_raw(execute(&derived, &q), expected);
        }
    }
}
