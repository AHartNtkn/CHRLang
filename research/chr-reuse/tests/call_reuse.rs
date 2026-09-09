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
