//! Finite source-derived elimination of a private pure rule prefix.
//! Contract: source-order selection, positive logical equations, ordinary remaining effects.
use chr_syntax::{Constraint, Goal, Guard, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet};
type Key = (String, usize);
fn key(c: &Constraint) -> Key {
    (c.name.clone(), c.args.len())
}
fn calls(g: &Goal, out: &mut BTreeSet<Key>) {
    match g {
        Goal::Constraint(c) => {
            out.insert(key(c));
        }
        Goal::And(gs) => {
            for g in gs {
                calls(g, out)
            }
        }
        Goal::Or(a, b) => {
            calls(a, out);
            calls(b, out)
        }
        _ => (),
    }
}
fn vars(t: &Term, out: &mut BTreeSet<Var>) {
    match t {
        Term::Var(v) => {
            out.insert(*v);
        }
        Term::App(_, xs) => {
            for x in xs {
                vars(x, out)
            }
        }
    }
}
fn goal_vars(g: &Goal, out: &mut BTreeSet<Var>) {
    match g {
        Goal::Constraint(c) => {
            for t in &c.args {
                vars(t, out)
            }
        }
        Goal::Unify(a, b) => {
            vars(a, out);
            vars(b, out)
        }
        Goal::And(gs) => {
            for g in gs {
                goal_vars(g, out)
            }
        }
        Goal::Or(a, b) => {
            goal_vars(a, out);
            goal_vars(b, out)
        }
        _ => (),
    }
}
fn acyclic(
    k: &Key,
    defs: &BTreeMap<Key, Rule>,
    active: &mut BTreeSet<Key>,
    done: &mut BTreeSet<Key>,
) -> bool {
    if done.contains(k) {
        return true;
    }
    if !active.insert(k.clone()) {
        return false;
    }
    let Some(r) = defs.get(k) else {
        return false;
    };
    let mut deps = BTreeSet::new();
    calls(&r.body, &mut deps);
    if !deps.iter().all(|d| acyclic(d, defs, active, done)) {
        return false;
    }
    active.remove(k);
    done.insert(k.clone());
    true
}
pub struct Program {
    source: Vec<Rule>,
    defs: BTreeMap<Key, Rule>,
    prefix: usize,
}
impl Program {
    pub fn new(source: &[Rule]) -> Result<Self, String> {
        let mut uses = BTreeMap::<Key, usize>::new();
        for r in source {
            for c in r.kept.iter().chain(&r.removed) {
                *uses.entry(key(c)).or_default() += 1;
            }
        }
        let mut candidates = BTreeMap::new();
        let mut best = None;
        for (i, r) in source.iter().enumerate() {
            if !r.kept.is_empty() || r.removed.len() != 1 || !r.guards.is_empty() {
                break;
            }
            let h = &r.removed[0];
            let mut seen = BTreeSet::new();
            if uses[&key(h)] != 1
                || !h
                    .args
                    .iter()
                    .all(|t| matches!(t,Term::Var(v) if seen.insert(*v)))
            {
                break;
            }
            candidates.insert(key(h), r.clone());
            let mut done = BTreeSet::new();
            if candidates
                .keys()
                .all(|k| acyclic(k, &candidates, &mut BTreeSet::new(), &mut done))
            {
                best = Some((i + 1, candidates.clone()));
            }
        }
        let (prefix, defs) = best.ok_or("no closed acyclic private pure prefix")?;
        Ok(Self {
            source: source.to_vec(),
            defs,
            prefix,
        })
    }
    pub fn eliminated_predicates(&self) -> Vec<Key> {
        self.defs.keys().cloned().collect()
    }
    pub fn lower(&self, query: &Query) -> Result<(Vec<Rule>, Query), String> {
        let mut all = BTreeSet::new();
        let mut names = BTreeSet::new();
        for r in &self.source {
            for c in r.kept.iter().chain(&r.removed) {
                names.insert(c.name.clone());
                for t in &c.args {
                    vars(t, &mut all)
                }
            }
            for Guard::Equal(a, b) in &r.guards {
                vars(a, &mut all);
                vars(b, &mut all)
            }
            goal_vars(&r.body, &mut all);
            let mut cs = BTreeSet::new();
            calls(&r.body, &mut cs);
            names.extend(cs.into_iter().map(|k| k.0));
        }
        let mut query_vars = BTreeSet::new();
        for c in &query.constraints {
            names.insert(c.name.clone());
            for t in &c.args {
                vars(t, &mut query_vars)
            }
        }
        query_vars.extend(query.outputs.iter().map(|(_, v)| *v));
        all.extend(&query_vars);
        let next = all.last().map_or(Ok(0), |v| {
            v.0.checked_add(1).ok_or("variable identity exhausted")
        })?;
        let mut expansion = Expansion {
            defs: &self.defs,
            next,
            fuel: 100_000,
        };
        let mut rules = vec![];
        for r in &self.source[self.prefix..] {
            let mut r = r.clone();
            r.body = expansion.expand(&r.body)?;
            rules.push(r);
        }
        let mut name = "__prefix_entry".to_string();
        while names.contains(&name) {
            name.push('_');
        }
        let args = query_vars.into_iter().map(Term::Var).collect::<Vec<_>>();
        let body = expansion.expand(&Goal::And(
            query
                .constraints
                .iter()
                .cloned()
                .map(Goal::Constraint)
                .collect(),
        ))?;
        let entry = Constraint {
            name: name.clone(),
            args,
        };
        rules.insert(0, Rule::simplify(&name, [entry.clone()], body));
        Ok((
            rules,
            Query {
                constraints: vec![entry],
                outputs: query.outputs.clone(),
            },
        ))
    }
}
struct Expansion<'a> {
    defs: &'a BTreeMap<Key, Rule>,
    next: u64,
    fuel: usize,
}
impl Expansion<'_> {
    fn term(&mut self, t: &Term, env: &mut BTreeMap<Var, Term>) -> Result<Term, String> {
        Ok(match t {
            Term::Var(v) => {
                if let Some(t) = env.get(v) {
                    return Ok(t.clone());
                }
                let fresh = Term::Var(Var(self.next));
                self.next = self
                    .next
                    .checked_add(1)
                    .ok_or("variable identity exhausted")?;
                env.insert(*v, fresh.clone());
                fresh
            }
            Term::App(n, xs) => Term::App(
                n.clone(),
                xs.iter()
                    .map(|t| self.term(t, env))
                    .collect::<Result<_, _>>()?,
            ),
        })
    }
    fn instantiate(&mut self, g: &Goal, env: &mut BTreeMap<Var, Term>) -> Result<Goal, String> {
        Ok(match g {
            Goal::Constraint(c) => Goal::Constraint(Constraint {
                name: c.name.clone(),
                args: c
                    .args
                    .iter()
                    .map(|t| self.term(t, env))
                    .collect::<Result<_, _>>()?,
            }),
            Goal::Unify(a, b) => Goal::Unify(self.term(a, env)?, self.term(b, env)?),
            Goal::And(gs) => Goal::And(
                gs.iter()
                    .map(|g| self.instantiate(g, env))
                    .collect::<Result<_, _>>()?,
            ),
            Goal::Or(a, b) => Goal::Or(
                Box::new(self.instantiate(a, env)?),
                Box::new(self.instantiate(b, env)?),
            ),
            Goal::True => Goal::True,
            Goal::Fail => Goal::Fail,
        })
    }
    fn expand(&mut self, g: &Goal) -> Result<Goal, String> {
        self.fuel = self
            .fuel
            .checked_sub(1)
            .ok_or("compile expansion bound exceeded")?;
        Ok(match g {
            Goal::Constraint(c) if self.defs.contains_key(&key(c)) => {
                let r = &self.defs[&key(c)];
                let mut env = BTreeMap::new();
                for (p, a) in r.removed[0].args.iter().zip(&c.args) {
                    let Term::Var(v) = p else { unreachable!() };
                    env.insert(*v, a.clone());
                }
                let body = self.instantiate(&r.body, &mut env)?;
                self.expand(&body)?
            }
            Goal::And(gs) => Goal::And(
                gs.iter()
                    .map(|g| self.expand(g))
                    .collect::<Result<_, _>>()?,
            ),
            Goal::Or(a, b) => Goal::Or(Box::new(self.expand(a)?), Box::new(self.expand(b)?)),
            _ => g.clone(),
        })
    }
}
