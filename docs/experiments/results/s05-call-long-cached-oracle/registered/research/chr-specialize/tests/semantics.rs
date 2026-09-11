use chr_specialize::specialize;
use chr_syntax::*;
fn run(rules: Vec<Rule>, q: Query) -> chr_reference::Batch {
    chr_reference::Search::new(rules, q).unwrap().advance(10000)
}
#[test]
fn registered_arithmetic_modes_preserve_every_finite_answer() {
    for case in chr_cases::application_cases()
        .into_iter()
        .filter(|c| c.id.starts_with("app-add") || c.id == "app-sub")
    {
        for budget in [0, 1, 2, 4, 8] {
            let p = specialize(&case.rules, &case.query, budget).unwrap();
            let actual = run(p.rules, p.query);
            assert!(actual.exhausted, "{} budget {budget}", case.id);
            assert_eq!(actual.answers.len(), case.expected.len());
            assert!(
                case.expected.iter().all(|e| actual
                    .answers
                    .iter()
                    .any(|a| chr_observe::equivalent(a, e, &mut chr_observe::Stats::default()))),
                "{} budget {budget}",
                case.id
            );
        }
    }
}
#[test]
fn output_shape_keeps_zero_arm_and_unknown_caller_aliases() {
    let q = chr_cases::query(
        vec![
            c("add", [v(0), v(1), t("s", [v(2)])]),
            c("passive", [v(0), v(1), v(2)]),
        ],
        &[0, 1, 2],
    );
    let p = specialize(&chr_programs::arithmetic(), &q, 8).unwrap();
    let mut search = chr_reference::Search::new(p.rules, p.query).unwrap();
    let a = search.advance(100).answers;
    assert!(
        a.iter().any(|a| a.outputs[0].1 == atom("z")
            && matches!(a.outputs[1].1,Term::App(ref n,_) if n=="s"))
    );
}
#[test]
fn non_tail_inlining_has_a_real_operational_counterexample() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [])],
            and([c("p", []).into(), c("q", []).into()]),
        ),
        Rule::simplify("p", [c("p", [])], c("p", []).into()),
        Rule::simplify("q", [c("q", [])], Goal::Fail),
    ];
    let q = chr_cases::query(vec![c("start", [])], &[]);
    assert!(!run(rules.clone(), q.clone()).exhausted);
    let mut inlined = rules.clone();
    inlined[0].body = and([c("p", []).into(), Goal::Fail]);
    let changed = run(inlined, q.clone());
    assert!(changed.exhausted);
    assert!(changed.answers.is_empty());
    assert!(specialize(&rules, &q, 8).is_err());
}
#[test]
fn another_consumer_invalidates_unique_expansion() {
    let mut rules = chr_programs::arithmetic();
    rules.push(Rule::simplify(
        "intercept",
        [c("add", [v(0), v(1), v(2)])],
        Goal::Fail,
    ));
    assert!(
        specialize(
            &rules,
            &chr_cases::query(vec![c("add", [atom("z"), atom("z"), v(0)])], &[0]),
            4
        )
        .is_err()
    );
}

#[test]
fn branch_bindings_fresh_variables_and_duplicate_derivations_are_preserved() {
    let rules = vec![Rule::simplify(
        "choose",
        [c("choose", [v(0)])],
        or(
            and([eq(v(0), t("f", [v(1)])), eq(v(1), atom("a"))]),
            or(
                and([eq(v(0), t("f", [v(2)])), eq(v(2), atom("b"))]),
                eq(v(0), t("f", [atom("a")])),
            ),
        ),
    )];
    let q = chr_cases::query(vec![c("choose", [v(90)]), c("__spec_0", [v(90)])], &[90]);
    let mut reference = chr_reference::Search::new(rules.clone(), q.clone()).unwrap();
    let expected = reference.advance(100);
    assert!(expected.exhausted);
    assert_eq!(reference.stats().completed_branches, 3);
    for budget in [1, 2, 4, 8] {
        let p = specialize(&rules, &q, budget).unwrap();
        let mut s = chr_reference::Search::new(p.rules, p.query).unwrap();
        let actual = s.advance(100);
        assert!(actual.exhausted);
        assert_eq!(s.stats().completed_branches, 3);
        assert_eq!(actual.answers.len(), 2);
        assert!(expected.answers.iter().all(|e| {
            actual
                .answers
                .iter()
                .any(|a| chr_observe::equivalent(a, e, &mut chr_observe::Stats::default()))
        }));
    }
}
#[test]
fn unused_active_failure_and_recursion_remain_active() {
    for body in [Goal::Fail, c("loop", []).into()] {
        let rules = vec![
            Rule::simplify("start", [c("start", [])], c("loop", []).into()),
            Rule::simplify("loop", [c("loop", [])], body.clone()),
        ];
        let q = chr_cases::query(vec![c("start", [])], &[]);
        for budget in [0, 1, 2, 8] {
            let p = specialize(&rules, &q, budget).unwrap();
            assert!(p.stats.expansions <= budget);
            let a = run(p.rules, p.query);
            assert!(a.answers.is_empty());
            assert_eq!(a.exhausted, matches!(body, Goal::Fail));
        }
    }
}
#[test]
fn equations_cannot_create_cyclic_compile_time_terms() {
    let rules = vec![Rule::simplify(
        "cycle",
        [c("p", [v(0), v(1)])],
        and([eq(v(0), v(1)), eq(v(1), t("f", [v(0)]))]),
    )];
    let q = chr_cases::query(vec![c("p", [v(0), v(1)])], &[0, 1]);
    let p = specialize(&rules, &q, 2).unwrap();
    assert_eq!(p.stats.contradictions, 1);
    let a = run(p.rules, p.query);
    assert!(a.exhausted);
    assert!(a.answers.is_empty());
}

#[path = "../examples/support/cases.rs"]
mod measured_cases;
#[test]
fn registered_cost_workloads_preserve_observations_on_scalar_and_reference() {
    for case in measured_cases::cases() {
        if case.id.starts_with("specialize-") {
            let expected = run(case.rules.clone(), case.query.clone());
            assert!(expected.exhausted);
            assert_eq!(expected.answers.len(), case.expected.len());
            assert!(case.expected.iter().all(|e| {
                expected
                    .answers
                    .iter()
                    .any(|a| chr_observe::equivalent(a, e, &mut chr_observe::Stats::default()))
            }));
        }
        for budget in [0, 1, 2, 4, 8, 16] {
            let p = specialize(&case.rules, &case.query, budget).unwrap();
            assert!(p.stats.expansions <= budget);
            let mut s =
                chr_persistent::Search::new(p.rules, p.query, chr_persistent::Snapshot::Persistent)
                    .unwrap();
            let mut actual = vec![];
            let mut exhausted = false;
            for _ in 0..case.budget {
                let b = s.advance(1);
                actual.extend(b.answers);
                exhausted = b.exhausted;
                if exhausted || case.answer_limit.is_some_and(|n| actual.len() >= n) {
                    break;
                }
            }
            assert_eq!(exhausted, case.exhausted, "{} {budget}", case.id);
            assert_eq!(
                s.stats().completed,
                case.raw_answers,
                "{} {budget}",
                case.id
            );
            assert_eq!(actual.len(), case.expected.len(), "{} {budget}", case.id);
            assert!(
                case.expected
                    .iter()
                    .all(|e| actual.iter().any(|a| chr_observe::equivalent(
                        a,
                        e,
                        &mut chr_observe::Stats::default()
                    ))),
                "{} {budget}",
                case.id
            );
        }
    }
}
