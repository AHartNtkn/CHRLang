//! Checked leading predicates with direct ordered occurrence selection.
//!
//! Eligibility is a whole-program fact: every head use must lead a fully
//! consuming rule, alone or with a distinct nullary second head. Selection uses prepared register operations,
//! not generic tuple cursors or source-pattern interpretation. Other predicates
//! retain ordinary selection. Source rules, occurrence order and body boundaries
//! remain those of Policy::Global; Active is explicitly unsupported.
use super::*;
use std::ops::Bound::{Excluded, Unbounded};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Eligibility {
    pub predicate: (String, usize),
    pub eligible: bool,
    pub reason: Option<String>,
}
impl PreparedRuleset {
    pub fn region_eligibility(&self) -> Vec<Eligibility> {
        self.predicates
            .iter()
            .enumerate()
            .map(|(pred, predicate)| {
                let reason = match self.dispatch.get(&pred) {
                    None => Some("predicate has no source head uses".into()),
                    Some(uses) => uses.iter().find_map(|&(rule, head)| {
                        let r = &self.rules[rule];
                        let shape = r.heads.len() == 1 || (r.heads.len() == 2
                            && r.heads[1].args.is_empty() && r.heads[0].pred != r.heads[1].pred);
                        (head != 0 || r.kept != 0 || !shape).then(|| {
                            format!("rule {rule} requires a leading removed head, no kept heads, and at most one distinct nullary partner")
                        })
                    }),
                };
                Eligibility {
                    predicate: predicate.clone(),
                    eligible: reason.is_none(),
                    reason,
                }
            })
            .collect()
    }
    /// Select every eligible predicate. The returned prepared object explicitly
    /// uses specialization; its query starts require Global policy.
    pub fn specialize_inferred(&self) -> Self {
        let requested = self
            .region_eligibility()
            .into_iter()
            .filter(|e| e.eligible)
            .map(|e| e.predicate)
            .collect::<Vec<_>>();
        self.specialize_checked(&requested)
            .expect("inferred sealed predicates")
    }
    /// Select exactly the requested set, checking all source head uses. The
    /// receiver remains unchanged; no unchecked declaration or fallback exists.
    pub fn specialize_checked(&self, requested: &[(String, usize)]) -> Result<Self, String> {
        let reports = self.region_eligibility();
        let mut predicates = BTreeSet::new();
        for predicate in requested {
            let report = reports
                .iter()
                .find(|e| &e.predicate == predicate)
                .ok_or_else(|| format!("unknown predicate {}/{}", predicate.0, predicate.1))?;
            if let Some(reason) = &report.reason {
                return Err(format!(
                    "cannot specialize {}/{}: {reason}",
                    predicate.0, predicate.1
                ));
            }
            let pred = self.predicates.iter().position(|p| p == predicate).unwrap();
            predicates.insert(pred);
        }
        let plans = self
            .rules
            .iter()
            .map(|r| {
                predicates
                    .contains(&r.heads[0].pred)
                    .then(|| Plan::compile(r))
            })
            .collect();
        let mut prepared = self.clone();
        prepared.regions = Some(Arc::new(plans));
        Ok(prepared)
    }
}
#[derive(Clone)]
enum Instruction {
    Bind {
        input: usize,
        slot: usize,
    },
    Constructor {
        input: usize,
        name: String,
        start: usize,
        arity: usize,
    },
    Variable {
        output: usize,
        slot: usize,
    },
    Make {
        output: usize,
        name: String,
        inputs: Vec<usize>,
    },
    Equal {
        left: usize,
        right: usize,
    },
}
pub(crate) struct Plan {
    instructions: Vec<Instruction>,
    registers: usize,
}
impl Plan {
    fn compile(rule: &Prepared) -> Self {
        fn pattern(p: &Template, input: usize, plan: &mut Plan) {
            match p {
                Template::Slot(slot) => plan
                    .instructions
                    .push(Instruction::Bind { input, slot: *slot }),
                Template::App(name, args) => {
                    let start = plan.registers;
                    plan.registers += args.len();
                    plan.instructions.push(Instruction::Constructor {
                        input,
                        name: name.clone(),
                        start,
                        arity: args.len(),
                    });
                    for (i, p) in args.iter().enumerate() {
                        pattern(p, start + i, plan);
                    }
                }
            }
        }
        fn value(t: &Template, plan: &mut Plan) -> usize {
            let output = plan.registers;
            plan.registers += 1;
            match t {
                Template::Slot(slot) => plan.instructions.push(Instruction::Variable {
                    output,
                    slot: *slot,
                }),
                Template::App(name, args) => {
                    let inputs = args.iter().map(|a| value(a, plan)).collect();
                    plan.instructions.push(Instruction::Make {
                        output,
                        name: name.clone(),
                        inputs,
                    });
                }
            }
            output
        }
        let mut plan = Self {
            instructions: vec![],
            registers: rule.heads[0].args.len(),
        };
        for (i, p) in rule.heads[0].args.iter().enumerate() {
            pattern(p, i, &mut plan);
        }
        for (a, b) in &rule.guards {
            let left = value(a, &mut plan);
            let right = value(b, &mut plan);
            plan.instructions.push(Instruction::Equal { left, right });
        }
        plan
    }
    fn matches(&self, core: &mut Core, args: Vec<Term>, slots: usize) -> Option<Frame> {
        let mut registers = vec![None; self.registers];
        for (i, t) in args.into_iter().enumerate() {
            registers[i] = Some(t);
        }
        let mut frame = Frame::new(slots, core.next_var);
        for instruction in &self.instructions {
            if COLLECT_METRICS {
                core.stats.specialized_head_instructions += 1;
            }
            match instruction {
                Instruction::Bind { input, slot } => {
                    if !core.bind(&mut frame, *slot, registers[*input].unwrap()) {
                        return None;
                    }
                }
                Instruction::Constructor {
                    input,
                    name,
                    start,
                    arity,
                } => {
                    let fields = core.constructor(registers[*input].unwrap(), name, *arity)?;
                    for (i, t) in fields.into_iter().enumerate() {
                        registers[start + i] = Some(t);
                    }
                }
                Instruction::Variable { output, slot } => {
                    registers[*output] = Some(core.variable(&mut frame, *slot))
                }
                Instruction::Make {
                    output,
                    name,
                    inputs,
                } => {
                    let fields = inputs.iter().map(|&r| registers[r].unwrap()).collect();
                    registers[*output] = Some(core.make(name, fields));
                }
                Instruction::Equal { left, right } => {
                    if !core.equal(registers[*left].unwrap(), registers[*right].unwrap()) {
                        return None;
                    }
                }
            }
        }
        Some(frame)
    }
}
#[derive(Clone)]
enum Bucket {
    Predicate(usize),
    Argument(IndexKey),
}
/// Ordered leading occurrence and optional nullary resource: no tuple frames or history keys.
#[derive(Clone)]
pub(crate) struct DirectCursor {
    bucket: Bucket,
    after: Option<u64>,
}
impl DirectCursor {
    pub(crate) fn new(core: &mut Core, rule: usize) -> Self {
        let frame = Frame::new(core.rules[rule].slots, core.next_var);
        let bucket = if let Some(key) = core.best_key(rule, 0, &frame) {
            if COLLECT_METRICS {
                core.stats.index_lookups += 1;
            }
            Bucket::Argument(key)
        } else {
            Bucket::Predicate(core.rules[rule].heads[0].pred)
        };
        Self {
            bucket,
            after: None,
        }
    }
    pub(crate) fn tick(&mut self, core: &mut Core, rule: usize) -> Selection {
        // A nullary partner cannot affect bindings or guards. Its oldest live
        // occurrence is the same one selected by ordinary source-order tuples.
        let partner = match core.rules[rule].heads.get(1) {
            None => None,
            Some(head) => {
                let Some(id) = core
                    .pools
                    .get(&head.pred)
                    .and_then(|pool| pool.first())
                    .copied()
                else {
                    return Selection::Done;
                };
                Some(id)
            }
        };
        let bucket = match self.bucket {
            Bucket::Predicate(p) => core.pools.get(&p),
            Bucket::Argument(k) => core.index.get(&k),
        };
        let next = bucket
            .and_then(|b| match self.after {
                Some(id) => b.range((Excluded(id), Unbounded)).next(),
                None => b.first(),
            })
            .copied();
        let Some(id) = next else {
            return Selection::Done;
        };
        self.after = Some(id);
        if COLLECT_METRICS {
            core.stats.specialized_candidates += 1;
        }
        let Some(args) = core.arguments(id) else {
            return Selection::Yield;
        };
        let plans = Arc::clone(
            core.regions
                .as_ref()
                .expect("explicit specialized selection"),
        );
        let program = Arc::clone(&core.rules);
        let desc = &program[rule];
        let Some(mut frame) = plans[rule]
            .as_ref()
            .unwrap()
            .matches(core, args, desc.slots)
        else {
            return Selection::Yield;
        };
        let body = core.body(&desc.body, &mut frame);
        let mut ids = vec![id];
        ids.extend(partner);
        Selection::Found(Application {
            rule,
            ids,
            body,
            next: frame.next,
        })
    }
}
#[cfg(test)]
mod tests {
    use crate::*;
    use chr_syntax::{atom, c, v};
    #[test]
    fn sealed_selection_bypasses_tuple_cursor_and_history() {
        let ordinary = PreparedRuleset::new(
            vec![Rule::simplify("take", [c("p", [v(0)])], Goal::True)],
            None,
        )
        .unwrap();
        let prepared = ordinary.specialize_checked(&[("p".into(), 1)]).unwrap();
        let mut engine = prepared
            .start(
                Query {
                    constraints: vec![c("p", [atom("a")])],
                    outputs: vec![],
                },
                Policy::Global,
                Access::Indexed,
            )
            .unwrap();
        assert!(engine.advance(100).exhausted);
        assert!(engine.observe().unwrap().residual.is_empty());
        assert_eq!(engine.stats().cursor_steps, 0);
        assert_eq!(engine.stats().history_checks, 0);
        assert_eq!(engine.retention().history, 0);
        assert_eq!(
            engine.stats().specialized_applications,
            u64::from(COLLECT_METRICS)
        );
    }
}
