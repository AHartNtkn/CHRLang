use chr_observe::graph;
use chr_persistent::continuations::{BorrowedStep, Machine, Step};
use chr_persistent::observation::CaptureStats;
use std::collections::VecDeque;
fn counts(m: &Machine) -> Vec<u64> {
    let s = m.stats();
    let e = m.eager_export_stats();
    vec![
        s.steps,
        s.applications,
        s.introductions,
        s.equations,
        s.pairs,
        s.dereferences.checked_sub(e.dereferences).unwrap(),
        s.occurs_visits,
        s.head_candidates,
        s.splits,
        s.failed,
        s.completed,
        s.duplicates,
        s.max_frontier as u64,
        s.term_nodes as u64,
        s.term_requests,
        s.pending_allocations,
        s.storage.visits.checked_sub(e.storage_visits).unwrap(),
        s.storage.allocations,
        s.storage.snapshot_copies,
    ]
}
#[test]
#[ignore = "registered isolated E14 graph-source gate"]
fn registered_completion_case() {
    let i: usize = std::env::var("CHR_GRAPH_CASE")
        .expect("registered case index")
        .parse()
        .unwrap();
    let registry = chr_cases::registry();
    assert_eq!(registry.len(), 64);
    let case = &registry[i];
    let (mut eager, e) = Machine::new(case.rules.clone(), case.query.clone()).unwrap();
    let (mut borrowed, g) = Machine::new(case.rules.clone(), case.query.clone()).unwrap();
    let mut eq = VecDeque::from([e]);
    let mut gq = VecDeque::from([g]);
    let mut capture = CaptureStats::default();
    let mut compare = graph::Stats::default();
    let mut export = graph::Stats::default();
    let mut snapshots = Vec::new();
    let mut raw = Vec::new();
    let mut unique = Vec::new();
    let mut eager_set = chr_observe::AnswerSet::default();
    let mut peak = 1;
    for _ in 0..case.budget {
        let Some(e) = eq.pop_front() else { break };
        let g = gq.pop_front().unwrap();
        match (eager.step(e), borrowed.step_borrowed(g, &mut capture)) {
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
                let duplicate = snapshots.iter().any(|old| {
                    graph::equivalent(
                        &borrowed.answer_view(old).unwrap(),
                        &borrowed.answer_view(&g).unwrap(),
                        &mut compare,
                    )
                });
                let a = borrowed.export_answer(&g, &mut export).unwrap();
                assert_eq!(a, e, "{} raw completion", case.id);
                assert_eq!(eager_set.insert(e), !duplicate);
                if !duplicate {
                    unique.push(a.clone());
                }
                snapshots.push(g);
                raw.push(a);
            }
            _ => panic!("{} event discrepancy", case.id),
        }
        assert_eq!(eq.len(), gq.len());
        peak = peak.max(eq.len());
        assert_eq!(
            counts(&eager),
            counts(&borrowed),
            "{} source counts",
            case.id
        );
        if eq.is_empty() || case.answer_limit.is_some_and(|limit| unique.len() >= limit) {
            break;
        }
    }
    assert_eq!(
        unique.len(),
        case.expected.len(),
        "{} unique count",
        case.id
    );
    for wanted in &case.expected {
        assert!(
            unique.iter().any(|a| chr_observe::equivalent(
                a,
                wanted,
                &mut chr_observe::Stats::default()
            )),
            "{} expected observation",
            case.id
        );
    }
    assert_eq!(eq.is_empty(), case.exhausted, "{} exhaustion", case.id);
    assert_eq!(raw.len() as u64, case.raw_answers, "{} raw count", case.id);
    for (s, a) in snapshots.iter().zip(&raw) {
        assert_eq!(&borrowed.export_answer(s, &mut export).unwrap(), a);
    }
    assert_eq!(counts(&eager), counts(&borrowed));
    if chr_persistent::COLLECT_METRICS {
        assert_eq!(capture.snapshots, case.raw_answers);
        assert_eq!(eager.eager_export_stats().answers, case.raw_answers);
    }
    assert_eq!(borrowed.eager_export_stats().answers, 0);
    println!(
        "SOURCE\t{i}\t{}\t{}\t{}\t{}\t{peak}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
        case.id,
        raw.len(),
        unique.len(),
        eq.is_empty(),
        counts(&borrowed),
        capture,
        compare,
        export,
        raw
    );
}
