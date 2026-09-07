use crate::{Stats, Support};
use chr_syntax::{Term, Var};
use std::collections::BTreeMap;
#[derive(Clone)]
pub(crate) struct Edge {
    pub target: Term,
    pub support: Support,
}
pub(crate) type Bindings = BTreeMap<Var, Vec<Edge>>;
pub(crate) type Delta = BTreeMap<Var, Term>;

pub(crate) fn copy(term: &Term, stats: &mut Stats) -> Term {
    stats.term_copies += 1;
    match term {
        Term::Var(v) => Term::Var(*v),
        Term::App(n, args) => Term::App(n.clone(), args.iter().map(|t| copy(t, stats)).collect()),
    }
}
pub(crate) fn deref(
    term: &Term,
    ticket: u64,
    bindings: &Bindings,
    local: &Delta,
    stats: &mut Stats,
) -> Term {
    let mut current = copy(term, stats);
    while let Term::Var(var) = current {
        if let Some(t) = local.get(&var) {
            current = copy(t, stats);
            continue;
        }
        let target = bindings.get(&var).and_then(|edges| {
            edges.iter().find(|e| {
                stats.binding_scans += 1;
                stats.support_reads += 1;
                e.support.contains(&ticket)
            })
        });
        if let Some(e) = target {
            current = copy(&e.target, stats);
        } else {
            break;
        }
    }
    current
}
pub(crate) fn resolved(term: &Term, ticket: u64, bindings: &Bindings, stats: &mut Stats) -> Term {
    match deref(term, ticket, bindings, &Delta::new(), stats) {
        Term::Var(v) => Term::Var(v),
        Term::App(n, args) => Term::App(
            n,
            args.iter()
                .map(|t| resolved(t, ticket, bindings, stats))
                .collect(),
        ),
    }
}
pub(crate) fn equal(
    left: &Term,
    right: &Term,
    ticket: u64,
    bindings: &Bindings,
    stats: &mut Stats,
) -> bool {
    let a = deref(left, ticket, bindings, &Delta::new(), stats);
    let b = deref(right, ticket, bindings, &Delta::new(), stats);
    stats.term_visits += 1;
    match (a, b) {
        (Term::Var(a), Term::Var(b)) => a == b,
        (Term::App(a, x), Term::App(b, y)) => {
            a == b
                && x.len() == y.len()
                && x.iter()
                    .zip(y)
                    .all(|(x, y)| equal(x, &y, ticket, bindings, stats))
        }
        _ => false,
    }
}
pub(crate) fn unify(
    left: &Term,
    right: &Term,
    ticket: u64,
    bindings: &Bindings,
    stats: &mut Stats,
) -> Option<Delta> {
    let mut local = Delta::new();
    let mut work = vec![(copy(left, stats), copy(right, stats))];
    while let Some((left, right)) = work.pop() {
        stats.unification_pairs += 1;
        let a = deref(&left, ticket, bindings, &local, stats);
        let b = deref(&right, ticket, bindings, &local, stats);
        match (a, b) {
            (Term::Var(a), Term::Var(b)) if a == b => {}
            (Term::Var(a), Term::Var(b)) => {
                let (child, parent) = if a > b { (a, b) } else { (b, a) };
                local.insert(child, Term::Var(parent));
            }
            (Term::Var(var), term) | (term, Term::Var(var)) => {
                let mut todo = vec![copy(&term, stats)];
                while let Some(t) = todo.pop() {
                    stats.occurs_visits += 1;
                    match deref(&t, ticket, bindings, &local, stats) {
                        Term::Var(v) if v == var => return None,
                        Term::App(_, args) => todo.extend(args),
                        _ => {}
                    }
                }
                local.insert(var, term);
            }
            (Term::App(a, x), Term::App(b, y)) => {
                if a != b || x.len() != y.len() {
                    return None;
                }
                work.extend(x.into_iter().zip(y));
            }
        }
    }
    Some(local)
}
