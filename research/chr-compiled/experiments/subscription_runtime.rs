//! Exact-source runtime for the S01 subscription comparison.
use super::{
    join::{Join, Mode, Stats},
    source,
};
use chr_syntax::{Answer, Constraint, Query, Rule, Term, Var, atom, c, t};
use std::collections::{BTreeMap, VecDeque};
type Bindings = BTreeMap<Var, Term>;
fn resolve(x: &Term, b: &Bindings) -> Term {
    match x {
        Term::Var(v) => b.get(v).map_or_else(|| x.clone(), |x| resolve(x, b)),
        Term::App(n, xs) => t(n, xs.iter().map(|x| resolve(x, b)).collect::<Vec<_>>()),
    }
}
fn occurs(v: Var, x: &Term) -> bool {
    match x {
        Term::Var(w) => v == *w,
        Term::App(_, xs) => xs.iter().any(|x| occurs(v, x)),
    }
}
fn unify(a: Term, b: Term, bindings: &mut Bindings) -> bool {
    let mut pending = vec![(a, b)];
    while let Some((a, b)) = pending.pop() {
        let a = resolve(&a, bindings);
        let b = resolve(&b, bindings);
        if a == b {
            continue;
        }
        match (a, b) {
            (Term::Var(v), x) | (x, Term::Var(v)) => {
                if occurs(v, &x) {
                    return false;
                }
                bindings.insert(v, x);
            }
            (Term::App(n, xs), Term::App(m, ys)) => {
                if n != m || xs.len() != ys.len() {
                    return false;
                }
                pending.extend(xs.into_iter().zip(ys));
            }
        }
    }
    true
}
fn projected(relation: usize, row: &Constraint, b: &Bindings) -> Option<(Term, Term)> {
    let payload = resolve(&row.args[1], b);
    match payload {
        Term::App(n, xs) if n == ["f", "g", "h"][relation] && xs.len() == 1 => {
            Some((resolve(&row.args[0], b), xs[0].clone()))
        }
        _ => None,
    }
}
pub struct Prepared {
    consuming: bool,
}
impl Prepared {
    pub fn new(rules: &[Rule]) -> Result<Self, String> {
        for consuming in [false, true] {
            if rules == source::source_rules(consuming) {
                return Ok(Self { consuming });
            }
        }
        Err("unsupported subscription source".into())
    }
    pub fn start(&self, q: Query, mode: Mode) -> Result<Engine, String> {
        Engine::new(q, mode, self.consuming)
    }
}
struct Pulse {
    round: Term,
    candidates: VecDeque<[usize; 4]>,
}
pub struct Engine {
    join: Join,
    consuming: bool,
    rows: [BTreeMap<usize, Constraint>; 3],
    demands: BTreeMap<usize, Constraint>,
    passive: Vec<Constraint>,
    next: usize,
    bindings: Bindings,
    source_script: Term,
    ops: Vec<Term>,
    position: usize,
    pulse: Option<Pulse>,
    outputs: Vec<(String, Var)>,
    done: bool,
    blocked: bool,
    failed: bool,
}
impl Engine {
    fn new(q: Query, mode: Mode, consuming: bool) -> Result<Self, String> {
        let mut names = std::collections::BTreeSet::new();
        if q.outputs.iter().any(|(n, _)| !names.insert(n.clone())) {
            return Err("duplicate output name".into());
        }
        let mut this = Self {
            join: Join::new(mode),
            consuming,
            rows: std::array::from_fn(|_| BTreeMap::new()),
            demands: BTreeMap::new(),
            passive: vec![],
            next: 0,
            bindings: BTreeMap::new(),
            source_script: atom("nil"),
            ops: vec![],
            position: 0,
            pulse: None,
            outputs: q.outputs,
            done: false,
            blocked: false,
            failed: false,
        };
        let mut script = None;
        for row in q.constraints {
            match (row.name.as_str(), row.args.len()) {
                ("drive", 1) if script.is_none() => {
                    script = Some(row.args[0].clone());
                }
                ("left" | "middle" | "right", 2) => {
                    let rel = ["left", "middle", "right"]
                        .iter()
                        .position(|n| *n == row.name)
                        .unwrap();
                    this.insert_row(rel, row);
                }
                ("demand", 3) => {
                    let id = this.next;
                    this.next += 1;
                    this.demands.insert(id, row);
                }
                ("receipt" | "done", _) => this.passive.push(row),
                _ => return Err("unsupported query constraint or duplicate driver".into()),
            }
        }
        this.source_script = script.ok_or("one driver required")?;
        let mut tail = &this.source_script;
        while *tail != atom("nil") {
            match tail {
                Term::App(n, xs) if n == "cons" && xs.len() == 2 => {
                    this.ops.push(xs[0].clone());
                    tail = &xs[1];
                }
                _ => return Err("closed instruction spine required".into()),
            }
        }
        for (&id, row) in &this.demands {
            this.join
                .open(id, (row.args[0].clone(), row.args[1].clone()));
        }
        Ok(this)
    }
    fn insert_row(&mut self, relation: usize, row: Constraint) {
        let id = self.next;
        self.next += 1;
        if let Some(key) = projected(relation, &row, &self.bindings) {
            self.join.insert(relation, id, key);
        }
        self.rows[relation].insert(id, row);
    }
    fn remove_row(&mut self, relation: usize, id: usize) {
        if projected(relation, &self.rows[relation][&id], &self.bindings).is_some() {
            self.join.remove(relation, id);
        }
        self.rows[relation].remove(&id);
    }
    /// Binding transitions update only changed projections; the scan itself is charged.
    fn bind(&mut self, a: Term, b: Term) {
        let old = self.bindings.clone();
        if !unify(a, b, &mut self.bindings) {
            self.failed = true;
            self.done = true;
            return;
        }
        let changed_demands: Vec<_> = self
            .demands
            .iter()
            .filter_map(|(&id, row)| {
                let before = (resolve(&row.args[0], &old), resolve(&row.args[1], &old));
                let after = (
                    resolve(&row.args[0], &self.bindings),
                    resolve(&row.args[1], &self.bindings),
                );
                (before != after).then_some((id, after))
            })
            .collect();
        for (id, _) in &changed_demands {
            self.join.close(*id);
        }
        for rel in 0..3 {
            let changed: Vec<_> = self.rows[rel]
                .iter()
                .filter_map(|(&id, row)| {
                    let before = projected(rel, row, &old);
                    let after = projected(rel, row, &self.bindings);
                    (before != after).then_some((id, before, after))
                })
                .collect();
            for (id, before, _) in &changed {
                if before.is_some() {
                    self.join.remove(rel, *id);
                }
            }
            for (id, _, after) in changed {
                if let Some(key) = after {
                    self.join.insert(rel, id, key);
                }
            }
        }
        for (id, key) in changed_demands {
            self.join.open(id, key);
        }
    }
    fn begin_pulse(&mut self, id: Term, round: Term) {
        let demands: Vec<_> = self
            .demands
            .iter()
            .filter_map(|(&i, d)| (resolve(&d.args[2], &self.bindings) == id).then_some(i))
            .collect();
        let mut candidates = Vec::new();
        for d in demands {
            for [l, m, r] in self.join.request(d) {
                candidates.push([l, m, r, d]);
            }
        }
        if self.consuming {
            candidates.sort_unstable_by_key(|[l, m, r, d]| [*l, *m, *d, *r]);
        } else {
            candidates.sort_unstable();
        }
        self.pulse = Some(Pulse {
            round,
            candidates: candidates.into(),
        });
    }
    fn step(&mut self) {
        if let Some(mut pulse) = self.pulse.take() {
            if let Some([l, m, r, d]) = pulse.candidates.pop_front() {
                if self.rows[2].contains_key(&r) {
                    let round = pulse.round.clone();
                    let receipt = c(
                        "receipt",
                        [
                            self.demands[&d].args[2].clone(),
                            round,
                            self.rows[0][&l].args[1].clone(),
                            self.rows[1][&m].args[1].clone(),
                            self.rows[2][&r].args[1].clone(),
                        ],
                    );
                    self.passive.push(receipt);
                    if self.consuming {
                        self.remove_row(2, r);
                    }
                }
                self.pulse = Some(pulse);
            }
            return;
        }
        if self.position == self.ops.len() {
            self.done = true;
            return;
        }
        let op = resolve(&self.ops[self.position], &self.bindings);
        let Term::App(n, args) = op else {
            self.blocked = true;
            self.done = true;
            return;
        };
        match (n.as_str(), args.as_slice()) {
            ("open", [key, value, name]) => {
                let id = self.next;
                self.next += 1;
                self.join.open(id, (key.clone(), value.clone()));
                self.demands
                    .insert(id, c("demand", [key.clone(), value.clone(), name.clone()]));
            }
            ("close", [name]) => {
                let id = self.demands.iter().find_map(|(&i, d)| {
                    (resolve(&d.args[2], &self.bindings) == *name).then_some(i)
                });
                if let Some(id) = id {
                    self.join.close(id);
                    self.demands.remove(&id);
                } else {
                    self.blocked = true;
                    self.done = true;
                    return;
                }
            }
            ("ask", [id, round]) => self.begin_pulse(id.clone(), round.clone()),
            ("bind", [a, b]) => self.bind(a.clone(), b.clone()),
            (name, [a, b]) if name.starts_with("insert_") || name.starts_with("remove_") => {
                let relation = name.split_once('_').unwrap().1;
                let Some(rel) = ["left", "middle", "right"]
                    .iter()
                    .position(|n| *n == relation)
                else {
                    self.blocked = true;
                    self.done = true;
                    return;
                };
                if name.starts_with("insert_") {
                    self.insert_row(rel, c(relation, [a.clone(), b.clone()]));
                } else {
                    let id = self.rows[rel].iter().find_map(|(&i, row)| {
                        (resolve(&row.args[0], &self.bindings) == *a
                            && resolve(&row.args[1], &self.bindings) == *b)
                            .then_some(i)
                    });
                    if let Some(id) = id {
                        self.remove_row(rel, id);
                    } else {
                        self.blocked = true;
                        self.done = true;
                        return;
                    }
                }
            }
            _ => {
                self.blocked = true;
                self.done = true;
                return;
            }
        }
        self.position += 1;
    }
    pub fn advance(&mut self, budget: usize) -> bool {
        for _ in 0..budget {
            if self.done {
                break;
            }
            self.step();
        }
        self.done
    }
    pub fn observe(&self) -> Result<Vec<Answer>, String> {
        if !self.done {
            return Err("source work pending".into());
        }
        if self.failed {
            return Ok(vec![]);
        }
        let mut residual: Vec<_> = self
            .rows
            .iter()
            .flat_map(|rows| rows.values().cloned())
            .chain(self.demands.values().cloned())
            .chain(self.passive.iter().cloned())
            .collect();
        if self.blocked {
            let mut tail = &self.source_script;
            for _ in 0..self.position {
                let Term::App(_, xs) = tail else {
                    unreachable!()
                };
                tail = &xs[1];
            }
            residual.push(c("drive", [tail.clone()]));
        } else {
            residual.push(c("done", []));
        }
        for row in &mut residual {
            for arg in &mut row.args {
                *arg = resolve(arg, &self.bindings);
            }
        }
        Ok(vec![Answer {
            outputs: self
                .outputs
                .iter()
                .map(|(n, v)| (n.clone(), resolve(&Term::Var(*v), &self.bindings)))
                .collect(),
            residual,
        }])
    }
    pub fn stats(&self) -> &Stats {
        self.join.stats()
    }
    pub fn retained(&self) -> usize {
        self.join.retained()
    }
}
