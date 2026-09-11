//! Transition-table ablation. Every FIFO job retains one explicit derivation.
use crate::allocation_profile::measure;
use chr_persistent::continuations::{
    CompactStateKey, Cursor, Machine, PreparedMachine, StateKey, Step,
};
use chr_syntax::{Answer, Query, Rule};
use std::collections::{BTreeMap, VecDeque};
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Direct,
    ExactIds,
    Alpha,
    AlphaLive,
    CompactExact,
    CompactAlpha,
    CompactLive,
}
pub const COLLECT_METRICS: bool = cfg!(feature = "metrics");
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Stats {
    pub logical_steps: u64,
    pub executed: u64,
    pub hits: u64,
    pub key_requests: u64,
    pub states: usize,
    pub completed: u64,
    pub failed: u64,
    pub max_frontier: usize,
}
enum Job {
    Direct(Cursor),
    Shared(usize),
}
#[derive(Clone)]
enum Edge {
    Continue(usize),
    Split(usize, usize),
    Failed,
    Answer(Answer),
}
struct Node {
    cursor: Option<Cursor>,
    edge: Option<Edge>,
}
pub struct Batch {
    /// One complete owned answer per derivation. Unique presentation belongs to the consumer.
    pub answers: Vec<Answer>,
    pub exhausted: bool,
}
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum TableKey {
    Owned(StateKey),
    Compact(CompactStateKey),
}
pub struct Search {
    machine: Machine,
    mode: Mode,
    keys: BTreeMap<TableKey, usize>,
    nodes: Vec<Node>,
    frontier: VecDeque<Job>,
    stats: Stats,
    #[cfg(feature = "stage-alloc")]
    profile: crate::allocation_profile::Profile,
}
pub struct Prepared {
    machine: PreparedMachine,
    mode: Mode,
}
impl Prepared {
    pub fn new(rules: Vec<Rule>, mode: Mode) -> Result<Self, String> {
        Ok(Self {
            machine: PreparedMachine::new(rules)?,
            mode,
        })
    }
    pub fn start(&self, query: Query) -> Result<Search, String> {
        let (machine, cursor) = self.machine.start(query)?;
        Search::start(machine, cursor, self.mode)
    }
}
impl Search {
    pub fn new(rules: Vec<Rule>, query: Query, mode: Mode) -> Result<Self, String> {
        Prepared::new(rules, mode)?.start(query)
    }
    fn start(machine: Machine, cursor: Cursor, mode: Mode) -> Result<Self, String> {
        let mut s = Self {
            machine,
            mode,
            keys: BTreeMap::new(),
            nodes: vec![],
            frontier: VecDeque::new(),
            stats: Stats::default(),
            #[cfg(feature = "stage-alloc")]
            profile: Default::default(),
        };
        match mode {
            Mode::Direct => s.frontier.push_back(Job::Direct(cursor)),
            Mode::ExactIds
            | Mode::Alpha
            | Mode::AlphaLive
            | Mode::CompactExact
            | Mode::CompactAlpha
            | Mode::CompactLive => {
                let id = s.intern(cursor);
                s.frontier.push_back(Job::Shared(id));
            }
        }
        if COLLECT_METRICS {
            s.stats.max_frontier = 1;
        }
        Ok(s)
    }
    fn intern(&mut self, cursor: Cursor) -> usize {
        if COLLECT_METRICS {
            self.stats.key_requests += 1;
        }
        let key = measure!(
            self,
            0,
            match self.mode {
                Mode::CompactExact | Mode::CompactAlpha | Mode::CompactLive => {
                    let key = self.machine.compact_key(&cursor);
                    TableKey::Compact(match self.mode {
                        Mode::CompactAlpha => key.alpha(),
                        Mode::CompactLive => key.alpha_live_history(),
                        _ => key,
                    })
                }
                _ => {
                    let key = self.machine.key(&cursor);
                    TableKey::Owned(match self.mode {
                        Mode::Alpha => key.alpha(),
                        Mode::AlphaLive => key.alpha_live_history(),
                        _ => key,
                    })
                }
            }
        );
        if let Some(&id) = self.keys.get(&key) {
            return id;
        }
        let id = self.nodes.len();
        self.keys.insert(key, id);
        self.nodes.push(Node {
            cursor: Some(cursor),
            edge: None,
        });
        if COLLECT_METRICS {
            self.stats.states = self.nodes.len();
        }
        id
    }
    #[cfg(feature = "stage-alloc")]
    pub fn allocation_profile(&self) -> &crate::allocation_profile::Profile {
        &self.profile
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn source_stats(&self) -> &chr_persistent::Stats {
        self.machine.stats()
    }
    pub fn mode(&self) -> Mode {
        self.mode
    }
    pub fn advance(&mut self, budget: usize) -> Batch {
        let mut answers = vec![];
        for _ in 0..budget {
            let Some(job) = self.frontier.pop_front() else {
                break;
            };
            if COLLECT_METRICS {
                self.stats.logical_steps += 1;
            }
            match job {
                Job::Direct(cursor) => {
                    if COLLECT_METRICS {
                        self.stats.executed += 1;
                    }
                    let event = measure!(self, 2, self.machine.step(cursor));
                    measure!(
                        self,
                        4,
                        match event {
                            Step::Continue(c) => self.frontier.push_back(Job::Direct(c)),
                            Step::Split(a, b) => {
                                self.frontier.push_back(Job::Direct(a));
                                self.frontier.push_back(Job::Direct(b));
                            }
                            Step::Failed => {
                                if COLLECT_METRICS {
                                    self.stats.failed += 1;
                                }
                            }
                            Step::Answer(a) => {
                                if COLLECT_METRICS {
                                    self.stats.completed += 1;
                                }
                                answers.push(a);
                            }
                        }
                    );
                }
                Job::Shared(id) => {
                    let edge = if let Some(edge) = &self.nodes[id].edge {
                        if COLLECT_METRICS {
                            self.stats.hits += 1;
                        }
                        measure!(self, 3, edge.clone())
                    } else {
                        if COLLECT_METRICS {
                            self.stats.executed += 1;
                        }
                        let cursor = self.nodes[id].cursor.take().expect("unexpanded state");
                        let edge = match measure!(self, 2, self.machine.step(cursor)) {
                            Step::Continue(c) => Edge::Continue(self.intern(c)),
                            Step::Split(a, b) => Edge::Split(self.intern(a), self.intern(b)),
                            Step::Failed => Edge::Failed,
                            Step::Answer(a) => Edge::Answer(a),
                        };
                        self.nodes[id].edge = Some(measure!(self, 3, edge.clone()));
                        edge
                    };
                    measure!(
                        self,
                        4,
                        match edge {
                            Edge::Continue(id) => self.frontier.push_back(Job::Shared(id)),
                            Edge::Split(a, b) => {
                                self.frontier.push_back(Job::Shared(a));
                                self.frontier.push_back(Job::Shared(b));
                            }
                            Edge::Failed => {
                                if COLLECT_METRICS {
                                    self.stats.failed += 1;
                                }
                            }
                            Edge::Answer(a) => {
                                if COLLECT_METRICS {
                                    self.stats.completed += 1;
                                }
                                answers.push(a);
                            }
                        }
                    );
                }
            }
            if COLLECT_METRICS {
                self.stats.max_frontier = self.stats.max_frontier.max(self.frontier.len());
            }
        }
        Batch {
            answers,
            exhausted: self.frontier.is_empty(),
        }
    }
}
