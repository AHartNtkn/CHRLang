//! Experimental source-derived finite initial phases. No engine or oracle dependency.
//! Atom choices are weighted domains; deterministic source matching partitions them
//! on demand. Service advances one symbolic state; results publish only after
//! complete admission. A service call has variable work within the input limits.
use chr_syntax::{Constraint, Goal, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet};
type Key = (String, usize);
type Domain = BTreeMap<String, u128>;
type Env = BTreeMap<Var, Term>;
fn key(c: &Constraint) -> Key {
    (c.name.clone(), c.args.len())
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    AlreadyFinished,
    Source(&'static str),
    Suspended,
    Limit(&'static str),
    MultiplicityOverflow,
}
#[derive(Clone, Copy)]
pub struct Limits {
    pub steps: usize,
    pub partitions: usize,
    pub solutions: usize,
    pub term_nodes: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            steps: 100_000,
            partitions: 4096,
            solutions: 4096,
            term_nodes: 2_000_000,
        }
    }
}
struct Budget {
    left: usize,
}
impl Budget {
    fn node(&mut self, depth: usize) -> Result<(), Error> {
        if depth > 128 {
            return Err(Error::Limit("term depth"));
        }
        self.left = self.left.checked_sub(1).ok_or(Error::Limit("term nodes"))?;
        Ok(())
    }
}
fn resolve(t: &Term, env: &Env, budget: &mut Budget, depth: usize) -> Result<Term, Error> {
    budget.node(depth)?;
    Ok(match t {
        Term::Var(v) => match env.get(v) {
            Some(t) => return resolve(t, env, budget, depth + 1),
            None => t.clone(),
        },
        Term::App(n, xs) => Term::App(
            n.clone(),
            xs.iter()
                .map(|x| resolve(x, env, budget, depth + 1))
                .collect::<Result<_, _>>()?,
        ),
    })
}
fn term_vars(t: &Term, out: &mut BTreeSet<Var>) {
    match t {
        Term::Var(v) => {
            out.insert(*v);
        }
        Term::App(_, xs) => {
            for x in xs {
                term_vars(x, out)
            }
        }
    }
}
fn goal_check(
    g: &Goal,
    head: &BTreeSet<Var>,
    calls: &mut Vec<Key>,
    budget: &mut Budget,
    depth: usize,
) -> Result<(), Error> {
    budget.node(depth)?;
    let mut check = |t: &Term| -> Result<(), Error> {
        resolve(t, &Env::new(), budget, 0)?;
        let mut vars = BTreeSet::new();
        term_vars(t, &mut vars);
        if !vars.is_subset(head) {
            return Err(Error::Source("fresh body variable"));
        }
        Ok(())
    };
    match g {
        Goal::Constraint(c) => {
            for t in &c.args {
                check(t)?;
            }
            calls.push(key(c));
        }
        Goal::Unify(a, b) => {
            check(a)?;
            check(b)?;
        }
        Goal::And(gs) => {
            for g in gs {
                goal_check(g, head, calls, budget, depth + 1)?;
            }
        }
        Goal::Or(..) => return Err(Error::Source("non-producer alternative")),
        Goal::True | Goal::Fail => (),
    }
    Ok(())
}
fn choice(
    g: &Goal,
    variable: Var,
    budget: &mut Budget,
    depth: usize,
) -> Result<Option<Domain>, Error> {
    budget.node(depth)?;
    let mut result = Domain::new();
    match g {
        Goal::Or(a, b) => {
            let (Some(a), Some(b)) = (
                choice(a, variable, budget, depth + 1)?,
                choice(b, variable, budget, depth + 1)?,
            ) else {
                return Ok(None);
            };
            result = a;
            for (v, w) in b {
                let e = result.entry(v).or_default();
                *e = e.checked_add(w).ok_or(Error::MultiplicityOverflow)?;
            }
        }
        Goal::Unify(a, b) => {
            let value = match (a, b) {
                (Term::Var(v), Term::App(n, xs)) | (Term::App(n, xs), Term::Var(v))
                    if *v == variable && xs.is_empty() =>
                {
                    n
                }
                _ => return Ok(None),
            };
            result.insert(value.clone(), 1);
        }
        Goal::Fail => (),
        _ => return Ok(None),
    }
    Ok(Some(result))
}
#[derive(Clone)]
pub struct Prepared {
    private: BTreeSet<Key>,
    producers: BTreeMap<Key, Domain>,
    rules: Vec<Rule>,
}
impl Prepared {
    /// `prefix` selects an actual initial source phase, not an unchecked declaration.
    pub fn new(source: &[Rule], prefix: usize) -> Result<Self, Error> {
        if prefix == 0 || prefix > source.len() || source.len() > 4096 {
            return Err(Error::Source("invalid prefix"));
        }
        let mut names = BTreeSet::new();
        if source.iter().any(|r| !names.insert(&r.name)) {
            return Err(Error::Source("duplicate rule name"));
        }
        let mut budget = Budget { left: 2_000_000 };
        let mut private = BTreeSet::new();
        let mut producers = BTreeMap::new();
        let mut rules = vec![];
        let mut deterministic = false;
        let mut pending_calls = vec![];
        for r in &source[..prefix] {
            if !r.kept.is_empty() || r.removed.len() != 1 || !r.guards.is_empty() {
                return Err(Error::Source(
                    "private rule must be unguarded single-head consumption",
                ));
            }
            let h = &r.removed[0];
            let k = key(h);
            private.insert(k.clone());
            let mut seen = BTreeSet::new();
            fn linear(t: &Term, seen: &mut BTreeSet<Var>) -> bool {
                match t {
                    Term::Var(v) => seen.insert(*v),
                    Term::App(_, xs) => xs.iter().all(|x| linear(x, seen)),
                }
            }
            for t in &h.args {
                resolve(t, &Env::new(), &mut budget, 0)?;
                if !linear(t, &mut seen) {
                    return Err(Error::Source("nonlinear private head"));
                }
            }
            let extracted = if let [Term::Var(v)] = h.args.as_slice() {
                choice(&r.body, *v, &mut budget, 0)?
            } else {
                None
            };
            if let Some(domain) = extracted {
                if deterministic || producers.insert(k, domain).is_some() {
                    return Err(Error::Source(
                        "choice producers must be unique and precede deterministic rules",
                    ));
                }
            } else {
                deterministic = true;
                goal_check(&r.body, &seen, &mut pending_calls, &mut budget, 0)?;
                rules.push(r.clone());
            }
        }
        if producers.is_empty() {
            return Err(Error::Source("no finite choice producer"));
        }
        if rules
            .iter()
            .any(|r| producers.contains_key(&key(&r.removed[0])))
        {
            return Err(Error::Source("competing choice head"));
        }
        if pending_calls
            .iter()
            .any(|k| !private.contains(k) || producers.contains_key(k))
        {
            return Err(Error::Source("body escapes phase or creates choices"));
        }
        if source[prefix..]
            .iter()
            .flat_map(|r| r.kept.iter().chain(&r.removed))
            .any(|c| private.contains(&key(c)))
        {
            return Err(Error::Source("outside head touches private predicate"));
        }
        Ok(Self {
            private,
            producers,
            rules,
        })
    }
    pub fn start<'a>(&'a self, query: &Query, limits: Limits) -> Result<Machine<'a>, Error> {
        let mut budget = Budget {
            left: limits.term_nodes,
        };
        for c in &query.constraints {
            for t in &c.args {
                resolve(t, &Env::new(), &mut budget, 0)?;
            }
        }
        let mut state = State {
            live: vec![],
            bindings: Env::new(),
            domains: BTreeMap::new(),
            weight: 1,
        };
        let mut caller = query.clone();
        caller.constraints.clear();
        for c in &query.constraints {
            if let Some(domain) = self.producers.get(&key(c)) {
                match &c.args[0] {
                    Term::Var(v) => {
                        let joined = match state.domains.remove(v) {
                            Some(old) => intersection(&old, domain)?,
                            None => domain.clone(),
                        };
                        if joined.is_empty() {
                            return Ok(Machine::new(self, caller, budget, limits, vec![]));
                        }
                        state.domains.insert(*v, joined);
                    }
                    Term::App(n, xs) if xs.is_empty() => {
                        let Some(w) = domain.get(n) else {
                            return Ok(Machine::new(self, caller, budget, limits, vec![]));
                        };
                        state.weight = mul(state.weight, *w)?;
                    }
                    _ => return Err(Error::Source("choice input must be a variable or atom")),
                }
            } else if self.private.contains(&key(c)) {
                state.live.push(c.clone());
            } else {
                caller.constraints.push(c.clone());
            }
        }
        Ok(Machine::new(self, caller, budget, limits, vec![state]))
    }
    pub fn solve(&self, query: &Query, limits: Limits) -> Result<Report, Error> {
        self.start(query, limits)?.finish()
    }
}
pub enum Event {
    Progress,
    Complete(Report),
    Exhausted,
}
/// Owns all query work. Dropping it cancels; the prepared rules are only borrowed.
pub struct Machine<'a> {
    prepared: &'a Prepared,
    work: Option<Work>,
}
struct Work {
    caller: Query,
    budget: Budget,
    limits: Limits,
    queue: Vec<State>,
    report: Report,
}
impl<'a> Machine<'a> {
    fn new(
        prepared: &'a Prepared,
        caller: Query,
        budget: Budget,
        limits: Limits,
        queue: Vec<State>,
    ) -> Self {
        Self {
            prepared,
            work: Some(Work {
                caller,
                budget,
                limits,
                queue,
                report: Report {
                    solutions: vec![],
                    steps: 0,
                    partitions: 0,
                },
            }),
        }
    }
    pub fn advance(&mut self) -> Result<Event, Error> {
        let Some(mut work) = self.work.take() else {
            return Ok(Event::Exhausted);
        };
        // An error drops both pending branches and unpublished solutions.
        if work.step(self.prepared)? {
            Ok(Event::Complete(work.report))
        } else {
            self.work = Some(work);
            Ok(Event::Progress)
        }
    }
    pub fn retained(&self) -> (usize, usize) {
        self.work
            .as_ref()
            .map_or((0, 0), |w| (w.queue.len(), w.report.solutions.len()))
    }
    pub fn finish(mut self) -> Result<Report, Error> {
        loop {
            match self.advance()? {
                Event::Progress => (),
                Event::Complete(r) => return Ok(r),
                Event::Exhausted => return Err(Error::AlreadyFinished),
            }
        }
    }
}
impl Work {
    fn step(&mut self, prepared: &Prepared) -> Result<bool, Error> {
        let Self {
            caller,
            budget,
            limits,
            queue,
            report,
        } = self;
        let limits = *limits;
        let Some(mut state) = queue.pop() else {
            return Ok(true);
        };

        if report.steps == limits.steps {
            return Err(Error::Limit("steps"));
        }
        report.steps += 1;
        let mut selected = None;
        'rules: for r in &prepared.rules {
            for (index, c) in state.live.iter().enumerate() {
                if key(c) != key(&r.removed[0]) {
                    continue;
                }
                let mut env = Env::new();
                let mut conditions = BTreeMap::new();
                let mut matched = true;
                for (p, t) in r.removed[0].args.iter().zip(&c.args) {
                    let t = resolve(t, &state.bindings, budget, 0)?;
                    if !pattern(p, &t, &state.domains, &mut env, &mut conditions) {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    selected = Some((r, index, env, conditions));
                    break 'rules;
                }
            }
        }
        if let Some((r, index, env, conditions)) = selected {
            // Complement of a conjunction is disjoint boxes, not a full product.
            for (v, value) in conditions {
                // The matching value is present; a singleton has no complement.
                if state.domains[&v].len() > 1 {
                    let mut negative = state.clone();
                    negative
                        .domains
                        .get_mut(&v)
                        .expect("condition domain")
                        .remove(&value);
                    fork(report, limits)?;
                    queue.push(negative);
                }
                state.select(v, &value)?;
            }
            state.live.remove(index);
            if state.body(&r.body, &env, budget)? {
                queue.push(state);
            }
            return Ok(false);
        }
        if !state.live.is_empty() {
            return Err(Error::Suspended);
        }
        if let Some((&v, values)) = state.domains.first_key_value() {
            for value in values.keys() {
                let mut child = state.clone();
                child.select(v, value)?;
                fork(report, limits)?;
                queue.push(child);
            }
            return Ok(false);
        }
        if report.solutions.len() == limits.solutions {
            return Err(Error::Limit("solutions"));
        }
        let mut q = caller.clone();
        for c in &mut q.constraints {
            for t in &mut c.args {
                *t = resolve(t, &state.bindings, budget, 0)?;
            }
        }
        let equations = q
            .outputs
            .iter()
            .map(|(_, v)| {
                Ok((
                    Term::Var(*v),
                    resolve(&Term::Var(*v), &state.bindings, budget, 0)?,
                ))
            })
            .collect::<Result<_, Error>>()?;
        report.solutions.push(Solution {
            query: q,
            equations,
            multiplicity: state.weight,
        });

        Ok(false)
    }
}

fn fork(report: &mut Report, limits: Limits) -> Result<(), Error> {
    if report.partitions == limits.partitions {
        return Err(Error::Limit("partitions"));
    }
    report.partitions += 1;
    Ok(())
}
#[derive(Debug)]
pub struct Solution {
    pub query: Query,
    pub equations: Vec<(Term, Term)>,
    pub multiplicity: u128,
}
#[derive(Debug)]
pub struct Report {
    pub solutions: Vec<Solution>,
    pub steps: usize,
    pub partitions: usize,
}
fn mul(a: u128, b: u128) -> Result<u128, Error> {
    a.checked_mul(b).ok_or(Error::MultiplicityOverflow)
}
fn intersection(a: &Domain, b: &Domain) -> Result<Domain, Error> {
    a.iter()
        .filter_map(|(v, x)| b.get(v).map(|y| mul(*x, *y).map(|w| (v.clone(), w))))
        .collect()
}
#[derive(Clone)]
struct State {
    live: Vec<Constraint>,
    bindings: Env,
    domains: BTreeMap<Var, Domain>,
    weight: u128,
}
impl State {
    fn select(&mut self, v: Var, value: &str) -> Result<(), Error> {
        let d = self.domains.remove(&v).expect("selected domain");
        self.weight = mul(self.weight, d[value])?;
        self.bindings.insert(v, Term::App(value.into(), vec![]));
        Ok(())
    }
    fn unify(&mut self, a: Term, b: Term, budget: &mut Budget) -> Result<bool, Error> {
        let mut pending = vec![(a, b)];
        while let Some((a, b)) = pending.pop() {
            let a = resolve(&a, &self.bindings, budget, 0)?;
            let b = resolve(&b, &self.bindings, budget, 0)?;
            if a == b {
                continue;
            }
            match (a, b) {
                (Term::Var(a), Term::Var(b)) => {
                    match (self.domains.remove(&a), self.domains.remove(&b)) {
                        (Some(a_domain), Some(b_domain)) => {
                            let d = intersection(&a_domain, &b_domain)?;
                            if d.is_empty() {
                                return Ok(false);
                            }
                            self.domains.insert(a, d);
                        }
                        (Some(d), None) | (None, Some(d)) => {
                            self.domains.insert(a, d);
                        }
                        _ => (),
                    }
                    self.bindings.insert(b, Term::Var(a));
                }
                (Term::Var(v), t) | (t, Term::Var(v)) => {
                    if let Some(d) = self.domains.get(&v) {
                        let Term::App(n, xs) = &t else { unreachable!() };
                        if !xs.is_empty() || !d.contains_key(n) {
                            return Ok(false);
                        }
                        self.select(v, n)?;
                    } else {
                        let mut vars = BTreeSet::new();
                        term_vars(&t, &mut vars);
                        if vars.contains(&v) {
                            return Ok(false);
                        }
                        self.bindings.insert(v, t);
                    }
                }
                (Term::App(a, x), Term::App(b, y)) => {
                    if a != b || x.len() != y.len() {
                        return Ok(false);
                    }
                    pending.extend(x.into_iter().zip(y));
                }
            }
        }
        Ok(true)
    }
    fn body(&mut self, g: &Goal, env: &Env, budget: &mut Budget) -> Result<bool, Error> {
        // Instantiation substitutes rule variables once: query ids may equal rule ids.
        fn instantiate(
            t: &Term,
            env: &Env,
            budget: &mut Budget,
            depth: usize,
        ) -> Result<Term, Error> {
            budget.node(depth)?;
            match t {
                Term::Var(v) => resolve(&env[v], &Env::new(), budget, depth + 1),
                Term::App(n, xs) => Ok(Term::App(
                    n.clone(),
                    xs.iter()
                        .map(|x| instantiate(x, env, budget, depth + 1))
                        .collect::<Result<_, _>>()?,
                )),
            }
        }
        match g {
            Goal::True => Ok(true),
            Goal::Fail => Ok(false),
            Goal::Unify(a, b) => {
                let a = instantiate(a, env, budget, 0)?;
                let b = instantiate(b, env, budget, 0)?;
                self.unify(a, b, budget)
            }
            Goal::Constraint(c) => {
                self.live.push(Constraint {
                    name: c.name.clone(),
                    args: c
                        .args
                        .iter()
                        .map(|t| instantiate(t, env, budget, 0))
                        .collect::<Result<_, _>>()?,
                });
                Ok(true)
            }
            Goal::And(gs) => {
                for g in gs {
                    if !self.body(g, env, budget)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Goal::Or(..) => unreachable!("checked deterministic body"),
        }
    }
}
fn pattern(
    p: &Term,
    t: &Term,
    domains: &BTreeMap<Var, Domain>,
    env: &mut Env,
    conditions: &mut BTreeMap<Var, String>,
) -> bool {
    match (p, t) {
        (Term::Var(v), t) => {
            env.insert(*v, t.clone());
            true
        }
        (Term::App(n, xs), Term::Var(v)) => {
            if !xs.is_empty() || !domains.get(v).is_some_and(|d| d.contains_key(n)) {
                return false;
            }
            match conditions.get(v) {
                Some(old) => old == n,
                None => {
                    conditions.insert(*v, n.clone());
                    true
                }
            }
        }
        (Term::App(a, x), Term::App(b, y)) => {
            a == b
                && x.len() == y.len()
                && x.iter()
                    .zip(y)
                    .all(|(p, t)| pattern(p, t, domains, env, conditions))
        }
    }
}

#[allow(dead_code)]
#[path = "finite_learning.rs"]
pub mod learning;
