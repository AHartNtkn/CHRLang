//! One equality interpretation owns both relation facts and source resources.
//! Forking clones ownership. This is not shared contextual equality.
use crate::{Evaluation, HeadPlan, Match, Occurrence, Relation, Value, View};
use chr_syntax::{Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
#[derive(Default, Clone)]
pub struct Store {
    pub(crate) view: View,
    parents: Vec<Value>,
    ranks: Vec<usize>,
    equations: VecDeque<(Value, Value)>,
    queued: BTreeSet<(Value, Value)>,
    failed: bool,
    next_occurrence: usize,
}
impl Store {
    pub fn unknown(&mut self) -> Value {
        #[cfg(feature = "admission-profile")]
        let _scope =
            crate::deduction_profile::Scope::new(crate::deduction_profile::Phase::ValueCreate);
        let value = Value(self.parents.len());
        self.parents.push(value);
        self.ranks.push(0);
        value
    }
    pub fn root(&self, mut value: Value) -> Value {
        while self.parents[value.0] != value {
            value = self.parents[value.0];
        }
        value
    }
    pub fn constructor(&mut self, name: &str, children: &[Value]) -> Value {
        #[cfg(feature = "admission-profile")]
        let _scope =
            crate::deduction_profile::Scope::new(crate::deduction_profile::Phase::Constructor);
        let children = children.iter().map(|v| self.root(*v)).collect::<Vec<_>>();
        let peers = {
            #[cfg(feature = "admission-profile")]
            let _scope = crate::deduction_profile::Scope::new(
                crate::deduction_profile::Phase::ConstructorLookup,
            );
            self.view.peers(name, &children)
        };
        if let Some(value) = peers.first() {
            return self.root(*value);
        }
        let value = self.unknown();
        self.view.constructor(name, value, &children);
        value
    }
    pub fn post(&mut self, name: &str, args: &[Value]) -> Occurrence {
        #[cfg(feature = "admission-profile")]
        let _scope = crate::deduction_profile::Scope::new(crate::deduction_profile::Phase::Post);
        let args = args.iter().map(|v| self.root(*v)).collect::<Vec<_>>();
        let id = Occurrence(self.next_occurrence);
        self.next_occurrence = self
            .next_occurrence
            .checked_add(1)
            .expect("occurrence identity exhausted");
        self.view.occurrence(id, name, &args);
        id
    }
    pub fn equate(&mut self, a: Value, b: Value) {
        if self.failed {
            return;
        }
        let (a, b) = (self.root(a), self.root(b));
        if a == b {
            return;
        }
        let pair = if a < b { (a, b) } else { (b, a) };
        if self.queued.insert(pair) {
            self.equations.push_back(pair);
        }
    }
    fn reaches(&self, from: Value, target: Value) -> bool {
        #[cfg(feature = "execution-profile")]
        let _scope = crate::deduction_profile::Scope::new(crate::deduction_profile::Phase::Cycles);
        let mut todo = vec![from];
        let mut seen = BTreeSet::new();
        while let Some(value) = todo.pop() {
            if !seen.insert(value) {
                continue;
            }
            #[cfg(feature = "borrowed-cycles")]
            for children in self.view.constructor_children(value) {
                if children.contains(&target) {
                    return true;
                }
                todo.extend(children.iter().copied());
            }
            #[cfg(not(feature = "borrowed-cycles"))]
            for (_, children) in self.view.descriptors(value) {
                if children.contains(&target) {
                    return true;
                }
                todo.extend(children);
            }
        }
        false
    }
    fn fail(&mut self) {
        self.failed = true;
        self.equations.clear();
        self.queued.clear();
    }
    /// Process one equality; incidence repair and cycle checks are finite but
    /// size-dependent. Child and congruence deductions remain queued for service.
    pub fn step(&mut self) -> bool {
        #[cfg(feature = "execution-profile")]
        let _scope =
            crate::deduction_profile::Scope::new(crate::deduction_profile::Phase::Equality);
        let Some(pair) = self.equations.pop_front() else {
            return false;
        };
        self.queued.remove(&pair);
        let (mut a, mut b) = (self.root(pair.0), self.root(pair.1));
        if a == b {
            return true;
        }
        let left = self.view.descriptors(a);
        let right = self.view.descriptors(b);
        if left
            .iter()
            .any(|(n, xs)| right.iter().any(|(m, ys)| n != m || xs.len() != ys.len()))
            || self.reaches(a, b)
            || self.reaches(b, a)
        {
            self.fail();
            return true;
        }
        for (_, xs) in &left {
            for (_, ys) in &right {
                for (x, y) in xs.iter().zip(ys) {
                    self.equate(*x, *y);
                }
            }
        }
        if self.ranks[a.0] < self.ranks[b.0] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parents[b.0] = a;
        if self.ranks[a.0] == self.ranks[b.0] {
            self.ranks[a.0] += 1;
        }
        let changed = self.view.replace_value(b, a);
        for (key, index) in changed {
            let values = self.view.tables[&key].rows[index].values.clone();
            let Relation::Constructor(name, _) = key else {
                unreachable!()
            };
            for peer in self.view.peers(&name, &values[1..]) {
                self.equate(values[0], peer);
            }
        }
        true
    }
    pub fn pending(&self) -> usize {
        self.equations.len()
    }
    pub fn failed(&self) -> bool {
        self.failed
    }
    pub fn matches(&self, plan: &HeadPlan) -> Evaluation {
        #[cfg(feature = "execution-profile")]
        let _scope =
            crate::deduction_profile::Scope::new(crate::deduction_profile::Phase::Discovery);
        if self.failed {
            Evaluation::default()
        } else {
            plan.evaluate(&self.view)
        }
    }
    pub fn consume(&mut self, claim: &Match) -> bool {
        if self.failed {
            return false;
        }
        let ids = claim
            .kept
            .iter()
            .chain(&claim.removed)
            .copied()
            .collect::<Vec<_>>();
        if ids.iter().copied().collect::<BTreeSet<_>>().len() != ids.len()
            || ids.iter().any(|id| !self.view.locations.contains_key(id))
        {
            return false;
        }
        for id in &claim.removed {
            self.view.retire(*id);
        }
        true
    }
    fn term(&self, value: Value) -> Term {
        let value = self.root(value);
        match self.view.descriptors(value).first() {
            Some((name, children)) => Term::App(
                name.clone(),
                children.iter().map(|v| self.term(*v)).collect(),
            ),
            None => Term::Var(Var(value.0 as u64)),
        }
    }
    /// Equality-settled export, not a claim that source rules are quiescent.
    pub fn export(&self, values: &[Value]) -> Option<Vec<Term>> {
        if self.failed || !self.equations.is_empty() {
            None
        } else {
            Some(values.iter().map(|v| self.term(*v)).collect())
        }
    }
}

/// Source columns whose values affect matching or positive guards. Opaque,
/// single-use head variables passed only to a body need no settled value.
pub struct MatcherReads {
    columns: BTreeMap<Relation, BTreeSet<usize>>,
}
impl MatcherReads {
    pub fn new(rules: &[chr_syntax::Rule]) -> Self {
        fn variables(t: &Term, out: &mut Vec<Var>) {
            match t {
                Term::Var(v) => out.push(*v),
                Term::App(_, xs) => {
                    for x in xs {
                        variables(x, out);
                    }
                }
            }
        }
        let mut columns: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
        for rule in rules {
            let mut head_vars = vec![];
            for head in rule.kept.iter().chain(&rule.removed) {
                for arg in &head.args {
                    variables(arg, &mut head_vars);
                }
            }
            let mut counts = BTreeMap::new();
            for v in head_vars {
                *counts.entry(v).or_insert(0usize) += 1;
            }
            let mut guard_vars = vec![];
            for chr_syntax::Guard::Equal(a, b) in &rule.guards {
                variables(a, &mut guard_vars);
                variables(b, &mut guard_vars);
            }
            let guard_vars = guard_vars.into_iter().collect::<BTreeSet<_>>();
            for head in rule.kept.iter().chain(&rule.removed) {
                for (column, arg) in head.args.iter().enumerate() {
                    let needed = match arg {
                        Term::App(..) => true,
                        Term::Var(v) => counts[v] > 1 || guard_vars.contains(v),
                    };
                    if needed {
                        columns
                            .entry(Relation::Source(head.name.clone(), head.args.len()))
                            .or_default()
                            .insert(column);
                    }
                }
            }
        }
        Self { columns }
    }
}
impl Store {
    /// Service one equation in a component visible to a live source matcher.
    /// Recompute after each step: merges can change incidence and queued work.
    /// Remaining unrelated equations still require fair service and consistency
    /// before publication; false is not an assertion of complete quiescence.
    pub fn step_for_matching(&mut self, reads: &MatcherReads) -> bool {
        #[cfg(feature = "execution-profile")]
        let _scope =
            crate::deduction_profile::Scope::new(crate::deduction_profile::Phase::Readiness);
        if self.equations.is_empty() {
            return false;
        }
        let mut todo = VecDeque::new();
        for (key, index) in self.view.locations.values() {
            if let Some(columns) = reads.columns.get(key) {
                let row = &self.view.tables[key].rows[*index];
                todo.extend(columns.iter().map(|column| self.root(row.values[*column])));
            }
        }
        if todo.is_empty() {
            return false;
        }
        let mut edges: BTreeMap<Value, Vec<Value>> = BTreeMap::new();
        for (a, b) in &self.equations {
            let (a, b) = (self.root(*a), self.root(*b));
            edges.entry(a).or_default().push(b);
            edges.entry(b).or_default().push(a);
        }
        let mut relevant = BTreeSet::new();
        while let Some(value) = todo.pop_front() {
            if !relevant.insert(value) {
                continue;
            }
            if let Some(neighbors) = edges.get(&value) {
                todo.extend(neighbors);
            }
            // Both child and parent incidence matter: changing a child can
            // trigger congruence repair of an observed parent constructor.
            if let Some(incidents) = self.view.incidence.get(&value) {
                for (key, index) in incidents {
                    if matches!(key, Relation::Constructor(..)) {
                        todo.extend(
                            self.view.tables[key].rows[*index]
                                .values
                                .iter()
                                .map(|v| self.root(*v)),
                        );
                    }
                }
            }
        }
        let Some(index) = self.equations.iter().position(|(a, b)| {
            relevant.contains(&self.root(*a)) || relevant.contains(&self.root(*b))
        }) else {
            return false;
        };
        let equation = self.equations.remove(index).unwrap();
        self.equations.push_front(equation);
        self.step()
    }
}
