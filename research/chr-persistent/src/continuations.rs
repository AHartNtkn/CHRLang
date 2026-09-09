//! Experimental source-step interface. Cursors belong to their creating machine.
use crate::{state, terms, Snapshot, Stats};
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
pub(crate) enum WorkKey<T = Term> {
    Insert(String, Vec<T>),
    Equal(T, T),
    And(Vec<WorkKey<T>>),
    Or(Box<WorkKey<T>>, Box<WorkKey<T>>),
    True,
    Fail,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateKey<T = Term> {
    pub(crate) pending: Vec<WorkKey<T>>,
    pub(crate) store: Vec<(String, u64, Vec<T>)>,
    pub(crate) outputs: Vec<(String, T)>,
    pub(crate) history: Vec<(usize, Vec<u64>)>,
    pub(crate) next_var: u64,
    pub(crate) next_occ: u64,
}
/// Variable traversal shared by owned and compact experimental key terms.
pub trait RenameVariables {
    fn rename(&mut self, variables: &mut BTreeMap<Var, Var>);
}
impl RenameVariables for Term {
    fn rename(&mut self, variables: &mut BTreeMap<Var, Var>) {
        match self {
            Term::Var(v) => {
                let next = Var(variables.len() as u64);
                *v = *variables.entry(*v).or_insert(next);
            }
            Term::App(_, args) => {
                for t in args {
                    t.rename(variables);
                }
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CompactTerm {
    Ground(usize),
    Variable(Var),
    App(String, Vec<CompactTerm>),
}
impl RenameVariables for CompactTerm {
    fn rename(&mut self, variables: &mut BTreeMap<Var, Var>) {
        match self {
            Self::Ground(_) => {}
            Self::Variable(v) => {
                let next = Var(variables.len() as u64);
                *v = *variables.entry(*v).or_insert(next);
            }
            Self::App(_, args) => {
                for t in args {
                    t.rename(variables);
                }
            }
        }
    }
}
#[derive(Clone, Debug)]
struct KeyOwner(std::rc::Rc<()>);
impl PartialEq for KeyOwner {
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for KeyOwner {}
impl PartialOrd for KeyOwner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for KeyOwner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::rc::Rc::as_ptr(&self.0).cmp(&std::rc::Rc::as_ptr(&other.0))
    }
}
/// Nonportable, machine-owned key. The retained owner prevents address recycling
/// from making a later machine's node IDs equal to these IDs.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompactStateKey {
    owner: KeyOwner,
    shape: StateKey<CompactTerm>,
}
impl CompactStateKey {
    pub fn alpha(mut self) -> Self {
        self.shape = self.shape.alpha();
        self
    }
    pub fn alpha_live_history(mut self) -> Self {
        self.shape = self.shape.alpha_live_history();
        self
    }
}
impl<T: RenameVariables> StateKey<T> {
    /// Forget only history tokens that cannot fire again: every future match
    /// uses live occurrences, and fresh occurrence IDs never reuse consumed IDs.
    pub fn alpha_live_history(mut self) -> Self {
        let live: std::collections::BTreeSet<_> = self.store.iter().map(|(_, id, _)| *id).collect();
        self.history
            .retain(|(_, ids)| ids.iter().all(|id| live.contains(id)));
        self.alpha()
    }

    /// Canonical whole-state renaming. Retains work, aliases, occurrence order,
    /// output labels and propagation history; does not project caller state.
    pub fn alpha(mut self) -> Self {
        fn work<T: RenameVariables>(w: &mut WorkKey<T>, vars: &mut BTreeMap<Var, Var>) {
            match w {
                WorkKey::Insert(_, args) => {
                    for t in args {
                        t.rename(vars);
                    }
                }
                WorkKey::Equal(a, b) => {
                    a.rename(vars);
                    b.rename(vars);
                }
                WorkKey::And(gs) => {
                    for g in gs {
                        work(g, vars);
                    }
                }
                WorkKey::Or(a, b) => {
                    work(a, vars);
                    work(b, vars);
                }
                WorkKey::True | WorkKey::Fail => {}
            }
        }
        let mut vars = BTreeMap::new();
        for w in &mut self.pending {
            work(w, &mut vars);
        }
        for (_, _, args) in &mut self.store {
            for t in args {
                t.rename(&mut vars);
            }
        }
        for (_, t) in &mut self.outputs {
            t.rename(&mut vars);
        }
        // Preserve relative occurrence order globally, including history-only IDs.
        let ids: std::collections::BTreeSet<_> = self
            .store
            .iter()
            .map(|(_, id, _)| *id)
            .chain(self.history.iter().flat_map(|(_, ids)| ids.iter().copied()))
            .collect();
        let occurrences: BTreeMap<_, _> = ids
            .into_iter()
            .enumerate()
            .map(|(i, id)| (id, i as u64))
            .collect();
        for (_, id, _) in &mut self.store {
            *id = occurrences[id];
        }
        for (_, ids) in &mut self.history {
            for id in ids {
                *id = occurrences[id];
            }
        }
        // The representative cursor retains its real fresh counters. Canonical
        // counters express only freshness beyond identities reachable in this key.
        self.next_var = vars.len() as u64;
        self.next_occ = occurrences.len() as u64;
        self
    }
}
pub enum Step {
    Continue(Cursor),
    Split(Cursor, Cursor),
    Failed,
    Answer(Answer),
}
pub struct Machine {
    rules: std::rc::Rc<Vec<Rule>>,
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
pub struct PreparedMachine {
    rules: std::rc::Rc<Vec<Rule>>,
}
impl PreparedMachine {
    pub fn new(rules: Vec<Rule>) -> Result<Self, String> {
        let mut names = std::collections::BTreeSet::new();
        for r in &rules {
            if r.kept.is_empty() && r.removed.is_empty() {
                return Err("empty rule heads".into());
            }
            if !names.insert(&r.name) {
                return Err("duplicate rule name".into());
            }
        }
        Ok(Self {
            rules: std::rc::Rc::new(rules),
        })
    }
    pub fn start(&self, query: Query) -> Result<(Machine, Cursor), String> {
        self.start_replaying(query, vec![])
    }
    /// Import validated interface equations and the remaining caller through
    /// one scope. Equations execute before any resumed source work.
    pub fn start_replaying(
        &self,
        query: Query,
        equations: Vec<(Var, Term)>,
    ) -> Result<(Machine, Cursor), String> {
        let mut names = std::collections::BTreeSet::new();
        for (name, _) in &query.outputs {
            if !names.insert(name) {
                return Err("duplicate output name".into());
            }
        }
        let mut arena = terms::Arena::default();
        let mut stats = Stats {
            max_frontier: usize::from(crate::COLLECT_METRICS),
            ..Stats::default()
        };
        let state = state::State::new_replaying(query, equations, &mut arena, &mut stats);
        let owner = std::rc::Rc::new(());
        let cursor = Cursor(state, owner.clone());
        Ok((
            Machine {
                rules: self.rules.clone(),
                arena,
                stats,
                owner,
                eager_export: crate::EagerExportStats::default(),
            },
            cursor,
        ))
    }
}
impl Machine {
    pub fn new(rules: Vec<Rule>, query: Query) -> Result<(Self, Cursor), String> {
        PreparedMachine::new(rules)?.start(query)
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
        cursor.0.key_with(
            &mut self.arena,
            &mut self.stats,
            &mut |term, bindings, arena, stats| arena.export(term, bindings, stats),
        )
    }
    pub fn compact_key(&mut self, cursor: &Cursor) -> CompactStateKey {
        self.check_cursor(cursor);
        CompactStateKey {
            owner: KeyOwner(self.owner.clone()),
            shape: cursor.0.key_with(
                &mut self.arena,
                &mut self.stats,
                &mut |term, bindings, arena, stats| arena.compact_export(term, bindings, stats),
            ),
        }
    }
    /// Diagnostic expansion for independent comparison with the owned key.
    pub fn expand_compact_key(&mut self, key: CompactStateKey) -> StateKey {
        assert!(
            std::rc::Rc::ptr_eq(&key.owner.0, &self.owner),
            "key belongs to another machine"
        );
        fn term(t: CompactTerm, arena: &terms::Arena, stats: &mut Stats) -> Term {
            match t {
                CompactTerm::Ground(id) => {
                    arena.export(terms::Term::Node(id), &terms::Bindings::default(), stats)
                }
                CompactTerm::Variable(v) => Term::Var(v),
                CompactTerm::App(n, args) => {
                    Term::App(n, args.into_iter().map(|t| term(t, arena, stats)).collect())
                }
            }
        }
        fn work(w: WorkKey<CompactTerm>, arena: &terms::Arena, stats: &mut Stats) -> WorkKey {
            match w {
                WorkKey::Insert(n, args) => {
                    WorkKey::Insert(n, args.into_iter().map(|t| term(t, arena, stats)).collect())
                }
                WorkKey::Equal(a, b) => {
                    WorkKey::Equal(term(a, arena, stats), term(b, arena, stats))
                }
                WorkKey::And(gs) => {
                    WorkKey::And(gs.into_iter().map(|w| work(w, arena, stats)).collect())
                }
                WorkKey::Or(a, b) => WorkKey::Or(
                    Box::new(work(*a, arena, stats)),
                    Box::new(work(*b, arena, stats)),
                ),
                WorkKey::True => WorkKey::True,
                WorkKey::Fail => WorkKey::Fail,
            }
        }
        let s = key.shape;
        StateKey {
            pending: s
                .pending
                .into_iter()
                .map(|w| work(w, &self.arena, &mut self.stats))
                .collect(),
            store: s
                .store
                .into_iter()
                .map(|(n, id, args)| {
                    (
                        n,
                        id,
                        args.into_iter()
                            .map(|t| term(t, &self.arena, &mut self.stats))
                            .collect(),
                    )
                })
                .collect(),
            outputs: s
                .outputs
                .into_iter()
                .map(|(n, t)| (n, term(t, &self.arena, &mut self.stats)))
                .collect(),
            history: s.history,
            next_var: s.next_var,
            next_occ: s.next_occ,
        }
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
