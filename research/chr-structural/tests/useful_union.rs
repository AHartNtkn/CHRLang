//! Substantive finite-set union; source observations and multiplicity stay separate.
use chr_structural::{
    graph_simplification::Reduced,
    name_disequality::{Formula, Name},
    reusable_diagram::{Diagram, Expr},
};
use chr_syntax::{Answer, Goal, Query, Rule, Var, atom, c, eq, or, v};
use std::collections::{BTreeMap, BTreeSet};
const LIMIT: usize = 1_000_000;
type Branch = Vec<(usize, usize)>;
fn families(n: usize) -> Vec<(&'static str, Vec<Branch>)> {
    let cycle = (0..n).map(|i| (i, (i + 1) % n)).collect::<Vec<_>>();
    let overlap = (0..n)
        .map(|skip| {
            cycle
                .iter()
                .enumerate()
                .filter_map(|(i, e)| (i != skip).then_some(*e))
                .collect()
        })
        .collect();
    let disjoint = [(0, 1), (2, 3)]
        .iter()
        .map(|skip| {
            let mut edges = (0..4)
                .flat_map(|i| (i + 1..4).map(move |j| (i, j)))
                .filter(|e| e != skip)
                .collect::<Vec<_>>();
            edges.extend((4..n).map(|i| (i - 1, i)));
            edges
        })
        .collect();
    vec![
        ("overlap", overlap),
        ("disjoint", disjoint),
        ("redundant", vec![cycle.clone(); n]),
        ("single", vec![cycle]),
    ]
}
fn expr(b: &Branch) -> Expr {
    Expr::And(b.iter().map(|&(a, b)| Expr::Different(a, b)).collect())
}
fn assignment(mut code: usize, n: usize) -> Vec<usize> {
    (0..n)
        .map(|_| {
            let x = code % 3;
            code /= 3;
            x
        })
        .collect()
}
fn accepts(b: &Branch, a: &[usize]) -> bool {
    b.iter().all(|&(i, j)| a[i] != a[j])
}
fn caller(mode: usize, a: &[usize]) -> bool {
    match mode {
        0 => true,
        1 => a[0] == 0,
        2 => a[0] == 1,
        3 => a[0] == a[1],
        _ => false,
    }
}
fn minimum<T>(mut run: impl FnMut(usize) -> Result<T, String>) -> usize {
    assert!(run(LIMIT).is_ok());
    let (mut lo, mut hi) = (0, LIMIT);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if run(mid).is_ok() {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    assert!(run(lo).is_ok());
    if lo > 0 {
        assert!(run(lo - 1).is_err());
    }
    lo
}
fn source_check(branches: &[Branch], n: usize, mode: usize, counts: &BTreeMap<Vec<usize>, usize>) {
    let disj = |gs: Vec<Goal>| gs.into_iter().reduce(or).unwrap_or(Goal::Fail);
    let alternatives = branches
        .iter()
        .map(|b| {
            let mut gs = (0..n)
                .map(|i| {
                    disj(
                        (0..3)
                            .map(|x| eq(v(i as u64), atom(&format!("a{x}"))))
                            .collect(),
                    )
                })
                .collect::<Vec<_>>();
            gs.extend(
                b.iter()
                    .map(|&(i, j)| c("different", [v(i as u64), v(j as u64)]).into()),
            );
            Goal::And(gs)
        })
        .collect();
    let rules = vec![
        Rule::simplify(
            "choose",
            [c("start", (0..n).map(|i| v(i as u64)).collect::<Vec<_>>())],
            disj(alternatives),
        ),
        Rule::simplify("reject", [c("different", [v(0), v(0)])], Goal::Fail),
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
    ];
    let mut constraints = vec![c("start", (0..n).map(|i| v(i as u64)).collect::<Vec<_>>())];
    match mode {
        1 => constraints.push(c("bind", [v(0), atom("a0")])),
        2 => constraints.push(c("bind", [v(0), atom("a1")])),
        3 => constraints.push(c("bind", [v(0), v(1)])),
        4 => {
            constraints.push(c("bind", [v(0), atom("a0")]));
            constraints.push(c("bind", [v(0), atom("a1")]));
        }
        _ => (),
    }
    let q = Query {
        constraints,
        outputs: (0..n).map(|i| (format!("o{i}"), Var(i as u64))).collect(),
    };
    let mut search = chr_reference::Search::new(rules, q).unwrap();
    let batch = search.advance(200_000);
    assert!(batch.exhausted, "source cutoff");
    let mut observations = BTreeSet::new();
    for (a, _) in counts.iter().filter(|(a, _)| caller(mode, a)) {
        for b in branches.iter().filter(|b| accepts(b, a)) {
            let mut residual = b
                .iter()
                .map(|&(i, j)| {
                    c(
                        "different",
                        [atom(&format!("a{}", a[i])), atom(&format!("a{}", a[j]))],
                    )
                })
                .collect::<Vec<_>>();
            residual.sort();
            observations.insert(Answer {
                outputs: a
                    .iter()
                    .enumerate()
                    .map(|(i, x)| (format!("o{i}"), atom(&format!("a{x}"))))
                    .collect(),
                residual,
            });
        }
    }
    let count = counts
        .iter()
        .filter(|(a, _)| caller(mode, a))
        .map(|(_, c)| c)
        .sum::<usize>();
    assert_eq!(search.stats().completed_branches as usize, count);
    let got = batch
        .answers
        .into_iter()
        .map(|mut a| {
            a.residual.sort();
            a
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(got, observations);
}
#[test]
fn union_survives_substantive_constraints_and_preserves_correlations() {
    for n in [4, 6, 8] {
        for (family, branches) in families(n) {
            let mut compile_budget = 0;
            let diagrams = branches
                .iter()
                .map(|b| {
                    let e = expr(b);
                    compile_budget += minimum(|l| Diagram::compile(n, 3, &e, l));
                    Diagram::compile(n, 3, &e, LIMIT).unwrap()
                })
                .collect::<Vec<_>>();
            let compiled_nodes = diagrams.iter().map(Diagram::node_count).sum::<usize>();
            let mut union = diagrams[0].clone();
            let mut union_budget = 0;
            for d in &diagrams[1..] {
                union_budget += minimum(|l| union.union(d, l));
                assert!(union.union(d, 0).is_err());
                union = union.union(d, LIMIT).unwrap();
            }
            let reduced =
                Reduced::compile(n, 3, &branches, &(0..n).collect::<Vec<_>>(), LIMIT).unwrap();
            assert!(reduced.is_closed());
            assert!(reduced.constraints().iter().all(|b| !b.is_empty()));
            let names = branches
                .iter()
                .map(|b| {
                    Formula::compile(
                        n,
                        &[],
                        &b.iter()
                            .map(|&(i, j)| (Name::Variable(i), Name::Variable(j)))
                            .collect::<Vec<_>>(),
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();
            let alphabet = (0..3).map(|x| format!("a{x}")).collect::<Vec<_>>();
            let mut truth = BTreeMap::new();
            let (mut branch_probes, mut edge_probes) = (0, 0);
            for code in 0..3usize.pow(n as u32) {
                let a = assignment(code, n);
                let multiplicity = branches.iter().filter(|b| accepts(b, &a)).count();
                let want = multiplicity > 0;
                if want {
                    truth.insert(a.clone(), multiplicity);
                }
                let explicit = branches.iter().any(|b| {
                    branch_probes += 1;
                    b.iter().all(|&(i, j)| {
                        edge_probes += 1;
                        a[i] != a[j]
                    })
                });
                let given = a
                    .iter()
                    .enumerate()
                    .map(|(i, x)| (i, alphabet[*x].clone()))
                    .collect();
                assert_eq!(union.contains(&a).unwrap(), want);
                assert_eq!(diagrams.iter().any(|d| d.contains(&a).unwrap()), want);
                assert_eq!(reduced.contains(&a).unwrap(), want);
                assert_eq!(
                    names
                        .iter()
                        .any(|f| f.finite(&alphabet, &given).satisfiable),
                    want
                );
                assert_eq!(explicit, want);
            }
            assert!(!truth.is_empty());
            assert!(truth.len() < 3usize.pow(n as u32));
            // Every coordinate admits every name, but their independent product is unsound.
            for i in 0..n {
                assert_eq!(
                    truth.keys().map(|a| a[i]).collect::<BTreeSet<_>>(),
                    BTreeSet::from([0, 1, 2])
                );
            }
            assert!(!union.contains(&vec![0; n]).unwrap());
            let raw = truth.values().sum::<usize>();
            if family == "overlap" || family == "redundant" {
                assert!(raw > truth.len());
            }
            if family == "disjoint" {
                assert_eq!(raw, truth.len());
            }
            let mut held = vec![];
            for mode in 0..5 {
                let expected = truth
                    .keys()
                    .filter(|a| caller(mode, a))
                    .cloned()
                    .collect::<BTreeSet<_>>();
                let got = (0..3usize.pow(n as u32))
                    .map(|code| assignment(code, n))
                    .filter(|a| caller(mode, a) && union.contains(a).unwrap())
                    .collect::<BTreeSet<_>>();
                assert_eq!(got, expected);
                if n == 4 {
                    source_check(&branches, n, mode, &truth);
                }
                held.push((got, expected));
            }
            println!(
                "UNION,{family},{n},{},{},{},{},{},{},{},{},{}",
                branches.len(),
                truth.len(),
                raw,
                compiled_nodes,
                union.node_count(),
                compile_budget,
                union_budget,
                branch_probes,
                edge_probes
            );
            drop(diagrams);
            assert_eq!(union.contains(&vec![0; n]), Ok(false));
            drop(union);
            for (got, expected) in held {
                assert_eq!(got, expected);
            }
        }
    }
}
