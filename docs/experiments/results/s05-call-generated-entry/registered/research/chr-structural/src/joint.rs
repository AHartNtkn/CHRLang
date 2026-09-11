//! Experimental logical conjunction; caller equality and raw CHR occurrences remain external.
use crate::{
    name_disequality::{Formula, Name},
    names::{NameFormula, Theory},
    normal_forms::{Requirement, Summary},
};
use chr_syntax::{Term, Var};
use std::collections::{BTreeMap, BTreeSet};
/// Existential feasibility under a caller substitution, without modifying that substitution.
/// The alphabet restricts name obligations/exclusion endpoints, not unrelated structural leaves.
/// Caller equality must already supply a consistent substitution; this is not a unifier.
pub fn feasible(
    names: &[Term],
    structure: &[(Term, Requirement)],
    unequal: &[(Term, Term)],
    bindings: &BTreeMap<Var, Term>,
    alphabet: Option<&[String]>,
) -> Result<bool, String> {
    fn expand(
        t: &Term,
        b: &BTreeMap<Var, Term>,
        active: &mut BTreeSet<Var>,
    ) -> Result<Term, String> {
        match t {
            Term::Var(x) => match b.get(x) {
                None => Ok(t.clone()),
                Some(value) if value == t => Ok(t.clone()),
                Some(value) => {
                    if !active.insert(*x) {
                        return Err("cyclic caller substitution in joint requirement".into());
                    }
                    let result = expand(value, b, active);
                    active.remove(x);
                    result
                }
            },
            Term::App(n, a) => Ok(Term::App(
                n.clone(),
                a.iter()
                    .map(|t| expand(t, b, active))
                    .collect::<Result<_, _>>()?,
            )),
        }
    }
    let resolve = |t: &Term| expand(t, bindings, &mut BTreeSet::new());
    let mut names = names.iter().map(resolve).collect::<Result<Vec<_>, _>>()?;
    let structure = structure
        .iter()
        .map(|(t, r)| Ok((resolve(t)?, *r)))
        .collect::<Result<Vec<_>, String>>()?;
    let unequal = unequal
        .iter()
        .map(|(a, b)| Ok((resolve(a)?, resolve(b)?)))
        .collect::<Result<Vec<_>, String>>()?;
    for (a, b) in &unequal {
        names.push(a.clone());
        names.push(b.clone());
    }
    let Some(name_summary) = NameFormula::compile(Theory::AtomicNames, &names) else {
        return Ok(false);
    };
    let Some(_structural_summary) = Summary::compile(&structure) else {
        return Ok(false);
    };
    // Each name is normal and neutral. Every remaining structural variable can be an atom.
    // Thus the structural component adds no exclusion between otherwise unknown names.
    let ids = name_summary
        .variables()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<BTreeMap<_, _>>();
    let endpoint = |t: &Term| match t {
        Term::Var(x) => Name::Variable(ids[x]),
        Term::App(n, a) => {
            debug_assert!(a.is_empty());
            Name::Atom(n.clone())
        }
    };
    // Reflexive obligations retain fixed names and unconstrained name variables in the domain.
    let equal = names
        .iter()
        .map(|t| {
            let n = endpoint(t);
            (n.clone(), n)
        })
        .collect::<Vec<_>>();
    let unequal = unequal
        .iter()
        .map(|(a, b)| (endpoint(a), endpoint(b)))
        .collect::<Vec<_>>();
    let Some(formula) = Formula::compile(ids.len(), &equal, &unequal) else {
        return Ok(false);
    };
    Ok(match alphabet {
        Some(a) => formula.finite(a, &BTreeMap::new()).satisfiable,
        None => formula.unbounded(&BTreeMap::new()),
    })
}
