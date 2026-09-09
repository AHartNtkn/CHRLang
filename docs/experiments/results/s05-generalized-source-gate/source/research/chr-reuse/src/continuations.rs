//! Transition-table ablation. Every FIFO job retains one explicit derivation.
use chr_persistent::continuations::{Cursor, Machine, StateKey, Step};
use chr_syntax::{Answer, Query, Rule};
use std::collections::{BTreeMap, VecDeque};
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Direct,
    ExactIds,
    Alpha,
    AlphaLive,
}
#[derive(Default, Debug)]
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
    pub answers: Vec<Answer>,
    /// One complete answer per derivation, before unique-answer presentation.
    pub raw_answers: Vec<Answer>,
    pub exhausted: bool,
}
pub struct Search {
    machine: Machine,
    mode: Mode,
    keys: BTreeMap<StateKey, usize>,
    nodes: Vec<Node>,
    frontier: VecDeque<Job>,
    seen: chr_observe::AnswerSet,
    stats: Stats,
}
impl Search {
    pub fn new(rules: Vec<Rule>, query: Query, mode: Mode) -> Result<Self, String> {
        let (machine, cursor) = Machine::new(rules, query)?;
        let mut s = Self {
            machine,
            mode,
            keys: BTreeMap::new(),
            nodes: vec![],
            frontier: VecDeque::new(),
            seen: chr_observe::AnswerSet::default(),
            stats: Stats::default(),
        };
        match mode {
            Mode::Direct => s.frontier.push_back(Job::Direct(cursor)),
            Mode::ExactIds | Mode::Alpha | Mode::AlphaLive => {
                let id = s.intern(cursor);
                s.frontier.push_back(Job::Shared(id));
            }
        }
        s.stats.max_frontier = 1;
        Ok(s)
    }
    fn intern(&mut self, cursor: Cursor) -> usize {
        self.stats.key_requests += 1;
        let key = self.machine.key(&cursor);
        let key = match self.mode {
            Mode::Alpha => key.alpha(),
            Mode::AlphaLive => key.alpha_live_history(),
            _ => key,
        };
        if let Some(&id) = self.keys.get(&key) {
            return id;
        }
        let id = self.nodes.len();
        self.keys.insert(key, id);
        self.nodes.push(Node {
            cursor: Some(cursor),
            edge: None,
        });
        self.stats.states = self.nodes.len();
        id
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
    fn answer(&mut self, answer: Answer, answers: &mut Vec<Answer>) {
        self.stats.completed += 1;
        if self.seen.insert(answer.clone()) {
            answers.push(answer);
        }
    }
    pub fn advance(&mut self, budget: usize) -> Batch {
        let mut answers = vec![];
        let mut raw_answers = vec![];
        for _ in 0..budget {
            let Some(job) = self.frontier.pop_front() else {
                break;
            };
            self.stats.logical_steps += 1;
            match job {
                Job::Direct(cursor) => {
                    self.stats.executed += 1;
                    match self.machine.step(cursor) {
                        Step::Continue(c) => self.frontier.push_back(Job::Direct(c)),
                        Step::Split(a, b) => {
                            self.frontier.push_back(Job::Direct(a));
                            self.frontier.push_back(Job::Direct(b));
                        }
                        Step::Failed => self.stats.failed += 1,
                        Step::Answer(a) => {
                            raw_answers.push(a.clone());
                            self.answer(a, &mut answers);
                        }
                    }
                }
                Job::Shared(id) => {
                    let edge = if let Some(edge) = &self.nodes[id].edge {
                        self.stats.hits += 1;
                        edge.clone()
                    } else {
                        self.stats.executed += 1;
                        let cursor = self.nodes[id].cursor.take().expect("unexpanded state");
                        let edge = match self.machine.step(cursor) {
                            Step::Continue(c) => Edge::Continue(self.intern(c)),
                            Step::Split(a, b) => Edge::Split(self.intern(a), self.intern(b)),
                            Step::Failed => Edge::Failed,
                            Step::Answer(a) => Edge::Answer(a),
                        };
                        self.nodes[id].edge = Some(edge.clone());
                        edge
                    };
                    match edge {
                        Edge::Continue(id) => self.frontier.push_back(Job::Shared(id)),
                        Edge::Split(a, b) => {
                            self.frontier.push_back(Job::Shared(a));
                            self.frontier.push_back(Job::Shared(b));
                        }
                        Edge::Failed => self.stats.failed += 1,
                        Edge::Answer(a) => {
                            raw_answers.push(a.clone());
                            self.answer(a, &mut answers);
                        }
                    }
                }
            }
            self.stats.max_frontier = self.stats.max_frontier.max(self.frontier.len());
        }
        Batch {
            answers,
            raw_answers,
            exhausted: self.frontier.is_empty(),
        }
    }
}
