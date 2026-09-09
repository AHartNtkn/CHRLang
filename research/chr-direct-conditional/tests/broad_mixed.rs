#[allow(dead_code)]
mod composition_support;
mod runtime_support;
#[path = "../examples/support/broad_mixed_source.rs"]
mod source;
use composition_support::Engine;
#[test]
fn independently_constructed_complete_mixed_observations() {
    for family in source::FAMILIES {
        for n in [0, 1, 3] {
            let rules = source::rules(family);
            let compiled = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
            let specialized = chr_compiled::PreparedRuleset::new(rules.clone(), None)
                .unwrap()
                .specialize_inferred();
            let contextual = chr_relational::contextual_execute::Prepared::new(&rules).unwrap();
            let conditional =
                chr_direct_conditional::engine::PreparedRuleset::new(rules.clone()).unwrap();
            for seed in 0..2 {
                let q = source::query(family, n, seed);
                let expected = source::expected(family, n, seed);
                runtime_support::same_raw(
                    runtime_support::run(&rules, &q, 200000),
                    expected.clone(),
                );
                let engines = [
                    Engine::Compiled(
                        compiled
                            .start_search(
                                q.clone(),
                                chr_compiled::Policy::Global,
                                chr_compiled::Access::Scan,
                            )
                            .unwrap(),
                    ),
                    Engine::Compiled(
                        compiled
                            .start_search(
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
                    Engine::Contextual(contextual.start(&q)),
                    Engine::Contextual(contextual.start_demand(&q)),
                    Engine::Contextual(contextual.start_resumable(&q)),
                    Engine::Contextual(contextual.start_persistent_equality(&q, true)),
                    Engine::Conditional(conditional.start(q.clone()).unwrap()),
                ];
                for (mode, e) in engines.into_iter().enumerate() {
                    let answers = e.collect();
                    eprintln!("qualified {family} n={n} seed={seed} mode={mode}");
                    runtime_support::same_raw(answers, expected.clone());
                }
            }
        }
    }
}
