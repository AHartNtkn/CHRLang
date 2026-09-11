use chr_structural::joint_region::{Observation, Predicate, Region};
use chr_syntax::{Query, Term, Var, atom, c, eq, v};
use std::collections::BTreeMap;
#[path = "../examples/support/ordered_hidden.rs"]
#[allow(dead_code)]
mod runtime;
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
#[allow(dead_code)]
mod scalar;
use runtime::*;
#[test]
fn contracted_hidden_choices_preserve_ordered_consuming_answers() {
    let (mut cases, mut cartesian, mut admitted) = (0, 0, 0);
    for n in [2, 4] {
        for duplicate in [false, true] {
            for reverse in [false, true] {
                let mut domain = ["a", "b", "c", if duplicate { "a" } else { "d" }]
                    .map(atom)
                    .to_vec();
                if reverse {
                    domain.reverse();
                }
                for visible in [0, n - 1] {
                    let region = Region {
                        domains: (0..n).map(|i| (Var(i as u64), domain.clone())).collect(),
                        predicates: (0..n)
                            .filter(|i| *i != visible)
                            .map(|i| Predicate::Different(v(visible as u64), v(i as u64)))
                            .collect(),
                    };
                    let prepared = region
                        .prepare_sparse(
                            &[Var(visible as u64)],
                            &[],
                            &[],
                            Observation::Counted,
                            100_000,
                        )
                        .unwrap();
                    let weights = prepared
                        .weighted_iter(&[], 100_000)
                        .unwrap()
                        .collect::<BTreeMap<_, _>>();
                    drop((prepared, region));
                    for priority in 0..3 {
                        for alias in [false, true] {
                            for padding in [0, 3] {
                                let (rs, q) = source(&domain, n, visible, priority, alias, padding);
                                let scalar = bag(scalar::run(&rs, &q, 1_000_000));
                                let raw = run(rs, q);
                                assert_eq!(bag(raw.clone()), scalar);
                                let source_indices = raw
                                    .iter()
                                    .map(|a| {
                                        a.outputs
                                            .iter()
                                            .filter(|(name, _)| name.starts_with("index"))
                                            .map(|(_, t)| {
                                                let Term::App(name, args) = t else { panic!() };
                                                assert!(args.is_empty());
                                                name.strip_prefix('i')
                                                    .unwrap()
                                                    .parse::<usize>()
                                                    .unwrap()
                                            })
                                            .collect::<Vec<_>>()
                                    })
                                    .collect::<Vec<_>>();
                                let ordered = Ordered::new(&domain, n, visible, &weights, padding)
                                    .collect::<Vec<_>>();
                                assert_eq!(
                                    ordered, source_indices,
                                    "n={n} dup={duplicate} reverse={reverse} visible={visible} priority={priority}"
                                );
                                let expected = raw.into_iter().map(strip).collect::<Vec<_>>();
                                let actual = ordered
                                    .iter()
                                    .map(|row| {
                                        let x = domain[row[visible]].clone();
                                        let rs = host(eq(v(0), x), vec![v(0)], priority, 0);
                                        let mut outputs = vec![("out".into(), Var(0))];
                                        if alias {
                                            outputs.push(("alias".into(), Var(0)));
                                        }
                                        let mut answer = run(
                                            rs,
                                            Query {
                                                constraints: vec![
                                                    c("start", [v(0)]),
                                                    c("watch", [v(0)]),
                                                    c("token", []),
                                                ],
                                                outputs,
                                            },
                                        );
                                        assert_eq!(answer.len(), 1);
                                        strip(answer.pop().unwrap())
                                    })
                                    .collect::<Vec<_>>();
                                assert_eq!(actual, expected);
                                // Every answer-boundary cancellation, including zero and exhaustion.
                                for cutoff in 0..=ordered.len() {
                                    let held = Ordered::new(&domain, n, visible, &weights, padding)
                                        .take(cutoff)
                                        .collect::<Vec<_>>();
                                    assert_eq!(held, source_indices[..cutoff]);
                                }
                                assert_eq!(
                                    Ordered::new(&domain, n, visible, &weights, padding)
                                        .collect::<Vec<_>>(),
                                    ordered
                                );
                                cartesian += 4usize.pow(n as u32);
                                admitted += ordered.len();
                                cases += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(cases, 192);
    assert_eq!((cartesian, admitted), (26_112, 9_600));
    eprintln!("cases={cases} source_assignments={cartesian} admitted_assignments={admitted}");
}
