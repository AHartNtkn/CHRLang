//! Semijoin filtering: retain original occurrence order and ordinary commitment.
use super::*;
fn slots(t: &Template, out: &mut BTreeSet<usize>) {
    match t {
        Template::Slot(s) => {
            out.insert(*s);
        }
        Template::App(_, args) => {
            for t in args {
                slots(t, out)
            }
        }
    }
}
pub(super) fn prepare(heads: &[Head]) -> Vec<Vec<usize>> {
    let variables: Vec<_> = heads
        .iter()
        .map(|h| {
            let mut out = BTreeSet::new();
            for t in &h.args {
                slots(t, &mut out)
            }
            out
        })
        .collect();
    variables
        .iter()
        .enumerate()
        .map(|(i, a)| {
            variables
                .iter()
                .enumerate()
                .skip(i + 1)
                .filter_map(|(j, b)| (!a.is_disjoint(b)).then_some(j))
                .collect()
        })
        .collect()
}
impl Core {
    pub(super) fn probe_pool(&mut self, rule: usize, head: usize, frame: &Frame) -> Vec<u64> {
        let current_key = self.best_key(rule, head, frame);
        if self.access != Access::Indexed || current_key.is_some() {
            return self.pool_for_key(rule, head, current_key);
        }
        let program = self.rules.clone();
        let prepared = &program[rule];
        for &later in &prepared.probes[head] {
            let Some(key) = self.best_key(rule, later, frame) else {
                continue;
            };
            let current_size = self
                .pools
                .get(&prepared.heads[head].pred)
                .map_or(0, BTreeSet::len);
            let probe_size = self.index.get(&key).map_or(0, BTreeSet::len);
            if probe_size >= current_size {
                continue;
            }

            if COLLECT_METRICS {
                self.stats.probe_starts += 1;
            }
            let later_ids = self.pool_for_key(rule, later, Some(key));
            let mut candidates = BTreeSet::new();
            let mut projected = BTreeSet::new();
            for id in later_ids {
                if COLLECT_METRICS {
                    self.stats.probe_visits += 1;
                }
                let Some(args) = self.store.get(&id).map(|o| o.args.clone()) else {
                    continue;
                };
                let mut bound = self.copy_frame(frame);
                if prepared.heads[later]
                    .args
                    .iter()
                    .zip(args)
                    .all(|(p, v)| self.generic_pattern(p, v, &mut bound))
                {
                    // An unknown projected key may admit the whole current pool.
                    // This overapproximation is necessary for nonbinding matches.
                    let key = self.best_key(rule, head, &bound);
                    if projected.insert(key) {
                        candidates.extend(self.pool_for_key(rule, head, key));
                    }
                }
            }
            if COLLECT_METRICS {
                self.stats.probe_candidates += candidates.len() as u64;
            }
            return candidates.into_iter().collect();
        }
        self.pool_for_key(rule, head, None)
    }
}
