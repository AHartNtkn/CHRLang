//! Small independent owned-syntax scalar semantics for the runtime gate.
use chr_syntax::{Answer, Constraint, Goal, Guard, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
type Bindings = BTreeMap<Var, Term>;
fn resolve(term: &Term, bindings: &Bindings) -> Term {
    match term {
        Term::Var(v) => bindings
            .get(v)
            .map_or_else(|| term.clone(), |t| resolve(t, bindings)),
        Term::App(n, xs) => Term::App(n.clone(), xs.iter().map(|x| resolve(x, bindings)).collect()),
    }
}
fn occurs(variable: Var, term: &Term) -> bool {
    match term {
        Term::Var(v) => *v == variable,
        Term::App(_, xs) => xs.iter().any(|x| occurs(variable, x)),
    }
}
fn unify(a: Term, b: Term, bindings: &mut Bindings) -> bool {
    let mut pending = vec![(a, b)];
    while let Some((a, b)) = pending.pop() {
        let a = resolve(&a, bindings);
        let b = resolve(&b, bindings);
        if a == b {
            continue;
        }
        match (a, b) {
            (Term::Var(v), t) | (t, Term::Var(v)) => {
                if occurs(v, &t) {
                    return false;
                }
                bindings.insert(v, t);
            }
            (Term::App(a, x), Term::App(b, y)) => {
                if a != b || x.len() != y.len() {
                    return false;
                }
                pending.extend(x.into_iter().zip(y));
            }
        }
    }
    true
}
fn match_term(pattern: &Term, value: &Term, slots: &mut Bindings) -> bool {
    match pattern {
        Term::Var(v) => match slots.get(v) {
            Some(old) => old == value,
            None => {
                slots.insert(*v, value.clone());
                true
            }
        },
        Term::App(n, xs) => match value {
            Term::App(m, ys) => {
                n == m
                    && xs.len() == ys.len()
                    && xs.iter().zip(ys).all(|(p, t)| match_term(p, t, slots))
            }
            _ => false,
        },
    }
}
fn instantiate(term: &Term, slots: &mut Bindings, next: &mut u64) -> Term {
    match term {
        Term::Var(v) => slots
            .entry(*v)
            .or_insert_with(|| {
                let fresh = Term::Var(Var(*next));
                *next += 1;
                fresh
            })
            .clone(),
        Term::App(n, xs) => Term::App(
            n.clone(),
            xs.iter().map(|x| instantiate(x, slots, next)).collect(),
        ),
    }
}
fn body(goal: &Goal, slots: &mut Bindings, next: &mut u64) -> Goal {
    match goal {
        Goal::True => Goal::True,
        Goal::Fail => Goal::Fail,
        Goal::Constraint(c) => Goal::Constraint(Constraint {
            name: c.name.clone(),
            args: c.args.iter().map(|t| instantiate(t, slots, next)).collect(),
        }),
        Goal::Unify(a, b) => Goal::Unify(instantiate(a, slots, next), instantiate(b, slots, next)),
        Goal::And(xs) => Goal::And(xs.iter().map(|g| body(g, slots, next)).collect()),
        Goal::Or(a, b) => Goal::Or(
            Box::new(body(a, slots, next)),
            Box::new(body(b, slots, next)),
        ),
    }
}
#[derive(Clone)]
struct State {
    live: Vec<(u64, Constraint)>,
    bindings: Bindings,
    history: BTreeSet<(usize, Vec<u64>)>,
    pending: Vec<Goal>,
    next_var: u64,
    next_occ: u64,
    trace: Vec<usize>,
}
struct Application {
    rule: usize,
    ids: Vec<u64>,
    body: Goal,
    next_var: u64,
}
fn tuple(
    state: &State,
    rule: &Rule,
    rule_id: usize,
    heads: &[&Constraint],
    ids: Vec<u64>,
    slots: Bindings,
) -> Option<Application> {
    if ids.len() == heads.len() {
        if rule.removed.is_empty() && state.history.contains(&(rule_id, ids.clone())) {
            return None;
        }
        let mut slots = slots;
        let mut next = state.next_var;
        for Guard::Equal(a, b) in &rule.guards {
            let a = instantiate(a, &mut slots, &mut next);
            let b = instantiate(b, &mut slots, &mut next);
            if resolve(&a, &state.bindings) != resolve(&b, &state.bindings) {
                return None;
            }
        }
        return Some(Application {
            rule: rule_id,
            ids,
            body: body(&rule.body, &mut slots, &mut next),
            next_var: next,
        });
    }
    let head = heads[ids.len()];
    for (id, actual) in &state.live {
        if ids.contains(id) || head.name != actual.name || head.args.len() != actual.args.len() {
            continue;
        }
        let mut extended = slots.clone();
        if !head
            .args
            .iter()
            .zip(&actual.args)
            .all(|(p, t)| match_term(p, &resolve(t, &state.bindings), &mut extended))
        {
            continue;
        }
        let mut chosen = ids.clone();
        chosen.push(*id);
        if let Some(found) = tuple(state, rule, rule_id, heads, chosen, extended) {
            return Some(found);
        }
    }
    None
}
fn max_var(term: &Term) -> Option<u64> {
    match term {
        Term::Var(Var(v)) => Some(*v),
        Term::App(_, xs) => xs.iter().filter_map(max_var).max(),
    }
}
/// Complete finite executions only. The caller supplies a bound and cannot turn
/// a cutoff into an empty or complete answer set.
pub fn run(rules: &[Rule], query: &Query, bound: usize) -> Vec<Answer> {
    run_traced(rules, query, bound)
        .into_iter()
        .map(|(answer, _)| answer)
        .collect()
}
pub fn run_traced(rules: &[Rule], query: &Query, bound: usize) -> Vec<(Answer, Vec<usize>)> {
    let next_var = query
        .constraints
        .iter()
        .flat_map(|c| &c.args)
        .filter_map(max_var)
        .chain(query.outputs.iter().map(|(_, Var(v))| *v))
        .max()
        .map_or(0, |v| v + 1);
    let initial = State {
        live: query
            .constraints
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, c)| (i as u64, c))
            .collect(),
        bindings: Bindings::new(),
        history: BTreeSet::new(),
        pending: vec![],
        trace: vec![],
        next_var,
        next_occ: query.constraints.len() as u64,
    };
    let mut frontier = VecDeque::from([initial]);
    let mut answers = vec![];
    for _ in 0..bound {
        let Some(mut state) = frontier.pop_front() else {
            return answers;
        };
        if let Some(goal) = state.pending.pop() {
            match goal {
                Goal::True => (),
                Goal::Fail => continue,
                Goal::Constraint(c) => {
                    state.live.push((state.next_occ, c));
                    state.next_occ += 1;
                }
                Goal::Unify(a, b) => {
                    if !unify(a, b, &mut state.bindings) {
                        continue;
                    }
                }
                Goal::And(xs) => state.pending.extend(xs.into_iter().rev()),
                Goal::Or(left, right) => {
                    let mut other = state.clone();
                    state.pending.push(*left);
                    other.pending.push(*right);
                    frontier.push_back(other);
                }
            }
            frontier.push_back(state);
            continue;
        }
        let application = rules.iter().enumerate().find_map(|(i, r)| {
            let heads: Vec<_> = r.kept.iter().chain(&r.removed).collect();
            tuple(&state, r, i, &heads, vec![], Bindings::new())
        });
        if let Some(app) = application {
            state.trace.push(app.rule);
            let rule = &rules[app.rule];
            let removed = &app.ids[rule.kept.len()..];
            state.live.retain(|(id, _)| !removed.contains(id));
            if rule.removed.is_empty() {
                state.history.insert((app.rule, app.ids));
            }
            state.next_var = app.next_var;
            state.pending.push(app.body);
            frontier.push_back(state);
        } else {
            answers.push((
                Answer {
                    outputs: query
                        .outputs
                        .iter()
                        .map(|(name, v)| (name.clone(), resolve(&Term::Var(*v), &state.bindings)))
                        .collect(),
                    residual: state
                        .live
                        .iter()
                        .map(|(_, c)| Constraint {
                            name: c.name.clone(),
                            args: c.args.iter().map(|t| resolve(t, &state.bindings)).collect(),
                        })
                        .collect(),
                },
                state.trace,
            ));
        }
    }
    panic!("independent scalar oracle cutoff");
}
pub fn same_raw(mut actual: Vec<Answer>, expected: Vec<Answer>) {
    assert_eq!(actual.len(), expected.len(), "raw multiplicity differs");
    for wanted in expected {
        let index = actual
            .iter()
            .position(|a| chr_observe::equivalent(a, &wanted, &mut Default::default()))
            .unwrap_or_else(|| panic!("missing full observation {wanted:?}; remaining {actual:?}"));
        actual.swap_remove(index);
    }
    assert!(actual.is_empty());
}
