//! Explicit source disjunction search over complete branch continuations.
use crate::{COLLECT_METRICS, Engine, Stats, Work};
#[cfg(feature = "fork-diagnostics")]
use chr_persistent::kernel::{ForkInterning, ForkObserver, NoopForkObserver};
use std::collections::VecDeque;

/// Scheduler diagnostics. Branch `Engine::stats()` counters cover only the
/// segment since its most recent fork; copied execution prefixes are not counted
/// twice. Split events transfer each retired prefix segment exactly once, so
/// monotone work counters sum across split and terminal events. Retained-state
/// gauges describe copied state and must not be summed as work.
#[derive(Default, Debug)]
pub struct SearchStats {
    pub service_steps: u64,
    pub forks: u64,
    pub failed_branches: u64,
    pub raw_completions: u64,
}
/// A terminal branch owns its full engine and optional diagnostic causal history.
/// False denotes a left choice and true a right choice, from root to leaf.
pub struct CompletedBranch {
    pub lineage: Vec<bool>,
    pub engine: Engine,
}
#[allow(clippy::large_enum_variant)]
pub enum SearchEvent {
    Progress,
    Split {
        lineage: Vec<bool>,
        /// Retired prefix segment, owned exactly once; absent in counter-free builds.
        work: Option<Box<Stats>>,
    },
    Failed(CompletedBranch),
    Complete(CompletedBranch),
    Exhausted,
}
/// FIFO round-robin service branches only at explicit source OR. Competing
/// applications continue to use the ordinary engine's chosen execution schedule.
pub struct SearchEngine {
    frontier: VecDeque<CompletedBranch>,
    stats: SearchStats,
    #[cfg(feature = "fork-diagnostics")]
    interning: ForkInterning,
}
impl SearchEngine {
    pub fn new(engine: Engine) -> Self {
        Self {
            frontier: VecDeque::from([CompletedBranch {
                lineage: vec![],
                engine,
            }]),
            stats: SearchStats::default(),
            #[cfg(feature = "fork-diagnostics")]
            interning: ForkInterning::default(),
        }
    }
    pub fn enable_trace(&mut self) {
        for branch in &mut self.frontier {
            branch.engine.enable_trace();
        }
    }
    pub fn enable_audit(&mut self) {
        for branch in &mut self.frontier {
            branch.engine.enable_audit();
        }
    }
    pub fn stats(&self) -> &SearchStats {
        &self.stats
    }
    pub fn pending_branches(&self) -> usize {
        self.frontier.len()
    }
    /// Service at most one ordinary engine step and its resulting branch event.
    /// Terminal branches are transferred to the caller, without answer deduplication.
    #[cfg(feature = "fork-diagnostics")]
    pub fn fork_interning(&self) -> ForkInterning {
        self.interning
    }
    pub fn tick(&mut self) -> SearchEvent {
        self.tick_impl(
            #[cfg(feature = "fork-diagnostics")]
            &mut NoopForkObserver,
        )
    }
    #[cfg(feature = "fork-diagnostics")]
    pub fn tick_observed(&mut self, observer: &mut impl ForkObserver) -> SearchEvent {
        self.tick_impl(observer)
    }
    fn tick_impl(
        &mut self,
        #[cfg(feature = "fork-diagnostics")] observer: &mut impl ForkObserver,
    ) -> SearchEvent {
        let Some(mut branch) = self.frontier.pop_front() else {
            return SearchEvent::Exhausted;
        };
        if COLLECT_METRICS {
            self.stats.service_steps += 1;
        }
        branch.engine.step();
        #[cfg(feature = "fork-diagnostics")]
        self.interning
            .add(branch.engine.core.arena.take_fork_interning());
        #[cfg(feature = "fork-diagnostics")]
        if let Some(segment) = branch.engine.core.arena.fork_segment() {
            if branch.engine.pending_split() {
                observer.segment("split", segment);
            } else if branch.engine.failed {
                observer.segment("failed", segment);
            } else if branch.engine.done {
                observer.segment("complete", segment);
            }
        }
        if branch.engine.pending_split() {
            let Some(Work::Or(left, right)) = branch.engine.core.pending.pop() else {
                unreachable!("explicit split");
            };
            let mut sibling = branch.engine.fork_clone(
                #[cfg(feature = "fork-diagnostics")]
                observer,
            );
            #[cfg(feature = "fork-diagnostics")]
            {
                branch.engine.core.arena.mark_fork_prefix();
                sibling.core.arena.mark_fork_prefix();
            }
            let segment = branch.engine.core.segment_stats();
            let work = if COLLECT_METRICS {
                Some(Box::new(std::mem::replace(
                    &mut branch.engine.core.stats,
                    segment,
                )))
            } else {
                branch.engine.core.stats = segment;
                None
            };
            branch.engine.core.pending.push(*left);
            sibling.core.pending.push(*right);
            let parent = branch.lineage.clone();
            let mut right_path = parent.clone();
            right_path.push(true);
            branch.lineage.push(false);
            self.frontier.push_back(branch);
            self.frontier.push_back(CompletedBranch {
                lineage: right_path,
                engine: sibling,
            });
            if COLLECT_METRICS {
                self.stats.forks += 1;
            }
            return SearchEvent::Split {
                lineage: parent,
                work,
            };
        }
        if branch.engine.failed {
            if COLLECT_METRICS {
                self.stats.failed_branches += 1;
            }
            return SearchEvent::Failed(branch);
        }
        if branch.engine.done {
            if COLLECT_METRICS {
                self.stats.raw_completions += 1;
            }
            return SearchEvent::Complete(branch);
        }
        self.frontier.push_back(branch);
        SearchEvent::Progress
    }
}
#[cfg(test)]
mod tests {
    use crate::{Access, Policy, PreparedRuleset, SearchEvent};
    use chr_syntax::{Answer, Goal, Query, Rule, Var, and, atom, c, eq, or, v};

    #[test]
    fn ordinary_engine_stops_at_explicit_split_then_search_separates_bindings() {
        let rules = vec![Rule::simplify(
            "choose",
            [c("choose", [v(0)])],
            and(vec![
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
                c("after", [v(0)]).into(),
            ]),
        )];
        assert!(crate::generate::emit("choice", &rules).is_ok());
        let prepared = PreparedRuleset::new(rules, None).unwrap();
        let mut engine = prepared
            .start(
                Query {
                    constraints: vec![c("choose", [v(0)])],
                    outputs: vec![("out".into(), Var(0))],
                },
                Policy::Active,
                Access::Indexed,
            )
            .unwrap();
        let status = engine.advance(1000);
        assert!(status.pending_split && !status.exhausted && !status.failed);
        assert!(engine.observe().is_none());
        engine.step();
        assert!(engine.status().pending_split);
        let mut search = engine.into_search();
        let mut answers = vec![];
        for _ in 0..1000 {
            match search.tick() {
                SearchEvent::Complete(mut branch) => {
                    answers.push((branch.lineage, branch.engine.observe().unwrap()))
                }
                SearchEvent::Exhausted => break,
                SearchEvent::Progress | SearchEvent::Split { .. } | SearchEvent::Failed(_) => (),
            }
        }
        assert_eq!(
            answers,
            vec![
                (
                    vec![false],
                    Answer {
                        outputs: vec![("out".into(), atom("a"))],
                        residual: vec![c("after", [atom("a")])]
                    }
                ),
                (
                    vec![true],
                    Answer {
                        outputs: vec![("out".into(), atom("b"))],
                        residual: vec![c("after", [atom("b")])]
                    }
                ),
            ]
        );
        assert!(matches!(search.tick(), SearchEvent::Exhausted));
    }

    #[test]
    fn finite_sibling_receives_service_beside_divergence() {
        let prepared = PreparedRuleset::new(
            vec![
                Rule::simplify(
                    "choose",
                    [c("start", [])],
                    or(c("loop", []).into(), c("answer", []).into()),
                ),
                Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
            ],
            None,
        )
        .unwrap();
        let mut search = prepared
            .start_search(
                Query {
                    constraints: vec![c("start", [])],
                    outputs: vec![],
                },
                Policy::Active,
                Access::Indexed,
            )
            .unwrap();
        for _ in 0..1000 {
            if let SearchEvent::Complete(mut branch) = search.tick() {
                assert_eq!(branch.lineage, vec![true]);
                assert_eq!(
                    branch.engine.observe().unwrap().residual,
                    vec![c("answer", [])]
                );
                return;
            }
        }
        panic!("finite sibling starved");
    }

    #[test]
    fn failure_is_branch_local_and_raw_duplicates_are_preserved() {
        let prepared = PreparedRuleset::new(
            vec![Rule::simplify(
                "choose",
                [c("start", [])],
                or(Goal::Fail, or(Goal::True, Goal::True)),
            )],
            None,
        )
        .unwrap();
        let mut search = prepared
            .start_search(
                Query {
                    constraints: vec![c("start", [])],
                    outputs: vec![],
                },
                Policy::Active,
                Access::Indexed,
            )
            .unwrap();
        let mut branches = vec![];
        for _ in 0..1000 {
            match search.tick() {
                SearchEvent::Complete(mut branch) => {
                    branches.push((branch.lineage, branch.engine.observe().unwrap()))
                }
                SearchEvent::Exhausted => break,
                SearchEvent::Progress | SearchEvent::Split { .. } | SearchEvent::Failed(_) => (),
            }
        }
        assert_eq!(branches.len(), 2);
        assert_eq!(branches[0].1, branches[1].1);
        assert_ne!(branches[0].0, branches[1].0);
    }
    #[test]
    fn each_child_owns_live_indexed_resources_and_segment_counters() {
        let prepared = PreparedRuleset::new(
            vec![
                Rule::simplify(
                    "choose",
                    [c("start", [])],
                    or(c("take", [atom("a")]).into(), c("take", [atom("a")]).into()),
                ),
                Rule::simplify(
                    "take",
                    [c("take", [v(0)]), c("token", [v(0)])],
                    c("done", [v(0)]).into(),
                ),
            ],
            None,
        )
        .unwrap();
        let mut search = prepared
            .start_search(
                Query {
                    constraints: vec![c("token", [atom("a")]), c("start", [])],
                    outputs: vec![],
                },
                Policy::Active,
                Access::Indexed,
            )
            .unwrap();
        search.enable_trace();
        let mut completed = 0;
        let mut total_applications = 0;
        let mut split_events = 0;
        for _ in 0..1000 {
            match search.tick() {
                SearchEvent::Complete(mut branch) => {
                    assert_eq!(
                        branch.engine.observe().unwrap().residual,
                        vec![c("done", [atom("a")])]
                    );
                    assert_eq!(branch.engine.trace().len(), 2);
                    if crate::COLLECT_METRICS {
                        assert_eq!(branch.engine.stats().applications, 1);
                    } else {
                        assert_eq!(branch.engine.stats().applications, 0);
                    }
                    total_applications += branch.engine.stats().applications;
                    completed += 1;
                }
                SearchEvent::Split { work, .. } => {
                    split_events += 1;
                    if crate::COLLECT_METRICS {
                        let work = work.expect("split owns the retired prefix counters");
                        assert_eq!(work.applications, 1);
                        total_applications += work.applications;
                    } else {
                        assert!(work.is_none());
                    }
                }
                SearchEvent::Failed(_) => panic!("independent child lost its resource"),
                SearchEvent::Exhausted => break,
                _ => (),
            }
        }
        assert_eq!(completed, 2);
        assert_eq!(split_events, 1);
        assert_eq!(
            total_applications,
            if crate::COLLECT_METRICS { 3 } else { 0 }
        );
        if crate::COLLECT_METRICS {
            assert_eq!(search.stats().forks, 1);
            assert_eq!(search.stats().raw_completions, 2);
        } else {
            assert_eq!(search.stats().service_steps, 0);
            assert_eq!(search.stats().forks, 0);
            assert_eq!(search.stats().raw_completions, 0);
            assert_eq!(search.stats().failed_branches, 0);
        }
    }
}
