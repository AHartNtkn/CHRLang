//! Resumable transition reuse with source-unreadable ground observations owned
//! by each derivation. Active resources and history remain in the state key.
use crate::allocation_profile::measure;
use crate::continuations::{Batch, COLLECT_METRICS, Stats};
use chr_persistent::continuations::{Cursor, Machine, PreparedMachine, StateKey, Step};
use chr_syntax::{Answer, Constraint, Query, Rule};
use std::collections::{BTreeMap, VecDeque};
#[derive(Clone)]
struct Target {
    id: usize,
    added: Vec<Constraint>,
}
#[derive(Clone)]
enum Edge {
    Continue(Target),
    Split(Target, Target),
    Failed,
    Answer(Answer),
}
struct Node {
    cursor: Option<Cursor>,
    edge: Option<Edge>,
}
struct Job {
    id: usize,
    residual: Vec<Constraint>,
}
pub struct Prepared {
    machine: PreparedMachine,
    memo: bool,
}
pub struct Search {
    machine: Machine,
    memo: bool,
    keys: BTreeMap<StateKey, usize>,
    nodes: Vec<Node>,
    free: Vec<usize>,
    frontier: VecDeque<Job>,
    stats: Stats,
    #[cfg(feature = "stage-alloc")]
    profile: crate::allocation_profile::Profile,
}
impl Prepared {
    pub fn new(rules: Vec<Rule>, memo: bool) -> Result<Self, String> {
        Ok(Self {
            machine: PreparedMachine::new(rules)?,
            memo,
        })
    }
    pub fn start(&self, q: Query) -> Result<Search, String> {
        let (machine, cursor) = self.machine.start(q)?;
        let mut run = Search {
            machine,
            memo: self.memo,
            keys: BTreeMap::new(),
            nodes: vec![],
            free: vec![],
            frontier: VecDeque::new(),
            stats: Stats::default(),
            #[cfg(feature = "stage-alloc")]
            profile: Default::default(),
        };
        let target = run.intern(cursor);
        run.frontier.push_back(Job {
            id: target.id,
            residual: target.added,
        });
        if COLLECT_METRICS {
            run.stats.max_frontier = 1;
        }
        Ok(run)
    }
}
impl Search {
    fn intern(&mut self, mut cursor: Cursor) -> Target {
        let added = measure!(self, 1, self.machine.detach_inert_ground(&mut cursor));
        let key = if self.memo {
            if COLLECT_METRICS {
                self.stats.key_requests += 1;
            }
            let key = measure!(self, 0, self.machine.key(&cursor).alpha_live_history());
            if let Some(&id) = self.keys.get(&key) {
                return Target { id, added };
            }
            Some(key)
        } else {
            None
        };
        let id = self.free.pop().unwrap_or_else(|| {
            let id = self.nodes.len();
            self.nodes.push(Node {
                cursor: None,
                edge: None,
            });
            id
        });
        if let Some(key) = key {
            self.keys.insert(key, id);
        }
        self.nodes[id] = Node {
            cursor: Some(cursor),
            edge: None,
        };
        if COLLECT_METRICS {
            self.stats.states = self.nodes.len();
        }
        Target { id, added }
    }
    #[cfg(feature = "stage-alloc")]
    pub fn allocation_profile(&self) -> &crate::allocation_profile::Profile {
        &self.profile
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
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
            let edge = if let Some(edge) = &self.nodes[job.id].edge {
                if COLLECT_METRICS {
                    self.stats.hits += 1;
                }
                measure!(self, 3, edge.clone())
            } else {
                if COLLECT_METRICS {
                    self.stats.executed += 1;
                }
                let cursor = self.nodes[job.id].cursor.take().expect("unexpanded state");
                let edge = match measure!(self, 2, self.machine.step(cursor)) {
                    Step::Continue(c) => Edge::Continue(self.intern(c)),
                    Step::Split(a, b) => Edge::Split(self.intern(a), self.intern(b)),
                    Step::Failed => Edge::Failed,
                    Step::Answer(a) => Edge::Answer(a),
                };
                if self.memo {
                    self.nodes[job.id].edge = Some(measure!(self, 3, edge.clone()));
                }
                edge
            };
            if !self.memo {
                self.free.push(job.id);
            }
            measure!(
                self,
                4,
                match edge {
                    Edge::Continue(target) => {
                        let mut residual = job.residual;
                        residual.extend(target.added);
                        self.frontier.push_back(Job {
                            id: target.id,
                            residual,
                        });
                    }
                    Edge::Split(a, b) => {
                        let mut left = job.residual.clone();
                        left.extend(a.added);
                        let mut right = job.residual;
                        right.extend(b.added);
                        self.frontier.push_back(Job {
                            id: a.id,
                            residual: left,
                        });
                        self.frontier.push_back(Job {
                            id: b.id,
                            residual: right,
                        });
                    }
                    Edge::Failed => {
                        if COLLECT_METRICS {
                            self.stats.failed += 1;
                        }
                    }
                    Edge::Answer(mut answer) => {
                        answer.residual.extend(job.residual);
                        answers.push(answer);
                        if COLLECT_METRICS {
                            self.stats.completed += 1;
                        }
                    }
                }
            );
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
