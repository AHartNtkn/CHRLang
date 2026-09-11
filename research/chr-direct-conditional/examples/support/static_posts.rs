//! Source-derived initialization of static, nullary, inert propagation facts.
use chr_syntax::{Constraint, Goal, Query, Rule};
use std::collections::BTreeSet;
fn calls(goal: &Goal, out: &mut BTreeSet<(String, usize)>) {
    match goal {
        Goal::Constraint(c) => {
            out.insert((c.name.clone(), c.args.len()));
        }
        Goal::And(gs) => {
            for g in gs {
                calls(g, out);
            }
        }
        Goal::Or(a, b) => {
            calls(a, out);
            calls(b, out);
        }
        _ => (),
    }
}
fn posts(goal: &Goal, out: &mut Vec<Constraint>) -> bool {
    match goal {
        Goal::Constraint(c) if c.args.is_empty() => {
            out.push(c.clone());
            true
        }
        Goal::And(gs) => gs.iter().all(|g| posts(g, out)),
        Goal::True => true,
        _ => false,
    }
}
pub struct Prepared {
    rules: Vec<Rule>,
    initializers: Vec<(String, Vec<Constraint>)>,
}
impl Prepared {
    pub fn infer(rules: &[Rule]) -> Result<Self, &'static str> {
        let heads: BTreeSet<_> = rules
            .iter()
            .flat_map(|r| r.kept.iter().chain(&r.removed))
            .map(|c| (c.name.clone(), c.args.len()))
            .collect();
        let mut generated = BTreeSet::new();
        for r in rules {
            calls(&r.body, &mut generated);
        }
        let mut remaining = vec![];
        let mut initializers = vec![];
        for r in rules {
            if !r.removed.is_empty() {
                remaining.push(r.clone());
                continue;
            }
            if !r.guards.is_empty() || r.kept.len() != 1 || !r.kept[0].args.is_empty() {
                return Err("propagation must have one static nullary head");
            }
            let head = &r.kept[0];
            if generated.contains(&(head.name.clone(), 0))
                || rules.iter().any(|r| {
                    r.removed
                        .iter()
                        .any(|c| c.name == head.name && c.args.is_empty())
                })
            {
                return Err("propagation head may change during execution");
            }
            let mut emitted = vec![];
            if !posts(&r.body, &mut emitted)
                || emitted.iter().any(|c| heads.contains(&(c.name.clone(), 0)))
            {
                return Err("propagation body must only post inert nullary facts");
            }
            initializers.push((head.name.clone(), emitted));
        }
        Ok(Self {
            rules: remaining,
            initializers,
        })
    }
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }
    pub fn initialize(&self, q: &Query) -> Query {
        let mut result = q.clone();
        for (name, posts) in &self.initializers {
            for _ in q
                .constraints
                .iter()
                .filter(|c| c.name == *name && c.args.is_empty())
            {
                result.constraints.extend(posts.iter().cloned());
            }
        }
        result
    }
}
