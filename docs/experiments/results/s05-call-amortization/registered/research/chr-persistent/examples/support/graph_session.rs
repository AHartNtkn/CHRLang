//! One FIFO source driver with four explicit observation/retention policies.
use chr_observe::{AnswerSet, Stats as EagerStats, graph};
use chr_persistent::{
    continuations::{BorrowedStep, Cursor, Machine, Step},
    observation::{CaptureStats, CompletedAnswer},
};
use chr_syntax::{Answer, Query, Rule};
use std::collections::VecDeque;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Policy {
    EagerClone,
    EagerCompare,
    EagerGraphCompare,
    GraphCompare,
}
impl Policy {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "eager_clone" => Ok(Self::EagerClone),
            "eager_compare" => Ok(Self::EagerCompare),
            "eager_graph_compare" => Ok(Self::EagerGraphCompare),
            "graph_compare" => Ok(Self::GraphCompare),
            _ => Err(format!("unknown observation policy {s}")),
        }
    }
}
pub struct Engine {
    pub machine: Machine,
    pub frontier: VecDeque<Cursor>,
    pub queue_peak: usize,
}
/// Delivered answers themselves are the index for EagerCompare and
/// EagerGraphCompare; neither adds duplicate ownership. Other policies retain
/// independently droppable keys.
pub enum Index {
    Eager(AnswerSet, usize),
    Delivered,
    DeliveredGraph,
    Graph(Vec<CompletedAnswer>),
}
impl Index {
    pub fn len(&self) -> usize {
        match self {
            Self::Eager(_, n) => *n,
            Self::Delivered | Self::DeliveredGraph => 0,
            Self::Graph(v) => v.len(),
        }
    }
}
#[derive(Default)]
pub struct Counters {
    pub capture: CaptureStats,
    pub graph_compare: graph::Stats,
    pub graph_export: graph::Stats,
    pub eager_compare: EagerStats,
    pub raw_completions: u64,
    pub exports: u64,
}
pub struct Session {
    pub engine: Engine,
    pub index: Index,
    pub deliveries: Vec<Answer>,
    pub counters: Counters,
}
impl Session {
    pub fn new(rules: Vec<Rule>, query: Query, policy: Policy) -> Result<Self, String> {
        let (machine, cursor) = Machine::new(rules, query)?;
        Ok(Self {
            engine: Engine {
                machine,
                frontier: VecDeque::from([cursor]),
                queue_peak: 1,
            },
            index: match policy {
                Policy::EagerClone => Index::Eager(AnswerSet::default(), 0),
                Policy::EagerCompare => Index::Delivered,
                Policy::EagerGraphCompare => Index::DeliveredGraph,
                Policy::GraphCompare => Index::Graph(vec![]),
            },
            deliveries: vec![],
            counters: Counters::default(),
        })
    }
    pub fn exhausted(&self) -> bool {
        self.engine.frontier.is_empty()
    }
    pub fn eager_stats(&self) -> &EagerStats {
        match &self.index {
            Index::Eager(s, _) => &s.stats,
            _ => &self.counters.eager_compare,
        }
    }
    pub fn step(&mut self) -> Result<bool, String> {
        Ok(self.advance(1)? > 0)
    }
    pub fn advance(&mut self, budget: usize) -> Result<usize, String> {
        let before = self.deliveries.len();
        for _ in 0..budget {
            let Some(cursor) = self.engine.frontier.pop_front() else {
                break;
            };
            match &mut self.index {
                Index::Graph(seen) => match self
                    .engine
                    .machine
                    .step_borrowed(cursor, &mut self.counters.capture)
                {
                    BorrowedStep::Continue(c) => self.engine.frontier.push_back(c),
                    BorrowedStep::Split(a, b) => self.engine.frontier.extend([a, b]),
                    BorrowedStep::Failed => {}
                    BorrowedStep::Answer(answer) => {
                        self.counters.raw_completions += 1;
                        let view = self.engine.machine.answer_view(&answer)?;
                        let mut duplicate = false;
                        for previous in seen.iter() {
                            if graph::equivalent(
                                &self.engine.machine.answer_view(previous)?,
                                &view,
                                &mut self.counters.graph_compare,
                            ) {
                                duplicate = true;
                                break;
                            }
                        }
                        if !duplicate {
                            self.deliveries.push(
                                self.engine
                                    .machine
                                    .export_answer(&answer, &mut self.counters.graph_export)?,
                            );
                            self.counters.exports += 1;
                            seen.push(answer);
                        }
                    }
                },
                _ => match self.engine.machine.step(cursor) {
                    Step::Continue(c) => self.engine.frontier.push_back(c),
                    Step::Split(a, b) => self.engine.frontier.extend([a, b]),
                    Step::Failed => {}
                    Step::Answer(answer) => {
                        self.counters.raw_completions += 1;
                        self.counters.exports += 1;
                        let novel = match &mut self.index {
                            Index::Eager(seen, n) => {
                                let novel = seen.insert(answer.clone());
                                if novel {
                                    *n += 1;
                                }
                                novel
                            }
                            Index::Delivered => !self.deliveries.iter().any(|previous| {
                                chr_observe::equivalent(
                                    previous,
                                    &answer,
                                    &mut self.counters.eager_compare,
                                )
                            }),
                            Index::DeliveredGraph => !self.deliveries.iter().any(|previous| {
                                graph::equivalent(
                                    &graph::TreeView(previous),
                                    &graph::TreeView(&answer),
                                    &mut self.counters.graph_compare,
                                )
                            }),
                            Index::Graph(_) => unreachable!(),
                        };
                        if novel {
                            self.deliveries.push(answer);
                        }
                    }
                },
            }
            self.engine.queue_peak = self.engine.queue_peak.max(self.engine.frontier.len());
        }
        Ok(self.deliveries.len() - before)
    }
}

/// Fixed numeric snapshot: no heap allocation or retained borrows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub source: [u64; 19],
    pub eager_export: [u64; 3],
    pub capture: [u64; 3],
    pub eager_compare: [u64; 4],
    pub graph_compare: [u64; 6],
    pub graph_export: [u64; 6],
    pub raw_completions: u64,
    pub exports: u64,
    pub deliveries: usize,
    pub index_keys: usize,
    pub frontier: usize,
    pub queue_peak: usize,
}
impl Session {
    pub fn snapshot(&self) -> Snapshot {
        let s = self.engine.machine.stats();
        let e = self.engine.machine.eager_export_stats();
        let c = &self.counters.capture;
        let o = self.eager_stats();
        let graph = |s: &graph::Stats| {
            [
                s.term_pairs,
                s.occurrence_scans,
                s.occurrence_candidates,
                s.backtracks,
                s.dereferences,
                s.binding_visits,
            ]
        };
        Snapshot {
            // Historical total fields; subtract eager_export[1,2] from indices5,16 for source-only work.
            source: [
                s.steps,
                s.applications,
                s.introductions,
                s.equations,
                s.pairs,
                s.dereferences,
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
                s.storage.visits,
                s.storage.allocations,
                s.storage.snapshot_copies,
            ],
            eager_export: [e.answers, e.dereferences, e.storage_visits],
            capture: [c.snapshots, c.residual_occurrences, c.store_visits],
            eager_compare: [
                o.term_pairs,
                o.occurrence_scans,
                o.occurrence_candidates,
                o.backtracks,
            ],
            graph_compare: graph(&self.counters.graph_compare),
            graph_export: graph(&self.counters.graph_export),
            raw_completions: self.counters.raw_completions,
            exports: self.counters.exports,
            deliveries: self.deliveries.len(),
            index_keys: self.index.len(),
            frontier: self.engine.frontier.len(),
            queue_peak: self.engine.queue_peak,
        }
    }
}
