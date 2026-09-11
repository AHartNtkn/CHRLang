//! Reusable ordinary-caller bridge. All equality transport uses a prepared rule.
use super::finite_phase::Solution;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEngine, SearchEvent};
use chr_syntax::{Answer, Constraint, Goal, Query, Rule, c, eq, v};
fn calls(g: &Goal, out: &mut Vec<String>) {
    match g {
        Goal::Constraint(c) => out.push(c.name.clone()),
        Goal::And(gs) => {
            for g in gs {
                calls(g, out)
            }
        }
        Goal::Or(a, b) => {
            calls(a, out);
            calls(b, out)
        }
        _ => (),
    }
}
pub struct Bridge {
    prepared: PreparedRuleset,
    name: String,
}
impl Bridge {
    pub fn new(mut rules: Vec<Rule>) -> Self {
        let mut names = vec![];
        for r in &rules {
            names.push(r.name.clone());
            names.extend(r.kept.iter().chain(&r.removed).map(|c| c.name.clone()));
            calls(&r.body, &mut names);
        }
        let mut name = "$finite-bind".to_owned();
        while names.contains(&name) {
            name.push('_');
        }
        rules.insert(
            0,
            Rule::simplify(&name, [c(&name, [v(0), v(1)])], eq(v(0), v(1))),
        );
        Self {
            prepared: PreparedRuleset::new(rules, None).unwrap(),
            name,
        }
    }
    pub fn transport(&self, solution: Solution) -> (Query, u128) {
        let Solution {
            mut query,
            equations,
            multiplicity,
        } = solution;
        assert!(
            !query.constraints.iter().any(|c| c.name == self.name),
            "query collides with private transport predicate"
        );
        query
            .constraints
            .extend(
                equations
                    .into_iter()
                    .filter(|(a, b)| a != b)
                    .map(|(a, b)| Constraint {
                        name: self.name.clone(),
                        args: vec![a, b],
                    }),
            );
        (query, multiplicity)
    }
    pub fn start(&self, q: Query, weight: u128) -> Caller {
        Caller {
            engine: self
                .prepared
                .start_search(q, Policy::Global, Access::Scan)
                .unwrap(),
            weight,
            repeat: None,
        }
    }
}
pub enum Event {
    Progress,
    Answer(Answer),
    Exhausted,
}
pub struct Caller {
    engine: SearchEngine,
    weight: u128,
    repeat: Option<(Answer, u128)>,
}
impl Caller {
    pub fn step(&mut self) -> Event {
        if let Some((answer, left)) = &mut self.repeat {
            if *left == 1 {
                return Event::Answer(self.repeat.take().unwrap().0);
            }
            *left -= 1;
            return Event::Answer(answer.clone());
        }
        match self.engine.tick() {
            SearchEvent::Complete(mut b) => {
                let a = b.engine.observe().unwrap();
                assert!(self.weight > 0);
                if self.weight == 1 {
                    Event::Answer(a)
                } else {
                    self.repeat = Some((a, self.weight));
                    Event::Progress
                }
            }
            SearchEvent::Exhausted => Event::Exhausted,
            _ => Event::Progress,
        }
    }
}
