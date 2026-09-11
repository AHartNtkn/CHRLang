//! Positive inclusion of fixed choice assignments; no compatibility relaxation.
use std::collections::BTreeMap;

#[cfg(any(not(feature = "ordered-context"), test))]
fn lookup<K: Ord, V: Eq>(support: &BTreeMap<K, V>, context: &BTreeMap<K, V>) -> bool {
    support
        .iter()
        .all(|(key, value)| context.get(key) == Some(value))
}
#[cfg(any(all(feature = "ordered-context", not(feature = "seek-context")), test))]
fn ordered<K: Ord, V: Eq>(support: &BTreeMap<K, V>, context: &BTreeMap<K, V>) -> bool {
    let mut current = context.iter();
    for (key, value) in support {
        loop {
            let Some((candidate, actual)) = current.next() else {
                return false;
            };
            match candidate.cmp(key) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Equal => {
                    if actual != value {
                        return false;
                    }
                    break;
                }
                std::cmp::Ordering::Greater => return false,
            }
        }
    }
    true
}

#[cfg(any(feature = "seek-context", test))]
fn seeking<K: Ord, V: Eq>(support: &BTreeMap<K, V>, context: &BTreeMap<K, V>) -> bool {
    use std::{cmp::Ordering, ops::Bound};
    let mut current = context.range::<K, _>(..);
    for (key, value) in support {
        let Some((candidate, actual)) = current.next() else {
            return false;
        };
        match candidate.cmp(key) {
            Ordering::Equal => {
                if actual != value {
                    return false;
                }
            }
            Ordering::Greater => return false,
            Ordering::Less => {
                current = context.range::<K, _>((Bound::Included(key), Bound::Unbounded));
                let Some((candidate, actual)) = current.next() else {
                    return false;
                };
                if candidate.cmp(key) != Ordering::Equal || actual != value {
                    return false;
                }
            }
        }
    }
    true
}

pub(super) fn includes<K: Ord, V: Eq>(support: &BTreeMap<K, V>, context: &BTreeMap<K, V>) -> bool {
    #[cfg(feature = "seek-context")]
    return seeking(support, context);
    #[cfg(all(feature = "ordered-context", not(feature = "seek-context")))]
    return ordered(support, context);
    #[cfg(not(feature = "ordered-context"))]
    lookup(support, context)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn assignment(mut code: usize) -> [u8; 6] {
        std::array::from_fn(|_| {
            let value = (code % 3) as u8;
            code /= 3;
            value
        })
    }
    #[test]
    fn ordered_inclusion_matches_independent_assignment_truth() {
        let keys = [0, 1, 2, 7, 64, usize::MAX];
        let inputs: Vec<_> = (0..729)
            .map(|code| {
                let a = assignment(code);
                let map: BTreeMap<_, _> = keys
                    .iter()
                    .zip(a)
                    .filter(|(_, v)| *v != 0)
                    .map(|(k, v)| (*k, v == 2))
                    .collect();
                (a, map)
            })
            .collect();
        for (a, support) in &inputs {
            for (b, context) in &inputs {
                let expected = a.iter().zip(b).all(|(x, y)| *x == 0 || x == y);
                assert_eq!(
                    ordered(support, context),
                    expected,
                    "support={a:?} context={b:?}"
                );
                assert_eq!(lookup(support, context), expected);
                assert_eq!(seeking(support, context), expected);
                assert_eq!(includes(support, context), expected);
            }
        }
        println!("CONTEXT_TRUTH,cases=531441");
    }

    thread_local! { static COMPARISONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
    #[derive(PartialEq, Eq)]
    struct Key(usize);
    impl Ord for Key {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            COMPARISONS.with(|n| n.set(n.get() + 1));
            self.0.cmp(&other.0)
        }
    }
    impl PartialOrd for Key {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    fn measured(f: impl FnOnce() -> bool) -> (bool, usize) {
        COMPARISONS.with(|n| n.set(0));
        let value = f();
        (value, COMPARISONS.with(|n| n.get()))
    }
    #[test]
    fn ordered_comparisons_expose_dense_savings_and_sparse_costs() {
        for size in [16, 64, 256] {
            for pattern in ["dense", "prefix2", "suffix2", "spread4", "empty"] {
                let keys: Vec<usize> = match pattern {
                    "dense" => (0..size).collect(),
                    "prefix2" => vec![0, 1],
                    "suffix2" => vec![size - 2, size - 1],
                    "spread4" => vec![0, size / 3, 2 * size / 3, size - 1],
                    "empty" => vec![],
                    _ => unreachable!(),
                };
                for outcome in ["match", "first-conflict", "last-conflict", "missing"] {
                    if pattern == "empty" && outcome != "match" {
                        continue;
                    }
                    let support: BTreeMap<_, _> = keys.iter().map(|k| (Key(*k), false)).collect();
                    let context: BTreeMap<_, _> = (0..size)
                        .filter(|k| outcome != "missing" || Some(k) != keys.last())
                        .map(|k| {
                            let conflict = (outcome == "first-conflict"
                                && Some(&k) == keys.first())
                                || (outcome == "last-conflict" && Some(&k) == keys.last());
                            (Key(k), conflict)
                        })
                        .collect();
                    let (a, lookup_work) = measured(|| lookup(&support, &context));
                    let (b, ordered_work) = measured(|| ordered(&support, &context));
                    let (c, seek_work) = measured(|| seeking(&support, &context));
                    assert_eq!(c, a);
                    assert_eq!(a, outcome == "match");
                    assert_eq!(b, a);
                    if pattern == "dense" && outcome == "match" {
                        assert!(ordered_work < lookup_work);
                    }
                    if pattern == "suffix2" && outcome == "match" && size == 256 {
                        assert!(ordered_work > lookup_work);
                        assert!(seek_work < ordered_work);
                    }
                    println!(
                        "CONTEXT_WORK,size={size},pattern={pattern},outcome={outcome},lookup={lookup_work},ordered={ordered_work},seek={seek_work}"
                    );
                }
            }
        }
    }

    #[test]
    #[ignore = "explicit bounded native operation experiment"]
    fn native_operation_sizing() {
        use std::{hint::black_box, time::Instant};
        let mut case = 0;
        for size in [16, 64, 256] {
            for pattern in ["dense", "prefix2", "suffix2", "spread4", "empty"] {
                let keys: Vec<usize> = match pattern {
                    "dense" => (0..size).collect(),
                    "prefix2" => vec![0, 1],
                    "suffix2" => vec![size - 2, size - 1],
                    "spread4" => vec![0, size / 3, 2 * size / 3, size - 1],
                    "empty" => vec![],
                    _ => unreachable!(),
                };
                for outcome in ["match", "first-conflict", "last-conflict", "missing"] {
                    if pattern == "empty" && outcome != "match" {
                        continue;
                    }
                    let support: BTreeMap<_, _> = keys.iter().map(|k| (*k, false)).collect();
                    let context: BTreeMap<_, _> = (0..size)
                        .filter(|k| outcome != "missing" || Some(k) != keys.last())
                        .map(|k| {
                            let conflict = (outcome == "first-conflict"
                                && Some(&k) == keys.first())
                                || (outcome == "last-conflict" && Some(&k) == keys.last());
                            (k, conflict)
                        })
                        .collect();
                    for method in 0..3 {
                        for _ in 0..2000 {
                            black_box(match method {
                                0 => lookup(black_box(&support), black_box(&context)),
                                1 => ordered(black_box(&support), black_box(&context)),
                                _ => seeking(black_box(&support), black_box(&context)),
                            });
                        }
                    }
                    for rep in 0..9 {
                        for position in 0..3 {
                            let method = (case + rep + position) % 3;
                            let mut matches = 0usize;
                            let start = Instant::now();
                            match method {
                                0 => {
                                    for _ in 0..20_000 {
                                        matches += usize::from(black_box(lookup(
                                            black_box(&support),
                                            black_box(&context),
                                        )));
                                    }
                                }
                                1 => {
                                    for _ in 0..20_000 {
                                        matches += usize::from(black_box(ordered(
                                            black_box(&support),
                                            black_box(&context),
                                        )));
                                    }
                                }
                                _ => {
                                    for _ in 0..20_000 {
                                        matches += usize::from(black_box(seeking(
                                            black_box(&support),
                                            black_box(&context),
                                        )));
                                    }
                                }
                            }
                            let ns = start.elapsed().as_nanos();
                            assert_eq!(matches, if outcome == "match" { 20_000 } else { 0 });
                            println!(
                                "CONTEXT_TIME,size={size},pattern={pattern},outcome={outcome},rep={rep},method={method},ns={ns}"
                            );
                        }
                    }
                    case += 1;
                }
            }
        }
        assert_eq!(case, 51);
    }
}
