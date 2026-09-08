#[path = "../examples/support/graph_cost_cases.rs"]
mod graph_cost_cases;
#[path = "../examples/support/graph_session.rs"]
mod graph_session;
use graph_session::{Policy, Session};
#[test]
fn fixtures_and_policies_preserve_full_expected_answers_and_raw_counts() {
    let ids = graph_cost_cases::ids();
    assert_eq!(ids.len(), 16);
    for id in ids {
        let c = graph_cost_cases::case(&id);
        assert_eq!(c.raw_answers, 32);
        assert!(c.exhausted);
        let mut preceding = None;
        let mut preceding_source = None;
        let mut graph_algorithm_counts = None;
        for policy in [
            Policy::EagerClone,
            Policy::EagerCompare,
            Policy::GraphCompare,
            Policy::EagerGraphCompare,
        ] {
            let mut s = Session::new(c.rules.clone(), c.query.clone(), policy).unwrap();
            assert_eq!(
                Policy::parse(match policy {
                    Policy::EagerClone => "eager_clone",
                    Policy::EagerCompare => "eager_compare",
                    Policy::GraphCompare => "graph_compare",
                    Policy::EagerGraphCompare => "eager_graph_compare",
                })
                .unwrap(),
                policy
            );
            let mut steps = 0;
            while !s.exhausted() && steps < c.budget {
                let before = s.deliveries.len();
                let new = s.step().unwrap();
                assert_eq!(new, s.deliveries.len() > before);
                steps += 1;
            }
            assert_eq!(s.snapshot().raw_completions, 32);
            assert_eq!(s.snapshot().frontier, 0);
            let snap = s.snapshot();
            if matches!(policy, Policy::GraphCompare | Policy::EagerGraphCompare) {
                let counts = [
                    snap.graph_compare[0],
                    snap.graph_compare[1],
                    snap.graph_compare[2],
                    snap.graph_compare[3],
                ];
                if let Some(previous) = graph_algorithm_counts {
                    assert_eq!(counts, previous, "same comparison algorithm {id}");
                }
                graph_algorithm_counts = Some(counts);
            }
            if policy == Policy::EagerGraphCompare {
                assert_eq!(snap.capture, [0; 3]);
                assert_eq!(
                    snap.eager_export[0],
                    if chr_persistent::COLLECT_METRICS {
                        32
                    } else {
                        0
                    }
                );
                assert_eq!(snap.eager_compare, [0; 4]);
                assert_eq!(snap.graph_compare[4..], [0; 2]);
            }
            let mut source = snap.source;
            source[5] -= snap.eager_export[1];
            source[16] -= snap.eager_export[2];
            if let Some(previous) = preceding_source {
                assert_eq!(source, previous, "source accounting {id} {policy:?}");
            }
            preceding_source = Some(source);
            assert_eq!(
                snap.exports,
                if policy == Policy::GraphCompare {
                    c.expected.len() as u64
                } else {
                    32
                }
            );
            assert_eq!(
                snap.index_keys,
                if matches!(policy, Policy::EagerCompare | Policy::EagerGraphCompare) {
                    0
                } else {
                    c.expected.len()
                }
            );
            assert!(s.exhausted(), "{id} {policy:?}");
            if chr_persistent::COLLECT_METRICS {
                assert_eq!(s.engine.machine.stats().completed, c.raw_answers, "{id}");
            }
            assert_eq!(s.deliveries.len(), c.expected.len(), "{id} {policy:?}");
            for answer in &c.expected {
                assert!(
                    s.deliveries.iter().any(|actual| chr_observe::equivalent(
                        actual,
                        answer,
                        &mut chr_observe::Stats::default()
                    )),
                    "{id} {policy:?}: {answer:?}"
                );
            }
            if let Some(previous) = preceding {
                assert_eq!(s.deliveries, previous, "same FIFO delivery {id}");
            }
            preceding = Some(s.deliveries.clone());
        }
    }
}
#[test]
fn graph_mode_exports_only_recognized_completions_and_retains_only_their_snapshots() {
    let c = graph_cost_cases::case("dag-4-repeat");
    let mut s = Session::new(c.rules, c.query, Policy::GraphCompare).unwrap();
    s.advance(c.budget).unwrap();
    assert_eq!(s.snapshot().raw_completions, 32);
    assert_eq!(
        s.counters.capture.snapshots,
        if chr_persistent::COLLECT_METRICS {
            32
        } else {
            0
        }
    );
    assert_eq!(s.counters.exports, 1);
    assert_eq!(s.index.len(), 1);
    assert_eq!(s.engine.machine.eager_export_stats().answers, 0);
    assert_eq!(s.deliveries.len(), 1);
}
