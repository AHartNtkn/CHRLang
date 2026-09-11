//! Finite-coordinate existential elimination; domain choices count, filters do not.
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug)]
pub struct Relation {
    pub scope: Vec<usize>,
    pub rows: Vec<Vec<u8>>,
}
#[derive(Clone, Debug)]
pub struct Problem {
    pub domains: Vec<Vec<u8>>,
    pub filters: Vec<Relation>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Semantics {
    Set,
    Counted,
}
type Table = BTreeMap<Vec<u8>, u128>;
#[derive(Clone, Debug)]
struct Factor {
    scope: Vec<usize>,
    table: Table,
}
#[derive(Clone, Debug)]
pub struct Projection {
    domains: Vec<Vec<u8>>,
    visible: Vec<usize>,
    factors: Vec<Factor>,
    semantics: Semantics,
    pub elimination_visits: usize,
    pub peak_entries: usize,
}
fn add(a: u128, b: u128, s: Semantics) -> Result<u128, String> {
    if s == Semantics::Set {
        Ok(u128::from(a > 0 || b > 0))
    } else {
        a.checked_add(b).ok_or_else(|| "count overflow".into())
    }
}
fn assignments(
    scope: &[usize],
    domains: &[Vec<u8>],
    limit: usize,
    mut f: impl FnMut(&[u8]) -> Result<(), String>,
) -> Result<usize, String> {
    if scope.iter().any(|&i| domains[i].is_empty()) {
        return Ok(0);
    }
    let count = scope.iter().try_fold(1usize, |n, &i| {
        n.checked_mul(domains[i].len()).ok_or("assignment bound")
    })?;
    if count > limit {
        return Err("assignment bound".into());
    }
    let mut values = vec![0; domains.len()];
    for mut index in 0..count {
        for &i in scope.iter().rev() {
            let d = &domains[i];
            values[i] = d[index % d.len()];
            index /= d.len();
        }
        f(&values)?;
    }
    Ok(count)
}
fn weight(factors: &[Factor], values: &[u8], semantics: Semantics) -> Result<u128, String> {
    // An overflowing partial product is not an error until every factor is known
    // nonzero: a later impossible filter/domain annihilates the entire product.
    let mut product = Some(1_u128);
    for factor in factors {
        let key = factor.scope.iter().map(|&i| values[i]).collect::<Vec<_>>();
        let value = factor.table.get(&key).copied().unwrap_or(0);
        if value == 0 {
            return Ok(0);
        }
        product = match semantics {
            Semantics::Set => Some(1),
            Semantics::Counted => product.and_then(|p| p.checked_mul(value)),
        };
    }
    product.ok_or_else(|| "count overflow".into())
}
impl Problem {
    /// Greedy scope/domain estimate. This does not inspect relation answers or
    /// promise an optimal order; its preparation cost belongs in comparisons.
    pub fn elimination_order(&self, visible: &[usize]) -> Result<Vec<usize>, String> {
        let n = self.domains.len();
        let valid = |xs: &[usize]| {
            xs.iter().all(|&i| i < n)
                && xs.iter().copied().collect::<BTreeSet<_>>().len() == xs.len()
        };
        if !valid(visible) {
            return Err("invalid coordinate list".into());
        }
        if self
            .filters
            .iter()
            .any(|r| !valid(&r.scope) || r.rows.iter().any(|row| row.len() != r.scope.len()))
        {
            return Err("invalid relation".into());
        }
        let sizes = self
            .domains
            .iter()
            .map(|d| d.iter().collect::<BTreeSet<_>>().len() as u128)
            .collect::<Vec<_>>();
        let mut scopes = (0..n)
            .map(|i| BTreeSet::from([i]))
            .chain(
                self.filters
                    .iter()
                    .map(|r| r.scope.iter().copied().collect()),
            )
            .collect::<Vec<_>>();
        let mut remaining = (0..n)
            .filter(|i| !visible.contains(i))
            .collect::<BTreeSet<_>>();
        let mut order = Vec::with_capacity(remaining.len());
        while !remaining.is_empty() {
            let (_, _, variable, mut union) = remaining
                .iter()
                .map(|&i| {
                    let union = scopes
                        .iter()
                        .filter(|s| s.contains(&i))
                        .flat_map(|s| s.iter().copied())
                        .collect::<BTreeSet<_>>();
                    let work = if union.iter().any(|&j| sizes[j] == 0) {
                        0
                    } else {
                        union
                            .iter()
                            .fold(1_u128, |w, &j| w.saturating_mul(sizes[j]))
                    };
                    (work, union.len(), i, union)
                })
                .min_by_key(|(work, width, i, _)| (*work, *width, *i))
                .unwrap();
            scopes.retain(|s| !s.contains(&variable));
            union.remove(&variable);
            scopes.push(union);
            remaining.remove(&variable);
            order.push(variable);
        }
        Ok(order)
    }

    pub fn project(
        &self,
        visible: &[usize],
        caller_shared: &[usize],
        order: &[usize],
        semantics: Semantics,
        limit: usize,
    ) -> Result<Projection, String> {
        let n = self.domains.len();
        let valid = |xs: &[usize]| {
            xs.iter().all(|&i| i < n)
                && xs.iter().copied().collect::<BTreeSet<_>>().len() == xs.len()
        };
        if !valid(visible) || !valid(caller_shared) || !valid(order) {
            return Err("invalid coordinate list".into());
        }
        let hidden = (0..n)
            .filter(|i| !visible.contains(i))
            .collect::<BTreeSet<_>>();
        if order.iter().copied().collect::<BTreeSet<_>>() != hidden {
            return Err("elimination order must contain every hidden coordinate".into());
        }
        if caller_shared.iter().any(|i| !visible.contains(i)) {
            return Err("caller-shared coordinate cannot be hidden".into());
        }
        let domains = self
            .domains
            .iter()
            .map(|d| {
                d.iter()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut factors = vec![];
        for (i, d) in self.domains.iter().enumerate() {
            let mut table = Table::new();
            for &v in d {
                let old = table.get(&vec![v]).copied().unwrap_or(0);
                table.insert(vec![v], add(old, 1, semantics)?);
            }
            factors.push(Factor {
                scope: vec![i],
                table,
            });
        }
        for r in &self.filters {
            if !valid(&r.scope) || r.rows.iter().any(|row| row.len() != r.scope.len()) {
                return Err("invalid relation".into());
            }
            factors.push(Factor {
                scope: r.scope.clone(),
                table: r.rows.iter().cloned().map(|row| (row, 1)).collect(),
            });
        }
        let mut visits = 0;
        let mut peak = if cfg!(feature = "metrics") {
            factors.iter().map(|f| f.table.len()).max().unwrap_or(0)
        } else {
            0
        };
        for &variable in order {
            let (bucket, rest): (Vec<_>, Vec<_>) = factors
                .into_iter()
                .partition(|f| f.scope.contains(&variable));
            factors = rest;
            let union = bucket
                .iter()
                .flat_map(|f| f.scope.iter().copied())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let scope = union
                .iter()
                .copied()
                .filter(|&i| i != variable)
                .collect::<Vec<_>>();
            let mut table = Table::new();
            let work = assignments(&union, &domains, limit, |values| {
                let w = weight(&bucket, values, semantics)?;
                if w > 0 {
                    let key = scope.iter().map(|&i| values[i]).collect::<Vec<_>>();
                    let old = table.get(&key).copied().unwrap_or(0);
                    table.insert(key, add(old, w, semantics)?);
                }
                Ok(())
            })?;
            if cfg!(feature = "metrics") {
                visits += work;
                peak = peak.max(table.len());
            }
            factors.push(Factor { scope, table });
        }
        Ok(Projection {
            domains,
            visible: visible.to_vec(),
            factors,
            semantics,
            elimination_visits: visits,
            peak_entries: peak,
        })
    }
}
impl Projection {
    /// Materialize weighted tuples, then expand them in tuple order. This owns
    /// its pending answers, but does not reconstruct source derivation order.
    pub fn expanded_answers(
        &self,
        restrictions: &[(usize, u8)],
        assignment_limit: usize,
        output_limit: u128,
    ) -> Result<ExpandedAnswers, String> {
        let answers = self.answers(restrictions, assignment_limit)?;
        let total = answers
            .values()
            .try_fold(0_u128, |n, &w| n.checked_add(w).ok_or("count overflow"))?;
        if total > output_limit {
            return Err("output bound".into());
        }
        Ok(ExpandedAnswers {
            pending: answers.into_iter(),
            current: None,
        })
    }

    pub fn answers(&self, restrictions: &[(usize, u8)], limit: usize) -> Result<Table, String> {
        if restrictions.iter().any(|(i, _)| !self.visible.contains(i)) {
            return Err("restriction on eliminated or unknown coordinate".into());
        }
        let mut domains = self.domains.clone();
        for &(i, v) in restrictions {
            domains[i].retain(|x| *x == v);
        }
        let mut answers = Table::new();
        assignments(&self.visible, &domains, limit, |values| {
            let w = weight(&self.factors, values, self.semantics)?;
            if w > 0 {
                answers.insert(self.visible.iter().map(|&i| values[i]).collect(), w);
            }
            Ok(())
        })?;
        Ok(answers)
    }
}

/// Each yielded tuple is independent of the producer and other yielded tuples.
/// Dropping this iterator cancels pending expansion without visiting repeats.
pub struct ExpandedAnswers {
    pending: std::collections::btree_map::IntoIter<Vec<u8>, u128>,
    current: Option<(Vec<u8>, u128)>,
}
impl Iterator for ExpandedAnswers {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            self.current = self.pending.next();
        }
        let (row, count) = self.current.as_mut()?;
        if *count == 1 {
            self.current.take().map(|(row, _)| row)
        } else {
            *count -= 1;
            Some(row.clone())
        }
    }
}
