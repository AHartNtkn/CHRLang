//! Raw-answer FIFO source execution with stable-identity equation interception.
use crate::stable::{EquationCache, Policy};
use chr_persistent::continuations::{Cursor, Machine, Step};
use chr_syntax::{Answer, Query, Rule};
use std::collections::VecDeque;
pub struct Search {
    machine: Machine,
    frontier: VecDeque<Cursor>,
    table: EquationCache,
    policy: Policy,
    hits: u64,
}
pub struct Batch {
    pub answers: Vec<Answer>,
    pub exhausted: bool,
}
impl Search {
    pub fn new(
        rules: Vec<Rule>,
        query: Query,
        policy: Policy,
        capacity: usize,
    ) -> Result<Self, String> {
        let (machine, cursor) = Machine::new(rules, query)?;
        Ok(Self {
            machine,
            frontier: VecDeque::from([cursor]),
            table: EquationCache::new(capacity),
            policy,
            hits: 0,
        })
    }
    pub fn hits(&self) -> u64 {
        self.hits
    }
    pub fn advance(&mut self, budget: usize) -> Batch {
        let mut answers = vec![];
        for _ in 0..budget {
            let Some(cursor) = self.frontier.pop_front() else {
                break;
            };
            let event = self.machine.step_with_equation(cursor, |eq| {
                let outcome = self
                    .table
                    .solve(eq, self.policy)
                    .expect("cache owner is its source machine");
                if cfg!(feature = "metrics") && outcome.hit {
                    self.hits += 1;
                }
                outcome.success
            });
            match event {
                Step::Continue(c) => self.frontier.push_back(c),
                Step::Split(a, b) => {
                    self.frontier.push_back(a);
                    self.frontier.push_back(b);
                }
                Step::Failed => (),
                Step::Answer(a) => answers.push(a),
            }
        }
        Batch {
            answers,
            exhausted: self.frontier.is_empty(),
        }
    }
}
