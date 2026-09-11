//! Actual generated execution and activation controls for retained deep matching.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
#[allow(dead_code)]
#[path = "../experiments/structural_prefix_source.rs"]
mod source;
use chr_compiled::{Access, Policy, PreparedRuleset};
// Independent closed-form endpoint for the FIFO activation counterexample.
// left_i is killed before right_i is activated. Until the final left is killed,
// each right activation can use the last left/middle pair, so n-1 rights fire.
fn active_kill_answer(q: &chr_syntax::Query, n: usize) -> chr_syntax::Answer {
    chr_syntax::Answer {
        outputs: q
            .outputs
            .iter()
            .enumerate()
            .map(|(i, (name, var))| {
                (
                    name.clone(),
                    if i + 1 < n {
                        chr_syntax::atom("hit")
                    } else {
                        chr_syntax::Term::Var(*var)
                    },
                )
            })
            .collect(),
        residual: q
            .constraints
            .iter()
            .enumerate()
            .filter(|(i, c)| c.name != "left" && (c.name != "right" || *i == 3 * n - 1))
            .map(|(_, c)| c.clone())
            .collect(),
    }
}
#[test]
fn generated_prefix_controls() {
    let mut configurations = 0;
    for family in ["sparse", "keyed", "kill", "miss"] {
        for n in [4, 8] {
            for depth in [0, 8, 32] {
                let rules = source::source(family, n, depth, 0).0;
                let id = source::program_id(family, depth);
                assert_eq!(rules, source::programs()[id]);
                let preparations = [
                    PreparedRuleset::new(rules.clone(), None).unwrap(),
                    PreparedRuleset::new(rules.clone(), Some(chr_compiled::prefix_bundled(id)))
                        .unwrap(),
                    PreparedRuleset::new(
                        rules.clone(),
                        Some(chr_compiled::access_prefix_bundled(id)),
                    )
                    .unwrap(),
                ];
                let mut held = vec![];
                for seed in [0, 1] {
                    let q = source::source(family, n, depth, seed).1;
                    let expected = scalar::run(&rules, &q, 200_000);
                    assert_eq!(expected.len(), 1);
                    let hits = usize::from(family == "sparse" || family == "keyed") * n;
                    assert_eq!(
                        expected[0]
                            .outputs
                            .iter()
                            .filter(|(_, x)| *x == chr_syntax::atom("hit"))
                            .count(),
                        hits
                    );
                    assert_eq!(
                        expected[0]
                            .residual
                            .iter()
                            .filter(|c| c.name == "right")
                            .count(),
                        n - hits
                    );
                    for (mode, prep) in preparations.iter().enumerate() {
                        for policy in [Policy::Global, Policy::Active] {
                            for access in [Access::Scan, Access::Indexed] {
                                let mut cancelled = prep.start(q.clone(), policy, access).unwrap();
                                cancelled.advance(1);
                                drop(cancelled);
                                let mut e = prep.start(q.clone(), policy, access).unwrap();
                                e.enable_trace();
                                assert!(
                                    e.advance(200_000).exhausted,
                                    "cutoff {family} {n} {depth} {mode} {policy:?} {access:?}"
                                );
                                let w = e.stats();
                                println!(
                                    "PREFIX,{family},{n},{depth},{seed},{mode},{policy:?},{access:?},{},{},{},{},{},{},{},{}",
                                    w.structural_tests,
                                    w.candidate_visits,
                                    w.generic_ast_visits,
                                    w.key_visits,
                                    w.binding_slot_copies,
                                    w.cursor_pool_entries,
                                    w.key_template_visits,
                                    w.applications
                                );
                                if chr_compiled::COLLECT_METRICS && mode > 0 {
                                    assert_eq!(w.generic_ast_visits, 0);
                                }
                                let got = e.observe().into_iter().collect::<Vec<_>>();
                                let contract = if family == "kill" && policy == Policy::Active {
                                    let mut trace = vec![];
                                    for i in 0..n {
                                        trace.push((0, vec![(3 * n) as u64, (3 * i) as u64]));
                                        if i + 1 < n {
                                            trace.push((
                                                1,
                                                vec![
                                                    (3 * (n - 1)) as u64,
                                                    (3 * (n - 1) + 1) as u64,
                                                    (3 * i + 2) as u64,
                                                ],
                                            ));
                                        }
                                    }
                                    assert_eq!(e.trace(), trace);
                                    vec![active_kill_answer(&q, n)]
                                } else {
                                    expected.clone()
                                };
                                scalar::same_raw(got.clone(), contract.clone());
                                held.push((got, contract));
                                configurations += 1;
                            }
                        }
                    }
                }
                drop(preparations);
                drop(rules);
                for (got, expected) in held {
                    scalar::same_raw(got, expected);
                }
            }
        }
    }
    assert_eq!(configurations, 576);
    println!("PREFIX_COMPLETE,{configurations}");
}
