#![cfg(feature = "fork-diagnostics")]
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
use chr_persistent::kernel::{ForkObserver, ForkSegment};
use chr_syntax::{Query, Rule, Var, and, atom, c, eq, or, t, v};
#[derive(Default)]
struct Observer {
    active: Option<&'static str>,
    count: usize,
    segments: Vec<(&'static str, ForkSegment)>,
}
impl ForkObserver for Observer {
    fn segment(&mut self, endpoint: &'static str, segment: ForkSegment) {
        self.segments.push((endpoint, segment));
    }

    fn before(&mut self, owner: &'static str) {
        assert!(self.active.replace(owner).is_none());
    }
    fn after(&mut self, owner: &'static str) {
        assert_eq!(self.active.take(), Some(owner));
        self.count += 1;
    }
}
#[test]
fn failed_branch_interning_is_counted_once_and_siblings_stay_independent() {
    let prepared = PreparedRuleset::new(
        vec![
            Rule::simplify(
                "start",
                [c("start", [v(0)])],
                or(c("left", [v(0)]).into(), c("right", [v(0)]).into()),
            ),
            Rule::simplify(
                "left",
                [c("left", [v(0)])],
                and(vec![
                    eq(v(0), t("f", [atom("a")])),
                    eq(v(0), t("f", [atom("a")])),
                    eq(v(0), atom("b")),
                ]),
            ),
            Rule::simplify("right", [c("right", [v(0)])], eq(v(0), t("g", [atom("a")]))),
        ],
        None,
    )
    .unwrap();
    for _ in 0..2 {
        let mut search = prepared
            .start_search(
                Query {
                    constraints: vec![c("start", [v(0)]), c("seed", [atom("a")])],
                    outputs: vec![("x".into(), Var(0))],
                },
                Policy::Global,
                Access::Indexed,
            )
            .unwrap();
        let mut observer = Observer::default();
        let (mut failures, mut complete, mut splits, mut exhausted) = (0, 0, 0, false);
        for _ in 0..10000 {
            match search.tick_observed(&mut observer) {
                SearchEvent::Split { .. } => splits += 1,
                SearchEvent::Failed(_) => failures += 1,
                SearchEvent::Complete(mut branch) => {
                    complete += 1;
                    let answer = branch.engine.observe().unwrap();
                    assert_eq!(answer.outputs, vec![("x".into(), t("g", [atom("a")]))]);
                    assert_eq!(answer.residual, vec![c("seed", [atom("a")])]);
                }
                SearchEvent::Exhausted => {
                    exhausted = true;
                    break;
                }
                SearchEvent::Progress => (),
            }
        }
        assert_eq!((splits, failures, complete, exhausted), (1, 1, 1, true));
        assert_eq!(observer.count, 23);
        assert!(observer.active.is_none());
        assert_eq!(observer.segments.len(), 2);
        for (endpoint, segment) in &observer.segments {
            assert!(["failed", "complete"].contains(endpoint));
            assert_eq!(segment.inherited_nodes, 1);
            assert_eq!(segment.first_miss_nodes, Some(1));
            assert_eq!(segment.predicate_insertions, 0);
        }
        let counts = search.fork_interning();
        assert_eq!(
            (counts.inherited_hits, counts.local_hits, counts.misses),
            (3, 1, 3)
        );
    }
}
