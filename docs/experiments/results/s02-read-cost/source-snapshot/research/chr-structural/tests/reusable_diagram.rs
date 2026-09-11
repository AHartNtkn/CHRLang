use chr_structural::reusable_diagram::{Diagram, Expr};
const LIMIT: usize = 1_000_000;
fn assignment(mut code: usize, k: usize) -> Vec<usize> {
    (0..3)
        .map(|_| {
            let x = code % k;
            code /= k;
            x
        })
        .collect()
}
fn accepts(mask: usize, equal: bool, a: &[usize]) -> bool {
    (!equal || a[0] == a[1])
        && [(0, 1), (0, 2), (1, 2)]
            .iter()
            .enumerate()
            .all(|(i, (x, y))| mask & (1 << i) == 0 || a[*x] != a[*y])
}
fn formula(mask: usize, equal: bool) -> Expr {
    let mut clauses = Vec::new();
    for (i, (x, y)) in [(0, 1), (0, 2), (1, 2)].into_iter().enumerate() {
        if mask & (1 << i) != 0 {
            clauses.push(Expr::Different(x, y));
        }
    }
    if equal {
        clauses.push(Expr::Equal(0, 1));
    }
    Expr::And(clauses)
}
#[test]
fn independent_assignment_union_projection_and_inclusion() {
    let mut checks = 0;
    for k in 1usize..=3 {
        let assignments = (0..k.pow(3)).map(|i| assignment(i, k)).collect::<Vec<_>>();
        let mut diagrams = Vec::new();
        let mut truths = Vec::new();
        for mask in 0..8 {
            for equal in [false, true] {
                let d = Diagram::compile(3, k, &formula(mask, equal), LIMIT).unwrap();
                let truth = assignments
                    .iter()
                    .map(|a| accepts(mask, equal, a))
                    .collect::<Vec<_>>();
                for (a, want) in assignments.iter().zip(&truth) {
                    assert_eq!(d.contains(a).unwrap(), *want);
                    checks += 1;
                }
                for hidden in 0..8 {
                    let forgotten = (0..3)
                        .filter(|i| hidden & (1 << i) != 0)
                        .collect::<Vec<_>>();
                    let p = d.exists(&forgotten, LIMIT).unwrap();
                    for a in &assignments {
                        let want = assignments.iter().zip(&truth).any(|(b, t)| {
                            *t && (0..3).all(|i| forgotten.contains(&i) || a[i] == b[i])
                        });
                        assert_eq!(p.contains(a).unwrap(), want);
                        checks += 1;
                    }
                }
                diagrams.push(d);
                truths.push(truth);
            }
        }
        for (i, a) in diagrams.iter().enumerate() {
            for (j, b) in diagrams.iter().enumerate() {
                let union = a.union(b, LIMIT).unwrap();
                let both = a.intersection(b, LIMIT).unwrap();
                for (q, v) in assignments.iter().enumerate() {
                    assert_eq!(union.contains(v).unwrap(), truths[i][q] || truths[j][q]);
                    assert_eq!(both.contains(v).unwrap(), truths[i][q] && truths[j][q]);
                    checks += 2;
                }
                assert_eq!(
                    a.included_in(b, LIMIT).unwrap(),
                    truths[i].iter().zip(&truths[j]).all(|(x, y)| !*x || *y)
                );
                checks += 1;
            }
        }
    }
    println!("independent_denotation_checks={checks}");
}
#[test]
fn compactness_and_invalid_requests_have_explicit_outcomes() {
    let universal = Diagram::compile(3, 3, &Expr::And(vec![]), LIMIT).unwrap();
    assert_eq!(universal.node_count(), 2);
    let empty = Diagram::compile(3, 3, &Expr::Different(1, 1), LIMIT).unwrap();
    assert!(!empty.contains(&[0, 0, 0]).unwrap());
    assert!(universal.contains(&[0, 0]).is_err());
    assert!(universal.contains(&[0, 0, 3]).is_err());
    assert!(universal.exists(&[3], LIMIT).is_err());
    assert!(Diagram::compile(3, 3, &Expr::Equal(0, 3), LIMIT).is_err());
    assert!(Diagram::compile(3, 0, &Expr::And(vec![]), LIMIT).is_err());
    assert!(Diagram::compile(3, 3, &Expr::Equal(0, 1), 0).is_err());
    assert!(universal.union(&empty, 0).is_err());
    let other = Diagram::compile(3, 2, &Expr::And(vec![]), LIMIT).unwrap();
    assert!(universal.union(&other, LIMIT).is_err());
    let zero = Diagram::compile(0, 2, &Expr::And(vec![]), LIMIT).unwrap();
    assert!(zero.contains(&[]).unwrap());
}
