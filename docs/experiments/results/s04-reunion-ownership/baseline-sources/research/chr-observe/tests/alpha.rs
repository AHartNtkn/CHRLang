use chr_observe::{Stats, equivalent};
use chr_syntax::{Answer, Constraint, Term, atom, c, t, v};
fn a(outputs: Vec<Term>, residual: Vec<Constraint>) -> Answer {
    Answer {
        outputs: outputs
            .into_iter()
            .enumerate()
            .map(|(i, t)| (i.to_string(), t))
            .collect(),
        residual,
    }
}
fn eq(a: &Answer, b: &Answer) -> bool {
    equivalent(a, b, &mut Stats::default())
}
#[test]
fn output_renaming_preserves_aliases_and_constructor_positions() {
    assert!(eq(
        &a(vec![t("f", [v(2)]), v(2)], vec![]),
        &a(vec![t("f", [v(9)]), v(9)], vec![])
    ));
    assert!(!eq(
        &a(vec![v(0), v(0)], vec![]),
        &a(vec![v(7), v(8)], vec![])
    ));
    assert!(!eq(&a(vec![v(0)], vec![]), &a(vec![atom("a")], vec![])));
}
#[test]
fn output_and_residual_share_one_injective_renaming() {
    assert!(eq(
        &a(vec![v(0)], vec![c("edge", [v(0), v(1)])]),
        &a(vec![v(8)], vec![c("edge", [v(8), v(9)])])
    ));
    assert!(!eq(
        &a(vec![v(0)], vec![c("edge", [v(0), v(1)])]),
        &a(vec![v(8)], vec![c("edge", [v(9), v(8)])])
    ));
}
#[test]
fn residual_permutations_require_backtracking_and_preserve_multiplicity() {
    let left = a(
        vec![],
        vec![
            c("edge", [v(0), v(1)]),
            c("edge", [v(1), v(2)]),
            c("node", [v(2)]),
        ],
    );
    let right = a(
        vec![],
        vec![
            c("edge", [v(7), v(8)]),
            c("node", [v(8)]),
            c("edge", [v(9), v(7)]),
        ],
    );
    assert!(eq(&left, &right));
    assert!(!eq(
        &a(vec![], vec![c("p", [v(0)]), c("p", [v(0)])]),
        &a(vec![], vec![c("p", [v(1)]), c("p", [v(2)])])
    ));
    assert!(!eq(
        &a(vec![], vec![c("p", [v(0)]), c("p", [v(0)])]),
        &a(vec![], vec![c("p", [v(1)])])
    ));
}
#[test]
fn equal_local_degrees_do_not_prove_global_equivalence() {
    let cycle = a(
        vec![],
        vec![
            c("e", [v(0), v(1)]),
            c("e", [v(1), v(2)]),
            c("e", [v(2), v(3)]),
            c("e", [v(3), v(0)]),
        ],
    );
    let pairs = a(
        vec![],
        vec![
            c("e", [v(0), v(1)]),
            c("e", [v(1), v(0)]),
            c("e", [v(2), v(3)]),
            c("e", [v(3), v(2)]),
        ],
    );
    assert!(eq(&cycle, &cycle));
    assert!(!eq(&cycle, &pairs));
}
#[test]
fn ground_terms_and_output_names_remain_observable() {
    assert!(eq(
        &a(vec![atom("a")], vec![c("p", [])]),
        &a(vec![atom("a")], vec![c("p", [])])
    ));
    assert!(!eq(
        &a(vec![atom("a")], vec![]),
        &a(vec![atom("b")], vec![])
    ));
    let mut named = a(vec![v(0)], vec![]);
    named.outputs[0].0 = "other".into();
    assert!(!eq(&a(vec![v(0)], vec![]), &named));
}

#[test]
fn exact_dedup_keeps_nonisomorphic_answers() {
    let mut set = chr_observe::AnswerSet::default();
    assert!(set.insert(a(vec![v(0)], vec![c("p", [v(0)])])));
    assert!(!set.insert(a(vec![v(7)], vec![c("p", [v(7)])])));
    assert!(set.insert(a(vec![v(7)], vec![c("p", [v(8)])])));
    assert_eq!(set.into_answers().len(), 2);
}

#[test]
fn all_three_vertex_directed_graphs_agree_with_permutation_oracle() {
    const EDGES: [(usize, usize); 6] = [(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)];
    const PERMS: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    fn graph(bits: u8) -> Answer {
        let mut r = (0..3).map(|i| c("node", [v(i)])).collect::<Vec<_>>();
        for (i, (x, y)) in EDGES.iter().enumerate() {
            if bits & (1 << i) != 0 {
                r.push(c("edge", [v(*x as u64), v(*y as u64)]));
            }
        }
        a(vec![], r)
    }
    for left in 0..64u8 {
        for right in 0..64u8 {
            let expected = PERMS.iter().any(|p| {
                EDGES.iter().enumerate().all(|(i, (x, y))| {
                    let target = EDGES
                        .iter()
                        .position(|&(a, b)| a == p[*x] && b == p[*y])
                        .unwrap();
                    ((left >> i) & 1) == ((right >> target) & 1)
                })
            });
            assert_eq!(
                eq(&graph(left), &graph(right)),
                expected,
                "graphs {left} and {right}"
            );
        }
    }
}
