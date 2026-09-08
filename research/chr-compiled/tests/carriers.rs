#![cfg(feature = "carrier-contraction")]
#[allow(dead_code)]
mod search_support;
use chr_compiled::{Access, Policy, PreparedRuleset};
use chr_syntax::{Query, Rule, Term, Var, atom, c, t, v};
fn source() -> Vec<Rule> {
    vec![
        Rule::simplify(
            "step",
            [c("carry", [t("s", [v(0)]), v(1)])],
            c("carry", [v(0), v(1)]).into(),
        ),
        Rule::simplify(
            "base",
            [c("carry", [atom("z"), v(1)])],
            c("done", [v(1), v(1)]).into(),
        ),
    ]
}
fn query(n: usize) -> Query {
    let control = (0..n).fold(atom("z"), |x, _| t("s", [x]));
    Query {
        constraints: vec![c("carry", [control, v(9)])],
        outputs: vec![("x".into(), Var(9))],
    }
}
#[test]
fn primary_contracts_but_retains_ordinary_final_ids() {
    let rules = source();
    let ordinary = PreparedRuleset::new(rules, None)
        .unwrap()
        .specialize_inferred();
    let contracted = ordinary.contract_carriers_inferred().unwrap();
    for n in [0, 1, 2, 8, 32] {
        let q = query(n);
        let mut a = ordinary
            .start(q.clone(), Policy::Global, Access::Indexed)
            .unwrap();
        let mut b = contracted
            .start(q, Policy::Global, Access::Indexed)
            .unwrap();
        assert!(a.advance(10000).exhausted);
        assert!(b.advance(10000).exhausted);
        assert_eq!(a.view().store, b.view().store);
        assert_eq!(a.observe(), b.observe());
        if chr_compiled::COLLECT_METRICS {
            assert_eq!(b.stats().carrier_steps, n.saturating_sub(1) as u64);
            assert_eq!(a.stats().applications, b.stats().applications);
        }
        let answer = b.observe().unwrap();
        assert!(matches!(answer.outputs[0].1, Term::Var(_)));
        assert_eq!(answer.residual.len(), 1);
        assert_eq!(b.view().store[0].0, n as u64 + 1);
        assert_eq!(
            answer.residual[0],
            c(
                "done",
                [answer.outputs[0].1.clone(), answer.outputs[0].1.clone()]
            )
        );
    }
}

fn compare(source: Vec<Rule>, q: Query, diagnostic: bool) -> u64 {
    let ordinary = PreparedRuleset::new(source.clone(), None)
        .unwrap()
        .specialize_inferred();
    let prepared = ordinary.contract_carriers_inferred().unwrap();
    let mut left = ordinary
        .start(q.clone(), Policy::Global, Access::Indexed)
        .unwrap();
    let mut right = prepared
        .start(q.clone(), Policy::Global, Access::Indexed)
        .unwrap();
    if diagnostic {
        left.enable_trace();
        right.enable_trace();
        left.enable_audit();
        right.enable_audit();
    }
    let a = left.advance(100000);
    let b = right.advance(100000);
    assert!(a.exhausted && b.exhausted);
    assert_eq!(a.failed, b.failed);
    assert_eq!(left.view(), right.view());
    assert_eq!(left.observe(), right.observe());
    if diagnostic {
        assert_eq!(left.trace(), right.trace());
        assert_eq!(left.audit().len(), right.audit().len());
        for (a, b) in left.audit().iter().zip(right.audit()) {
            assert_eq!(
                (a.rule, &a.ids, &a.before, &a.after),
                (b.rule, &b.ids, &b.before, &b.after)
            );
        }
        let mut replay = search_support::Replay::new(&q, &[]);
        for (rule, ids) in right.trace() {
            replay.fire(*rule, &source[*rule], ids).unwrap();
        }
        assert!(replay.terminal(&source));
        assert!(chr_observe::equivalent(
            &right.observe().unwrap(),
            &replay.answer(),
            &mut Default::default()
        ));
    }
    right.stats().carrier_steps
}
#[test]
fn traced_contraction_expands_exact_source_commits_and_audits() {
    let steps = compare(source(), query(12), true);
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(steps, 11);
    }
}
#[test]
fn renamed_controls_opaque_aliases_and_terminal_equations_keep_order() {
    use chr_syntax::{and, eq};
    for control in 0..3 {
        let mut head = vec![v(71), v(82), v(93)];
        head[control] = t("advance\"", [v(55)]);
        let mut next = head.clone();
        next[control] = v(55);
        let mut base = head.clone();
        base[control] = atom("halt\n");
        let opaque = if control == 0 { 82 } else { 71 };
        let rules = vec![
            Rule::simplify(
                "terminal",
                [c("opaque-p", base)],
                and(vec![
                    eq(v(opaque), atom("bound")),
                    c("done", [v(opaque), v(opaque)]).into(),
                ]),
            ),
            Rule::simplify(
                "decrement",
                [c("opaque-p", head)],
                c("opaque-p", next).into(),
            ),
            Rule::simplify("watch", [c("watch", [atom("bound")])], c("seen", []).into()),
        ];
        let mut args = vec![v(2); 3];
        args[control] = (0..8).fold(atom("halt\n"), |x, _| t("advance\"", [x]));
        let q = Query {
            constraints: vec![c("opaque-p", args), c("watch", [v(2)])],
            outputs: vec![("shared".into(), Var(2))],
        };
        compare(rules.clone(), q.clone(), false);
        compare(rules, q, true);
    }
}
#[test]
fn multiple_carriers_partial_controls_and_later_bindings_stay_ordinary_when_needed() {
    use chr_syntax::eq;
    let mut q = query(5);
    q.constraints.extend(query(1).constraints);
    compare(source(), q, false);
    let mut q = query(5);
    q.constraints.extend(query(1).constraints);
    compare(source(), q, true);
    for tail in [v(3), atom("wrong"), t("s", [v(3)])] {
        let q = Query {
            constraints: vec![c("carry", [t("s", [t("s", [tail])]), v(9)])],
            outputs: vec![("x".into(), Var(9))],
        };
        compare(source(), q, false);
    }
    let mut rules = source();
    rules.push(Rule::simplify(
        "bind",
        [c("bind", [v(0)])],
        eq(v(0), t("s", [t("s", [atom("z")])])),
    ));
    let q = Query {
        constraints: vec![
            c("carry", [t("s", [t("s", [v(3)])]), v(9)]),
            c("bind", [v(3)]),
        ],
        outputs: vec![("x".into(), Var(9))],
    };
    let steps = compare(rules.clone(), q.clone(), false);
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(steps, 1);
    }
    compare(rules, q, true);
}
#[test]
fn certificate_rejects_observers_repeated_control_guards_and_extra_work() {
    use chr_syntax::{Goal, Guard, and};
    let mut cases = vec![];
    let mut r = source();
    r[0].removed[0] = c("carry", [t("s", [v(0)]), v(0)]);
    r[0].body = c("carry", [v(0), v(0)]).into();
    cases.push(r);
    let mut r = source();
    r[0].guards.push(Guard::Equal(v(1), v(1)));
    cases.push(r);
    let mut r = source();
    r[0].body = and(vec![r[0].body.clone(), Goal::True]);
    cases.push(r);
    let mut r = source();
    r.push(Rule::simplify(
        "observer",
        [c("carry", [v(0), v(1)]), c("other", [])],
        Goal::True,
    ));
    cases.push(r);
    let mut r = source();
    r[0].kept.push(c("other", []));
    cases.push(r);
    for rules in cases {
        let p = PreparedRuleset::new(rules, None)
            .unwrap()
            .specialize_inferred();
        assert!(p.contract_carriers_checked(&[("carry".into(), 2)]).is_err());
    }
    let mut terminal = source();
    terminal[1].guards.push(Guard::Equal(v(1), atom("never")));
    let p = PreparedRuleset::new(terminal.clone(), None)
        .unwrap()
        .specialize_inferred();
    assert!(p.contract_carriers_checked(&[("carry".into(), 2)]).is_ok());
    compare(terminal, query(4), true);
}

#[test]
fn unsuccessful_spine_walk_is_not_repeated_along_same_chain() {
    let p = PreparedRuleset::new(source(), None)
        .unwrap()
        .specialize_inferred()
        .contract_carriers_inferred()
        .unwrap();
    let tail = (0..32).fold(v(3), |x, _| t("s", [x]));
    let q = Query {
        constraints: vec![c("carry", [tail, v(9)])],
        outputs: vec![("tail".into(), Var(3))],
    };
    let mut e = p.start(q, Policy::Global, Access::Indexed).unwrap();
    assert!(e.advance(10000).exhausted);
    let answer = e.observe().unwrap();
    assert!(matches!(answer.outputs[0].1, Term::Var(_)));
    assert_eq!(answer.residual.len(), 1);
    assert_eq!(answer.residual[0].args[0], answer.outputs[0].1);
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(e.stats().carrier_checks, 32);
        assert_eq!(e.stats().carrier_steps, 0);
    }
}
#[test]
fn cancellation_during_validation_preserves_prepared_reuse() {
    let p = PreparedRuleset::new(source(), None)
        .unwrap()
        .specialize_inferred()
        .contract_carriers_inferred()
        .unwrap();
    let mut e = p
        .start(query(128), Policy::Global, Access::Indexed)
        .unwrap();
    assert!(!e.advance(20).exhausted);
    assert!(e.observe().is_none());
    if chr_compiled::COLLECT_METRICS {
        assert!(e.stats().carrier_checks > 0);
        assert_eq!(e.stats().carrier_steps, 0);
    }
    drop(e);
    let mut e = p.start(query(5), Policy::Global, Access::Indexed).unwrap();
    assert!(e.advance(10000).exhausted);
    assert_eq!(e.view().store[0].0, 6);
}
#[test]
fn finite_sibling_is_serviced_beside_a_divergent_source_branch() {
    use chr_compiled::SearchEvent;
    use chr_syntax::or;
    let mut rules = source();
    rules.push(Rule::simplify(
        "choose",
        [c("choose", [v(0), v(1)])],
        or(c("carry", [v(0), v(1)]).into(), c("spin", []).into()),
    ));
    rules.push(Rule::simplify(
        "loop",
        [c("spin", [])],
        c("spin", []).into(),
    ));
    let p = PreparedRuleset::new(rules, None)
        .unwrap()
        .specialize_inferred()
        .contract_carriers_inferred()
        .unwrap();
    let mut q = query(64);
    q.constraints[0].name = "choose".into();
    let mut search = p.start_search(q, Policy::Global, Access::Indexed).unwrap();
    let mut answer = None;
    for _ in 0..10000 {
        match search.tick() {
            SearchEvent::Complete(mut branch) => {
                assert_eq!(branch.lineage, vec![false]);
                if chr_compiled::COLLECT_METRICS {
                    assert_eq!(branch.engine.stats().carrier_steps, 63);
                }
                answer = branch.engine.observe();
                break;
            }
            SearchEvent::Exhausted | SearchEvent::Failed(_) => {
                panic!("finite sibling or loop lost")
            }
            _ => (),
        }
    }
    let answer = answer.expect("finite sibling completes");
    assert_eq!(
        answer.residual,
        vec![c(
            "done",
            [answer.outputs[0].1.clone(), answer.outputs[0].1.clone()]
        )]
    );
    for _ in 0..1000 {
        assert!(matches!(search.tick(), SearchEvent::Progress));
    }
}
#[test]
fn repeated_terminal_payload_stays_nonbinding_and_repeated_step_is_not_contracted() {
    let rules = vec![
        Rule::simplify(
            "step",
            [c("p", [t("s", [v(0)]), v(1), v(2)])],
            c("p", [v(0), v(1), v(2)]).into(),
        ),
        Rule::simplify(
            "base",
            [c("p", [atom("z"), v(1), v(1)])],
            c("done", [v(1)]).into(),
        ),
    ];
    let q = Query {
        constraints: vec![c("p", [t("s", [t("s", [atom("z")])]), v(8), v(9)])],
        outputs: vec![("x".into(), Var(8)), ("y".into(), Var(9))],
    };
    compare(rules, q, true);
    let mut rules = source();
    rules[0].removed[0] = c("carry", [t("s", [v(0)]), v(0)]);
    rules[0].body = c("carry", [v(0), v(0)]).into();
    let q = Query {
        constraints: vec![c(
            "carry",
            [t("s", [t("s", [atom("z")])]), t("s", [atom("z")])],
        )],
        outputs: vec![],
    };
    let p = PreparedRuleset::new(rules.clone(), None)
        .unwrap()
        .specialize_inferred()
        .contract_carriers_inferred()
        .unwrap();
    let mut e = p.start(q.clone(), Policy::Global, Access::Indexed).unwrap();
    assert!(e.advance(1000).exhausted);
    assert_eq!(
        e.observe().unwrap().residual,
        vec![c("carry", [t("s", [atom("z")]), t("s", [atom("z")])])]
    );
    compare(rules, q, true);
}

#[test]
fn reselection_cannot_bypass_generic_history_owner() {
    let p = PreparedRuleset::new(source(), None)
        .unwrap()
        .specialize_inferred()
        .contract_carriers_inferred()
        .unwrap()
        .specialize_checked(&[])
        .unwrap();
    let ordinary = PreparedRuleset::new(source(), None).unwrap();
    let mut a = p.start(query(4), Policy::Global, Access::Scan).unwrap();
    let mut b = ordinary
        .start(query(4), Policy::Global, Access::Scan)
        .unwrap();
    assert!(a.advance(10000).exhausted);
    assert!(b.advance(10000).exhausted);
    assert_eq!(a.view(), b.view());
    assert_eq!(a.stats().carrier_steps, 0);
}

#[test]
fn multiple_carrier_rotation_preserves_first_terminal_winner() {
    use chr_syntax::Goal;
    let mut rules = source();
    rules[1].body = c("win", [v(1)]).into();
    rules.insert(
        0,
        Rule::simplify(
            "winner",
            [c("win", [v(0)]), c("token", [])],
            Goal::from(c("winner", [v(0)])),
        ),
    );
    let p = PreparedRuleset::new(rules, None)
        .unwrap()
        .specialize_inferred()
        .contract_carriers_inferred()
        .unwrap();
    let mut long = query(4).constraints.remove(0);
    long.args[1] = atom("long");
    let mut short = query(1).constraints.remove(0);
    short.args[1] = atom("short");
    let mut e = p
        .start(
            Query {
                constraints: vec![long, short, c("token", [])],
                outputs: vec![],
            },
            Policy::Global,
            Access::Indexed,
        )
        .unwrap();
    assert!(e.advance(10000).exhausted);
    assert_eq!(e.stats().carrier_steps, 0);
    assert_eq!(
        e.observe().unwrap().residual,
        vec![c("winner", [atom("short")]), c("win", [atom("long")])]
    );
}
#[test]
fn terminal_failure_and_or_multiplicity_remain_ordinary() {
    use chr_compiled::SearchEvent;
    use chr_syntax::{Goal, or};
    let mut rules = source();
    rules[1].body = or(
        Goal::Fail,
        or(
            c("done", [v(1), v(1)]).into(),
            c("done", [v(1), v(1)]).into(),
        ),
    );
    let prepared = PreparedRuleset::new(rules.clone(), None)
        .unwrap()
        .specialize_inferred()
        .contract_carriers_inferred()
        .unwrap();
    for traced in [false, true] {
        let q = query(6);
        let mut e = prepared
            .start_search(q.clone(), Policy::Global, Access::Indexed)
            .unwrap();
        if traced {
            e.enable_trace();
        }
        let (mut complete, mut failed, mut splits, mut exhausted, mut steps) = (0, 0, 0, false, 0);
        for _ in 0..10000 {
            match e.tick() {
                SearchEvent::Split { work, .. } => {
                    splits += 1;
                    if let Some(work) = work {
                        steps += work.carrier_steps;
                    }
                }
                SearchEvent::Complete(mut b) | SearchEvent::Failed(mut b) => {
                    steps += b.engine.stats().carrier_steps;
                    let is_failed = b.engine.status().failed;
                    if traced {
                        let mut replay = search_support::Replay::new(&q, &b.lineage);
                        let mut error = None;
                        for (r, ids) in b.engine.trace() {
                            assert!(error.is_none());
                            error = replay.fire(*r, &rules[*r], ids).err();
                        }
                        assert_eq!(error.is_some(), is_failed);
                        assert!(replay.choices_consumed());
                        if !is_failed {
                            assert!(replay.terminal(&rules));
                            assert!(chr_observe::equivalent(
                                &b.engine.observe().unwrap(),
                                &replay.answer(),
                                &mut Default::default()
                            ));
                        }
                    }
                    if is_failed {
                        failed += 1;
                        assert!(b.engine.observe().is_none());
                    } else {
                        complete += 1;
                        let a = b.engine.observe().unwrap();
                        assert_eq!(
                            a.residual,
                            vec![c("done", [a.outputs[0].1.clone(), a.outputs[0].1.clone()])]
                        );
                    }
                }
                SearchEvent::Exhausted => {
                    exhausted = true;
                    break;
                }
                SearchEvent::Progress => (),
            }
        }
        assert_eq!((complete, failed, splits, exhausted), (2, 1, 2, true));
        if chr_compiled::COLLECT_METRICS {
            assert_eq!(steps, 5);
        }
    }
}

#[test]
fn singleton_admission_is_rechecked_after_later_carrier_births() {
    use chr_syntax::and;
    let mut rules = source();
    rules.push(Rule::simplify(
        "later",
        [c("trigger", [v(0)])],
        and(vec![
            c("carry", [t("s", [atom("z")]), v(0)]).into(),
            c("carry", [t("s", [t("s", [t("s", [atom("z")])])]), v(0)]).into(),
        ]),
    ));
    let mut q = query(5);
    q.constraints.push(c("trigger", [v(9)]));
    let p = PreparedRuleset::new(rules.clone(), None)
        .unwrap()
        .specialize_inferred()
        .contract_carriers_inferred()
        .unwrap();
    let mut e = p.start(q.clone(), Policy::Global, Access::Indexed).unwrap();
    assert!(e.advance(10000).exhausted);
    assert_eq!(
        e.view().store.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
        vec![7, 14, 15]
    );
    let answer = e.observe().unwrap();
    assert_eq!(
        answer.residual,
        vec![
            c(
                "done",
                [answer.outputs[0].1.clone(), answer.outputs[0].1.clone()]
            );
            3
        ]
    );
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(e.stats().carrier_steps, 4);
    }
    let steps = compare(rules, q, true);
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(steps, 4);
    }
}
