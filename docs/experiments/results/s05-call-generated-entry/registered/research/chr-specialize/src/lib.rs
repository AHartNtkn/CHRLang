//! Bounded query-entry specialization for a certified single-active-call fragment.
use chr_syntax::{Constraint, Goal, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
type Predicate = (String, usize);
type Environment = BTreeMap<Var, Term>;
fn predicate(c: &Constraint) -> Predicate {
    (c.name.clone(), c.args.len())
}
#[derive(Default, Debug)]
pub struct Stats {
    pub expansions: usize,
    pub contradictions: usize,
    pub identities: usize,
    pub variants: usize,
    pub generated_goals: usize,
}
pub struct Program {
    pub rules: Vec<Rule>,
    pub query: Query,
    pub stats: Stats,
}
fn calls(g: &Goal, out: &mut Vec<Predicate>) {
    match g {
        Goal::Constraint(c) => out.push(predicate(c)),
        Goal::And(gs) => {
            for g in gs {
                calls(g, out);
            }
        }
        Goal::Or(a, b) => {
            calls(a, out);
            calls(b, out);
        }
        _ => {}
    }
}
fn certificate(rules: &[Rule], query: &Query) -> Result<BTreeMap<Predicate, usize>, String> {
    let mut queue = query
        .constraints
        .iter()
        .map(predicate)
        .collect::<VecDeque<_>>();
    let mut seen = BTreeSet::new();
    let mut definitions = BTreeMap::new();
    while let Some(p) = queue.pop_front() {
        if !seen.insert(p.clone()) {
            continue;
        }
        let matches = rules
            .iter()
            .enumerate()
            .filter(|(_, r)| r.kept.iter().chain(&r.removed).any(|c| predicate(c) == p))
            .collect::<Vec<_>>();
        if matches.is_empty() {
            continue;
        }
        if matches.len() != 1 {
            return Err(format!("multiple consumers of {p:?}"));
        }
        let (index, r) = matches[0];
        if !r.kept.is_empty() || r.removed.len() != 1 || !r.guards.is_empty() {
            return Err(format!("not a single unguarded simplification: {p:?}"));
        }
        let mut vars = BTreeSet::new();
        for t in &r.removed[0].args {
            let Term::Var(v) = t else {
                return Err(format!("constructor head: {p:?}"));
            };
            if !vars.insert(*v) {
                return Err(format!("nonlinear head: {p:?}"));
            }
        }
        definitions.insert(p, index);
        let mut next = vec![];
        calls(&r.body, &mut next);
        queue.extend(next);
    }
    fn tail(g: &Goal, defs: &BTreeMap<Predicate, usize>) -> Result<bool, String> {
        match g {
            Goal::Constraint(c) => Ok(defs.contains_key(&predicate(c))),
            Goal::And(gs) => {
                let mut active = false;
                for g in gs {
                    if active && !matches!(g, Goal::True) {
                        return Err("work after an active call is not tail-position".into());
                    }
                    active |= tail(g, defs)?;
                }
                Ok(active)
            }
            Goal::Or(a, b) => Ok(tail(a, defs)? | tail(b, defs)?),
            _ => Ok(false),
        }
    }
    for &index in definitions.values() {
        tail(&rules[index].body, &definitions)?;
    }
    if query
        .constraints
        .iter()
        .filter(|c| definitions.contains_key(&predicate(c)))
        .count()
        > 1
    {
        return Err("multiple active query calls".into());
    }
    Ok(definitions)
}
fn resolve(t: &Term, env: &Environment) -> Term {
    match t {
        Term::Var(v) => env.get(v).map_or_else(|| t.clone(), |x| resolve(x, env)),
        Term::App(n, args) => Term::App(n.clone(), args.iter().map(|a| resolve(a, env)).collect()),
    }
}
fn unify(a: Term, b: Term, env: &mut Environment) -> bool {
    fn occurs(v: Var, t: &Term) -> bool {
        match t {
            Term::Var(w) => v == *w,
            Term::App(_, args) => args.iter().any(|a| occurs(v, a)),
        }
    }
    let mut stack = vec![(a, b)];
    while let Some((a, b)) = stack.pop() {
        let a = resolve(&a, env);
        let b = resolve(&b, env);
        if a == b {
            continue;
        }
        match (a, b) {
            (Term::Var(v), t) | (t, Term::Var(v)) => {
                if occurs(v, &t) {
                    return false;
                }
                env.insert(v, t);
            }
            (Term::App(a, x), Term::App(b, y)) => {
                if a != b || x.len() != y.len() {
                    return false;
                }
                stack.extend(x.into_iter().zip(y));
            }
        }
    }
    true
}
fn fresh(t: &Term, scope: &mut Environment, next: &mut u64) -> Term {
    match t {
        Term::Var(v) => scope
            .entry(*v)
            .or_insert_with(|| {
                let t = Term::Var(Var(*next));
                *next += 1;
                t
            })
            .clone(),
        Term::App(n, args) => Term::App(
            n.clone(),
            args.iter().map(|a| fresh(a, scope, next)).collect(),
        ),
    }
}
fn instantiate(g: &Goal, scope: &mut Environment, next: &mut u64) -> Goal {
    match g {
        Goal::Constraint(c) => Goal::Constraint(Constraint {
            name: c.name.clone(),
            args: c.args.iter().map(|a| fresh(a, scope, next)).collect(),
        }),
        Goal::Unify(a, b) => Goal::Unify(fresh(a, scope, next), fresh(b, scope, next)),
        Goal::And(gs) => Goal::And(gs.iter().map(|g| instantiate(g, scope, next)).collect()),
        Goal::Or(a, b) => Goal::Or(
            Box::new(instantiate(a, scope, next)),
            Box::new(instantiate(b, scope, next)),
        ),
        _ => g.clone(),
    }
}
struct Compiler<'a> {
    rules: &'a [Rule],
    definitions: BTreeMap<Predicate, usize>,
    remaining: usize,
    next: u64,
    stats: Stats,
}
impl Compiler<'_> {
    fn goal(&mut self, g: &Goal, env: &mut Environment) -> Goal {
        match g {
            Goal::Constraint(c) => {
                let c = Constraint {
                    name: c.name.clone(),
                    args: c.args.iter().map(|a| resolve(a, env)).collect(),
                };
                if self.remaining > 0
                    && let Some(&index) = self.definitions.get(&predicate(&c))
                {
                    self.remaining -= 1;
                    self.stats.expansions += 1;
                    let r = &self.rules[index];
                    let mut scope = r.removed[0]
                        .args
                        .iter()
                        .zip(&c.args)
                        .map(|(p, a)| {
                            let Term::Var(v) = p else { unreachable!() };
                            (*v, a.clone())
                        })
                        .collect();
                    let body = instantiate(&r.body, &mut scope, &mut self.next);
                    return self.goal(&body, env);
                }
                Goal::Constraint(c)
            }
            Goal::Unify(a, b) => {
                let a = resolve(a, env);
                let b = resolve(b, env);
                if a == b {
                    self.stats.identities += 1;
                    return Goal::True;
                }
                let mut trial = env.clone();
                if unify(a.clone(), b.clone(), &mut trial) {
                    *env = trial;
                    Goal::Unify(a, b)
                } else {
                    self.stats.contradictions += 1;
                    Goal::Fail
                }
            }
            Goal::And(gs) => {
                let mut result = vec![];
                for g in gs {
                    let g = self.goal(g, env);
                    let failed = matches!(g, Goal::Fail);
                    if !matches!(g, Goal::True) {
                        result.push(g);
                    }
                    if failed {
                        break;
                    }
                }
                if result.is_empty() {
                    Goal::True
                } else {
                    Goal::And(result)
                }
            }
            Goal::Or(a, b) => Goal::Or(
                Box::new(self.goal(a, &mut env.clone())),
                Box::new(self.goal(b, &mut env.clone())),
            ),
            _ => g.clone(),
        }
    }
}
fn goal_size(g: &Goal) -> usize {
    1 + match g {
        Goal::And(gs) => gs.iter().map(goal_size).sum(),
        Goal::Or(a, b) => goal_size(a) + goal_size(b),
        _ => 0,
    }
}
pub fn specialize(rules: &[Rule], query: &Query, budget: usize) -> Result<Program, String> {
    let definitions = certificate(rules, query)?;
    let Some(index) = query
        .constraints
        .iter()
        .position(|c| definitions.contains_key(&predicate(c)))
    else {
        return Ok(Program {
            rules: rules.to_vec(),
            query: query.clone(),
            stats: Stats::default(),
        });
    };
    if budget == 0 {
        return Ok(Program {
            rules: rules.to_vec(),
            query: query.clone(),
            stats: Stats::default(),
        });
    }
    let mut names = query
        .constraints
        .iter()
        .map(|c| c.name.clone())
        .collect::<BTreeSet<_>>();
    for r in rules {
        names.extend(r.kept.iter().chain(&r.removed).map(|c| c.name.clone()));
        let mut cs = vec![];
        calls(&r.body, &mut cs);
        names.extend(cs.into_iter().map(|p| p.0));
    }
    let name = (0..)
        .map(|i| format!("__spec_{i}"))
        .find(|n| !names.contains(n))
        .unwrap();
    let rule_name = (0..)
        .map(|i| format!("__spec_rule_{i}"))
        .find(|n| rules.iter().all(|r| r.name != *n))
        .unwrap();
    let source = &query.constraints[index];
    let mut scope = Environment::new();
    let mut next = 0;
    let args = source
        .args
        .iter()
        .map(|a| fresh(a, &mut scope, &mut next))
        .collect::<Vec<_>>();
    let call = Goal::Constraint(Constraint {
        name: source.name.clone(),
        args: args.clone(),
    });
    let mut compiler = Compiler {
        rules,
        definitions,
        remaining: budget,
        next,
        stats: Stats::default(),
    };
    let body = compiler.goal(&call, &mut Environment::new());
    compiler.stats.variants = 1;
    compiler.stats.generated_goals = goal_size(&body);
    let mut result = vec![Rule::simplify(
        &rule_name,
        [Constraint {
            name: name.clone(),
            args,
        }],
        body,
    )];
    result.extend_from_slice(rules);
    let mut q = query.clone();
    q.constraints[index].name = name;
    Ok(Program {
        rules: result,
        query: q,
        stats: compiler.stats,
    })
}
