use chr_reuse::{
    calls::trace::caller::Caller,
    continuations::{Mode, Search},
};
use chr_syntax::{Goal, Query, Rule, Var, atom, c, eq, t, v};
#[path = "../examples/support/call_trace_source.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use fixture::source;
fn check(caller: &mut Caller, rs: &[Rule], q: &Query, cutoff: usize) -> usize {
    let mut run = caller.start(q.clone()).unwrap();
    let mut direct = Search::new(rs.to_vec(), q.clone(), Mode::Direct).unwrap();
    let mut answers = vec![];
    for step in 0..cutoff {
        let actual = caller.advance(&mut run, 1).unwrap();
        let expected = direct.advance(1);
        assert_eq!(
            actual.exhausted, expected.exhausted,
            "exhaustion step {step}"
        );
        assert_eq!(
            actual.answers.len(),
            expected.answers.len(),
            "delivery step {step}"
        );
        for (a, b) in actual.answers.iter().zip(&expected.answers) {
            assert!(
                chr_observe::equivalent(a, b, &mut Default::default()),
                "answer at step {step}: {a:?} / {b:?}"
            );
        }
        answers.extend(actual.answers);
        if actual.exhausted {
            drop(run);
            drop(direct);
            scalar::same_raw(answers, scalar::run(rs, q, 100_000));
            return step + 1;
        }
    }
    assert!(cutoff < 100_000, "source bound");
    cutoff
}
#[test]
fn complete_callers_preserve_delivery_resources_history_and_restarts() {
    let mut cases = 0;
    let mut events = 0;
    for a in [0, 1, 4, 8] {
        for b in [0, 1, 4, 8] {
            for reverse in [false, true] {
                for mode in 0..4 {
                    for offset in [10, 1000] {
                        let (rs, count, q) = source(a, b, reverse, mode, offset);
                        let mut caller = Caller::new(rs.clone(), count).unwrap();
                        for cutoff in [0, 1, 5] {
                            check(&mut caller, &rs, &q, cutoff);
                            events += check(&mut caller, &rs, &q, 100_000);
                        }
                        if cfg!(feature = "metrics") {
                            assert!(caller.stats().hits > 0);
                        }
                        cases += 1;
                    }
                }
            }
        }
    }
    assert_eq!(cases, 256);
    eprintln!("callers={cases} complete_service_events={events}");
}
#[test]
fn a_suspended_call_resumes_after_caller_input() {
    let rs = vec![
        Rule::simplify("go", [c("go", [v(0)])], c("blocked", [v(0)]).into()),
        Rule::simplify("ready", [c("blocked", [atom("a")])], Goal::True),
        Rule::simplify("supply", [c("input", [v(0)])], eq(v(0), atom("a"))),
    ];
    let q = Query {
        constraints: vec![c("go", [v(10)]), c("input", [v(10)])],
        outputs: vec![("out".into(), Var(10))],
    };
    let mut caller = Caller::new(rs.clone(), 2).unwrap();
    for _ in 0..2 {
        check(&mut caller, &rs, &q, 100_000);
    }
    if cfg!(feature = "metrics") {
        assert!(caller.stats().hits > 0);
    }
}

#[test]
fn callers_enforce_table_ownership_and_admit_observers() {
    let (mut rs, n, q) = source(1, 4, false, 0, 10);
    let a = Caller::new(rs.clone(), n).unwrap();
    let mut b = Caller::new(rs.clone(), n).unwrap();
    assert!(b.advance(&mut a.start(q).unwrap(), 1).is_err());
    rs.push(Rule::simplify(
        "external_reader",
        [c("wait", [v(0), v(1), v(2)]), c("token", [])],
        Goal::True,
    ));
    let mut observed = Caller::new(rs.clone(), n).unwrap();
    check(&mut observed, &rs, &fixture::input(1, 4, 10), 100_000);
}

#[test]
fn suspended_fresh_alias_is_shared_with_the_caller_input() {
    let rs = vec![
        Rule::simplify(
            "go",
            [c("go", [v(0)])],
            Goal::And(vec![
                eq(v(0), t("box", [v(1)])),
                c("blocked", [v(1)]).into(),
            ]),
        ),
        Rule::simplify("ready", [c("blocked", [atom("a")])], Goal::True),
        Rule::simplify(
            "supply",
            [c("input", [t("box", [v(0)])])],
            eq(v(0), atom("a")),
        ),
    ];
    let mut caller = Caller::new(rs.clone(), 2).unwrap();
    for id in [10, 1000] {
        let q = Query {
            constraints: vec![c("go", [v(id)]), c("input", [v(id)])],
            outputs: vec![("out".into(), Var(id))],
        };
        check(&mut caller, &rs, &q, 100_000);
    }
}

#[test]
fn changed_query_retention_tracks_completed_and_unfinished_traces() {
    for mode in [0, 1] {
        let (rs, count) = fixture::program(false, mode);
        let mut caller = Caller::new(rs, count).unwrap();
        for q in 0..4 {
            let mut run = caller
                .start(fixture::input(4 + 2 * q, 5 + 2 * q, (1000 * q + 7) as u64))
                .unwrap();
            for _ in 0..100_000 {
                let batch = caller.advance(&mut run, 1).unwrap();
                if batch.exhausted || (mode == 0 && !batch.answers.is_empty()) {
                    break;
                }
            }
            drop(run);
            eprintln!(
                "mode={mode} query={q} unfinished={} nodes={}",
                caller.unfinished_calls(),
                caller.retained_nodes()
            );
            assert_eq!(caller.retained_nodes(), 2 * (q + 1));
            assert_eq!(caller.unfinished_calls(), if mode == 0 { q + 1 } else { 0 });
        }
    }
}

#[test]
fn reclamation_preserves_service_restarts_and_regenerates_calls() {
    let mut events = 0;
    for family in 0..4 {
        for window in [0, 1, 4, 16] {
            let (rs, count) = fixture::program(false, family);
            let mut caller = Caller::new(rs.clone(), count).unwrap();
            for q in 0..32 {
                let depth = 8 + 2 * (q % 16);
                let input = fixture::input(depth, depth + 1, (1000 * q + 7) as u64);
                check(&mut caller, &rs, &input, 5);
                events += check(&mut caller, &rs, &input, 100_000);
                if window != 0 && (q + 1) % window == 0 {
                    caller.clear_traces().unwrap();
                    assert_eq!(caller.retained_nodes(), 0);
                    assert_eq!(caller.unfinished_calls(), 0);
                }
            }
            if cfg!(feature = "metrics") {
                eprintln!("family={family} window={window} work={:?}", caller.stats());
            }
        }
    }
    let (rs, count, q) = fixture::source(8, 9, false, 0, 10);
    let mut caller = Caller::new(rs.clone(), count).unwrap();
    let mut live = caller.start(q.clone()).unwrap();
    caller.advance(&mut live, 5).unwrap();
    assert!(caller.clear_traces().is_err());
    drop(live);
    caller.clear_traces().unwrap();
    check(&mut caller, &rs, &q, 100_000);
    let executed = caller.stats().executed;
    check(&mut caller, &rs, &q, 100_000);
    if cfg!(feature = "metrics") {
        assert_eq!(caller.stats().executed, executed);
    }
    caller.clear_traces().unwrap();
    check(&mut caller, &rs, &q, 100_000);
    if cfg!(feature = "metrics") {
        assert!(caller.stats().executed > executed);
    }
    eprintln!("reclamation_queries=512 service_events={events}");
}

#[test]
fn live_observers_consume_suspended_facts_and_reenable_private_rules() {
    let mut cases = 0;
    let mut events = 0;
    for depth in [0, 2, 5] {
        for family in 0..7 {
            for consuming in [false, true] {
                for supply_first in [false, true] {
                    let base = match family {
                        0 | 6 => c("blocked", [v(0)]).into(),
                        1 => Goal::And(vec![
                            eq(v(0), t("box", [v(1), v(1)])),
                            c("blocked", [v(1)]).into(),
                        ]),
                        2 => {
                            chr_syntax::or(c("blocked", [v(0)]).into(), c("blocked", [v(0)]).into())
                        }
                        3 => chr_syntax::or(c("blocked", [v(0)]).into(), Goal::Fail),
                        4 | 5 => {
                            let second = if family == 4 { v(2) } else { v(1) };
                            Goal::And(vec![
                                eq(v(0), t("pair", [v(1), second.clone()])),
                                c("blocked", [v(1)]).into(),
                                c("blocked", [second]).into(),
                            ])
                        }
                        _ => unreachable!(),
                    };
                    let mut rs = vec![
                        Rule::simplify(
                            "recurse",
                            [c("go", [t("s", [v(0)]), v(1)])],
                            c("go", [v(0), v(1)]).into(),
                        ),
                        Rule::simplify("base", [c("go", [atom("z"), v(0)])], base),
                        Rule::simplify("ready", [c("blocked", [atom("ready")])], Goal::True),
                    ];
                    let mut heads = vec![c("blocked", [v(0)])];
                    if matches!(family, 4 | 5) {
                        heads.push(c("blocked", [v(1)]));
                    }
                    let observation = c(
                        "seen",
                        [if matches!(family, 4 | 5) {
                            t("pair", [v(0), v(1)])
                        } else {
                            v(0)
                        }],
                    );
                    let observer = if consuming {
                        heads.push(c("token", []));
                        let mut body = vec![eq(v(0), atom("claimed")), observation.into()];
                        if matches!(family, 4 | 5) {
                            body.push(eq(v(1), atom("second")));
                        }
                        Rule::simplify("observe", heads, Goal::And(body))
                    } else {
                        Rule::propagate("observe", heads, observation.into())
                    };
                    let supply = Rule {
                        name: "supply".into(),
                        kept: vec![c("blocked", [v(0)])],
                        removed: vec![c("input", [])],
                        guards: vec![],
                        body: if family == 6 {
                            Goal::And(vec![
                                eq(v(0), atom("ready")),
                                c("go", [atom("z"), v(1)]).into(),
                            ])
                        } else {
                            eq(v(0), atom("ready"))
                        },
                    };
                    if supply_first {
                        rs.extend([supply, observer]);
                    } else {
                        rs.extend([observer, supply]);
                    }
                    rs.push(Rule::simplify(
                        "competitor",
                        [c("token", [])],
                        c("winner", [atom("other")]).into(),
                    ));
                    let mut caller = Caller::new(rs.clone(), 3).unwrap();
                    for tokens in 0..3 {
                        for offset in [10, 1000] {
                            let mut q = Query {
                                constraints: vec![
                                    c("go", [fixture::nat(depth), v(offset)]),
                                    c("input", []),
                                ],
                                outputs: vec![("out".into(), Var(offset))],
                            };
                            q.constraints.extend((0..tokens).map(|_| c("token", [])));
                            let oracle = scalar::run(&rs, &q, 100_000);
                            if !consuming && (!supply_first || family == 6) {
                                assert!(
                                    oracle
                                        .iter()
                                        .any(|a| a.residual.iter().any(|c| c.name == "seen")),
                                    "observer must actually fire"
                                );
                            }
                            if family == 5 && consuming && !supply_first && tokens > 0 {
                                assert!(
                                    oracle.is_empty(),
                                    "consuming aliased facts must expose conflicting bindings"
                                );
                            }
                            for cutoff in [0, 1, 5] {
                                check(&mut caller, &rs, &q, cutoff);
                                events += check(&mut caller, &rs, &q, 100000);
                            }
                            cases += 1;
                        }
                    }
                    if cfg!(feature = "metrics") {
                        assert!(caller.stats().hits > 0);
                        assert!(caller.stats().replayed > 0);
                    }
                }
            }
        }
    }
    assert_eq!(cases, 504);
    eprintln!("live_observer_cases={cases} source_checkpoints={events}");
}
