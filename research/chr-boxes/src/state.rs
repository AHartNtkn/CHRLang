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
    Or(Rc<Work>, Rc<Work>, usize),
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
    Answer(Answer),
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
            0,
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
    #[allow(clippy::too_many_arguments)]
    pub fn step(
        &mut self,
        rules: &[Rule],
        safe_rules: &[bool],
        quota: usize,
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
                Work::Or(a, b, spent) => {
                    if quota > 0 && spent < quota {
                        stats.lifting_checks += 1;
                        let mut tail = self.pending.0.as_ref();
                        let mut pure = pure_alternatives(&a, stats) && pure_alternatives(&b, stats);
                        while let Some(node) = tail {
                            pure &= pure_alternatives(&node.0, stats);
                            tail = node.1.0.as_ref();
                        }
                        if !pure {
                            stats.barrier_rejections += 1;
                        }
                        if pure {
                            for (rule_id, rule) in rules.iter().enumerate() {
                                if !safe_rules[rule_id] {
                                    continue;
                                }
                                let heads = vec![&rule.removed[0]];
                                if let Some(app) = self.find(
                                    rule_id,
                                    rule,
                                    &heads,
                                    vec![],
                                    Scope::new(),
                                    arena,
                                    stats,
                                ) {
                                    for key in &app.keys {
                                        self.store.remove(key, &mut stats.storage);
                                    }
                                    self.history.insert(app.token, (), &mut stats.storage);
                                    self.next_var = app.next_var;
                                    self.pending.push(Work::Or(a, b, spent + 1), stats);
                                    self.pending.push(app.body, stats);
                                    stats.applications += 1;
                                    stats.lifted_applications += 1;
                                    stats.max_delay = stats.max_delay.max((spent + 1) as u64);
                                    return Event::Continue;
                                }
                            }
                        }
                    }
                    if quota > 0 && spent >= quota {
                        stats.forced_splits += 1;
                    }

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
        Event::Answer(Answer { outputs, residual })
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

fn pure_alternatives(work: &Work, stats: &mut Stats) -> bool {
    stats.barrier_nodes += 1;
    match work {
        Work::Equal(..) | Work::True | Work::Fail => true,
        Work::And(gs) => gs.iter().all(|g| pure_alternatives(g, stats)),
        Work::Or(a, b, _) => pure_alternatives(a, stats) && pure_alternatives(b, stats),
        Work::Insert(..) => false,
    }
}
/// A deliberately conservative commuting certificate for this first delayed-split probe.
pub(crate) fn certify(rules: &[Rule], stats: &mut Stats) -> Vec<bool> {
    use std::collections::BTreeSet;
    fn variables(t: &Source, vars: &mut BTreeSet<u64>, stats: &mut Stats) -> bool {
        stats.certificate_terms += 1;
        match t {
            Source::Var(Var(v)) => vars.insert(*v),
            Source::App(_, args) => args.iter().all(|a| variables(a, vars, stats)),
        }
    }
    fn subset(t: &Source, vars: &BTreeSet<u64>, stats: &mut Stats) -> bool {
        stats.certificate_terms += 1;
        match t {
            Source::Var(Var(v)) => vars.contains(v),
            Source::App(_, args) => args.iter().all(|a| subset(a, vars, stats)),
        }
    }
    rules
        .iter()
        .enumerate()
        .map(|(i, r)| {
            stats.certificate_checks += 1;
            if !r.kept.is_empty() || r.removed.len() != 1 || !r.guards.is_empty() {
                return false;
            }
            let head = &r.removed[0];
            let mut vars = BTreeSet::new();
            if !head.args.iter().all(|a| variables(a, &mut vars, stats)) {
                return false;
            }
            let Goal::Constraint(body) = &r.body else {
                return false;
            };
            if body.name != head.name
                || body.args.len() != head.args.len()
                || !body.args.iter().all(|a| subset(a, &vars, stats))
            {
                return false;
            }
            !rules.iter().enumerate().any(|(j, other)| {
                i != j
                    && other.kept.iter().chain(&other.removed).any(|h| {
                        stats.certificate_checks += 1;
                        h.name == head.name && h.args.len() == head.args.len()
                    })
            })
        })
        .collect()
}
