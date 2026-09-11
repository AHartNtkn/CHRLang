#[allow(dead_code)]
mod composition_support;
#[allow(dead_code)]
mod runtime_support;
#[allow(dead_code)]
#[path = "../examples/support/post_choice_source.rs"]
mod source;
use chr_syntax::{Answer, Constraint, Goal, Query, Rule, Term, Var};
fn term(t: &mut Term) {
    match t {
        Term::Var(v) => v.0 += 20000,
        Term::App(n, xs) => {
            *n = format!("renamed_{n}");
            for x in xs {
                term(x)
            }
        }
    }
}
fn constraint(c: &mut Constraint) {
    c.name = format!("renamed_{}", c.name);
    for x in &mut c.args {
        term(x)
    }
}
fn goal(g: &mut Goal) {
    match g {
        Goal::Constraint(c) => constraint(c),
        Goal::Unify(a, b) => {
            term(a);
            term(b)
        }
        Goal::And(gs) => {
            for g in gs {
                goal(g)
            }
        }
        Goal::Or(a, b) => {
            goal(a);
            goal(b)
        }
        _ => (),
    }
}
fn rename(r: &mut [Rule], q: &mut Query, answers: &mut [Answer]) {
    for r in r {
        r.name = format!("label_{}", r.name);
        for c in r.kept.iter_mut().chain(&mut r.removed) {
            constraint(c)
        }
        goal(&mut r.body);
    }
    for c in &mut q.constraints {
        constraint(c)
    }
    for (_, v) in &mut q.outputs {
        *v = Var(v.0 + 20000)
    }
    for a in answers {
        for (_, t) in &mut a.outputs {
            term(t)
        }
        for c in &mut a.residual {
            constraint(c)
        }
    }
}
fn run(mut e: composition_support::Engine) -> Vec<Answer> {
    let mut answers = vec![];
    for _ in 0..500000 {
        match e.step() {
            composition_support::Event::Answer(a) => answers.push(a),
            composition_support::Event::Exhausted => return answers,
            _ => (),
        }
    }
    panic!("source contraction cutoff")
}
#[test]
fn source_derived_counting_preserves_choices_resources_and_freshness() {
    for family in source::FAMILIES {
        for k in [0, 1, 3] {
            for d in [0, 1, 4, 16] {
                for seed in 0..2 {
                    for renamed in [false, true] {
                        let mut r = source::rules(family, k);
                        let mut q = source::query(family, k, d, seed);
                        let mut expected = source::expected(family, k, d, seed);
                        if renamed {
                            rename(&mut r, &mut q, &mut expected);
                        }
                        let program = chr_compiled::resource_count::Program::infer(&r).unwrap();
                        let shorter = program.lower(&q).unwrap();
                        assert_eq!(q.constraints.len() - shorter.constraints.len(), d);
                        runtime_support::same_raw(
                            runtime_support::run(&r, &q, 500000),
                            expected.clone(),
                        );
                        runtime_support::same_raw(
                            runtime_support::run(&r, &shorter, 500000),
                            expected.clone(),
                        );
                        let compiled = chr_compiled::PreparedRuleset::new(r.clone(), None).unwrap();
                        let special = chr_compiled::PreparedRuleset::new(r.clone(), None)
                            .unwrap()
                            .specialize_inferred();
                        let context =
                            chr_relational::contextual_execute::Prepared::new(&r).unwrap();
                        let conditional =
                            chr_direct_conditional::engine::PreparedRuleset::new(r).unwrap();
                        for query in [&q, &shorter] {
                            let engines = [
                                composition_support::Engine::Compiled(
                                    compiled
                                        .start_search(
                                            query.clone(),
                                            chr_compiled::Policy::Global,
                                            chr_compiled::Access::Scan,
                                        )
                                        .unwrap(),
                                ),
                                composition_support::Engine::Compiled(
                                    compiled
                                        .start_search(
                                            query.clone(),
                                            chr_compiled::Policy::Global,
                                            chr_compiled::Access::Indexed,
                                        )
                                        .unwrap(),
                                ),
                                composition_support::Engine::Compiled(
                                    special
                                        .start_search(
                                            query.clone(),
                                            chr_compiled::Policy::Global,
                                            chr_compiled::Access::Scan,
                                        )
                                        .unwrap(),
                                ),
                                composition_support::Engine::Contextual(context.start(query)),
                                composition_support::Engine::Contextual(
                                    context.start_demand(query),
                                ),
                                composition_support::Engine::Contextual(
                                    context.start_persistent_equality(query, true),
                                ),
                                composition_support::Engine::Conditional(
                                    conditional.start(query.clone()).unwrap(),
                                ),
                            ];
                            for e in engines {
                                runtime_support::same_raw(run(e), expected.clone());
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn elimination_removes_common_steps_and_preserves_branch_suffixes() {
    use chr_direct_conditional::engine::{Event, PreparedRuleset, Trace};
    for (family, remaining) in [("common", 0), ("independent", 12), ("early", 0)] {
        let rules = source::rules(family, 3);
        let q = source::query(family, 3, 16, 0);
        let shorter = chr_compiled::resource_count::Program::infer(&rules)
            .unwrap()
            .lower(&q)
            .unwrap();
        let mut e = PreparedRuleset::new(rules).unwrap().start(shorter).unwrap();
        e.enable_trace();
        let mut answers = vec![];
        let mut exhausted = false;
        for _ in 0..500000 {
            match e.tick() {
                Event::Answer(a) => answers.push(a),
                Event::Exhausted => {
                    exhausted = true;
                    break;
                }
                _ => (),
            }
        }
        assert!(exhausted);
        runtime_support::same_raw(answers, source::expected(family, 3, 16, 0));
        assert_eq!(
            e.trace()
                .iter()
                .filter(|t| matches!(t, Trace::Application { rule: 1, .. }))
                .count(),
            remaining
        );
    }
}

#[test]
fn branch_specific_shortage_has_independently_constructed_residuals() {
    use chr_syntax::{atom, c, t, v};
    let rules = source::rules("independent", 3);
    let mut q = source::query("independent", 3, 4, 0);
    let mut removed = 0;
    q.constraints.retain(|c| {
        if c.name == "fuel" && removed < 3 {
            removed += 1;
            false
        } else {
            true
        }
    });
    let shorter = chr_compiled::resource_count::Program::infer(&rules)
        .unwrap()
        .lower(&q)
        .unwrap();
    let mut expected = source::expected("independent", 3, 4, 0);
    for (bits, a) in expected.iter_mut().enumerate() {
        a.residual.retain(|c| c.name != "fuel");
        if bits != 0 {
            a.outputs[0].1 = v(99999);
            a.residual.retain(|c| c.name != "fresh");
            let mut depth = atom("z");
            for _ in 0..bits.count_ones() {
                depth = t("s", [depth]);
            }
            a.residual.push(c("run", [depth, v(99999)]));
        }
    }
    for q in [&q, &shorter] {
        runtime_support::same_raw(runtime_support::run(&rules, q, 500000), expected.clone());
        for mode in [0, 1, 5] {
            runtime_support::same_raw(
                run(composition_support::Engine::new(mode, &rules, q)),
                expected.clone(),
            );
        }
        let p = chr_relational::contextual_execute::Prepared::new(&rules).unwrap();
        runtime_support::same_raw(
            run(composition_support::Engine::Contextual(p.start_demand(q))),
            expected.clone(),
        );
    }
}
