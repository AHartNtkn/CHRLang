use crate::{
    Snapshot, Stats,
    map::Map,
    terms::{Arena, Bindings, Scope, Term, deref},
};
use chr_syntax::{Answer, Constraint, Goal, Guard, Query, Rule, Term as Source, Var};
use std::rc::Rc;

#[derive(Clone)]
enum Work {
    Insert(usize, Rc<[Term]>),
    Equal(Term, Term),
    And(Rc<[Work]>),
    Or(Rc<Work>, Rc<Work>),
    True,
    Fail,
}
#[derive(Clone, Default)]
struct Pending(Option<Rc<(Work, Pending)>>);
impl Pending {
    fn push(&mut self, work: Work, stats: &mut Stats) {
        stats.pending_allocations += 1;
        self.0 = Some(Rc::new((work, self.clone())));
    }
    fn pop(&mut self) -> Option<Work> {
        let node = self.0.take()?;
        let (work, tail) = Rc::try_unwrap(node).unwrap_or_else(|n| (*n).clone());
        *self = tail;
        Some(work)
    }
    fn copied(&self, stats: &mut Stats) -> Self {
        let mut work = vec![];
        let mut current = self.clone();
        while let Some(w) = current.pop() {
            work.push(w);
        }
        let mut result = Self::default();
        for w in work.into_iter().rev() {
            result.push(w, stats);
            stats.storage.snapshot_copies += 1;
        }
        result
    }
}
type Key = (usize, u64);
type Token = (usize, Vec<u64>);
#[derive(Clone)]
pub(crate) struct State {
    pending: Pending,
    store: Map<Key, Rc<[Term]>>,
    bindings: Bindings,
    history: Map<Token, ()>,
    next_var: u64,
    next_occ: u64,
    outputs: Rc<[(String, Term)]>,
}
pub(crate) enum Event {
    Continue,
    Split(Box<State>),
    Failed,
    Complete,
}
fn work(
    source: &Goal,
    arena: &mut Arena,
    scope: &mut Scope,
    next: &mut u64,
    stats: &mut Stats,
) -> Work {
    match source {
        Goal::Constraint(c) => {
            let pred = arena.predicate(&c.name, c.args.len());
            Work::Insert(
                pred,
                c.args
                    .iter()
                    .map(|a| arena.instantiate(a, scope, next, stats))
                    .collect(),
            )
        }
        Goal::Unify(a, b) => Work::Equal(
            arena.instantiate(a, scope, next, stats),
            arena.instantiate(b, scope, next, stats),
        ),
        Goal::And(goals) => Work::And(
            goals
                .iter()
                .map(|g| work(g, arena, scope, next, stats))
                .collect(),
        ),
        Goal::Or(a, b) => Work::Or(
            Rc::new(work(a, arena, scope, next, stats)),
            Rc::new(work(b, arena, scope, next, stats)),
        ),
        Goal::True => Work::True,
        Goal::Fail => Work::Fail,
    }
}
struct Application {
    keys: Vec<Key>,
    token: Token,
    body: Work,
    next_var: u64,
}
impl State {
    pub fn new(query: Query, arena: &mut Arena, stats: &mut Stats) -> Self {
        let mut scope = Scope::new();
        let mut next_var = 0;
        let initial = query
            .constraints
            .into_iter()
            .map(|c| {
                work(
                    &Goal::Constraint(c),
                    arena,
                    &mut scope,
                    &mut next_var,
                    stats,
                )
            })
            .collect::<Vec<_>>();
        let outputs = query
            .outputs
            .into_iter()
            .map(|(n, var)| {
                (
                    n,
                    arena.instantiate(&Source::Var(var), &mut scope, &mut next_var, stats),
                )
            })
            .collect();
        let mut pending = Pending::default();
        for w in initial.into_iter().rev() {
            pending.push(w, stats);
        }
        Self {
            pending,
            store: Map::default(),
            bindings: Bindings::default(),
            history: Map::default(),
            next_var,
            next_occ: 0,
            outputs,
        }
    }
    fn snapshot(&self, mode: Snapshot, stats: &mut Stats) -> Self {
        match mode {
            Snapshot::Persistent => self.clone(),
            Snapshot::Copy => Self {
                pending: self.pending.copied(stats),
                store: self.store.copied(&mut stats.storage),
                bindings: self.bindings.copied(&mut stats.storage),
                history: self.history.copied(&mut stats.storage),
                ..self.clone()
            },
        }
    }
    pub fn step(
        &mut self,
        rules: &[Rule],
        arena: &mut Arena,
        mode: Snapshot,
        stats: &mut Stats,
    ) -> Event {
        if let Some(w) = self.pending.pop() {
            match w {
                Work::Insert(pred, args) => {
                    self.store
                        .insert((pred, self.next_occ), args, &mut stats.storage);
                    self.next_occ += 1;
                    stats.introductions += 1;
                }
                Work::Equal(a, b) => {
                    stats.equations += 1;
                    if !arena.unify(a, b, &mut self.bindings, stats) {
                        return Event::Failed;
                    }
                }
                Work::And(goals) => {
                    for g in goals.iter().rev() {
                        self.pending.push(g.clone(), stats);
                    }
                }
                Work::Or(a, b) => {
                    let mut sibling = self.snapshot(mode, stats);
                    self.pending.push((*a).clone(), stats);
                    sibling.pending.push((*b).clone(), stats);
                    return Event::Split(Box::new(sibling));
                }
                Work::True => {}
                Work::Fail => return Event::Failed,
            }
            return Event::Continue;
        }
        for (rule_id, rule) in rules.iter().enumerate() {
            let heads = rule.kept.iter().chain(&rule.removed).collect::<Vec<_>>();
            if let Some(app) = self.find(rule_id, rule, &heads, vec![], Scope::new(), arena, stats)
            {
                for key in app.keys.iter().skip(rule.kept.len()) {
                    self.store.remove(key, &mut stats.storage);
                }
                self.history.insert(app.token, (), &mut stats.storage);
                self.next_var = app.next_var;
                self.pending.push(app.body, stats);
                stats.applications += 1;
                return Event::Continue;
            }
        }
        Event::Complete
    }
    pub(crate) fn export_answer(
        &self,
        arena: &Arena,
        stats: &mut Stats,
        export: &mut crate::EagerExportStats,
    ) -> Answer {
        let before = (stats.dereferences, stats.storage.visits);
        let outputs = self
            .outputs
            .iter()
            .map(|(n, t)| (n.clone(), arena.export(*t, &self.bindings, stats)))
            .collect();
        let residual = self
            .store
            .entries(&mut stats.storage)
            .into_iter()
            .map(|((pred, _), args)| Constraint {
                name: arena.predicates[pred].0.clone(),
                args: args
                    .iter()
                    .map(|t| arena.export(*t, &self.bindings, stats))
                    .collect(),
            })
            .collect();
        export.answers += 1;
        export.dereferences += stats.dereferences - before.0;
        export.storage_visits += stats.storage.visits - before.1;
        Answer { outputs, residual }
    }
    pub(crate) fn into_observation(
        self,
        owner: Rc<()>,
        stats: &mut crate::observation::CaptureStats,
    ) -> crate::observation::CompletedAnswer {
        let mut storage = crate::Storage::default();
        let residual = self
            .store
            .entries(&mut storage)
            .into_iter()
            .map(|((pred, _), args)| (pred, args))
            .collect::<Vec<_>>();
        stats.snapshots += 1;
        stats.residual_occurrences += residual.len() as u64;
        stats.store_visits += storage.visits;
        crate::observation::CompletedAnswer {
            owner,
            outputs: self.outputs,
            bindings: self.bindings,
            residual,
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn find(
        &self,
        rule_id: usize,
        rule: &Rule,
        heads: &[&Constraint],
        keys: Vec<Key>,
        scope: Scope,
        arena: &mut Arena,
        stats: &mut Stats,
    ) -> Option<Application> {
        if heads.is_empty() {
            let token = (rule_id, keys.iter().map(|key| key.1).collect());
            if self.history.get(&token, &mut stats.storage).is_some() {
                return None;
            }
            let mut next = self.next_var;
            let mut scope = scope;
            for guard in &rule.guards {
                let Guard::Equal(a, b) = guard;
                let a = arena.instantiate(a, &mut scope, &mut next, stats);
                let b = arena.instantiate(b, &mut scope, &mut next, stats);
                if !arena.equal(a, b, &self.bindings, stats) {
                    return None;
                }
            }
            let body = work(&rule.body, arena, &mut scope, &mut next, stats);
            return Some(Application {
                keys,
                token,
                body,
                next_var: next,
            });
        }
        let first = heads[0];
        let pred = arena.predicate(&first.name, first.args.len());
        for (key, args) in self
            .store
            .range(&(pred, 0), &(pred, u64::MAX), &mut stats.storage)
        {
            if keys.contains(&key) {
                continue;
            }
            stats.head_candidates += 1;
            let mut trial = scope.clone();
            if first
                .args
                .iter()
                .zip(args.iter())
                .all(|(pattern, &value)| self.matches(pattern, value, &mut trial, arena, stats))
            {
                let mut chosen = keys.clone();
                chosen.push(key);
                if let Some(app) =
                    self.find(rule_id, rule, &heads[1..], chosen, trial, arena, stats)
                {
                    return Some(app);
                }
            }
        }
        None
    }
    fn matches(
        &self,
        pattern: &Source,
        value: Term,
        scope: &mut Scope,
        arena: &Arena,
        stats: &mut Stats,
    ) -> bool {
        let value = deref(value, &self.bindings, stats);
        match pattern {
            Source::Var(Var(id)) => match scope.get(id) {
                Some(&existing) => arena.equal(existing, value, &self.bindings, stats),
                None => {
                    scope.insert(*id, value);
                    true
                }
            },
            Source::App(name, args) => match value {
                Term::Node(id) => {
                    let n = &arena.nodes[id];
                    name == &n.name
                        && args.len() == n.args.len()
                        && args
                            .iter()
                            .zip(&n.args)
                            .all(|(p, &v)| self.matches(p, v, scope, arena, stats))
                }
                Term::Var(_) => false,
            },
        }
    }
}

impl State {
    pub(crate) fn complete_equation(
        &mut self,
        solution: Option<std::collections::BTreeMap<Var, Source>>,
        arena: &mut Arena,
        stats: &mut Stats,
    ) -> bool {
        assert!(matches!(self.pending.pop(), Some(Work::Equal(..))));
        let Some(solution) = solution else {
            return false;
        };
        fn import(t: &Source, next: u64, arena: &mut Arena, stats: &mut Stats) -> Term {
            match t {
                Source::Var(Var(id)) => {
                    assert!(*id < next, "result introduced a fresh hole");
                    Term::Var(*id)
                }
                Source::App(n, args) => {
                    let args = args.iter().map(|t| import(t, next, arena, stats)).collect();
                    arena.make(n, args, stats)
                }
            }
        }
        for (Var(id), t) in solution {
            assert!(
                id < self.next_var && self.bindings.get(&id, &mut stats.storage).is_none(),
                "result overwrote a resolved binding"
            );
            if t == Source::Var(Var(id)) {
                continue;
            }
            let value = import(&t, self.next_var, arena, stats);
            self.bindings.insert(id, value, &mut stats.storage);
        }
        true
    }
    pub(crate) fn has_pending_equation(&self) -> bool {
        self.pending
            .0
            .as_ref()
            .is_some_and(|n| matches!(n.0, Work::Equal(..)))
    }
    pub(crate) fn pending_head(
        &self,
        arena: &Arena,
        stats: &mut Stats,
        left: bool,
        path: &[usize],
    ) -> Option<crate::continuations::TermHead> {
        use crate::continuations::TermHead;
        let Work::Equal(a, b) = &self.pending.0.as_ref()?.0 else {
            return None;
        };
        let mut term = if left { *a } else { *b };
        for &index in path {
            let Term::Node(id) = deref(term, &self.bindings, stats) else {
                return None;
            };
            term = *arena.nodes[id].args.get(index)?;
        }
        Some(match deref(term, &self.bindings, stats) {
            Term::Var(v) => TermHead::Variable(v),
            Term::Node(id) => {
                TermHead::Constructor(arena.nodes[id].name.clone(), arena.nodes[id].args.len())
            }
        })
    }
    pub(crate) fn pending_equation(
        &self,
        arena: &Arena,
        stats: &mut Stats,
    ) -> Option<(Source, Source)> {
        match &self.pending.0.as_ref()?.0 {
            Work::Equal(a, b) => Some((
                arena.export(*a, &self.bindings, stats),
                arena.export(*b, &self.bindings, stats),
            )),
            _ => None,
        }
    }
    pub(crate) fn key(&self, arena: &Arena, stats: &mut Stats) -> crate::continuations::StateKey {
        use crate::continuations::{StateKey, WorkKey};
        fn work_key(w: &Work, arena: &Arena, bindings: &Bindings, stats: &mut Stats) -> WorkKey {
            match w {
                Work::Insert(p, args) => WorkKey::Insert(
                    arena.predicates[*p].0.clone(),
                    args.iter()
                        .map(|&t| arena.export(t, bindings, stats))
                        .collect(),
                ),
                Work::Equal(a, b) => WorkKey::Equal(
                    arena.export(*a, bindings, stats),
                    arena.export(*b, bindings, stats),
                ),
                Work::And(gs) => WorkKey::And(
                    gs.iter()
                        .map(|g| work_key(g, arena, bindings, stats))
                        .collect(),
                ),
                Work::Or(a, b) => WorkKey::Or(
                    Box::new(work_key(a, arena, bindings, stats)),
                    Box::new(work_key(b, arena, bindings, stats)),
                ),
                Work::True => WorkKey::True,
                Work::Fail => WorkKey::Fail,
            }
        }
        let mut pending = self.pending.clone();
        let mut goals = vec![];
        while let Some(w) = pending.pop() {
            goals.push(work_key(&w, arena, &self.bindings, stats));
        }
        let mut store = self
            .store
            .entries(&mut stats.storage)
            .into_iter()
            .map(|((p, id), args)| {
                (
                    arena.predicates[p].0.clone(),
                    id,
                    args.iter()
                        .map(|&t| arena.export(t, &self.bindings, stats))
                        .collect(),
                )
            })
            .collect::<Vec<_>>();
        // Predicate IDs are arena intern indices, not source-visible identities.
        // Within each matching predicate, occurrence IDs preserve selection order.
        store.sort();
        StateKey {
            pending: goals,
            store,
            outputs: self
                .outputs
                .iter()
                .map(|(n, t)| (n.clone(), arena.export(*t, &self.bindings, stats)))
                .collect(),
            history: self
                .history
                .entries(&mut stats.storage)
                .into_iter()
                .map(|(t, ())| t)
                .collect(),
            next_var: self.next_var,
            next_occ: self.next_occ,
        }
    }
}
