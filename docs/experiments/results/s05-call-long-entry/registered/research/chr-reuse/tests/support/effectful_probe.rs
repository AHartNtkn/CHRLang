//! Deliberately unchecked contraction hypotheses, not a source optimizer.
//! Caller scheduling, occurrence order and propagation history are NOT certified.
use chr_persistent::continuations::{PreparedMachine, Step};
use chr_syntax::{Answer, Constraint, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
#[derive(Clone, Copy, Debug)]
pub enum Key {
    None,
    Call,
    Region,
}
pub struct Probe {
    prepared: PreparedMachine,
    names: BTreeSet<String>,
    mode: Key,
    cache: BTreeMap<(Vec<Constraint>, usize), Vec<Answer>>,
    pub phase_budget: usize,
    pub computed: usize,
    pub hits: usize,
}
fn rename(t: &Term, f: &mut impl FnMut(Var) -> Var) -> Term {
    match t {
        Term::Var(v) => Term::Var(f(*v)),
        Term::App(n, x) => Term::App(n.clone(), x.iter().map(|t| rename(t, f)).collect()),
    }
}
fn constraint(c: &Constraint, f: &mut impl FnMut(Var) -> Var) -> Constraint {
    Constraint {
        name: c.name.clone(),
        args: c.args.iter().map(|t| rename(t, f)).collect(),
    }
}
fn collect(
    mut machine: chr_persistent::continuations::Machine,
    cursor: chr_persistent::continuations::Cursor,
    bound: usize,
) -> Result<Vec<Answer>, String> {
    let mut pending = VecDeque::from([cursor]);
    let mut answers = vec![];
    for _ in 0..bound {
        let Some(cursor) = pending.pop_front() else {
            return Ok(answers);
        };
        match machine.step(cursor) {
            Step::Continue(c) => pending.push_back(c),
            Step::Split(a, b) => {
                pending.push_back(a);
                pending.push_back(b);
            }
            Step::Failed => (),
            Step::Answer(a) => answers.push(a),
        }
    }
    Err("source probe exceeded service bound".into())
}
pub fn direct(rules: &[Rule], q: &Query) -> Vec<Answer> {
    let p = PreparedMachine::new(rules.to_vec()).unwrap();
    let (m, c) = p.start(q.clone()).unwrap();
    collect(m, c, 200_000).unwrap()
}
impl Probe {
    pub fn new(rules: Vec<Rule>, names: Vec<&str>, mode: Key) -> Self {
        Self {
            prepared: PreparedMachine::new(rules).unwrap(),
            names: names.into_iter().map(str::to_owned).collect(),
            mode,
            cache: BTreeMap::new(),
            phase_budget: 200_000,
            computed: 0,
            hits: 0,
        }
    }
    pub fn run(&mut self, rules: &[Rule], q: &Query) -> Vec<Answer> {
        self.try_run(rules, q).unwrap()
    }
    pub fn try_run(&mut self, rules: &[Rule], q: &Query) -> Result<Vec<Answer>, String> {
        let mut slots = BTreeMap::new();
        let mut originals = vec![];
        let mut intern = |v| {
            *slots.entry(v).or_insert_with(|| {
                let slot = Var(originals.len() as u64);
                originals.push(v);
                slot
            })
        };
        let projected = q
            .constraints
            .iter()
            .filter(|c| self.names.contains(&c.name))
            .map(|c| constraint(c, &mut intern))
            .collect::<Vec<_>>();
        // Export every caller variable; values on this interface reconnect outside aliases.
        for c in &q.constraints {
            constraint(c, &mut intern);
        }
        for (_, v) in &q.outputs {
            intern(*v);
        }
        let key = (
            if matches!(self.mode, Key::Call) {
                projected
                    .iter()
                    .filter(|c| c.name == "work")
                    .cloned()
                    .collect()
            } else {
                projected.clone()
            },
            originals.len(),
        );
        let answers = if let Some(a) = self
            .cache
            .get(&key)
            .filter(|_| !matches!(self.mode, Key::None))
        {
            self.hits += 1;
            a.clone()
        } else {
            self.computed += 1;
            let phase = Query {
                constraints: projected,
                outputs: (0..originals.len())
                    .map(|i| (format!("slot{i:08}"), Var(i as u64)))
                    .collect(),
            };
            let (m, c) = self.prepared.start(phase).unwrap();
            let a = collect(m, c, self.phase_budget)?;
            if !matches!(self.mode, Key::None) {
                self.cache.insert(key, a.clone());
            }
            a
        };
        let caller = PreparedMachine::new(rules.to_vec()).unwrap();
        let mut result = vec![];
        let mut next = originals
            .iter()
            .map(|v| v.0)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .unwrap();
        for answer in answers {
            // All exported variable labels are local to this answer. Import outputs
            // and residuals through one fresh scope; equations reconnect the interface.
            let mut imported = BTreeMap::new();
            let mut fresh = |v| {
                *imported.entry(v).or_insert_with(|| {
                    let v = Var(next);
                    next = next.checked_add(1).unwrap();
                    v
                })
            };
            let bindings = answer
                .outputs
                .iter()
                .enumerate()
                .map(|(i, (name, t))| {
                    assert_eq!(*name, format!("slot{i:08}"));
                    (originals[i], rename(t, &mut fresh))
                })
                .collect();
            let mut constraints = q
                .constraints
                .iter()
                .filter(|c| !self.names.contains(&c.name))
                .cloned()
                .collect::<Vec<_>>();
            constraints.extend(answer.residual.iter().map(|c| constraint(c, &mut fresh)));
            let resumed = Query {
                constraints,
                outputs: q.outputs.clone(),
            };
            let (m, c) = caller.start_replaying(resumed, bindings).unwrap();
            result.extend(collect(m, c, 200_000)?);
        }
        Ok(result)
    }
}
