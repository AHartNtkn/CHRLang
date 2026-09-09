//! Source-generated index maintenance; Active wake ordering remains unchanged.
use super::{Access, COLLECT_METRICS, Core, IndexKey, Policy, Term};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
#[derive(Clone)]
pub(crate) enum Plan {
    Generated(Arc<BTreeMap<usize, Repair>>),
    Columns(Arc<BTreeMap<usize, Vec<bool>>>),
}
pub type Repair = fn(&mut Core, u64);
pub type Spec = (&'static str, usize, Repair);
#[derive(Default)]
pub struct Update {
    vars: BTreeSet<u64>,
    nodes: BTreeSet<usize>,
    todo: Vec<Term>,
    keys: Vec<Option<IndexKey>>,
}
impl Core {
    pub(crate) fn update_from_columns(&mut self, id: u64, columns: &[bool]) {
        let mut work = self.update_start();
        for (argument, &indexed) in columns.iter().enumerate() {
            if self.update_active() || (self.access_indexed() && indexed) {
                self.update_watch(id, argument, &mut work);
            }
            if self.access_indexed() && indexed {
                self.update_index(id, argument, &mut work);
            }
        }
        self.update_finish(id, work);
    }

    pub fn update_active(&self) -> bool {
        self.policy == Policy::Active
    }
    pub fn update_start(&self) -> Update {
        Update::default()
    }
    pub fn update_watch(&mut self, id: u64, argument: usize, work: &mut Update) {
        work.todo.push(self.store[&id].args[argument]);
        while let Some(t) = work.todo.pop() {
            if COLLECT_METRICS {
                self.stats.dependency_visits += 1;
            }
            match t {
                Term::Var(v) => {
                    if work.vars.insert(v)
                        && let Some(t) = self.bindings.get(&v, &mut self.stats.kernel.storage)
                    {
                        work.todo.push(t);
                    }
                }
                Term::Node(n) => {
                    if !self.arena.is_closed(n) && work.nodes.insert(n) {
                        work.todo.extend(&self.arena.node(n).args);
                    }
                }
            }
        }
    }
    pub fn update_index(&mut self, id: u64, argument: usize, work: &mut Update) {
        let occ = &self.store[&id];
        let pred = occ.pred;
        let value = occ.args[argument];
        let key = self.ground_key(value).map(|key| (pred, argument, key));
        if let Some(key) = key {
            assert!(self.index.entry(key).or_default().insert(id));
            if COLLECT_METRICS {
                self.stats.index_inserts += 1;
                self.stats.index_entries += 1;
            }
        }
        work.keys.push(key);
    }
    pub fn update_finish(&mut self, id: u64, work: Update) {
        for v in &work.vars {
            self.watchers.entry(*v).or_default().insert(id);
        }
        if COLLECT_METRICS {
            self.stats.dependency_edges += work.vars.len();
            self.stats.max_dependency_edges = self
                .stats
                .max_dependency_edges
                .max(self.stats.dependency_edges);
            self.stats.max_index_entries =
                self.stats.max_index_entries.max(self.stats.index_entries);
        }
        if !work.vars.is_empty() {
            self.dependencies.insert(id, work.vars);
        }
        if self.access == Access::Indexed && !work.keys.is_empty() {
            self.occurrence_keys.insert(id, work.keys);
        }
    }
}
