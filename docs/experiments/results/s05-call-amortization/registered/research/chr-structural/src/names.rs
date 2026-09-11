//! Experimental logical name restrictions. These formulas are not CHR residual stores.
use chr_syntax::{Term, Var};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theory {
    /// Exactly the known-root rejection of the literal var rules.
    LiteralRoots,
    /// Declared names are nullary constructors; this is a different domain contract.
    AtomicNames,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameFormula {
    theory: Theory,
    pending: BTreeSet<Var>,
}
impl NameFormula {
    /// Compile a logical conjunction without choosing values for unknowns.
    pub fn compile(theory: Theory, obligations: &[Term]) -> Option<Self> {
        let mut pending = BTreeSet::new();
        for term in obligations {
            match term {
                Term::Var(x) => {
                    pending.insert(*x);
                }
                Term::App(name, args) => {
                    let rejected = match theory {
                        Theory::LiteralRoots => args.len() == 2 && (name == "app" || name == "lam"),
                        Theory::AtomicNames => !args.is_empty(),
                    };
                    if rejected {
                        return None;
                    }
                }
            }
        }
        Some(Self { theory, pending })
    }
    pub fn variables(&self) -> impl Iterator<Item = Var> + '_ {
        self.pending.iter().copied()
    }
    /// Apply caller bindings to restrictions. Cyclic root substitutions are invalid input.
    /// The caller owns finite-tree equality and must enforce its other occurs checks.
    pub fn refine(&self, bindings: &BTreeMap<Var, Term>) -> Result<Option<Self>, String> {
        let mut terms = Vec::with_capacity(self.pending.len());
        for start in &self.pending {
            let mut current = Term::Var(*start);
            let mut seen = BTreeSet::new();
            while let Term::Var(x) = current {
                let Some(value) = bindings.get(&x) else {
                    break;
                };
                if !seen.insert(x) {
                    return Err("cyclic root substitution".into());
                }
                current = value.clone();
            }
            terms.push(current);
        }
        Ok(Self::compile(self.theory, &terms))
    }
    /// Transport restricted variables bijectively onto their chosen caller identities.
    /// Freshness relative to the caller's other state is a separate caller obligation.
    pub fn transport(&self, mapping: &BTreeMap<Var, Var>) -> Result<Self, String> {
        let mut pending = BTreeSet::new();
        for x in &self.pending {
            let y = mapping.get(x).ok_or("missing caller variable")?;
            if !pending.insert(*y) {
                return Err("noninjective caller transport".into());
            }
        }
        Ok(Self {
            theory: self.theory,
            pending,
        })
    }
}
