use std::{
    cmp::Ordering,
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};
#[derive(Default, Debug)]
pub struct Storage {
    pub visits: u64,
    pub allocations: u64,
    pub snapshot_copies: u64,
}
type Link<K, V> = Option<Rc<Node<K, V>>>;
struct Node<K, V> {
    key: K,
    value: V,
    priority: u64,
    left: Link<K, V>,
    right: Link<K, V>,
}
#[derive(Clone)]
pub struct Map<K, V>(Link<K, V>);
impl<K, V> Default for Map<K, V> {
    fn default() -> Self {
        Self(None)
    }
}
fn node<K, V>(
    key: K,
    value: V,
    priority: u64,
    left: Link<K, V>,
    right: Link<K, V>,
    s: &mut Storage,
) -> Rc<Node<K, V>> {
    if crate::COLLECT_KERNEL_METRICS {
        s.allocations += 1;
    }
    Rc::new(Node {
        key,
        value,
        priority,
        left,
        right,
    })
}
fn put<K: Ord + Clone, V: Clone>(
    root: &Link<K, V>,
    key: K,
    value: V,
    priority: u64,
    s: &mut Storage,
) -> Rc<Node<K, V>> {
    let Some(n) = root else {
        return node(key, value, priority, None, None, s);
    };
    if crate::COLLECT_KERNEL_METRICS {
        s.visits += 1;
    }
    match key.cmp(&n.key) {
        Ordering::Equal => node(key, value, priority, n.left.clone(), n.right.clone(), s),
        Ordering::Less => {
            let child = put(&n.left, key, value, priority, s);
            if child.priority > n.priority {
                let right = node(
                    n.key.clone(),
                    n.value.clone(),
                    n.priority,
                    child.right.clone(),
                    n.right.clone(),
                    s,
                );
                node(
                    child.key.clone(),
                    child.value.clone(),
                    child.priority,
                    child.left.clone(),
                    Some(right),
                    s,
                )
            } else {
                node(
                    n.key.clone(),
                    n.value.clone(),
                    n.priority,
                    Some(child),
                    n.right.clone(),
                    s,
                )
            }
        }
        Ordering::Greater => {
            let child = put(&n.right, key, value, priority, s);
            if child.priority > n.priority {
                let left = node(
                    n.key.clone(),
                    n.value.clone(),
                    n.priority,
                    n.left.clone(),
                    child.left.clone(),
                    s,
                );
                node(
                    child.key.clone(),
                    child.value.clone(),
                    child.priority,
                    Some(left),
                    child.right.clone(),
                    s,
                )
            } else {
                node(
                    n.key.clone(),
                    n.value.clone(),
                    n.priority,
                    n.left.clone(),
                    Some(child),
                    s,
                )
            }
        }
    }
}
fn merge<K: Clone, V: Clone>(left: &Link<K, V>, right: &Link<K, V>, s: &mut Storage) -> Link<K, V> {
    match (left, right) {
        (None, _) => right.clone(),
        (_, None) => left.clone(),
        (Some(a), Some(b)) => {
            if crate::COLLECT_KERNEL_METRICS {
                s.visits += 1;
            }
            Some(if a.priority > b.priority {
                node(
                    a.key.clone(),
                    a.value.clone(),
                    a.priority,
                    a.left.clone(),
                    merge(&a.right, right, s),
                    s,
                )
            } else {
                node(
                    b.key.clone(),
                    b.value.clone(),
                    b.priority,
                    merge(left, &b.left, s),
                    b.right.clone(),
                    s,
                )
            })
        }
    }
}
fn erase<K: Ord + Clone, V: Clone>(root: &Link<K, V>, key: &K, s: &mut Storage) -> Link<K, V> {
    let n = root.as_ref()?;
    if crate::COLLECT_KERNEL_METRICS {
        s.visits += 1;
    }
    match key.cmp(&n.key) {
        Ordering::Equal => merge(&n.left, &n.right, s),
        Ordering::Less => Some(node(
            n.key.clone(),
            n.value.clone(),
            n.priority,
            erase(&n.left, key, s),
            n.right.clone(),
            s,
        )),
        Ordering::Greater => Some(node(
            n.key.clone(),
            n.value.clone(),
            n.priority,
            n.left.clone(),
            erase(&n.right, key, s),
            s,
        )),
    }
}
impl<K: Ord + Clone + Hash, V: Clone> Map<K, V> {
    /// Identity of immutable contents, not extensional equality. Both maps
    /// retain their roots, so allocation reuse cannot produce a false match.
    pub fn same_root(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (None, None) => true,
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
    pub fn get(&self, key: &K, s: &mut Storage) -> Option<V> {
        let mut cursor = &self.0;
        while let Some(n) = cursor {
            if crate::COLLECT_KERNEL_METRICS {
                s.visits += 1;
            }
            match key.cmp(&n.key) {
                Ordering::Equal => return Some(n.value.clone()),
                Ordering::Less => cursor = &n.left,
                Ordering::Greater => cursor = &n.right,
            }
        }
        None
    }
    pub fn insert(&mut self, key: K, value: V, s: &mut Storage) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        self.0 = Some(put(&self.0, key, value, hasher.finish(), s));
    }
    pub fn remove(&mut self, key: &K, s: &mut Storage) {
        self.0 = erase(&self.0, key, s);
    }
    pub fn range(&self, lo: &K, hi: &K, s: &mut Storage) -> Vec<(K, V)> {
        fn visit<K: Ord + Clone, V: Clone>(
            root: &Link<K, V>,
            lo: &K,
            hi: &K,
            out: &mut Vec<(K, V)>,
            s: &mut Storage,
        ) {
            if let Some(n) = root {
                if crate::COLLECT_KERNEL_METRICS {
                    s.visits += 1;
                }
                if &n.key > lo {
                    visit(&n.left, lo, hi, out, s);
                }
                if &n.key >= lo && &n.key <= hi {
                    out.push((n.key.clone(), n.value.clone()));
                }
                if &n.key < hi {
                    visit(&n.right, lo, hi, out, s);
                }
            }
        }
        let mut out = vec![];
        visit(&self.0, lo, hi, &mut out, s);
        out
    }
    pub fn entries(&self, s: &mut Storage) -> Vec<(K, V)> {
        fn visit<K: Clone, V: Clone>(root: &Link<K, V>, out: &mut Vec<(K, V)>, s: &mut Storage) {
            if let Some(n) = root {
                if crate::COLLECT_KERNEL_METRICS {
                    s.visits += 1;
                }
                visit(&n.left, out, s);
                out.push((n.key.clone(), n.value.clone()));
                visit(&n.right, out, s);
            }
        }
        let mut out = vec![];
        visit(&self.0, &mut out, s);
        out
    }
    pub fn copied(&self, s: &mut Storage) -> Self {
        fn copy<K: Clone, V: Clone>(root: &Link<K, V>, s: &mut Storage) -> Link<K, V> {
            root.as_ref().map(|n| {
                if crate::COLLECT_KERNEL_METRICS {
                    s.snapshot_copies += 1;
                }
                node(
                    n.key.clone(),
                    n.value.clone(),
                    n.priority,
                    copy(&n.left, s),
                    copy(&n.right, s),
                    s,
                )
            })
        }
        Self(copy(&self.0, s))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn old_snapshots_survive_replacements_and_removals() {
        let mut stats = Storage::default();
        let mut m = Map::default();
        m.insert(3, "old", &mut stats);
        m.insert(1, "one", &mut stats);
        m.insert(8, "eight", &mut stats);
        let old = m.clone();
        m.insert(3, "new", &mut stats);
        m.remove(&1, &mut stats);
        assert_eq!(old.get(&3, &mut stats), Some("old"));
        assert_eq!(old.get(&1, &mut stats), Some("one"));
        assert_eq!(m.get(&3, &mut stats), Some("new"));
        assert_eq!(m.get(&1, &mut stats), None);
        assert_eq!(m.range(&0, &9, &mut stats), vec![(3, "new"), (8, "eight")]);
    }
    #[test]
    fn indexed_ranges_preserve_key_order_and_boundaries() {
        let mut stats = Storage::default();
        let mut m = Map::default();
        for key in [(2, 4), (1, 8), (1, 2), (3, 0), (1, 5)] {
            m.insert(key, key.1, &mut stats);
        }
        assert_eq!(
            m.range(&(1, 0), &(1, 9), &mut stats),
            vec![((1, 2), 2), ((1, 5), 5), ((1, 8), 8)]
        );
        m.remove(&(1, 5), &mut stats);
        assert_eq!(m.range(&(1, 5), &(1, 8), &mut stats), vec![((1, 8), 8)]);
    }
}

#[cfg(test)]
mod model_test {
    use super::*;
    #[test]
    fn mixed_updates_and_snapshots_match_an_ordered_map() {
        let mut s = Storage::default();
        let mut actual = Map::default();
        let mut expected = std::collections::BTreeMap::new();
        let mut snapshots = vec![];
        let mut seed = 19u64;
        for i in 0..512 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let key = (seed >> 32) % 64;
            if i % 3 == 0 {
                actual.remove(&key, &mut s);
                expected.remove(&key);
            } else {
                actual.insert(key, i, &mut s);
                expected.insert(key, i);
            }
            if i % 37 == 0 {
                snapshots.push((actual.clone(), expected.clone()));
            }
            assert_eq!(
                actual.entries(&mut s),
                expected.iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>()
            );
        }
        for (old, expected) in snapshots {
            assert_eq!(
                old.entries(&mut s),
                expected.into_iter().collect::<Vec<_>>()
            );
        }
        let copied = actual.copied(&mut s);
        assert_eq!(actual.entries(&mut s), copied.entries(&mut s));
        if crate::COLLECT_KERNEL_METRICS {
            assert!(s.snapshot_copies > 0);
        }
    }
}
