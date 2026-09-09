//! Shared immutable syntax with context-local equality and resource ownership.
//! A store gate, not a complete source executor. Matching scans live occurrences.
use crate::{Match, Occurrence, Value};
use chr_syntax::{Constraint, Term, Var};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, VecDeque},
    rc::Rc,
};
type Descriptor = (String, Vec<Value>);
/// Experimental storage contrast for equality state; occurrence ownership is separate.
#[derive(Clone)]
enum EqualityMap<V: Clone> {
    Ordered(Rc<BTreeMap<Value, V>>),
    Persistent(chr_persistent::Map<usize, V>),
}
impl<V: Clone> Default for EqualityMap<V> {
    fn default() -> Self {
        Self::Ordered(Rc::new(BTreeMap::new()))
    }
}
impl<V: Clone> EqualityMap<V> {
    fn get(&self, key: &Value) -> Option<V> {
        match self {
            Self::Ordered(m) => m.get(key).cloned(),
            Self::Persistent(m) => m.get(&key.0, &mut Default::default()),
        }
    }
    fn insert(&mut self, key: Value, value: V) {
        match self {
            Self::Ordered(m) => {
                Rc::make_mut(m).insert(key, value);
            }
            Self::Persistent(m) => m.insert(key.0, value, &mut Default::default()),
        }
    }
    fn remove(&mut self, key: &Value) {
        match self {
            Self::Ordered(m) => {
                Rc::make_mut(m).remove(key);
            }
            Self::Persistent(m) => m.remove(&key.0, &mut Default::default()),
        }
    }
    fn same_root(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ordered(a), Self::Ordered(b)) => Rc::ptr_eq(a, b),
            (Self::Persistent(a), Self::Persistent(b)) => a.same_root(b),
            _ => false,
        }
    }
    fn persistent(&self) -> Self {
        match self {
            Self::Persistent(m) => Self::Persistent(m.clone()),
            Self::Ordered(m) => {
                let mut p = chr_persistent::Map::default();
                for (k, v) in m.iter() {
                    p.insert(k.0, v.clone(), &mut Default::default());
                }
                Self::Persistent(p)
            }
        }
    }
}

#[derive(Default)]
struct Arena {
    nodes: Vec<Option<Descriptor>>,
    next_occurrence: usize,
    next_equality_state: usize,
    deductions: BTreeMap<(usize, Value, Value), Rc<Deduction>>,
}
struct Deduction {
    state: usize,
    parents: EqualityMap<Value>,
    descriptors: EqualityMap<Vec<Descriptor>>,
    children: Vec<(Value, Value)>,
    failed: bool,
}
#[derive(Clone)]
struct Resource {
    name: String,
    args: Vec<Value>,
}
#[derive(Clone, Default)]
pub struct Store {
    arena: Rc<RefCell<Arena>>,
    parents: EqualityMap<Value>,
    descriptors: EqualityMap<Vec<Descriptor>>,
    live: Rc<BTreeMap<Occurrence, Rc<Resource>>>,
    equations: VecDeque<(Value, Value)>,
    failed: bool,
    share_deductions: bool,
    equality_state: usize,
}
impl Store {
    pub fn with_persistent_equality(mut self) -> Self {
        self.parents = self.parents.persistent();
        self.descriptors = self.descriptors.persistent();
        if self.share_deductions {
            self.equality_state = self.fresh_equality_state();
        }
        self
    }

    /// Experimental exact equality-transition reuse; resource ownership stays local.
    pub fn with_shared_deductions(mut self) -> Self {
        if !self.share_deductions {
            self.equality_state = self.fresh_equality_state();
            self.share_deductions = true;
        }
        self
    }
    fn fresh_equality_state(&self) -> usize {
        let mut arena = self.arena.borrow_mut();
        arena.next_equality_state = arena
            .next_equality_state
            .checked_add(1)
            .expect("equality state identity exhausted");
        arena.next_equality_state
    }
    pub fn retained_deductions(&self) -> usize {
        self.arena.borrow().deductions.len()
    }
    pub fn shares_equality(&self, other: &Self) -> bool {
        self.shares_nodes(other)
            && self.parents.same_root(&other.parents)
            && self.descriptors.same_root(&other.descriptors)
    }

    pub fn shares_nodes(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.arena, &other.arena)
    }
    pub fn unknown(&mut self) -> Value {
        self.node(None)
    }
    fn node(&mut self, d: Option<Descriptor>) -> Value {
        let mut arena = self.arena.borrow_mut();
        let id = Value(arena.nodes.len());
        arena.nodes.push(d);
        id
    }
    pub fn constructor(&mut self, name: &str, children: &[Value]) -> Value {
        self.node(Some((name.into(), children.to_vec())))
    }
    pub fn root(&self, mut id: Value) -> Value {
        while let Some(next) = self.parents.get(&id) {
            id = next;
        }
        id
    }
    pub(crate) fn descriptions(&self, id: Value) -> Vec<Descriptor> {
        let id = self.root(id);
        self.descriptors.get(&id).unwrap_or_else(|| {
            self.arena.borrow().nodes[id.0]
                .clone()
                .into_iter()
                .collect()
        })
    }
    pub fn post(&mut self, name: &str, args: &[Value]) -> Occurrence {
        let mut arena = self.arena.borrow_mut();
        let id = Occurrence(arena.next_occurrence);
        arena.next_occurrence = arena
            .next_occurrence
            .checked_add(1)
            .expect("occurrence identity exhausted");
        drop(arena);
        Rc::make_mut(&mut self.live).insert(
            id,
            Rc::new(Resource {
                name: name.into(),
                args: args.to_vec(),
            }),
        );
        id
    }
    pub fn equate(&mut self, a: Value, b: Value) {
        if !self.failed {
            self.equations.push_back((a, b));
        }
    }
    fn reaches(&self, a: Value, b: Value) -> bool {
        let mut todo = vec![a];
        let mut seen = BTreeSet::new();
        while let Some(id) = todo.pop() {
            let id = self.root(id);
            if id == b {
                return true;
            }
            if !seen.insert(id) {
                continue;
            }
            for (_, xs) in self.descriptions(id) {
                todo.extend(xs);
            }
        }
        false
    }
    pub fn step(&mut self) -> bool {
        let Some((a, b)) = self.equations.pop_front() else {
            return false;
        };
        let (a, b) = (self.root(a), self.root(b));
        if a == b {
            return true;
        }
        let key = (self.equality_state, a, b);
        if self.share_deductions {
            let cached = self.arena.borrow().deductions.get(&key).cloned();
            if let Some(d) = cached {
                self.equality_state = d.state;
                self.parents = d.parents.clone();
                self.descriptors = d.descriptors.clone();
                self.failed = d.failed;
                if self.failed {
                    self.equations.clear();
                } else {
                    self.equations.extend(d.children.iter().copied());
                }
                return true;
            }
        }
        let (mut da, db) = (self.descriptions(a), self.descriptions(b));
        if self.reaches(a, b)
            || self.reaches(b, a)
            || da
                .iter()
                .any(|(n, xs)| db.iter().any(|(m, ys)| n != m || xs.len() != ys.len()))
        {
            self.failed = true;
            self.equations.clear();
            self.record_deduction(key, vec![]);
            return true;
        }
        let pending = self.equations.len();
        for (_, xs) in &da {
            for (_, ys) in &db {
                self.equations
                    .extend(xs.iter().copied().zip(ys.iter().copied()));
            }
        }
        da.extend(db);
        da.sort();
        da.dedup();
        self.parents.insert(b, a);
        let descriptions = &mut self.descriptors;
        descriptions.remove(&b);
        descriptions.insert(a, da);
        let children = if self.share_deductions {
            self.equations.iter().skip(pending).copied().collect()
        } else {
            vec![]
        };
        self.record_deduction(key, children);
        true
    }
    fn record_deduction(&mut self, key: (usize, Value, Value), children: Vec<(Value, Value)>) {
        if !self.share_deductions {
            return;
        }
        self.equality_state = self.fresh_equality_state();
        let mut arena = self.arena.borrow_mut();
        if arena.deductions.len() < 4096 {
            arena.deductions.insert(
                key,
                Rc::new(Deduction {
                    state: self.equality_state,
                    parents: self.parents.clone(),
                    descriptors: self.descriptors.clone(),
                    children,
                    failed: self.failed,
                }),
            );
        }
    }
    pub fn failed(&self) -> bool {
        self.failed
    }
    pub fn pending(&self) -> usize {
        self.equations.len()
    }
    fn term(&self, id: Value) -> Term {
        let id = self.root(id);
        match self.descriptions(id).first() {
            Some((n, xs)) => Term::App(n.clone(), xs.iter().map(|v| self.term(*v)).collect()),
            None => Term::Var(Var(id.0 as u64)),
        }
    }
    pub fn export(&self, ids: &[Value]) -> Option<Vec<Term>> {
        if self.failed || !self.equations.is_empty() {
            None
        } else {
            Some(ids.iter().map(|v| self.term(*v)).collect())
        }
    }
    fn equal(&self, a: Value, b: Value) -> bool {
        if self.root(a) == self.root(b) {
            return true;
        }
        let (da, db) = (self.descriptions(a), self.descriptions(b));
        da.iter().any(|(n, xs)| {
            db.iter().any(|(m, ys)| {
                n == m && xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| self.equal(*x, *y))
            })
        })
    }
    fn pattern(
        &self,
        p: &Term,
        id: Value,
        env: &BTreeMap<Var, Value>,
    ) -> Vec<BTreeMap<Var, Value>> {
        match p {
            Term::Var(v) => match env.get(v) {
                Some(old) if !self.equal(*old, id) => vec![],
                _ => {
                    let mut next = env.clone();
                    next.entry(*v).or_insert(id);
                    vec![next]
                }
            },
            Term::App(n, xs) => {
                let mut results = BTreeSet::new();
                for (m, ys) in self.descriptions(id) {
                    if *n != m || xs.len() != ys.len() {
                        continue;
                    }
                    let mut paths = vec![env.clone()];
                    for (x, y) in xs.iter().zip(ys) {
                        paths = paths
                            .into_iter()
                            .flat_map(|e| self.pattern(x, y, &e))
                            .collect();
                    }
                    results.extend(paths);
                }
                results.into_iter().collect()
            }
        }
    }
    /// Select the first accepted match in occurrence-tuple/environment order.
    /// Prefix environments retain alternative partial constructor descriptions;
    /// only the complete accepted tuple is returned to the caller.
    pub fn find_match(
        &self,
        kept: &[Constraint],
        removed: &[Constraint],
        mut accept: impl FnMut(&Match) -> bool,
    ) -> Option<Match> {
        fn walk(
            store: &Store,
            heads: &[&Constraint],
            kept: usize,
            ids: &mut Vec<Occurrence>,
            environments: BTreeSet<BTreeMap<Var, Value>>,
            accept: &mut impl FnMut(&Match) -> bool,
        ) -> Option<Match> {
            if ids.len() == heads.len() {
                for bindings in environments {
                    let candidate = Match {
                        kept: ids[..kept].to_vec(),
                        removed: ids[kept..].to_vec(),
                        bindings,
                    };
                    if accept(&candidate) {
                        return Some(candidate);
                    }
                }
                return None;
            }
            let head = heads[ids.len()];
            for (id, resource) in store.live.iter() {
                if ids.contains(id)
                    || head.name != resource.name
                    || head.args.len() != resource.args.len()
                {
                    continue;
                }
                let mut next = environments.clone();
                for (pattern, value) in head.args.iter().zip(&resource.args) {
                    next = next
                        .into_iter()
                        .flat_map(|env| store.pattern(pattern, *value, &env))
                        .collect();
                    if next.is_empty() {
                        break;
                    }
                }
                if next.is_empty() {
                    continue;
                }
                ids.push(*id);
                let found = walk(store, heads, kept, ids, next, accept);
                ids.pop();
                if found.is_some() {
                    return found;
                }
            }
            None
        }
        if self.failed {
            return None;
        }
        walk(
            self,
            &kept.iter().chain(removed).collect::<Vec<_>>(),
            kept.len(),
            &mut Vec::with_capacity(kept.len() + removed.len()),
            BTreeSet::from([BTreeMap::new()]),
            &mut accept,
        )
    }
    pub fn matches(&self, kept: &[Constraint], removed: &[Constraint]) -> Vec<Match> {
        if self.failed {
            return vec![];
        }
        let mut paths = vec![(vec![], BTreeMap::new())];
        for head in kept.iter().chain(removed) {
            let mut next = vec![];
            for (ids, env) in paths {
                for (id, r) in self.live.iter() {
                    if ids.contains(id) || head.name != r.name || head.args.len() != r.args.len() {
                        continue;
                    }
                    let mut bindings = vec![env.clone()];
                    for (p, v) in head.args.iter().zip(&r.args) {
                        bindings = bindings
                            .into_iter()
                            .flat_map(|e| self.pattern(p, *v, &e))
                            .collect();
                    }
                    for env in bindings {
                        let mut chosen = ids.clone();
                        chosen.push(*id);
                        next.push((chosen, env));
                    }
                }
            }
            paths = next;
        }
        paths
            .into_iter()
            .map(|(ids, bindings)| Match {
                kept: ids[..kept.len()].to_vec(),
                removed: ids[kept.len()..].to_vec(),
                bindings,
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
    pub(crate) fn residual(&self) -> Option<Vec<Constraint>> {
        if self.failed || !self.equations.is_empty() {
            return None;
        }
        Some(
            self.live
                .values()
                .map(|r| Constraint {
                    name: r.name.clone(),
                    args: r.args.iter().map(|v| self.term(*v)).collect(),
                })
                .collect(),
        )
    }
    pub fn consume(&mut self, m: &Match) -> bool {
        let ids = m.kept.iter().chain(&m.removed).collect::<Vec<_>>();
        if self.failed
            || ids.iter().collect::<BTreeSet<_>>().len() != ids.len()
            || ids.iter().any(|id| !self.live.contains_key(id))
        {
            return false;
        }
        let live = Rc::make_mut(&mut self.live);
        for id in &m.removed {
            live.remove(id);
        }
        true
    }
}

/// Owned depth-first continuation. Only the executor may use it: matching-head
/// insertion and any equality work must invalidate it. Consumption is monotone.
#[derive(Clone)]
pub(crate) struct MatchCursor {
    heads: Vec<Constraint>,
    kept: usize,
    ids: Vec<Occurrence>,
    frames: Vec<MatchFrame>,
    done: bool,
}
#[derive(Clone)]
struct MatchFrame {
    after: Option<Occurrence>,
    environments: BTreeSet<BTreeMap<Var, Value>>,
}
impl MatchCursor {
    pub(crate) fn new(kept: &[Constraint], removed: &[Constraint]) -> Self {
        Self {
            heads: kept.iter().chain(removed).cloned().collect(),
            kept: kept.len(),
            ids: vec![],
            frames: vec![MatchFrame {
                after: None,
                environments: BTreeSet::from([BTreeMap::new()]),
            }],
            done: false,
        }
    }
    fn backtrack(&mut self) {
        self.frames.pop();
        if self.ids.pop().is_none() {
            self.done = true;
        }
    }
    pub(crate) fn next(&mut self, store: &Store) -> Option<Match> {
        use std::ops::Bound::{Excluded, Unbounded};
        if self.done || store.failed {
            return None;
        }
        // Removal cannot enable an earlier tuple. Skip the whole subtree whose
        // selected prefix includes a consumed occurrence, retaining its successor.
        if let Some(i) = self.ids.iter().position(|id| !store.live.contains_key(id)) {
            self.ids.truncate(i);
            self.frames.truncate(i + 1);
        }
        while !self.done {
            if self.ids.len() == self.heads.len() {
                if let Some(bindings) = self.frames.last_mut().unwrap().environments.pop_first() {
                    return Some(Match {
                        kept: self.ids[..self.kept].to_vec(),
                        removed: self.ids[self.kept..].to_vec(),
                        bindings,
                    });
                }
                self.backtrack();
                continue;
            }
            let frame = self.frames.last_mut().unwrap();
            let next = match frame.after {
                Some(id) => store.live.range((Excluded(id), Unbounded)).next(),
                None => store.live.iter().next(),
            };
            let Some((&id, resource)) = next else {
                self.backtrack();
                continue;
            };
            frame.after = Some(id);
            let head = &self.heads[self.ids.len()];
            if self.ids.contains(&id)
                || head.name != resource.name
                || head.args.len() != resource.args.len()
            {
                continue;
            }
            let mut environments = frame.environments.clone();
            for (p, v) in head.args.iter().zip(&resource.args) {
                environments = environments
                    .into_iter()
                    .flat_map(|env| store.pattern(p, *v, &env))
                    .collect();
                if environments.is_empty() {
                    break;
                }
            }
            if !environments.is_empty() {
                self.ids.push(id);
                self.frames.push(MatchFrame {
                    after: None,
                    environments,
                });
            }
        }
        None
    }
}

#[cfg(test)]
mod ownership_tests {
    use super::*;
    #[test]
    fn cached_maps_do_not_keep_their_arena_alive() {
        let mut base = Store::default().with_shared_deductions();
        let weak = Rc::downgrade(&base.arena);
        let x = base.unknown();
        let a = base.constructor("a", &[]);
        let mut branch = base.clone();
        branch.equate(x, a);
        while branch.step() {}
        assert_eq!(base.retained_deductions(), 1);
        drop(base);
        assert!(weak.upgrade().is_some());
        drop(branch);
        assert!(weak.upgrade().is_none());
    }
}

#[cfg(test)]
mod cursor_order_tests {
    use super::*;
    use chr_syntax::{c, t, v};
    #[test]
    fn cursor_retains_all_partial_constructor_environments_in_order() {
        let mut s = Store::default();
        let x = s.unknown();
        let y = s.unknown();
        let fx = s.constructor("f", &[x]);
        let fy = s.constructor("f", &[y]);
        s.post("open", &[fx]);
        s.post("open", &[fy]);
        s.post("other", &[x]);
        s.post("other", &[y]);
        s.equate(fx, fy);
        assert!(s.step());
        assert!(s.pending() > 0);
        let heads = [c("open", [t("f", [v(0)])]), c("other", [v(1)])];
        for settled in [false, true] {
            if settled {
                while s.step() {}
            }
            let expected = s.matches(&heads[..1], &heads[1..]);
            assert!(!expected.is_empty());
            let mut cursor = MatchCursor::new(&heads[..1], &heads[1..]);
            let mut actual = vec![];
            while let Some(m) = cursor.next(&s) {
                actual.push(m);
            }
            assert_eq!(actual, expected);
            assert!(cursor.next(&s).is_none());
        }
    }
}
