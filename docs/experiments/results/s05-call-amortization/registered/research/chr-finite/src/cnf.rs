//! One-hot conflict/support encodings and independent unit propagation.
use crate::model::Problem;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encoding {
    Support,
    Conflict,
}
#[derive(Clone, Debug)]
pub struct Cnf {
    pub variables: usize,
    pub clauses: Vec<Vec<i32>>,
}
fn literal(variable: usize, value: usize) -> i32 {
    (1 + 3 * variable + value) as i32
}
impl Cnf {
    pub fn encode(problem: &Problem, encoding: Encoding) -> Self {
        let variables = problem
            .variables
            .checked_mul(3)
            .expect("Boolean variable count overflow");
        assert!(
            variables <= i32::MAX as usize,
            "CNF literals exceed i32 range"
        );
        let mut clauses = vec![];
        if problem.inconsistent {
            clauses.push(vec![]);
        }
        for variable in 0..problem.variables {
            clauses.push((0..3).map(|value| literal(variable, value)).collect());
            for a in 0..3 {
                for b in a + 1..3 {
                    clauses.push(vec![-literal(variable, a), -literal(variable, b)]);
                }
                if problem.unary[variable] & (1 << a) == 0 {
                    clauses.push(vec![-literal(variable, a)]);
                }
            }
        }
        for edge in &problem.edges {
            match encoding {
                Encoding::Conflict => {
                    for a in 0..3 {
                        for b in 0..3 {
                            if edge.support[a] & (1 << b) == 0 {
                                clauses.push(vec![-literal(edge.left, a), -literal(edge.right, b)]);
                            }
                        }
                    }
                }
                Encoding::Support => {
                    for a in 0..3 {
                        let mut clause = vec![-literal(edge.left, a)];
                        clause.extend(
                            (0..3)
                                .filter(|b| edge.support[a] & (1 << b) != 0)
                                .map(|b| literal(edge.right, b)),
                        );
                        clauses.push(clause);
                    }
                    for b in 0..3 {
                        let mut clause = vec![-literal(edge.right, b)];
                        clause.extend(
                            (0..3)
                                .filter(|a| edge.support[*a] & (1 << b) != 0)
                                .map(|a| literal(edge.left, a)),
                        );
                        clauses.push(clause);
                    }
                }
            }
        }
        Self { variables, clauses }
    }
}
/// Unit closure under domain exclusions; no search, resolution, or finite-domain
/// support reasoning is used here. None means a clause has become false.
pub fn unit_propagate(cnf: &Cnf, domains: &[u8]) -> Option<Vec<u8>> {
    assert_eq!(cnf.variables, 3 * domains.len());
    let mut assignments = vec![None; cnf.variables + 1];
    for (variable, domain) in domains.iter().enumerate() {
        assert_eq!(domain & !7, 0, "domain contains an out-of-range value");
        for value in 0..3 {
            if domain & (1 << value) == 0 {
                assignments[literal(variable, value) as usize] = Some(false);
            }
        }
    }
    loop {
        let mut changed = false;
        for clause in &cnf.clauses {
            let mut unit = None;
            let mut unknown = 0;
            let mut satisfied = false;
            for lit in clause {
                let variable = lit.unsigned_abs() as usize;
                assert!(variable > 0 && variable <= cnf.variables);
                match assignments[variable] {
                    Some(value) if value == (*lit > 0) => {
                        satisfied = true;
                        break;
                    }
                    None => {
                        unit = Some(*lit);
                        unknown += 1;
                    }
                    _ => (),
                }
            }
            if satisfied {
                continue;
            }
            if unknown == 0 {
                return None;
            }
            if unknown == 1 {
                let lit = unit.unwrap();
                assignments[lit.unsigned_abs() as usize] = Some(lit > 0);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    Some(
        (0..domains.len())
            .map(|variable| {
                (0..3).fold(0, |mask, value| {
                    if assignments[literal(variable, value) as usize] == Some(false) {
                        mask
                    } else {
                        mask | (1 << value)
                    }
                })
            })
            .collect(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Endpoint, Forbidden};
    #[test]
    fn support_units_find_unsupported_values_without_branching() {
        let problem = Problem::new(
            2,
            (0..3)
                .map(|b| Forbidden {
                    left: Endpoint::Var(0),
                    right: Endpoint::Var(1),
                    a: 2,
                    b,
                })
                .collect(),
        )
        .unwrap();
        assert_eq!(
            unit_propagate(&Cnf::encode(&problem, Encoding::Support), &[7, 7]),
            Some(vec![3, 7])
        );
        assert_eq!(
            unit_propagate(&Cnf::encode(&problem, Encoding::Conflict), &[7, 7]),
            Some(vec![7, 7])
        );
        for encoding in [Encoding::Support, Encoding::Conflict] {
            assert_eq!(
                unit_propagate(&Cnf::encode(&problem, encoding), &[4, 7]),
                None
            );
        }
    }
    #[test]
    fn both_support_directions_and_empty_contradictions_propagate() {
        let problem = Problem::new(
            2,
            (0..3)
                .map(|a| Forbidden {
                    left: Endpoint::Var(0),
                    right: Endpoint::Var(1),
                    a,
                    b: 1,
                })
                .collect(),
        )
        .unwrap();
        assert_eq!(
            unit_propagate(&Cnf::encode(&problem, Encoding::Support), &[7, 7]),
            Some(vec![7, 5])
        );
        let p = Problem::new(0, vec![]).unwrap();
        assert_eq!(
            unit_propagate(&Cnf::encode(&p, Encoding::Support), &[]),
            Some(vec![])
        );
        let p = Problem::new(
            0,
            vec![Forbidden {
                left: Endpoint::Value(1),
                right: Endpoint::Value(1),
                a: 1,
                b: 1,
            }],
        )
        .unwrap();
        assert_eq!(
            unit_propagate(&Cnf::encode(&p, Encoding::Conflict), &[]),
            None
        );
    }
}
