//! Streaming finite-domain search with reversible domain changes.
use crate::model::{Endpoint, Problem};
use std::collections::VecDeque;

struct DirectedArc {
    variable: usize,
    neighbor: usize,
    support: [u8; 3],
}
/// Native propagation topology is prepared once and shared by query searches.
pub struct Prepared<'a> {
    problem: &'a Problem,
    arcs: Vec<DirectedArc>,
    affected: Vec<Vec<usize>>,
}
impl<'a> Prepared<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        // Immutable directed support topology is prepared once for every query.
        let mut arcs = Vec::with_capacity(2 * problem.edges.len());
        let mut affected = vec![vec![]; problem.variables];
        for edge in &problem.edges {
            affected[edge.right].push(arcs.len());
            arcs.push(DirectedArc {
                variable: edge.left,
                neighbor: edge.right,
                support: edge.support,
            });
            let mut reverse = [0; 3];
            for (a, support) in edge.support.iter().enumerate() {
                for (b, row) in reverse.iter_mut().enumerate() {
                    if support & (1 << b) != 0 {
                        *row |= 1 << a;
                    }
                }
            }
            affected[edge.left].push(arcs.len());
            arcs.push(DirectedArc {
                variable: edge.right,
                neighbor: edge.left,
                support: reverse,
            });
        }
        Self {
            problem,
            arcs,
            affected,
        }
    }
    pub fn start(&self, givens: &[(Endpoint, u8)]) -> Result<NativeSearch<'_>, String> {
        let query = self.problem.domains(givens)?;
        let mut domains = query.masks;
        let mut propagator = Propagator::new(self);
        let finished = query.inconsistent || !propagator.propagate(&mut domains, &mut vec![], None);
        Ok(NativeSearch {
            problem: self.problem,
            domains,
            propagator,
            trail: vec![],
            decisions: vec![],
            after_solution: false,
            finished,
        })
    }
}
struct Propagator<'a> {
    arcs: &'a [DirectedArc],
    affected: &'a [Vec<usize>],
    queue: VecDeque<usize>,
    queued: Vec<bool>,
}
impl<'a> Propagator<'a> {
    fn new(prepared: &'a Prepared<'_>) -> Self {
        Self {
            arcs: &prepared.arcs,
            affected: &prepared.affected,
            queue: VecDeque::new(),
            queued: vec![false; prepared.arcs.len()],
        }
    }
    fn enqueue(&mut self, id: usize) {
        if !self.queued[id] {
            self.queued[id] = true;
            self.queue.push_back(id);
        }
    }
    fn propagate(
        &mut self,
        domains: &mut [u8],
        trail: &mut Vec<(usize, u8)>,
        changed: Option<usize>,
    ) -> bool {
        self.queue.clear();
        self.queued.fill(false);
        if domains.contains(&0) {
            return false;
        }
        if let Some(variable) = changed {
            for i in 0..self.affected[variable].len() {
                self.enqueue(self.affected[variable][i]);
            }
        } else {
            for i in 0..self.arcs.len() {
                self.enqueue(i);
            }
        }
        while let Some(id) = self.queue.pop_front() {
            self.queued[id] = false;
            let arc = &self.arcs[id];
            let variable = arc.variable;
            let mut next = domains[variable];
            for value in 0..3 {
                if arc.support[value] & domains[arc.neighbor] == 0 {
                    next &= !(1 << value);
                }
            }
            if next != domains[variable] {
                trail.push((variable, domains[variable]));
                domains[variable] = next;
                if next == 0 {
                    return false;
                }
                for i in 0..self.affected[variable].len() {
                    self.enqueue(self.affected[variable][i]);
                }
            }
        }
        true
    }
}
/// Enforce unary restrictions and arc consistency in place. False means failure.
pub fn arc_consistency(problem: &Problem, domains: &mut [u8]) -> bool {
    assert_eq!(domains.len(), problem.variables);
    if problem.inconsistent {
        return false;
    }
    for (domain, unary) in domains.iter_mut().zip(&problem.unary) {
        *domain &= *unary;
    }
    Propagator::new(&Prepared::new(problem)).propagate(domains, &mut vec![], None)
}
struct Decision {
    variable: usize,
    remaining: u8,
    mark: usize,
}
pub struct NativeSearch<'a> {
    problem: &'a Problem,
    domains: Vec<u8>,
    propagator: Propagator<'a>,
    trail: Vec<(usize, u8)>,
    decisions: Vec<Decision>,
    after_solution: bool,
    finished: bool,
}
impl NativeSearch<'_> {
    fn alternative(&mut self) -> bool {
        while let Some(decision) = self.decisions.last_mut() {
            while self.trail.len() > decision.mark {
                let (variable, old) = self.trail.pop().unwrap();
                self.domains[variable] = old;
            }
            if decision.remaining == 0 {
                self.decisions.pop();
                continue;
            }
            let selected = 1 << decision.remaining.trailing_zeros();
            decision.remaining &= !selected;
            let variable = decision.variable;
            self.trail.push((variable, self.domains[variable]));
            self.domains[variable] = selected;
            if self
                .propagator
                .propagate(&mut self.domains, &mut self.trail, Some(variable))
            {
                return true;
            }
        }
        self.finished = true;
        false
    }
}
impl Iterator for NativeSearch<'_> {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        if self.after_solution && !self.alternative() {
            return None;
        }
        self.after_solution = false;
        loop {
            let variable = self
                .domains
                .iter()
                .enumerate()
                .filter(|(_, d)| d.count_ones() > 1)
                .min_by_key(|(_, d)| d.count_ones())
                .map(|(i, _)| i);
            let Some(variable) = variable else {
                self.after_solution = true;
                return Some(
                    (0..self.problem.variables)
                        .map(|i| self.domains[i].trailing_zeros() as u8)
                        .collect(),
                );
            };
            self.decisions.push(Decision {
                variable,
                remaining: self.domains[variable],
                mark: self.trail.len(),
            });
            if !self.alternative() {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Forbidden;
    #[test]
    fn trail_restores_siblings_and_prepared_queries_do_not_share_bindings() {
        let p = Problem::new(
            2,
            (0..3)
                .map(|a| Forbidden {
                    left: Endpoint::Var(0),
                    right: Endpoint::Var(1),
                    a,
                    b: a,
                })
                .collect(),
        )
        .unwrap();
        let prepared = Prepared::new(&p);
        let answers: Vec<_> = prepared.start(&[]).unwrap().collect();
        assert_eq!(
            answers,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![1, 2],
                vec![2, 0],
                vec![2, 1]
            ]
        );
        assert_eq!(
            prepared
                .start(&[(Endpoint::Var(0), 2)])
                .unwrap()
                .collect::<Vec<_>>(),
            vec![vec![2, 0], vec![2, 1]]
        );
        assert_eq!(prepared.start(&[]).unwrap().count(), 6);
    }
    #[test]
    fn unsupported_values_propagate_in_both_directions() {
        let p = Problem::new(
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
        let mut domains = vec![7, 7];
        assert!(arc_consistency(&p, &mut domains));
        assert_eq!(domains, vec![3, 7]);
        let p = Problem::new(0, vec![]).unwrap();
        assert_eq!(
            Prepared::new(&p).start(&[]).unwrap().collect::<Vec<_>>(),
            vec![vec![]]
        );
        assert_eq!(
            Prepared::new(&p)
                .start(&[(Endpoint::Value(0), 1)])
                .unwrap()
                .count(),
            0
        );
    }
    #[test]
    fn failed_decisions_restore_an_arc_consistent_unsatisfiable_triangle() {
        let mut forbidden = vec![];
        for variable in 0..3 {
            forbidden.push(Forbidden {
                left: Endpoint::Var(variable),
                right: Endpoint::Var(variable),
                a: 2,
                b: 2,
            });
        }
        for (left, right) in [(0, 1), (1, 2), (0, 2)] {
            for value in 0..3 {
                forbidden.push(Forbidden {
                    left: Endpoint::Var(left),
                    right: Endpoint::Var(right),
                    a: value,
                    b: value,
                });
            }
        }
        let problem = Problem::new(3, forbidden).unwrap();
        let mut domains = problem.domains(&[]).unwrap().masks;
        assert!(arc_consistency(&problem, &mut domains));
        assert_eq!(domains, vec![3, 3, 3]);
        assert_eq!(Prepared::new(&problem).start(&[]).unwrap().count(), 0);
        assert_eq!(Prepared::new(&problem).start(&[]).unwrap().count(), 0);
    }
}
