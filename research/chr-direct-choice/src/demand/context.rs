//! Positive inclusion of fixed choice assignments; no compatibility relaxation.
use std::collections::BTreeMap;

macro_rules! checked {
    ($site:ident, $support:expr, $context:expr) => {{
        #[cfg(feature = "validity-profile")]
        {
            let support = $support;
            let context = $context;
            $crate::demand::validity_profile::measure(
                $crate::demand::validity_profile::Site::$site,
                support.len(),
                context.len(),
                || $crate::demand::context::includes(support, context),
            )
        }
        #[cfg(not(feature = "validity-profile"))]
        $crate::demand::context::includes($support, $context)
    }};
}
pub(crate) use checked;

#[cfg(any(not(feature = "ordered-context"), test))]
fn lookup<K: Ord, V: Eq>(support: &BTreeMap<K, V>, context: &BTreeMap<K, V>) -> bool {
    support.iter().all(|(key, value)| {
        #[cfg(feature = "validity-profile")]
        super::validity_profile::work(1, 0, 0);
        context.get(key) == Some(value)
    })
}
#[cfg(any(all(feature = "ordered-context", not(feature = "seek-context")), test))]
fn ordered<K: Ord, V: Eq>(support: &BTreeMap<K, V>, context: &BTreeMap<K, V>) -> bool {
    let mut current = context.iter();
    for (key, value) in support {
        #[cfg(feature = "validity-profile")]
        super::validity_profile::work(1, 0, 0);
        loop {
            #[cfg(feature = "validity-profile")]
            super::validity_profile::work(0, 1, 0);
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
    #[cfg(feature = "validity-profile")]
    super::validity_profile::work(0, 0, 1);
    let mut current = context.range::<K, _>(..);
    for (key, value) in support {
        #[cfg(feature = "validity-profile")]
        super::validity_profile::work(1, 0, 0);
        #[cfg(feature = "validity-profile")]
        super::validity_profile::work(0, 1, 0);
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
                #[cfg(feature = "validity-profile")]
                super::validity_profile::work(0, 0, 1);
                current = context.range::<K, _>((Bound::Included(key), Bound::Unbounded));
                #[cfg(feature = "validity-profile")]
                super::validity_profile::work(0, 1, 0);
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

#[cfg(all(test, feature = "validity-profile"))]
mod attribution_gate {
    use super::*;
    #[test]
    fn inclusion_counts_distinguish_success_and_early_rejection() {
        use crate::demand::validity_profile::{Site, reset, snapshot};
        for mismatch in [None, Some(0), Some(7)] {
            let support: BTreeMap<_, _> = (0..8).map(|k| (k, true)).collect();
            let context: BTreeMap<_, _> = (0..8).map(|k| (k, Some(k) != mismatch)).collect();
            reset();
            let result = checked!(Finite, &support, &context);
            assert_eq!(result, mismatch.is_none());
            let row = snapshot()[Site::Finite as usize];
            assert_eq!(row.calls, 1);
            assert_eq!(row.accepted, usize::from(mismatch.is_none()));
            assert_eq!(row.visited, mismatch.map_or(8, |k| k + 1));
            assert_eq!((row.support, row.context), (8, 8));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn selected<K: Ord, V: Eq>(support: &BTreeMap<K, V>, context: &BTreeMap<K, V>) -> bool {
        if support.len() > context.len() / 4 {
            seeking(support, context)
        } else {
            lookup(support, context)
        }
    }
    fn repaired<K: Ord, V: Eq>(support: &BTreeMap<K, V>, context: &BTreeMap<K, V>) -> bool {
        if context.len() <= 8 || support.len() <= context.len() / 4 {
            return lookup(support, context);
        }
        let Some((first, _)) = support.first_key_value() else {
            return true;
        };
        let mut current = context.range(first..);
        for (key, value) in support {
            let Some((candidate, actual)) = current.next() else {
                return false;
            };
            match candidate.cmp(key) {
                std::cmp::Ordering::Equal => {
                    if actual != value {
                        return false;
                    }
                }
                std::cmp::Ordering::Greater => return false,
                std::cmp::Ordering::Less => {
                    current = context.range(key..);
                    let Some((candidate, actual)) = current.next() else {
                        return false;
                    };
                    if candidate.cmp(key) != std::cmp::Ordering::Equal || actual != value {
                        return false;
                    }
                }
            }
        }
        true
    }
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
                assert_eq!(selected(support, context), expected);
                assert_eq!(repaired(support, context), expected);
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
    #[test]
    #[ignore = "registered native selection experiment"]
    fn native_selection_sizing() {
        use std::{hint::black_box, time::Instant};
        fn sample(mut f: impl FnMut() -> bool, expected: bool) -> u128 {
            let mut matches = 0;
            let start = Instant::now();
            for _ in 0..10_000 {
                matches += usize::from(black_box(f()));
            }
            let ns = start.elapsed().as_nanos();
            assert_eq!(matches, if expected { 10_000 } else { 0 });
            ns
        }
        let mut case = 0;
        for n in [0usize, 1, 2, 8, 16, 63, 64, 256] {
            let counts: std::collections::BTreeSet<_> = [
                0,
                1,
                2,
                (n / 4).saturating_sub(1),
                n / 4,
                n / 4 + 1,
                n / 2,
                n,
            ]
            .into_iter()
            .filter(|k| *k <= n)
            .collect();
            for k in counts {
                let mut placements = std::collections::BTreeSet::new();
                placements.insert((0..k).collect::<Vec<_>>());
                placements.insert((n - k..n).collect::<Vec<_>>());
                placements.insert(
                    (0..k)
                        .map(|i| if k <= 1 { 0 } else { i * (n - 1) / (k - 1) })
                        .collect::<Vec<_>>(),
                );
                for keys in placements {
                    for outcome in ["match", "first-conflict", "last-conflict", "missing"] {
                        if k == 0 && outcome != "match" {
                            continue;
                        }
                        let support: BTreeMap<_, _> = keys.iter().map(|k| (*k, false)).collect();
                        let context: BTreeMap<_, _> = (0..n)
                            .filter(|i| outcome != "missing" || Some(i) != keys.last())
                            .map(|i| {
                                (
                                    i,
                                    (outcome == "first-conflict" && Some(&i) == keys.first())
                                        || (outcome == "last-conflict" && Some(&i) == keys.last()),
                                )
                            })
                            .collect();
                        let expected = outcome == "match";
                        assert_eq!(lookup(&support, &context), expected);
                        assert_eq!(seeking(&support, &context), expected);
                        assert_eq!(selected(&support, &context), expected);
                        println!(
                            "SELECT_CASE,id={case},n={n},k={k},keys={keys:?},outcome={outcome}"
                        );
                        for _ in 0..2000 {
                            black_box(lookup(black_box(&support), black_box(&context)));
                            black_box(seeking(black_box(&support), black_box(&context)));
                            black_box(selected(black_box(&support), black_box(&context)));
                        }
                        for rep in 0..9 {
                            for position in 0..3 {
                                let method = (case + rep + position) % 3;
                                let ns = match method {
                                    0 => sample(
                                        || lookup(black_box(&support), black_box(&context)),
                                        expected,
                                    ),
                                    1 => sample(
                                        || seeking(black_box(&support), black_box(&context)),
                                        expected,
                                    ),
                                    _ => sample(
                                        || selected(black_box(&support), black_box(&context)),
                                        expected,
                                    ),
                                };
                                println!("SELECT_TIME,id={case},rep={rep},method={method},ns={ns}");
                            }
                        }
                        case += 1;
                    }
                }
            }
        }
        println!("SELECT_TOTAL,cases={case}");
    }
    #[test]
    #[ignore = "registered native selection experiment"]
    fn native_selection_confirmation() {
        use std::{hint::black_box, time::Instant};
        fn sample(mut f: impl FnMut() -> bool, expected: bool) -> u128 {
            let mut matches = 0;
            let start = Instant::now();
            for _ in 0..10_000 {
                matches += usize::from(black_box(f()));
            }
            let ns = start.elapsed().as_nanos();
            assert_eq!(matches, if expected { 10_000 } else { 0 });
            ns
        }
        let mut case = 0;
        for n in [0usize, 1, 2, 8, 16, 32, 63, 64, 65, 127, 256, 257] {
            let counts: std::collections::BTreeSet<_> = [
                0,
                1,
                2,
                (n / 4).saturating_sub(1),
                n / 4,
                n / 4 + 1,
                n / 2,
                n,
            ]
            .into_iter()
            .filter(|k| *k <= n)
            .collect();
            for k in counts {
                let mut placements = std::collections::BTreeSet::new();
                placements.insert((0..k).collect::<Vec<_>>());
                placements.insert((n - k..n).collect::<Vec<_>>());
                placements.insert(
                    (0..k)
                        .map(|i| if k <= 1 { 0 } else { i * (n - 1) / (k - 1) })
                        .collect::<Vec<_>>(),
                );
                for keys in placements {
                    for outcome in ["match", "first-conflict", "last-conflict", "missing"] {
                        if k == 0 && outcome != "match" {
                            continue;
                        }
                        let support: BTreeMap<_, _> = keys.iter().map(|k| (*k, false)).collect();
                        let context: BTreeMap<_, _> = (0..n)
                            .filter(|i| outcome != "missing" || Some(i) != keys.last())
                            .map(|i| {
                                (
                                    i,
                                    (outcome == "first-conflict" && Some(&i) == keys.first())
                                        || (outcome == "last-conflict" && Some(&i) == keys.last()),
                                )
                            })
                            .collect();
                        let expected = outcome == "match";
                        assert_eq!(lookup(&support, &context), expected);
                        assert_eq!(seeking(&support, &context), expected);
                        assert_eq!(repaired(&support, &context), expected);
                        println!(
                            "SELECT_CASE,id={case},n={n},k={k},keys={keys:?},outcome={outcome}"
                        );
                        for _ in 0..2000 {
                            black_box(lookup(black_box(&support), black_box(&context)));
                            black_box(seeking(black_box(&support), black_box(&context)));
                            black_box(repaired(black_box(&support), black_box(&context)));
                        }
                        for rep in 0..9 {
                            for position in 0..3 {
                                let method = (case + rep + position) % 3;
                                let ns = match method {
                                    0 => sample(
                                        || lookup(black_box(&support), black_box(&context)),
                                        expected,
                                    ),
                                    1 => sample(
                                        || seeking(black_box(&support), black_box(&context)),
                                        expected,
                                    ),
                                    _ => sample(
                                        || repaired(black_box(&support), black_box(&context)),
                                        expected,
                                    ),
                                };
                                println!("SELECT_TIME,id={case},rep={rep},method={method},ns={ns}");
                            }
                        }
                        case += 1;
                    }
                }
            }
        }
        println!("SELECT_TOTAL,cases={case}");
    }
    #[test]
    fn selection_failure_work_attribution() {
        for (n, k, start, conflict) in [
            (2, 1, 1, false),
            (16, 5, 11, true),
            (256, 65, 191, true),
            (256, 256, 0, false),
            (256, 4, 0, false),
        ] {
            let support: BTreeMap<_, _> = (start..start + k).map(|i| (Key(i), false)).collect();
            let context: BTreeMap<_, _> =
                (0..n).map(|i| (Key(i), conflict && i == start)).collect();
            let a = measured(|| lookup(&support, &context));
            let b = measured(|| seeking(&support, &context));
            let c = measured(|| selected(&support, &context));
            let d = measured(|| repaired(&support, &context));
            assert_eq!(d.0, a.0);
            println!(
                "REPAIR_WORK,n={n},k={k},start={start},conflict={conflict},comparisons={}",
                d.1
            );
            assert_eq!(a.0, !conflict);
            assert_eq!(b.0, a.0);
            assert_eq!(c.0, a.0);
            assert_eq!(c.1, if k > n / 4 { b.1 } else { a.1 });
            println!(
                "SELECT_WORK,n={n},k={k},start={start},conflict={conflict},lookup={},seek={},selection={}",
                a.1, b.1, c.1
            );
        }
    }
}
