//! Symbolic conjunction answers with existential locals and explicit caller imports.
use crate::{joint, normal_forms::Requirement};
use chr_syntax::{Term, Var};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug)]
pub struct Answer {
    pub imports: BTreeSet<Var>,
    pub outputs: Vec<Term>,
    pub names: Vec<Term>,
    pub structure: Vec<(Term, Requirement)>,
    pub unequal: Vec<(Term, Term)>,
    pub alphabet: Option<Vec<String>>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fresh {
    pub next: Option<u64>,
    pub occupied: BTreeSet<Var>,
}
pub struct Instance {
    answer: Answer,
    hidden: BTreeSet<Var>,
}
fn variables(t: &Term, out: &mut BTreeSet<Var>) {
    match t {
        Term::Var(x) => {
            out.insert(*x);
        }
        Term::App(_, a) => {
            for t in a {
                variables(t, out)
            }
        }
    }
}
fn renamed(t: &Term, map: &BTreeMap<Var, Var>) -> Term {
    match t {
        Term::Var(x) => Term::Var(map[x]),
        Term::App(n, a) => Term::App(n.clone(), a.iter().map(|t| renamed(t, map)).collect()),
    }
}
impl Fresh {
    fn take(&mut self) -> Result<Var, String> {
        loop {
            let n = self.next.ok_or("fresh identity space exhausted")?;
            self.next = n.checked_add(1);
            if self.occupied.insert(Var(n)) {
                return Ok(Var(n));
            }
        }
    }
}
impl Answer {
    pub fn instantiate(
        &self,
        caller: &BTreeMap<Var, Var>,
        fresh: &mut Fresh,
    ) -> Result<Instance, String> {
        if caller.keys().copied().collect::<BTreeSet<_>>() != self.imports {
            return Err("caller mapping must cover exactly the declared imports".into());
        }
        let mut vars = self.imports.clone();
        for t in self
            .outputs
            .iter()
            .chain(&self.names)
            .chain(self.structure.iter().map(|(t, _)| t))
            .chain(self.unequal.iter().flat_map(|(a, b)| [a, b]))
        {
            variables(t, &mut vars)
        }
        let mut staged = fresh.clone();
        staged.occupied.extend(caller.values().copied());
        let mut mapping = caller.clone();
        for x in vars {
            if let std::collections::btree_map::Entry::Vacant(entry) = mapping.entry(x) {
                entry.insert(staged.take()?);
            }
        }
        let answer = Answer {
            imports: caller.values().copied().collect(),
            outputs: self.outputs.iter().map(|t| renamed(t, &mapping)).collect(),
            names: self.names.iter().map(|t| renamed(t, &mapping)).collect(),
            structure: self
                .structure
                .iter()
                .map(|(t, r)| (renamed(t, &mapping), *r))
                .collect(),
            unequal: self
                .unequal
                .iter()
                .map(|(a, b)| (renamed(a, &mapping), renamed(b, &mapping)))
                .collect(),
            alphabet: self.alphabet.clone(),
        };
        let mut visible = answer.imports.clone();
        for t in &answer.outputs {
            variables(t, &mut visible)
        }
        let hidden = mapping
            .values()
            .copied()
            .filter(|v| !visible.contains(v))
            .collect();
        *fresh = staged;
        Ok(Instance { answer, hidden })
    }
}
impl Instance {
    pub fn outputs(&self) -> &[Term] {
        &self.answer.outputs
    }
    /// Hidden identities are existential, so caller bindings must not select or capture them.
    pub fn consistent(&self, bindings: &BTreeMap<Var, Term>) -> Result<bool, String> {
        for (x, t) in bindings {
            let mut mentioned = BTreeSet::from([*x]);
            variables(t, &mut mentioned);
            if !mentioned.is_disjoint(&self.hidden) {
                return Err("caller binding captures a hidden existential".into());
            }
        }
        fn finite(
            t: &Term,
            b: &BTreeMap<Var, Term>,
            active: &mut BTreeSet<Var>,
        ) -> Result<(), String> {
            match t {
                Term::Var(x) => {
                    if let Some(value) = b.get(x)
                        && value != t
                    {
                        if !active.insert(*x) {
                            return Err("cyclic output binding".into());
                        }
                        finite(value, b, active)?;
                        active.remove(x);
                    }
                }
                Term::App(_, a) => {
                    for t in a {
                        finite(t, b, active)?;
                    }
                }
            }
            Ok(())
        }
        for t in &self.answer.outputs {
            finite(t, bindings, &mut BTreeSet::new())?;
        }
        joint::feasible(
            &self.answer.names,
            &self.answer.structure,
            &self.answer.unequal,
            bindings,
            self.answer.alphabet.as_deref(),
        )
    }
}
