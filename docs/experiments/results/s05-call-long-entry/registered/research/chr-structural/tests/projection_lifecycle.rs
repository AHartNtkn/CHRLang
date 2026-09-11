use chr_structural::projection::{Problem, Relation, Semantics};
use std::collections::BTreeMap;

fn oracle(
    p: &Problem,
    visible: &[usize],
    restrictions: &[(usize, u8)],
    set: bool,
) -> BTreeMap<Vec<u8>, u128> {
    let mut out = BTreeMap::new();
    for mut index in 0..p.domains.iter().map(Vec::len).product::<usize>() {
        let mut values = vec![0; p.domains.len()];
        for i in (0..values.len()).rev() {
            values[i] = p.domains[i][index % p.domains[i].len()];
            index /= p.domains[i].len();
        }
        if restrictions.iter().any(|&(i, v)| values[i] != v)
            || p.filters.iter().any(|f| {
                !f.rows
                    .iter()
                    .any(|row| f.scope.iter().zip(row).all(|(&i, &v)| values[i] == v))
            })
        {
            continue;
        }
        let count = out
            .entry(visible.iter().map(|&i| values[i]).collect())
            .or_insert(0);
        if set {
            *count = 1;
        } else {
            *count += 1;
        }
    }
    out
}

#[test]
fn generated_orders_restrictions_and_expansion_match_full_enumeration() {
    let mut seed = 0x9e3779b9_u64;
    let mut random = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        (seed >> 32) as usize
    };
    let mut comparisons = 0;
    for case in 0..256 {
        let n = 3 + random() % 4;
        let mut domains = (0..n)
            .map(|_| (0..3).map(|_| (random() % 3) as u8).collect())
            .collect::<Vec<Vec<_>>>();
        if case % 17 == 0 {
            domains[0].clear();
        }
        let filters = (0..random() % 4)
            .map(|_| {
                let width = 1 + random() % 3;
                let start = random() % n;
                Relation {
                    scope: (0..width).map(|j| (start + j) % n).collect(),
                    rows: (0..6)
                        .map(|_| (0..width).map(|_| (random() % 3) as u8).collect())
                        .collect(),
                }
            })
            .collect();
        let p = Problem { domains, filters };
        for mask in 0..8 {
            let visible = (0..n).filter(|&i| mask & (1 << i) != 0).collect::<Vec<_>>();
            let ascending = (0..n).filter(|i| !visible.contains(i)).collect::<Vec<_>>();
            let greedy = p.elimination_order(&visible).unwrap();
            for semantics in [Semantics::Set, Semantics::Counted] {
                let want = oracle(&p, &visible, &[], semantics == Semantics::Set);
                for order in [
                    &ascending,
                    &ascending.iter().rev().copied().collect(),
                    &greedy,
                ] {
                    let q = p.project(&visible, &[], order, semantics, 100_000).unwrap();
                    assert_eq!(q.answers(&[], 100_000).unwrap(), want);
                    comparisons += 1;
                }
                let q = p
                    .project(&visible, &[], &greedy, semantics, 100_000)
                    .unwrap();
                let mut expanded = BTreeMap::new();
                for row in q.expanded_answers(&[], 100_000, 100_000).unwrap() {
                    *expanded.entry(row).or_insert(0) += 1;
                }
                assert_eq!(expanded, want);
                for &v in &visible {
                    for value in 0..3 {
                        assert_eq!(
                            q.answers(&[(v, value)], 100_000).unwrap(),
                            oracle(&p, &visible, &[(v, value)], semantics == Semantics::Set)
                        );
                    }
                }
            }
        }
    }
    assert_eq!(comparisons, 12_288);
    println!("complete weighted-map comparisons: {comparisons}");
}

#[test]
fn greedy_order_avoids_the_connected_star_intermediate() {
    let p = Problem {
        domains: vec![vec![0, 1]; 8],
        filters: (1..8)
            .map(|i| Relation {
                scope: vec![0, i],
                rows: vec![vec![0, 0], vec![0, 1], vec![1, 0]],
            })
            .collect(),
    };
    let order = p.elimination_order(&[6, 7]).unwrap();
    assert_eq!(order, vec![1, 2, 3, 4, 5, 0]);
    let q = p
        .project(&[6, 7], &[], &order, Semantics::Counted, 1000)
        .unwrap();
    assert_eq!(
        q.answers(&[], 100).unwrap(),
        oracle(&p, &[6, 7], &[], false)
    );
    if cfg!(feature = "metrics") {
        assert_eq!((q.elimination_visits, q.peak_entries), (28, 4));
    }
}

#[test]
fn dense_outputs_and_changing_restrictions_reuse_preparation() {
    let p = Problem {
        domains: vec![vec![0, 1]; 10],
        filters: vec![],
    };
    let visible = (0..10).rev().collect::<Vec<_>>();
    let q = p
        .project(
            &visible,
            &[],
            &p.elimination_order(&visible).unwrap(),
            Semantics::Counted,
            2048,
        )
        .unwrap();
    for r in [
        vec![],
        vec![(0, 0)],
        vec![(0, 1)],
        vec![(9, 1), (0, 0)],
        vec![(0, 0), (0, 1)],
    ] {
        assert_eq!(
            q.answers(&r, 2048).unwrap(),
            oracle(&p, &visible, &r, false)
        );
        assert_eq!(
            q.expanded_answers(&r, 2048, 2048).unwrap().count() as u128,
            oracle(&p, &visible, &r, false).values().sum()
        );
    }
}

#[test]
fn expansion_is_owned_bounded_and_cancellable_without_full_materialization() {
    let p = Problem {
        domains: vec![vec![0, 1]; 24],
        filters: vec![],
    };
    let q = p
        .project(
            &[0],
            &[],
            &p.elimination_order(&[0]).unwrap(),
            Semantics::Counted,
            100,
        )
        .unwrap();
    assert_eq!(
        q.expanded_answers(&[], 100, 100).err().unwrap(),
        "output bound"
    );
    let mut answers = q.expanded_answers(&[], 100, 1 << 24).unwrap();
    drop(q);
    drop(p);
    let mut first = answers.next().unwrap();
    first[0] = 9;
    for _ in 0..7 {
        assert_eq!(answers.next(), Some(vec![0]));
    }
    drop(answers); // Cancels the remaining represented outputs without visiting them.
    assert_eq!(first, vec![9]);
}

#[test]
fn grouped_expansion_preserves_multiplicity_but_not_source_choice_order() {
    let p = Problem {
        domains: vec![vec![0, 1]; 2],
        filters: vec![],
    };
    let q = p.project(&[1], &[], &[0], Semantics::Counted, 10).unwrap();
    let grouped = q.expanded_answers(&[], 10, 4).unwrap().collect::<Vec<_>>();
    let source_choices = p.domains[0]
        .iter()
        .flat_map(|_| p.domains[1].iter().map(|&v| vec![v]))
        .collect::<Vec<_>>();
    assert_eq!(source_choices, vec![vec![0], vec![1], vec![0], vec![1]]);
    assert_eq!(grouped, vec![vec![0], vec![0], vec![1], vec![1]]);
    assert_ne!(grouped, source_choices);
}

#[test]
fn empty_inputs_and_invalid_planning_inputs_are_explicit() {
    let p = Problem {
        domains: vec![],
        filters: vec![],
    };
    assert_eq!(p.elimination_order(&[]).unwrap(), Vec::<usize>::new());
    let q = p.project(&[], &[], &[], Semantics::Counted, 1).unwrap();
    assert_eq!(
        q.expanded_answers(&[], 1, 1).unwrap().collect::<Vec<_>>(),
        vec![Vec::<u8>::new()]
    );
    let mut p = Problem {
        domains: vec![vec![0], vec![]],
        filters: vec![],
    };
    assert_eq!(p.elimination_order(&[]).unwrap(), vec![1, 0]);
    assert!(
        p.project(&[], &[], &[1, 0], Semantics::Counted, 10)
            .unwrap()
            .expanded_answers(&[], 10, 0)
            .unwrap()
            .next()
            .is_none()
    );
    assert!(p.elimination_order(&[2]).is_err());
    assert!(p.elimination_order(&[0, 0]).is_err());
    p.filters.push(Relation {
        scope: vec![0, 2],
        rows: vec![vec![0, 0]],
    });
    assert!(p.elimination_order(&[]).is_err());
    p.filters[0].scope = vec![0, 0];
    assert!(p.elimination_order(&[]).is_err());
    p.filters[0].scope = vec![0];
    assert!(p.elimination_order(&[]).is_err());
}

#[test]
fn expansion_total_overflow_is_distinct_from_output_bound() {
    let p = Problem {
        domains: vec![vec![0, 1]; 128],
        filters: vec![],
    };
    let q = p
        .project(
            &[0],
            &[],
            &(1..128).collect::<Vec<_>>(),
            Semantics::Counted,
            100,
        )
        .unwrap();
    assert_eq!(
        q.answers(&[], 100)
            .unwrap()
            .values()
            .copied()
            .collect::<Vec<_>>(),
        vec![1_u128 << 127; 2]
    );
    assert_eq!(
        q.expanded_answers(&[], 100, u128::MAX).err().unwrap(),
        "count overflow"
    );
    assert_eq!(
        q.expanded_answers(&[(0, 0)], 100, (1_u128 << 127) - 1)
            .err()
            .unwrap(),
        "output bound"
    );
    assert!(q.expanded_answers(&[(1, 0)], 100, u128::MAX).is_err());
}
