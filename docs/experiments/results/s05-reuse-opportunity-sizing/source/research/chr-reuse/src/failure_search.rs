//! Failure-learning ablation at the actual next-equation source boundary.
use crate::failure::Learner;
use chr_persistent::continuations::{Cursor, Machine, Step};
use chr_syntax::{Answer, Query, Rule};
use std::collections::VecDeque;
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Direct,
    Learn,
    Native,
}
#[derive(Default, Debug)]
pub struct Stats {
    pub logical_steps: u64,
    pub executed: u64,
    pub completed: u64,
    pub failed: u64,
    pub projected_equations: u64,
    pub head_reads: u64,
    pub max_frontier: usize,
}
pub struct Batch {
    pub answers: Vec<Answer>,
    pub exhausted: bool,
}
pub struct Search {
    machine: Machine,
    frontier: VecDeque<Cursor>,
    mode: Mode,
    learner: Learner,
    seen: chr_observe::AnswerSet,
    stats: Stats,
}
impl Search {
    pub fn new(rules: Vec<Rule>, query: Query, mode: Mode) -> Result<Self, String> {
        let (machine, cursor) = Machine::new(rules, query)?;
        Ok(Self {
            machine,
            frontier: VecDeque::from([cursor]),
            mode,
            learner: Learner::default(),
            seen: chr_observe::AnswerSet::default(),
            stats: Stats {
                max_frontier: 1,
                ..Stats::default()
            },
        })
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn proof_stats(&self) -> &crate::failure::Stats {
        self.learner.stats()
    }
    pub fn source_stats(&self) -> &chr_persistent::Stats {
        self.machine.stats()
    }
    pub fn advance(&mut self, budget: usize) -> Batch {
        let mut answers = vec![];
        for _ in 0..budget {
            let Some(cursor) = self.frontier.pop_front() else {
                break;
            };
            self.stats.logical_steps += 1;
            // No rule-body speculation or inspection of arbitrary residuals.
            // A proof substitutes only for the source equation now being executed.
            let equation = if matches!(self.mode, Mode::Learn) {
                self.machine.pending_equation(&cursor)
            } else {
                None
            };
            if let Some((a, b)) = &equation {
                self.stats.projected_equations += 1;
                if self.learner.proves_failure(a, b) {
                    self.stats.failed += 1;
                    continue;
                }
            }
            let native_equation =
                matches!(self.mode, Mode::Native) && self.machine.has_pending_equation(&cursor);
            if native_equation {
                let machine = &mut self.machine;
                let reads = &mut self.stats.head_reads;
                if self.learner.proves_failure_paths(|left, path| {
                    *reads += 1;
                    machine.pending_head(&cursor, left, path)
                }) {
                    self.stats.failed += 1;
                    continue;
                }
            }
            let saved = native_equation.then(|| cursor.clone());
            self.stats.executed += 1;
            match self.machine.step(cursor) {
                Step::Continue(c) => self.frontier.push_back(c),
                Step::Split(a, b) => {
                    self.frontier.push_back(a);
                    self.frontier.push_back(b);
                }
                Step::Failed => {
                    self.stats.failed += 1;
                    let equation = equation.or_else(|| {
                        saved.and_then(|c| {
                            let input = self.machine.pending_equation(&c);
                            if input.is_some() {
                                self.stats.projected_equations += 1;
                            }
                            input
                        })
                    });
                    if let Some((a, b)) = equation {
                        self.learner.learn(&a, &b);
                    }
                }
                Step::Answer(answer) => {
                    self.stats.completed += 1;
                    if self.seen.insert(answer.clone()) {
                        answers.push(answer);
                    }
                }
            }
            self.stats.max_frontier = self.stats.max_frontier.max(self.frontier.len());
        }
        Batch {
            answers,
            exhausted: self.frontier.is_empty(),
        }
    }
}
