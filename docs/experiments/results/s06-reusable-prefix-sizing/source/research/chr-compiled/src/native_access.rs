//! Primitives for source-generated streaming continuations.
//! A selector's store is immutable until it returns an application. Ranges may
//! therefore resume by occurrence identity without retaining candidate vectors.
use super::{Access, COLLECT_METRICS, Core, IndexKey, Selection, Term};
use std::ops::Bound::{Excluded, Unbounded};

pub trait Continuation: Send + Sync {
    fn tick(&mut self, core: &mut Core, anchor: Option<(usize, u64)>) -> Selection;
    fn duplicate(&self) -> Box<dyn Continuation>;
}
impl Clone for Box<dyn Continuation> {
    fn clone(&self) -> Self {
        self.duplicate()
    }
}
pub type Factory = fn(u64) -> Box<dyn Continuation>;
#[derive(Clone, Copy)]
pub enum Range {
    Anchor(Option<u64>),
    Pool {
        pred: usize,
        key: Option<IndexKey>,
        size: usize,
        after: Option<u64>,
    },
}
impl Core {
    pub fn access_indexed(&self) -> bool {
        self.access == Access::Indexed
    }
    pub fn access_ground(&mut self, term: Term) -> Option<usize> {
        self.ground_key(term)
    }
    pub fn access_node(&mut self, name: &str, children: Vec<usize>) -> usize {
        self.key_make(name, children.into_iter().map(Term::Node).collect())
    }
    pub fn access_range(&self, rule: usize, head: usize, anchor: Option<(usize, u64)>) -> Range {
        if let Some((_, id)) = anchor.filter(|(h, _)| *h == head) {
            Range::Anchor(Some(id))
        } else {
            Range::Pool {
                pred: self.rules[rule].heads[head].pred,
                key: None,
                size: usize::MAX,
                after: None,
            }
        }
    }
    /// Returns true when no occurrence can match, so later keys need no work.
    pub fn access_consider(
        &mut self,
        range: &mut Range,
        argument: usize,
        value: Option<usize>,
    ) -> bool {
        let Range::Pool {
            pred, key, size, ..
        } = range
        else {
            return false;
        };
        let Some(value) = value else { return false };
        if COLLECT_METRICS {
            self.stats.index_lookups += 1;
        }
        let candidate = (*pred, argument, value);
        let n = self
            .index
            .get(&candidate)
            .map_or(0, std::collections::BTreeSet::len);
        if n < *size {
            *key = Some(candidate);
            *size = n;
        }
        n == 0
    }
    pub fn access_next(&mut self, range: &mut Range) -> Option<u64> {
        match range {
            Range::Anchor(next) => next.take(),
            Range::Pool {
                pred, key, after, ..
            } => {
                let bucket = if let Some(key) = key {
                    if COLLECT_METRICS {
                        self.stats.index_lookups += 1;
                    }
                    self.index.get(key)
                } else {
                    self.pools.get(pred)
                };
                let start = after.map_or(Unbounded, Excluded);
                let id = bucket.and_then(|b| b.range((start, Unbounded)).next().copied());
                if let Some(id) = id {
                    *after = Some(id);
                    if COLLECT_METRICS {
                        if key.is_some() {
                            self.stats.index_bucket_entries += 1;
                        } else {
                            self.stats.pool_visits += 1;
                        }
                    }
                }
                id
            }
        }
    }
}
