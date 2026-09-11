use chr_boxes::{Search, Snapshot};
#[test]
fn delayed_execution_preserves_registered_observations() {
    for quota in [0, 1, 8, 64] {
        for case in chr_cases::registry() {
            let mut s = Search::new(case.rules, case.query, Snapshot::Persistent, quota).unwrap();
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
            assert_eq!(
                actual.len(),
                case.expected.len(),
                "{} quota {quota}",
                case.id
            );
            for e in case.expected {
                assert!(
                    actual
                        .iter()
                        .any(|a| chr_observe::equivalent(a, &e, &mut Default::default())),
                    "{} quota {quota}",
                    case.id
                );
            }
            assert_eq!(exhausted, case.exhausted, "{} quota {quota}", case.id);
            assert_eq!(
                s.stats().completed,
                case.raw_answers,
                "{} quota {quota}",
                case.id
            );
        }
    }
}
#[test]
fn opaque_reductions_can_precede_explicit_distribution() {
    let c = chr_cases::carry_case(3, 4, 16);
    let mut s = Search::new(c.rules, c.query, Snapshot::Persistent, 64).unwrap();
    let b = s.advance(1000);
    assert!(b.exhausted);
    assert_eq!(b.answers.len(), 8);
    assert_eq!(s.stats().applications, 11);
    assert_eq!(s.stats().lifted_applications, 4);
}

#[test]
fn a_ground_resource_consumed_after_binding_blocks_common_lifting() {
    use chr_syntax::{Rule, atom, c, eq, or, t, v};
    let rules = vec![
        Rule::simplify(
            "choose",
            [c("choose", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify(
            "consume",
            [c("p", [t("s", [atom("a")])]), c("q", [atom("a")])],
            c("hit", []).into(),
        ),
        Rule::simplify("step", [c("p", [t("s", [v(0)])])], c("p", [v(0)]).into()),
    ];
    for quota in [0, 64] {
        let mut s = Search::new(
            rules.clone(),
            chr_cases::query(
                vec![
                    c("p", [t("s", [atom("a")])]),
                    c("q", [v(0)]),
                    c("choose", [v(0)]),
                ],
                &[],
            ),
            Snapshot::Persistent,
            quota,
        )
        .unwrap();
        let b = s.advance(100);
        assert!(b.exhausted);
        assert_eq!(b.answers.len(), 2);
        assert_eq!(s.stats().lifted_applications, 0);
        let expected = [
            chr_cases::answer(vec![], vec![c("hit", [])]),
            chr_cases::answer(vec![], vec![c("p", [atom("a")]), c("q", [atom("b")])]),
        ];
        for e in expected {
            assert!(b.answers.iter().any(|a| chr_observe::equivalent(
                a,
                &e,
                &mut Default::default()
            )));
        }
    }
}
#[test]
fn finite_preference_cannot_hide_refutation_behind_an_infinite_common_producer() {
    use chr_syntax::{Goal, Rule, c, or, v};
    let rules = vec![
        Rule::simplify("choose", [c("choose", [])], or(Goal::Fail, Goal::Fail)),
        Rule::simplify("loop", [c("loop", [v(0)])], c("loop", [v(0)]).into()),
    ];
    for quota in [0, 1, 8, 64, usize::MAX] {
        let mut s = Search::new(
            rules.clone(),
            chr_cases::query(vec![c("loop", [v(0)]), c("choose", [])], &[]),
            Snapshot::Persistent,
            quota,
        )
        .unwrap();
        let b = s.advance(200);
        assert!(b.answers.is_empty());
        if quota == usize::MAX {
            assert!(!b.exhausted);
            assert_eq!(s.stats().failed, 0);
        } else {
            assert!(b.exhausted);
            assert_eq!(s.stats().failed, 2);
            assert_eq!(s.stats().lifted_applications, quota as u64);
        }
    }
}
