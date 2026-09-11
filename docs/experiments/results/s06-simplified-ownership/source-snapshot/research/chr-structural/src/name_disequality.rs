//! Experimental atomic-name equality/disequality, with an existential feasibility endpoint.
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Name {
    Variable(usize),
    Atom(String),
}
#[derive(Clone, Debug)]
pub struct Formula {
    variable_class: Vec<usize>,
    fixed: Vec<Option<String>>,
    edges: Vec<(usize, usize)>,
}
#[derive(Clone, Copy, Debug)]
pub struct FiniteResult {
    pub satisfiable: bool,
    pub assignment_attempts: usize,
}
fn root(parent: &[usize], mut x: usize) -> usize {
    while parent[x] != x {
        x = parent[x];
    }
    x
}
impl Formula {
    /// Compile atomic-name equations. Variable indices must be below `variables`.
    pub fn compile(
        variables: usize,
        equal: &[(Name, Name)],
        unequal: &[(Name, Name)],
    ) -> Option<Self> {
        let mut atoms = BTreeSet::new();
        for (a, b) in equal.iter().chain(unequal) {
            for x in [a, b] {
                match x {
                    Name::Atom(s) => {
                        atoms.insert(s.clone());
                    }
                    Name::Variable(i) => {
                        assert!(*i < variables, "variable outside declared interface")
                    }
                }
            }
        }
        let atoms: BTreeMap<_, _> = atoms
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s, variables + i))
            .collect();
        let index = |x: &Name| match x {
            Name::Variable(i) => *i,
            Name::Atom(s) => atoms[s],
        };
        let mut parent: Vec<_> = (0..variables + atoms.len()).collect();
        for (a, b) in equal {
            let a = root(&parent, index(a));
            let b = root(&parent, index(b));
            parent[b] = a;
        }
        let roots: BTreeSet<_> = (0..parent.len()).map(|i| root(&parent, i)).collect();
        let classes: BTreeMap<_, _> = roots.into_iter().enumerate().map(|(i, r)| (r, i)).collect();
        let class = |i| classes[&root(&parent, i)];
        let mut fixed = vec![None; classes.len()];
        for (name, i) in atoms {
            let cell = &mut fixed[class(i)];
            if cell.as_ref().is_some_and(|s| s != &name) {
                return None;
            }
            *cell = Some(name);
        }
        // Recover atom class identifiers from fixed values after equality normalization.
        let atom_class: BTreeMap<_, _> = fixed
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.as_ref().map(|s| (s.clone(), i)))
            .collect();
        let endpoint = |x: &Name| match x {
            Name::Variable(i) => class(*i),
            Name::Atom(s) => atom_class[s],
        };
        let mut edges = BTreeSet::new();
        for (a, b) in unequal {
            let a = endpoint(a);
            let b = endpoint(b);
            if a == b {
                return None;
            }
            edges.insert((a.min(b), a.max(b)));
        }
        Some(Self {
            variable_class: (0..variables).map(class).collect(),
            fixed,
            edges: edges.into_iter().collect(),
        })
    }
    fn seed(&self, given: &BTreeMap<usize, String>) -> Option<Vec<Option<String>>> {
        let mut values = self.fixed.clone();
        for (variable, name) in given {
            let class = *self.variable_class.get(*variable)?;
            if values[class].as_ref().is_some_and(|v| v != name) {
                return None;
            }
            values[class] = Some(name.clone());
        }
        self.consistent(&values).then_some(values)
    }
    fn consistent(&self, values: &[Option<String>]) -> bool {
        self.edges
            .iter()
            .all(|(a, b)| match (&values[*a], &values[*b]) {
                (Some(x), Some(y)) => x != y,
                _ => true,
            })
    }
    /// All remaining classes can use distinct fresh atoms outside the finite assigned set.
    pub fn unbounded(&self, given: &BTreeMap<usize, String>) -> bool {
        self.seed(given).is_some()
    }
    /// Exact finite-alphabet feasibility; this does not enumerate projected answers.
    pub fn finite(&self, alphabet: &[String], given: &BTreeMap<usize, String>) -> FiniteResult {
        let mut result = FiniteResult {
            satisfiable: false,
            assignment_attempts: 0,
        };
        let alphabet: Vec<_> = alphabet
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let Some(mut values) = self.seed(given) else {
            return result;
        };
        if values.iter().flatten().any(|v| !alphabet.contains(v)) {
            return result;
        }
        let mut order: Vec<_> = (0..values.len()).filter(|i| values[*i].is_none()).collect();
        order.sort_by_key(|i| {
            std::cmp::Reverse(self.edges.iter().filter(|(a, b)| a == i || b == i).count())
        });
        fn search(
            f: &Formula,
            values: &mut [Option<String>],
            alphabet: &[String],
            order: &[usize],
            attempts: &mut usize,
        ) -> bool {
            let Some((&next, rest)) = order.split_first() else {
                return true;
            };
            for name in alphabet {
                *attempts += 1;
                values[next] = Some(name.clone());
                if f.consistent(values) && search(f, values, alphabet, rest, attempts) {
                    return true;
                }
            }
            values[next] = None;
            false
        }
        result.satisfiable = search(
            self,
            &mut values,
            &alphabet,
            &order,
            &mut result.assignment_attempts,
        );
        result
    }
}
