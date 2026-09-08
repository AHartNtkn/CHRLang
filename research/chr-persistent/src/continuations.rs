//! Experimental source-step interface. Cursors belong to their creating machine.
use crate::{Search, Snapshot, Stats, state, terms};
use chr_syntax::{Answer, Query, Rule, Term, Var};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Cursor(pub(crate) state::State, std::rc::Rc<()>);
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
    owner: std::rc::Rc<()>,
    eager_export: crate::EagerExportStats,
}
/// A pending equation over this machine's append-only arena. Construction is
/// private so an external service cannot invent a different arena for its owner.
pub struct EquationAccess<'a> {
    owner: &'a std::rc::Rc<()>,
    arena: &'a terms::Arena,
    left: terms::Term,
    right: terms::Term,
    bindings: &'a mut terms::Bindings,
    stats: &'a mut Stats,
}
impl EquationAccess<'_> {
    pub fn solve_with<T>(
        &mut self,
        solve: impl FnOnce(
            &std::rc::Rc<()>,
            &terms::Arena,
            terms::Term,
            terms::Term,
            &mut terms::Bindings,
            &mut Stats,
        ) -> T,
    ) -> T {
        solve(
            self.owner,
            self.arena,
            self.left,
            self.right,
            self.bindings,
            self.stats,
        )
    }
}
impl Machine {
    pub fn new(rules: Vec<Rule>, query: Query) -> Result<(Self, Cursor), String> {
        let mut search = Search::new(rules, query, Snapshot::Persistent)?;
        let owner = std::rc::Rc::new(());
        let cursor = Cursor(
            search.frontier.pop_front().expect("initial state"),
            owner.clone(),
        );
        Ok((
            Self {
                rules: search.rules,
                arena: search.arena,
                stats: search.stats,
                owner,
                eager_export: search.eager_export,
            },
            cursor,
        ))
    }
    /// Intercept only an actual pending equation. All other source transitions
    /// use the ordinary machine. A successful transaction is followed by its
    /// normal whole-store eligibility scan once pending source effects drain.
    pub fn step_with_equation(
        &mut self,
        mut cursor: Cursor,
        mut solve: impl FnMut(&mut EquationAccess<'_>) -> bool,
    ) -> Step {
        self.check_cursor(&cursor);
        if !cursor.0.has_pending_equation() {
            return self.step(cursor);
        }
        if crate::COLLECT_METRICS {
            self.stats.steps += 1;
            self.stats.equations += 1;
        }
        let (left, right, bindings) = cursor.0.take_shared_equation();
        let success = solve(&mut EquationAccess {
            owner: &self.owner,
            arena: &self.arena,
            left,
            right,
            bindings,
            stats: &mut self.stats,
        });
        if success {
            Step::Continue(cursor)
        } else {
            if crate::COLLECT_METRICS {
                self.stats.failed += 1;
            }
            Step::Failed
        }
    }
    fn check_cursor(&self, cursor: &Cursor) {
        assert!(
            std::rc::Rc::ptr_eq(&self.owner, &cursor.1),
            "cursor belongs to another machine"
        );
    }
    pub fn key(&mut self, cursor: &Cursor) -> StateKey {
        self.check_cursor(cursor);
        cursor.0.key(&self.arena, &mut self.stats)
    }
    pub fn eager_export_stats(&self) -> &crate::EagerExportStats {
        &self.eager_export
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    /// Only the next pending equation, resolved under this cursor's environment.
    pub fn pending_equation(&mut self, cursor: &Cursor) -> Option<(Term, Term)> {
        self.check_cursor(cursor);
        cursor.0.pending_equation(&self.arena, &mut self.stats)
    }
    pub fn has_pending_equation(&self, cursor: &Cursor) -> bool {
        self.check_cursor(cursor);
        cursor.0.has_pending_equation()
    }
    /// Complete the next equation using a validated most-general solution over
    /// its resolved input holes. Identity rows are allowed; no fresh IDs occur.
    pub fn complete_equation(
        &mut self,
        mut cursor: Cursor,
        solution: Option<BTreeMap<Var, Term>>,
    ) -> Step {
        self.check_cursor(&cursor);
        assert!(
            cursor.0.has_pending_equation(),
            "expected an active equation"
        );
        if crate::COLLECT_METRICS {
            self.stats.steps += 1;
        }
        if crate::COLLECT_METRICS {
            self.stats.equations += 1;
        }
        if cursor
            .0
            .complete_equation(solution, &mut self.arena, &mut self.stats)
        {
            Step::Continue(cursor)
        } else {
            if crate::COLLECT_METRICS {
                self.stats.failed += 1;
            }
            Step::Failed
        }
    }
    pub fn pending_head(
        &mut self,
        cursor: &Cursor,
        left: bool,
        path: &[usize],
    ) -> Option<TermHead> {
        self.check_cursor(cursor);
        cursor
            .0
            .pending_head(&self.arena, &mut self.stats, left, path)
    }
    fn transition(&mut self, cursor: &mut Cursor) -> state::Event {
        self.check_cursor(cursor);
        if crate::COLLECT_METRICS {
            self.stats.steps += 1;
        }
        let event = cursor.0.step(
            &self.rules,
            &mut self.arena,
            Snapshot::Persistent,
            &mut self.stats,
        );
        match &event {
            state::Event::Split(_) => {
                if crate::COLLECT_METRICS {
                    self.stats.splits += 1;
                }
            }
            state::Event::Failed => {
                if crate::COLLECT_METRICS {
                    self.stats.failed += 1;
                }
            }
            state::Event::Complete => {
                if crate::COLLECT_METRICS {
                    self.stats.completed += 1;
                }
            }
            state::Event::Continue => {}
        }
        event
    }
    pub fn step(&mut self, mut cursor: Cursor) -> Step {
        match self.transition(&mut cursor) {
            state::Event::Continue => Step::Continue(cursor),
            state::Event::Split(sibling) => {
                Step::Split(cursor, Cursor(*sibling, self.owner.clone()))
            }
            state::Event::Failed => Step::Failed,
            state::Event::Complete => Step::Answer(cursor.0.export_answer(
                &self.arena,
                &mut self.stats,
                &mut self.eager_export,
            )),
        }
    }
    /// Same source transition as `step`, capturing roots before tree export.
    pub fn step_borrowed(
        &mut self,
        mut cursor: Cursor,
        stats: &mut crate::observation::CaptureStats,
    ) -> BorrowedStep {
        match self.transition(&mut cursor) {
            state::Event::Continue => BorrowedStep::Continue(cursor),
            state::Event::Split(sibling) => {
                BorrowedStep::Split(cursor, Cursor(*sibling, self.owner.clone()))
            }
            state::Event::Failed => BorrowedStep::Failed,
            state::Event::Complete => {
                BorrowedStep::Answer(cursor.0.into_observation(self.owner.clone(), stats))
            }
        }
    }
    pub fn answer_view<'a>(
        &'a self,
        answer: &'a crate::observation::CompletedAnswer,
    ) -> Result<crate::observation::View<'a>, String> {
        if !std::rc::Rc::ptr_eq(&self.owner, &answer.owner) {
            return Err("answer belongs to another machine".into());
        }
        Ok(crate::observation::View {
            arena: &self.arena,
            answer,
        })
    }
    pub fn export_answer(
        &self,
        answer: &crate::observation::CompletedAnswer,
        stats: &mut chr_observe::graph::Stats,
    ) -> Result<Answer, String> {
        Ok(self.answer_view(answer)?.export(stats))
    }
}
/// Completed answers retain immutable bindings and roots, not execution state.
pub enum BorrowedStep {
    Continue(Cursor),
    Split(Cursor, Cursor),
    Failed,
    Answer(crate::observation::CompletedAnswer),
}
