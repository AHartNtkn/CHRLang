use chr_reuse::{
    calls::body::Caller,
    continuations::{Mode, Search},
};
use chr_syntax::{Goal, Query, Rule, atom, c, eq, v};
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
#[allow(dead_code)]
mod scalar;
#[path = "../examples/support/call_trace_source.rs"]
mod source;
fn check(caller: &mut Caller, rules: &[Rule], q: &Query, bound: usize) -> usize {
    let mut run = caller.start(q.clone()).unwrap();
    let mut direct = Search::new(rules.to_vec(), q.clone(), Mode::Direct).unwrap();
    let mut held = vec![];
    for step in 0..bound {
        let a = caller.advance(&mut run, 1).unwrap();
        let b = direct.advance(1);
        assert_eq!(a.exhausted, b.exhausted, "exhaustion at{step}");
        assert_eq!(a.answers.len(), b.answers.len(), "delivery at{step}");
        for (x, y) in a.answers.iter().zip(&b.answers) {
            assert!(
                chr_observe::equivalent(x, y, &mut Default::default()),
                "at{step}: {x:?}/{y:?}"
            );
        }
        held.extend(a.answers);
        if a.exhausted {
            drop(run);
            drop(direct);
            scalar::same_raw(held, scalar::run(rules, q, 100000));
            return step + 1;
        }
    }
    assert!(bound < 100000, "source bound");
    bound
}
#[test]
fn observers_interleave_with_reused_bodies_in_source_order() {
    let (mut cases, mut events) = (0, 0);
    for depth in [0, 2, 4] {
        for family in 0..4 {
            for reverse in [false, true] {
                for position in 0..3 {
                    for consuming in [false, true] {
                        let (mut rules, count) = source::program(reverse, family);
                        let heads = vec![c("wait", [v(0), v(1), v(2)]), c("token", [])];
                        let observer = if consuming {
                            Rule::simplify(
                                "observer",
                                heads,
                                Goal::And(vec![
                                    eq(v(2), atom("observed")),
                                    c("observed", [v(1)]).into(),
                                ]),
                            )
                        } else {
                            Rule::propagate("observer", heads, c("observed", [v(1)]).into())
                        };
                        rules.insert([0, 1, count][position], observer);
                        let mut caller = Caller::new(rules.clone()).unwrap();
                        for tokens in 0..3 {
                            for offset in [10, 1000] {
                                let mut q = source::input(depth, depth + 1, offset);
                                q.constraints.retain(|c| c.name != "token");
                                q.constraints.extend((0..tokens).map(|_| c("token", [])));
                                if position == 0 && consuming && tokens > 0 {
                                    let answers = scalar::run(&rules, &q, 100000);
                                    assert!(!answers.is_empty());
                                    assert!(
                                        answers.iter().all(|a| a.outputs[0].1 == atom("observed")),
                                        "higher-priority observer must intercept private work"
                                    );
                                }
                                for cutoff in [0, 1, 5] {
                                    check(&mut caller, &rules, &q, cutoff);
                                    let executed = caller.executed();
                                    events += check(&mut caller, &rules, &q, 100000);
                                    if cfg!(feature = "metrics") && cutoff > 0 {
                                        assert_eq!(
                                            caller.executed(),
                                            executed,
                                            "warm body execution"
                                        );
                                    }
                                }
                                cases += 1;
                            }
                        }
                        if cfg!(feature = "metrics") {
                            assert!(caller.replayed() > 0);
                        }
                    }
                }
            }
        }
    }
    assert_eq!(cases, 864);
    eprintln!("body_sources={cases} checkpoints={events}");
}
#[test]
fn body_runs_belong_to_their_caller() {
    let (rules, _, q) = source::source(0, 1, false, 0, 10);
    let a = Caller::new(rules.clone()).unwrap();
    let mut b = Caller::new(rules).unwrap();
    assert!(b.advance(&mut a.start(q).unwrap(), 1).is_err());
}

#[test]
fn posted_names_fresh_aliases_and_existing_occurrences_survive_body_reuse() {
    use chr_syntax::{Var, t};
    let rules = vec![
        Rule::simplify(
            "launch",
            [c("start", [v(0)])],
            Goal::And(vec![
                eq(v(0), t("pair", [v(1), v(1)])),
                c("$body", [v(1)]).into(),
                c("$body_", [v(1)]).into(),
            ]),
        ),
        Rule::simplify(
            "first",
            [c("$body", [v(0)]), c("token", [])],
            Goal::And(vec![eq(v(0), atom("a")), c("seen", [v(0)]).into()]),
        ),
        Rule::simplify("second", [c("$body_", [v(0)])], eq(v(0), atom("a"))),
    ];
    let mut caller = Caller::new(rules.clone()).unwrap();
    for id in [10, 1000] {
        let q = Query {
            constraints: vec![
                c("$body", [atom("a")]),
                c("start", [v(id)]),
                c("token", []),
                c("token", []),
            ],
            outputs: vec![("out".into(), Var(id))],
        };
        for _ in 0..2 {
            check(&mut caller, &rules, &q, 100000);
        }
        let answers = scalar::run(&rules, &q, 100000);
        assert_eq!(answers.len(), 1);
        assert_eq!(answers[0].outputs[0].1, t("pair", [atom("a"), atom("a")]));
        assert_eq!(
            answers[0]
                .residual
                .iter()
                .filter(|c| c.name == "seen")
                .count(),
            2
        );
    }
}

#[test]
fn guards_and_consuming_heads_remain_in_the_live_caller() {
    let rules = vec![
        Rule {
            name: "guarded".into(),
            kept: vec![c("ready", [v(0)])],
            removed: vec![c("token", [])],
            guards: vec![chr_syntax::Guard::Equal(v(0), atom("a"))],
            body: c("picked", [v(0)]).into(),
        },
        Rule::simplify("other", [c("token", [])], c("miss", []).into()),
    ];
    let mut caller = Caller::new(rules.clone()).unwrap();
    for name in ["a", "b", "a"] {
        let q = Query {
            constraints: vec![c("ready", [atom(name)]), c("token", []), c("token", [])],
            outputs: vec![],
        };
        check(&mut caller, &rules, &q, 100000);
        let expected = scalar::run(&rules, &q, 100000);
        assert_eq!(expected.len(), 1);
        assert_eq!(
            expected[0]
                .residual
                .iter()
                .filter(|c| c.name == if name == "a" { "picked" } else { "miss" })
                .count(),
            2
        );
    }
}

#[test]
fn prior_body_bindings_participate_in_later_occurs_checks() {
    use chr_syntax::{Var, t};
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0), v(1)])],
            eq(v(0), t("f", [v(1)])),
        ),
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
    ];
    let mut caller = Caller::new(rules.clone()).unwrap();
    for cyclic in [false, true, false, true] {
        let q = Query {
            constraints: vec![
                c("start", [v(10), v(11)]),
                c("bind", [v(11), if cyclic { v(10) } else { atom("a") }]),
            ],
            outputs: vec![("out".into(), Var(10))],
        };
        check(&mut caller, &rules, &q, 100000);
        let answers = scalar::run(&rules, &q, 100000);
        assert_eq!(answers.len(), usize::from(!cyclic));
        if !cyclic {
            assert_eq!(answers[0].outputs[0].1, t("f", [atom("a")]));
        }
    }
}
