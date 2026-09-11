//! Structural control: count actual serviced equations outside production code.
use super::*;
use chr_syntax::{atom, c, eq, t, v};
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;

fn nest(depth: usize, mut leaf: Term) -> Term {
    for _ in 0..depth {
        leaf = t("f", [leaf]);
    }
    leaf
}
fn source(depth: usize, deep: bool, fail: bool) -> (Vec<Rule>, Query) {
    let pattern = if deep {
        nest(depth, atom("a"))
    } else {
        t("f", [v(0)])
    };
    let mut rules = vec![
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
        Rule::simplify(
            "use",
            [c("open", [pattern, v(1)])],
            if fail {
                Goal::Fail
            } else {
                c("done", []).into()
            },
        ),
    ];
    if deep {
        rules[1]
            .guards
            .push(chr_syntax::Guard::Equal(v(1), atom("a")));
    }
    let query = Query {
        constraints: vec![
            c("bind", [nest(depth, v(10)), nest(depth, atom("a"))]),
            c("open", [nest(depth, v(10)), v(10)]),
        ],
        outputs: vec![("leaf".into(), Var(10))],
    };
    (rules, query)
}
fn run(rules: &[Rule], query: &Query, barrier: bool) -> (Vec<Answer>, usize) {
    let (answers, deductions, _) =
        run_observed(Prepared::new(rules).unwrap().start(query), barrier);
    (answers, deductions)
}
fn run_observed(mut engine: Engine, barrier: bool) -> (Vec<Answer>, usize, Vec<usize>) {
    let mut answers = vec![];
    let mut consumed_pending = vec![];
    let mut deductions = 0;
    for _ in 0..100_000 {
        if let Some(state) = engine.frontier.front_mut() {
            if barrier {
                while state.store.step() {
                    deductions += 1;
                    state.candidates.fill(None);
                }
            } else if state.store.pending() > 0 {
                deductions += 1;
            }
        }
        let was_live = engine
            .frontier
            .front()
            .is_some_and(|state| state.store.view.locations.contains_key(&Occurrence(1)));
        match engine.advance() {
            Step::Answer(a) => answers.push(a),
            Step::Exhausted => return (answers, deductions, consumed_pending),
            Step::Progress => (),
        }
        if was_live
            && let Some(state) = engine.frontier.front()
            && !state.store.view.locations.contains_key(&Occurrence(1))
        {
            consumed_pending.push(state.store.pending());
        }
    }
    panic!("finite experiment exceeded service bound");
}
#[test]
fn useful_early_failure_and_settlement_dependent_control() {
    for depth in [4, 16, 64] {
        for deep in [false, true] {
            for fail in [false, true] {
                let (rules, query) = source(depth, deep, fail);
                let expected = oracle::run(&rules, &query, 10_000);
                assert_eq!(expected.len(), usize::from(!fail));
                let (early, early_work) = run(&rules, &query, false);
                let (settled, settled_work) = run(&rules, &query, true);
                oracle::same_raw(early, expected.clone());
                oracle::same_raw(settled, expected);
                if fail && !deep {
                    assert!(early_work < settled_work);
                } else {
                    assert_eq!(early_work, settled_work);
                }
                println!(
                    "depth={depth} deep={deep} fail={fail} interleaved={early_work} settled={settled_work}"
                );
            }
        }
    }
}

#[test]
fn merged_constructor_facts_are_unique_but_source_occurrences_are_not() {
    let mut store = Store::default();
    let x = store.unknown();
    let a = store.constructor("a", &[]);
    let fx = store.constructor("f", &[x]);
    let fa = store.constructor("f", &[a]);
    store.post("p", &[fx]);
    store.post("p", &[fa]);
    store.equate(x, a);
    while store.step() {}
    assert_eq!(store.view.descriptors(store.root(fx)).len(), 1);
    store
        .view
        .constructor("f", store.root(fx), &[store.root(a)]);
    assert_eq!(store.view.descriptors(store.root(fx)).len(), 1);
    let plan = HeadPlan::compile(&[], &[c("p", [t("f", [atom("a")])])]);
    assert_eq!(store.matches(&plan).matches.len(), 2);
}

fn priority_source(depth: usize, early: bool, clash: bool, tokens: usize) -> (Vec<Rule>, Query) {
    let tail_left = nest(depth, v(11));
    let tail_right = nest(depth, atom("b"));
    let (left, right) = if early {
        (
            t("pair", [v(10), tail_left]),
            t("pair", [atom("a"), tail_right]),
        )
    } else {
        (
            t("pair", [tail_left, v(10)]),
            t("pair", [tail_right, atom("a")]),
        )
    };
    let mut older = Rule::simplify(
        "older",
        [c("use", [v(0), v(1), v(2)]), c("token", [])],
        eq(v(2), atom("older")),
    );
    older.guards.extend([
        chr_syntax::Guard::Equal(v(0), atom("a")),
        chr_syntax::Guard::Equal(v(1), atom("b")),
    ]);
    let mut newer = Rule::simplify(
        "newer",
        [c("use", [v(0), v(1), v(2)]), c("token", [])],
        eq(v(2), atom("newer")),
    );
    newer.guards.push(chr_syntax::Guard::Equal(v(0), atom("a")));
    let rules = vec![
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
        older,
        newer,
    ];
    let mut constraints = vec![c("bind", [left, right]), c("use", [v(10), v(11), v(12)])];
    // The conflicting leaf is supplied before source execution, while the
    // constructor equation discovers that contradiction only after decomposition.
    if clash && let Term::App(_, xs) = &mut constraints[0].args[0] {
        let at = usize::from(early);
        xs[at] = nest(depth, atom("c"));
    }
    constraints.extend((0..tokens).map(|_| c("token", [])));
    (
        rules,
        Query {
            constraints,
            outputs: vec![
                ("winner".into(), Var(12)),
                ("x".into(), Var(10)),
                ("y".into(), Var(11)),
            ],
        },
    )
}

#[test]
fn partial_equality_distinguishes_priority_from_permitted_serialization() {
    for depth in [4, 16, 64] {
        for early in [false, true] {
            for clash in [false, true] {
                for tokens in [1, 2] {
                    let (rules, query) = priority_source(depth, early, clash, tokens);
                    let atomic = oracle::run(&rules, &query, 100_000);
                    let mut other_order = rules.clone();
                    other_order.swap(1, 2);
                    let other_atomic = oracle::run(&other_order, &query, 100_000);
                    let prepared = Prepared::new(&rules).unwrap();
                    let (settled, settled_work, settled_pending) =
                        run_observed(prepared.start(&query), true);
                    let (eager, eager_work, eager_pending) =
                        run_observed(prepared.start(&query), false);
                    assert_eq!(eager_pending.len(), 1);
                    assert!(eager_pending[0] > 0);
                    if !clash {
                        assert_eq!(settled_pending, vec![0]);
                    }
                    let mut cancelled = prepared.start(&query);
                    for _ in 0..1000 {
                        assert!(matches!(cancelled.advance(), Step::Progress));
                        if cancelled
                            .frontier
                            .front()
                            .is_some_and(|s| !s.store.view.locations.contains_key(&Occurrence(1)))
                        {
                            break;
                        }
                    }
                    assert!(
                        cancelled
                            .frontier
                            .front()
                            .is_some_and(|s| s.store.pending() > 0
                                && !s.store.view.locations.contains_key(&Occurrence(1)))
                    );
                    drop(cancelled);
                    let (after_cancel, _, _) = run_observed(prepared.start(&query), true);
                    oracle::same_raw(after_cancel, atomic.clone());
                    oracle::same_raw(settled, atomic.clone());
                    if clash {
                        assert!(atomic.is_empty() && other_atomic.is_empty() && eager.is_empty());
                    } else {
                        let expected = vec![Answer {
                            outputs: vec![
                                ("winner".into(), atom("newer")),
                                ("x".into(), atom("a")),
                                ("y".into(), atom("b")),
                            ],
                            residual: (1..tokens).map(|_| c("token", [])).collect(),
                        }];
                        oracle::same_raw(eager, expected.clone());
                        oracle::same_raw(other_atomic, expected);
                        assert_eq!(atomic[0].outputs[0].1, atom("older"));
                    }
                    println!(
                        "PRIORITY,depth={depth},early={early},clash={clash},tokens={tokens},eager={eager_work},settled={settled_work},pending_at_claim={}",
                        eager_pending[0]
                    );
                }
            }
        }
    }
}
