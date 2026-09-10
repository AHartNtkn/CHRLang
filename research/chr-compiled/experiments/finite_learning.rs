//! Completed failed-domain regions, owned by one immutable finite phase.
//! No successful result is cached. Regions are subtracted from later domains.
use super::*;
use std::collections::VecDeque;
#[derive(Clone, PartialEq, Eq)]
struct Region {
    goals: Vec<Constraint>,
    domains: Vec<BTreeSet<String>>,
}
#[derive(Default, Clone, Copy, Debug)]
pub struct Stats {
    pub probes: usize,
    pub excluded_regions: usize,
    pub learned_regions: usize,
}
pub struct Learner<'p> {
    prepared: &'p Prepared,
    capacity: usize,
    regions: VecDeque<Region>,
    stats: Stats,
}
pub struct Session<'a, 'p> {
    owner: &'a mut Learner<'p>,
    machine: Machine<'p>,
    candidate: Option<Region>,
}
fn region(state: &State) -> Result<(Region, Vec<Var>), Error> {
    fn rename(t: &Term, vars: &mut Vec<Var>) -> Term {
        match t {
            Term::Var(v) => {
                let index = vars.iter().position(|x| x == v).unwrap_or_else(|| {
                    vars.push(*v);
                    vars.len() - 1
                });
                Term::Var(Var(index as u64))
            }
            Term::App(n, xs) => Term::App(n.clone(), xs.iter().map(|x| rename(x, vars)).collect()),
        }
    }
    let mut vars = vec![];
    let goals = state
        .live
        .iter()
        .map(|c| Constraint {
            name: c.name.clone(),
            args: c.args.iter().map(|t| rename(t, &mut vars)).collect(),
        })
        .collect();
    let domains = vars
        .iter()
        .map(|v| {
            state
                .domains
                .get(v)
                .map(|d| d.keys().cloned().collect())
                .ok_or(Error::Source("learning requires finite private variables"))
        })
        .collect::<Result<_, _>>()?;
    Ok((Region { goals, domains }, vars))
}
impl<'p> Learner<'p> {
    pub fn new(prepared: &'p Prepared, capacity: usize) -> Self {
        Self {
            prepared,
            capacity,
            regions: VecDeque::new(),
            stats: Stats::default(),
        }
    }
    pub fn retained(&self) -> usize {
        self.regions.len()
    }
    pub fn stats(&self) -> Stats {
        self.stats
    }
    pub fn start<'a>(
        &'a mut self,
        query: &Query,
        limits: Limits,
    ) -> Result<Session<'a, 'p>, Error> {
        let mut machine = self.prepared.start(query, limits)?;
        let work = machine.work.as_mut().expect("new machine");
        let candidate = if let Some(initial) = work.queue.first() {
            let (candidate, vars) = region(initial)?;
            for known in &self.regions {
                self.stats.probes += 1;
                if candidate.goals != known.goals {
                    continue;
                }
                assert_eq!(vars.len(), known.domains.len());
                let mut survivors = vec![];
                for mut state in work.queue.drain(..) {
                    let overlaps = vars.iter().zip(&known.domains).all(|(v, forbidden)| {
                        state.domains[v]
                            .keys()
                            .any(|value| forbidden.contains(value))
                    });
                    if !overlaps {
                        survivors.push(state);
                        continue;
                    }
                    self.stats.excluded_regions += 1;
                    // Disjoint complement of a box. Never select a value here:
                    // retain the new query's domain weights until ordinary solving.
                    for (v, forbidden) in vars.iter().zip(&known.domains) {
                        let outside: Domain = state.domains[v]
                            .iter()
                            .filter(|(value, _)| !forbidden.contains(*value))
                            .map(|(v, w)| (v.clone(), *w))
                            .collect();
                        if !outside.is_empty() {
                            fork(&mut work.report, limits)?;
                            let mut sibling = state.clone();
                            sibling.domains.insert(*v, outside);
                            survivors.push(sibling);
                        }
                        state
                            .domains
                            .get_mut(v)
                            .unwrap()
                            .retain(|value, _| forbidden.contains(value));
                    }
                    // The remaining intersection has a completed failure proof.
                }
                work.queue = survivors;
            }
            Some(candidate)
        } else {
            // An impossible producer input is query-specific, not a private-goal proof.
            None
        };
        Ok(Session {
            owner: self,
            machine,
            candidate,
        })
    }
    pub fn solve(&mut self, query: &Query, limits: Limits) -> Result<Report, Error> {
        self.start(query, limits)?.finish()
    }
}
impl Session<'_, '_> {
    pub fn advance(&mut self) -> Result<Event, Error> {
        match self.machine.advance() {
            Ok(Event::Complete(report)) => {
                if let Some(candidate) = self.candidate.take()
                    && report.solutions.is_empty()
                    && self.owner.capacity > 0
                    && !self.owner.regions.contains(&candidate)
                {
                    if self.owner.regions.len() == self.owner.capacity {
                        self.owner.regions.pop_front();
                    }
                    self.owner.regions.push_back(candidate);
                    self.owner.stats.learned_regions += 1;
                }
                Ok(Event::Complete(report))
            }
            Err(e) => {
                self.candidate = None;
                Err(e)
            }
            other => other,
        }
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
