//! Executable semantic premises for the S07 language-property comparison.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/examples/support/post_choice_source.rs"]
mod source;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent, resource_count::Program};
use chr_syntax::{Answer, Goal, Query, Rule, and, atom, c, eq, t, v};

fn check(rules: &[Rule], query: &Query, expected: Vec<Answer>) {
    scalar::same_raw(scalar::run(rules, query, 200_000), expected.clone());
    for access in [Access::Scan, Access::Indexed] {
        let prepared = PreparedRuleset::new(rules.to_vec(), None).unwrap();
        let mut engine = prepared
            .start_search(query.clone(), Policy::Global, access)
            .unwrap();
        let mut answers = vec![];
        let mut exhausted = false;
        for _ in 0..200_000 {
            match engine.tick() {
                SearchEvent::Complete(mut b) => answers.push(b.engine.observe().unwrap()),
                SearchEvent::Exhausted => {
                    exhausted = true;
                    break;
                }
                _ => (),
            }
        }
        assert!(exhausted, "property witness cutoff");
        scalar::same_raw(answers, expected.clone());
    }
}

#[test]
fn a_silent_resource_reader_does_not_require_private_fuel() {
    for k in [0, 2] {
        for d in [0, 1, 4] {
            let base = source::rules("common", k);
            let q = source::query("common", k, d, 0);
            let shorter = Program::infer(&base).unwrap().lower(&q).unwrap();
            let mut rules = base;
            rules.insert(
                0,
                Rule::propagate("silent-reader", [c("fuel", [])], Goal::True),
            );
            assert!(Program::infer(&rules).is_err());
            let expected = source::expected("common", k, d, 0);
            check(&rules, &q, expected.clone());
            check(&rules, &shorter, expected);
        }
    }
}

#[test]
fn an_independent_active_entry_effect_can_commute_with_counting() {
    for k in [0, 2] {
        for d in [0, 1, 4] {
            let base = source::rules("common", k);
            let q = source::query("common", k, d, 0);
            let shorter = Program::infer(&base).unwrap().lower(&q).unwrap();
            let mut rules = base;
            rules[3].body = and([c("ping", []).into(), rules[3].body.clone()]);
            rules.insert(
                0,
                Rule::simplify("ping", [c("ping", [])], c("tagged", []).into()),
            );
            assert!(Program::infer(&rules).is_err());
            let mut expected = source::expected("common", k, d, 0);
            for a in &mut expected {
                a.residual.push(c("tagged", []));
            }
            check(&rules, &q, expected.clone());
            check(&rules, &shorter, expected);
        }
    }
}

#[test]
fn late_groundness_has_a_valid_ordinary_execution_and_a_specific_reformulation() {
    for k in [0, 2] {
        for d in [0, 1, 4] {
            let mut rules = source::rules("common", k);
            rules.push(Rule::simplify(
                "provide",
                [c("provide", [v(0), v(1)])],
                eq(v(0), v(1)),
            ));
            let prepared = Program::infer(&rules).unwrap();
            let ground = source::query("common", k, d, 0);
            let mut late = ground.clone();
            let entry = late
                .constraints
                .iter_mut()
                .find(|c| c.name == "start")
                .unwrap();
            let depth = std::mem::replace(&mut entry.args[0], v(900));
            // Unknown input alone is outside this query certificate.
            assert!(prepared.lower(&late).is_err());
            late.constraints.push(c("provide", [v(900), depth]));
            assert!(prepared.lower(&late).is_err());
            let expected = source::expected("common", k, d, 0);
            check(&rules, &late, expected.clone());
            // Moving this independent, total producer to query construction
            // preserves the full answer, but requires supplying its ground value.
            check(&rules, &ground, expected.clone());
            check(&rules, &prepared.lower(&ground).unwrap(), expected);
        }
    }
}

#[test]
fn observable_depth_cannot_be_changed_without_preserving_the_observation() {
    let base = source::rules("common", 0);
    let q = source::query("common", 0, 4, 0);
    let shorter = Program::infer(&base).unwrap().lower(&q).unwrap();
    let depth = q
        .constraints
        .iter()
        .find(|c| c.name == "start")
        .unwrap()
        .args[0]
        .clone();
    let mut rules = base;
    rules[3].body = and([c("depth", [v(0)]).into(), rules[3].body.clone()]);
    assert!(Program::infer(&rules).is_err());
    let mut before = source::expected("common", 0, 4, 0);
    before[0].residual.push(c("depth", [depth]));
    let mut after = source::expected("common", 0, 4, 0);
    after[0].residual.push(c("depth", [atom("z")]));
    assert_ne!(before, after);
    check(&rules, &q, before);
    check(&rules, &shorter, after);
}

#[test]
fn ground_private_input_can_still_suspend_when_resources_are_insufficient() {
    for d in [1, 4] {
        let rules = source::rules("common", 0);
        let mut q = source::query("common", 0, d, 0);
        let index = q.constraints.iter().position(|c| c.name == "fuel").unwrap();
        q.constraints.remove(index);
        assert!(Program::infer(&rules).unwrap().lower(&q).is_err());
        check(
            &rules,
            &q,
            vec![Answer {
                outputs: vec![("result".into(), v(100))],
                residual: vec![
                    c("permit", []),
                    c("mark", []),
                    c("query", [atom("q0")]),
                    c("run", [t("s", [atom("z")]), v(100)]),
                ],
            }],
        );
    }
}
