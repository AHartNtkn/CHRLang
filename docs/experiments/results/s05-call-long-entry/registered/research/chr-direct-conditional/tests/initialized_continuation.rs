#[allow(dead_code)]
mod composition_support;
#[allow(dead_code)]
mod runtime_support;
#[path = "../examples/support/post_continuation_source.rs"]
#[allow(dead_code)]
mod source;
#[path = "../examples/support/static_posts.rs"]
mod static_posts;
use composition_support::{Engine, Event};
fn collect(mut e: Engine) -> Vec<chr_syntax::Answer> {
    let mut answers = vec![];
    for _ in 0..2_000_000 {
        match e.step() {
            Event::Answer(a) => answers.push(a),
            Event::Exhausted => return answers,
            Event::Progress => (),
        }
    }
    panic!("source cutoff")
}
#[test]
fn initialized_controls_execute_the_same_complete_source() {
    let mut cases = 0;
    for (fi, family) in source::FAMILIES.iter().enumerate() {
        for (ki, k) in [0, 1, 3].into_iter().enumerate() {
            for history in [false, true] {
                let rules = source::rules(family, k, history);
                let init = static_posts::Prepared::infer(&rules).unwrap();
                let counter = chr_compiled::resource_count::Program::infer(&rules).unwrap();
                let generic =
                    chr_compiled::PreparedRuleset::new(init.rules().to_vec(), None).unwrap();
                let inferred = chr_compiled::PreparedRuleset::new(init.rules().to_vec(), None)
                    .unwrap()
                    .specialize_inferred();
                let native = chr_compiled::PreparedRuleset::new(
                    init.rules().to_vec(),
                    Some(chr_compiled::access_initialized_continuation_bundled(
                        fi * 6 + ki * 2 + usize::from(history),
                    )),
                )
                .unwrap();
                let conditional =
                    chr_direct_conditional::engine::PreparedRuleset::new(init.rules().to_vec())
                        .unwrap();
                let mut held = vec![];
                for d in [0, 4, 16] {
                    for reverse in [false, true] {
                        for seed in 0..2 {
                            cases += 1;
                            let original = source::query(family, k, d, seed, reverse);
                            let expected = source::expected(family, k, seed, history);
                            runtime_support::same_raw(
                                runtime_support::run(&rules, &original, 2_000_000),
                                expected.clone(),
                            );
                            for counted in [false, true] {
                                let q = if counted {
                                    counter.lower(&original).unwrap()
                                } else {
                                    original.clone()
                                };
                                let q = init.initialize(&q);
                                runtime_support::same_raw(
                                    runtime_support::run(init.rules(), &q, 2_000_000),
                                    expected.clone(),
                                );
                                for (p, policy) in [
                                    (&generic, chr_compiled::Policy::Global),
                                    (&generic, chr_compiled::Policy::Active),
                                    (&inferred, chr_compiled::Policy::Global),
                                    (&native, chr_compiled::Policy::Global),
                                    (&native, chr_compiled::Policy::Active),
                                ] {
                                    for access in
                                        [chr_compiled::Access::Scan, chr_compiled::Access::Indexed]
                                    {
                                        let answers = collect(Engine::Compiled(
                                            p.start_search(q.clone(), policy, access).unwrap(),
                                        ));
                                        runtime_support::same_raw(
                                            answers.clone(),
                                            expected.clone(),
                                        );
                                        held.push((answers, expected.clone()));
                                    }
                                }
                                let answers =
                                    collect(Engine::Conditional(conditional.start(q).unwrap()));
                                runtime_support::same_raw(answers.clone(), expected.clone());
                                held.push((answers, expected.clone()));
                            }
                        }
                    }
                }
                drop(conditional);
                drop(native);
                drop(inferred);
                drop(generic);
                drop(init);
                for (answers, expected) in held {
                    runtime_support::same_raw(answers, expected);
                }
            }
        }
    }
    assert_eq!(cases, 216);
}
