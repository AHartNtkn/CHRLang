//! Private effectful boundary fusion, conditional on counted query resources.
use chr_syntax::{Constraint, Goal, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet};
type Key = (String, usize);
fn key(c: &Constraint) -> Key {
    (c.name.clone(), c.args.len())
}
fn goal_calls(g: &Goal, out: &mut Vec<Key>) {
    match g {
        Goal::Constraint(c) => out.push(key(c)),
        Goal::And(gs) => {
            for g in gs {
                goal_calls(g, out)
            }
        }
        Goal::Or(a, b) => {
            goal_calls(a, out);
            goal_calls(b, out)
        }
        _ => (),
    }
}
fn term_vars(t: &Term, out: &mut BTreeSet<Var>) {
    match t {
        Term::Var(v) => {
            out.insert(*v);
        }
        Term::App(_, xs) => {
            for t in xs {
                term_vars(t, out)
            }
        }
    }
}
fn goal_vars(g: &Goal, out: &mut BTreeSet<Var>) {
    match g {
        Goal::Constraint(c) => {
            for t in &c.args {
                term_vars(t, out)
            }
        }
        Goal::Unify(a, b) => {
            term_vars(a, out);
            term_vars(b, out)
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
fn subst_term(t: &Term, env: &BTreeMap<Var, Term>) -> Term {
    match t {
        Term::Var(v) => env[v].clone(),
        Term::App(n, xs) => Term::App(n.clone(), xs.iter().map(|x| subst_term(x, env)).collect()),
    }
}
fn subst_constraint(c: &Constraint, env: &BTreeMap<Var, Term>) -> Constraint {
    Constraint {
        name: c.name.clone(),
        args: c.args.iter().map(|x| subst_term(x, env)).collect(),
    }
}
fn subst_goal(g: &Goal, env: &BTreeMap<Var, Term>) -> Goal {
    match g {
        Goal::Constraint(c) => Goal::Constraint(subst_constraint(c, env)),
        Goal::Unify(a, b) => Goal::Unify(subst_term(a, env), subst_term(b, env)),
        Goal::And(gs) => Goal::And(gs.iter().map(|g| subst_goal(g, env)).collect()),
        Goal::Or(a, b) => Goal::Or(Box::new(subst_goal(a, env)), Box::new(subst_goal(b, env))),
        _ => g.clone(),
    }
}
fn ground(t: &Term) -> bool {
    match t {
        Term::Var(_) => false,
        Term::App(_, xs) => xs.iter().all(ground),
    }
}
pub struct Program {
    emitted: Vec<Rule>,
    producer: BTreeMap<Key, usize>,
    extra: BTreeMap<Key, usize>,
    middle: Key,
    pub fused_rules: (usize, usize),
}
impl Program {
    pub fn infer(rules: &[Rule]) -> Result<Self, &'static str> {
        for producer in 0..rules.len() {
            for consumer in 0..producer {
                if let Some(p) = Self::pair(rules, producer, consumer)? {
                    return Ok(p);
                }
            }
        }
        Err("no certified private resource boundary")
    }
    fn pair(rules: &[Rule], pi: usize, ci: usize) -> Result<Option<Self>, &'static str> {
        let (p, c) = (&rules[pi], &rules[ci]);
        if !p.kept.is_empty()
            || !c.kept.is_empty()
            || !p.guards.is_empty()
            || !c.guards.is_empty()
            || p.removed.is_empty()
        {
            return Ok(None);
        }
        let Goal::Constraint(middle) = &p.body else {
            return Ok(None);
        };
        let Some(Term::Var(pk)) = middle.args.first() else {
            return Ok(None);
        };
        let mut bound = BTreeSet::from([*pk]);
        let mut producer = BTreeMap::new();
        for head in &p.removed {
            if head.args.first() != Some(&Term::Var(*pk)) {
                return Ok(None);
            }
            for arg in &head.args[1..] {
                let Term::Var(v) = arg else {
                    return Ok(None);
                };
                if !bound.insert(*v) {
                    return Ok(None);
                }
            }
            *producer.entry(key(head)).or_insert(0) += 1;
        }
        if middle
            .args
            .iter()
            .any(|t| !matches!(t,Term::Var(v) if bound.contains(v)))
        {
            return Ok(None);
        }
        let matches = c
            .removed
            .iter()
            .enumerate()
            .filter(|(_, h)| key(h) == key(middle))
            .collect::<Vec<_>>();
        if matches.len() != 1 || c.removed.len() < 2 {
            return Ok(None);
        }
        let (mi, mh) = matches[0];
        let mut parameters = BTreeSet::new();
        let mut env = BTreeMap::new();
        for (arg, actual) in mh.args.iter().zip(&middle.args) {
            let Term::Var(v) = arg else {
                return Ok(None);
            };
            if !parameters.insert(*v) {
                return Ok(None);
            }
            env.insert(*v, actual.clone());
        }
        let Term::Var(ck) = mh.args[0] else {
            return Ok(None);
        };
        let mut extra = BTreeMap::new();
        for (i, head) in c.removed.iter().enumerate() {
            if i == mi {
                continue;
            }
            if head.args != vec![Term::Var(ck)] {
                return Ok(None);
            }
            *extra.entry(key(head)).or_insert(0) += 1;
        }
        let mid = key(middle);
        if producer.contains_key(&mid)
            || extra.contains_key(&mid)
            || producer.keys().any(|k| extra.contains_key(k))
        {
            return Ok(None);
        }
        for (i, r) in rules.iter().enumerate() {
            for h in r.kept.iter().chain(&r.removed) {
                let k = key(h);
                if (k == mid && i != ci)
                    || (producer.contains_key(&k) && i != pi)
                    || (extra.contains_key(&k) && i != ci)
                {
                    return Ok(None);
                }
            }
            let mut calls = vec![];
            goal_calls(&r.body, &mut calls);
            if calls.iter().any(|k| {
                producer.contains_key(k) || extra.contains_key(k) || (*k == mid && i != pi)
            }) {
                return Ok(None);
            }
        }
        let mut used = BTreeSet::new();
        for h in p.removed.iter().chain(&c.removed) {
            for t in &h.args {
                term_vars(t, &mut used);
            }
        }
        goal_vars(&c.body, &mut used);
        let mut next = used
            .iter()
            .map(|v| v.0)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or("variable identity exhausted")?;
        for v in used {
            if let std::collections::btree_map::Entry::Vacant(entry) = env.entry(v) {
                entry.insert(Term::Var(Var(next)));
                next = next.checked_add(1).ok_or("variable identity exhausted")?;
            }
        }
        let mut fused = p.clone();
        fused.name = format!("{}+{}", p.name, c.name);
        fused.removed.extend(
            c.removed
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != mi)
                .map(|(_, h)| subst_constraint(h, &env)),
        );
        fused.body = subst_goal(&c.body, &env);
        let emitted = rules
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != ci)
            .map(|(i, r)| if i == pi { fused.clone() } else { r.clone() })
            .collect();
        Ok(Some(Self {
            emitted,
            producer,
            extra,
            middle: mid,
            fused_rules: (pi, ci),
        }))
    }
    /// The emitted rules are available only after this query's count certificate.
    pub fn lower(&self, q: &Query) -> Result<&[Rule], &'static str> {
        let mut counts: BTreeMap<Term, BTreeMap<Key, usize>> = BTreeMap::new();
        for c in &q.constraints {
            let k = key(c);
            if k == self.middle {
                return Err("initial private intermediate requires ordinary execution");
            }
            if !self.producer.contains_key(&k) && !self.extra.contains_key(&k) {
                continue;
            }
            let Some(owner) = c.args.first() else {
                return Err("missing key");
            };
            if !ground(owner) {
                return Err("resource key must be ground");
            }
            *counts
                .entry(owner.clone())
                .or_default()
                .entry(k)
                .or_default() += 1;
        }
        for group in counts.values() {
            let firings = self
                .producer
                .iter()
                .map(|(k, m)| group.get(k).copied().unwrap_or(0) / m)
                .min()
                .unwrap();
            for (k, m) in &self.extra {
                let required = firings.checked_mul(*m).ok_or("resource count exhausted")?;
                if group.get(k).copied().unwrap_or(0) < required {
                    return Err("insufficient guaranteed consumer resources");
                }
            }
        }
        Ok(&self.emitted)
    }
}
