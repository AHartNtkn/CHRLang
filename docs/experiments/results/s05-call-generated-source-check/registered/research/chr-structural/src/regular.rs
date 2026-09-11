//! Regular tree languages interpreted as sets of finite ground trees.
use crate::{Datum, Node};
use chr_syntax::Term;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Transition {
    pub symbol: String,
    pub children: Vec<usize>,
}
impl Transition {
    pub fn new(symbol: &str, children: impl IntoIterator<Item = usize>) -> Self {
        Self {
            symbol: symbol.into(),
            children: children.into_iter().collect(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Automaton {
    states: Vec<Vec<Transition>>,
    root: usize,
}
#[derive(Default, Debug)]
pub struct Stats {
    pub intersection_pairs: u64,
    pub transition_pairs: u64,
    pub productivity_checks: u64,
    pub witness_nodes: u64,
    pub ground_checks: u64,
    pub membership_steps: u64,
    pub membership_hits: u64,
    pub enumeration_checks: u64,
    pub size_partitions: u64,
    pub enumeration_candidates: u64,
}
impl Automaton {
    pub fn new(states: Vec<Vec<Transition>>, root: usize) -> Result<Self, String> {
        if root >= states.len() {
            return Err("invalid root state".into());
        }
        if states
            .iter()
            .flatten()
            .flat_map(|t| &t.children)
            .any(|c| *c >= states.len())
        {
            return Err("invalid child state".into());
        }
        Ok(Self { states, root })
    }
    pub fn state_count(&self) -> usize {
        self.states.len()
    }
    pub fn transition_count(&self) -> usize {
        self.states.iter().map(Vec::len).sum()
    }
    pub fn intersect(&self, other: &Self, stats: &mut Stats) -> Self {
        let mut ids = BTreeMap::from([((self.root, other.root), 0)]);
        let mut pairs = vec![(self.root, other.root)];
        let mut states = vec![];
        let mut cursor = 0;
        while cursor < pairs.len() {
            let (a, b) = pairs[cursor];
            cursor += 1;
            stats.intersection_pairs += 1;
            let mut transitions = BTreeSet::new();
            for x in &self.states[a] {
                for y in &other.states[b] {
                    stats.transition_pairs += 1;
                    if x.symbol != y.symbol || x.children.len() != y.children.len() {
                        continue;
                    }
                    let children = x
                        .children
                        .iter()
                        .zip(&y.children)
                        .map(|(a, b)| {
                            let pair = (*a, *b);
                            if let Some(id) = ids.get(&pair) {
                                *id
                            } else {
                                let id = pairs.len();
                                pairs.push(pair);
                                ids.insert(pair, id);
                                id
                            }
                        })
                        .collect();
                    transitions.insert(Transition {
                        symbol: x.symbol.clone(),
                        children,
                    });
                }
            }
            states.push(transitions.into_iter().collect());
        }
        Self { states, root: 0 }
    }
    /// Least-fixed-point productivity: each witness uses already constructed finite children.
    pub fn witness(&self, stats: &mut Stats) -> Option<Datum> {
        let mut witnesses: Vec<Option<Datum>> = vec![None; self.states.len()];
        loop {
            let mut changed = false;
            for (i, transitions) in self.states.iter().enumerate() {
                if witnesses[i].is_some() {
                    continue;
                }
                for t in transitions {
                    stats.productivity_checks += 1;
                    if t.children.iter().all(|c| witnesses[*c].is_some()) {
                        let args = t
                            .children
                            .iter()
                            .map(|c| witnesses[*c].as_ref().unwrap().clone())
                            .collect::<Vec<_>>();
                        witnesses[i] = Some(Datum::app(&t.symbol, args));
                        stats.witness_nodes += 1;
                        changed = true;
                        break;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        witnesses[self.root].clone()
    }
    /// None denotes a nonground input, not failed membership or permission to bind it.
    pub fn contains(&self, input: &Datum, stats: &mut Stats) -> Option<bool> {
        contains(self, input, stats)
    }
    pub fn contains_term(&self, input: &Term, stats: &mut Stats) -> Option<bool> {
        contains(self, input, stats)
    }
    pub fn enumerate(&self, max_nodes: usize, stats: &mut Stats) -> Vec<Term> {
        fn sizes(total: usize, arity: usize, prefix: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
            if arity == 0 {
                if total == 0 {
                    out.push(prefix.clone());
                }
                return;
            }
            if total < arity {
                return;
            }
            for n in 1..=total - (arity - 1) {
                prefix.push(n);
                sizes(total - n, arity - 1, prefix, out);
                prefix.pop();
            }
        }
        fn product(
            symbol: &str,
            sets: &[&BTreeSet<Term>],
            i: usize,
            args: &mut Vec<Term>,
            out: &mut BTreeSet<Term>,
            stats: &mut Stats,
        ) {
            if i == sets.len() {
                stats.enumeration_candidates += 1;
                out.insert(Term::App(symbol.into(), args.clone()));
                return;
            }
            for t in sets[i] {
                args.push(t.clone());
                product(symbol, sets, i + 1, args, out, stats);
                args.pop();
            }
        }
        let mut values = vec![vec![BTreeSet::new(); self.states.len()]; max_nodes + 1];
        for n in 1..=max_nodes {
            for state in 0..self.states.len() {
                let mut terms = BTreeSet::new();
                for transition in &self.states[state] {
                    stats.enumeration_checks += 1;
                    let mut partitions = vec![];
                    sizes(
                        n - 1,
                        transition.children.len(),
                        &mut vec![],
                        &mut partitions,
                    );
                    stats.size_partitions += partitions.len() as u64;
                    for partition in partitions {
                        let sets = partition
                            .iter()
                            .zip(&transition.children)
                            .map(|(n, s)| &values[*n][*s])
                            .collect::<Vec<_>>();
                        product(&transition.symbol, &sets, 0, &mut vec![], &mut terms, stats);
                    }
                }
                values[n][state] = terms;
            }
        }
        values
            .into_iter()
            .flat_map(|mut row| std::mem::take(&mut row[self.root]))
            .collect()
    }
}

trait Tree: Sized {
    fn identity(&self) -> usize;
    fn application(&self) -> Option<(&str, &[Self])>;
}
impl Tree for Datum {
    fn identity(&self) -> usize {
        Rc::as_ptr(&self.0) as usize
    }
    fn application(&self) -> Option<(&str, &[Self])> {
        match self.0.as_ref() {
            Node::Var(_) => None,
            Node::App(n, args) => Some((n, args)),
        }
    }
}
impl Tree for Term {
    fn identity(&self) -> usize {
        self as *const Self as usize
    }
    fn application(&self) -> Option<(&str, &[Self])> {
        match self {
            Term::Var(_) => None,
            Term::App(n, args) => Some((n, args)),
        }
    }
}
fn contains<T: Tree>(automaton: &Automaton, input: &T, stats: &mut Stats) -> Option<bool> {
    let mut pending = vec![input];
    let mut seen = BTreeSet::new();
    while let Some(t) = pending.pop() {
        if !seen.insert(t.identity()) {
            continue;
        }
        stats.ground_checks += 1;
        let (_, args) = t.application()?;
        pending.extend(args);
    }
    fn check<T: Tree>(
        a: &Automaton,
        t: &T,
        state: usize,
        memo: &mut BTreeMap<(usize, usize), bool>,
        stats: &mut Stats,
    ) -> bool {
        stats.membership_steps += 1;
        let key = (t.identity(), state);
        if let Some(result) = memo.get(&key) {
            stats.membership_hits += 1;
            return *result;
        }
        let (symbol, args) = t.application().unwrap();
        let result = a.states[state].iter().any(|transition| {
            transition.symbol == symbol
                && transition.children.len() == args.len()
                && transition
                    .children
                    .iter()
                    .zip(args)
                    .all(|(s, t)| check(a, t, *s, memo, stats))
        });
        memo.insert(key, result);
        result
    }
    Some(check(
        automaton,
        input,
        automaton.root,
        &mut BTreeMap::new(),
        stats,
    ))
}
