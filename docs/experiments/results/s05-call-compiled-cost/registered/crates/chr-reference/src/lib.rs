//! Direct, copying reference execution of CHR with explicit disjunction.
mod answer;
mod branch;
mod instantiate;
mod matching;
mod unify;
use branch::{Branch, Step};
use chr_syntax::{Answer, Query, Rule};
use std::collections::{BTreeSet, VecDeque};

#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub steps: u64,
    pub introductions: u64,
    pub rule_applications: u64,
    pub head_candidates: u64,
    pub equations: u64,
    pub unification_pairs: u64,
    pub splits: u64,
    pub branch_copies: u64,
    pub failed_branches: u64,
    pub completed_branches: u64,
    pub duplicate_answers: u64,
    pub max_frontier: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub struct ProgramError(pub String);
impl std::fmt::Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for ProgramError {}
#[derive(Debug)]
pub struct Batch {
    pub answers: Vec<Answer>,
    pub exhausted: bool,
}
/// A FIFO collection of independent alternatives. There is no shared mutable branch state.
pub struct Search {
    rules: Vec<Rule>,
    frontier: VecDeque<Branch>,
    seen: BTreeSet<Answer>,
    stats: Stats,
}
impl Search {
    pub fn new(rules: Vec<Rule>, query: Query) -> Result<Self, ProgramError> {
        let mut names = BTreeSet::new();
        for rule in &rules {
            if rule.kept.is_empty() && rule.removed.is_empty() {
                return Err(ProgramError(format!("rule {} has no heads", rule.name)));
            }
            if !names.insert(&rule.name) {
                return Err(ProgramError(format!("duplicate rule name: {}", rule.name)));
            }
        }
        let mut names = BTreeSet::new();
        for (name, _) in &query.outputs {
            if !names.insert(name) {
                return Err(ProgramError(format!("duplicate output name: {name}")));
            }
        }
        Ok(Self {
            rules,
            frontier: VecDeque::from([Branch::new(query)]),
            seen: BTreeSet::new(),
            stats: Stats {
                max_frontier: 1,
                ..Stats::default()
            },
        })
    }
    /// Advance at most this many reference steps. A spent budget is not search exhaustion.
    /// A step includes a finite whole-store match or a complete equation, not a time quantum.
    pub fn advance(&mut self, step_budget: usize) -> Batch {
        let mut answers = vec![];
        for _ in 0..step_budget {
            let Some(mut branch) = self.frontier.pop_front() else {
                break;
            };
            self.stats.steps += 1;
            match branch.step(&self.rules, &mut self.stats) {
                Step::Continue => self.frontier.push_back(branch),
                Step::Split(left, right) => {
                    let mut sibling = branch.clone();
                    branch.pending.push_front(left);
                    sibling.pending.push_front(right);
                    self.frontier.push_back(branch);
                    self.frontier.push_back(sibling);
                    self.stats.splits += 1;
                    self.stats.branch_copies += 1;
                }
                Step::Failed => self.stats.failed_branches += 1,
                Step::Answer(answer) => {
                    self.stats.completed_branches += 1;
                    let answer = answer::canonicalize(answer);
                    if self.seen.insert(answer.clone()) {
                        answers.push(answer);
                    } else {
                        self.stats.duplicate_answers += 1;
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
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn pending_alternatives(&self) -> usize {
        self.frontier.len()
    }
}
