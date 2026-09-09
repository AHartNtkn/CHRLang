use crate::{Stats, map::Map};
use chr_syntax::{Term as Source, Var};
use std::collections::{BTreeMap, HashMap, HashSet};
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Term {
    Var(u64),
    Node(usize),
}
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Node {
    pub name: String,
    pub args: Vec<Term>,
}
#[derive(Default, Clone)]
pub struct Arena {
    #[cfg(feature = "fork-diagnostics")]
    fork_prefix: Option<usize>,
    #[cfg(feature = "fork-diagnostics")]
    fork_interning: ForkInterning,
    #[cfg(feature = "fork-diagnostics")]
    first_miss_nodes: Option<usize>,
    #[cfg(feature = "fork-diagnostics")]
    fork_predicate_insertions: u64,
    #[cfg(feature = "fork-diagnostics")]
    segment_requests: u64,
    #[cfg(feature = "fork-diagnostics")]
    requests_before_first_miss: Option<u64>,
    #[cfg(feature = "arena-cow")]
    data: std::rc::Rc<ArenaData>,
    #[cfg(not(feature = "arena-cow"))]
    data: ArenaData,
}
#[derive(Default, Clone)]
struct ArenaData {
    nodes: Vec<Node>,
    closed: Vec<bool>,
    intern: HashMap<Node, usize>,
    predicates: Vec<(String, usize)>,
    predicates_by_name: HashMap<(String, usize), usize>,
}
/// Diagnostic clone boundaries; callers must not mutate measured owners.
#[cfg(feature = "fork-diagnostics")]
pub trait ForkObserver {
    fn before(&mut self, owner: &'static str);
    fn after(&mut self, owner: &'static str);
    fn segment(&mut self, _endpoint: &'static str, _segment: ForkSegment) {}
}
#[cfg(feature = "fork-diagnostics")]
#[derive(Clone, Copy, Debug)]
pub struct ForkSegment {
    pub inherited_nodes: usize,
    pub first_miss_nodes: Option<usize>,
    pub predicate_insertions: u64,
    pub requests_before_first_miss: Option<u64>,
}
#[cfg(feature = "fork-diagnostics")]
pub struct NoopForkObserver;
#[cfg(feature = "fork-diagnostics")]
impl ForkObserver for NoopForkObserver {
    fn before(&mut self, _: &'static str) {}
    fn after(&mut self, _: &'static str) {}
}
#[cfg(feature = "fork-diagnostics")]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct ForkInterning {
    pub inherited_hits: u64,
    pub local_hits: u64,
    pub misses: u64,
}
#[cfg(feature = "fork-diagnostics")]
impl ForkInterning {
    pub fn add(&mut self, other: Self) {
        self.inherited_hits += other.inherited_hits;
        self.local_hits += other.local_hits;
        self.misses += other.misses;
    }
}
#[cfg(feature = "fork-diagnostics")]
pub fn observed_clone<T: Clone>(
    value: &T,
    owner: &'static str,
    observer: &mut impl ForkObserver,
) -> T {
    observer.before(owner);
    let result = value.clone();
    observer.after(owner);
    result
}
pub type Scope = BTreeMap<u64, Term>;
pub type Bindings = Map<u64, Term>;
impl Arena {
    fn data(&self) -> &ArenaData {
        &self.data
    }
    fn data_mut(&mut self) -> &mut ArenaData {
        #[cfg(feature = "arena-cow")]
        {
            std::rc::Rc::make_mut(&mut self.data)
        }
        #[cfg(not(feature = "arena-cow"))]
        {
            &mut self.data
        }
    }
    pub fn predicates(&self) -> &[(String, usize)] {
        &self.data.predicates
    }
    #[allow(
        dead_code,
        reason = "Shared terms consumers may retain the complete arena"
    )]
    pub fn into_predicates(self) -> Vec<(String, usize)> {
        #[cfg(feature = "arena-cow")]
        {
            match std::rc::Rc::try_unwrap(self.data) {
                Ok(data) => data.predicates,
                Err(data) => data.predicates.clone(),
            }
        }
        #[cfg(not(feature = "arena-cow"))]
        {
            self.data.predicates
        }
    }

    #[cfg(feature = "fork-diagnostics")]
    pub fn clone_observed(&self, observer: &mut impl ForkObserver) -> Self {
        Self {
            #[cfg(feature = "arena-cow")]
            data: observed_clone(&self.data, "arena_shared", observer),
            #[cfg(not(feature = "arena-cow"))]
            data: ArenaData {
                nodes: observed_clone(&self.data.nodes, "arena_nodes", observer),
                closed: observed_clone(&self.data.closed, "arena_closed", observer),
                intern: observed_clone(&self.data.intern, "arena_intern", observer),
                predicates: observed_clone(&self.data.predicates, "arena_predicates", observer),
                predicates_by_name: observed_clone(
                    &self.data.predicates_by_name,
                    "arena_predicate_lookup",
                    observer,
                ),
            },
            fork_prefix: self.fork_prefix,
            fork_interning: ForkInterning::default(),
            first_miss_nodes: None,
            fork_predicate_insertions: 0,
            segment_requests: 0,
            requests_before_first_miss: None,
        }
    }
    #[cfg(feature = "fork-diagnostics")]
    pub fn mark_fork_prefix(&mut self) {
        self.fork_prefix = Some(self.node_count());
        self.first_miss_nodes = None;
        self.fork_predicate_insertions = 0;
        self.segment_requests = 0;
        self.requests_before_first_miss = None;
    }
    #[cfg(feature = "fork-diagnostics")]
    pub fn fork_segment(&self) -> Option<ForkSegment> {
        self.fork_prefix.map(|inherited_nodes| ForkSegment {
            inherited_nodes,
            first_miss_nodes: self.first_miss_nodes,
            predicate_insertions: self.fork_predicate_insertions,
            requests_before_first_miss: self.requests_before_first_miss,
        })
    }
    #[cfg(feature = "fork-diagnostics")]
    pub fn take_fork_interning(&mut self) -> ForkInterning {
        std::mem::take(&mut self.fork_interning)
    }

    /// Interned nodes cannot be mutated after insertion. IDs belong to this arena.
    pub fn node(&self, id: usize) -> &Node {
        &self.data().nodes[id]
    }
    pub fn node_count(&self) -> usize {
        self.data.nodes.len()
    }
    /// True exactly when the constructor subtree contains no syntactic variables.
    /// Bindings never participate in this property, including after a fork.
    pub fn is_closed(&self, id: usize) -> bool {
        self.data.closed[id]
    }

    pub fn predicate(&mut self, name: &str, arity: usize) -> usize {
        let key = (name.to_owned(), arity);
        if let Some(id) = self.data.predicates_by_name.get(&key) {
            return *id;
        }
        #[cfg(feature = "fork-diagnostics")]
        if self.fork_prefix.is_some() {
            self.fork_predicate_insertions += 1;
        }
        let id = self.data.predicates.len();
        let data = self.data_mut();
        data.predicates.push(key.clone());
        data.predicates_by_name.insert(key, id);
        id
    }
    pub fn make(&mut self, name: &str, args: Vec<Term>, stats: &mut Stats) -> Term {
        if crate::COLLECT_KERNEL_METRICS {
            stats.term_requests += 1;
        }
        #[cfg(feature = "fork-diagnostics")]
        if self.fork_prefix.is_some() {
            self.segment_requests += 1;
        }
        let key = Node {
            name: name.to_owned(),
            args,
        };
        if let Some(id) = self.data.intern.get(&key) {
            #[cfg(feature = "fork-diagnostics")]
            if let Some(prefix) = self.fork_prefix {
                if *id < prefix {
                    self.fork_interning.inherited_hits += 1;
                } else {
                    self.fork_interning.local_hits += 1;
                }
            }
            return Term::Node(*id);
        }
        #[cfg(feature = "fork-diagnostics")]
        if self.fork_prefix.is_some() {
            self.fork_interning.misses += 1;
            if self.first_miss_nodes.is_none() {
                self.first_miss_nodes = Some(self.node_count());
                self.requests_before_first_miss = Some(self.segment_requests - 1);
            }
        }
        let closed = key.args.iter().all(|term| match term {
            Term::Var(_) => false,
            Term::Node(id) => self.is_closed(*id),
        });
        let id = self.node_count();
        let data = self.data_mut();
        data.nodes.push(key.clone());
        data.closed.push(closed);
        data.intern.insert(key, id);
        if crate::COLLECT_KERNEL_METRICS {
            stats.term_nodes = self.node_count();
        }
        Term::Node(id)
    }
    pub fn instantiate(
        &mut self,
        source: &Source,
        scope: &mut Scope,
        next: &mut u64,
        stats: &mut Stats,
    ) -> Term {
        match source {
            Source::Var(Var(id)) => *scope.entry(*id).or_insert_with(|| {
                let t = Term::Var(*next);
                *next += 1;
                t
            }),
            Source::App(name, args) => {
                let terms = args
                    .iter()
                    .map(|a| self.instantiate(a, scope, next, stats))
                    .collect();
                self.make(name, terms, stats)
            }
        }
    }
    pub fn export(&self, term: Term, bindings: &Bindings, stats: &mut Stats) -> Source {
        match deref(term, bindings, stats) {
            Term::Var(id) => Source::Var(Var(id)),
            Term::Node(id) => {
                let n = &self.data().nodes[id];
                Source::App(
                    n.name.clone(),
                    n.args
                        .iter()
                        .map(|&a| self.export(a, bindings, stats))
                        .collect(),
                )
            }
        }
    }
    pub fn equal(&self, left: Term, right: Term, bindings: &Bindings, stats: &mut Stats) -> bool {
        let mut todo = vec![(left, right)];
        while let Some((left, right)) = todo.pop() {
            if crate::COLLECT_KERNEL_METRICS {
                stats.pairs += 1;
            }
            let left = deref(left, bindings, stats);
            let right = deref(right, bindings, stats);
            if left == right {
                continue;
            }
            match (left, right) {
                (Term::Node(a), Term::Node(b)) => {
                    let a = &self.data.nodes[a];
                    let b = &self.data.nodes[b];
                    if a.name != b.name || a.args.len() != b.args.len() {
                        return false;
                    }
                    todo.extend(a.args.iter().zip(&b.args).map(|(&a, &b)| (a, b)));
                }
                _ => return false,
            }
        }
        true
    }
    pub fn unify(
        &self,
        left: Term,
        right: Term,
        bindings: &mut Bindings,
        stats: &mut Stats,
    ) -> bool {
        self.unify_impl::<false>(left, right, bindings, stats, &mut Vec::new())
    }
    pub(crate) fn unify_impl<const REPORT: bool>(
        &self,
        left: Term,
        right: Term,
        bindings: &mut Bindings,
        stats: &mut Stats,
        changed: &mut Vec<u64>,
    ) -> bool {
        let mut trial = bindings.clone();
        let mut todo = vec![(left, right)];
        while let Some((left, right)) = todo.pop() {
            if crate::COLLECT_KERNEL_METRICS {
                stats.pairs += 1;
            }
            let left = deref(left, &trial, stats);
            let right = deref(right, &trial, stats);
            if left == right {
                continue;
            }
            match (left, right) {
                (Term::Var(a), Term::Var(b)) => {
                    let (child, parent) = if a > b { (a, b) } else { (b, a) };
                    trial.insert(child, Term::Var(parent), &mut stats.storage);
                    if REPORT {
                        changed.push(child);
                    }
                }
                (Term::Var(var), term) | (term, Term::Var(var)) => {
                    let mut stack = vec![term];
                    let mut seen = HashSet::new();
                    while let Some(t) = stack.pop() {
                        if crate::COLLECT_KERNEL_METRICS {
                            stats.occurs_visits += 1;
                        }
                        match deref(t, &trial, stats) {
                            Term::Var(id) if id == var => return false,
                            // Structural closedness is immutable and independent of
                            // trial bindings: this subtree cannot contain `var`.
                            Term::Node(id) if !self.is_closed(id) && seen.insert(id) => {
                                stack.extend(&self.data().nodes[id].args)
                            }
                            _ => {}
                        }
                    }
                    trial.insert(var, term, &mut stats.storage);
                    if REPORT {
                        changed.push(var);
                    }
                }
                (Term::Node(a), Term::Node(b)) => {
                    let a = &self.data.nodes[a];
                    let b = &self.data.nodes[b];
                    if a.name != b.name || a.args.len() != b.args.len() {
                        return false;
                    }
                    todo.extend(a.args.iter().zip(&b.args).map(|(&a, &b)| (a, b)));
                }
            }
        }
        *bindings = trial;
        true
    }
}
pub fn deref(mut term: Term, bindings: &Bindings, stats: &mut Stats) -> Term {
    while let Term::Var(id) = term {
        if crate::COLLECT_KERNEL_METRICS {
            stats.dereferences += 1;
        }
        match bindings.get(&id, &mut stats.storage) {
            Some(t) => term = t,
            None => break,
        }
    }
    term
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "arena-cow")]
    #[test]
    fn cow_hits_share_storage_and_insertions_detach_all_dictionaries() {
        let mut a = Arena::default();
        let mut stats = Stats::default();
        let term = a.make("a", vec![], &mut stats);
        let pred = a.predicate("p", 1);
        let mut b = a.clone();
        assert!(
            std::ptr::eq(a.data(), b.data()),
            "fork must share the data owner"
        );
        assert_eq!(b.make("a", vec![], &mut stats), term);
        assert_eq!(b.predicate("p", 1), pred);
        assert!(std::ptr::eq(a.data(), b.data()), "hits must not detach");
        let new = b.make("f", vec![term], &mut stats);
        assert!(!std::ptr::eq(a.data(), b.data()));
        assert_eq!(a.node_count(), 1);
        assert_eq!(b.node_count(), 2);
        assert_eq!(b.make("f", vec![term], &mut stats), new);
        let mut c = a.clone();
        c.predicate("q", 0);
        assert!(!std::ptr::eq(a.data(), c.data()));
        assert_eq!(a.predicates().len(), 1);
        assert_eq!(c.predicates().len(), 2);
        assert_eq!(a.make("other", vec![], &mut stats), Term::Node(1));
        assert_ne!(a.node(1).name, b.node(1).name);
        drop(b);
        drop(c);
        let owner = a.data() as *const ArenaData;
        a.make("unique", vec![], &mut stats);
        assert_eq!(
            owner,
            a.data() as *const ArenaData,
            "unique owner must not detach"
        );
    }
    #[test]
    fn closedness_is_canonical_structural_and_independent_of_bindings() {
        let mut arena = Arena::default();
        let mut stats = Stats::default();
        let a = arena.make("a", vec![], &mut stats);
        let b = arena.make("b", vec![], &mut stats);
        let closed = arena.make("f", vec![a], &mut stats);
        assert_eq!(closed, arena.make("f", vec![a], &mut stats));
        assert_ne!(closed, arena.make("f", vec![b], &mut stats));
        let open = arena.make("g", vec![closed, Term::Var(1)], &mut stats);
        let Term::Node(closed_id) = closed else {
            unreachable!()
        };
        let Term::Node(open_id) = open else {
            unreachable!()
        };
        assert!(arena.is_closed(closed_id));
        assert!(!arena.is_closed(open_id));
        let mut left = Bindings::default();
        assert!(arena.unify(Term::Var(0), open, &mut left, &mut stats));
        let fork = arena.clone();
        let mut right = left.clone();
        assert!(arena.unify(Term::Var(1), a, &mut left, &mut stats));
        assert!(fork.unify(Term::Var(1), b, &mut right, &mut stats));
        assert!(!arena.is_closed(open_id));
        assert!(!fork.is_closed(open_id));
        assert_ne!(
            arena.export(open, &left, &mut stats),
            fork.export(open, &right, &mut stats)
        );
        assert!(!arena.unify(Term::Var(1), b, &mut left, &mut stats));
        assert!(arena.equal(Term::Var(1), a, &left, &mut stats));
        assert!(arena.is_closed(closed_id));
        assert!(!arena.is_closed(open_id));
        assert_eq!(arena.node(open_id).args, vec![closed, Term::Var(1)]);
    }
    #[test]
    fn failed_equation_does_not_publish_earlier_pairs() {
        let mut arena = Arena::default();
        let mut stats = Stats::default();
        let mut bindings = Bindings::default();
        let a = arena.make("a", vec![], &mut stats);
        let b = arena.make("b", vec![], &mut stats);
        let left = arena.make("pair", vec![a, Term::Var(0)], &mut stats);
        let right = arena.make("pair", vec![b, a], &mut stats);
        assert!(!arena.unify(left, right, &mut bindings, &mut stats));
        assert!(bindings.get(&0, &mut stats.storage).is_none());
        let f = arena.make("f", vec![Term::Var(1)], &mut stats);
        assert!(arena.unify(Term::Var(0), f, &mut bindings, &mut stats));
        assert!(!arena.unify(Term::Var(1), Term::Var(0), &mut bindings, &mut stats));
        assert!(bindings.get(&1, &mut stats.storage).is_none());
    }
}
