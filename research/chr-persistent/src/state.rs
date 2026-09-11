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
        if crate::COLLECT_METRICS {
            stats.pending_allocations += 1;
        }
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
            if crate::COLLECT_METRICS {
                stats.storage.snapshot_copies += 1;
            }
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
        Self::new_replaying(query, vec![], arena, stats)
    }
    pub fn new_replaying(
        query: Query,
        equations: Vec<(Var, Source)>,
        arena: &mut Arena,
        stats: &mut Stats,
    ) -> Self {
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
        for (var, value) in equations.into_iter().rev() {
            let equation = work(
                &Goal::Unify(Source::Var(var), value),
                arena,
                &mut scope,
                &mut next_var,
                stats,
            );
            pending.push(equation, stats);
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
                    if crate::COLLECT_METRICS {
                        stats.introductions += 1;
                    }
                }
                Work::Equal(a, b) => {
                    if crate::COLLECT_METRICS {
                        stats.equations += 1;
                    }
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
                if crate::COLLECT_METRICS {
                    stats.applications += 1;
                }
                return Event::Continue;
            }
        }
        Event::Complete
    }
    pub(crate) fn has_pending_work(&self) -> bool {
        self.pending.0.is_some()
    }
    pub(crate) fn take_body(&mut self, arena: &mut Arena, stats: &mut Stats) -> (Goal, u64) {
        fn export(w: &Work, arena: &Arena, bindings: &Bindings, stats: &mut Stats) -> Goal {
            match w {
                Work::Insert(p, args) => Goal::Constraint(Constraint {
                    name: arena.predicates()[*p].0.clone(),
                    args: args
                        .iter()
                        .map(|t| arena.export(*t, bindings, stats))
                        .collect(),
                }),
                Work::Equal(a, b) => Goal::Unify(
                    arena.export(*a, bindings, stats),
                    arena.export(*b, bindings, stats),
                ),
                Work::And(gs) => Goal::And(
                    gs.iter()
                        .map(|g| export(g, arena, bindings, stats))
                        .collect(),
                ),
                Work::Or(a, b) => Goal::Or(
                    Box::new(export(a, arena, bindings, stats)),
                    Box::new(export(b, arena, bindings, stats)),
                ),
                Work::True => Goal::True,
                Work::Fail => Goal::Fail,
            }
        }
        let body = self.pending.pop().expect("selected rule body");
        assert!(
            self.pending.0.is_none(),
            "body extraction requires a single selected body"
        );
        (export(&body, arena, &self.bindings, stats), self.next_var)
    }
    pub(crate) fn take_private_call(
        &mut self,
        rules: &[Rule],
        count: usize,
        arena: &mut Arena,
        stats: &mut Stats,
    ) -> Option<(Constraint, u64)> {
        if self.pending.0.is_some() {
            return None;
        }
        let family = rules[..count]
            .iter()
            .map(|r| (&r.removed[0].name, r.removed[0].args.len()))
            .collect::<std::collections::BTreeSet<_>>();
        let candidates = self
            .store
            .entries(&mut stats.storage)
            .into_iter()
            .filter(|((p, _), _)| {
                let (name, arity) = &arena.predicates()[*p];
                family.contains(&(name, *arity))
            })
            .collect::<Vec<_>>();
        if candidates.len() != 1 {
            return None;
        }
        if !rules[..count].iter().enumerate().any(|(id, r)| {
            self.find(id, r, &[&r.removed[0]], vec![], Scope::new(), arena, stats)
                .is_some()
        }) {
            return None;
        }
        let (key, args) = &candidates[0];
        let call = Constraint {
            name: arena.predicates()[key.0].0.clone(),
            args: args
                .iter()
                .map(|t| arena.export(*t, &self.bindings, stats))
                .collect(),
        };
        self.store.remove(key, &mut stats.storage);
        Some((call, self.next_var))
    }
    pub(crate) fn resume_private(
        &mut self,
        equations: Vec<(Var, Source)>,
        residual: Vec<Constraint>,
        arena: &mut Arena,
        stats: &mut Stats,
    ) -> bool {
        fn import(t: &Source, next: &mut u64, arena: &mut Arena, stats: &mut Stats) -> Term {
            match t {
                Source::Var(Var(id)) => {
                    *next = (*next).max(id.checked_add(1).expect("variable overflow"));
                    Term::Var(*id)
                }
                Source::App(n, args) => {
                    let args = args.iter().map(|t| import(t, next, arena, stats)).collect();
                    arena.make(n, args, stats)
                }
            }
        }
        for (Var(id), t) in equations {
            let value = import(&t, &mut self.next_var, arena, stats);
            if !arena.unify(Term::Var(id), value, &mut self.bindings, stats) {
                return false;
            }
        }
        for c in residual {
            let pred = arena.predicate(&c.name, c.args.len());
            let args = c
                .args
                .iter()
                .map(|t| import(t, &mut self.next_var, arena, stats))
                .collect();
            self.store
                .insert((pred, self.next_occ), args, &mut stats.storage);
            self.next_occ += 1;
        }
        true
    }
    /// Move only source-unreadable ground observations out of active execution.
    pub(crate) fn detach_inert_ground(
        &mut self,
        rules: &[Rule],
        arena: &Arena,
        stats: &mut Stats,
    ) -> Vec<Constraint> {
        fn ground(t: &Source) -> bool {
            match t {
                Source::Var(_) => false,
                Source::App(_, xs) => xs.iter().all(ground),
            }
        }
        let mut detached = vec![];
        for (key, args) in self.store.entries(&mut stats.storage) {
            let (name, arity) = &arena.predicates()[key.0];
            if rules
                .iter()
                .flat_map(|r| r.kept.iter().chain(&r.removed))
                .any(|c| c.name == *name && c.args.len() == *arity)
            {
                continue;
            }
            let args = args
                .iter()
                .map(|&t| arena.export(t, &self.bindings, stats))
                .collect::<Vec<_>>();
            if args.iter().all(ground) {
                self.store.remove(&key, &mut stats.storage);
                detached.push(Constraint {
                    name: name.clone(),
                    args,
                });
            }
        }
        detached
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
                name: arena.predicates()[pred].0.clone(),
                args: args
                    .iter()
                    .map(|t| arena.export(*t, &self.bindings, stats))
                    .collect(),
            })
            .collect();
        if crate::COLLECT_METRICS {
            export.answers += 1;
        }
        if crate::COLLECT_METRICS {
            export.dereferences += stats.dereferences - before.0;
        }
        if crate::COLLECT_METRICS {
            export.storage_visits += stats.storage.visits - before.1;
        }
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
        if crate::COLLECT_METRICS {
            stats.snapshots += 1;
        }
        if crate::COLLECT_METRICS {
            stats.residual_occurrences += residual.len() as u64;
        }
        if crate::COLLECT_METRICS {
            stats.store_visits += storage.visits;
        }
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
            if crate::COLLECT_METRICS {
                stats.head_candidates += 1;
            }
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
                    let n = arena.node(id);
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
    pub(crate) fn take_shared_equation(&mut self) -> (Term, Term, &mut Bindings) {
        let Some(Work::Equal(a, b)) = self.pending.pop() else {
            panic!("expected pending equation")
        };
        (a, b, &mut self.bindings)
    }
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
            term = *arena.node(id).args.get(index)?;
        }
        Some(match deref(term, &self.bindings, stats) {
            Term::Var(v) => TermHead::Variable(v),
            Term::Node(id) => {
                TermHead::Constructor(arena.node(id).name.clone(), arena.node(id).args.len())
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
    pub(crate) fn key_with<T: Ord>(
        &self,
        arena: &mut Arena,
        stats: &mut Stats,
        export: &mut impl FnMut(Term, &Bindings, &mut Arena, &mut Stats) -> T,
    ) -> crate::continuations::StateKey<T> {
        use crate::continuations::{StateKey, WorkKey};
        fn work_key<T>(
            w: &Work,
            arena: &mut Arena,
            bindings: &Bindings,
            stats: &mut Stats,
            export: &mut impl FnMut(Term, &Bindings, &mut Arena, &mut Stats) -> T,
        ) -> WorkKey<T> {
            match w {
                Work::Insert(p, args) => WorkKey::Insert(
                    arena.predicates()[*p].0.clone(),
                    args.iter()
                        .map(|&t| export(t, bindings, arena, stats))
                        .collect(),
                ),
                Work::Equal(a, b) => WorkKey::Equal(
                    export(*a, bindings, arena, stats),
                    export(*b, bindings, arena, stats),
                ),
                Work::And(gs) => WorkKey::And(
                    gs.iter()
                        .map(|g| work_key(g, arena, bindings, stats, export))
                        .collect(),
                ),
                Work::Or(a, b) => WorkKey::Or(
                    Box::new(work_key(a, arena, bindings, stats, export)),
                    Box::new(work_key(b, arena, bindings, stats, export)),
                ),
                Work::True => WorkKey::True,
                Work::Fail => WorkKey::Fail,
            }
        }
        let mut pending = self.pending.clone();
        let mut goals = vec![];
        while let Some(w) = pending.pop() {
            goals.push(work_key(&w, arena, &self.bindings, stats, export));
        }
        let mut store = self
            .store
            .entries(&mut stats.storage)
            .into_iter()
            .map(|((p, id), args)| {
                (
                    arena.predicates()[p].0.clone(),
                    id,
                    args.iter()
                        .map(|&t| export(t, &self.bindings, arena, stats))
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
                .map(|(n, t)| (n.clone(), export(*t, &self.bindings, arena, stats)))
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
