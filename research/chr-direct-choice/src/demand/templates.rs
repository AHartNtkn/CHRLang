//! Input-dependent residual source plans, containing no runtime graph identities.
use super::{Action, Clause, Plan};
use chr_syntax::{Term, Var};
use std::collections::BTreeMap;
use std::rc::Rc;

pub(super) type Expr = Rc<ExprNode>;
pub(super) struct ExprNode {
    pub id: usize,
    pub kind: ExprKind,
    pub ground: bool,
}
pub(super) enum ExprKind {
    Local(Var),
    App(String, Vec<Expr>),
}
type Bindings = BTreeMap<Var, Expr>;
pub(super) struct Template {
    pub body: Plan<Expr>,
    pub followed_calls: usize,
}
struct Budget {
    next_expr: usize,
    nodes: usize,
    fresh: u64,
}
impl Budget {
    fn expression(&mut self, kind: ExprKind, ground: bool) -> Expr {
        let id = self.next_expr;
        self.next_expr += 1;
        Rc::new(ExprNode { id, kind, ground })
    }
    fn node(&mut self) -> Option<()> {
        self.nodes = self.nodes.checked_sub(1)?;
        Some(())
    }
    fn app(&mut self, name: String, args: Vec<Expr>) -> Option<Expr> {
        self.node()?;
        let ground = args.iter().all(|x| x.ground);
        Some(self.expression(ExprKind::App(name, args), ground))
    }
}
fn matches(
    pattern: &Term,
    value: &Expr,
    bindings: &mut Bindings,
    budget: &mut Budget,
) -> Option<bool> {
    match (pattern, &value.kind) {
        (Term::Var(v), _) => {
            budget.node()?;
            bindings.insert(*v, value.clone());
            Some(true)
        }
        (Term::App(n, ps), ExprKind::App(m, xs)) => {
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
fn match_inputs(clause: &Clause, args: &[Expr], budget: &mut Budget) -> Option<Option<Bindings>> {
    let mut env = Bindings::new();
    for (p, x) in clause.inputs.iter().zip(args) {
        if !matches(p, x, &mut env, budget)? {
            return Some(None);
        }
    }
    Some(Some(env))
}
fn term(t: &Term, env: &mut Bindings, budget: &mut Budget) -> Option<Expr> {
    match t {
        Term::Var(v) => {
            if let Some(t) = env.get(v) {
                // A shared edge still occupies a slot in the residual plan.
                budget.node()?;
                return Some(t.clone());
            }
            budget.node()?;
            let fresh = Var(budget.fresh);
            budget.fresh += 1;
            let value = budget.expression(ExprKind::Local(fresh), false);
            env.insert(*v, value.clone());
            Some(value)
        }
        Term::App(n, xs) => {
            let args = xs
                .iter()
                .map(|x| term(x, env, budget))
                .collect::<Option<_>>()?;
            budget.app(n.clone(), args)
        }
    }
}
fn instantiate(plan: &Plan, env: &mut Bindings, budget: &mut Budget) -> Option<Plan<Expr>> {
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
        Plan::BindOutput(out, body) => {
            let local = term(&Term::Var(*out), env, budget)?;
            let ExprKind::Local(out) = local.kind else {
                unreachable!("distinct output binder");
            };
            Plan::BindOutput(out, Box::new(instantiate(body, env, budget)?))
        }
        Plan::Producers(actions, last) => {
            let actions = actions
                .iter()
                .map(|a| {
                    budget.node()?;
                    Some(match a {
                        Action::Post(name, args) => Action::Post(
                            name.clone(),
                            args.iter()
                                .map(|x| term(x, env, budget))
                                .collect::<Option<_>>()?,
                        ),
                        Action::Call(out, n, args) => {
                            let local = term(&Term::Var(*out), env, budget)?;
                            let ExprKind::Local(out) = &local.kind else {
                                unreachable!("fresh producer output")
                            };
                            Action::Call(
                                *out,
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
fn follow(
    plan: Plan<Expr>,
    clauses: &[Clause],
    fuel: &mut usize,
    budget: &mut Budget,
) -> Plan<Expr> {
    match plan {
        Plan::Call(n, args) if *fuel > 0 && budget.nodes > 0 && args.iter().all(|x| x.ground) => {
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
        next_expr: 0,
        nodes: 16384,
        fresh: 0,
    };
    let args = args
        .iter()
        .map(|x| term(x, &mut Bindings::new(), &mut budget))
        .collect::<Option<Vec<_>>>()?;
    let mut env = match_inputs(clause, &args, &mut budget)??;
    let body = instantiate(&clause.body, &mut env, &mut budget)?;
    let mut fuel = 64;
    let body = follow(body, clauses, &mut fuel, &mut budget);
    Some(Template {
        body,
        followed_calls: 64 - fuel,
    })
}

#[cfg(test)]
mod identity_tests {
    use super::*;
    #[test]
    fn expression_identity_is_unique_and_preserves_shared_edges() {
        let mut budget = Budget {
            next_expr: 0,
            nodes: 16384,
            fresh: 0,
        };
        let a = budget.app("a".into(), vec![]).unwrap();
        let shared = a.clone();
        let separate = budget.app("a".into(), vec![]).unwrap();
        let parent = budget
            .app("p".into(), vec![shared.clone(), separate.clone()])
            .unwrap();
        assert_eq!(a.id, shared.id);
        assert_eq!([a.id, separate.id, parent.id], [0, 1, 2]);
    }
}
