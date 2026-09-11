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
    /// Cartesian assignments, or complete compatible joins for sparse traversal.
    pub elimination_visits: usize,
    /// Candidate relation rows inspected by sparse traversal.
    pub join_probes: usize,
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
fn compatible_rows(
    factors: &[&Factor],
    domains: &[Vec<u8>],
    values: &mut [u8],
    bound: &mut [bool],
    remaining: &mut usize,
    emit: &mut impl FnMut(&[u8]) -> Result<(), String>,
) -> Result<(), String> {
    let Some((factor, rest)) = factors.split_first() else {
        return emit(values);
    };
    let fresh = factor
        .scope
        .iter()
        .copied()
        .filter(|&i| !bound[i])
        .collect::<Vec<_>>();
    for row in factor.table.keys() {
        *remaining = remaining.checked_sub(1).ok_or("join probe bound")?;
        if factor
            .scope
            .iter()
            .zip(row)
            .any(|(&i, &v)| (bound[i] && values[i] != v) || domains[i].binary_search(&v).is_err())
        {
            continue;
        }
        for (&i, &v) in factor.scope.iter().zip(row) {
            values[i] = v;
            bound[i] = true;
        }
        compatible_rows(rest, domains, values, bound, remaining, emit)?;
        for &i in &fresh {
            bound[i] = false;
        }
    }
    Ok(())
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
        self.project_using(visible, caller_shared, order, semantics, limit, false)
    }

    /// Join existing rows; `limit` bounds row probes per elimination bucket.
    pub fn project_sparse(
        &self,
        visible: &[usize],
        caller_shared: &[usize],
        order: &[usize],
        semantics: Semantics,
        limit: usize,
    ) -> Result<Projection, String> {
        self.project_using(visible, caller_shared, order, semantics, limit, true)
    }

    fn project_using(
        &self,
        visible: &[usize],
        caller_shared: &[usize],
        order: &[usize],
        semantics: Semantics,
        limit: usize,
        sparse: bool,
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
        let mut join_probes = 0;
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
            let mut joined = 0;
            let mut emit = |values: &[u8]| {
                joined += 1;
                let w = weight(&bucket, values, semantics)?;
                if w > 0 {
                    let key = scope.iter().map(|&i| values[i]).collect::<Vec<_>>();
                    let old = table.get(&key).copied().unwrap_or(0);
                    table.insert(key, add(old, w, semantics)?);
                }
                Ok(())
            };
            let work = if sparse {
                let mut ordered = bucket.iter().collect::<Vec<_>>();
                ordered.sort_by_key(|f| f.table.len());
                let mut remaining = limit;
                compatible_rows(
                    &ordered,
                    &domains,
                    &mut vec![0; n],
                    &mut vec![false; n],
                    &mut remaining,
                    &mut emit,
                )?;
                if cfg!(feature = "metrics") {
                    join_probes += limit - remaining;
                }
                joined
            } else {
                assignments(&union, &domains, limit, emit)?
            };
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
            join_probes,
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

    pub fn weighted_iter(
        &self,
        restrictions: &[(usize, u8)],
        limit: usize,
    ) -> Result<WeightedAnswers<'_>, String> {
        if restrictions.iter().any(|(i, _)| !self.visible.contains(i)) {
            return Err("restriction on eliminated or unknown coordinate".into());
        }
        let axes = self
            .visible
            .iter()
            .map(|&i| {
                let mut axis = self.domains[i].as_slice();
                for &(_, v) in restrictions.iter().filter(|(j, _)| *j == i) {
                    axis = match axis.binary_search(&v) {
                        Ok(j) => &axis[j..j + 1],
                        Err(_) => &[],
                    };
                }
                axis
            })
            .collect::<Vec<_>>();
        let count = if axes.iter().any(|a| a.is_empty()) {
            0
        } else {
            axes.iter().try_fold(1usize, |n, a| {
                n.checked_mul(a.len()).ok_or("assignment bound")
            })?
        };
        if count > limit {
            return Err("assignment bound".into());
        }
        let mut output = WeightedAnswers {
            projection: self,
            axes,
            values: vec![0; self.domains.len()],
            next: 0,
            count,
        };
        let safe = self.semantics == Semantics::Set
            || self.factors.iter().any(|f| f.table.is_empty())
            || self
                .factors
                .iter()
                .try_fold(1u128, |n, f| {
                    n.checked_mul(f.table.values().copied().max().unwrap_or(0))
                })
                .is_some();
        if !safe {
            for i in 0..count {
                output.position(i);
                weight(&self.factors, &output.values, self.semantics)?;
            }
        }
        Ok(output)
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

/// Borrows the prepared factors; each yielded weighted tuple is owned.
pub struct WeightedAnswers<'a> {
    projection: &'a Projection,
    axes: Vec<&'a [u8]>,
    values: Vec<u8>,
    next: usize,
    count: usize,
}
impl WeightedAnswers<'_> {
    fn position(&mut self, mut index: usize) {
        for (&i, axis) in self.projection.visible.iter().zip(&self.axes).rev() {
            self.values[i] = axis[index % axis.len()];
            index /= axis.len();
        }
    }
}
impl Iterator for WeightedAnswers<'_> {
    type Item = (Vec<u8>, u128);
    fn next(&mut self) -> Option<Self::Item> {
        while self.next < self.count {
            self.position(self.next);
            self.next += 1;
            let w = weight(
                &self.projection.factors,
                &self.values,
                self.projection.semantics,
            )
            .expect("weight products checked before output");
            if w > 0 {
                return Some((
                    self.projection
                        .visible
                        .iter()
                        .map(|&i| self.values[i])
                        .collect(),
                    w,
                ));
            }
        }
        None
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
