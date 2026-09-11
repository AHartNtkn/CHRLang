//! Reuse pending bodies while matching, heads and histories stay in the live cursor.
use super::{
    Fresh, rename,
    trace::{Event, Job, Table},
};
use crate::continuations::Batch;
use chr_persistent::continuations::{Cursor, Machine, PreparedMachine, Step};
use chr_syntax::{Constraint, Goal, Query, Rule, Term, Var};
use std::{
    collections::{BTreeMap, VecDeque},
    rc::Rc,
};
pub struct Caller {
    program: PreparedMachine,
    keys: BTreeMap<Goal, usize>,
    bodies: Vec<(Table, String)>,
    owner: Rc<()>,
}
type ActiveBody = (usize, Job, Fresh);
pub struct Run {
    machine: Machine,
    frontier: VecDeque<(Cursor, Option<ActiveBody>)>,
    owner: Rc<()>,
}
fn canonical(body: &Goal) -> (Goal, Vec<Var>) {
    fn walk(g: &Goal, map: &mut impl FnMut(Var) -> Var) -> Goal {
        match g {
            Goal::Constraint(c) => Goal::Constraint(Constraint {
                name: c.name.clone(),
                args: c.args.iter().map(|t| rename(t, map)).collect(),
            }),
            Goal::Unify(a, b) => Goal::Unify(rename(a, map), rename(b, map)),
            Goal::And(gs) => Goal::And(gs.iter().map(|g| walk(g, map)).collect()),
            Goal::Or(a, b) => Goal::Or(Box::new(walk(a, map)), Box::new(walk(b, map))),
            Goal::True => Goal::True,
            Goal::Fail => Goal::Fail,
        }
    }
    let mut mapping = BTreeMap::new();
    let mut originals = vec![];
    let body = walk(body, &mut |v| {
        *mapping.entry(v).or_insert_with(|| {
            let id = Var(originals.len() as u64);
            originals.push(v);
            id
        })
    });
    (body, originals)
}
impl Caller {
    pub fn new(rules: Vec<Rule>) -> Result<Self, String> {
        Ok(Self {
            program: PreparedMachine::new(rules)?,
            keys: BTreeMap::new(),
            bodies: vec![],
            owner: Rc::new(()),
        })
    }
    pub fn start(&self, query: Query) -> Result<Run, String> {
        let (machine, cursor) = self.program.start(query)?;
        Ok(Run {
            machine,
            frontier: VecDeque::from([(cursor, None)]),
            owner: self.owner.clone(),
        })
    }
    pub fn executed(&self) -> usize {
        self.bodies.iter().map(|(t, _)| t.stats().executed).sum()
    }
    pub fn replayed(&self) -> usize {
        self.bodies.iter().map(|(t, _)| t.stats().replayed).sum()
    }
    fn attach(&mut self, body: Goal, next: u64) -> Result<ActiveBody, String> {
        let (key, originals) = canonical(&body);
        let id = if let Some(id) = self.keys.get(&key) {
            *id
        } else {
            let id = self.bodies.len();
            self.bodies.push(Table::body(key.clone(), originals.len())?);
            self.keys.insert(key, id);
            id
        };
        let (table, name) = &mut self.bodies[id];
        let job = table.start_body(&Constraint {
            name: name.clone(),
            args: originals.into_iter().map(Term::Var).collect(),
        })?;
        Ok((id, job, Fresh::from_next(next)))
    }
    pub fn advance(&mut self, run: &mut Run, budget: usize) -> Result<Batch, String> {
        if !Rc::ptr_eq(&self.owner, &run.owner) {
            return Err("run belongs to another body caller".into());
        }
        let mut answers = vec![];
        for _ in 0..budget {
            let Some((cursor, active)) = run.frontier.pop_front() else {
                break;
            };
            let (step, selected) = if let Some((id, job, mut fresh)) = active {
                match self.bodies[id].0.step(job, &mut fresh)? {
                    Event::Continue(job) => {
                        run.frontier.push_back((cursor, Some((id, job, fresh))));
                        continue;
                    }
                    Event::Split(a, b) => {
                        run.frontier
                            .push_back((cursor.clone(), Some((id, a, fresh.clone()))));
                        run.frontier.push_back((cursor, Some((id, b, fresh))));
                        continue;
                    }
                    Event::Failed => (Step::Failed, false),
                    Event::Answer(bindings) => {
                        (run.machine.resume_private(cursor, bindings, vec![]), true)
                    }
                    Event::Suspended(bindings, residual) => {
                        (run.machine.resume_private(cursor, bindings, residual), true)
                    }
                }
            } else {
                let selected = !run.machine.has_pending_work(&cursor);
                (run.machine.step(cursor), selected)
            };
            match step {
                Step::Continue(mut cursor) => {
                    let active = if selected {
                        let (body, next) = run.machine.take_body(&mut cursor);
                        Some(self.attach(body, next)?)
                    } else {
                        None
                    };
                    run.frontier.push_back((cursor, active));
                }
                Step::Split(a, b) => {
                    run.frontier.push_back((a, None));
                    run.frontier.push_back((b, None));
                }
                Step::Failed => (),
                Step::Answer(a) => answers.push(a),
            }
        }
        Ok(Batch {
            answers,
            exhausted: run.frontier.is_empty(),
        })
    }
}
