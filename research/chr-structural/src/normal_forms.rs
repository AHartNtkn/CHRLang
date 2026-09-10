//! Experimental regular structural requirements; not literal CHR normalization.
use chr_syntax::{Term, Var};
use std::collections::{BTreeMap, BTreeSet};

/// Ordered by strength: Neutral is a subset of Normal, which is a subset of Domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Requirement {
    Domain,
    Normal,
    Neutral,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Summary {
    pending: BTreeMap<Var, Requirement>,
}
impl Summary {
    pub fn compile(obligations: &[(Term, Requirement)]) -> Option<Self> {
        let mut pending = BTreeMap::new();
        let mut work: Vec<_> = obligations.iter().map(|(t, r)| (t, *r)).collect();
        while let Some((term, required)) = work.pop() {
            match term {
                Term::Var(x) => {
                    pending
                        .entry(*x)
                        .and_modify(|r| *r = std::cmp::max(*r, required))
                        .or_insert(required);
                }
                Term::App(_, args) if args.is_empty() => (),
                Term::App(name, args) if name == "app" && args.len() == 2 => {
                    let (left, right) = if required == Requirement::Domain {
                        (Requirement::Domain, Requirement::Domain)
                    } else {
                        (Requirement::Neutral, Requirement::Normal)
                    };
                    work.push((&args[0], left));
                    work.push((&args[1], right));
                }
                Term::App(name, args) if name == "lam" && args.len() == 2 => {
                    if required == Requirement::Neutral {
                        return None;
                    }
                    work.push((&args[0], Requirement::Domain));
                    work.push((&args[1], required));
                }
                _ => return None,
            }
        }
        Some(Self { pending })
    }
    pub fn variables(&self) -> impl Iterator<Item = (Var, Requirement)> + '_ {
        self.pending.iter().map(|(v, r)| (*v, *r))
    }
    /// Refine using finite-tree caller bindings, detecting cycles in their expansion.
    /// This does not own the caller's equality store or invent constructor assignments.
    pub fn refine(&self, bindings: &BTreeMap<Var, Term>) -> Result<Option<Self>, String> {
        fn expand(
            t: &Term,
            bindings: &BTreeMap<Var, Term>,
            active: &mut BTreeSet<Var>,
        ) -> Result<Term, String> {
            match t {
                Term::Var(x) => match bindings.get(x) {
                    None => Ok(t.clone()),
                    Some(value) => {
                        if value == t {
                            return Ok(t.clone());
                        }
                        if !active.insert(*x) {
                            return Err("cyclic finite-tree binding".into());
                        }
                        let result = expand(value, bindings, active);
                        active.remove(x);
                        result
                    }
                },
                Term::App(n, args) => Ok(Term::App(
                    n.clone(),
                    args.iter()
                        .map(|t| expand(t, bindings, active))
                        .collect::<Result<_, _>>()?,
                )),
            }
        }
        let mut obligations = vec![];
        for (x, r) in &self.pending {
            obligations.push((expand(&Term::Var(*x), bindings, &mut BTreeSet::new())?, *r));
        }
        Ok(Self::compile(&obligations))
    }
}
