//! Completed private-phase results using the existing finite execution plan.
//! Keys preserve ordered private syntax; caller state is transported on every use.
use super::*;
use std::collections::VecDeque;
#[derive(Default, Clone, Copy)]
pub struct Stats {
    pub computed: usize,
    pub hits: usize,
}
struct Row {
    values: Vec<Term>,
    weight: u128,
}
pub struct Cache<'p, const DIAGNOSTICS: bool = false> {
    prepared: &'p Prepared,
    capacity: usize,
    entries: VecDeque<(Vec<Constraint>, Vec<Row>)>,
    stats: Stats,
}
pub struct Session<'a, 'p, const DIAGNOSTICS: bool> {
    owner: &'a mut Cache<'p, DIAGNOSTICS>,
    machine: Option<Machine<'p>>,
    key: Vec<Constraint>,
    vars: Vec<Var>,
    caller: Query,
    limits: Limits,
    hit: Option<usize>,
    finished: bool,
}
fn rename(
    t: &Term,
    map: &mut impl FnMut(Var) -> Result<Var, Error>,
    budget: &mut Budget,
    depth: usize,
) -> Result<Term, Error> {
    budget.node(depth)?;
    Ok(match t {
        Term::Var(v) => Term::Var(map(*v)?),
        Term::App(n, xs) => Term::App(
            n.clone(),
            xs.iter()
                .map(|x| rename(x, map, budget, depth + 1))
                .collect::<Result<_, _>>()?,
        ),
    })
}
impl<'p, const DIAGNOSTICS: bool> Cache<'p, DIAGNOSTICS> {
    pub fn new(prepared: &'p Prepared, capacity: usize) -> Self {
        Self {
            prepared,
            capacity,
            entries: VecDeque::new(),
            stats: Stats::default(),
        }
    }
    pub fn stats(&self) -> Stats {
        self.stats
    }
    pub fn retained(&self) -> usize {
        self.entries.len()
    }
    pub fn start<'a>(
        &'a mut self,
        query: &Query,
        limits: Limits,
    ) -> Result<Session<'a, 'p, DIAGNOSTICS>, Error> {
        // The ordinary admission path validates even a hit. Its allocation and
        // domain preparation belong to query setup in subsequent cost comparisons.
        let mut machine = self.prepared.start(query, limits)?;
        let mut vars = vec![];
        let mut slots = BTreeMap::new();
        let mut budget = Budget {
            left: limits.term_nodes,
        };
        let mut signature = vec![];
        let mut caller = Query {
            constraints: vec![],
            outputs: query.outputs.clone(),
        };
        for c in &query.constraints {
            if self.prepared.private.contains(&key(c)) {
                signature.push(Constraint {
                    name: c.name.clone(),
                    args: c
                        .args
                        .iter()
                        .map(|t| {
                            rename(
                                t,
                                &mut |v| {
                                    Ok(*slots.entry(v).or_insert_with(|| {
                                        let slot = Var(vars.len() as u64);
                                        vars.push(v);
                                        slot
                                    }))
                                },
                                &mut budget,
                                0,
                            )
                        })
                        .collect::<Result<_, _>>()?,
                });
            } else {
                caller.constraints.push(c.clone());
            }
        }
        let hit = self.entries.iter().position(|(k, _)| k == &signature);
        if DIAGNOSTICS {
            if hit.is_some() {
                self.stats.hits += 1;
            } else {
                self.stats.computed += 1;
            }
        }
        // Observe every private variable, including those used only by caller
        // constraints. The finite source cannot introduce fresh variables.
        if hit.is_none() {
            let work = machine.work.as_mut().ok_or(Error::AlreadyFinished)?;
            work.caller.constraints.clear();
            work.caller.outputs = vars
                .iter()
                .enumerate()
                .map(|(i, v)| (format!("slot{i}"), *v))
                .collect();
        }
        Ok(Session {
            owner: self,
            machine: if hit.is_some() { None } else { Some(machine) },
            key: signature,
            vars,
            caller,
            limits,
            hit,
            finished: false,
        })
    }
    pub fn solve(&mut self, query: &Query, limits: Limits) -> Result<Report, Error> {
        self.start(query, limits)?.finish()
    }
}
fn transport(
    rows: &[Row],
    vars: &[Var],
    caller: &Query,
    limits: Limits,
    steps: usize,
    partitions: usize,
) -> Result<Report, Error> {
    if rows.len() > limits.solutions {
        return Err(Error::Limit("solutions"));
    }
    let mut budget = Budget {
        left: limits.term_nodes,
    };
    let mut solutions = Vec::with_capacity(rows.len());
    for row in rows {
        let mut bindings = Env::new();
        for (var, value) in vars.iter().zip(&row.values) {
            let value = rename(
                value,
                &mut |v| {
                    vars.get(v.0 as usize)
                        .copied()
                        .ok_or(Error::Source("result outside private interface"))
                },
                &mut budget,
                0,
            )?;
            if value != Term::Var(*var) {
                bindings.insert(*var, value);
            }
        }
        let mut query = caller.clone();
        for c in &mut query.constraints {
            for t in &mut c.args {
                *t = resolve(t, &bindings, &mut budget, 0)?;
            }
        }
        let equations = query
            .outputs
            .iter()
            .map(|(_, v)| {
                Ok((
                    Term::Var(*v),
                    resolve(&Term::Var(*v), &bindings, &mut budget, 0)?,
                ))
            })
            .collect::<Result<_, Error>>()?;
        solutions.push(Solution {
            query,
            equations,
            multiplicity: row.weight,
        });
    }
    Ok(Report {
        solutions,
        steps,
        partitions,
    })
}
impl<const DIAGNOSTICS: bool> Session<'_, '_, DIAGNOSTICS> {
    pub fn advance(&mut self) -> Result<Event, Error> {
        if self.finished {
            return Ok(Event::Exhausted);
        }
        if let Some(index) = self.hit {
            self.finished = true;
            return transport(
                &self.owner.entries[index].1,
                &self.vars,
                &self.caller,
                self.limits,
                0,
                0,
            )
            .map(Event::Complete);
        }
        let event = match self
            .machine
            .as_mut()
            .ok_or(Error::AlreadyFinished)?
            .advance()
        {
            Ok(event) => event,
            Err(error) => {
                self.finished = true;
                return Err(error);
            }
        };
        let Event::Complete(report) = event else {
            return Ok(event);
        };
        self.finished = true;
        let slots: BTreeMap<_, _> = self
            .vars
            .iter()
            .enumerate()
            .map(|(i, v)| (*v, Var(i as u64)))
            .collect();
        let mut budget = Budget {
            left: self.limits.term_nodes,
        };
        let rows = report
            .solutions
            .iter()
            .map(|s| {
                let values = s
                    .equations
                    .iter()
                    .map(|(_, t)| {
                        rename(
                            t,
                            &mut |v| {
                                slots
                                    .get(&v)
                                    .copied()
                                    .ok_or(Error::Source("result outside private interface"))
                            },
                            &mut budget,
                            0,
                        )
                    })
                    .collect::<Result<_, Error>>()?;
                Ok(Row {
                    values,
                    weight: s.multiplicity,
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let output = transport(
            &rows,
            &self.vars,
            &self.caller,
            self.limits,
            report.steps,
            report.partitions,
        )?;
        if self.owner.capacity > 0 {
            if self.owner.entries.len() == self.owner.capacity {
                self.owner.entries.pop_front();
            }
            self.owner
                .entries
                .push_back((std::mem::take(&mut self.key), rows));
        }
        Ok(Event::Complete(output))
    }
    pub fn finish(mut self) -> Result<Report, Error> {
        loop {
            match self.advance()? {
                Event::Progress => (),
                Event::Complete(report) => return Ok(report),
                Event::Exhausted => return Err(Error::AlreadyFinished),
            }
        }
    }
}
