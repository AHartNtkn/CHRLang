use chr_observe::{Stats, equivalent};
use chr_persistent::{Search, Snapshot};
#[test]
fn both_snapshot_modes_preserve_registered_semantics() {
    for mode in [Snapshot::Persistent, Snapshot::Copy] {
        for case in chr_cases::registry() {
            let mut search = Search::new(case.rules, case.query, mode).unwrap();
            let mut actual = vec![];
            let mut exhausted = false;
            for _ in 0..case.budget {
                let b = search.advance(1);
                actual.extend(b.answers);
                exhausted = b.exhausted;
                if exhausted || case.answer_limit.is_some_and(|n| actual.len() >= n) {
                    break;
                }
            }
            assert_eq!(
                actual.len(),
                case.expected.len(),
                "{} {mode:?} answer count",
                case.id
            );
            for wanted in &case.expected {
                assert!(
                    actual
                        .iter()
                        .any(|got| equivalent(got, wanted, &mut Stats::default())),
                    "{} {mode:?} missing {wanted:?}; actual {actual:?}",
                    case.id
                );
            }
            assert_eq!(exhausted, case.exhausted, "{} {mode:?} completion", case.id);
            assert_eq!(
                search.stats().completed,
                case.raw_answers,
                "{} {mode:?} multiplicity",
                case.id
            );
        }
    }
}
#[test]
fn snapshot_sharing_does_not_claim_to_share_source_execution() {
    for mode in [Snapshot::Persistent, Snapshot::Copy] {
        let case = chr_cases::carry_case(3, 4, 16);
        let mut s = Search::new(case.rules, case.query, mode).unwrap();
        assert!(s.advance(case.budget).exhausted);
        assert_eq!(s.stats().applications, 39);
        assert_eq!(s.stats().splits, 7);
        if chr_persistent::COLLECT_KERNEL_METRICS {
            match mode {
                Snapshot::Persistent => assert_eq!(s.stats().storage.snapshot_copies, 0),
                Snapshot::Copy => assert!(s.stats().storage.snapshot_copies > 0),
            }
        }
    }
}

#[test]
fn kept_heads_and_ordered_propagation_tuples_remain_distinct() {
    use chr_syntax::{Goal, Rule, atom, c, v};
    let rules = vec![
        Rule {
            name: "consume".into(),
            kept: vec![c("p", [v(0)])],
            removed: vec![c("q", [v(1)])],
            guards: vec![],
            body: Goal::True,
        },
        Rule::propagate(
            "ordered",
            [c("p", [v(0)]), c("p", [v(1)])],
            c("pair", [v(0), v(1)]).into(),
        ),
    ];
    for mode in [Snapshot::Copy, Snapshot::Persistent] {
        let query = chr_cases::query(
            vec![
                c("p", [atom("a")]),
                c("p", [atom("b")]),
                c("q", [atom("x")]),
                c("q", [atom("y")]),
            ],
            &[],
        );
        let mut search = Search::new(rules.clone(), query, mode).unwrap();
        let batch = search.advance(1000);
        assert!(batch.exhausted);
        assert_eq!(batch.answers.len(), 1);
        let expected = chr_cases::answer(
            vec![],
            vec![
                c("p", [atom("a")]),
                c("p", [atom("b")]),
                c("pair", [atom("a"), atom("b")]),
                c("pair", [atom("b"), atom("a")]),
            ],
        );
        assert!(equivalent(
            &batch.answers[0],
            &expected,
            &mut Stats::default()
        ));
    }
}
