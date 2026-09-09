use chr_reuse::equation_search::{Mode, Search};
#[test]
fn all_modes_preserve_registered_observations_and_prefixes() {
    for case in chr_cases::registry() {
        let mut counts = None;
        for mode in [Mode::Shared, Mode::Owned, Mode::Memo] {
            let mut s = Search::new(case.rules.clone(), case.query.clone(), mode).unwrap();
            let mut answers = vec![];
            let mut exhausted = false;
            for _ in 0..case.budget {
                let b = s.advance(1);
                answers.extend(b.answers);
                exhausted = b.exhausted;
                if exhausted || case.answer_limit.is_some_and(|n| answers.len() >= n) {
                    break;
                }
            }
            assert_eq!(exhausted, case.exhausted, "{} {mode:?}", case.id);
            assert_eq!(
                s.stats().completed,
                case.raw_answers,
                "{} {mode:?}",
                case.id
            );
            assert_eq!(answers.len(), case.expected.len(), "{} {mode:?}", case.id);
            assert!(
                case.expected
                    .iter()
                    .all(|e| answers.iter().any(|a| chr_observe::equivalent(
                        a,
                        e,
                        &mut chr_observe::Stats::default()
                    ))),
                "{} {mode:?}",
                case.id
            );
            let got = (s.stats().steps, s.stats().failed, s.stats().completed);
            if let Some(expected) = counts {
                assert_eq!(got, expected, "{} {mode:?}", case.id);
            } else {
                counts = Some(got);
            }
        }
    }
}
#[test]
fn cached_bindings_wake_suspended_constraints_and_preserve_other_aliases() {
    use chr_syntax::{Goal, Rule, atom, c, eq, or, t, v};
    let rules = vec![
        Rule::simplify(
            "bind",
            [c("bind", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("a"))),
        ),
        Rule::simplify(
            "awake",
            [c("wait", [atom("a"), v(0)])],
            c("seen", [v(0)]).into(),
        ),
        Rule::simplify("check", [c("guarded", [t("f", [atom("b")])])], Goal::Fail),
    ];
    let query = chr_cases::query(
        vec![
            c("wait", [v(0), v(1)]),
            c("alias", [v(0), v(1), v(1)]),
            c("bind", [v(0)]),
            c("guarded", [t("f", [v(0)])]),
        ],
        &[0, 1],
    );
    let mut reference = chr_reference::Search::new(rules.clone(), query.clone()).unwrap();
    let expected = reference.advance(100);
    assert!(expected.exhausted);
    assert_eq!(expected.answers.len(), 1);
    for mode in [Mode::Shared, Mode::Owned, Mode::Memo] {
        let mut s = Search::new(rules.clone(), query.clone(), mode).unwrap();
        let actual = s.advance(100);
        assert!(actual.exhausted);
        assert_eq!(actual.answers.len(), 1);
        assert!(chr_observe::equivalent(
            &actual.answers[0],
            &expected.answers[0],
            &mut chr_observe::Stats::default()
        ));
        assert_eq!(s.stats().completed, 2);
        if matches!(mode, Mode::Memo) {
            assert_eq!(s.operation_stats().hits, 1);
        }
    }
}

#[path = "../examples/support/equation_cases.rs"]
mod measured_cases;
#[test]
fn repeated_unique_and_identity_workloads_have_registered_results_and_hits() {
    for case in measured_cases::cases()
        .into_iter()
        .filter(|c| c.id.starts_with("equation-"))
    {
        let mut reference =
            chr_reference::Search::new(case.rules.clone(), case.query.clone()).unwrap();
        let expected = reference.advance(case.budget);
        assert!(expected.exhausted, "{}", case.id);
        assert_eq!(expected.answers.len(), 1);
        assert!(chr_observe::equivalent(
            &expected.answers[0],
            &case.expected[0],
            &mut chr_observe::Stats::default()
        ));
        let parts = case.id.split('-').collect::<Vec<_>>();
        let count = parts[2][1..].parse::<u64>().unwrap();
        for mode in [Mode::Shared, Mode::Owned, Mode::Memo] {
            let mut s = Search::new(case.rules.clone(), case.query.clone(), mode).unwrap();
            let actual = s.advance(case.budget);
            assert!(actual.exhausted);
            assert_eq!(actual.answers.len(), 1);
            assert!(
                chr_observe::equivalent(
                    &actual.answers[0],
                    &expected.answers[0],
                    &mut chr_observe::Stats::default()
                ),
                "{} {mode:?}",
                case.id
            );
            if matches!(mode, Mode::Memo) {
                assert_eq!(s.operation_stats().calls, count);
                assert_eq!(
                    s.operation_stats().hits,
                    if parts[1] == "unique" { 0 } else { count - 1 }
                );
            }
        }
    }
}
