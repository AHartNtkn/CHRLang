//! Matched mutable source semantics for copying, reversible paths and root replay.
use chr_syntax::{Answer, Constraint, Goal, Guard, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::num::NonZeroUsize;
use std::sync::Arc;
type History = (usize, Vec<usize>);
type Slots = BTreeMap<Var, Term>;
#[derive(Clone, Default, Debug, PartialEq, Eq)]
struct State {
    bindings: BTreeMap<Var, Arc<Term>>,
    live: BTreeMap<usize, Arc<Constraint>>,
    pending: Vec<Arc<Goal>>,
    history: BTreeSet<History>,
    next_var: u64,
    next_occ: usize,
}
#[derive(Clone)]
enum Change {
    Binding(Var, Option<Arc<Term>>, Option<Arc<Term>>),
    Live(usize, Option<Arc<Constraint>>, Option<Arc<Constraint>>),
    History(History, bool, bool),
    Push(Arc<Goal>),
    Pop(Arc<Goal>),
    VariableCounter(u64, u64),
    OccurrenceCounter(usize, usize),
}
impl Change {
    fn apply(&self, s: &mut State, forward: bool) {
        match self {
            Self::Binding(k, a, b) => match if forward { b } else { a } {
                Some(v) => {
                    s.bindings.insert(*k, v.clone());
                }
                None => {
                    s.bindings.remove(k);
                }
            },
            Self::Live(k, a, b) => match if forward { b } else { a } {
                Some(v) => {
                    s.live.insert(*k, v.clone());
                }
                None => {
                    s.live.remove(k);
                }
            },
            Self::History(k, a, b) => {
                if if forward { *b } else { *a } {
                    s.history.insert(k.clone());
                } else {
                    s.history.remove(k);
                }
            }
            Self::Push(g) => {
                if forward {
                    s.pending.push(g.clone());
                } else {
                    let popped = s.pending.pop();
                    debug_assert_eq!(popped.as_ref(), Some(g));
                }
            }
            Self::Pop(g) => {
                if forward {
                    let popped = s.pending.pop();
                    debug_assert_eq!(popped.as_ref(), Some(g));
                } else {
                    s.pending.push(g.clone());
                }
            }
            Self::VariableCounter(a, b) => s.next_var = if forward { *b } else { *a },
            Self::OccurrenceCounter(a, b) => s.next_occ = if forward { *b } else { *a },
        }
    }
}
struct Recorder {
    enabled: bool,
    changes: Vec<Change>,
}
impl Recorder {
    fn new(enabled: bool) -> Self {
        Self {
            enabled,
            changes: Vec::new(),
        }
    }
    fn change(&mut self, s: &mut State, c: Change) {
        c.apply(s, true);
        if self.enabled {
            self.changes.push(c);
        }
    }
}
fn vars(t: &Term, out: &mut BTreeSet<Var>) {
    match t {
        Term::Var(v) => {
            out.insert(*v);
        }
        Term::App(_, xs) => {
            for t in xs {
                vars(t, out);
            }
        }
    }
}
fn local(t: &Term, env: &mut Slots, next: &mut u64) -> Term {
    match t {
        Term::Var(v) => env
            .entry(*v)
            .or_insert_with(|| {
                let v = Var(*next);
                *next = next.checked_add(1).expect("variable identity exhausted");
                Term::Var(v)
            })
            .clone(),
        Term::App(n, xs) => Term::App(n.clone(), xs.iter().map(|t| local(t, env, next)).collect()),
    }
}
fn body(g: &Goal, env: &mut Slots, next: &mut u64) -> Goal {
    match g {
        Goal::True => Goal::True,
        Goal::Fail => Goal::Fail,
        Goal::Constraint(c) => Goal::Constraint(Constraint {
            name: c.name.clone(),
            args: c.args.iter().map(|t| local(t, env, next)).collect(),
        }),
        Goal::Unify(a, b) => Goal::Unify(local(a, env, next), local(b, env, next)),
        Goal::And(gs) => Goal::And(gs.iter().map(|g| body(g, env, next)).collect()),
        Goal::Or(a, b) => Goal::Or(Box::new(body(a, env, next)), Box::new(body(b, env, next))),
    }
}
fn matches(pattern: &Term, value: &Term, env: &mut Slots) -> bool {
    match pattern {
        Term::Var(v) => match env.get(v) {
            Some(old) => old == value,
            None => {
                env.insert(*v, value.clone());
                true
            }
        },
        Term::App(n, xs) => match value {
            Term::App(m, ys) => {
                n == m && xs.len() == ys.len() && xs.iter().zip(ys).all(|(p, t)| matches(p, t, env))
            }
            _ => false,
        },
    }
}
fn occurs(v: Var, t: &Term) -> bool {
    match t {
        Term::Var(x) => v == *x,
        Term::App(_, xs) => xs.iter().any(|t| occurs(v, t)),
    }
}
struct Application {
    rule: usize,
    ids: Vec<usize>,
    effect: Arc<Goal>,
    next: u64,
}
enum Event {
    Progress,
    Fork(Arc<Goal>, Arc<Goal>),
    Answer,
    Failed,
}
impl State {
    fn new(q: &Query) -> Result<Self, String> {
        let mut names = BTreeSet::new();
        for c in &q.constraints {
            for t in &c.args {
                vars(t, &mut names);
            }
        }
        for (_, v) in &q.outputs {
            names.insert(*v);
        }
        let next_var = names.last().map_or(Ok(0), |v| {
            v.0.checked_add(1).ok_or("variable identity exhausted")
        })?;
        Ok(Self {
            live: q
                .constraints
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, c)| (i, Arc::new(c)))
                .collect(),
            next_var,
            next_occ: q.constraints.len(),
            ..Self::default()
        })
    }
    fn resolve(&self, t: &Term) -> Term {
        match t {
            Term::Var(v) => self
                .bindings
                .get(v)
                .map_or_else(|| t.clone(), |t| self.resolve(t)),
            Term::App(n, xs) => Term::App(n.clone(), xs.iter().map(|t| self.resolve(t)).collect()),
        }
    }
    fn unify(&mut self, a: &Term, b: &Term, log: &mut Recorder) -> bool {
        let mut todo = vec![(a.clone(), b.clone())];
        while let Some((a, b)) = todo.pop() {
            let (a, b) = (self.resolve(&a), self.resolve(&b));
            if a == b {
                continue;
            }
            match (a, b) {
                (Term::Var(v), t) | (t, Term::Var(v)) => {
                    if occurs(v, &t) {
                        return false;
                    }
                    let old = self.bindings.get(&v).cloned();
                    log.change(self, Change::Binding(v, old, Some(Arc::new(t))));
                }
                (Term::App(n, xs), Term::App(m, ys)) => {
                    if n != m || xs.len() != ys.len() {
                        return false;
                    }
                    todo.extend(xs.into_iter().zip(ys));
                }
            }
        }
        true
    }
    fn root<'a>(&'a self, mut value: &'a Term) -> &'a Term {
        while let Term::Var(v) = value {
            match self.bindings.get(v) {
                Some(next) => value = next.as_ref(),
                None => break,
            }
        }
        value
    }
    /// A necessary root condition, not full matching. Pattern variables are
    /// scoped through the entry slots, never through query-variable identities.
    /// Equal constructor roots may still disagree deeper in their structure.
    fn roots_compatible(&self, head: &Constraint, candidate: &Constraint, env: &Slots) -> bool {
        head.args
            .iter()
            .zip(&candidate.args)
            .all(|(pattern, value)| {
                let wanted = match pattern {
                    Term::Var(v) => match env.get(v) {
                        Some(wanted) => wanted,
                        None => return true,
                    },
                    Term::App(..) => pattern,
                };
                match (wanted, self.root(value)) {
                    (Term::Var(a), Term::Var(b)) => a == b,
                    (Term::App(a, xs), Term::App(b, ys)) => a == b && xs.len() == ys.len(),
                    _ => false,
                }
            })
    }
    fn tuple(
        &self,
        ri: usize,
        rule: &Rule,
        heads: &[&Constraint],
        ids: Vec<usize>,
        env: Slots,
    ) -> Option<Application> {
        if ids.len() == heads.len() {
            if rule.removed.is_empty() && self.history.contains(&(ri, ids.clone())) {
                return None;
            }
            let mut env = env;
            let mut next = self.next_var;
            for Guard::Equal(a, b) in &rule.guards {
                let a = local(a, &mut env, &mut next);
                let b = local(b, &mut env, &mut next);
                if self.resolve(&a) != self.resolve(&b) {
                    return None;
                }
            }
            let effect = Arc::new(body(&rule.body, &mut env, &mut next));
            return Some(Application {
                rule: ri,
                ids,
                effect,
                next,
            });
        }
        let head = heads[ids.len()];
        for (id, c) in &self.live {
            if ids.contains(id) || head.name != c.name || head.args.len() != c.args.len() {
                continue;
            }
            if !env.is_empty() && !self.roots_compatible(head, c, &env) {
                #[cfg(feature = "replay-diagnostic")]
                diagnostics::record_precheck_rejection();
                continue;
            }
            let mut slots = env.clone();
            let accepted = head
                .args
                .iter()
                .zip(&c.args)
                .all(|(p, t)| matches(p, &self.resolve(t), &mut slots));
            #[cfg(feature = "replay-diagnostic")]
            diagnostics::record_environment(&env, !accepted);
            if !accepted {
                continue;
            }
            let mut chosen = ids.clone();
            chosen.push(*id);
            if let Some(a) = self.tuple(ri, rule, heads, chosen, slots) {
                return Some(a);
            }
        }
        None
    }
    fn needs_choice(&self) -> bool {
        self.pending
            .last()
            .is_some_and(|g| matches!(g.as_ref(), Goal::Or(..)))
    }
    fn step(&mut self, rules: &[Rule], decision: Option<bool>, log: &mut Recorder) -> Event {
        #[cfg(feature = "replay-diagnostic")]
        diagnostics::record_step();
        if let Some(g) = self.pending.last().cloned() {
            log.change(self, Change::Pop(g.clone()));
            match g.as_ref() {
                Goal::True => (),
                Goal::Fail => return Event::Failed,
                Goal::Constraint(c) => {
                    let id = self.next_occ;
                    let next = id.checked_add(1).expect("occurrence identity exhausted");
                    log.change(self, Change::OccurrenceCounter(id, next));
                    log.change(self, Change::Live(id, None, Some(Arc::new(c.clone()))));
                }
                Goal::Unify(a, b) => {
                    if !self.unify(a, b, log) {
                        return Event::Failed;
                    }
                }
                Goal::And(gs) => {
                    for g in gs.iter().rev() {
                        log.change(self, Change::Push(Arc::new(g.clone())));
                    }
                }
                Goal::Or(a, b) => {
                    if let Some(right) = decision {
                        log.change(
                            self,
                            Change::Push(Arc::new(
                                if right { b.as_ref() } else { a.as_ref() }.clone(),
                            )),
                        );
                    } else {
                        return Event::Fork(
                            Arc::new(a.as_ref().clone()),
                            Arc::new(b.as_ref().clone()),
                        );
                    }
                }
            }
            return Event::Progress;
        }
        let application = rules.iter().enumerate().find_map(|(i, r)| {
            self.tuple(
                i,
                r,
                &r.kept.iter().chain(&r.removed).collect::<Vec<_>>(),
                Vec::new(),
                Slots::new(),
            )
        });
        let Some(a) = application else {
            return Event::Answer;
        };
        let rule = &rules[a.rule];
        for id in &a.ids[rule.kept.len()..] {
            let before = self.live.get(id).cloned();
            log.change(self, Change::Live(*id, before, None));
        }
        if rule.removed.is_empty() {
            log.change(self, Change::History((a.rule, a.ids), false, true));
        }
        if a.next != self.next_var {
            log.change(self, Change::VariableCounter(self.next_var, a.next));
        }
        log.change(self, Change::Push(a.effect));
        Event::Progress
    }
    fn answer(&self, outputs: &[(String, Var)]) -> Answer {
        Answer {
            outputs: outputs
                .iter()
                .map(|(n, v)| (n.clone(), self.resolve(&Term::Var(*v))))
                .collect(),
            residual: self
                .live
                .values()
                .map(|c| Constraint {
                    name: c.name.clone(),
                    args: c.args.iter().map(|t| self.resolve(t)).collect(),
                })
                .collect(),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Copy,
    Trail,
    Replay,
    /// Save after the next non-fork progress step at or beyond this interval.
    Checkpoint(NonZeroUsize),
}
pub struct Prepared {
    rules: Vec<Rule>,
}
impl Prepared {
    pub fn new(rules: &[Rule]) -> Result<Arc<Self>, String> {
        if rules
            .iter()
            .any(|r| r.kept.is_empty() && r.removed.is_empty())
        {
            return Err("empty-head rules unsupported".into());
        }
        Ok(Arc::new(Self {
            rules: rules.to_vec(),
        }))
    }
    pub fn start(self: &Arc<Self>, q: &Query, mode: Mode) -> Result<Engine, String> {
        let initial = State::new(q)?;
        let backend = match mode {
            Mode::Copy => Backend::Copy(VecDeque::from([initial])),
            Mode::Replay | Mode::Checkpoint(_) => Backend::Replay {
                interval: match mode {
                    Mode::Checkpoint(n) => Some(n),
                    _ => None,
                },
                frontier: VecDeque::new(),
                resident: Some(initial),
            },
            Mode::Trail => {
                let root = Arc::new(Node {
                    parent: None,
                    depth: 0,
                    changes: vec![],
                });
                Backend::Trail {
                    state: initial,
                    current: root.clone(),
                    frontier: VecDeque::from([root]),
                }
            }
        };
        Ok(Engine {
            prepared: self.clone(),
            outputs: q.outputs.clone(),
            backend,
        })
    }
}
struct Node {
    parent: Option<Arc<Node>>,
    depth: usize,
    changes: Vec<Change>,
}
fn extend(parent: Arc<Node>, changes: Vec<Change>) -> Arc<Node> {
    if changes.is_empty() {
        parent
    } else {
        Arc::new(Node {
            depth: parent.depth + 1,
            parent: Some(parent),
            changes,
        })
    }
}
fn switch(s: &mut State, current: &mut Arc<Node>, target: Arc<Node>) {
    let mut left = current.clone();
    let mut right = target.clone();
    let mut redo = Vec::new();
    while !Arc::ptr_eq(&left, &right) {
        if left.depth >= right.depth {
            for c in left.changes.iter().rev() {
                c.apply(s, false);
            }
            left = left.parent.as_ref().expect("common root").clone();
        } else {
            redo.push(right.clone());
            right = right.parent.as_ref().expect("common root").clone();
        }
    }
    for node in redo.into_iter().rev() {
        for c in &node.changes {
            c.apply(s, true);
        }
    }
    *current = target;
}
struct Recipe {
    checkpoint: Arc<State>,
    choices: Vec<bool>,
    steps: usize,
}

enum Backend {
    Copy(VecDeque<State>),
    Trail {
        state: State,
        current: Arc<Node>,
        frontier: VecDeque<Arc<Node>>,
    },
    Replay {
        interval: Option<NonZeroUsize>,
        frontier: VecDeque<Recipe>,
        // Mutually exclusive with pending saved recipes: the sole working state.
        resident: Option<State>,
    },
}
#[derive(Debug, PartialEq, Eq)]
pub enum Step {
    Progress,
    Answer(Answer),
    Exhausted,
}
pub struct Engine {
    prepared: Arc<Prepared>,
    outputs: Vec<(String, Var)>,
    backend: Backend,
}
impl Engine {
    pub fn advance(&mut self) -> Step {
        match &mut self.backend {
            Backend::Copy(frontier) => {
                let Some(mut s) = frontier.pop_front() else {
                    return Step::Exhausted;
                };
                match s.step(&self.prepared.rules, None, &mut Recorder::new(false)) {
                    Event::Progress => frontier.push_back(s),
                    Event::Failed => (),
                    Event::Answer => return Step::Answer(s.answer(&self.outputs)),
                    Event::Fork(a, b) => {
                        let mut other = s.clone();
                        s.pending.push(a);
                        other.pending.push(b);
                        frontier.push_back(s);
                        frontier.push_back(other);
                    }
                }
            }
            Backend::Trail {
                state,
                current,
                frontier,
            } => {
                let Some(target) = frontier.pop_front() else {
                    return Step::Exhausted;
                };
                switch(state, current, target);
                let mut log = Recorder::new(!frontier.is_empty());
                let event = state.step(&self.prepared.rules, None, &mut log);
                if frontier.is_empty() {
                    *current = Arc::new(Node {
                        parent: None,
                        depth: 0,
                        changes: vec![],
                    });
                } else {
                    *current = extend(current.clone(), log.changes);
                }
                match event {
                    Event::Progress => frontier.push_back(current.clone()),
                    Event::Failed => (),
                    Event::Answer => return Step::Answer(state.answer(&self.outputs)),
                    Event::Fork(a, b) => {
                        frontier.push_back(extend(current.clone(), vec![Change::Push(a)]));
                        frontier.push_back(extend(current.clone(), vec![Change::Push(b)]));
                    }
                }
            }
            Backend::Replay {
                interval,
                frontier,
                resident,
            } => {
                let (mut state, saved) = if let Some(state) = resident.take() {
                    assert!(frontier.is_empty(), "resident replay state must be sole");
                    (state, None)
                } else {
                    let Some(Recipe {
                        checkpoint,
                        choices,
                        steps,
                    }) = frontier.pop_front()
                    else {
                        return Step::Exhausted;
                    };
                    let mut state = checkpoint.as_ref().clone();
                    let mut decisions = choices.iter();
                    for _ in 0..steps {
                        let choice = if state.needs_choice() {
                            Some(*decisions.next().expect("recorded choice"))
                        } else {
                            None
                        };
                        assert!(
                            matches!(
                                state.step(&self.prepared.rules, choice, &mut Recorder::new(false)),
                                Event::Progress
                            ),
                            "replayed nonterminal prefix"
                        );
                    }
                    assert!(decisions.next().is_none());
                    (state, Some((checkpoint, choices, steps)))
                };
                // A sole live branch needs a checkpoint only if it now forks.
                // Retain the actual pending choice to restore the pre-fork state.
                let pending_choice = if saved.is_none() && state.needs_choice() {
                    state.pending.last().cloned()
                } else {
                    None
                };
                match state.step(&self.prepared.rules, None, &mut Recorder::new(false)) {
                    Event::Progress if frontier.is_empty() => *resident = Some(state),
                    Event::Progress => {
                        let (checkpoint, choices, steps) = saved.expect("competing saved branch");
                        if interval.is_some_and(|n| steps + 1 >= n.get()) {
                            frontier.push_back(Recipe {
                                checkpoint: Arc::new(state),
                                choices: vec![],
                                steps: 0,
                            });
                        } else {
                            frontier.push_back(Recipe {
                                checkpoint,
                                choices,
                                steps: steps + 1,
                            });
                        }
                    }
                    Event::Failed => (),
                    Event::Answer => return Step::Answer(state.answer(&self.outputs)),
                    Event::Fork(_, _) => {
                        let (checkpoint, choices, steps) = if let Some(saved) = saved {
                            saved
                        } else {
                            state
                                .pending
                                .push(pending_choice.expect("fork had a pending choice"));
                            (Arc::new(state), vec![], 0)
                        };
                        for choice in [false, true] {
                            let mut child_choices = choices.clone();
                            child_choices.push(choice);
                            frontier.push_back(Recipe {
                                checkpoint: checkpoint.clone(),
                                choices: child_choices,
                                steps: steps + 1,
                            });
                        }
                    }
                }
            }
        }
        Step::Progress
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chr_syntax::{and, atom, c, eq, v};
    #[cfg(feature = "replay-diagnostic")]
    #[test]
    fn sole_replay_branch_executes_one_source_step_per_service() {
        let rules = [Rule::simplify(
            "loop",
            [c("loop", [])],
            c("loop", []).into(),
        )];
        let p = Prepared::new(&rules).unwrap();
        let q = Query {
            constraints: vec![c("loop", [])],
            outputs: vec![],
        };
        for mode in [
            Mode::Replay,
            Mode::Checkpoint(4.try_into().unwrap()),
            Mode::Checkpoint(16.try_into().unwrap()),
        ] {
            let mut engine = p.start(&q, mode).unwrap();
            diagnostics::reset();
            for _ in 0..100 {
                assert_eq!(engine.advance(), Step::Progress);
            }
            assert_eq!(diagnostics::steps(), 100);
        }
    }
    #[test]
    fn root_precheck_preserves_full_matching_with_aliases_and_pattern_scopes() {
        use chr_syntax::t;
        let terms = vec![
            v(0),
            v(1),
            atom("a"),
            atom("b"),
            t("f", [v(0)]),
            t("f", [v(1)]),
            t("f", [atom("a")]),
            t("g", [atom("a")]),
            t("pair", [v(0), v(1)]),
        ];
        let mut environments = vec![Slots::new()];
        for slot in [Var(0), Var(1)] {
            for term in &terms {
                environments.push(BTreeMap::from([(slot, term.clone())]));
            }
        }
        let mut accepted = 0;
        let mut rejected = 0;
        for equation in [
            None,
            Some((v(0), atom("a"))),
            Some((v(0), v(1))),
            Some((v(0), t("f", [v(1)]))),
            Some((v(1), atom("b"))),
        ] {
            let mut state = State::default();
            if let Some((a, b)) = equation {
                assert!(state.unify(&a, &b, &mut Recorder::new(false)));
            }
            for a in &terms {
                for b in &terms {
                    let head = c("row", [a.clone(), b.clone()]);
                    for x in &terms {
                        for y in &terms {
                            let candidate = c("row", [x.clone(), y.clone()]);
                            for env in &environments {
                                let possible = state.roots_compatible(&head, &candidate, env);
                                let mut slots = env.clone();
                                let full = head
                                    .args
                                    .iter()
                                    .zip(&candidate.args)
                                    .all(|(p, t)| matches(p, &state.resolve(t), &mut slots));
                                if full {
                                    accepted += 1;
                                    assert!(possible, "precheck rejected a full match");
                                }
                                if !possible {
                                    rejected += 1;
                                }
                            }
                        }
                    }
                }
            }
            // An unbound pattern slot 0 must not read query binding 0.
            assert!(state.roots_compatible(
                &c("row", [v(0)]),
                &c("row", [atom("other")]),
                &BTreeMap::from([(Var(1), atom("known"))])
            ));
        }
        assert!(accepted > 1000 && rejected > 1000);
    }
    #[test]
    fn lone_branch_releases_obsolete_checkpoint_and_trail_history() {
        let rules = [Rule::simplify(
            "loop",
            [c("loop", [])],
            c("loop", []).into(),
        )];
        let prepared = Prepared::new(&rules).unwrap();
        let q = Query {
            constraints: vec![c("loop", [])],
            outputs: vec![],
        };
        for interval in [1, 4, 16] {
            let mut engine = prepared
                .start(&q, Mode::Checkpoint(NonZeroUsize::new(interval).unwrap()))
                .unwrap();
            for _ in 0..100 {
                assert_eq!(engine.advance(), Step::Progress);
            }
            match &engine.backend {
                Backend::Replay {
                    frontier, resident, ..
                } => {
                    assert!(frontier.is_empty() && resident.is_some())
                }
                _ => unreachable!(),
            }
        }
        let mut engine = prepared.start(&q, Mode::Trail).unwrap();
        let old = match &engine.backend {
            Backend::Trail { current, .. } => Arc::downgrade(current),
            _ => unreachable!(),
        };
        for _ in 0..100 {
            assert!(matches!(engine.advance(), Step::Progress));
        }
        assert!(old.upgrade().is_none(), "obsolete trail root retained");
        match &engine.backend {
            Backend::Trail {
                current, frontier, ..
            } => {
                assert_eq!(frontier.len(), 1);
                assert!(current.parent.is_none());
                assert!(current.changes.is_empty());
            }
            _ => unreachable!(),
        }
    }
    #[test]
    fn returning_to_a_sole_branch_releases_its_saved_checkpoint() {
        use chr_syntax::{Goal, or};
        let rules = [
            Rule::simplify(
                "start",
                [c("start", [])],
                or(Goal::Fail, c("loop", []).into()),
            ),
            Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
        ];
        let prepared = Prepared::new(&rules).unwrap();
        let query = Query {
            constraints: vec![c("start", [])],
            outputs: vec![],
        };
        for mode in [
            Mode::Replay,
            Mode::Checkpoint(1.try_into().unwrap()),
            Mode::Checkpoint(16.try_into().unwrap()),
        ] {
            let mut engine = prepared.start(&query, mode).unwrap();
            let mut saved = None;
            for _ in 0..20 {
                assert_eq!(engine.advance(), Step::Progress);
                if let Backend::Replay { frontier, .. } = &engine.backend
                    && let Some(Recipe { checkpoint, .. }) = frontier.front()
                {
                    saved = Some(Arc::downgrade(checkpoint));
                    break;
                }
            }
            let saved = saved.expect("fork must create a shared checkpoint");
            assert!(saved.upgrade().is_some());
            for _ in 0..100 {
                assert_eq!(engine.advance(), Step::Progress);
            }
            assert!(
                saved.upgrade().is_none(),
                "sole branch retained saved history"
            );
            match &engine.backend {
                Backend::Replay {
                    frontier, resident, ..
                } => assert!(frontier.is_empty() && resident.is_some()),
                _ => unreachable!(),
            }
        }
    }
    #[test]
    fn change_sequence_restores_all_mutable_state_then_redoes_it() {
        let rules = vec![
            Rule::propagate("watch", [c("token", [v(0)])], c("seen", [v(0)]).into()),
            Rule::simplify(
                "take",
                [c("token", [v(0)])],
                and(vec![eq(v(0), atom("a")), c("local", [v(1), v(1)]).into()]),
            ),
        ];
        let q = Query {
            constraints: vec![c("token", [v(50)])],
            outputs: vec![],
        };
        let mut s = State::new(&q).unwrap();
        let initial = s.clone();
        let mut log = Recorder::new(true);
        for _ in 0..20 {
            if matches!(s.step(&rules, None, &mut log), Event::Answer) {
                break;
            }
        }
        let final_state = s.clone();
        assert_ne!(initial, final_state);
        assert!(s.next_var > initial.next_var);
        assert!(s.next_occ > initial.next_occ);
        assert!(!s.bindings.is_empty());
        assert!(!s.history.is_empty());
        for edit in log.changes.iter().rev() {
            edit.apply(&mut s, false);
        }
        assert_eq!(s, initial);
        for edit in &log.changes {
            edit.apply(&mut s, true);
        }
        assert_eq!(s, final_state);
    }
}

/// Work-only instrumentation and a deterministic-prefix projection. Absent from
/// timing builds. Projection executes each source step once, preserving FIFO
/// branch decisions, and sums the prefix each root-replay service would repeat.
#[cfg(feature = "replay-diagnostic")]
pub mod diagnostics {
    use super::*;
    use std::cell::Cell;
    thread_local! { static STEPS: Cell<u64> = const { Cell::new(0) }; }
    pub(super) fn record_step() {
        STEPS.with(|n| n.set(n.get() + 1));
    }
    thread_local! { static ENVIRONMENTS: Cell<[u64; 5]> = const { Cell::new([0; 5]) }; }
    pub fn reset() {
        STEPS.with(|n| n.set(0));
        ENVIRONMENTS.with(|n| n.set([0; 5]));
    }
    pub fn environments() -> [u64; 5] {
        ENVIRONMENTS.with(Cell::get)
    }
    pub(super) fn record_precheck_rejection() {
        ENVIRONMENTS.with(|count| {
            let mut n = count.get();
            n[4] += 1;
            count.set(n);
        });
    }
    pub(super) fn record_environment(env: &Slots, rejected: bool) {
        if env.is_empty() {
            return;
        }
        fn nodes(t: &Term) -> u64 {
            match t {
                Term::Var(_) => 1,
                Term::App(_, xs) => 1 + xs.iter().map(nodes).sum::<u64>(),
            }
        }
        let size = env.values().map(nodes).sum::<u64>();
        ENVIRONMENTS.with(|count| {
            let mut n = count.get();
            n[0] += 1;
            n[1] += size;
            if rejected {
                n[2] += 1;
                n[3] += size;
            }
            count.set(n);
        });
    }
    pub fn steps() -> u64 {
        STEPS.with(Cell::get)
    }
    pub struct Projection {
        pub service_steps: u64,
        pub root_replay_steps: u64,
        pub answers: Vec<Answer>,
    }
    pub fn project(rules: &[Rule], query: &Query, bound: usize) -> Projection {
        let mut frontier = VecDeque::from([(State::new(query).unwrap(), None::<u64>)]);
        let mut result = Projection {
            service_steps: 0,
            root_replay_steps: 0,
            answers: vec![],
        };
        for _ in 0..bound {
            let Some((mut state, prefix)) = frontier.pop_front() else {
                return result;
            };
            result.service_steps += 1;
            result.root_replay_steps += prefix.map_or(1, |n| n + 1);
            match state.step(rules, None, &mut Recorder::new(false)) {
                Event::Progress => {
                    let next = if frontier.is_empty() {
                        None
                    } else {
                        Some(prefix.expect("saved competing branch") + 1)
                    };
                    frontier.push_back((state, next));
                }
                Event::Failed => (),
                Event::Answer => result.answers.push(state.answer(&query.outputs)),
                Event::Fork(a, b) => {
                    let mut right = state.clone();
                    state.pending.push(a);
                    right.pending.push(b);
                    frontier.push_back((state, Some(prefix.map_or(1, |n| n + 1))));
                    frontier.push_back((right, Some(prefix.map_or(1, |n| n + 1))));
                }
            }
        }
        panic!("projection source-service cutoff");
    }
}
