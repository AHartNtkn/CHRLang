//! Resumable isolated calls. A caller must establish that the private call owns
//! its computation; this API does not extract calls from arbitrary live states.
use super::{Fresh, canonical, checked_family, rename};
use chr_persistent::continuations::{Cursor, Machine, PreparedMachine, Step};
use chr_syntax::{Constraint, Query, Rule, Term, Var};
use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};
#[derive(Clone)]
enum End {
    Split(usize, usize),
    Failed,
    Answer(Vec<Term>),
    Invalid,
}
struct Node {
    progress: usize,
    cursor: Option<Cursor>,
    end: Option<End>,
}
struct Trace {
    machine: Option<Machine>,
    unfinished: usize,
    nodes: Vec<Node>,
}
#[derive(Default, Debug)]
pub struct Stats {
    pub lookups: usize,
    pub hits: usize,
    pub executed: usize,
    pub replayed: usize,
}
pub struct Table {
    prepared: PreparedMachine,
    family: BTreeSet<(String, usize)>,
    keys: BTreeMap<Constraint, usize>,
    traces: Vec<Trace>,
    owner: Rc<()>,
    stats: Stats,
}
pub struct Job {
    trace: usize,
    node: usize,
    position: usize,
    originals: Vec<Var>,
    owner: Rc<()>,
}
pub enum Event {
    Continue(Job),
    Split(Job, Job),
    Failed,
    Answer(Vec<(Var, Term)>),
}
impl Table {
    pub fn new(rules: Vec<Rule>) -> Result<Self, String> {
        let family = checked_family(&rules)?;
        Ok(Self {
            prepared: PreparedMachine::new(rules)?,
            family,
            keys: BTreeMap::new(),
            traces: vec![],
            owner: Rc::new(()),
            stats: Stats::default(),
        })
    }
    pub fn start(&mut self, call: &Constraint) -> Result<Job, String> {
        if !self.family.contains(&(call.name.clone(), call.args.len())) {
            return Err("call outside private family".into());
        }
        let (key, originals) = canonical(call);
        if cfg!(feature = "metrics") {
            self.stats.lookups += 1;
        }
        let trace = if let Some(i) = self.keys.get(&key) {
            if cfg!(feature = "metrics") {
                self.stats.hits += 1;
            }
            *i
        } else {
            let (machine, cursor) = self.prepared.start(Query {
                constraints: vec![key.clone()],
                outputs: (0..originals.len())
                    .map(|i| (format!("arg{i}"), Var(i as u64)))
                    .collect(),
            })?;
            let id = self.traces.len();
            self.traces.push(Trace {
                machine: Some(machine),
                unfinished: 1,
                nodes: vec![Node {
                    progress: 0,
                    cursor: Some(cursor),
                    end: None,
                }],
            });
            self.keys.insert(key, id);
            id
        };
        Ok(Job {
            trace,
            node: 0,
            position: 0,
            originals,
            owner: self.owner.clone(),
        })
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn unfinished_calls(&self) -> usize {
        self.traces
            .iter()
            .filter(|trace| trace.machine.is_some())
            .count()
    }
    pub fn retained_nodes(&self) -> usize {
        self.traces.iter().map(|t| t.nodes.len()).sum()
    }
    /// `fresh` must cover all live caller variables, including other jobs.
    pub fn step(&mut self, mut job: Job, fresh: &mut Fresh) -> Result<Event, String> {
        if !Rc::ptr_eq(&self.owner, &job.owner) {
            return Err("job belongs to another call table".into());
        }
        let trace = &mut self.traces[job.trace];
        let node = &trace.nodes[job.node];
        if job.position < node.progress {
            if cfg!(feature = "metrics") {
                self.stats.replayed += 1;
            }
            job.position += 1;
            return Ok(Event::Continue(job));
        }
        let end = if let Some(end) = &node.end {
            if cfg!(feature = "metrics") {
                self.stats.replayed += 1;
            }
            end.clone()
        } else {
            if cfg!(feature = "metrics") {
                self.stats.executed += 1;
            }
            let cursor = trace.nodes[job.node]
                .cursor
                .take()
                .expect("live trace cursor");
            let end = match trace
                .machine
                .as_mut()
                .expect("unfinished call")
                .step(cursor)
            {
                Step::Continue(cursor) => {
                    let node = &mut trace.nodes[job.node];
                    node.progress += 1;
                    node.cursor = Some(cursor);
                    job.position += 1;
                    return Ok(Event::Continue(job));
                }
                Step::Split(a, b) => {
                    trace.unfinished += 2;
                    let i = trace.nodes.len();
                    trace.nodes.push(Node {
                        progress: 0,
                        cursor: Some(a),
                        end: None,
                    });
                    trace.nodes.push(Node {
                        progress: 0,
                        cursor: Some(b),
                        end: None,
                    });
                    End::Split(i, i + 1)
                }
                Step::Failed => End::Failed,
                Step::Answer(a) => {
                    if a.residual.is_empty() {
                        End::Answer(a.outputs.into_iter().map(|(_, t)| t).collect())
                    } else {
                        End::Invalid
                    }
                }
            };
            trace.nodes[job.node].end = Some(end.clone());
            trace.unfinished -= 1;
            if trace.unfinished == 0 {
                trace.machine = None;
            }
            end
        };
        match end {
            End::Split(a, b) => {
                let other = Job {
                    trace: job.trace,
                    node: b,
                    position: 0,
                    originals: job.originals.clone(),
                    owner: job.owner.clone(),
                };
                job.node = a;
                job.position = 0;
                Ok(Event::Split(job, other))
            }
            End::Failed => Ok(Event::Failed),
            End::Invalid => Err("isolated call suspended with residual work".into()),
            End::Answer(result) => {
                let mut mapping = job
                    .originals
                    .iter()
                    .enumerate()
                    .map(|(i, v)| (Var(i as u64), *v))
                    .collect::<BTreeMap<_, _>>();
                Ok(Event::Answer(
                    job.originals
                        .into_iter()
                        .zip(result.into_iter().map(|t| {
                            rename(&t, &mut |v| {
                                *mapping.entry(v).or_insert_with(|| fresh.take())
                            })
                        }))
                        .collect(),
                ))
            }
        }
    }
}
