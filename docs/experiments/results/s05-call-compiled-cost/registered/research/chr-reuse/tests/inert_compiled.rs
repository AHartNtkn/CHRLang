#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../examples/support/inert_source.rs"]
mod source;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
#[test]
fn prepared_and_inferred_controls_preserve_deep_call_answers() {
    let mut cases = 0;
    let mut rejected = 0;
    for family in 0..6 {
        for depth in [0, 4, 32, 128] {
            for distinct in [false, true] {
                let rules = source::source(family, depth, distinct, 7).0;
                for mode in 0..3 {
                    let p = PreparedRuleset::new(rules.clone(), None).unwrap();
                    let p = if mode == 2 {
                        p.specialize_inferred()
                    } else {
                        p
                    };
                    let access = if mode == 1 {
                        Access::Indexed
                    } else {
                        Access::Scan
                    };
                    let mut held = vec![];
                    for policy in [Policy::Global, Policy::Active] {
                        for offset in [7, 1007] {
                            let (_, query) = source::source(family, depth, distinct, offset);
                            if mode == 2 && matches!(policy, Policy::Active) {
                                assert!(p.start_search(query, policy, access).is_err());
                                rejected += 1;
                                continue;
                            }
                            let expected = oracle::run(&rules, &query, 200_000);
                            let direct = chr_reuse::continuations::Prepared::new(
                                rules.clone(),
                                chr_reuse::continuations::Mode::Direct,
                            )
                            .unwrap();
                            let direct = direct.start(query.clone()).unwrap().advance(200_000);
                            assert!(direct.exhausted);
                            let mut cancelled =
                                p.start_search(query.clone(), policy, access).unwrap();
                            drop(cancelled.tick());
                            drop(cancelled);
                            let mut run = p.start_search(query, policy, access).unwrap();
                            let mut answers = vec![];
                            let mut complete = false;
                            for _ in 0..2_000_000 {
                                match run.tick() {
                                    SearchEvent::Complete(mut branch) => {
                                        answers.push(branch.engine.observe().unwrap())
                                    }
                                    SearchEvent::Exhausted => {
                                        complete = true;
                                        break;
                                    }
                                    _ => (),
                                }
                            }
                            assert!(complete, "compiled service bound");
                            drop(run);
                            oracle::same_raw(answers.clone(), expected.clone());
                            assert_eq!(answers.len(), direct.answers.len());
                            for (actual, expected) in answers.iter().zip(&direct.answers) {
                                assert!(chr_observe::equivalent(
                                    actual,
                                    expected,
                                    &mut Default::default()
                                ));
                            }
                            held.push((answers, expected));
                            cases += 1;
                        }
                    }
                    drop(p);
                    for (answers, expected) in held {
                        oracle::same_raw(answers, expected);
                    }
                }
            }
        }
    }
    assert_eq!(cases, 480);
    assert_eq!(rejected, 96);
}
