//! General generated execution and activation on qualified passive-post sources.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
#[path = "../../chr-direct-conditional/tests/runtime_support/post_source.rs"]
mod source;
use chr_compiled::{Access, Policy, PreparedRuleset};
#[test]
fn generated_post_controls_preserve_complete_sources() {
    let mut count = 0;
    for (id, family) in source::FAMILIES.iter().enumerate() {
        let rules = source::rules(family);
        let preparations = [
            PreparedRuleset::new(rules.clone(), None).unwrap(),
            PreparedRuleset::new(rules.clone(), Some(chr_compiled::post_bundled(id))).unwrap(),
            PreparedRuleset::new(rules.clone(), Some(chr_compiled::access_post_bundled(id)))
                .unwrap(),
        ];
        let mut held = vec![];
        for n in [1, 8, 32] {
            for reverse in [false, true] {
                for value in ["a", "b"] {
                    let q = source::query(n, family, reverse, value);
                    let expected = source::expected(n, family, value);
                    scalar::same_raw(scalar::run(&rules, &q, 2_000_000), expected.clone());
                    let mut traces = std::collections::BTreeMap::new();
                    for (mode, prep) in preparations.iter().enumerate() {
                        for policy in [Policy::Global, Policy::Active] {
                            for access in [Access::Scan, Access::Indexed] {
                                for advance in [false, true] {
                                    let mut cancelled =
                                        prep.start(q.clone(), policy, access).unwrap();
                                    if advance {
                                        cancelled.advance(1);
                                    }
                                    drop(cancelled);
                                }
                                let mut e = prep.start(q.clone(), policy, access).unwrap();
                                e.enable_trace();
                                assert!(
                                    e.advance(2_000_000).exhausted,
                                    "cutoff {family} {n} {reverse} {value} {mode} {policy:?} {access:?}"
                                );
                                let key = format!("{policy:?}-{access:?}");
                                if mode == 0 {
                                    traces.insert(key, e.trace().to_vec());
                                } else {
                                    assert_eq!(e.trace(), traces[&key]);
                                }
                                let w = e.stats();
                                println!(
                                    "POST_CONTROL,{family},{n},{reverse},{value},{mode},{policy:?},{access:?},{},{},{},{},{},{},{},{}",
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
                                if chr_compiled::COLLECT_METRICS && mode == 2 {
                                    assert_eq!(
                                        (
                                            w.binding_slot_copies,
                                            w.cursor_pool_entries,
                                            w.key_template_visits
                                        ),
                                        (0, 0, 0)
                                    );
                                }
                                let got = e.observe().into_iter().collect::<Vec<_>>();
                                scalar::same_raw(got.clone(), expected.clone());
                                held.push((got, expected.clone()));
                                count += 1;
                            }
                        }
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
    assert_eq!(count, 864);
    println!("POST_CONTROL_COMPLETE,{count}");
}
