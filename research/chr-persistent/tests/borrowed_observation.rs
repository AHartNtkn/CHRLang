use chr_observe::graph::{Stats, equivalent};
use chr_persistent::{
    continuations::{BorrowedStep, Machine},
    observation::CaptureStats,
};
use chr_syntax::{Goal, Query, Rule, and, atom, c, or, t, v};
fn machine() -> (Machine, chr_persistent::continuations::Cursor) {
    Machine::new(
        vec![Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(
                and(vec![
                    Goal::Unify(v(0), atom("a")),
                    Goal::Constraint(c("p", [t("f", [v(0)])])),
                ]),
                and(vec![
                    Goal::Unify(v(0), atom("b")),
                    Goal::Constraint(c("p", [t("f", [v(0)])])),
                ]),
            ),
        )],
        Query {
            constraints: vec![c("start", [v(0)])],
            outputs: vec![("x".into(), chr_syntax::Var(0))],
        },
    )
    .unwrap()
}
#[test]
fn snapshots_keep_branch_bindings_and_reject_another_machine() {
    let (mut m, cursor) = machine();
    let mut frontier = std::collections::VecDeque::from([cursor]);
    let mut completed = vec![];
    let mut capture = CaptureStats::default();
    while let Some(cursor) = frontier.pop_front() {
        match m.step_borrowed(cursor, &mut capture) {
            BorrowedStep::Continue(c) => frontier.push_back(c),
            BorrowedStep::Split(a, b) => {
                frontier.push_back(a);
                frontier.push_back(b);
            }
            BorrowedStep::Failed => panic!("unexpected failure"),
            BorrowedStep::Answer(a) => completed.push(a),
        }
    }
    assert_eq!(completed.len(), 2);
    assert_eq!(capture.snapshots, 2);
    let source_before = format!("{:?}", m.stats());
    assert!(!equivalent(
        &m.answer_view(&completed[0]).unwrap(),
        &m.answer_view(&completed[1]).unwrap(),
        &mut Stats::default()
    ));
    let a = m
        .export_answer(&completed[0], &mut Stats::default())
        .unwrap();
    let b = m
        .export_answer(&completed[1], &mut Stats::default())
        .unwrap();
    assert_eq!(a.outputs[0].1, atom("a"));
    assert_eq!(b.outputs[0].1, atom("b"));
    assert_eq!(a.residual, vec![c("p", [t("f", [atom("a")])])]);
    assert_eq!(source_before, format!("{:?}", m.stats()));
    let (other, _) = machine();
    assert!(other.answer_view(&completed[0]).is_err());
    assert!(
        other
            .export_answer(&completed[0], &mut Stats::default())
            .is_err()
    );
}

#[test]
fn borrowed_and_eager_transitions_agree_on_aliases_duplicates_and_failure() {
    use chr_persistent::continuations::Step;
    let success = and(vec![
        Goal::Unify(v(0), t("f", [v(1)])),
        Goal::Constraint(c("p", [v(1)])),
        Goal::Constraint(c("p", [v(1)])),
    ]);
    let rules = vec![Rule::simplify(
        "go",
        [c("go", [v(0)])],
        or(
            success.clone(),
            or(success, Goal::Unify(v(0), t("f", [v(0)]))),
        ),
    )];
    let query = Query {
        constraints: vec![c("go", [v(0)])],
        outputs: vec![("x".into(), chr_syntax::Var(0))],
    };
    let (mut eager, e) = Machine::new(rules.clone(), query.clone()).unwrap();
    let (mut graph, g) = Machine::new(rules, query).unwrap();
    let mut eq = std::collections::VecDeque::from([e]);
    let mut gq = std::collections::VecDeque::from([g]);
    let mut eager_answers = vec![];
    let mut snapshots = vec![];
    let mut capture = CaptureStats::default();
    while let Some(e) = eq.pop_front() {
        let g = gq.pop_front().expect("matching frontier");
        match (eager.step(e), graph.step_borrowed(g, &mut capture)) {
            (Step::Continue(e), BorrowedStep::Continue(g)) => {
                eq.push_back(e);
                gq.push_back(g);
            }
            (Step::Split(e, f), BorrowedStep::Split(g, h)) => {
                eq.extend([e, f]);
                gq.extend([g, h]);
            }
            (Step::Failed, BorrowedStep::Failed) => {}
            (Step::Answer(e), BorrowedStep::Answer(g)) => {
                assert_eq!(e, graph.export_answer(&g, &mut Stats::default()).unwrap());
                eager_answers.push(e);
                snapshots.push(g);
            }
            _ => panic!("source transitions differ"),
        }
    }
    assert!(gq.is_empty());
    assert_eq!(eager_answers.len(), 2);
    assert_eq!(graph.stats().failed, 1);
    assert_eq!(eager.stats().steps, graph.stats().steps);
    assert_eq!(eager.stats().equations, graph.stats().equations);
    assert_eq!(eager.stats().applications, graph.stats().applications);
    assert_eq!(eager.stats().completed, graph.stats().completed);
    assert_eq!(eager.eager_export_stats().answers, 2);
    assert_eq!(
        eager.stats().dereferences - eager.eager_export_stats().dereferences,
        graph.stats().dereferences
    );
    assert_eq!(
        eager.stats().storage.visits - eager.eager_export_stats().storage_visits,
        graph.stats().storage.visits
    );
    assert!(eager.eager_export_stats().dereferences > 0);
    assert_eq!(graph.eager_export_stats().answers, 0);
    assert_eq!(eager_answers[0].residual.len(), 2);
    assert!(equivalent(
        &graph.answer_view(&snapshots[0]).unwrap(),
        &graph.answer_view(&snapshots[1]).unwrap(),
        &mut Stats::default()
    ));
    let mut eager_set = chr_observe::AnswerSet::default();
    assert!(eager_set.insert(eager_answers[0].clone()));
    assert!(!eager_set.insert(eager_answers[1].clone()));
}
