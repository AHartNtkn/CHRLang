use chr_reference::Search;
#[test]
fn reference_observations_match_independent_registered_expectations() {
    for case in chr_cases::registry() {
        let mut search = Search::new(case.rules, case.query).unwrap();
        let mut actual = vec![];
        let mut exhausted = false;
        for _ in 0..case.budget {
            let batch = search.advance(1);
            actual.extend(batch.answers);
            exhausted = batch.exhausted;
            if exhausted || case.answer_limit.is_some_and(|n| actual.len() >= n) {
                break;
            }
        }
        actual.sort();
        let mut expected = case.expected;
        expected.sort();
        assert_eq!(actual, expected, "{} answers", case.id);
        assert_eq!(exhausted, case.exhausted, "{} exhaustion", case.id);
        assert_eq!(
            search.stats().completed_branches,
            case.raw_answers,
            "{} raw success multiplicity",
            case.id
        );
    }
}
#[test]
fn carry_counts_charge_independent_work_after_each_choice() {
    for k in [0, 1, 2, 3] {
        for w in [0, 1, 4] {
            for noise in [0, 3] {
                let case = chr_cases::carry_case(k, w, noise);
                let mut search = Search::new(case.rules, case.query).unwrap();
                assert!(search.advance(case.budget).exhausted);
                assert_eq!(
                    search.stats().rule_applications,
                    ((1u64 << k) - 1) + (1u64 << k) * w as u64
                );
                assert_eq!(search.stats().splits, (1u64 << k) - 1);
            }
        }
    }
}

#[test]
fn scalar_microtrace_counts_introduction_application_equation_and_completion() {
    let case = chr_cases::semantic_cases()
        .into_iter()
        .find(|c| c.id == "U02-self")
        .unwrap();
    let mut s = Search::new(case.rules, case.query).unwrap();
    assert!(s.advance(4).exhausted);
    let x = s.stats();
    assert_eq!(
        (
            x.steps,
            x.introductions,
            x.rule_applications,
            x.equations,
            x.unification_pairs,
            x.completed_branches
        ),
        (4, 1, 1, 1, 1, 1)
    );
}
