#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_reuse::calls::{Fresh, Table};
use chr_syntax::{and, atom, c, eq, or, t, v, Query, Rule, Term, Var};
fn rules() -> Vec<Rule> {
    vec![
        Rule::simplify(
            "step",
            [c("work", [t("s", [v(0)]), v(1), v(2)])],
            c("work", [v(0), v(1), v(2)]).into(),
        ),
        Rule::simplify(
            "finish",
            [c("work", [atom("z"), v(0), v(1)])],
            or(
                eq(v(1), t("pair", [v(0), v(99), v(99)])),
                eq(v(1), t("pair", [v(0), v(99), v(99)])),
            ),
        ),
    ]
}
fn depth(n: usize) -> Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
fn replay(bindings: Vec<(Var, Term)>, mut caller: Query) -> Vec<chr_syntax::Answer> {
    let rule = Rule::simplify("replay", [c("bind", [v(0), v(1)])], eq(v(0), v(1)));
    caller.constraints.retain(|c| c.name != "work");
    caller.constraints.splice(
        0..0,
        bindings
            .into_iter()
            .map(|(var, value)| c("bind", [Term::Var(var), value])),
    );
    oracle::run(&[rule], &caller, 200_000)
}
#[test]
fn distinct_callers_reuse_work_and_preserve_fresh_aliases() {
    for memo in [false, true] {
        let mut table = Table::new(rules(), memo).unwrap();
        for tag in ["a", "b"] {
            let call = c("work", [depth(8), v(7), v(8)]);
            let caller = Query {
                constraints: vec![c("caller", [atom(tag), v(1000)]), call.clone()],
                outputs: vec![
                    ("input".into(), Var(7)),
                    ("result".into(), Var(8)),
                    ("unrelated".into(), Var(1000)),
                ],
            };
            let expected = oracle::run(&rules(), &caller, 200_000);
            let mut fresh = Fresh::for_query(&caller);
            let result = table.expand(&call, &mut fresh, 200_000).unwrap();
            assert_eq!(result.len(), 2);
            let actual = result
                .into_iter()
                .flat_map(|b| replay(b, caller.clone()))
                .collect();
            oracle::same_raw(actual, expected);
        }
        if chr_reuse::continuations::COLLECT_METRICS {
            assert_eq!(table.stats().computed, if memo { 1 } else { 2 });
            assert_eq!(table.stats().hits, if memo { 1 } else { 0 });
        }
    }
}

#[test]
fn reuse_transports_two_independent_fresh_results_into_one_caller() {
    let calls = [
        c("work", [depth(3), v(7), v(8)]),
        c("work", [depth(3), v(7), v(9)]),
    ];
    let caller = Query {
        constraints: vec![calls[0].clone(), calls[1].clone(), c("caller", [v(1000)])],
        outputs: vec![
            ("a".into(), Var(8)),
            ("b".into(), Var(9)),
            ("input".into(), Var(7)),
            ("unrelated".into(), Var(1000)),
        ],
    };
    let expected = oracle::run(&rules(), &caller, 200_000);
    assert_eq!(expected.len(), 4);
    let mut table = Table::new(rules(), true).unwrap();
    let mut fresh = Fresh::for_query(&caller);
    let first = table.expand(&calls[0], &mut fresh, 200_000).unwrap();
    let second = table.expand(&calls[1], &mut fresh, 200_000).unwrap();
    let mut actual = vec![];
    for a in &first {
        for b in &second {
            actual.extend(replay(a.iter().chain(b).cloned().collect(), caller.clone()));
        }
    }
    oracle::same_raw(actual, expected);
}

#[test]
fn call_keys_distinguish_aliases_bindings_and_preserve_caller_only_variables() {
    for memo in [false, true] {
        let mut table = Table::new(rules(), memo).unwrap();
        for offset in [0, 100, 10_000] {
            for n in [0, 1, 8] {
                for kind in 0..6 {
                    for tag in ["a", "b"] {
                        let (a, b, r, u) = (offset + 7, offset + 9, offset + 8, offset + 1000);
                        let arg = match kind {
                            0 => v(a),
                            1 => t("pair", [v(a), v(a)]),
                            2 => t("pair", [v(a), v(b)]),
                            3 => atom("a"),
                            4 => atom("b"),
                            _ => v(r),
                        };
                        let call = c("work", [depth(n), arg, v(r)]);
                        let caller = Query {
                            constraints: vec![call.clone(), c("caller", [atom(tag), v(u)])],
                            outputs: vec![
                                ("input".into(), Var(a)),
                                ("other".into(), Var(b)),
                                ("result".into(), Var(r)),
                                ("unrelated".into(), Var(u)),
                            ],
                        };
                        let expected = oracle::run(&rules(), &caller, 200_000);
                        let mut fresh = Fresh::for_query(&caller);
                        let actual = table
                            .expand(&call, &mut fresh, 200_000)
                            .unwrap()
                            .into_iter()
                            .flat_map(|b| replay(b, caller.clone()))
                            .collect();
                        oracle::same_raw(actual, expected);
                    }
                }
            }
        }
    }
}

#[test]
fn call_reuse_succeeds_where_complete_caller_keys_differ() {
    use chr_reuse::continuations::{Mode, Search};
    let mut family = rules();
    family[1].body = eq(v(1), t("pair", [v(0), v(99), v(99)]));
    let body = |tag| {
        and([
            c("caller", [atom(tag), v(2)]).into(),
            c("work", [depth(8), v(0), v(1)]).into(),
        ])
    };
    let mut source = family.clone();
    source.push(Rule::simplify(
        "start",
        [c("start", [v(0), v(1), v(2)])],
        or(body("a"), body("b")),
    ));
    let query = Query {
        constraints: vec![c("start", [v(7), v(8), v(1000)])],
        outputs: vec![
            ("input".into(), Var(7)),
            ("result".into(), Var(8)),
            ("unrelated".into(), Var(1000)),
        ],
    };
    let expected = oracle::run(&source, &query, 200_000);
    let mut whole = Search::new(source, query.clone(), Mode::CompactLive).unwrap();
    let batch = whole.advance(200_000);
    assert!(batch.exhausted);
    oracle::same_raw(batch.answers, expected.clone());
    let mut direct = Table::new(family.clone(), false).unwrap();
    let mut table = Table::new(family, true).unwrap();
    let mut actual = vec![];
    let mut direct_actual = vec![];
    for tag in ["a", "b"] {
        let call = c("work", [depth(8), v(7), v(8)]);
        let caller = Query {
            constraints: vec![c("caller", [atom(tag), v(1000)]), call.clone()],
            outputs: query.outputs.clone(),
        };
        let mut fresh = Fresh::for_query(&caller);
        let mut direct_fresh = Fresh::for_query(&caller);
        let uncached = direct.expand(&call, &mut direct_fresh, 200_000).unwrap();
        assert_eq!(uncached.len(), 1);
        for bindings in uncached {
            direct_actual.extend(replay(bindings, caller.clone()));
        }
        for bindings in table.expand(&call, &mut fresh, 200_000).unwrap() {
            actual.extend(replay(bindings, caller.clone()));
        }
    }
    oracle::same_raw(direct_actual, expected.clone());
    oracle::same_raw(actual, expected);
    if chr_reuse::continuations::COLLECT_METRICS {
        assert_eq!(whole.stats().hits, 0);
        assert_eq!(table.stats().computed, 1);
        assert_eq!(table.stats().hits, 1);
        assert_eq!(direct.stats().executed, 2 * table.stats().executed);
        println!(
            "whole executed={} hits=0; isolated memo call executed={} computed=1 hits=1 (direct isolated calls execute twice this; caller replay excluded)",
            whole.stats().executed,
            table.stats().executed
        );
    }
}

#[test]
fn resource_access_suspension_and_cutoff_do_not_become_cached_answers() {
    let mut consuming = rules();
    consuming[0].removed.push(c("token", []));
    assert!(Table::new(consuming, true).is_err());
    let mut external = rules();
    external[0].body = c("outside", []).into();
    assert!(Table::new(external, true).is_err());
    let mut propagating = rules();
    propagating[0].kept.push(c("fact", []));
    assert!(Table::new(propagating, true).is_err());
    let mut table = Table::new(rules(), true).unwrap();
    let suspended = c("work", [v(0), v(1), v(2)]);
    let query = Query {
        constraints: vec![suspended.clone()],
        outputs: vec![],
    };
    let mut fresh = Fresh::for_query(&query);
    assert!(table
        .expand(&suspended, &mut fresh, 200_000)
        .unwrap_err()
        .contains("suspended"));
    let complete = c("work", [depth(8), v(1), v(2)]);
    assert!(table
        .expand(&complete, &mut fresh, 1)
        .unwrap_err()
        .contains("bound"));
    assert_eq!(
        table.expand(&complete, &mut fresh, 200_000).unwrap().len(),
        2
    );
    if chr_reuse::continuations::COLLECT_METRICS {
        assert_eq!(table.stats().computed, 3);
        assert_eq!(table.stats().hits, 0);
    }
}

#[test]
fn a_private_family_does_not_authorize_moving_a_call_before_input_supply() {
    let family = vec![
        Rule::simplify(
            "known",
            [c("work", [t("f", [atom("a")]), v(1)])],
            eq(v(1), atom("known")),
        ),
        Rule::simplify(
            "unknown",
            [c("work", [v(0), v(1)])],
            eq(v(1), atom("unknown")),
        ),
    ];
    let supply = Rule::simplify(
        "supply",
        [c("supply", [v(0)])],
        eq(v(0), t("f", [atom("a")])),
    );
    let caller = Query {
        constraints: vec![c("work", [v(0), v(1)]), c("supply", [v(0)])],
        outputs: vec![("input".into(), Var(0)), ("result".into(), Var(1))],
    };
    let mut source = vec![supply.clone()];
    source.extend(family.clone());
    let expected = oracle::run(&source, &caller, 200_000);
    assert_eq!(expected[0].outputs[1].1, atom("known"));
    let mut table = Table::new(family, true).unwrap();
    let mut fresh = Fresh::for_query(&caller);
    let bindings = table
        .expand(&caller.constraints[0], &mut fresh, 200_000)
        .unwrap();
    assert_eq!(bindings.len(), 1);
    let mut replayed = caller;
    replayed.constraints.retain(|c| c.name != "work");
    replayed.constraints.extend(
        bindings[0]
            .iter()
            .map(|(v, t)| c("bind", [Term::Var(*v), t.clone()])),
    );
    let bind = Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1)));
    let actual = oracle::run(&[bind, supply], &replayed, 200_000);
    assert_eq!(actual[0].outputs[1].1, atom("unknown"));
    assert!(!chr_observe::equivalent(
        &actual[0],
        &expected[0],
        &mut Default::default()
    ));
}

#[test]
fn checked_query_entry_rejects_priority_interference_and_external_consumers() {
    let call = c("work", [depth(3), v(7), v(8)]);
    let independent = Query {
        constraints: vec![call.clone(), c("caller", [v(1000)])],
        outputs: vec![("result".into(), Var(8))],
    };
    let mut table = Table::new(rules(), true).unwrap();
    assert_eq!(
        table
            .expand_query(&independent, 0, &rules(), 200_000)
            .unwrap()
            .len(),
        2
    );
    let mut shared = independent.clone();
    shared.constraints[1] = c("supply", [v(7)]);
    let mut preceding = vec![Rule::simplify(
        "supply",
        [c("supply", [v(0)])],
        eq(v(0), atom("a")),
    )];
    preceding.extend(rules());
    assert!(table
        .expand_query(&shared, 0, &preceding, 200_000)
        .unwrap_err()
        .contains("prefix"));
    let mut two = independent.clone();
    two.constraints.push(call.clone());
    assert!(table
        .expand_query(&two, 0, &rules(), 200_000)
        .unwrap_err()
        .contains("one initial"));
    let mut external = rules();
    external.push(Rule::simplify(
        "steal",
        [call.clone(), c("token", [])],
        chr_syntax::Goal::True,
    ));
    assert!(table
        .expand_query(&independent, 0, &external, 200_000)
        .unwrap_err()
        .contains("ownership"));
}

#[test]
fn selecting_the_first_call_rule_does_not_make_the_whole_call_atomic() {
    let family = vec![
        Rule::simplify(
            "step",
            [c("work", [t("s", [v(0)]), v(1), v(2)])],
            c("work", [v(0), v(1), v(2)]).into(),
        ),
        Rule::simplify(
            "supply",
            [c("supply", [v(0)])],
            eq(v(0), t("f", [atom("a")])),
        ),
        Rule::simplify(
            "known",
            [c("work", [atom("z"), t("f", [atom("a")]), v(0)])],
            eq(v(0), atom("known")),
        ),
        Rule::simplify(
            "unknown",
            [c("work", [atom("z"), v(0), v(1)])],
            eq(v(1), atom("unknown")),
        ),
    ];
    let call = c("work", [depth(1), v(0), v(1)]);
    let query = Query {
        constraints: vec![call.clone(), c("supply", [v(0)])],
        outputs: vec![("input".into(), Var(0)), ("result".into(), Var(1))],
    };
    let trace = oracle::run_traced(&family, &query, 200_000);
    assert_eq!(trace.len(), 1);
    assert_eq!(trace[0].1[0], 0, "the call really is selected first");
    assert_eq!(trace[0].0.outputs[1].1, atom("known"));
    let mut table = Table::new(family.clone(), true).unwrap();
    assert!(table.expand_query(&query, 0, &family, 200_000).is_err());
    let mut fresh = Fresh::for_query(&query);
    let isolated = table.expand(&call, &mut fresh, 200_000).unwrap();
    assert_eq!(
        isolated[0].iter().find(|(v, _)| *v == Var(1)).unwrap().1,
        atom("unknown")
    );
}

#[test]
fn disjoint_call_contraction_preserves_independent_choice_products() {
    let bind = Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1)));
    for n in [0, 1, 8] {
        for reverse in [false, true] {
            for failure in [false, true] {
                for offset in [0, 100] {
                    let choice = Rule::simplify(
                        "choice",
                        [c("choice", [v(0)])],
                        or(
                            eq(v(0), atom("a")),
                            if failure {
                                chr_syntax::Goal::Fail
                            } else {
                                eq(v(0), atom("b"))
                            },
                        ),
                    );
                    let mut global = rules();
                    global.push(choice.clone());
                    let call = c("work", [depth(n), v(offset + 7), v(offset + 8)]);
                    let mut query = Query {
                        constraints: vec![call, c("choice", [v(offset + 1000)])],
                        outputs: vec![
                            ("input".into(), Var(offset + 7)),
                            ("result".into(), Var(offset + 8)),
                            ("unrelated".into(), Var(offset + 1000)),
                        ],
                    };
                    if reverse {
                        query.constraints.reverse();
                    }
                    let selected = usize::from(reverse);
                    let expected = oracle::run(&global, &query, 200_000);
                    let mut table = Table::new(rules(), true).unwrap();
                    let mut actual = vec![];
                    for bindings in table
                        .expand_query(&query, selected, &global, 200_000)
                        .unwrap()
                    {
                        let mut q = query.clone();
                        q.constraints.remove(selected);
                        q.constraints.extend(
                            bindings
                                .into_iter()
                                .map(|(var, t)| c("bind", [Term::Var(var), t])),
                        );
                        actual.extend(oracle::run(&[bind.clone(), choice.clone()], &q, 200_000));
                    }
                    oracle::same_raw(actual, expected);
                }
            }
        }
    }
}

#[test]
fn private_phase_accepts_shared_observation_variables() {
    let call = c("work", [depth(3), v(7), v(8)]);
    // observer has no rule: it cannot write the shared parameter, and work only
    // carries that parameter opaquely into its result in this source family.
    let query = Query {
        constraints: vec![call.clone(), c("observer", [v(7)])],
        outputs: vec![("result".into(), Var(8))],
    };
    let mut table = Table::new(rules(), true).unwrap();
    let expected = oracle::run(&rules(), &query, 200_000);
    let actual = table
        .expand_query(&query, 0, &rules(), 200_000)
        .unwrap()
        .into_iter()
        .flat_map(|b| replay(b, query.clone()))
        .collect();
    oracle::same_raw(actual, expected);
}

#[test]
fn disjoint_variables_do_not_prove_a_source_progress_boundary() {
    let family = vec![Rule::simplify(
        "fail",
        [c("work", [])],
        chr_syntax::Goal::Fail,
    )];
    let mut global = vec![Rule::simplify(
        "spin",
        [c("spin", [])],
        c("spin", []).into(),
    )];
    global.extend(family.clone());
    let query = Query {
        constraints: vec![c("work", []), c("spin", [])],
        outputs: vec![],
    };
    let mut ordinary = chr_reuse::continuations::Search::new(
        global.clone(),
        query.clone(),
        chr_reuse::continuations::Mode::Direct,
    )
    .unwrap();
    assert!(!ordinary.advance(1000).exhausted);
    let mut table = Table::new(family, true).unwrap();
    let mut fresh = Fresh::for_query(&query);
    assert!(table
        .expand(&query.constraints[0], &mut fresh, 1000)
        .unwrap()
        .is_empty());
    assert!(
        table.expand_query(&query, 0, &global, 1000).is_err(),
        "disjointness alone admits early failure across an infinite priority prefix"
    );
}

#[test]
fn private_phase_replays_bindings_before_shared_caller_updates() {
    let mut table = Table::new(rules(), true).unwrap();
    for (target, value) in [(7, "a"), (7, "b"), (8, "a")] {
        let supply = Rule::simplify("supply", [c("supply", [v(0)])], eq(v(0), atom(value)));
        let mut global = rules();
        global.push(supply.clone());
        let query = Query {
            constraints: vec![c("work", [depth(3), v(7), v(8)]), c("supply", [v(target)])],
            outputs: vec![("input".into(), Var(7)), ("result".into(), Var(8))],
        };
        let expected = oracle::run(&global, &query, 200_000);
        let mut actual = vec![];
        for bindings in table.expand_query(&query, 0, &global, 200_000).unwrap() {
            let mut replayed = query.clone();
            replayed.constraints.remove(0);
            replayed.constraints.extend(
                bindings
                    .into_iter()
                    .map(|(var, t)| c("bind", [Term::Var(var), t])),
            );
            let bind = Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1)));
            actual.extend(oracle::run(&[bind, supply.clone()], &replayed, 200_000));
        }
        oracle::same_raw(actual, expected);
    }
    if chr_reuse::continuations::COLLECT_METRICS {
        assert_eq!(table.stats().computed, 1);
        assert_eq!(table.stats().hits, 2);
    }
}

#[test]
fn resumed_caller_can_start_another_private_phase() {
    let later = Rule::simplify(
        "later",
        [c("later", [v(0), v(1)])],
        c("work", [depth(1), v(0), v(1)]).into(),
    );
    let mut global = rules();
    global.push(later);
    let query = Query {
        constraints: vec![c("work", [depth(3), v(7), v(8)]), c("later", [v(7), v(9)])],
        outputs: vec![
            ("first".into(), Var(8)),
            ("second".into(), Var(9)),
            ("input".into(), Var(7)),
        ],
    };
    let expected = oracle::run(&global, &query, 200_000);
    assert_eq!(expected.len(), 4);
    let mut table = Table::new(rules(), true).unwrap();
    let mut actual = vec![];
    for bindings in table.expand_query(&query, 0, &global, 200_000).unwrap() {
        let mut q = query.clone();
        q.constraints.remove(0);
        q.constraints.extend(
            bindings
                .into_iter()
                .map(|(v, t)| c("bind", [Term::Var(v), t])),
        );
        let mut continuation = vec![Rule::simplify(
            "bind",
            [c("bind", [v(0), v(1)])],
            eq(v(0), v(1)),
        )];
        continuation.extend(global.clone());
        actual.extend(oracle::run(&continuation, &q, 200_000));
    }
    oracle::same_raw(actual, expected);
}

#[test]
fn resumed_machine_imports_replay_and_caller_in_one_variable_scope() {
    use chr_persistent::continuations::{PreparedMachine, Step};
    use std::collections::VecDeque;
    let supply = Rule::simplify("supply", [c("supply", [v(0)])], eq(v(0), atom("a")));
    let query = Query {
        constraints: vec![c("supply", [v(7)]), c("observer", [v(8), v(1000)])],
        outputs: vec![
            ("input".into(), Var(7)),
            ("result".into(), Var(8)),
            ("unrelated".into(), Var(1000)),
        ],
    };
    let bindings = vec![(Var(8), t("pair", [v(7), v(2000), v(2000)]))];
    let prepared = PreparedMachine::new(vec![supply]).unwrap();
    let (mut machine, cursor) = prepared.start_replaying(query, bindings).unwrap();
    let mut frontier = VecDeque::from([cursor]);
    let mut answers = vec![];
    for _ in 0..200_000 {
        let Some(c) = frontier.pop_front() else {
            break;
        };
        match machine.step(c) {
            Step::Continue(c) => frontier.push_back(c),
            Step::Split(a, b) => {
                frontier.push_back(a);
                frontier.push_back(b);
            }
            Step::Failed => (),
            Step::Answer(a) => answers.push(a),
        }
    }
    assert!(frontier.is_empty());
    let pair = t("pair", [atom("a"), v(2000), v(2000)]);
    let expected = chr_syntax::Answer {
        outputs: vec![
            ("input".into(), atom("a")),
            ("result".into(), pair.clone()),
            ("unrelated".into(), v(1000)),
        ],
        residual: vec![c("observer", [pair, v(1000)])],
    };
    oracle::same_raw(answers, vec![expected]);
}

#[test]
fn complete_caller_path_agrees_and_cancellation_does_not_poison_reuse() {
    use chr_reuse::calls::{Caller, CallerEvent};
    for memo in [false, true] {
        for n in [0, 3, 8] {
            for conflict in [false, true] {
                let supply = Rule::simplify("supply", [c("supply", [v(0)])], eq(v(0), atom("a")));
                let mut global = rules();
                global.push(supply);
                let query = Query {
                    constraints: vec![
                        c("work", [depth(n), v(7), v(8)]),
                        c("supply", [v(if conflict { 8 } else { 7 })]),
                    ],
                    outputs: vec![("input".into(), Var(7)), ("result".into(), Var(8))],
                };
                let expected = oracle::run(&global, &query, 200_000);
                let mut caller = Caller::new(global, 2, memo).unwrap();
                // Cancellation before expansion and after expansion retain no caller
                // effects in prepared rules or cached interface answers.
                drop(caller.start(query.clone(), 200_000, 200_000));
                let mut canceled = caller.start(query.clone(), 200_000, 200_000);
                assert!(matches!(
                    caller.step(&mut canceled).unwrap(),
                    CallerEvent::Progress
                ));
                drop(canceled);
                let mut run = caller.start(query, 200_000, 200_000);
                let mut answers = vec![];
                let mut done = false;
                for _ in 0..200_000 {
                    match caller.step(&mut run).unwrap() {
                        CallerEvent::Progress => (),
                        CallerEvent::Answer(a) => answers.push(a),
                        CallerEvent::Done => {
                            done = true;
                            break;
                        }
                    }
                }
                assert!(done);
                drop(run);
                drop(caller);
                oracle::same_raw(answers, expected);
            }
        }
    }
}

#[test]
fn caller_limits_are_terminal_errors_not_exhaustion() {
    use chr_reuse::calls::{Caller, CallerEvent};
    let query = Query {
        constraints: vec![c("work", [depth(8), v(7), v(8)])],
        outputs: vec![("result".into(), Var(8))],
    };
    let mut caller = Caller::new(rules(), 2, true).unwrap();
    let mut short = caller.start(query.clone(), 1, 200_000);
    assert!(caller
        .step(&mut short)
        .unwrap_err()
        .contains("service bound"));
    assert!(caller.step(&mut short).is_err());
    let mut short = caller.start(query, 200_000, 0);
    assert!(matches!(
        caller.step(&mut short).unwrap(),
        CallerEvent::Progress
    ));
    assert!(caller
        .step(&mut short)
        .unwrap_err()
        .contains("resumed caller"));
    assert!(caller.step(&mut short).is_err());
}
