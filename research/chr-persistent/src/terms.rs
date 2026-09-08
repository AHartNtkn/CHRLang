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
#[derive(Default)]
pub struct Arena {
    pub nodes: Vec<Node>,
    intern: HashMap<Node, usize>,
    pub predicates: Vec<(String, usize)>,
    predicates_by_name: HashMap<(String, usize), usize>,
}
pub type Scope = BTreeMap<u64, Term>;
pub type Bindings = Map<u64, Term>;
impl Arena {
    pub fn predicate(&mut self, name: &str, arity: usize) -> usize {
        let key = (name.to_owned(), arity);
        if let Some(id) = self.predicates_by_name.get(&key) {
            return *id;
        }
        let id = self.predicates.len();
        self.predicates.push(key.clone());
        self.predicates_by_name.insert(key, id);
        id
    }
    pub fn make(&mut self, name: &str, args: Vec<Term>, stats: &mut Stats) -> Term {
        if crate::COLLECT_KERNEL_METRICS {
            stats.term_requests += 1;
        }
        let key = Node {
            name: name.to_owned(),
            args,
        };
        if let Some(id) = self.intern.get(&key) {
            return Term::Node(*id);
        }
        let id = self.nodes.len();
        self.nodes.push(key.clone());
        self.intern.insert(key, id);
        if crate::COLLECT_KERNEL_METRICS {
            stats.term_nodes = self.nodes.len();
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
                let n = &self.nodes[id];
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
                    let a = &self.nodes[a];
                    let b = &self.nodes[b];
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
                            Term::Node(id) if seen.insert(id) => stack.extend(&self.nodes[id].args),
                            _ => {}
                        }
                    }
                    trial.insert(var, term, &mut stats.storage);
                    if REPORT {
                        changed.push(var);
                    }
                }
                (Term::Node(a), Term::Node(b)) => {
                    let a = &self.nodes[a];
                    let b = &self.nodes[b];
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
