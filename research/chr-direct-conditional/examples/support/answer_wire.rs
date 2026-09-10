//! Owned binary answers plus the dictionaries required to interpret them.
use chr_syntax::{Answer, Constraint, Goal, Query, Rule, Term};
use std::collections::BTreeSet;
#[derive(Clone)]
pub struct Symbols {
    pub predicates: Vec<String>,
    pub atoms: Vec<String>,
}
fn term(t: &Term, atoms: &mut BTreeSet<String>) {
    if let Term::App(name, args) = t {
        assert!(args.is_empty(), "wire fragment is atom/unknown");
        atoms.insert(name.clone());
    }
}
fn constraint(c: &Constraint, predicates: &mut BTreeSet<String>, atoms: &mut BTreeSet<String>) {
    predicates.insert(c.name.clone());
    for t in &c.args {
        term(t, atoms);
    }
}
fn goal(g: &Goal, predicates: &mut BTreeSet<String>, atoms: &mut BTreeSet<String>) {
    match g {
        Goal::Constraint(c) => constraint(c, predicates, atoms),
        Goal::Unify(a, b) => {
            term(a, atoms);
            term(b, atoms);
        }
        Goal::And(gs) => {
            for g in gs {
                goal(g, predicates, atoms);
            }
        }
        Goal::Or(a, b) => {
            goal(a, predicates, atoms);
            goal(b, predicates, atoms);
        }
        Goal::True | Goal::Fail => (),
    }
}
impl Symbols {
    pub fn source(rules: &[Rule]) -> Self {
        let mut predicates = BTreeSet::new();
        let mut atoms = BTreeSet::new();
        for r in rules {
            assert!(r.guards.is_empty(), "wire source entry has no guards");
            for c in r.kept.iter().chain(&r.removed) {
                constraint(c, &mut predicates, &mut atoms);
            }
            goal(&r.body, &mut predicates, &mut atoms);
        }
        Self {
            predicates: predicates.into_iter().collect(),
            atoms: atoms.into_iter().collect(),
        }
    }
    pub fn query(&self, query: &Query) -> Self {
        let mut predicates = BTreeSet::new();
        let mut atoms = BTreeSet::new();
        for c in &query.constraints {
            constraint(c, &mut predicates, &mut atoms);
        }
        let mut out = self.clone();
        out.predicates.extend(
            predicates
                .into_iter()
                .filter(|p| !self.predicates.contains(p)),
        );
        out.atoms
            .extend(atoms.into_iter().filter(|a| !self.atoms.contains(a)));
        out
    }
    fn terms(&self, terms: impl Iterator<Item = Term>, bytes: &mut Vec<u8>) {
        for t in terms {
            let (tag, value) = match t {
                Term::Var(v) => (1, u32::try_from(v.0).expect("wire unknown id bound")),
                Term::App(name, args) => {
                    assert!(args.is_empty());
                    (
                        2,
                        u32::try_from(
                            self.atoms
                                .iter()
                                .position(|a| a == &name)
                                .expect("atom dictionary"),
                        )
                        .unwrap(),
                    )
                }
            };
            bytes.push(tag);
            bytes.extend(value.to_le_bytes());
        }
        bytes.push(0);
    }
}
pub struct OwnedWire {
    pub symbols: Symbols,
    pub bytes: Vec<u8>,
}
impl OwnedWire {
    pub fn new(symbols: Symbols, answers: Vec<Answer>) -> Self {
        let mut bytes = vec![];
        for answer in answers {
            bytes.push(4);
            symbols.terms(answer.outputs.into_iter().map(|(_, t)| t), &mut bytes);
            for c in answer.residual {
                bytes.push(3);
                let id = u32::try_from(
                    symbols
                        .predicates
                        .iter()
                        .position(|p| p == &c.name)
                        .expect("predicate dictionary"),
                )
                .unwrap();
                bytes.extend(id.to_le_bytes());
                symbols.terms(c.args.into_iter(), &mut bytes);
            }
            bytes.push(0);
        }
        Self { symbols, bytes }
    }
}
