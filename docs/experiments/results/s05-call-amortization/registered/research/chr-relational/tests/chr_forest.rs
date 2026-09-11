//! Operator gate: represented classes are ground names, merged by CHR rules.
#[path = "support/chr_forest.rs"]
mod forest;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_syntax::{atom, c, Answer, Query, Term, Var};
fn node(i: usize) -> Term {
    atom(&format!("n{i}"))
}
fn partition(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<bool>> {
    let mut reach = vec![vec![false; n]; n];
    for (i, row) in reach.iter_mut().enumerate() {
        row[i] = true;
    }
    for &(a, b) in edges {
        reach[a][b] = true;
        reach[b][a] = true;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                reach[i][j] |= reach[i][k] && reach[k][j];
            }
        }
    }
    reach
}
// Check representation independently of the returned find answers: exactly one
// root/outgoing edge per ground node, in-domain parents, and no directed cycles.
fn check_forest(answer: &Answer, n: usize) -> Vec<usize> {
    let id = |t: &Term| (0..n).find(|&i| *t == node(i)).expect("foreign node");
    let mut parents = vec![None; n];
    for fact in &answer.residual {
        let (child, parent) = match (fact.name.as_str(), fact.args.as_slice()) {
            ("root", [x]) => (id(x), id(x)),
            ("edge", [x, y]) => {
                assert_ne!(x, y, "self edge");
                (id(x), id(y))
            }
            _ => panic!("unfinished or malformed forest fact: {fact:?}"),
        };
        assert!(parents[child].replace(parent).is_none(), "multiple parents");
    }
    let parents: Vec<_> = parents
        .into_iter()
        .map(|p| p.expect("missing node"))
        .collect();
    (0..n)
        .map(|start| {
            let mut seen = vec![false; n];
            let mut current = start;
            while parents[current] != current {
                assert!(!seen[current], "parent cycle");
                seen[current] = true;
                current = parents[current];
            }
            current
        })
        .collect()
}
fn compiled(rules: Vec<chr_syntax::Rule>, q: Query, access: chr_compiled::Access) -> Answer {
    let p = chr_compiled::PreparedRuleset::new(rules, None).unwrap();
    let mut e = p
        .start_search(q, chr_compiled::Policy::Global, access)
        .unwrap();
    let mut answers = vec![];
    let mut done = false;
    for _ in 0..200_000 {
        match e.tick() {
            chr_compiled::SearchEvent::Complete(mut b) => answers.push(b.engine.observe().unwrap()),
            chr_compiled::SearchEvent::Exhausted => {
                done = true;
                break;
            }
            _ => (),
        }
    }
    assert!(done);
    assert_eq!(answers.len(), 1);
    answers.pop().unwrap()
}
#[test]
fn chr_rules_compute_graph_connectivity_without_host_class_unification() {
    let n = 4;
    for compress in [false, true] {
        let rules = forest::rules(compress);
        for a in 0..n {
            for b in 0..n {
                for x in 0..n {
                    for y in 0..n {
                        let edges = [(a, b), (x, y)];
                        let expected = partition(n, &edges);
                        for reverse in [false, true] {
                            let mut constraints =
                                (0..n).map(|i| c("root", [node(i)])).collect::<Vec<_>>();
                            constraints
                                .extend(edges.iter().map(|&(i, j)| c("union", [node(i), node(j)])));
                            if reverse {
                                constraints.reverse();
                            }
                            let query = Query {
                                constraints,
                                outputs: vec![],
                            };
                            let scalar = oracle::run(&rules, &query, 200_000);
                            assert_eq!(scalar.len(), 1);
                            for access in
                                [chr_compiled::Access::Scan, chr_compiled::Access::Indexed]
                            {
                                let answer = compiled(rules.clone(), query.clone(), access);
                                oracle::same_raw(vec![answer.clone()], scalar.clone());
                                assert!(answer
                                    .residual
                                    .iter()
                                    .all(|c| matches!(c.name.as_str(), "root" | "edge")));
                                let representatives = check_forest(&answer, n);
                                for (i, row) in expected.iter().enumerate() {
                                    for (j, connected) in row.iter().enumerate() {
                                        assert_eq!(
                                            representatives[i] == representatives[j],
                                            *connected
                                        );
                                    }
                                }
                                let mut constraints = answer.residual;
                                constraints.extend(
                                    (0..n).map(|i| c("find", [node(i), chr_syntax::v(i as u64)])),
                                );
                                let q = Query {
                                    constraints,
                                    outputs: (0..n)
                                        .map(|i| (format!("r{i}"), Var(i as u64)))
                                        .collect(),
                                };
                                let result = compiled(rules.clone(), q.clone(), access);
                                oracle::same_raw(
                                    vec![result.clone()],
                                    oracle::run(&rules, &q, 200_000),
                                );
                                check_forest(&result, n);
                                for (i, row) in expected.iter().enumerate() {
                                    for (j, connected) in row.iter().enumerate() {
                                        assert_eq!(
                                            result.outputs[i].1 == result.outputs[j].1,
                                            *connected
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn source_compression_shortens_a_real_path() {
    for n in [3, 8, 24] {
        let mut constraints = (0..n - 1)
            .map(|i| c("edge", [node(i), node(i + 1)]))
            .collect::<Vec<_>>();
        constraints.push(c("root", [node(n - 1)]));
        constraints.push(c("find", [node(0), chr_syntax::v(0)]));
        let query = Query {
            constraints,
            outputs: vec![("root".into(), Var(0))],
        };
        for compress in [false, true] {
            let rules = forest::rules(compress);
            let runs = oracle::run_traced(&rules, &query, 200_000);
            assert_eq!(runs.len(), 1);
            let (answer, trace) = &runs[0];
            assert_eq!(
                trace
                    .iter()
                    .filter(|&&i| rules[i].name == "find-edge")
                    .count(),
                n - 1
            );
            assert_eq!(
                trace
                    .iter()
                    .filter(|&&i| rules[i].name == "compress-path")
                    .count(),
                if compress { n - 1 } else { 0 }
            );
            check_forest(answer, n);
            for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                oracle::same_raw(
                    vec![compiled(rules.clone(), query.clone(), access)],
                    vec![answer.clone()],
                );
            }
            let mut again = query.clone();
            again.constraints = answer.residual.clone();
            again
                .constraints
                .push(c("find", [node(0), chr_syntax::v(0)]));
            let repeated = oracle::run_traced(&rules, &again, 200_000);
            assert_eq!(repeated.len(), 1);
            assert_eq!(
                repeated[0]
                    .1
                    .iter()
                    .filter(|&&i| rules[i].name == "find-edge")
                    .count(),
                if compress { 1 } else { n - 1 }
            );
        }
    }
}

#[test]
fn stale_representatives_require_source_redirects() {
    use chr_syntax::t;
    for left in [false, true] {
        let pair = if left { [0, 2] } else { [2, 0] };
        let query = Query {
            constraints: vec![
                c("edge", [node(0), node(1)]),
                c("root", [node(1)]),
                c("root", [node(2)]),
                c("link", pair.map(|i| t("found", [node(i)]))),
            ],
            outputs: vec![],
        };
        for compress in [false, true] {
            let rules = forest::rules(compress);
            let runs = oracle::run_traced(&rules, &query, 200_000);
            assert_eq!(runs.len(), 1);
            let (answer, trace) = &runs[0];
            let redirect = if left {
                "redirect-left"
            } else {
                "redirect-right"
            };
            assert!(trace.iter().any(|&i| rules[i].name == redirect));
            let roots = check_forest(answer, 3);
            assert!(roots.iter().all(|&r| r == roots[0]));
            for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                oracle::same_raw(
                    vec![compiled(rules.clone(), query.clone(), access)],
                    vec![answer.clone()],
                );
            }
            // An adverse implementation mutation must strand the request.
            let broken = rules
                .into_iter()
                .filter(|r| r.name != redirect)
                .collect::<Vec<_>>();
            let stranded = oracle::run(&broken, &query, 200_000);
            assert_eq!(stranded.len(), 1);
            assert!(stranded[0].residual.iter().any(|c| c.name == "link"));
        }
    }
}
