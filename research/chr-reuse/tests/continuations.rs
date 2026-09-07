use chr_reuse::continuations::{Mode, Search};
#[test]
fn both_modes_preserve_every_registered_prefix_and_raw_lineage() {
    for mode in [Mode::Direct, Mode::ExactIds] {
        for case in chr_cases::registry() {
            let mut search = Search::new(case.rules, case.query, mode).unwrap();
            let mut answers = vec![];
            let mut exhausted = false;
            for _ in 0..case.budget {
                let batch = search.advance(1);
                answers.extend(batch.answers);
                exhausted = batch.exhausted;
                if exhausted || case.answer_limit.is_some_and(|n| answers.len() >= n) {
                    break;
                }
            }
            assert_eq!(exhausted, case.exhausted, "{} {mode:?}", case.id);
            assert_eq!(
                search.stats().completed,
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
        }
    }
}
#[test]
fn reconvergent_continuation_keeps_each_later_choice_lineage() {
    use chr_syntax::{Goal, Rule, and, atom, c, eq, or, v};
    let rules = vec![Rule::simplify(
        "start",
        [c("start", [v(0)])],
        and([
            or(Goal::True, Goal::True),
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ]),
    )];
    let query = chr_cases::query(vec![c("start", [v(7)])], &[7]);
    let mut direct = Search::new(rules.clone(), query.clone(), Mode::Direct).unwrap();
    let mut memo = Search::new(rules, query, Mode::ExactIds).unwrap();
    let d = direct.advance(100);
    let m = memo.advance(100);
    assert!(d.exhausted && m.exhausted);
    assert_eq!(d.answers, m.answers);
    assert_eq!(m.answers.len(), 2);
    assert_eq!(memo.stats().completed, 4);
    assert_eq!(direct.stats().completed, 4);
    assert_eq!(memo.stats().logical_steps, direct.stats().logical_steps);
    assert!(memo.stats().executed < direct.stats().executed);
    assert!(memo.stats().hits > 0);
}

#[test]
fn aliased_and_independent_continuations_have_different_futures() {
    use chr_syntax::{Rule, and, atom, c, eq, or, v};
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0), v(1)])],
            or(c("p", [v(0), v(0)]).into(), c("p", [v(0), v(1)]).into()),
        ),
        Rule::simplify(
            "p",
            [c("p", [v(0), v(1)])],
            and([eq(v(0), atom("a")), eq(v(1), atom("b"))]),
        ),
    ];
    let query = chr_cases::query(vec![c("start", [v(0), v(1)])], &[0, 1]);
    let mut reference = chr_reference::Search::new(rules.clone(), query.clone()).unwrap();
    let expected = reference.advance(100);
    assert!(expected.exhausted);
    assert_eq!(expected.answers.len(), 1);
    for mode in [Mode::Direct, Mode::ExactIds] {
        let mut search = Search::new(rules.clone(), query.clone(), mode).unwrap();
        let actual = search.advance(100);
        assert!(actual.exhausted);
        assert_eq!(actual.answers.len(), 1);
        assert!(chr_observe::equivalent(
            &actual.answers[0],
            &expected.answers[0],
            &mut chr_observe::Stats::default()
        ));
        assert_eq!(search.stats().failed, 1);
    }
}

#[path = "../examples/support/cases.rs"]
mod measured_cases;
#[test]
fn duplicated_choice_workloads_match_independent_reference_and_keep_raw_counts() {
    for case in measured_cases::cases()
        .into_iter()
        .filter(|c| c.id.starts_with("duplicate-"))
    {
        let mut reference =
            chr_reference::Search::new(case.rules.clone(), case.query.clone()).unwrap();
        let expected = reference.advance(case.budget);
        assert!(expected.exhausted);
        assert_eq!(reference.stats().completed_branches, case.raw_answers);
        assert_eq!(expected.answers.len(), 1);
        assert!(chr_observe::equivalent(
            &expected.answers[0],
            &case.expected[0],
            &mut chr_observe::Stats::default()
        ));
        for mode in [Mode::Direct, Mode::ExactIds] {
            let mut search = Search::new(case.rules.clone(), case.query.clone(), mode).unwrap();
            let actual = search.advance(case.budget);
            assert!(actual.exhausted);
            assert_eq!(search.stats().completed, case.raw_answers);
            assert_eq!(actual.answers.len(), 1);
            assert!(chr_observe::equivalent(
                &actual.answers[0],
                &expected.answers[0],
                &mut chr_observe::Stats::default()
            ));
        }
    }
}
