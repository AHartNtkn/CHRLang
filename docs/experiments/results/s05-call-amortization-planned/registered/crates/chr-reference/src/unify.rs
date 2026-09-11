use chr_syntax::{Term, Var};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Substitution(BTreeMap<Var, Term>);
impl Substitution {
    pub(crate) fn resolve(&self, term: &Term) -> Term {
        match term {
            Term::Var(var) => match self.0.get(var) {
                Some(value) => self.resolve(value),
                None => term.clone(),
            },
            Term::App(name, args) => {
                Term::App(name.clone(), args.iter().map(|a| self.resolve(a)).collect())
            }
        }
    }

    /// All changes are private until the complete equation has succeeded.
    pub(crate) fn unify(&mut self, left: &Term, right: &Term, pairs: &mut u64) -> bool {
        let mut trial = self.clone();
        let mut pending = vec![(left.clone(), right.clone())];
        while let Some((left, right)) = pending.pop() {
            *pairs += 1;
            let left = trial.resolve(&left);
            let right = trial.resolve(&right);
            if left == right {
                continue;
            }
            match (left, right) {
                (Term::Var(var), term) | (term, Term::Var(var)) => {
                    if occurs(var, &term) {
                        return false;
                    }
                    trial.0.insert(var, term);
                }
                (Term::App(f, xs), Term::App(g, ys)) => {
                    if f != g || xs.len() != ys.len() {
                        return false;
                    }
                    pending.extend(xs.into_iter().zip(ys).rev());
                }
            }
        }
        *self = trial;
        true
    }
}

fn occurs(var: Var, term: &Term) -> bool {
    match term {
        Term::Var(other) => var == *other,
        Term::App(_, args) => args.iter().any(|a| occurs(var, a)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chr_syntax::{atom, t, v};

    #[test]
    fn aliases_propagate_through_nested_terms() {
        let mut s = Substitution::default();
        assert!(s.unify(&v(0), &t("f", [v(1)]), &mut 0));
        assert!(s.unify(&v(1), &v(2), &mut 0));
        assert!(s.unify(&v(2), &atom("a"), &mut 0));
        assert_eq!(s.resolve(&v(0)), t("f", [atom("a")]));
    }
    #[test]
    fn failed_equation_does_not_publish_partial_bindings() {
        let mut s = Substitution::default();
        let before = s.clone();
        assert!(!s.unify(
            &t("p", [v(0), atom("a")]),
            &t("p", [atom("b"), atom("c")]),
            &mut 0
        ));
        assert_eq!(s, before);
    }
    #[test]
    fn indirect_occurs_cycle_fails_but_self_equality_succeeds() {
        let mut s = Substitution::default();
        assert!(s.unify(&v(0), &t("s", [v(1)]), &mut 0));
        let before = s.clone();
        assert!(!s.unify(&v(1), &v(0), &mut 0));
        assert_eq!(s, before);
        assert!(s.unify(&v(1), &v(1), &mut 0));
    }
    #[test]
    fn same_constructor_decomposes_and_arity_mismatch_fails() {
        let mut s = Substitution::default();
        assert!(s.unify(&t("f", [v(0)]), &t("f", [atom("z")]), &mut 0));
        assert_eq!(s.resolve(&v(0)), atom("z"));
        assert!(!s.unify(&t("f", []), &t("f", [v(1)]), &mut 0));
    }
}
