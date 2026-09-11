use crate::{
    Stats, Support,
    equality::{self, Bindings, Delta, Edge},
};
use chr_syntax::{Answer, Constraint, Goal, Guard, Query, Rule, Term, Var};
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;
pub(crate) type Scope = BTreeMap<Var, Term>;
pub(crate) type Token = (usize, Vec<u64>);
#[derive(Clone)]
pub(crate) enum Kind {
    Insert(Constraint),
    Equal(Term, Term),
    And(Vec<Rc<Work>>),
    Or(Rc<Work>, Rc<Work>),
    True,
    Fail,
}
#[derive(Clone)]
pub(crate) struct Work {
    pub id: u64,
    pub kind: Kind,
}
pub(crate) struct Frame {
    pub ticket: u64,
    pub pending: VecDeque<Rc<Work>>,
}
struct Occurrence {
    constraint: Constraint,
    live: Support,
}
pub(crate) enum Operation {
    Pending(Rc<Work>),
    Apply {
        rule: usize,
        ids: Vec<u64>,
        scope: Scope,
    },
    Done,
}
pub(crate) struct Kernel {
    rules: Vec<Rule>,
    store: BTreeMap<u64, Occurrence>,
    bindings: Bindings,
    history: BTreeMap<Token, Support>,
    vars: BTreeMap<Var, Support>,
    births: BTreeMap<u64, Support>,
    outputs: Vec<(String, Term)>,
    next_var: u64,
    next_occ: u64,
    next_work: u64,
}
struct Allocator<'a> {
    next: &'a mut u64,
    births: &'a mut BTreeMap<Var, Support>,
    support: &'a Support,
}
fn instantiate(
    term: &Term,
    scope: &mut Scope,
    alloc: &mut Allocator<'_>,
    stats: &mut Stats,
) -> Term {
    stats.term_visits += 1;
    match term {
        Term::Var(v) => {
            if let Some(t) = scope.get(v) {
                return equality::copy(t, stats);
            }
            let fresh = Var(*alloc.next);
            *alloc.next += 1;
            alloc.births.insert(fresh, alloc.support.clone());
            stats.support_writes += alloc.support.len() as u64;
            let term = Term::Var(fresh);
            scope.insert(*v, term.clone());
            term
        }
        Term::App(n, args) => Term::App(
            n.clone(),
            args.iter()
                .map(|t| instantiate(t, scope, alloc, stats))
                .collect(),
        ),
    }
}
fn compile(
    goal: &Goal,
    scope: &mut Scope,
    alloc: &mut Allocator<'_>,
    next: &mut u64,
    stats: &mut Stats,
) -> Rc<Work> {
    let kind = match goal {
        Goal::Constraint(c) => Kind::Insert(Constraint {
            name: c.name.clone(),
            args: c
                .args
                .iter()
                .map(|t| instantiate(t, scope, alloc, stats))
                .collect(),
        }),
        Goal::Unify(a, b) => Kind::Equal(
            instantiate(a, scope, alloc, stats),
            instantiate(b, scope, alloc, stats),
        ),
        Goal::And(goals) => Kind::And(
            goals
                .iter()
                .map(|g| compile(g, scope, alloc, next, stats))
                .collect(),
        ),
        Goal::Or(a, b) => Kind::Or(
            compile(a, scope, alloc, next, stats),
            compile(b, scope, alloc, next, stats),
        ),
        Goal::True => Kind::True,
        Goal::Fail => Kind::Fail,
    };
    let id = *next;
    *next += 1;
    stats.work_nodes += 1;
    Rc::new(Work { id, kind })
}
fn inherit(support: &mut Support, parent: u64, left: u64, right: u64, stats: &mut Stats) {
    stats.support_reads += 1;
    if support.remove(&parent) {
        support.insert(left);
        support.insert(right);
        stats.support_writes += 3;
    }
}
fn clone_scope(scope: &Scope, stats: &mut Stats) -> Scope {
    scope
        .iter()
        .map(|(&v, t)| (v, equality::copy(t, stats)))
        .collect()
}
impl Kernel {
    pub fn new(rules: Vec<Rule>, query: Query, stats: &mut Stats) -> (Self, Frame) {
        let support = Support::from([0]);
        let mut scope = Scope::new();
        let mut vars = BTreeMap::new();
        let mut next_var = 0;
        let mut next_work = 0;
        let mut alloc = Allocator {
            next: &mut next_var,
            births: &mut vars,
            support: &support,
        };
        let pending = query
            .constraints
            .into_iter()
            .map(|c| {
                compile(
                    &Goal::Constraint(c),
                    &mut scope,
                    &mut alloc,
                    &mut next_work,
                    stats,
                )
            })
            .collect();
        let outputs = query
            .outputs
            .into_iter()
            .map(|(name, var)| {
                (
                    name,
                    instantiate(&Term::Var(var), &mut scope, &mut alloc, stats),
                )
            })
            .collect();
        (
            Self {
                rules,
                store: BTreeMap::new(),
                bindings: Bindings::new(),
                history: BTreeMap::new(),
                vars,
                births: BTreeMap::new(),
                outputs,
                next_var,
                next_occ: 0,
                next_work,
            },
            Frame { ticket: 0, pending },
        )
    }
    pub fn preview(&self, ticket: u64, pending: Option<Rc<Work>>, stats: &mut Stats) -> Operation {
        stats.previewed += 1;
        if let Some(job) = pending {
            return Operation::Pending(job);
        }
        for (index, rule) in self.rules.iter().enumerate() {
            let heads = rule.kept.iter().chain(&rule.removed).collect::<Vec<_>>();
            if let Some((ids, scope)) =
                self.find(ticket, index, rule, &heads, vec![], Scope::new(), stats)
            {
                return Operation::Apply {
                    rule: index,
                    ids,
                    scope,
                };
            }
        }
        Operation::Done
    }
    #[allow(clippy::too_many_arguments)]
    fn find(
        &self,
        ticket: u64,
        index: usize,
        rule: &Rule,
        heads: &[&Constraint],
        ids: Vec<u64>,
        scope: Scope,
        stats: &mut Stats,
    ) -> Option<(Vec<u64>, Scope)> {
        if heads.is_empty() {
            if let Some(s) = self.history.get(&(index, ids.clone())) {
                stats.support_reads += 1;
                if s.contains(&ticket) {
                    return None;
                }
            }
            let mut virtual_next = self.next_var;
            let mut virtual_births = BTreeMap::new();
            let virtual_support = Support::new();
            let mut alloc = Allocator {
                next: &mut virtual_next,
                births: &mut virtual_births,
                support: &virtual_support,
            };
            let mut guards_scope = clone_scope(&scope, stats);
            for Guard::Equal(a, b) in &rule.guards {
                let a = instantiate(a, &mut guards_scope, &mut alloc, stats);
                let b = instantiate(b, &mut guards_scope, &mut alloc, stats);
                if !equality::equal(&a, &b, ticket, &self.bindings, stats) {
                    return None;
                }
            }
            return Some((ids, scope));
        }
        let head = heads[0];
        for (&id, occ) in &self.store {
            stats.occurrence_scans += 1;
            if ids.contains(&id)
                || occ.constraint.name != head.name
                || occ.constraint.args.len() != head.args.len()
            {
                continue;
            }
            stats.support_reads += 1;
            if !occ.live.contains(&ticket) {
                continue;
            }
            stats.match_candidates += 1;
            let mut trial = clone_scope(&scope, stats);
            if head
                .args
                .iter()
                .zip(&occ.constraint.args)
                .all(|(p, t)| self.matches(p, t, ticket, &mut trial, stats))
            {
                let mut chosen = ids.clone();
                chosen.push(id);
                if let Some(found) =
                    self.find(ticket, index, rule, &heads[1..], chosen, trial, stats)
                {
                    return Some(found);
                }
            }
        }
        None
    }
    fn matches(
        &self,
        pattern: &Term,
        value: &Term,
        ticket: u64,
        scope: &mut Scope,
        stats: &mut Stats,
    ) -> bool {
        stats.term_visits += 1;
        match pattern {
            Term::Var(v) => match scope.get(v) {
                Some(old) => equality::equal(old, value, ticket, &self.bindings, stats),
                None => {
                    scope.insert(*v, equality::copy(value, stats));
                    true
                }
            },
            Term::App(n, args) => {
                match equality::deref(value, ticket, &self.bindings, &Delta::new(), stats) {
                    Term::App(m, values) => {
                        n == &m
                            && args.len() == values.len()
                            && args
                                .iter()
                                .zip(values)
                                .all(|(p, t)| self.matches(p, &t, ticket, scope, stats))
                    }
                    _ => false,
                }
            }
        }
    }
    pub fn apply(
        &mut self,
        rule: usize,
        ids: Vec<u64>,
        mut scope: Scope,
        support: &Support,
        stats: &mut Stats,
    ) -> Rc<Work> {
        let kept = self.rules[rule].kept.len();
        for id in ids.iter().skip(kept) {
            let occ = self.store.get_mut(id).expect("selected occurrence exists");
            for ticket in support {
                stats.support_reads += 1;
                assert!(occ.live.remove(ticket));
                stats.support_writes += 1;
            }
            if occ.live.is_empty() {
                self.store.remove(id);
            }
        }
        let entry = self.history.entry((rule, ids)).or_default();
        for &ticket in support {
            stats.support_reads += 1;
            assert!(entry.insert(ticket));
            stats.support_writes += 1;
        }
        let mut alloc = Allocator {
            next: &mut self.next_var,
            births: &mut self.vars,
            support,
        };
        let body = compile(
            &self.rules[rule].body,
            &mut scope,
            &mut alloc,
            &mut self.next_work,
            stats,
        );
        stats.expansions += 1;
        stats.projected_applications += support.len() as u64;
        body
    }
    pub fn insert(&mut self, constraint: &Constraint, support: &Support, stats: &mut Stats) {
        let id = self.next_occ;
        self.next_occ += 1;
        let constraint = Constraint {
            name: constraint.name.clone(),
            args: constraint
                .args
                .iter()
                .map(|t| equality::copy(t, stats))
                .collect(),
        };
        self.store.insert(
            id,
            Occurrence {
                constraint,
                live: support.clone(),
            },
        );
        self.births.insert(id, support.clone());
        stats.support_writes += 2 * support.len() as u64;
        stats.introductions += 1;
        stats.projected_introductions += support.len() as u64;
    }
    pub fn equate(&mut self, a: &Term, b: &Term, ticket: u64, stats: &mut Stats) -> bool {
        let Some(delta) = equality::unify(a, b, ticket, &self.bindings, stats) else {
            return false;
        };
        for (var, target) in delta {
            let edges = self.bindings.entry(var).or_default();
            for edge in edges.iter() {
                stats.support_reads += 1;
                assert!(
                    !edge.support.contains(&ticket),
                    "transaction overwrites a binding"
                );
            }
            if let Some(edge) = edges.iter_mut().find(|e| {
                stats.binding_scans += 1;
                e.target == target
            }) {
                edge.support.insert(ticket);
            } else {
                edges.push(Edge {
                    target,
                    support: Support::from([ticket]),
                });
            }
            stats.support_writes += 1;
        }
        true
    }
    pub fn fork(&mut self, parent: u64, left: u64, right: u64, stats: &mut Stats) {
        for s in self.vars.values_mut() {
            inherit(s, parent, left, right, stats);
        }
        for s in self.births.values_mut() {
            inherit(s, parent, left, right, stats);
        }
        for occ in self.store.values_mut() {
            inherit(&mut occ.live, parent, left, right, stats);
        }
        for edges in self.bindings.values_mut() {
            for e in edges {
                inherit(&mut e.support, parent, left, right, stats);
            }
        }
        for s in self.history.values_mut() {
            inherit(s, parent, left, right, stats);
        }
    }
    pub fn answer(&self, ticket: u64, stats: &mut Stats) -> Answer {
        let outputs = self
            .outputs
            .iter()
            .map(|(n, t)| {
                (
                    n.clone(),
                    equality::resolved(t, ticket, &self.bindings, stats),
                )
            })
            .collect();
        let mut residual = vec![];
        for o in self.store.values() {
            stats.support_reads += 1;
            if o.live.contains(&ticket) {
                residual.push(Constraint {
                    name: o.constraint.name.clone(),
                    args: o
                        .constraint
                        .args
                        .iter()
                        .map(|t| equality::resolved(t, ticket, &self.bindings, stats))
                        .collect(),
                });
            }
        }
        Answer { outputs, residual }
    }
    pub fn retained(&self) -> crate::Retained {
        crate::Retained {
            occurrences: self.store.len(),
            variables: self.vars.len(),
            occurrence_births: self.births.len(),
            binding_edges: self.bindings.values().map(Vec::len).sum(),
            tokens: self.history.len(),
            support_memberships: self.store.values().map(|o| o.live.len()).sum::<usize>()
                + self.vars.values().map(Support::len).sum::<usize>()
                + self.births.values().map(Support::len).sum::<usize>()
                + self
                    .bindings
                    .values()
                    .flatten()
                    .map(|e| e.support.len())
                    .sum::<usize>()
                + self.history.values().map(Support::len).sum::<usize>(),
        }
    }
    pub fn project(&self, frame: &Frame) -> crate::Projection {
        fn goal(w: &Work) -> Goal {
            match &w.kind {
                Kind::Insert(c) => Goal::Constraint(c.clone()),
                Kind::Equal(a, b) => Goal::Unify(a.clone(), b.clone()),
                Kind::And(ws) => Goal::And(ws.iter().map(|w| goal(w)).collect()),
                Kind::Or(a, b) => Goal::Or(Box::new(goal(a)), Box::new(goal(b))),
                Kind::True => Goal::True,
                Kind::Fail => Goal::Fail,
            }
        }
        let ticket = frame.ticket;
        crate::Projection {
            ticket,
            pending: frame.pending.iter().map(|w| (w.id, goal(w))).collect(),
            store: self
                .store
                .iter()
                .filter(|(_, o)| o.live.contains(&ticket))
                .map(|(&id, o)| (id, o.constraint.clone()))
                .collect(),
            bindings: self
                .bindings
                .iter()
                .flat_map(|(&var, edges)| {
                    edges
                        .iter()
                        .filter(move |e| e.support.contains(&ticket))
                        .map(move |e| (var, e.target.clone()))
                })
                .collect(),
            history: self
                .history
                .iter()
                .filter(|(_, s)| s.contains(&ticket))
                .map(|(token, _)| token.clone())
                .collect(),
            born_variables: self
                .vars
                .iter()
                .filter(|(_, s)| s.contains(&ticket))
                .map(|(&var, _)| var)
                .collect(),
            born_occurrences: self
                .births
                .iter()
                .filter(|(_, s)| s.contains(&ticket))
                .map(|(&id, _)| id)
                .collect(),
        }
    }
    pub fn validate(&self, frames: &VecDeque<Frame>) -> Result<(), String> {
        fn variables(
            t: &Term,
            action: &mut impl FnMut(Var) -> Result<(), String>,
        ) -> Result<(), String> {
            match t {
                Term::Var(v) => action(*v),
                Term::App(_, args) => {
                    for a in args {
                        variables(a, action)?;
                    }
                    Ok(())
                }
            }
        }
        fn pending(
            w: &Work,
            action: &mut impl FnMut(Var) -> Result<(), String>,
        ) -> Result<(), String> {
            match &w.kind {
                Kind::Insert(c) => {
                    for t in &c.args {
                        variables(t, action)?;
                    }
                    Ok(())
                }
                Kind::Equal(a, b) => {
                    variables(a, action)?;
                    variables(b, action)
                }
                Kind::And(ws) => {
                    for w in ws {
                        pending(w, action)?;
                    }
                    Ok(())
                }
                Kind::Or(a, b) => {
                    pending(a, action)?;
                    pending(b, action)
                }
                _ => Ok(()),
            }
        }
        for f in frames {
            let born = |v: Var| self.vars.get(&v).is_some_and(|s| s.contains(&f.ticket));
            let mut check = |v| {
                if born(v) {
                    Ok(())
                } else {
                    Err(format!("variable {v:?} absent in ticket {}", f.ticket))
                }
            };
            for (_, t) in &self.outputs {
                variables(t, &mut check)?;
            }
            for w in &f.pending {
                pending(w, &mut check)?;
            }
            for (&id, o) in &self.store {
                if o.live.contains(&f.ticket) {
                    if !self.births[&id].contains(&f.ticket) {
                        return Err("live occurrence outside birth".into());
                    }
                    for t in &o.constraint.args {
                        variables(t, &mut check)?;
                    }
                }
            }
            let mut projected = BTreeMap::new();
            for (&var, edges) in &self.bindings {
                for e in edges {
                    if e.support.contains(&f.ticket) {
                        check(var)?;
                        variables(&e.target, &mut check)?;
                        if projected.insert(var, &e.target).is_some() {
                            return Err("overlapping bindings".into());
                        }
                    }
                }
            }
            fn acyclic(
                var: Var,
                p: &BTreeMap<Var, &Term>,
                path: &mut Support,
                done: &mut Support,
            ) -> Result<(), String> {
                if done.contains(&var.0) {
                    return Ok(());
                }
                if !path.insert(var.0) {
                    return Err("projected occurs cycle".into());
                }
                if let Some(t) = p.get(&var) {
                    variables(t, &mut |v| acyclic(v, p, path, done))?;
                }
                path.remove(&var.0);
                done.insert(var.0);
                Ok(())
            }
            let mut done = Support::new();
            for &var in projected.keys() {
                acyclic(var, &projected, &mut Support::new(), &mut done)?;
            }
            for ((rule, ids), s) in &self.history {
                if s.contains(&f.ticket) {
                    if ids.len() != self.rules[*rule].kept.len() + self.rules[*rule].removed.len() {
                        return Err("bad history arity".into());
                    }
                    for id in ids {
                        if !self.births[id].contains(&f.ticket) {
                            return Err("token outside occurrence birth".into());
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
