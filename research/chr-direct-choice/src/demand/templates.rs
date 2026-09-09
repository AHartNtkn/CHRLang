//! Input-dependent residual source plans, containing no runtime graph identities.
use super::{Action, Clause, Plan};
use chr_syntax::{Constraint, Term, Var};
use std::collections::BTreeMap;

type Bindings = BTreeMap<Var, Term>;
pub(super) struct Template {
    pub body: Plan,
    pub followed_calls: usize,
}
struct Budget {
    nodes: usize,
    fresh: u64,
}
impl Budget {
    fn node(&mut self) -> Option<()> {
        self.nodes = self.nodes.checked_sub(1)?;
        Some(())
    }
    fn copy(&mut self, term: &Term) -> Option<Term> {
        self.node()?;
        Some(match term {
            Term::Var(v) => Term::Var(*v),
            Term::App(n, xs) => Term::App(
                n.clone(),
                xs.iter().map(|x| self.copy(x)).collect::<Option<_>>()?,
            ),
        })
    }
}
fn ground(term: &Term) -> bool {
    match term {
        Term::Var(_) => false,
        Term::App(_, xs) => xs.iter().all(ground),
    }
}
fn matches(
    pattern: &Term,
    value: &Term,
    bindings: &mut Bindings,
    budget: &mut Budget,
) -> Option<bool> {
    match (pattern, value) {
        (Term::Var(v), value) => {
            bindings.insert(*v, budget.copy(value)?);
            Some(true)
        }
        (Term::App(n, ps), Term::App(m, xs)) => {
            if n != m || ps.len() != xs.len() {
                return Some(false);
            }
            for (p, x) in ps.iter().zip(xs) {
                if !matches(p, x, bindings, budget)? {
                    return Some(false);
                }
            }
            Some(true)
        }
        _ => Some(false),
    }
}
fn match_inputs(clause: &Clause, args: &[Term], budget: &mut Budget) -> Option<Option<Bindings>> {
    let mut env = Bindings::new();
    for (p, x) in clause.inputs.iter().zip(args) {
        if !matches(p, x, &mut env, budget)? {
            return Some(None);
        }
    }
    Some(Some(env))
}
fn term(t: &Term, env: &mut Bindings, budget: &mut Budget) -> Option<Term> {
    match t {
        Term::Var(v) => {
            if let Some(t) = env.get(v) {
                return budget.copy(t);
            }
            budget.node()?;
            let fresh = Var(budget.fresh);
            budget.fresh += 1;
            env.insert(*v, Term::Var(fresh));
            Some(Term::Var(fresh))
        }
        Term::App(n, xs) => {
            budget.node()?;
            Some(Term::App(
                n.clone(),
                xs.iter()
                    .map(|x| term(x, env, budget))
                    .collect::<Option<_>>()?,
            ))
        }
    }
}
fn instantiate(plan: &Plan, env: &mut Bindings, budget: &mut Budget) -> Option<Plan> {
    budget.node()?;
    Some(match plan {
        Plan::Value(t) => Plan::Value(term(t, env, budget)?),
        Plan::Call(n, xs) => Plan::Call(
            n.clone(),
            xs.iter()
                .map(|x| term(x, env, budget))
                .collect::<Option<_>>()?,
        ),
        Plan::Choice(a, b) => Plan::Choice(
            Box::new(instantiate(a, env, budget)?),
            Box::new(instantiate(b, env, budget)?),
        ),
        Plan::Fail => Plan::Fail,
        Plan::Producers(actions, last) => {
            let actions = actions
                .iter()
                .map(|a| {
                    budget.node()?;
                    Some(match a {
                        Action::Post(c) => Action::Post(Constraint {
                            name: c.name.clone(),
                            args: c
                                .args
                                .iter()
                                .map(|x| term(x, env, budget))
                                .collect::<Option<_>>()?,
                        }),
                        Action::Call(out, n, args) => {
                            let Term::Var(out) = term(&Term::Var(*out), env, budget)? else {
                                unreachable!("fresh producer output")
                            };
                            Action::Call(
                                out,
                                n.clone(),
                                args.iter()
                                    .map(|x| term(x, env, budget))
                                    .collect::<Option<_>>()?,
                            )
                        }
                    })
                })
                .collect::<Option<_>>()?;
            Plan::Producers(actions, Box::new(instantiate(last, env, budget)?))
        }
    })
}
fn follow(plan: Plan, clauses: &[Clause], fuel: &mut usize, budget: &mut Budget) -> Plan {
    match plan {
        Plan::Call(n, args) if *fuel > 0 && budget.nodes > 0 && args.iter().all(ground) => {
            for clause in clauses
                .iter()
                .filter(|c| c.name == n && c.inputs.len() == args.len())
            {
                match match_inputs(clause, &args, budget) {
                    Some(Some(mut env)) => {
                        // A template is not authority to consume a resource.
                        if !clause.partners.is_empty() {
                            break;
                        }
                        let Some(body) = instantiate(&clause.body, &mut env, budget) else {
                            break;
                        };
                        *fuel -= 1;
                        return follow(body, clauses, fuel, budget);
                    }
                    Some(None) => {}
                    None => break,
                }
            }
            Plan::Call(n, args)
        }
        Plan::Choice(a, b) => Plan::Choice(
            Box::new(follow(*a, clauses, fuel, budget)),
            Box::new(follow(*b, clauses, fuel, budget)),
        ),
        // Prefix producer/effect operations retain their live execution obligations.
        other => other,
    }
}
pub(super) fn derive(clause: &Clause, args: &[Term], clauses: &[Clause]) -> Option<Template> {
    let mut budget = Budget {
        nodes: 16384,
        fresh: 0,
    };
    let mut env = match_inputs(clause, args, &mut budget)??;
    let body = instantiate(&clause.body, &mut env, &mut budget)?;
    let mut fuel = 64;
    let body = follow(body, clauses, &mut fuel, &mut budget);
    Some(Template {
        body,
        followed_calls: 64 - fuel,
    })
}
