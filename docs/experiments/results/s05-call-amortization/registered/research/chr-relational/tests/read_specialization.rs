#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[allow(dead_code)]
#[path = "../examples/support/read_near_source.rs"]
mod source;

#[test]
fn inferred_dispatch_executes_near_sources_and_preserves_owned_answers() {
    for n in [8, 32] {
        for kind in ["unique", "repeated", "mixed"] {
            for depth in [1, 8] {
                for token in [false, true] {
                    let rules = source::rules(n, kind, token);
                    let plain = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
                    for predicate in ["start"].into_iter().chain(token.then_some("take")) {
                        assert!(
                            plain
                                .region_eligibility()
                                .iter()
                                .any(|e| { e.predicate.0 == predicate && e.eligible })
                        );
                    }
                    let sealed = plain.specialize_inferred();
                    let mut held = Vec::new();
                    for reverse in [false, true] {
                        let query = source::query(depth + usize::from(reverse), token, reverse);
                        oracle::same_raw(
                            oracle::run(&rules, &query, 200_000),
                            source::expected(n, kind),
                        );
                        for (specialized, prepared) in [(false, &plain), (true, &sealed)] {
                            let mut search = prepared
                                .start_search(
                                    query.clone(),
                                    chr_compiled::Policy::Global,
                                    chr_compiled::Access::Indexed,
                                )
                                .unwrap();
                            let mut answers = Vec::new();
                            let mut done = false;
                            #[cfg(feature = "compiled-work")]
                            let mut applications = 0;
                            for _ in 0..200_000 {
                                match search.tick() {
                                    chr_compiled::SearchEvent::Complete(mut branch) => {
                                        #[cfg(feature = "compiled-work")]
                                        {
                                            applications +=
                                                branch.engine.stats().specialized_applications;
                                        }
                                        answers.push(branch.engine.observe().unwrap());
                                    }
                                    chr_compiled::SearchEvent::Exhausted => {
                                        done = true;
                                        break;
                                    }
                                    #[cfg(feature = "compiled-work")]
                                    chr_compiled::SearchEvent::Split { work, .. } => {
                                        applications += work.unwrap().specialized_applications;
                                    }
                                    #[cfg(feature = "compiled-work")]
                                    chr_compiled::SearchEvent::Failed(branch) => {
                                        applications +=
                                            branch.engine.stats().specialized_applications;
                                    }
                                    _ => (),
                                }
                            }
                            assert!(done);
                            #[cfg(feature = "compiled-work")]
                            assert_eq!(
                                applications,
                                if specialized {
                                    1 + if token {
                                        source::expected(n, kind).len() as u64
                                    } else {
                                        0
                                    }
                                } else {
                                    0
                                }
                            );
                            println!(
                                "SPECIALIZATION,{n},{kind},{depth},{token},{reverse},{specialized},{}",
                                answers.len()
                            );
                            drop(search);
                            held.push(answers);
                        }
                    }
                    drop(sealed);
                    drop(plain);
                    for answers in held {
                        oracle::same_raw(answers, source::expected(n, kind));
                    }
                }
            }
        }
    }
}
