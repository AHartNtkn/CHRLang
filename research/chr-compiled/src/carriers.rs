//! Source-derived pure carrier contraction under Global sealed selection.
//! Validation is resumable; terminal selection/body and unknown tails stay ordinary.
use super::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Eligibility {
    pub predicate: (String, usize),
    pub eligible: bool,
    pub reason: Option<String>,
}
#[derive(Clone)]
pub(crate) struct Plan {
    pred: usize,
    control: usize,
    step: String,
    base: String,
}
fn infer(rules: &[Prepared], uses: &[(usize, usize)]) -> Option<(usize, Plan)> {
    if uses.len() != 2 || uses[0].0 == uses[1].0 {
        return None;
    }
    for &(r, _) in uses {
        let step = &rules[r];
        let other = &rules[uses.iter().find(|x| x.0 != r)?.0];
        if step.kept != 0
            || step.heads.len() != 1
            || !step.guards.is_empty()
            || other.kept != 0
            || other.heads.len() != 1
        {
            continue;
        }
        let head = &step.heads[0];
        for (control, pattern) in head.args.iter().enumerate() {
            let Template::App(name, child) = pattern else {
                continue;
            };
            let [Template::Slot(child)] = child.as_slice() else {
                continue;
            };
            let mut slots = BTreeSet::from([*child]);
            let mut valid = true;
            for (i, arg) in head.args.iter().enumerate() {
                if i != control {
                    if let Template::Slot(slot) = arg {
                        valid &= slots.insert(*slot);
                    } else {
                        valid = false;
                    }
                }
            }
            if !valid {
                continue;
            }
            let Template::App(base, fields) = &other.heads[0].args[control] else {
                continue;
            };
            if !fields.is_empty() {
                continue;
            }
            let Body::Insert(pred, args) = &step.body else {
                continue;
            };
            if *pred != head.pred || args.len() != head.args.len() {
                continue;
            }
            for (i, arg) in args.iter().enumerate() {
                let expected = if i == control {
                    *child
                } else {
                    let Template::Slot(s) = &head.args[i] else {
                        unreachable!()
                    };
                    *s
                };
                if !matches!(arg,Template::Slot(slot) if *slot==expected) {
                    valid = false;
                }
            }
            if valid {
                return Some((
                    r,
                    Plan {
                        pred: *pred,
                        control,
                        step: name.clone(),
                        base: base.clone(),
                    },
                ));
            }
        }
    }
    None
}
impl PreparedRuleset {
    pub fn carrier_eligibility(&self) -> Vec<Eligibility> {
        self.predicates.iter().enumerate().map(|(pred,name)| {let eligible=self.dispatch.get(&pred).is_some_and(|uses|infer(&self.rules,uses).is_some()); Eligibility{predicate:name.clone(),eligible,reason:(!eligible).then(||"requires exactly one pure decreasing single-head step and one nullary-control terminal head".into())}}).collect()
    }
    pub fn contract_carriers_inferred(&self) -> Result<Self, String> {
        let requested = self
            .carrier_eligibility()
            .into_iter()
            .filter(|r| r.eligible)
            .map(|r| r.predicate)
            .collect::<Vec<_>>();
        self.contract_carriers_checked(&requested)
    }
    /// Requires existing sealed specialization; runtime starts retain its Global requirement.
    pub fn contract_carriers_checked(&self, requested: &[(String, usize)]) -> Result<Self, String> {
        let regions = self
            .regions
            .as_ref()
            .ok_or("carrier contraction requires sealed specialization")?;
        let mut rules = (*self.rules).clone();
        for r in &mut rules {
            r.carrier = None;
        }
        for name in requested {
            let pred = self
                .predicates
                .iter()
                .position(|p| p == name)
                .ok_or("unknown carrier predicate")?;
            let (r, plan) = self
                .dispatch
                .get(&pred)
                .and_then(|uses| infer(&rules, uses))
                .ok_or("ineligible carrier predicate")?;
            if regions[r].is_none() {
                return Err("carrier step lacks sealed specialization".into());
            }
            rules[r].carrier = Some(Arc::new(plan));
        }
        let mut prepared = self.clone();
        prepared.rules = Arc::new(rules);
        Ok(prepared)
    }
}
#[derive(Clone)]
enum Phase {
    AwaitInsert,
    SuppressedInsert,
    Check { cursor: Term, steps: u64 },
    Expand { remaining: u64 },
    ExpandedInsert { remaining: u64 },
}
#[derive(Clone)]
pub(crate) struct Job {
    plan: Arc<Plan>,
    rule: usize,
    id: u64,
    args: Vec<Term>,
    phase: Phase,
}
impl Engine {
    pub(crate) fn carrier_entry(&mut self, app: &Application) {
        let suppressed = app.ids.len() == 1 && self.carrier_blocked == Some(app.ids[0]);
        if app.ids.iter().any(|id| self.carrier_blocked == Some(*id)) {
            self.carrier_blocked = None;
        }
        let Some(plan) = self.core.rules[app.rule].carrier.clone() else {
            return;
        };
        // A later checked specialization may select a smaller predicate set.
        // Contraction still requires the history-free sealed selection boundary.
        if !self
            .core
            .regions
            .as_ref()
            .is_some_and(|plans| plans[app.rule].is_some())
        {
            return;
        }
        if self.core.pools.get(&plan.pred).map_or(0, BTreeSet::len) != 1 {
            return;
        }
        self.carrier = Some(Job {
            plan,
            rule: app.rule,
            id: 0,
            args: vec![],
            phase: if suppressed {
                Phase::SuppressedInsert
            } else {
                Phase::AwaitInsert
            },
        });
    }
    pub(crate) fn carrier_inserted(&mut self, id: u64) {
        let Some(mut job) = self.carrier.take() else {
            return;
        };
        match job.phase {
            Phase::SuppressedInsert => {
                self.carrier_blocked = Some(id);
                return;
            }
            Phase::AwaitInsert => {
                job.id = id;
                job.args = self.core.store[&id].args.clone();
                job.phase = Phase::Check {
                    cursor: job.args[job.plan.control],
                    steps: 0,
                };
            }
            Phase::ExpandedInsert { remaining } => {
                job.id = id;
                job.args = self.core.store[&id].args.clone();
                if remaining == 0 {
                    return;
                }
                job.phase = Phase::Expand { remaining };
            }
            _ => unreachable!("carrier inserts only own pure body"),
        }
        self.carrier = Some(job);
    }
    pub(crate) fn carrier_tick(&mut self) -> bool {
        let Some(mut job) = self.carrier.take() else {
            return false;
        };
        match job.phase {
            Phase::AwaitInsert | Phase::SuppressedInsert | Phase::ExpandedInsert { .. } => {
                self.carrier = Some(job);
                return false;
            }
            Phase::Check { cursor, steps } => {
                if COLLECT_METRICS {
                    self.core.stats.carrier_checks += 1;
                }
                let cursor = deref(cursor, &self.core.bindings, &mut self.core.stats.kernel);
                if let Term::Node(id) = cursor {
                    let node = self.core.arena.node(id);
                    if node.name == job.plan.step && node.args.len() == 1 {
                        job.phase = Phase::Check {
                            cursor: node.args[0],
                            steps: steps.checked_add(1).expect("carrier depth"),
                        };
                        self.carrier = Some(job);
                        return true;
                    }
                    if node.name == job.plan.base && node.args.is_empty() {
                        if steps == 0 {
                            return true;
                        }
                        if COLLECT_METRICS {
                            self.core.stats.carrier_contractions += 1;
                        }
                        if self.trace_enabled || self.audit_enabled {
                            job.phase = Phase::Expand { remaining: steps };
                            self.carrier = Some(job);
                            return true;
                        }
                        // All omitted applications consume/repost exactly once and allocate no vars.
                        let final_id = self
                            .core
                            .next_occ
                            .checked_add(steps - 1)
                            .expect("occurrence IDs exhausted");
                        self.core.remove(job.id);
                        job.args[job.plan.control] = cursor;
                        self.core.next_occ = final_id;
                        self.core.insert(job.plan.pred, job.args);
                        if COLLECT_METRICS {
                            self.core.stats.applications += steps;
                            self.core.stats.specialized_applications += steps;
                            self.core.stats.carrier_steps += steps;
                        }
                        return true;
                    }
                }
                self.carrier_blocked = Some(job.id);
            }
            Phase::Expand { remaining } => {
                let control = deref(
                    job.args[job.plan.control],
                    &self.core.bindings,
                    &mut self.core.stats.kernel,
                );
                let Term::Node(id) = control else {
                    unreachable!("validated spine")
                };
                job.args[job.plan.control] = self.core.arena.node(id).args[0];
                let body = Work::Insert(job.plan.pred, job.args.clone());
                self.commit_application(
                    Application {
                        rule: job.rule,
                        ids: vec![job.id],
                        body,
                        next: self.core.next_var,
                    },
                    None,
                );
                if COLLECT_METRICS {
                    self.core.stats.carrier_steps += 1;
                }
                job.phase = Phase::ExpandedInsert {
                    remaining: remaining - 1,
                };
                self.carrier = Some(job);
            }
        }
        true
    }
}
