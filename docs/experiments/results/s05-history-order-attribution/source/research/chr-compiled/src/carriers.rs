//! Source-derived pure carrier contraction under Global sealed selection.
//! Known-prefix inspection is resumable; arbitration at the actual tail stays ordinary.
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
    update: Option<Update>,
}
#[derive(Clone)]
struct Update {
    parameters: Vec<usize>,
    args: Vec<Template>,
    slots: usize,
}
impl Update {
    fn apply(&self, core: &mut Core, args: &[Term], control: usize, child: Term) -> Vec<Term> {
        let mut frame = Frame::new(self.slots, core.next_var);
        for (i, slot) in self.parameters.iter().enumerate() {
            frame.slots[*slot] = Some(if i == control { child } else { args[i] });
        }
        // Eligibility proves every slot read here was supplied by the head.
        self.args
            .iter()
            .map(|t| core.instantiate(t, &mut frame))
            .collect()
    }
}
fn bound(t: &Template, slots: &BTreeSet<usize>) -> bool {
    match t {
        Template::Slot(s) => slots.contains(s),
        Template::App(_, args) => args.iter().all(|t| bound(t, slots)),
    }
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
            let Template::App(_, fields) = &other.heads[0].args[control] else {
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
            let mut unchanged = true;
            let mut parameters = Vec::new();
            for (i, arg) in args.iter().enumerate() {
                let expected = if i == control {
                    *child
                } else {
                    let Template::Slot(s) = &head.args[i] else {
                        unreachable!()
                    };
                    *s
                };
                parameters.push(expected);
                let same = matches!(arg,Template::Slot(slot) if *slot==expected);
                unchanged &= same;
                valid &= if i == control {
                    same
                } else {
                    bound(arg, &slots)
                };
            }
            if valid {
                return Some((
                    r,
                    Plan {
                        pred: *pred,
                        control,
                        step: name.clone(),
                        update: (!unchanged).then(|| Update {
                            parameters,
                            args: args.clone(),
                            slots: step.slots,
                        }),
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
    updated: Option<Vec<Term>>,
    phase: Phase,
}
impl Engine {
    pub(crate) fn carrier_entry(&mut self, app: &Application) {
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
            updated: None,
            phase: Phase::AwaitInsert,
        });
    }
    pub(crate) fn carrier_inserted(&mut self, id: u64) {
        let Some(mut job) = self.carrier.take() else {
            return;
        };
        match job.phase {
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
            Phase::AwaitInsert | Phase::ExpandedInsert { .. } => {
                self.carrier = Some(job);
                return false;
            }
            Phase::Check { cursor, steps } => {
                if COLLECT_METRICS {
                    self.core.stats.carrier_checks += 1;
                }
                let value = deref(cursor, &self.core.bindings, &mut self.core.stats.kernel);
                if let Term::Node(id) = value {
                    let node = self.core.arena.node(id);
                    if node.name == job.plan.step && node.args.len() == 1 {
                        let child = node.args[0];
                        if !self.trace_enabled
                            && !self.audit_enabled
                            && let Some(update) = &job.plan.update
                        {
                            job.updated = Some(update.apply(
                                &mut self.core,
                                job.updated.as_deref().unwrap_or(&job.args),
                                job.plan.control,
                                child,
                            ));
                        }
                        job.phase = Phase::Check {
                            cursor: child,
                            steps: steps.checked_add(1).expect("carrier depth"),
                        };
                        self.carrier = Some(job);
                        return true;
                    }
                }
                // Only matching unary nodes were admitted. The actual tail can
                // be free, malformed or terminal; preserve its original handle.
                if steps == 0 {
                    return true;
                }
                if COLLECT_METRICS {
                    self.core.stats.carrier_contractions += 1;
                }
                if self.trace_enabled || self.audit_enabled {
                    // Replay from the original occurrence even if diagnostics
                    // were enabled partway through primary inspection.
                    job.updated = None;
                    job.phase = Phase::Expand { remaining: steps };
                    self.carrier = Some(job);
                    return true;
                }
                // Every omitted application consumes/reposts once with no fresh vars.
                let final_id = self
                    .core
                    .next_occ
                    .checked_add(steps - 1)
                    .expect("occurrence IDs exhausted");
                self.core.remove(job.id);
                if let Some(updated) = job.updated.take() {
                    job.args = updated;
                }
                job.args[job.plan.control] = cursor;
                self.core.next_occ = final_id;
                self.core.insert(job.plan.pred, job.args);
                if COLLECT_METRICS {
                    self.core.stats.applications += steps;
                    self.core.stats.specialized_applications += steps;
                    self.core.stats.carrier_steps += steps;
                }
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
                let child = self.core.arena.node(id).args[0];
                if let Some(update) = &job.plan.update {
                    job.args = update.apply(&mut self.core, &job.args, job.plan.control, child);
                } else {
                    job.args[job.plan.control] = child;
                }
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
