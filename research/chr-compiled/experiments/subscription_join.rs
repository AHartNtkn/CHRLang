//! Experimental three-way join owner. Inputs are resolved source row projections.
use chr_syntax::Term;
use std::collections::{BTreeMap, BTreeSet};
type Key = (Term, Term);
pub type Triple = [usize; 3];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Indexed,
    Eager,
    Subscribed,
}
#[derive(Default)]
struct Relation {
    rows: BTreeMap<usize, Key>,
    first: BTreeMap<Term, BTreeSet<usize>>,
    second: BTreeMap<Term, BTreeSet<usize>>,
    exact: BTreeMap<Key, BTreeSet<usize>>,
}
fn erase<K: Ord>(map: &mut BTreeMap<K, BTreeSet<usize>>, key: &K, id: usize) {
    if let Some(ids) = map.get_mut(key) {
        ids.remove(&id);
        if ids.is_empty() {
            map.remove(key);
        }
    }
}
impl Relation {
    fn insert(&mut self, id: usize, row: Key) {
        assert!(!self.rows.contains_key(&id));
        self.first.entry(row.0.clone()).or_default().insert(id);
        self.second.entry(row.1.clone()).or_default().insert(id);
        self.exact.entry(row.clone()).or_default().insert(id);
        self.rows.insert(id, row);
    }
    fn remove(&mut self, id: usize) {
        let row = self.rows.remove(&id).expect("live occurrence");
        erase(&mut self.first, &row.0, id);
        erase(&mut self.second, &row.1, id);
        erase(&mut self.exact, &row, id);
    }
}
#[derive(Default, Debug)]
pub struct Stats {
    pub constructed: usize,
    pub invalidated: usize,
    pub probes: usize,
}
macro_rules! count {
    ($s:expr,$f:ident) => {
        if chr_compiled::COLLECT_METRICS {
            $s.$f += 1;
        }
    };
}
pub struct Join {
    mode: Mode,
    relations: [Relation; 3],
    demands: BTreeMap<usize, Key>,
    subscribers: BTreeMap<Key, BTreeSet<usize>>,
    active_left: BTreeMap<Term, BTreeSet<Key>>,
    active_right: BTreeMap<Term, BTreeSet<Key>>,
    tuples: BTreeMap<Key, BTreeSet<Triple>>,
    incidence: [BTreeMap<usize, BTreeSet<Triple>>; 3],
    stats: Stats,
}
impl Join {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            relations: std::array::from_fn(|_| Relation::default()),
            demands: BTreeMap::new(),
            subscribers: BTreeMap::new(),
            active_left: BTreeMap::new(),
            active_right: BTreeMap::new(),
            tuples: BTreeMap::new(),
            incidence: std::array::from_fn(|_| BTreeMap::new()),
            stats: Stats::default(),
        }
    }
    fn eligible(&self, key: &Key) -> bool {
        self.mode == Mode::Eager
            || (self.mode == Mode::Subscribed && self.subscribers.contains_key(key))
    }
    fn key(&self, t: Triple) -> Key {
        (
            self.relations[0].rows[&t[0]].0.clone(),
            self.relations[2].rows[&t[2]].1.clone(),
        )
    }
    fn retain(&mut self, t: Triple) {
        let key = self.key(t);
        if self.eligible(&key) && self.tuples.entry(key).or_default().insert(t) {
            for (r, id) in t.into_iter().enumerate() {
                self.incidence[r].entry(id).or_default().insert(t);
            }
            count!(self.stats, constructed);
        }
    }
    fn forget(&mut self, t: Triple) {
        let key = self.key(t);
        let ts = self.tuples.get_mut(&key).expect("retained key");
        assert!(ts.remove(&t));
        if ts.is_empty() {
            self.tuples.remove(&key);
        }
        for (r, id) in t.into_iter().enumerate() {
            let ts = self.incidence[r].get_mut(&id).expect("retained incidence");
            assert!(ts.remove(&t));
            if ts.is_empty() {
                self.incidence[r].remove(&id);
            }
        }
        count!(self.stats, invalidated);
    }
    /// Enumerate from the smaller endpoint bucket; exact final probes avoid broad joins.
    fn discover(&mut self, key: &Key) -> Vec<Triple> {
        let left = self.relations[0].first.get(&key.0);
        let right = self.relations[2].second.get(&key.1);
        let mut out = Vec::new();
        if left.map_or(0, BTreeSet::len) <= right.map_or(0, BTreeSet::len) {
            for &l in left.into_iter().flatten() {
                count!(self.stats, probes);
                let a = &self.relations[0].rows[&l].1;
                for &m in self.relations[1].first.get(a).into_iter().flatten() {
                    count!(self.stats, probes);
                    let b = &self.relations[1].rows[&m].1;
                    for &r in self.relations[2]
                        .exact
                        .get(&(b.clone(), key.1.clone()))
                        .into_iter()
                        .flatten()
                    {
                        out.push([l, m, r]);
                    }
                }
            }
        } else {
            for &r in right.into_iter().flatten() {
                count!(self.stats, probes);
                let b = &self.relations[2].rows[&r].0;
                for &m in self.relations[1].second.get(b).into_iter().flatten() {
                    count!(self.stats, probes);
                    let a = &self.relations[1].rows[&m].0;
                    for &l in self.relations[0]
                        .exact
                        .get(&(key.0.clone(), a.clone()))
                        .into_iter()
                        .flatten()
                    {
                        out.push([l, m, r]);
                    }
                }
            }
        }
        out.sort_unstable();
        out
    }
    pub fn open(&mut self, id: usize, key: Key) {
        assert!(!self.demands.contains_key(&id));
        self.demands.insert(id, key.clone());
        if self.mode == Mode::Subscribed {
            let first = !self.subscribers.contains_key(&key);
            self.subscribers.entry(key.clone()).or_default().insert(id);
            if first {
                self.active_left
                    .entry(key.0.clone())
                    .or_default()
                    .insert(key.clone());
                self.active_right
                    .entry(key.1.clone())
                    .or_default()
                    .insert(key.clone());
                for t in self.discover(&key) {
                    self.retain(t);
                }
            }
        }
    }
    pub fn close(&mut self, id: usize) {
        let key = self.demands.remove(&id).expect("live demand");
        if self.mode == Mode::Subscribed {
            erase(&mut self.subscribers, &key, id);
            if !self.subscribers.contains_key(&key) {
                for (index, endpoint) in [
                    (&mut self.active_left, &key.0),
                    (&mut self.active_right, &key.1),
                ] {
                    let keys = index.get_mut(endpoint).expect("active endpoint");
                    assert!(keys.remove(&key));
                    if keys.is_empty() {
                        index.remove(endpoint);
                    }
                }
                let tuples = self
                    .tuples
                    .get(&key)
                    .into_iter()
                    .flatten()
                    .copied()
                    .collect::<Vec<_>>();
                for t in tuples {
                    self.forget(t);
                }
            }
        }
    }
    pub fn request(&mut self, id: usize) -> Vec<Triple> {
        let key = self.demands.get(&id).expect("live demand").clone();
        if self.mode == Mode::Indexed {
            self.discover(&key)
        } else {
            self.tuples
                .get(&key)
                .into_iter()
                .flatten()
                .copied()
                .collect()
        }
    }
    /// Insert a projected row. Subscription updates enumerate only active endpoint keys.
    pub fn insert(&mut self, relation: usize, id: usize, row: Key) {
        self.relations[relation].insert(id, row.clone());
        if self.mode == Mode::Indexed {
            return;
        }
        if self.mode == Mode::Subscribed {
            let keys: Vec<_> = match relation {
                0 => self
                    .active_left
                    .get(&row.0)
                    .into_iter()
                    .flatten()
                    .cloned()
                    .collect(),
                2 => self
                    .active_right
                    .get(&row.1)
                    .into_iter()
                    .flatten()
                    .cloned()
                    .collect(),
                _ => self.subscribers.keys().cloned().collect(),
            };
            let mut triples = Vec::new();
            for key in keys {
                count!(self.stats, probes);
                match relation {
                    0 => {
                        for &m in self.relations[1].first.get(&row.1).into_iter().flatten() {
                            count!(self.stats, probes);
                            let b = &self.relations[1].rows[&m].1;
                            for &r in self.relations[2]
                                .exact
                                .get(&(b.clone(), key.1.clone()))
                                .into_iter()
                                .flatten()
                            {
                                triples.push([id, m, r]);
                            }
                        }
                    }
                    1 => {
                        for &l in self.relations[0]
                            .exact
                            .get(&(key.0.clone(), row.0.clone()))
                            .into_iter()
                            .flatten()
                        {
                            count!(self.stats, probes);
                            for &r in self.relations[2]
                                .exact
                                .get(&(row.1.clone(), key.1.clone()))
                                .into_iter()
                                .flatten()
                            {
                                triples.push([l, id, r]);
                            }
                        }
                    }
                    2 => {
                        for &m in self.relations[1].second.get(&row.0).into_iter().flatten() {
                            count!(self.stats, probes);
                            let a = &self.relations[1].rows[&m].0;
                            for &l in self.relations[0]
                                .exact
                                .get(&(key.0.clone(), a.clone()))
                                .into_iter()
                                .flatten()
                            {
                                triples.push([l, m, id]);
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            for t in triples {
                self.retain(t);
            }
        } else {
            let mut triples = Vec::new();
            match relation {
                0 => {
                    for &m in self.relations[1].first.get(&row.1).into_iter().flatten() {
                        count!(self.stats, probes);
                        for &r in self.relations[2]
                            .first
                            .get(&self.relations[1].rows[&m].1)
                            .into_iter()
                            .flatten()
                        {
                            triples.push([id, m, r]);
                        }
                    }
                }
                1 => {
                    for &l in self.relations[0].second.get(&row.0).into_iter().flatten() {
                        count!(self.stats, probes);
                        for &r in self.relations[2].first.get(&row.1).into_iter().flatten() {
                            triples.push([l, id, r]);
                        }
                    }
                }
                2 => {
                    for &m in self.relations[1].second.get(&row.0).into_iter().flatten() {
                        count!(self.stats, probes);
                        for &l in self.relations[0]
                            .second
                            .get(&self.relations[1].rows[&m].0)
                            .into_iter()
                            .flatten()
                        {
                            triples.push([l, m, id]);
                        }
                    }
                }
                _ => unreachable!(),
            }
            for t in triples {
                self.retain(t);
            }
        }
    }
    pub fn remove(&mut self, relation: usize, id: usize) {
        let tuples = self.incidence[relation]
            .get(&id)
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        for t in tuples {
            self.forget(t);
        }
        self.relations[relation].remove(id);
    }
    pub fn retained(&self) -> usize {
        self.tuples.values().map(BTreeSet::len).sum()
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
}
