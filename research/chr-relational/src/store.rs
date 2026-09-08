//! One equality interpretation owns both relation facts and source resources.
//! Forking clones ownership. This is not shared contextual equality.
use crate::{Evaluation, HeadPlan, Match, Occurrence, Relation, Value, View};
use chr_syntax::{Term, Var};
use std::collections::{BTreeSet, VecDeque};
#[derive(Default, Clone)]
pub struct Store {
    view: View,
    parents: Vec<Value>,
    ranks: Vec<usize>,
    equations: VecDeque<(Value, Value)>,
    queued: BTreeSet<(Value, Value)>,
    failed: bool,
    next_occurrence: usize,
}
impl Store {
    pub fn unknown(&mut self) -> Value {
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
        let children = children.iter().map(|v| self.root(*v)).collect::<Vec<_>>();
        if let Some(value) = self.view.peers(name, &children).first() {
            return self.root(*value);
        }
        let value = self.unknown();
        self.view.constructor(name, value, &children);
        value
    }
    pub fn post(&mut self, name: &str, args: &[Value]) -> Occurrence {
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
        let mut todo = vec![from];
        let mut seen = BTreeSet::new();
        while let Some(value) = todo.pop() {
            if !seen.insert(value) {
                continue;
            }
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
