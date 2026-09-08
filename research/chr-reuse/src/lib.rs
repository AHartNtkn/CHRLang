//! Experimental exact unification-operation reuse over resolved finite terms.
//!
//! The caller supplies an equation that is being executed, after resolving its
//! environment, and applies the returned bindings with ordinary wake-ups. This
//! API does not establish a CHR continuation key or a branch-pruning certificate.
pub mod stable;
use chr_syntax::{Term, Var};
use std::collections::BTreeMap;

pub type Substitution = BTreeMap<Var, Term>;
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Direct,
    Memo,
}
#[derive(Default, Debug)]
pub struct Stats {
    pub calls: u64,
    pub computed: u64,
    pub hits: u64,
    pub key_nodes: u64,
    pub replay_nodes: u64,
    pub pairs: u64,
    pub resolve_nodes: u64,
    pub occurs_nodes: u64,
    pub cache_entries: usize,
}

pub struct EquationTable {
    mode: Mode,
    cache: BTreeMap<(Term, Term), Option<Vec<Term>>>,
    stats: Stats,
}
impl EquationTable {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            cache: BTreeMap::new(),
            stats: Stats::default(),
        }
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn solve(&mut self, left: &Term, right: &Term) -> Option<Substitution> {
        self.stats.calls += 1;
        if matches!(self.mode, Mode::Direct) {
            self.stats.computed += 1;
            return solve(left, right, &mut self.stats);
        }
        let mut vars = BTreeMap::new();
        let mut originals = vec![];
        let a = canonical(left, &mut vars, &mut originals, &mut self.stats);
        let b = canonical(right, &mut vars, &mut originals, &mut self.stats);
        let key = (a, b);
        let answer = if let Some(answer) = self.cache.get(&key) {
            self.stats.hits += 1;
            answer.clone()
        } else {
            self.stats.computed += 1;
            let answer = solve(&key.0, &key.1, &mut self.stats).map(|s| {
                (0..originals.len())
                    .map(|i| s[&Var(i as u64)].clone())
                    .collect::<Vec<_>>()
            });
            self.cache.insert(key, answer.clone());
            self.stats.cache_entries = self.cache.len();
            answer
        };
        answer.map(|values| {
            originals
                .iter()
                .zip(values.iter())
                .map(|(&v, t)| (v, replay(t, &originals, &mut self.stats)))
                .collect()
        })
    }
}
fn canonical(
    t: &Term,
    vars: &mut BTreeMap<Var, Var>,
    originals: &mut Vec<Var>,
    stats: &mut Stats,
) -> Term {
    stats.key_nodes += 1;
    match t {
        Term::Var(v) => Term::Var(*vars.entry(*v).or_insert_with(|| {
            let id = Var(originals.len() as u64);
            originals.push(*v);
            id
        })),
        Term::App(n, args) => Term::App(
            n.clone(),
            args.iter()
                .map(|t| canonical(t, vars, originals, stats))
                .collect(),
        ),
    }
}
fn replay(t: &Term, vars: &[Var], stats: &mut Stats) -> Term {
    stats.replay_nodes += 1;
    match t {
        Term::Var(v) => Term::Var(vars[v.0 as usize]),
        Term::App(n, args) => Term::App(
            n.clone(),
            args.iter().map(|t| replay(t, vars, stats)).collect(),
        ),
    }
}
fn resolve(t: &Term, sub: &Substitution, stats: &mut Stats) -> Term {
    stats.resolve_nodes += 1;
    match t {
        Term::Var(v) => sub
            .get(v)
            .map_or_else(|| t.clone(), |value| resolve(value, sub, stats)),
        Term::App(n, args) => Term::App(
            n.clone(),
            args.iter().map(|t| resolve(t, sub, stats)).collect(),
        ),
    }
}
fn occurs(v: Var, t: &Term, stats: &mut Stats) -> bool {
    stats.occurs_nodes += 1;
    match t {
        Term::Var(w) => v == *w,
        Term::App(_, args) => args.iter().any(|t| occurs(v, t, stats)),
    }
}
fn variables(t: &Term, vars: &mut BTreeMap<Var, ()>) {
    match t {
        Term::Var(v) => {
            vars.insert(*v, ());
        }
        Term::App(_, args) => {
            for t in args {
                variables(t, vars);
            }
        }
    }
}
fn solve(left: &Term, right: &Term, stats: &mut Stats) -> Option<Substitution> {
    let mut sub = Substitution::new();
    let mut pairs = vec![(left.clone(), right.clone())];
    while let Some((a, b)) = pairs.pop() {
        stats.pairs += 1;
        let a = resolve(&a, &sub, stats);
        let b = resolve(&b, &sub, stats);
        if a == b {
            continue;
        }
        match (a, b) {
            (Term::Var(v), t) | (t, Term::Var(v)) => {
                if occurs(v, &t, stats) {
                    return None;
                }
                sub.insert(v, t);
            }
            (Term::App(a, x), Term::App(b, y)) => {
                if a != b || x.len() != y.len() {
                    return None;
                }
                pairs.extend(x.into_iter().zip(y).rev());
            }
        }
    }
    let mut vars = BTreeMap::new();
    variables(left, &mut vars);
    variables(right, &mut vars);
    Some(
        vars.into_keys()
            .map(|v| (v, resolve(&Term::Var(v), &sub, stats)))
            .collect(),
    )
}
pub mod continuations;
pub mod equation_search;
pub mod failure;
pub mod failure_search;
pub mod parallel_equations;
