//! Three-valued variables and forbidden ordered value pairs.
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endpoint {
    Var(usize),
    Value(u8),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Forbidden {
    pub left: Endpoint,
    pub right: Endpoint,
    pub a: u8,
    pub b: u8,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub left: usize,
    pub right: usize,
    pub support: [u8; 3],
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryDomains {
    pub masks: Vec<u8>,
    pub inconsistent: bool,
}
#[derive(Clone, Debug)]
pub struct Problem {
    pub(crate) variables: usize,
    /// Original source multiset, including repeated restrictions.
    pub(crate) forbidden: Vec<Forbidden>,
    pub(crate) unary: Vec<u8>,
    pub(crate) edges: Vec<Edge>,

    pub(crate) inconsistent: bool,
}
impl Problem {
    pub fn variables(&self) -> usize {
        self.variables
    }
    pub fn forbidden(&self) -> &[Forbidden] {
        &self.forbidden
    }

    pub fn new(variables: usize, forbidden: Vec<Forbidden>) -> Result<Self, String> {
        let mut unary = vec![7; variables];
        let mut tables: BTreeMap<(usize, usize), [u8; 3]> = BTreeMap::new();
        let mut inconsistent = false;
        for f in &forbidden {
            Self::validate_endpoint(variables, f.left)?;
            Self::validate_endpoint(variables, f.right)?;
            Self::validate_value(f.a)?;
            Self::validate_value(f.b)?;
            match (f.left, f.right) {
                (Endpoint::Value(a), Endpoint::Value(b)) => inconsistent |= a == f.a && b == f.b,
                (Endpoint::Var(v), Endpoint::Value(b)) => {
                    if b == f.b {
                        unary[v] &= !(1 << f.a);
                    }
                }
                (Endpoint::Value(a), Endpoint::Var(v)) => {
                    if a == f.a {
                        unary[v] &= !(1 << f.b);
                    }
                }
                (Endpoint::Var(a), Endpoint::Var(b)) if a == b => {
                    if f.a == f.b {
                        unary[a] &= !(1 << f.a);
                    }
                }
                (Endpoint::Var(a), Endpoint::Var(b)) => {
                    let (left, right, x, y) = if a < b {
                        (a, b, f.a, f.b)
                    } else {
                        (b, a, f.b, f.a)
                    };
                    tables.entry((left, right)).or_insert([7; 3])[x as usize] &= !(1 << y);
                }
            }
        }
        inconsistent |= unary.contains(&0);
        let edges: Vec<Edge> = tables
            .into_iter()
            .map(|((left, right), support)| Edge {
                left,
                right,
                support,
            })
            .collect();
        Ok(Self {
            variables,
            forbidden,
            unary,
            edges,
            inconsistent,
        })
    }
    fn validate_value(value: u8) -> Result<(), String> {
        if value > 2 {
            Err("finite values must be 0, 1, or 2".into())
        } else {
            Ok(())
        }
    }
    fn validate_endpoint(variables: usize, endpoint: Endpoint) -> Result<(), String> {
        match endpoint {
            Endpoint::Var(v) if v >= variables => {
                Err("variable endpoint is outside the declared model".into())
            }
            Endpoint::Value(value) => Self::validate_value(value),
            _ => Ok(()),
        }
    }
    pub fn domains(&self, givens: &[(Endpoint, u8)]) -> Result<QueryDomains, String> {
        let mut masks = self.unary.clone();
        let mut inconsistent = self.inconsistent;
        for (endpoint, value) in givens {
            Self::validate_endpoint(self.variables, *endpoint)?;
            Self::validate_value(*value)?;
            match *endpoint {
                Endpoint::Var(v) => masks[v] &= 1 << value,
                Endpoint::Value(actual) => inconsistent |= actual != *value,
            }
        }
        inconsistent |= masks.contains(&0);
        Ok(QueryDomains {
            masks,
            inconsistent,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordered_tables_self_relations_and_constants_keep_their_meaning() {
        let source = vec![
            Forbidden {
                left: Endpoint::Var(1),
                right: Endpoint::Var(0),
                a: 2,
                b: 0,
            },
            Forbidden {
                left: Endpoint::Var(0),
                right: Endpoint::Var(0),
                a: 1,
                b: 1,
            },
            Forbidden {
                left: Endpoint::Var(0),
                right: Endpoint::Var(0),
                a: 0,
                b: 2,
            },
            Forbidden {
                left: Endpoint::Value(2),
                right: Endpoint::Var(1),
                a: 2,
                b: 1,
            },
        ];
        let problem = Problem::new(2, source.clone()).unwrap();
        assert_eq!(problem.forbidden, source);
        assert_eq!(problem.unary, vec![5, 5]);
        assert_eq!(
            problem.edges,
            vec![Edge {
                left: 0,
                right: 1,
                support: [3, 7, 7]
            }]
        );
        assert_eq!(
            problem.domains(&[(Endpoint::Var(0), 1)]).unwrap().masks,
            vec![0, 5]
        );
    }
    #[test]
    fn ground_contradictions_include_empty_models() {
        let p = Problem::new(0, vec![]).unwrap();
        assert!(!p.domains(&[]).unwrap().inconsistent);
        assert!(p.domains(&[(Endpoint::Value(0), 2)]).unwrap().inconsistent);
        assert!(
            Problem::new(
                0,
                vec![Forbidden {
                    left: Endpoint::Value(0),
                    right: Endpoint::Value(2),
                    a: 0,
                    b: 2
                }]
            )
            .unwrap()
            .inconsistent
        );
        assert!(p.domains(&[(Endpoint::Var(0), 0)]).is_err());
        assert!(p.domains(&[(Endpoint::Value(3), 0)]).is_err());
    }
}
