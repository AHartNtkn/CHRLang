use chr_conditional::{Grouping, Search};
use chr_observe::{Stats, equivalent};
#[test]
fn conditional_projections_preserve_registered_answers_and_raw_multiplicity() {
    for grouping in [Grouping::Singleton, Grouping::Ready] {
        for case in chr_cases::registry() {
            let mut s = Search::new(case.rules, case.query, grouping).unwrap();
            let mut actual = vec![];
            let mut exhausted = false;
            s.validate().unwrap();
            for _ in 0..case.budget {
                let b = s.advance(1);
                actual.extend(b.answers);
                exhausted = b.exhausted;
                s.validate().unwrap();
                if exhausted || case.answer_limit.is_some_and(|n| actual.len() >= n) {
                    break;
                }
            }
            assert_eq!(
                actual.len(),
                case.expected.len(),
                "{} {grouping:?}",
                case.id
            );
            for wanted in &case.expected {
                assert!(
                    actual
                        .iter()
                        .any(|got| equivalent(got, wanted, &mut Stats::default())),
                    "{} {grouping:?}: missing {wanted:?}; got {actual:?}",
                    case.id
                );
            }
            assert_eq!(exhausted, case.exhausted, "{} completion", case.id);
            assert_eq!(
                s.stats().completed,
                case.raw_answers,
                "{} raw alternatives",
                case.id
            );
        }
    }
}
#[test]
fn shared_opaque_expansion_is_counted_separately_from_projection_work() {
    for grouping in [Grouping::Singleton, Grouping::Ready] {
        let case = chr_cases::carry_case(3, 4, 16);
        let mut s = Search::new(case.rules, case.query, grouping).unwrap();
        let b = s.advance(1000);
        assert!(b.exhausted);
        assert_eq!(b.answers.len(), 8);
        assert_eq!(s.stats().projected_applications, 39);
        assert_eq!(
            s.stats().expansions,
            match grouping {
                Grouping::Singleton => 39,
                Grouping::Ready => 7,
            }
        );
        assert_eq!(s.stats().splits, 7);
        assert!(s.stats().support_reads > 0);
    }
}

#[test]
fn a_later_equal_branch_keeps_its_own_propagation_permission() {
    use chr_syntax::{Rule, atom, c, eq, or, v};
    let rules = vec![
        Rule::propagate(
            "emit",
            [c("p", [v(0)]), c("q", [atom("a")])],
            c("mark", [v(0)]).into(),
        ),
        Rule::simplify(
            "split",
            [c("split", [v(0)])],
            or(eq(v(0), atom("a")), c("later", [v(0)]).into()),
        ),
        Rule::simplify("later", [c("later", [v(0)])], eq(v(0), atom("a"))),
    ];
    for grouping in [Grouping::Singleton, Grouping::Ready] {
        let query = chr_cases::query(
            vec![c("p", [v(0)]), c("q", [v(1)]), c("split", [v(1)])],
            &[0, 1],
        );
        let mut s = Search::new(rules.clone(), query, grouping).unwrap();
        let result = s.advance(1000);
        assert!(result.exhausted);
        assert_eq!(result.answers.len(), 1);
        assert_eq!(s.stats().completed, 2);
        let expected = chr_cases::answer(
            vec![v(0), atom("a")],
            vec![c("p", [v(0)]), c("q", [atom("a")]), c("mark", [v(0)])],
        );
        assert!(equivalent(
            &result.answers[0],
            &expected,
            &mut Stats::default()
        ));
    }
}

#[test]
fn every_unserved_projection_is_unchanged_by_a_conditional_quantum() {
    for case in chr_cases::semantic_cases() {
        let mut s = Search::new(case.rules, case.query, Grouping::Ready).unwrap();
        for _ in 0..case.budget {
            let before = s.checkpoint();
            let batch = s.advance(1);
            let after = s.checkpoint();
            for old in before {
                if !s.last_served().contains(&old.ticket) {
                    assert_eq!(
                        after.iter().find(|p| p.ticket == old.ticket),
                        Some(&old),
                        "{} unserved ticket {} changed",
                        case.id,
                        old.ticket
                    );
                }
            }
            s.validate().unwrap();
            if batch.exhausted {
                break;
            }
        }
    }
}
