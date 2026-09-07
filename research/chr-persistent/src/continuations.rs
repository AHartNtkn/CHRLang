//! Experimental source-step interface. Cursors belong to their creating machine.
use crate::{Search, Snapshot, Stats, state, terms};
use chr_syntax::{Answer, Query, Rule, Term, Var};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Cursor(pub(crate) state::State);
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TermHead {
    Variable(u64),
    Constructor(String, usize),
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum WorkKey {
    Insert(String, Vec<Term>),
    Equal(Term, Term),
    And(Vec<WorkKey>),
    Or(Box<WorkKey>, Box<WorkKey>),
    True,
    Fail,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateKey {
    pub(crate) pending: Vec<WorkKey>,
    pub(crate) store: Vec<(String, u64, Vec<Term>)>,
    pub(crate) outputs: Vec<(String, Term)>,
    pub(crate) history: Vec<(usize, Vec<u64>)>,
    pub(crate) next_var: u64,
    pub(crate) next_occ: u64,
}
pub enum Step {
    Continue(Cursor),
    Split(Cursor, Cursor),
    Failed,
    Answer(Answer),
}
pub struct Machine {
    rules: Vec<Rule>,
    arena: terms::Arena,
    stats: Stats,
}
impl Machine {
    pub fn new(rules: Vec<Rule>, query: Query) -> Result<(Self, Cursor), String> {
        let mut search = Search::new(rules, query, Snapshot::Persistent)?;
        let cursor = Cursor(search.frontier.pop_front().expect("initial state"));
        Ok((
            Self {
                rules: search.rules,
                arena: search.arena,
                stats: search.stats,
            },
            cursor,
        ))
    }
    pub fn key(&mut self, cursor: &Cursor) -> StateKey {
        cursor.0.key(&self.arena, &mut self.stats)
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    /// Only the next pending equation, resolved under this cursor's environment.
    pub fn pending_equation(&mut self, cursor: &Cursor) -> Option<(Term, Term)> {
        cursor.0.pending_equation(&self.arena, &mut self.stats)
    }
    pub fn has_pending_equation(&self, cursor: &Cursor) -> bool {
        cursor.0.has_pending_equation()
    }
    /// Complete the next equation using a validated most-general solution over
    /// its resolved input holes. Identity rows are allowed; no fresh IDs occur.
    pub fn complete_equation(
        &mut self,
        mut cursor: Cursor,
        solution: Option<BTreeMap<Var, Term>>,
    ) -> Step {
        assert!(
            cursor.0.has_pending_equation(),
            "expected an active equation"
        );
        self.stats.steps += 1;
        self.stats.equations += 1;
        if cursor
            .0
            .complete_equation(solution, &mut self.arena, &mut self.stats)
        {
            Step::Continue(cursor)
        } else {
            self.stats.failed += 1;
            Step::Failed
        }
    }
    pub fn pending_head(
        &mut self,
        cursor: &Cursor,
        left: bool,
        path: &[usize],
    ) -> Option<TermHead> {
        cursor
            .0
            .pending_head(&self.arena, &mut self.stats, left, path)
    }
    pub fn step(&mut self, mut cursor: Cursor) -> Step {
        self.stats.steps += 1;
        match cursor.0.step(
            &self.rules,
            &mut self.arena,
            Snapshot::Persistent,
            &mut self.stats,
        ) {
            state::Event::Continue => Step::Continue(cursor),
            state::Event::Split(sibling) => {
                self.stats.splits += 1;
                Step::Split(cursor, Cursor(*sibling))
            }
            state::Event::Failed => {
                self.stats.failed += 1;
                Step::Failed
            }
            state::Event::Answer(answer) => {
                self.stats.completed += 1;
                Step::Answer(answer)
            }
        }
    }
}
