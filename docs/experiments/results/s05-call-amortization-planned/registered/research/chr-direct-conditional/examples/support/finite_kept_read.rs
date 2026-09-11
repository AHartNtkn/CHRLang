//! Checked zero-argument reads for an initial finite phase.
//! The caller must retain the original full source for all subsequent work.
use super::finite_phase::{self, Error, Limits, Machine, Report};
use chr_syntax::{Constraint, Query, Rule};
use std::collections::{BTreeMap, BTreeSet};
type Key = (String, usize);
fn key(c: &Constraint) -> Key {
    (c.name.clone(), c.args.len())
}
pub struct Prepared {
    phase: finite_phase::Prepared,
    required: BTreeMap<Key, usize>,
}
impl Prepared {
    pub fn new(source: &[Rule], prefix: usize) -> Result<Self, Error> {
        if prefix == 0 || prefix > source.len() {
            return Err(Error::Source("invalid prefix"));
        }
        let consumed: BTreeSet<_> = source[..prefix]
            .iter()
            .flat_map(|r| r.removed.iter().map(key))
            .collect();
        let mut required = BTreeMap::<Key, usize>::new();
        for rule in &source[..prefix] {
            let mut per_rule = BTreeMap::<Key, usize>::new();
            for head in &rule.kept {
                if !head.args.is_empty() {
                    return Err(Error::Source("kept read must have no arguments"));
                }
                let k = key(head);
                if consumed.contains(&k) {
                    return Err(Error::Source("prefix consumes a required kept read"));
                }
                *per_rule.entry(k).or_default() += 1;
            }
            for (k, n) in per_rule {
                let count = required.entry(k).or_default();
                *count = (*count).max(n);
            }
        }
        let mut normalized = source.to_vec();
        for rule in &mut normalized[..prefix] {
            rule.kept.clear();
        }
        let phase = finite_phase::Prepared::new(&normalized, prefix)?;
        Ok(Self { phase, required })
    }
    pub fn start<'a>(&'a self, query: &Query, limits: Limits) -> Result<Machine<'a>, Error> {
        let mut present = BTreeMap::<Key, usize>::new();
        for c in &query.constraints {
            let k = key(c);
            if self.required.contains_key(&k) {
                *present.entry(k).or_default() += 1;
            }
        }
        if self
            .required
            .iter()
            .any(|(k, n)| present.get(k).copied().unwrap_or(0) < *n)
        {
            return Err(Error::Source("initial kept-read occurrences are missing"));
        }
        self.phase.start(query, limits)
    }
    pub fn solve(&self, query: &Query, limits: Limits) -> Result<Report, Error> {
        self.start(query, limits)?.finish()
    }
}
