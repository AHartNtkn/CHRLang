use chr_syntax::{Answer, Constraint, Query, Term, Var, atom, c, t};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub enum Event {
    Request(Term, Term),
    Replace(Term, Term, Term),
    Bind(Var, Term),
}
#[derive(Clone)]
pub struct Case {
    pub left: Vec<(Term, Term)>,
    pub right: Vec<(Term, Term)>,
    pub events: Vec<Event>,
}
pub use chr_compiled::update_join::source_rules as rules;
impl Case {
    pub fn grid(n: usize, rounds: usize, groups: usize) -> Self {
        Self {
            left: (0..n)
                .map(|i| (atom(&format!("k{}", i % groups)), atom(&format!("l{i}"))))
                .collect(),
            right: (0..n)
                .map(|i| (atom(&format!("k{}", i % groups)), atom(&format!("r{i}"))))
                .collect(),
            events: (0..rounds)
                .map(|i| {
                    Event::Request(
                        atom(&format!("k{}", i % groups)),
                        atom(&format!("round{i}")),
                    )
                })
                .collect(),
        }
    }
    pub fn query(&self, driver_first: bool) -> Query {
        let script = self.events.iter().rev().fold(atom("nil"), |tail, e| {
            let head = match e {
                Event::Request(k, r) => t("req", [k.clone(), r.clone()]),
                Event::Replace(k, a, b) => t("replace", [k.clone(), a.clone(), b.clone()]),
                Event::Bind(x, value) => t("bind", [Term::Var(*x), value.clone()]),
            };
            t("cons", [head, tail])
        });
        let mut constraints: Vec<Constraint> = self
            .left
            .iter()
            .map(|(k, x)| c("left", [k.clone(), x.clone()]))
            .chain(
                self.right
                    .iter()
                    .map(|(k, x)| c("right", [k.clone(), x.clone()])),
            )
            .collect();
        if driver_first {
            constraints.insert(0, c("drive", [script]));
        } else {
            constraints.push(c("drive", [script]));
        }
        Query {
            constraints,
            outputs: vec![],
        }
    }
    /// Plain row-list/cartesian specification. No source matcher, index or engine kernel.
    pub fn expected(&self) -> Answer {
        let left = self.left.clone();
        let mut right = self.right.clone();
        let mut right_ids: Vec<usize> = (0..right.len()).collect();
        let mut next_right = right.len();
        let mut fired = BTreeSet::new();
        let mut bindings = BTreeMap::new();
        let mut receipts = vec![];
        for (request_id, event) in self.events.iter().enumerate() {
            match event {
                Event::Request(key, round) => {
                    let key = resolve(key, &bindings);
                    for (left_id, (lk, a)) in left.iter().enumerate() {
                        for (right_position, (rk, b)) in right.iter().enumerate() {
                            if resolve(lk, &bindings) == key && resolve(rk, &bindings) == key {
                                assert!(fired.insert((
                                    left_id,
                                    right_ids[right_position],
                                    request_id
                                )));
                                receipts.push(c("receipt", [round.clone(), a.clone(), b.clone()]));
                            }
                        }
                    }
                }
                Event::Replace(key, old, new) => {
                    let i = right
                        .iter()
                        .position(|(k, v)| {
                            resolve(k, &bindings) == resolve(key, &bindings)
                                && resolve(v, &bindings) == resolve(old, &bindings)
                        })
                        .expect("oracle replacement must exist");
                    right.remove(i);
                    right_ids.remove(i);
                    right_ids.push(next_right);
                    next_right += 1;
                    right.push((key.clone(), new.clone()));
                }
                Event::Bind(var, value) => {
                    assert!(!bindings.contains_key(var));
                    assert!(matches!(value,Term::App(_,xs) if xs.is_empty()));
                    bindings.insert(*var, value.clone());
                }
            }
        }
        let mut residual = left
            .into_iter()
            .map(|(k, x)| c("left", [k, x]))
            .chain(right.into_iter().map(|(k, x)| c("right", [k, x])))
            .chain(receipts)
            .collect::<Vec<_>>();
        residual.push(c("done", []));
        for item in &mut residual {
            for arg in &mut item.args {
                *arg = resolve(arg, &bindings);
            }
        }
        Answer {
            outputs: vec![],
            residual,
        }
    }
}
fn resolve(term: &Term, bindings: &BTreeMap<Var, Term>) -> Term {
    match term {
        Term::Var(x) => bindings.get(x).cloned().unwrap_or_else(|| term.clone()),
        Term::App(n, xs) => t(
            n,
            xs.iter().map(|x| resolve(x, bindings)).collect::<Vec<_>>(),
        ),
    }
}
