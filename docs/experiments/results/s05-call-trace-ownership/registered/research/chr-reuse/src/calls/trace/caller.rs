//! Private-prefix splicing into the original live caller cursor.
use super::{Event, Job, Table};
use crate::{
    calls::{Fresh, checked_family},
    continuations::Batch,
};
use chr_persistent::continuations::{Cursor, Machine, PreparedMachine, Step};
use chr_syntax::{Query, Rule};
use std::{collections::VecDeque, rc::Rc};
pub struct Caller {
    program: PreparedMachine,
    table: Table,
    count: usize,
    owner: Rc<()>,
}
pub struct Run {
    machine: Machine,
    frontier: VecDeque<(Cursor, Option<(Job, Fresh)>)>,
    owner: Rc<()>,
}
impl Caller {
    pub fn new(rules: Vec<Rule>, count: usize) -> Result<Self, String> {
        let private = rules.get(..count).ok_or("invalid private prefix")?.to_vec();
        let family = checked_family(&private)?;
        if rules[count..].iter().any(|r| {
            r.kept
                .iter()
                .chain(&r.removed)
                .any(|c| family.contains(&(c.name.clone(), c.args.len())))
        }) {
            return Err("caller reads private family".into());
        }
        Ok(Self {
            program: PreparedMachine::new(rules)?,
            table: Table::new(private)?,
            count,
            owner: Rc::new(()),
        })
    }
    pub fn start(&self, q: Query) -> Result<Run, String> {
        let (machine, cursor) = self.program.start(q)?;
        Ok(Run {
            machine,
            frontier: VecDeque::from([(cursor, None)]),
            owner: self.owner.clone(),
        })
    }
    pub fn stats(&self) -> &super::Stats {
        self.table.stats()
    }
    pub fn advance(&mut self, run: &mut Run, budget: usize) -> Result<Batch, String> {
        if !Rc::ptr_eq(&run.owner, &self.owner) {
            return Err("run belongs to another caller".into());
        }
        let mut answers = vec![];
        for _ in 0..budget {
            let Some((mut cursor, mut active)) = run.frontier.pop_front() else {
                break;
            };
            if active.is_none()
                && let Some((call, next)) = run.machine.take_private_call(&mut cursor, self.count)
            {
                active = Some((self.table.start_live(&call)?, Fresh::from_next(next)));
            }
            let step = if let Some((job, mut fresh)) = active {
                match self.table.step(job, &mut fresh)? {
                    Event::Continue(job) => {
                        run.frontier.push_back((cursor, Some((job, fresh))));
                        continue;
                    }
                    Event::Split(a, b) => {
                        run.frontier
                            .push_back((cursor.clone(), Some((a, fresh.clone()))));
                        run.frontier.push_back((cursor, Some((b, fresh))));
                        continue;
                    }
                    Event::Failed => Step::Failed,
                    Event::Answer(bindings) => run.machine.resume_private(cursor, bindings, vec![]),
                    Event::Suspended(bindings, residual) => {
                        run.machine.resume_private(cursor, bindings, residual)
                    }
                }
            } else {
                run.machine.step(cursor)
            };
            match step {
                Step::Continue(c) => run.frontier.push_back((c, None)),
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
