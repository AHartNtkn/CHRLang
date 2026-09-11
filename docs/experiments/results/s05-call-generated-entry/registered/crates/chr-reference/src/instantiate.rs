//! Renaming is reference-local. Rule variables and query variables have separate scopes.
use chr_syntax::{Constraint, Goal, Term, Var};
use std::collections::BTreeMap;

pub(crate) type Bindings = BTreeMap<Var, Term>;

pub(crate) fn term(input: &Term, bindings: &mut Bindings, next: &mut u64) -> Term {
    match input {
        Term::Var(var) => bindings
            .entry(*var)
            .or_insert_with(|| {
                let result = Term::Var(Var(*next));
                *next += 1;
                result
            })
            .clone(),
        Term::App(name, args) => Term::App(
            name.clone(),
            args.iter().map(|a| term(a, bindings, next)).collect(),
        ),
    }
}
pub(crate) fn constraint(
    input: &Constraint,
    bindings: &mut Bindings,
    next: &mut u64,
) -> Constraint {
    Constraint {
        name: input.name.clone(),
        args: input.args.iter().map(|a| term(a, bindings, next)).collect(),
    }
}
pub(crate) fn goal(input: &Goal, bindings: &mut Bindings, next: &mut u64) -> Goal {
    match input {
        Goal::Constraint(c) => Goal::Constraint(constraint(c, bindings, next)),
        Goal::Unify(a, b) => Goal::Unify(term(a, bindings, next), term(b, bindings, next)),
        Goal::And(gs) => Goal::And(gs.iter().map(|g| goal(g, bindings, next)).collect()),
        Goal::Or(a, b) => Goal::Or(
            Box::new(goal(a, bindings, next)),
            Box::new(goal(b, bindings, next)),
        ),
        Goal::True => Goal::True,
        Goal::Fail => Goal::Fail,
    }
}
