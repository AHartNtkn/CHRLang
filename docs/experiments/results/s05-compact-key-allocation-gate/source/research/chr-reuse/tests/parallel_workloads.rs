#[path = "../examples/support/parallel_cases.rs"]
mod cases;

#[test]
fn pilot_workloads_have_independent_reference_answers() {
    for id in cases::ids() {
        let case = cases::case(&id);
        let mut search = chr_reference::Search::new(case.rules, case.query).unwrap();
        let mut answers = Vec::new();
        let mut exhausted = false;
        for _ in 0..case.budget {
            let batch = search.advance(1);
            answers.extend(batch.answers);
            exhausted = batch.exhausted;
            if exhausted || case.answer_limit.is_some_and(|n| answers.len() >= n) {
                break;
            }
        }
        assert_eq!(exhausted, case.exhausted, "{id}");
        assert_eq!(search.stats().completed_branches, case.raw_answers, "{id}");
        assert_eq!(answers.len(), case.expected.len(), "{id}");
        for expected in &case.expected {
            assert!(
                answers.iter().any(|a| chr_observe::equivalent(
                    a,
                    expected,
                    &mut Default::default()
                )),
                "{id}: {expected:?}"
            );
        }
        if id == "mixed" {
            assert_eq!(search.stats().failed_branches, 5);
        }
    }
}

#[test]
fn prefix_pilot_leaves_accepted_equations_for_shutdown() {
    use chr_reuse::parallel_equations::{Mode, Search};
    for mode in [Mode::Inline, Mode::Threads(1), Mode::Threads(2)] {
        let case = cases::case("prefix-drain");
        let mut search = Search::new(case.rules, case.query, mode, 4, 8).unwrap();
        let mut found = false;
        for _ in 0..case.budget {
            if !search.advance(1).unwrap().answers.is_empty() {
                found = true;
                break;
            }
        }
        assert!(found);
        assert!(search.stats().issued > search.stats().committed_equations);
        search.shutdown().unwrap();
        assert!(search.stats().uncommitted_at_shutdown > 0);
    }
}
