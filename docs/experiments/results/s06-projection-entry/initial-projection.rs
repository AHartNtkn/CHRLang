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
fn mul(a: u128, b: u128, s: Semantics) -> Result<u128, String> {
    if s == Semantics::Set {
        Ok(u128::from(a > 0 && b > 0))
    } else {
        a.checked_mul(b).ok_or_else(|| "count overflow".into())
    }
}
fn assignments(
    scope: &[usize],
    domains: &[Vec<u8>],
    limit: usize,
    mut f: impl FnMut(&[u8]) -> Result<(), String>,
) -> Result<usize, String> {
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
fn weight(factors: &[Factor], values: &[u8], s: Semantics) -> Result<u128, String> {
    let mut result = 1;
    for f in factors {
        let key = f.scope.iter().map(|&i| values[i]).collect::<Vec<_>>();
        result = mul(result, f.table.get(&key).copied().unwrap_or(0), s)?;
        if result == 0 {
            break;
        }
    }
    Ok(result)
}
impl Problem {
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
