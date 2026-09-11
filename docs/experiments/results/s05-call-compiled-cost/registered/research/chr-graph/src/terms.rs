use crate::Stats;
use chr_syntax::{Term, Var};
use std::collections::BTreeMap;
pub(crate) type Bindings = BTreeMap<Var, Term>;
pub(crate) type Scope = BTreeMap<Var, Term>;
pub(crate) fn deref<'a>(mut t: &'a Term, bindings: &'a Bindings, stats: &mut Stats) -> &'a Term {
    while let Term::Var(v) = t {
        stats.binding_reads += 1;
        if let Some(next) = bindings.get(v) {
            t = next;
        } else {
            break;
        }
    }
    t
}
pub(crate) fn resolve(t: &Term, bindings: &Bindings, stats: &mut Stats) -> Term {
    stats.term_visits += 1;
    match deref(t, bindings, stats) {
        Term::Var(v) => Term::Var(*v),
        Term::App(n, args) => Term::App(
            n.clone(),
            args.iter().map(|t| resolve(t, bindings, stats)).collect(),
        ),
    }
}
pub(crate) fn equal(a: &Term, b: &Term, bindings: &Bindings, stats: &mut Stats) -> bool {
    stats.term_visits += 1;
    match (deref(a, bindings, stats), deref(b, bindings, stats)) {
        (Term::Var(a), Term::Var(b)) => a == b,
        (Term::App(a, x), Term::App(b, y)) => {
            a == b
                && x.len() == y.len()
                && x.iter().zip(y).all(|(x, y)| equal(x, y, bindings, stats))
        }
        _ => false,
    }
}
pub(crate) fn unify(a: &Term, b: &Term, bindings: &mut Bindings, stats: &mut Stats) -> bool {
    // A context-local transaction. Every speculative entry is charged and published only on success.
    stats.snapshot_entries += bindings.len() as u64;
    let mut trial = bindings.clone();
    let mut work = vec![(a.clone(), b.clone())];
    while let Some((a, b)) = work.pop() {
        stats.unification_pairs += 1;
        match (
            deref(&a, &trial, stats).clone(),
            deref(&b, &trial, stats).clone(),
        ) {
            (Term::Var(a), Term::Var(b)) if a == b => {}
            (Term::Var(a), Term::Var(b)) => {
                let (a, b) = if a > b { (a, b) } else { (b, a) };
                trial.insert(a, Term::Var(b));
            }
            (Term::Var(v), t) | (t, Term::Var(v)) => {
                let mut todo = vec![&t];
                while let Some(t) = todo.pop() {
                    stats.occurs_visits += 1;
                    match deref(t, &trial, stats) {
                        Term::Var(w) if *w == v => return false,
                        Term::App(_, args) => todo.extend(args),
                        _ => {}
                    }
                }
                trial.insert(v, t);
            }
            (Term::App(a, x), Term::App(b, y)) => {
                if a != b || x.len() != y.len() {
                    return false;
                }
                work.extend(x.into_iter().zip(y));
            }
        }
    }
    *bindings = trial;
    true
}
