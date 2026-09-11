use chr_structural::graph_simplification::Reduced;
use std::collections::BTreeSet;
const LIMIT: usize = 1_000_000;
fn assignments(n: usize, k: usize) -> Vec<Vec<usize>> {
    (0..k.pow(n as u32))
        .map(|mut x| {
            (0..n)
                .map(|_| {
                    let a = x % k;
                    x /= k;
                    a
                })
                .collect()
        })
        .collect()
}
fn edges(n: usize, mask: usize) -> Vec<(usize, usize)> {
    let pairs = (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
        .collect::<Vec<_>>();
    pairs
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| (mask & (1 << i) != 0).then_some(p))
        .collect()
}
fn original(
    branches: &[Vec<(usize, usize)>],
    all: &[Vec<usize>],
    visible: &[usize],
) -> BTreeSet<Vec<usize>> {
    all.iter()
        .filter(|a| {
            branches
                .iter()
                .any(|b| b.iter().all(|(x, y)| a[*x] != a[*y]))
        })
        .map(|a| visible.iter().map(|i| a[*i]).collect())
        .collect()
}
fn check(n: usize, k: usize, branches: &[Vec<(usize, usize)>], visible: &[usize]) -> usize {
    let all = assignments(n, k);
    let expected = original(branches, &all, visible);
    let reduced = Reduced::compile(n, k, branches, visible, LIMIT).unwrap();
    let actual = all
        .iter()
        .filter(|a| {
            reduced
                .constraints()
                .iter()
                .any(|b| b.iter().all(|r| (a[r.left] == a[r.right]) == r.equal))
        })
        .map(|a| visible.iter().map(|i| a[*i]).collect())
        .collect::<BTreeSet<Vec<_>>>();
    assert_eq!(
        actual, expected,
        "n={n} k={k} branches={branches:?} visible={visible:?}"
    );
    for a in assignments(visible.len(), k) {
        if reduced.is_closed() {
            assert_eq!(reduced.contains(&a).unwrap(), expected.contains(&a));
        } else {
            assert!(reduced.contains(&a).is_err());
        }
    }
    1
}
#[test]
fn exhaustive_graphs_and_unions_preserve_existential_denotation() {
    let mut graphs = 0;
    for k in 1..=3 {
        for mask in 0..64 {
            for output in 0..16 {
                let visible = (0..4)
                    .rev()
                    .filter(|i| output & (1 << i) != 0)
                    .collect::<Vec<_>>();
                graphs += check(4, k, &[edges(4, mask)], &visible);
            }
        }
    }
    let mut unions = 0;
    for k in [2, 3] {
        for a in 0..8 {
            for b in 0..8 {
                unions += check(3, k, &[edges(3, a), edges(3, b)], &[1, 0]);
            }
        }
    }
    assert_eq!(graphs, 3072);
    assert_eq!(unions, 128);
    println!("graph_projections={graphs} union_projections={unions}");
}
#[test]
fn unresolved_is_not_false_and_invalid_inputs_are_not_hidden() {
    let bipartite = (0..3)
        .flat_map(|x| (3..6).map(move |y| (x, y)))
        .collect::<Vec<_>>();
    let r = Reduced::compile(6, 3, std::slice::from_ref(&bipartite), &[], LIMIT).unwrap();
    assert!(!r.is_closed());
    assert!(r.contains(&[]).is_err());
    assert_eq!(
        original(&[bipartite], &assignments(6, 3), &[]),
        BTreeSet::from([vec![]])
    );
    for b in [vec![], vec![(0, 1), (1, 0), (0, 1)], vec![(0, 0)]] {
        check(3, 2, &[b], &[1, 0]);
    }
    check(3, 2, &[], &[0]);
    assert!(Reduced::compile(3, 2, &[vec![], vec![(0, 3)]], &[0], LIMIT).is_err());
    assert!(Reduced::compile(3, 2, &[vec![(0, 0), (0, 3)]], &[0], LIMIT).is_err());
    assert!(Reduced::compile(3, 2, &[], &[3], LIMIT).is_err());
    assert!(Reduced::compile(3, 2, &[], &[0, 0], LIMIT).is_err());
    assert!(Reduced::compile(3, 0, &[], &[0], LIMIT).is_err());
    assert!(Reduced::compile(3, 2, &[vec![]], &[0], 0).is_err());
    let r = Reduced::compile(3, 2, &[vec![]], &[1, 0], LIMIT).unwrap();
    assert!(r.contains(&[0]).is_err());
    assert!(r.contains(&[0, 2]).is_err());
}
