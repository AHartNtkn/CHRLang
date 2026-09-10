//! Closed consuming-capacity relations inferred from source, without an engine.
use chr_syntax::{Answer, Constraint, Goal, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet};
type Key = (String, usize);
type Domain = BTreeMap<String, usize>;
fn key(c: &Constraint) -> Key {
    (c.name.clone(), c.args.len())
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Source,
    Query,
    Limit,
    Multiplicity,
}
#[derive(Clone, Copy)]
pub struct Limits {
    pub states: usize,
    pub answers: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            states: 100000,
            answers: 4096,
        }
    }
}
pub struct Report {
    pub answers: Vec<Answer>,
    pub states: usize,
    pub branches: usize,
    pub capacity_prunes: usize,
}
pub struct Prepared {
    producers: BTreeMap<Key, Domain>,
    need: Key,
    token: Key,
    done: String,
}
fn atom(t: &Term) -> Option<&str> {
    match t {
        Term::App(n, xs) if xs.is_empty() => Some(n),
        _ => None,
    }
}
fn unary(c: &Constraint) -> Option<Var> {
    match c.args.as_slice() {
        [Term::Var(v)] => Some(*v),
        _ => None,
    }
}
fn charge(left: &mut usize, depth: usize) -> Result<(), Error> {
    if depth > 128 || *left == 0 {
        return Err(Error::Limit);
    }
    *left -= 1;
    Ok(())
}
fn flatten<'a>(
    g: &'a Goal,
    out: &mut Vec<&'a Goal>,
    left: &mut usize,
    depth: usize,
) -> Result<(), Error> {
    charge(left, depth)?;
    match g {
        Goal::And(gs) => {
            for g in gs {
                flatten(g, out, left, depth + 1)?
            }
        }
        _ => out.push(g),
    }
    Ok(())
}
fn choices(
    g: &Goal,
    var: Var,
    out: &mut Domain,
    left: &mut usize,
    depth: usize,
) -> Result<(), Error> {
    charge(left, depth)?;
    match g {
        Goal::Or(a, b) => {
            choices(a, var, out, left, depth + 1)?;
            choices(b, var, out, left, depth + 1)?;
        }
        Goal::Unify(a, b) => {
            let value = if a == &Term::Var(var) {
                atom(b)
            } else if b == &Term::Var(var) {
                atom(a)
            } else {
                None
            }
            .ok_or(Error::Source)?;
            let n = out.entry(value.into()).or_default();
            *n = n.checked_add(1).ok_or(Error::Multiplicity)?;
        }
        _ => return Err(Error::Source),
    }
    Ok(())
}
fn term_check(t: &Term, left: &mut usize, depth: usize) -> Result<(), Error> {
    charge(left, depth)?;
    if let Term::App(_, xs) = t {
        for x in xs {
            term_check(x, left, depth + 1)?
        }
    }
    Ok(())
}
fn substitute(t: &Term, env: &BTreeMap<Var, String>) -> Term {
    match t {
        Term::Var(v) => env
            .get(v)
            .map_or_else(|| t.clone(), |a| Term::App(a.clone(), vec![])),
        Term::App(n, xs) => Term::App(n.clone(), xs.iter().map(|x| substitute(x, env)).collect()),
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Identity {
    Variable(Var),
    Ground(String),
}
struct Group {
    identity: Identity,
    domain: Domain,
    demand: usize,
}
impl Prepared {
    pub fn new(source: &[Rule]) -> Result<Self, Error> {
        if !(3..=4096).contains(&source.len()) {
            return Err(Error::Source);
        }
        let mut names = BTreeSet::new();
        if source
            .iter()
            .any(|r| !r.kept.is_empty() || !r.guards.is_empty() || !names.insert(&r.name))
        {
            return Err(Error::Source);
        }
        let sink = &source[source.len() - 1];
        if sink.body != Goal::Fail || sink.removed.len() != 1 || unary(&sink.removed[0]).is_none() {
            return Err(Error::Source);
        }
        let need = key(&sink.removed[0]);
        let consumer = &source[source.len() - 2];
        if consumer.removed.len() != 2 {
            return Err(Error::Source);
        }
        let h = consumer
            .removed
            .iter()
            .find(|c| key(c) == need)
            .ok_or(Error::Source)?;
        let var = unary(h).ok_or(Error::Source)?;
        let token = consumer
            .removed
            .iter()
            .find(|c| key(c) != need)
            .ok_or(Error::Source)?;
        if unary(token) != Some(var) {
            return Err(Error::Source);
        }
        let Goal::Constraint(done) = &consumer.body else {
            return Err(Error::Source);
        };
        if unary(done) != Some(var) || key(done) == need || key(done) == key(token) {
            return Err(Error::Source);
        }
        let mut producers = BTreeMap::new();
        let mut budget = 100000;
        for r in &source[..source.len() - 2] {
            let [h] = r.removed.as_slice() else {
                return Err(Error::Source);
            };
            let var = unary(h).ok_or(Error::Source)?;
            let k = key(h);
            if k == need || k == key(token) || k == key(done) || producers.contains_key(&k) {
                return Err(Error::Source);
            }
            let mut flat = vec![];
            flatten(&r.body, &mut flat, &mut budget, 0)?;
            let [domain, Goal::Constraint(post)] = flat.as_slice() else {
                return Err(Error::Source);
            };
            if key(post) != need || unary(post) != Some(var) {
                return Err(Error::Source);
            }
            let mut values = Domain::new();
            choices(domain, var, &mut values, &mut budget, 0)?;
            if values.len() > 64 {
                return Err(Error::Limit);
            }
            producers.insert(k, values);
        }
        Ok(Self {
            producers,
            need,
            token: key(token),
            done: done.name.clone(),
        })
    }
    pub fn solve(&self, q: &Query, limits: Limits) -> Result<Report, Error> {
        let mut budget = 100000;
        let mut output_names = BTreeSet::new();
        if q.constraints.len() > 10000
            || q.outputs.len() > 10000
            || q.outputs.iter().any(|(n, _)| !output_names.insert(n))
        {
            return Err(Error::Query);
        }
        let mut groups: BTreeMap<Identity, Group> = BTreeMap::new();
        let mut supply: BTreeMap<String, usize> = BTreeMap::new();
        let mut inert = vec![];
        for c in &q.constraints {
            for t in &c.args {
                term_check(t, &mut budget, 0)?
            }
            let k = key(c);
            if let Some(domain) = self.producers.get(&k) {
                let identity = match &c.args[0] {
                    Term::Var(v) => Identity::Variable(*v),
                    t => Identity::Ground(atom(t).ok_or(Error::Query)?.into()),
                };
                let group = groups.entry(identity).or_insert_with_key(|id| Group {
                    identity: match id {
                        Identity::Variable(v) => Identity::Variable(*v),
                        Identity::Ground(s) => Identity::Ground(s.clone()),
                    },
                    domain: domain
                        .keys()
                        .filter(|a| match id {
                            Identity::Ground(s) => s == *a,
                            _ => true,
                        })
                        .map(|a| (a.clone(), 1))
                        .collect(),
                    demand: 0,
                });
                group.demand += 1;
                let mut next = Domain::new();
                for (value, w) in &group.domain {
                    if let Some(n) = domain.get(value) {
                        next.insert(value.clone(), w.checked_mul(*n).ok_or(Error::Multiplicity)?);
                    }
                }
                group.domain = next;
            } else if k == self.token {
                let value = atom(&c.args[0]).ok_or(Error::Query)?;
                *supply.entry(value.into()).or_default() += 1;
            } else if k == self.need {
                return Err(Error::Query);
            } else {
                inert.push(c.clone())
            }
        }
        if groups.len() > 64 {
            return Err(Error::Limit);
        }
        let groups = groups.into_values().collect::<Vec<_>>();
        let mut search = Search {
            prepared: self,
            query: q,
            groups,
            inert,
            supply,
            env: BTreeMap::new(),
            chosen: vec![],
            limits,
            report: Report {
                answers: vec![],
                states: 0,
                branches: 0,
                capacity_prunes: 0,
            },
        };
        search.visit(0, 1)?;
        Ok(search.report)
    }
}
struct Search<'a> {
    prepared: &'a Prepared,
    query: &'a Query,
    groups: Vec<Group>,
    inert: Vec<Constraint>,
    supply: BTreeMap<String, usize>,
    env: BTreeMap<Var, String>,
    chosen: Vec<String>,
    limits: Limits,
    report: Report,
}
impl Search<'_> {
    fn feasible(&self, start: usize) -> bool {
        let domains = self.groups[start..]
            .iter()
            .map(|g| {
                g.domain
                    .keys()
                    .filter(|v| self.supply.get(*v).copied().unwrap_or(0) >= g.demand)
                    .cloned()
                    .collect::<BTreeSet<_>>()
            })
            .collect::<Vec<_>>();
        if domains.iter().any(BTreeSet::is_empty) {
            return false;
        }
        let mut tests = domains.iter().cloned().collect::<BTreeSet<_>>();
        tests.insert(domains.iter().flat_map(|d| d.iter().cloned()).collect());
        for values in tests {
            let demand = domains
                .iter()
                .zip(&self.groups[start..])
                .filter(|(d, _)| d.is_subset(&values))
                .map(|(_, g)| g.demand)
                .sum::<usize>();
            let capacity = values
                .iter()
                .map(|v| self.supply.get(v).copied().unwrap_or(0))
                .sum::<usize>();
            if demand > capacity {
                return false;
            }
        }
        true
    }
    fn visit(&mut self, index: usize, weight: usize) -> Result<(), Error> {
        if self.report.states >= self.limits.states {
            return Err(Error::Limit);
        }
        self.report.states += 1;
        if !self.feasible(index) {
            self.report.capacity_prunes += 1;
            return Ok(());
        }
        if index == self.groups.len() {
            if weight
                > self
                    .limits
                    .answers
                    .saturating_sub(self.report.answers.len())
            {
                return Err(Error::Limit);
            }
            let mut residual = self
                .inert
                .iter()
                .map(|c| Constraint {
                    name: c.name.clone(),
                    args: c.args.iter().map(|t| substitute(t, &self.env)).collect(),
                })
                .collect::<Vec<_>>();
            for (g, value) in self.groups.iter().zip(&self.chosen) {
                for _ in 0..g.demand {
                    residual.push(Constraint {
                        name: self.prepared.done.clone(),
                        args: vec![Term::App(value.clone(), vec![])],
                    });
                }
            }
            for (value, n) in &self.supply {
                for _ in 0..*n {
                    residual.push(Constraint {
                        name: self.prepared.token.0.clone(),
                        args: vec![Term::App(value.clone(), vec![])],
                    });
                }
            }
            let outputs = self
                .query
                .outputs
                .iter()
                .map(|(n, v)| (n.clone(), substitute(&Term::Var(*v), &self.env)))
                .collect();
            let answer = Answer { outputs, residual };
            self.report
                .answers
                .extend((0..weight).map(|_| answer.clone()));
            return Ok(());
        }
        let demand = self.groups[index].demand;
        let choices = self.groups[index].domain.clone();
        for (value, w) in choices {
            let n = self.supply.get(&value).copied().unwrap_or(0);
            if n < demand {
                continue;
            }
            self.report.branches += 1;
            self.supply.insert(value.clone(), n - demand);
            let var = match self.groups[index].identity {
                Identity::Variable(v) => Some(v),
                _ => None,
            };
            if let Some(v) = var {
                self.env.insert(v, value.clone());
            }
            self.chosen.push(value.clone());
            self.visit(index + 1, weight.checked_mul(w).ok_or(Error::Multiplicity)?)?;
            self.chosen.pop();
            if let Some(v) = var {
                self.env.remove(&v);
            }
            self.supply.insert(value, n);
        }
        Ok(())
    }
}
