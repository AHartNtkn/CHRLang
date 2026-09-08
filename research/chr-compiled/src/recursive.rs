//! Sufficient source certificate for one deterministic, ground-controlled recursive call.
//!
//! Preparation owns reusable equation templates. Execution uses no CHR occurrence
//! store or selector. It is synchronous: the ground spine bounds iterations, not
//! the size of unification or exported terms. Arena/bindings belong to one query.
use chr_persistent::{
    Stats,
    kernel::{Arena, Bindings, Scope, Term},
};
use chr_syntax::{Answer, Goal, Query, Rule, Term as Source, Var};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unsupported(pub String);
struct Plan {
    parameters: Vec<Option<u64>>,
    child: Option<u64>,
    equations: Vec<(Source, Source)>,
    next: Option<Vec<Source>>,
}
pub struct Prepared {
    predicate: String,
    arity: usize,
    control: usize,
    base_constructor: String,
    step_constructor: String,
    base: Plan,
    step: Plan,
}
fn flatten<'a>(goal: &'a Goal, out: &mut Vec<&'a Goal>) {
    match goal {
        Goal::And(gs) => {
            for g in gs {
                flatten(g, out);
            }
        }
        Goal::True => (),
        _ => out.push(goal),
    }
}
fn plan(rule: &Rule, control: usize, step: bool) -> Option<(String, Plan)> {
    let head = &rule.removed[0];
    let Source::App(name, children) = &head.args[control] else {
        return None;
    };
    let child = if step {
        let [Source::Var(Var(v))] = children.as_slice() else {
            return None;
        };
        Some(*v)
    } else {
        if !children.is_empty() {
            return None;
        }
        None
    };
    let mut seen = BTreeSet::new();
    if let Some(v) = child {
        seen.insert(v);
    }
    let mut parameters = Vec::new();
    for (i, arg) in head.args.iter().enumerate() {
        if i == control {
            parameters.push(None);
            continue;
        }
        let Source::Var(Var(v)) = arg else {
            return None;
        };
        if !seen.insert(*v) {
            return None;
        }
        parameters.push(Some(*v));
    }
    let mut flat = vec![];
    flatten(&rule.body, &mut flat);
    let next = if step {
        let Goal::Constraint(call) = flat.pop()? else {
            return None;
        };
        if call.name != head.name
            || call.args.len() != head.args.len()
            || call.args[control] != Source::Var(Var(child?))
        {
            return None;
        }
        Some(call.args.clone())
    } else {
        None
    };
    let equations = flat
        .into_iter()
        .map(|g| match g {
            Goal::Unify(a, b) => Some((a.clone(), b.clone())),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()?;
    Some((
        name.clone(),
        Plan {
            parameters,
            child,
            equations,
            next,
        },
    ))
}
impl Prepared {
    /// Certify the entire sealed source, without predicate/constructor-name conventions.
    pub fn new(rules: Vec<Rule>) -> Result<Self, String> {
        if rules.len() != 2 || rules[0].name == rules[1].name {
            return Err("requires exactly two distinctly named rules".into());
        }
        if rules
            .iter()
            .any(|r| !r.kept.is_empty() || r.removed.len() != 1 || !r.guards.is_empty())
        {
            return Err("requires one removed head, no kept heads or guards".into());
        }
        let head = &rules[0].removed[0];
        let other = &rules[1].removed[0];
        if head.name != other.name || head.args.len() != other.args.len() {
            return Err("heads must have the same predicate and arity".into());
        }
        for control in 0..head.args.len() {
            for base_index in 0..2 {
                if let (Some((base_constructor, base)), Some((step_constructor, step))) = (
                    plan(&rules[base_index], control, false),
                    plan(&rules[1 - base_index], control, true),
                ) {
                    return Ok(Self {
                        predicate: head.name.clone(),
                        arity: head.args.len(),
                        control,
                        base_constructor,
                        step_constructor,
                        base,
                        step,
                    });
                }
            }
        }
        Err("source does not satisfy the base/step equation-recursion schema".into())
    }
    pub fn predicate(&self) -> (&str, usize) {
        (&self.predicate, self.arity)
    }
    pub fn control_position(&self) -> usize {
        self.control
    }
    /// None is finite-tree equality failure; Err is outside the certified query domain.
    /// Output-only source variables are fresh query variables, including unused ones.
    pub fn execute(&self, query: Query) -> Result<Option<Answer>, Unsupported> {
        let [call] = query.constraints.as_slice() else {
            return Err(Unsupported("requires exactly one call".into()));
        };
        if call.name != self.predicate || call.args.len() != self.arity {
            return Err(Unsupported("call predicate or arity differs".into()));
        }
        let mut names = BTreeSet::new();
        if query.outputs.iter().any(|(name, _)| !names.insert(name)) {
            return Err(Unsupported("duplicate output name".into()));
        }
        let mut control = &call.args[self.control];
        let mut depth = 0;
        loop {
            match control {
                Source::App(name, args) if *name == self.base_constructor && args.is_empty() => {
                    break;
                }
                Source::App(name, args) if *name == self.step_constructor && args.len() == 1 => {
                    depth += 1;
                    control = &args[0];
                }
                _ => {
                    return Err(Unsupported(
                        "control is not a finite ground base/step spine".into(),
                    ));
                }
            }
        }
        let mut arena = Arena::default();
        let mut bindings = Bindings::default();
        let mut stats = Stats::default();
        let mut next = 0;
        let mut query_scope = Scope::new();
        let mut args: Vec<_> = call
            .args
            .iter()
            .map(|t| arena.instantiate(t, &mut query_scope, &mut next, &mut stats))
            .collect();
        let outputs: Vec<_> = query
            .outputs
            .iter()
            .map(|(name, var)| {
                (
                    name.clone(),
                    arena.instantiate(&Source::Var(*var), &mut query_scope, &mut next, &mut stats),
                )
            })
            .collect();
        for remaining in (0..=depth).rev() {
            let plan = if remaining == 0 {
                &self.base
            } else {
                &self.step
            };
            let mut scope = Scope::new();
            for (parameter, value) in plan.parameters.iter().zip(&args) {
                if let Some(id) = parameter {
                    scope.insert(*id, *value);
                }
            }
            if let Some(child) = plan.child {
                let Term::Node(node) = args[self.control] else {
                    unreachable!("certified ground control")
                };
                scope.insert(child, arena.node(node).args[0]);
            }
            for (left, right) in &plan.equations {
                let left = arena.instantiate(left, &mut scope, &mut next, &mut stats);
                let right = arena.instantiate(right, &mut scope, &mut next, &mut stats);
                if !arena.unify(left, right, &mut bindings, &mut stats) {
                    return Ok(None);
                }
            }
            if let Some(call) = &plan.next {
                args = call
                    .iter()
                    .map(|t| arena.instantiate(t, &mut scope, &mut next, &mut stats))
                    .collect();
            }
        }
        Ok(Some(Answer {
            outputs: outputs
                .into_iter()
                .map(|(name, term)| (name, arena.export(term, &bindings, &mut stats)))
                .collect(),
            residual: vec![],
        }))
    }
}
