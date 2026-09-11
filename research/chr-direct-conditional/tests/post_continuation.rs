#[allow(dead_code)]
mod composition_support;
#[allow(dead_code)]
mod runtime_support;
#[path = "../examples/support/post_continuation_source.rs"]
mod source;
#[path = "../examples/support/static_posts.rs"]
mod static_posts;
#[path = "../examples/support/value_choices.rs"]
mod value_choices;
use chr_direct_choice::demand::{Prepared, Reuse};
use chr_syntax::Answer;
use composition_support::{Engine, Event};
const BOUND: usize = 2_000_000;
fn finish(mut engine: Engine) -> Vec<Answer> {
    let mut answers = vec![];
    for _ in 0..BOUND {
        match engine.step() {
            Event::Answer(a) => answers.push(a),
            Event::Exhausted => return answers,
            Event::Progress => (),
        }
    }
    panic!("complete engine cutoff");
}
fn demand(mut engine: chr_direct_choice::demand::Run) -> Vec<Answer> {
    let mut answers = vec![];
    for _ in 0..BOUND {
        match engine.tick() {
            chr_direct_choice::demand::Event::Answer(a) => answers.push(a),
            chr_direct_choice::demand::Event::Exhausted => {
                #[cfg(feature = "work-diagnostics")]
                {
                    let work = engine.work();
                    assert!(
                        work.resource_posts > 0
                            && work.output_binders > 0
                            && work.resource_candidates > 0,
                        "nonground post mechanism must execute"
                    );
                }
                return answers;
            }
            _ => (),
        }
    }
    panic!("demand cutoff");
}
#[test]
fn mixed_posts_preserve_complete_continuations_and_record_capability() {
    let mut configs = 0;
    for family in source::FAMILIES {
        for k in [0, 1, 3] {
            for d in [0, 4, 16] {
                for history in [false, true] {
                    let rules = source::rules(family, k, history);
                    let generic = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
                    let inferred = chr_compiled::PreparedRuleset::new(rules.clone(), None)
                        .unwrap()
                        .specialize_inferred();
                    let conditional =
                        chr_direct_conditional::engine::PreparedRuleset::new(rules.clone())
                            .unwrap();
                    let initialized = static_posts::Prepared::infer(&rules).unwrap();
                    let demand_rules = value_choices::lower(initialized.rules());
                    let demands = (0..3)
                        .map(|mode| {
                            let p = Prepared::with_reuse(demand_rules.clone(), Reuse::StaticBirth)
                                .unwrap();
                            match mode {
                                0 => p,
                                1 => p.with_miss_reuse(),
                                _ => p.with_miss_reuse().with_derivation_templates(),
                            }
                        })
                        .collect::<Vec<_>>();
                    let lowered = chr_compiled::resource_count::Program::infer(&rules)
                        .expect("mixed source must exercise the counting control");
                    let mut held = vec![];
                    for reverse in [false, true] {
                        for seed in 0..2 {
                            configs += 1;
                            println!("CASE,{family},{k},{d},{history},{reverse},{seed}");
                            let q = source::query(family, k, d, seed, reverse);
                            let expected = source::expected(family, k, seed, history);
                            let traced = runtime_support::run_traced(&rules, &q, BOUND);
                            let steps = traced
                                .iter()
                                .map(|(_, t)| t.iter().filter(|&&r| r == 3).count())
                                .sum::<usize>();
                            assert_eq!(steps, source::explicit_steps(family, k, d));
                            runtime_support::same_raw(
                                traced.into_iter().map(|(a, _)| a).collect(),
                                expected.clone(),
                            );
                            for (p, policy) in [
                                (&generic, chr_compiled::Policy::Global),
                                (&inferred, chr_compiled::Policy::Global),
                                (&generic, chr_compiled::Policy::Active),
                            ] {
                                for access in
                                    [chr_compiled::Access::Scan, chr_compiled::Access::Indexed]
                                {
                                    for n in [1, 4] {
                                        let mut partial = Engine::Compiled(
                                            p.start_search(q.clone(), policy, access).unwrap(),
                                        );
                                        for _ in 0..n {
                                            if let Event::Answer(a) = partial.step() {
                                                assert!(expected.iter().any(|e| {
                                                    chr_observe::equivalent(
                                                        e,
                                                        &a,
                                                        &mut Default::default(),
                                                    )
                                                }));
                                            }
                                        }
                                    }
                                    let answers = finish(Engine::Compiled(
                                        p.start_search(q.clone(), policy, access).unwrap(),
                                    ));
                                    runtime_support::same_raw(answers.clone(), expected.clone());
                                    held.push((answers, expected.clone()));
                                }
                            }
                            let mut e = conditional.start(q.clone()).unwrap();
                            e.enable_trace();
                            let mut answers = vec![];
                            let mut done = false;
                            for _ in 0..BOUND {
                                match e.tick() {
                                    chr_direct_conditional::engine::Event::Answer(a) => {
                                        answers.push(a)
                                    }
                                    chr_direct_conditional::engine::Event::Exhausted => {
                                        done = true;
                                        break;
                                    }
                                    _ => (),
                                }
                            }
                            assert!(done, "conditional cutoff {family}/{k}/{d}");
                            let actual_steps = e
                                .trace()
                                .iter()
                                .filter(|e| {
                                    matches!(
                                        e,
                                        chr_direct_conditional::engine::Trace::Application {
                                            rule: 3,
                                            ..
                                        }
                                    )
                                })
                                .count();
                            println!(
                                "WORK,{family},{k},{d},{history},{reverse},{seed},explicit={steps},conditional={actual_steps}"
                            );
                            if family == "common" {
                                assert_eq!(actual_steps, d);
                            }
                            runtime_support::same_raw(answers.clone(), expected.clone());
                            held.push((answers, expected.clone()));
                            for n in [1, 4] {
                                let mut p = conditional.start(q.clone()).unwrap();
                                for _ in 0..n {
                                    if let chr_direct_conditional::engine::Event::Answer(a) =
                                        p.tick()
                                    {
                                        assert!(expected.iter().any(|e| chr_observe::equivalent(
                                            e,
                                            &a,
                                            &mut Default::default()
                                        )));
                                    }
                                }
                            }
                            let demand_query = initialized.initialize(&q);
                            runtime_support::same_raw(
                                runtime_support::run(&demand_rules, &demand_query, BOUND),
                                expected.clone(),
                            );
                            for p in &demands {
                                for n in [1, 4] {
                                    let mut partial = p.start(demand_query.clone()).unwrap();
                                    for _ in 0..n {
                                        if let chr_direct_choice::demand::Event::Answer(a) =
                                            partial.tick()
                                        {
                                            assert!(expected.iter().any(|e| {
                                                chr_observe::equivalent(
                                                    e,
                                                    &a,
                                                    &mut Default::default(),
                                                )
                                            }));
                                        }
                                    }
                                }
                                let answers = demand(p.start(demand_query.clone()).unwrap());
                                runtime_support::same_raw(answers.clone(), expected.clone());
                                held.push((answers, expected.clone()));
                            }
                            {
                                let shortened = lowered.lower(&q).unwrap();
                                let scalar = runtime_support::run_traced(&rules, &shortened, BOUND);
                                let remaining = scalar
                                    .iter()
                                    .map(|(_, trace)| trace.iter().filter(|&&r| r == 3).count())
                                    .sum::<usize>();
                                assert_eq!(remaining, steps - expected.len() * d);
                                runtime_support::same_raw(
                                    scalar.into_iter().map(|(a, _)| a).collect(),
                                    expected.clone(),
                                );
                                for (p, policy) in [
                                    (&generic, chr_compiled::Policy::Global),
                                    (&inferred, chr_compiled::Policy::Global),
                                    (&generic, chr_compiled::Policy::Active),
                                ] {
                                    for access in
                                        [chr_compiled::Access::Scan, chr_compiled::Access::Indexed]
                                    {
                                        let answers = finish(Engine::Compiled(
                                            p.start_search(shortened.clone(), policy, access)
                                                .unwrap(),
                                        ));
                                        runtime_support::same_raw(
                                            answers.clone(),
                                            expected.clone(),
                                        );
                                        held.push((answers, expected.clone()));
                                    }
                                }
                                let mut e = conditional.start(shortened.clone()).unwrap();
                                e.enable_trace();
                                let mut answers = vec![];
                                let mut done = false;
                                for _ in 0..BOUND {
                                    match e.tick() {
                                        chr_direct_conditional::engine::Event::Answer(a) => {
                                            answers.push(a)
                                        }
                                        chr_direct_conditional::engine::Event::Exhausted => {
                                            done = true;
                                            break;
                                        }
                                        _ => (),
                                    }
                                }
                                assert!(done);
                                let counted = e
                                    .trace()
                                    .iter()
                                    .filter(|e| {
                                        matches!(
                                            e,
                                            chr_direct_conditional::engine::Trace::Application {
                                                rule: 3,
                                                ..
                                            }
                                        )
                                    })
                                    .count();
                                assert_eq!(counted, remaining);
                                println!(
                                    "COUNTED,{family},{k},{d},{history},{reverse},{seed},explicit={remaining},conditional={counted}"
                                );
                                runtime_support::same_raw(answers.clone(), expected.clone());
                                held.push((answers, expected.clone()));
                                runtime_support::same_raw(
                                    runtime_support::run(
                                        &demand_rules,
                                        &initialized.initialize(&shortened),
                                        BOUND,
                                    ),
                                    expected.clone(),
                                );
                                for p in &demands {
                                    let answers = demand(
                                        p.start(initialized.initialize(&shortened)).unwrap(),
                                    );
                                    runtime_support::same_raw(answers.clone(), expected.clone());
                                    held.push((answers, expected.clone()));
                                }
                            }
                        }
                    }
                    drop(demands);
                    drop(initialized);
                    drop(conditional);
                    drop(inferred);
                    drop(generic);
                    for (actual, expected) in held {
                        runtime_support::same_raw(actual, expected);
                    }
                }
            }
        }
    }
    assert_eq!(configs, 216);
}
