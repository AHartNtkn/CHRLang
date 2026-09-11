//! Finite ground regions with set or counted tuple observations.
use crate::{
    joint,
    normal_forms::Requirement,
    projection::{Problem, Projection, Relation, Semantics},
};
use chr_syntax::{Constraint, Goal, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug)]
pub enum Predicate {
    Name(Term),
    Different(Term, Term),
    Equal(Term, Term),
    Structure(Term, Requirement),
}
#[derive(Clone, Copy)]
pub enum Observation {
    LogicalSet,
    /// Domain-choice multiplicities, without source scheduling order.
    Counted,
    RawAnswers,
}
pub struct Region {
    pub domains: BTreeMap<Var, Vec<Term>>,
    pub predicates: Vec<Predicate>,
}
pub struct Prepared {
    #[cfg(feature = "phase-clock")]
    pub preparation_ns: [u128; 4],
    projection: Projection,
    variables: Vec<Var>,
    domains: Vec<Vec<Term>>,
    visible: Vec<usize>,
    output_slots: Vec<usize>,
}
fn variables(t: &Term, out: &mut BTreeSet<Var>) {
    match t {
        Term::Var(x) => {
            out.insert(*x);
        }
        Term::App(_, a) => {
            for t in a {
                variables(t, out)
            }
        }
    }
}
fn terms(p: &Predicate) -> Vec<&Term> {
    match p {
        Predicate::Name(t) | Predicate::Structure(t, _) => vec![t],
        Predicate::Different(a, b) | Predicate::Equal(a, b) => vec![a, b],
    }
}
fn value(t: &Term, b: &BTreeMap<Var, Term>) -> Term {
    match t {
        Term::Var(x) => b[x].clone(),
        Term::App(n, a) => Term::App(n.clone(), a.iter().map(|t| value(t, b)).collect()),
    }
}
fn literal(name: &str) -> bool {
    matches!(name, "var" | "neq" | "norm")
}
fn posts(g: &Goal) -> bool {
    match g {
        Goal::Constraint(c) => literal(&c.name),
        Goal::And(gs) => gs.iter().any(posts),
        Goal::Or(a, b) => posts(a) || posts(b),
        _ => false,
    }
}
impl Region {
    pub fn prepare(
        &self,
        visible: &[Var],
        host: &[Constraint],
        rules: &[Rule],
        observation: Observation,
        limit: usize,
    ) -> Result<Prepared, String> {
        #[cfg(feature = "phase-clock")]
        let mut clock = std::time::Instant::now();
        #[cfg(feature = "phase-clock")]
        let mut preparation_ns = [0; 4];
        if matches!(observation, Observation::RawAnswers) {
            return Err("logical set boundary cannot replace raw answers".into());
        }
        if rules
            .iter()
            .any(|r| r.kept.iter().chain(&r.removed).any(|c| literal(&c.name)) || posts(&r.body))
        {
            return Err("host rule can interact with literal theory predicates".into());
        }
        if host.iter().any(|c| literal(&c.name)) {
            return Err("literal theory occurrences remain in host".into());
        }
        let ids = self
            .domains
            .keys()
            .copied()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<BTreeMap<_, _>>();
        let output = visible
            .iter()
            .map(|x| {
                ids.get(x)
                    .copied()
                    .ok_or("unknown visible variable".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut projected = Vec::new();
        let output_slots = output
            .iter()
            .map(|i| {
                if let Some(j) = projected.iter().position(|x| x == i) {
                    j
                } else {
                    projected.push(*i);
                    projected.len() - 1
                }
            })
            .collect();
        let output = projected;
        let mut shared = BTreeSet::new();
        for c in host {
            for t in &c.args {
                variables(t, &mut shared)
            }
        }
        let shared = shared
            .into_iter()
            .filter_map(|x| ids.get(&x).copied())
            .collect::<Vec<_>>();
        let domains = self
            .domains
            .values()
            .map(|xs| {
                for t in xs {
                    let mut vars = BTreeSet::new();
                    variables(t, &mut vars);
                    if !vars.is_empty() {
                        return Err(
                            "nonground domain requires symbolic output contract".to_string()
                        );
                    }
                }
                let xs = xs
                    .iter()
                    .cloned()
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>();
                if xs.len() > 256 {
                    return Err("domain encoding capacity exceeded".into());
                }
                Ok(xs)
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut problem = Problem {
            domains: domains
                .iter()
                .zip(self.domains.values())
                .map(|(dictionary, choices)| {
                    choices
                        .iter()
                        .map(|t| dictionary.binary_search(t).unwrap() as u8)
                        .collect()
                })
                .collect(),
            filters: vec![],
        };
        #[cfg(feature = "phase-clock")]
        {
            preparation_ns[0] = clock.elapsed().as_nanos();
            clock = std::time::Instant::now();
        }
        let mut spent = 0usize;
        for p in &self.predicates {
            let mut scope = BTreeSet::new();
            for t in terms(p) {
                variables(t, &mut scope)
            }
            let scope = scope
                .into_iter()
                .map(|x| {
                    ids.get(&x)
                        .copied()
                        .ok_or("predicate variable has no domain".to_string())
                })
                .collect::<Result<Vec<_>, _>>()?;
            let count = scope.iter().try_fold(1usize, |n, i| {
                n.checked_mul(domains[*i].len())
                    .ok_or("relation work overflow")
            })?;
            spent = spent.checked_add(count).ok_or("relation work overflow")?;
            if spent > limit {
                return Err("relation preparation bound".into());
            }
            let mut rows = vec![];
            for mut k in 0..count {
                let mut row = vec![0; scope.len()];
                let mut binding = BTreeMap::new();
                for (j, i) in scope.iter().enumerate().rev() {
                    let index = k % domains[*i].len();
                    k /= domains[*i].len();
                    row[j] = index as u8;
                    binding.insert(
                        *ids.iter().find(|(_, v)| **v == *i).unwrap().0,
                        domains[*i][index].clone(),
                    );
                }
                let empty = BTreeMap::new();
                let ok = match p {
                    // Domains are ground before relation construction. These
                    // two predicates need no existential theory compilation.
                    Predicate::Name(t) => {
                        matches!(value(t, &binding), Term::App(_, args) if args.is_empty())
                    }
                    Predicate::Different(a, b) => {
                        let (a, b) = (value(a, &binding), value(b, &binding));
                        matches!((&a, &b), (Term::App(_, x), Term::App(_, y))
                            if x.is_empty() && y.is_empty() && a != b)
                    }
                    Predicate::Equal(a, b) => value(a, &binding) == value(b, &binding),
                    Predicate::Structure(t, r) => {
                        joint::feasible(&[], &[(value(t, &binding), *r)], &[], &empty, None)?
                    }
                };
                if ok {
                    rows.push(row)
                }
            }
            problem.filters.push(Relation { scope, rows });
        }
        #[cfg(feature = "phase-clock")]
        {
            preparation_ns[1] = clock.elapsed().as_nanos();
            clock = std::time::Instant::now();
        }
        let order = problem.elimination_order(&output)?;
        #[cfg(feature = "phase-clock")]
        {
            preparation_ns[2] = clock.elapsed().as_nanos();
            clock = std::time::Instant::now();
        }
        let semantics = match observation {
            Observation::Counted => Semantics::Counted,
            Observation::LogicalSet => Semantics::Set,
            Observation::RawAnswers => unreachable!(),
        };
        let projection = problem.project(&output, &shared, &order, semantics, limit)?;
        #[cfg(feature = "phase-clock")]
        {
            preparation_ns[3] = clock.elapsed().as_nanos();
        }
        Ok(Prepared {
            #[cfg(feature = "phase-clock")]
            preparation_ns,
            projection,
            variables: self.domains.keys().copied().collect(),
            domains,
            visible: output,
            output_slots,
        })
    }
}
impl Prepared {
    pub fn answers(
        &self,
        restrictions: &[(Var, Term)],
        limit: usize,
    ) -> Result<BTreeSet<Vec<Term>>, String> {
        Ok(self
            .weighted_answers(restrictions, limit)?
            .into_keys()
            .collect())
    }
    pub fn weighted_answers(
        &self,
        restrictions: &[(Var, Term)],
        limit: usize,
    ) -> Result<BTreeMap<Vec<Term>, u128>, String> {
        let mut coordinates = Vec::with_capacity(restrictions.len());
        for (x, t) in restrictions {
            let i = self
                .variables
                .iter()
                .position(|y| y == x)
                .ok_or("unknown caller restriction")?;
            if !self.visible.contains(&i) {
                return Err("caller restriction on hidden variable".into());
            }
            coordinates.push((i, t));
        }
        let mut mapped = vec![];
        for (i, t) in coordinates {
            let Some(value) = self.domains[i].iter().position(|v| v == t) else {
                return Ok(BTreeMap::new());
            };
            mapped.push((i, value as u8));
        }
        Ok(self
            .projection
            .answers(&mapped, limit)?
            .into_iter()
            .map(|(row, weight)| {
                (
                    self.output_slots
                        .iter()
                        .map(|j| self.domains[self.visible[*j]][row[*j] as usize].clone())
                        .collect(),
                    weight,
                )
            })
            .collect())
    }
}
