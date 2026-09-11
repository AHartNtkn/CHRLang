#[allow(dead_code)]
mod composition_support;
#[allow(dead_code)]
mod runtime_support;
#[path = "../examples/support/post_choice_source.rs"]
mod source;
use composition_support::Engine;
#[test]
fn common_continuation_has_complete_answers_and_measured_shared_work() {
    for family in source::FAMILIES {
        for k in [0, 1, 3] {
            for d in [0, 4, 16] {
                let rules = source::rules(family, k);
                let p = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
                let specialized = chr_compiled::PreparedRuleset::new(rules.clone(), None)
                    .unwrap()
                    .specialize_inferred();
                let context = chr_relational::contextual_execute::Prepared::new(&rules).unwrap();
                let conditional =
                    chr_direct_conditional::engine::PreparedRuleset::new(rules.clone()).unwrap();
                for seed in 0..2 {
                    let q = source::query(family, k, d, seed);
                    let expected = source::expected(family, k, d, seed);
                    let scalar = runtime_support::run_traced(&rules, &q, 500000);
                    assert_eq!(
                        scalar
                            .iter()
                            .map(|(_, trace)| trace.iter().filter(|&&r| r == 1).count())
                            .sum::<usize>(),
                        source::explicit_steps(family, k, d)
                    );
                    runtime_support::same_raw(
                        scalar.into_iter().map(|(a, _)| a).collect(),
                        expected.clone(),
                    );
                    let engines = [
                        Engine::Compiled(
                            p.start_search(
                                q.clone(),
                                chr_compiled::Policy::Global,
                                chr_compiled::Access::Scan,
                            )
                            .unwrap(),
                        ),
                        Engine::Compiled(
                            p.start_search(
                                q.clone(),
                                chr_compiled::Policy::Global,
                                chr_compiled::Access::Indexed,
                            )
                            .unwrap(),
                        ),
                        Engine::Compiled(
                            specialized
                                .start_search(
                                    q.clone(),
                                    chr_compiled::Policy::Global,
                                    chr_compiled::Access::Scan,
                                )
                                .unwrap(),
                        ),
                        Engine::Contextual(context.start(&q)),
                        Engine::Contextual(context.start_demand(&q)),
                        Engine::Contextual(context.start_persistent_equality(&q, true)),
                    ];
                    for mut e in engines {
                        let mut answers = vec![];
                        let mut exhausted = false;
                        for _ in 0..500000 {
                            match e.step() {
                                composition_support::Event::Answer(a) => answers.push(a),
                                composition_support::Event::Exhausted => {
                                    exhausted = true;
                                    break;
                                }
                                _ => (),
                            }
                        }
                        assert!(exhausted);
                        runtime_support::same_raw(answers, expected.clone());
                    }
                    let mut e = conditional.start(q).unwrap();
                    e.enable_trace();
                    let mut answers = vec![];
                    let mut exhausted = false;
                    for _ in 0..500000 {
                        match e.tick() {
                            chr_direct_conditional::engine::Event::Answer(a) => answers.push(a),
                            chr_direct_conditional::engine::Event::Exhausted => {
                                exhausted = true;
                                break;
                            }
                            _ => (),
                        }
                    }
                    assert!(exhausted);
                    runtime_support::same_raw(answers, expected);
                    let steps = e
                        .trace()
                        .iter()
                        .filter(|event| {
                            matches!(
                                event,
                                chr_direct_conditional::engine::Trace::Application { rule: 1, .. }
                            )
                        })
                        .count();
                    println!(
                        "family={family} choices={k} depth={d} seed={seed} explicit={} conditional={steps}",
                        source::explicit_steps(family, k, d)
                    );
                    if family == "common" {
                        assert_eq!(steps, d, "common work must actually be shared");
                    }
                }
            }
        }
    }
}
