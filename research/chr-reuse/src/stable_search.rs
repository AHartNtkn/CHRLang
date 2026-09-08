//! Raw-answer FIFO source execution with stable-identity equation interception.
use crate::stable::{EquationCache, Policy};
use chr_persistent::continuations::{Cursor, Machine, PreparedMachine, Step};
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
pub struct Prepared {
    machine: PreparedMachine,
    policy: Policy,
    capacity: usize,
}
impl Prepared {
    pub fn new(rules: Vec<Rule>, policy: Policy, capacity: usize) -> Result<Self, String> {
        Ok(Self {
            machine: PreparedMachine::new(rules)?,
            policy,
            capacity,
        })
    }
    pub fn start(&self, query: Query) -> Result<Search, String> {
        let (machine, cursor) = self.machine.start(query)?;
        Ok(Search {
            machine,
            frontier: VecDeque::from([cursor]),
            table: EquationCache::new(self.capacity),
            policy: self.policy,
            hits: 0,
        })
    }
}
impl Search {
    pub fn new(
        rules: Vec<Rule>,
        query: Query,
        policy: Policy,
        capacity: usize,
    ) -> Result<Self, String> {
        Prepared::new(rules, policy, capacity)?.start(query)
    }
    pub fn source_stats(&self) -> &chr_persistent::Stats {
        self.machine.stats()
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
