//! Shared-arena versus owned/direct and owned/memo equation-service ablations.
use crate::{EquationTable, Mode as OperationMode};
use chr_persistent::continuations::{Cursor, Machine, Step};
use chr_syntax::{Answer, Query, Rule};
use std::collections::VecDeque;
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Shared,
    Owned,
    Memo,
}
#[derive(Default, Debug)]
pub struct Stats {
    pub steps: u64,
    pub completed: u64,
    pub failed: u64,
    pub projected_equations: u64,
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
    table: EquationTable,
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
            table: EquationTable::new(if matches!(mode, Mode::Memo) {
                OperationMode::Memo
            } else {
                OperationMode::Direct
            }),
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
    pub fn operation_stats(&self) -> &crate::Stats {
        self.table.stats()
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
            self.stats.steps += 1;
            let equation = if matches!(self.mode, Mode::Shared) {
                None
            } else {
                self.machine.pending_equation(&cursor)
            };
            let event = if let Some((a, b)) = equation {
                self.stats.projected_equations += 1;
                let solution = self.table.solve(&a, &b);
                self.machine.complete_equation(cursor, solution)
            } else {
                self.machine.step(cursor)
            };
            match event {
                Step::Continue(c) => self.frontier.push_back(c),
                Step::Split(a, b) => {
                    self.frontier.push_back(a);
                    self.frontier.push_back(b);
                }
                Step::Failed => self.stats.failed += 1,
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
