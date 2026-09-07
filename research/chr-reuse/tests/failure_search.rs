use chr_reuse::failure_search::{Mode, Search};
#[test]
fn learning_preserves_registered_prefixes_answers_and_failure_counts() {
    for case in chr_cases::registry() {
        let mut expected = None;
        for mode in [Mode::Direct, Mode::Learn] {
            let mut search = Search::new(case.rules.clone(), case.query.clone(), mode).unwrap();
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
                    )))
            );
            let counts = (
                search.stats().logical_steps,
                search.stats().failed,
                search.stats().completed,
            );
            if let Some(expected) = expected {
                assert_eq!(counts, expected, "{}", case.id);
            } else {
                expected = Some(counts);
            }
        }
    }
}
#[test]
fn learned_failure_never_prunes_an_unposted_equation() {
    use chr_syntax::{Goal, Rule, and, atom, c, eq, or, v};
    // First arm learns a clash; second arm contains the same rigid arguments in
    // an ordinary residual constraint whose equation-producing rule cannot win.
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [])],
            or(
                eq(atom("a"), atom("b")),
                and([
                    c("safe", [atom("a"), atom("b")]).into(),
                    or(eq(atom("c"), atom("d")), Goal::True),
                ]),
            ),
        ),
        Rule::simplify("consume-first", [c("safe", [v(0), v(1)])], Goal::True),
        Rule::simplify(
            "would-post-clash",
            [c("safe", [v(0), v(1)])],
            eq(v(0), v(1)),
        ),
    ];
    let query = chr_cases::query(vec![c("start", [])], &[]);
    let mut search = Search::new(rules.clone(), query.clone(), Mode::Learn).unwrap();
    let actual = search.advance(100);
    assert!(actual.exhausted);
    assert_eq!(actual.answers.len(), 1);
    assert_eq!(search.stats().failed, 2);
    assert!(search.proof_stats().hits > 0);
    let mut reference = chr_reference::Search::new(rules, query).unwrap();
    let expected = reference.advance(100);
    assert!(expected.exhausted);
    assert_eq!(actual.answers, expected.answers);
}

#[path = "../examples/support/failure_cases.rs"]
mod measured_cases;
#[test]
fn registered_structural_failure_grid_matches_independent_execution() {
    for case in measured_cases::cases()
        .into_iter()
        .filter(|c| c.id.starts_with("failure-"))
    {
        let mut reference =
            chr_reference::Search::new(case.rules.clone(), case.query.clone()).unwrap();
        let expected = reference.advance(case.budget);
        assert!(expected.exhausted);
        assert!(expected.answers.is_empty());
        for mode in [Mode::Direct, Mode::Learn] {
            let mut search = Search::new(case.rules.clone(), case.query.clone(), mode).unwrap();
            let actual = search.advance(case.budget);
            assert!(actual.exhausted);
            assert!(actual.answers.is_empty());
            assert_eq!(search.stats().failed, reference.stats().failed_branches);
            assert_eq!(search.stats().logical_steps, reference.stats().steps);
        }
    }
}

#[test]
fn failure_workloads_exercise_the_registered_unification_work() {
    for case in measured_cases::cases()
        .into_iter()
        .filter(|c| c.id.starts_with("failure-"))
    {
        let parts = case.id.split('-').collect::<Vec<_>>();
        let k = parts[2][1..].parse::<u32>().unwrap();
        let depth = parts[3][1..].parse::<u64>().unwrap();
        let mut search = Search::new(case.rules, case.query, Mode::Direct).unwrap();
        assert!(search.advance(case.budget).exhausted);
        let repetitions = 1u64 << k;
        match parts[1] {
            "early" => assert_eq!(search.source_stats().pairs, 2 * repetitions),
            "late" => assert_eq!(search.source_stats().pairs, (depth + 3) * repetitions),
            "occurs" => assert_eq!(
                search.source_stats().occurs_visits,
                (depth + 3) * repetitions
            ),
            _ => panic!("registered family"),
        }
    }
}
