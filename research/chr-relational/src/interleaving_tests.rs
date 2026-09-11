//! Structural control: count actual serviced equations outside production code.
use super::*;
use chr_syntax::{atom, c, eq, t, v};
#[path = "../tests/support/readiness_source.rs"]
mod readiness_source;
use readiness_source::{nest, separated_source};
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;

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
fn run_observed(engine: Engine, barrier: bool) -> (Vec<Answer>, usize, Vec<usize>) {
    let (answers, deductions, pending, _) = run_observed_counted(engine, barrier);
    (answers, deductions, pending)
}
fn run_observed_counted(
    mut engine: Engine,
    barrier: bool,
) -> (Vec<Answer>, usize, Vec<usize>, usize) {
    let mut answers = vec![];
    let mut consumed_pending = vec![];
    let mut deductions = 0;
    for turn in 1..=100_000 {
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
            Step::Exhausted => return (answers, deductions, consumed_pending, turn),
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

struct ReadinessRun {
    turns: usize,
    answers: Vec<Answer>,
    background: usize,
    selected: usize,
    pending_at_claim: Vec<usize>,
}
fn run_relevant(rules: &[Rule], query: &Query) -> ReadinessRun {
    for budget in [1, 8, 256] {
        let (answers, turns, work) = run_batched(rules, query, budget);
        oracle::same_raw(answers, oracle::run(rules, query, 100_000));
        println!("DRAIN_SOURCE,budget={budget},turns={turns},work={work}");
    }
    let reads = crate::store::MatcherReads::new(rules);
    drive_relevant(Prepared::new(rules).unwrap().start(query), &reads)
}
fn drive_relevant(mut engine: Engine, reads: &crate::store::MatcherReads) -> ReadinessRun {
    let mut result = ReadinessRun {
        turns: 0,
        answers: vec![],
        background: 0,
        selected: 0,
        pending_at_claim: vec![],
    };
    for _ in 0..100_000 {
        if let Some(state) = engine.frontier.front_mut() {
            if state.pending.is_empty() {
                while state.store.step_for_matching(reads) {
                    result.selected += 1;
                    state.candidates.fill(None);
                }
            }
            result.background += usize::from(state.store.pending() > 0);
        }
        let was_live = engine
            .frontier
            .front()
            .is_some_and(|s| s.store.view.locations.contains_key(&Occurrence(1)));
        result.turns += 1;
        match engine.advance() {
            Step::Answer(a) => result.answers.push(a),
            Step::Exhausted => return result,
            Step::Progress => (),
        }
        if was_live
            && let Some(s) = engine.frontier.front()
            && !s.store.view.locations.contains_key(&Occurrence(1))
        {
            result.pending_at_claim.push(s.store.pending());
        }
    }
    panic!("readiness service cutoff")
}
#[test]
fn matcher_settlement_preserves_the_priority_witness() {
    for depth in [4, 16, 64] {
        for early in [false, true] {
            for clash in [false, true] {
                for tokens in [1, 2] {
                    let (rules, query) = priority_source(depth, early, clash, tokens);
                    let expected = oracle::run(&rules, &query, 100_000);
                    let r = run_relevant(&rules, &query);
                    oracle::same_raw(r.answers, expected);
                    println!(
                        "READY_PRIORITY,depth={depth},early={early},clash={clash},tokens={tokens},background={},selected={}",
                        r.background, r.selected
                    );
                }
            }
        }
    }
}

#[test]
fn matcher_settlement_separates_output_work_from_competing_reads() {
    for depth in [4, 16, 64] {
        for possible in [false, true] {
            for shared in [false, true] {
                for outcome in ["success", "fail", "clash"] {
                    for tokens in [1, 2] {
                        let (rules, query) =
                            separated_source(depth, possible, shared, outcome, tokens);
                        let expected = oracle::run(&rules, &query, 100_000);
                        let (_, full, _, full_turns) = run_observed_counted(
                            Prepared::new(&rules).unwrap().start(&query),
                            true,
                        );
                        let r = run_relevant(&rules, &query);
                        oracle::same_raw(r.answers, expected);
                        if !shared {
                            assert!(
                                r.pending_at_claim.iter().any(|n| *n > 0),
                                "separate tail must remain pending at claim"
                            );
                            if outcome == "fail" {
                                assert!(
                                    r.background + r.selected < full,
                                    "early source failure must save actual deductions"
                                );
                            }
                        }
                        if shared && outcome != "clash" {
                            assert_eq!(r.pending_at_claim, vec![0]);
                        }
                        println!(
                            "READY_SEPARATE,depth={depth},possible={possible},shared={shared},outcome={outcome},tokens={tokens},background={},selected={},full={full},pending={:?},turns={},full_turns={full_turns}",
                            r.background, r.selected, r.pending_at_claim, r.turns
                        );
                    }
                }
            }
        }
    }
}
#[test]
fn matcher_settlement_covers_new_posts_constructors_and_repeated_variables() {
    for depth in [4, 16, 64] {
        for kind in ["post", "constructor", "repeated"] {
            let (mut rules, mut query) = priority_source(depth, false, false, 1);
            match kind {
                "post" => {
                    let args = vec![
                        query.constraints[0].args[0].clone(),
                        query.constraints[0].args[1].clone(),
                        v(10),
                        v(11),
                        v(12),
                    ];
                    rules[0] = Rule::simplify(
                        "spawn",
                        [c("spawn", [v(0), v(1), v(2), v(3), v(4)])],
                        chr_syntax::and([
                            eq(v(0), v(1)),
                            c("use", [v(2), v(3), v(4)]).into(),
                            c("token", []).into(),
                        ]),
                    );
                    query.constraints = vec![c("spawn", args)];
                }
                "constructor" => {
                    rules[1].removed[0] = c("use", [atom("a"), v(1), v(2)]);
                    rules[1].guards.clear();
                    rules[2].guards.clear();
                }
                "repeated" => {
                    rules[1].removed[0] = c("use", [v(0), v(0), v(2)]);
                    rules[1].guards.clear();
                    rules[2].guards.clear();
                    query.constraints[0].args = vec![nest(depth, v(11)), nest(depth, atom("a"))];
                    query.constraints[1].args[0] = atom("a");
                }
                _ => unreachable!(),
            }
            let expected = oracle::run(&rules, &query, 100_000);
            assert_eq!(expected[0].outputs[0].1, atom("older"));
            let r = run_relevant(&rules, &query);
            oracle::same_raw(r.answers, expected);
            println!(
                "READY_READ,depth={depth},kind={kind},background={},selected={}",
                r.background, r.selected
            );
        }
    }
}

#[test]
fn matcher_settlement_handles_shared_and_disjoint_consumption() {
    for depth in [4, 16, 64] {
        for shared in [false, true] {
            for clash in [false, true] {
                let mut older = Rule::simplify(
                    "older",
                    [
                        c("older_request", [v(0), v(1)]),
                        c(if shared { "token" } else { "older_token" }, []),
                    ],
                    eq(v(1), atom("older")),
                );
                older.guards.push(chr_syntax::Guard::Equal(v(0), atom("b")));
                let mut newer = Rule::simplify(
                    "newer",
                    [
                        c("newer_request", [v(0), v(1)]),
                        c(if shared { "token" } else { "newer_token" }, []),
                    ],
                    eq(v(1), atom("newer")),
                );
                newer.guards.push(chr_syntax::Guard::Equal(v(0), atom("a")));
                let rules = vec![
                    Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
                    older,
                    newer,
                ];
                let mut constraints = vec![
                    c(
                        "bind",
                        [
                            t(
                                "pair",
                                [v(10), nest(depth, if clash { atom("bad") } else { v(11) })],
                            ),
                            t("pair", [atom("a"), nest(depth, atom("b"))]),
                        ],
                    ),
                    c("older_request", [v(11), v(12)]),
                    c("newer_request", [v(10), v(13)]),
                ];
                if shared {
                    constraints.push(c("token", []));
                } else {
                    constraints.extend([c("older_token", []), c("newer_token", [])]);
                }
                let query = Query {
                    constraints,
                    outputs: vec![("older".into(), Var(12)), ("newer".into(), Var(13))],
                };
                let expected = oracle::run(&rules, &query, 100_000);
                oracle::same_raw(run_relevant(&rules, &query).answers, expected);
                println!("READY_RESOURCE,depth={depth},shared={shared},clash={clash}");
            }
        }
    }
}
#[test]
fn matcher_settlement_cancellation_does_not_contaminate_prepared_reuse() {
    for outcome in ["success", "fail", "clash"] {
        let (rules, query) = separated_source(64, false, false, outcome, 2);
        let prepared = Prepared::new(&rules).unwrap();
        let reads = crate::store::MatcherReads::new(&rules);
        let mut cancelled = prepared.start(&query);
        for _ in 0..1000 {
            if let Some(state) = cancelled.frontier.front_mut()
                && state.pending.is_empty()
            {
                while state.store.step_for_matching(&reads) {
                    state.candidates.fill(None);
                }
            }
            assert!(matches!(cancelled.advance(), Step::Progress));
            if cancelled
                .frontier
                .front()
                .is_some_and(|s| !s.store.view.locations.contains_key(&Occurrence(1)))
            {
                break;
            }
        }
        assert!(cancelled.frontier.front().is_some_and(
            |s| s.store.pending() > 0 && !s.store.view.locations.contains_key(&Occurrence(1))
        ));
        drop(cancelled);
        assert_eq!(std::sync::Arc::strong_count(&prepared), 1);
        let again = drive_relevant(prepared.start(&query), &reads);
        oracle::same_raw(again.answers, oracle::run(&rules, &query, 100_000));
        println!("READY_CANCEL,outcome={outcome}");
    }
}

fn run_batched(rules: &[Rule], query: &Query, budget: usize) -> (Vec<Answer>, usize, usize) {
    let mut engine = Prepared::new_ready(rules).unwrap().start(query);
    let mut answers = vec![];
    let mut total_work = 0;
    for turn in 1..=100_000 {
        let (step, work) = engine.advance_scheduled::<true>(true, budget);
        total_work += work;
        assert!(work <= budget, "deduction budget exceeded");
        match step {
            Step::Answer(a) => answers.push(a),
            Step::Exhausted => return (answers, turn, total_work),
            Step::Progress => (),
        }
    }
    panic!("bounded drain service cutoff")
}

#[test]
fn bounded_drain_preserves_priority_and_avoids_repeated_idle_advances() {
    for depth in [4, 16, 64] {
        for possible in [false, true] {
            for shared in [false, true] {
                for outcome in ["success", "fail", "clash"] {
                    for tokens in [1, 2] {
                        let (rules, query) =
                            separated_source(depth, possible, shared, outcome, tokens);
                        let expected = oracle::run(&rules, &query, 100_000);
                        for budget in [1, 8, 256] {
                            let (answers, turns, work) = run_batched(&rules, &query, budget);
                            oracle::same_raw(answers, expected.clone());
                            if !shared && outcome == "fail" {
                                assert_eq!(work, 4, "do not drain across pending source failure");
                            }
                            if depth == 64 && !shared && outcome == "success" && budget == 256 {
                                assert!(
                                    turns < 72,
                                    "quiescent output must avoid repeated source advances"
                                );
                            }
                            println!(
                                "DRAIN,depth={depth},possible={possible},shared={shared},outcome={outcome},tokens={tokens},budget={budget},turns={turns},work={work}"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn bounded_drain_cancellation_reuse_and_branch_service() {
    for outcome in ["success", "fail", "clash"] {
        let (rules, query) = separated_source(64, false, false, outcome, 2);
        let prepared = Prepared::new_ready(&rules).unwrap();
        for budget in [1, 8, 256] {
            let mut engine = prepared.start(&query);
            for _ in 0..1000 {
                assert!(matches!(engine.advance_ready(budget), Step::Progress));
                if engine
                    .frontier
                    .front()
                    .is_some_and(|s| !s.store.view.locations.contains_key(&Occurrence(1)))
                {
                    break;
                }
            }
            assert!(
                engine
                    .frontier
                    .front()
                    .is_some_and(|s| s.store.pending() > 0
                        && !s.store.view.locations.contains_key(&Occurrence(1)))
            );
            drop(engine);
            assert_eq!(Arc::strong_count(&prepared), 1);
            let mut reused = prepared.start(&query);
            let mut answers = vec![];
            let mut complete = false;
            for _ in 0..100_000 {
                match reused.advance_ready(budget) {
                    Step::Answer(a) => answers.push(a),
                    Step::Exhausted => {
                        complete = true;
                        break;
                    }
                    Step::Progress => (),
                }
            }
            assert!(complete);
            oracle::same_raw(answers, oracle::run(&rules, &query, 100_000));
            println!("DRAIN_CANCEL,outcome={outcome},budget={budget}");
        }
    }
    let rules = vec![Rule::simplify(
        "start",
        [c("start", [v(0)])],
        Goal::Or(
            Box::new(chr_syntax::and([
                eq(nest(64, v(1)), nest(64, atom("end"))),
                eq(v(0), atom("long")),
            ])),
            Box::new(eq(v(0), atom("short"))),
        ),
    )];
    let query = Query {
        constraints: vec![c("start", [v(10)])],
        outputs: vec![("branch".into(), Var(10))],
    };
    for budget in [1, 8, 256] {
        let (answers, turns, work) = run_batched(&rules, &query, budget);
        assert_eq!(answers[0].outputs[0].1, atom("short"));
        oracle::same_raw(answers, oracle::run(&rules, &query, 100_000));
        println!("DRAIN_BRANCH,budget={budget},turns={turns},work={work}");
    }
}

#[test]
fn public_settlement_controls_match_qualified_controllers() {
    for depth in [4, 64] {
        for shared in [false, true] {
            for outcome in ["success", "fail", "clash"] {
                let (rules, query) = separated_source(depth, true, shared, outcome, 2);
                for selective in [false, true] {
                    let prepared = if selective {
                        Prepared::new_ready(&rules)
                    } else {
                        Prepared::new(&rules)
                    }
                    .unwrap();
                    let mut engine = prepared.start(&query);
                    let mut answers = vec![];
                    let mut done = false;
                    for _ in 0..100_000 {
                        match if selective {
                            engine.advance_selective()
                        } else {
                            engine.advance_settled()
                        } {
                            Step::Answer(a) => answers.push(a),
                            Step::Exhausted => {
                                done = true;
                                break;
                            }
                            Step::Progress => (),
                        }
                    }
                    assert!(done);
                    oracle::same_raw(answers, oracle::run(&rules, &query, 100_000));
                }
            }
        }
    }
}
