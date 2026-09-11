//! Exact structural alpha equivalence, private to this implementation.
use chr_syntax::{Answer, Constraint, Term, Var};
use std::collections::{BTreeMap, BTreeSet};

type Renaming = BTreeMap<Var, Var>;

pub(crate) fn canonicalize(answer: Answer) -> Answer {
    let mut fixed = Renaming::new();
    for (_, term) in &answer.outputs {
        visit(term, &mut |v| {
            let next = Var(fixed.len() as u64);
            fixed.entry(v).or_insert(next);
        });
    }
    let mut remaining = BTreeSet::new();
    for constraint in &answer.residual {
        for term in &constraint.args {
            visit(term, &mut |v| {
                if !fixed.contains_key(&v) {
                    remaining.insert(v);
                }
            });
        }
    }
    let remaining = remaining.into_iter().collect::<Vec<_>>();
    let first = fixed.len() as u64;
    let mut best = None;
    enumerate(&answer, &remaining, &mut fixed, first, &mut best);
    best.expect("even an empty renaming has one assignment")
}

fn enumerate(
    answer: &Answer,
    remaining: &[Var],
    mapping: &mut Renaming,
    next: u64,
    best: &mut Option<Answer>,
) {
    if remaining.is_empty() {
        let mut candidate = Answer {
            outputs: answer
                .outputs
                .iter()
                .map(|(n, t)| (n.clone(), rename(t, mapping)))
                .collect(),
            residual: answer
                .residual
                .iter()
                .map(|c| Constraint {
                    name: c.name.clone(),
                    args: c.args.iter().map(|t| rename(t, mapping)).collect(),
                })
                .collect(),
        };
        candidate.residual.sort();
        if best.as_ref().is_none_or(|b| candidate < *b) {
            *best = Some(candidate);
        }
        return;
    }
    // Try every residual-variable ordering. Output variables are already fixed.
    for var in remaining {
        mapping.insert(*var, Var(next));
        let rest = remaining
            .iter()
            .filter(|v| *v != var)
            .copied()
            .collect::<Vec<_>>();
        enumerate(answer, &rest, mapping, next + 1, best);
        mapping.remove(var);
    }
}
fn visit(term: &Term, action: &mut impl FnMut(Var)) {
    match term {
        Term::Var(v) => action(*v),
        Term::App(_, args) => {
            for t in args {
                visit(t, action);
            }
        }
    }
}
fn rename(term: &Term, mapping: &Renaming) -> Term {
    match term {
        Term::Var(v) => Term::Var(mapping[v]),
        Term::App(f, args) => {
            Term::App(f.clone(), args.iter().map(|t| rename(t, mapping)).collect())
        }
    }
}
