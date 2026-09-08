//! Independent successful source replay: owned substitutions, no candidate or reference kernel.
use chr_syntax::{Answer, Constraint, Goal, Guard, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
pub struct Replay {
    store: BTreeMap<u64, Constraint>,
    bindings: BTreeMap<Var, Term>,
    history: BTreeSet<(usize, Vec<u64>)>,
    outputs: Vec<(String, Var)>,
    choices: std::collections::VecDeque<bool>,
    next_occ: u64,
    next_var: u64,
}
fn vars(t: &Term, next: &mut u64) {
    match t {
        Term::Var(Var(v)) => *next = (*next).max(v + 1),
        Term::App(_, xs) => xs.iter().for_each(|x| vars(x, next)),
    }
}
fn substitute(t: &Term, bindings: &BTreeMap<Var, Term>) -> Term {
    match t {
        Term::Var(v) => bindings
            .get(v)
            .map_or_else(|| t.clone(), |x| substitute(x, bindings)),
        Term::App(n, xs) => Term::App(
            n.clone(),
            xs.iter().map(|x| substitute(x, bindings)).collect(),
        ),
    }
}
fn occurs(v: Var, t: &Term) -> bool {
    match t {
        Term::Var(x) => *x == v,
        Term::App(_, xs) => xs.iter().any(|x| occurs(v, x)),
    }
}
fn pattern(p: &Term, value: &Term, frame: &mut BTreeMap<Var, Term>) -> bool {
    match p {
        Term::Var(v) => match frame.get(v) {
            Some(old) => old == value,
            None => {
                frame.insert(*v, value.clone());
                true
            }
        },
        Term::App(n, ps) => match value {
            Term::App(m, vs) => {
                n == m
                    && ps.len() == vs.len()
                    && ps.iter().zip(vs).all(|(p, v)| pattern(p, v, frame))
            }
            Term::Var(_) => false,
        },
    }
}
fn instantiate(t: &Term, frame: &mut BTreeMap<Var, Term>, next: &mut u64) -> Term {
    match t {
        Term::Var(v) => frame
            .entry(*v)
            .or_insert_with(|| {
                let id = *next;
                *next += 1;
                Term::Var(Var(id))
            })
            .clone(),
        Term::App(n, xs) => Term::App(
            n.clone(),
            xs.iter().map(|x| instantiate(x, frame, next)).collect(),
        ),
    }
}
impl Replay {
    pub fn new(query: &Query, choices: &[bool]) -> Self {
        let mut next_var = 0;
        for c in &query.constraints {
            for t in &c.args {
                vars(t, &mut next_var);
            }
        }
        for (_, v) in &query.outputs {
            next_var = next_var.max(v.0 + 1);
        }
        Self {
            store: query
                .constraints
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, c)| (i as u64, c))
                .collect(),
            choices: choices.iter().copied().collect(),
            bindings: BTreeMap::new(),
            history: BTreeSet::new(),
            outputs: query.outputs.clone(),
            next_occ: query.constraints.len() as u64,
            next_var,
        }
    }
    fn unify(&mut self, a: Term, b: Term) -> Result<(), String> {
        let mut equations = vec![(a, b)];
        while let Some((a, b)) = equations.pop() {
            let a = substitute(&a, &self.bindings);
            let b = substitute(&b, &self.bindings);
            if a == b {
                continue;
            }
            match (a, b) {
                (Term::Var(v), t) | (t, Term::Var(v)) => {
                    if occurs(v, &t) {
                        return Err("finite-tree cycle".into());
                    }
                    self.bindings.insert(v, t);
                }
                (Term::App(n, xs), Term::App(m, ys)) => {
                    if n != m || xs.len() != ys.len() {
                        return Err("constructor clash".into());
                    }
                    equations.extend(xs.into_iter().zip(ys));
                }
            }
        }
        Ok(())
    }
    fn frame(
        &self,
        rule_id: usize,
        rule: &Rule,
        ids: &[u64],
    ) -> Option<(BTreeMap<Var, Term>, u64)> {
        if self.history.contains(&(rule_id, ids.to_vec()))
            || ids.iter().collect::<BTreeSet<_>>().len() != ids.len()
        {
            return None;
        }
        let heads = rule.kept.iter().chain(&rule.removed).collect::<Vec<_>>();
        if heads.len() != ids.len() {
            return None;
        }
        let mut frame = BTreeMap::new();
        for (head, id) in heads.iter().zip(ids) {
            let c = self.store.get(id)?;
            if head.name != c.name || head.args.len() != c.args.len() {
                return None;
            }
            for (p, v) in head.args.iter().zip(&c.args) {
                if !pattern(p, &substitute(v, &self.bindings), &mut frame) {
                    return None;
                }
            }
        }
        let mut next = self.next_var;
        for Guard::Equal(a, b) in &rule.guards {
            if instantiate(a, &mut frame, &mut next) != instantiate(b, &mut frame, &mut next) {
                return None;
            }
        }
        Some((frame, next))
    }
    pub fn fire(&mut self, rule_id: usize, rule: &Rule, ids: &[u64]) -> Result<(), String> {
        let (mut frame, next) = self
            .frame(rule_id, rule, ids)
            .ok_or("trace application is not enabled")?;
        self.next_var = next;
        for id in &ids[rule.kept.len()..] {
            self.store.remove(id).unwrap();
        }
        if rule.removed.is_empty() {
            self.history.insert((rule_id, ids.to_vec()));
        }
        self.body(&rule.body, &mut frame)
    }
    fn body(&mut self, goal: &Goal, frame: &mut BTreeMap<Var, Term>) -> Result<(), String> {
        match goal {
            Goal::Constraint(c) => {
                let c = Constraint {
                    name: c.name.clone(),
                    args: c
                        .args
                        .iter()
                        .map(|t| instantiate(t, frame, &mut self.next_var))
                        .collect(),
                };
                self.store.insert(self.next_occ, c);
                self.next_occ += 1;
            }
            Goal::Unify(a, b) => {
                let a = instantiate(a, frame, &mut self.next_var);
                let b = instantiate(b, frame, &mut self.next_var);
                self.unify(a, b)?;
            }
            Goal::And(gs) => {
                for g in gs {
                    self.body(g, frame)?;
                }
            }
            Goal::True => (),
            Goal::Fail => return Err("explicit source failure".into()),
            Goal::Or(left, right) => {
                let right_arm = self.choices.pop_front().ok_or("missing source choice")?;
                self.body(if right_arm { right } else { left }, frame)?;
            }
        }
        Ok(())
    }
    pub fn choices_consumed(&self) -> bool {
        self.choices.is_empty()
    }
    pub fn terminal(&self, rules: &[Rule]) -> bool {
        fn walk(replay: &Replay, r: usize, rule: &Rule, ids: &mut Vec<u64>) -> bool {
            if ids.len() == rule.kept.len() + rule.removed.len() {
                return replay.frame(r, rule, ids).is_some();
            }
            for id in replay.store.keys() {
                if ids.contains(id) {
                    continue;
                }
                ids.push(*id);
                if walk(replay, r, rule, ids) {
                    return true;
                }
                ids.pop();
            }
            false
        }
        !rules
            .iter()
            .enumerate()
            .any(|(r, rule)| walk(self, r, rule, &mut vec![]))
    }
    pub fn live_ids(&self) -> Vec<u64> {
        self.store.keys().copied().collect()
    }
    pub fn history(&self) -> Vec<(usize, Vec<u64>)> {
        self.history.iter().cloned().collect()
    }
    pub fn answer(&self) -> Answer {
        Answer {
            outputs: self
                .outputs
                .iter()
                .map(|(n, v)| (n.clone(), substitute(&Term::Var(*v), &self.bindings)))
                .collect(),
            residual: self
                .store
                .values()
                .map(|c| Constraint {
                    name: c.name.clone(),
                    args: c
                        .args
                        .iter()
                        .map(|t| substitute(t, &self.bindings))
                        .collect(),
                })
                .collect(),
        }
    }
}
