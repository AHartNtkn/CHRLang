//! Source control: eliminate syntactically identical, necessarily reflexive equations.
//! Guards and nonidentical equations are left intact; no equality analysis is used.
use chr_syntax::{Goal, Rule};
fn simplify(goal: &mut Goal) {
    match goal {
        Goal::Unify(a, b) if a == b => *goal = Goal::True,
        Goal::And(xs) => {
            for x in xs {
                simplify(x);
            }
        }
        Goal::Or(a, b) => {
            simplify(a);
            simplify(b);
        }
        _ => (),
    }
}
pub fn eliminate(rules: &[Rule]) -> Vec<Rule> {
    let mut rules = rules.to_vec();
    for r in &mut rules {
        simplify(&mut r.body);
    }
    rules
}
