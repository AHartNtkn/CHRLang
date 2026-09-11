#[allow(dead_code)]
mod composition_support;
#[allow(dead_code)]
mod runtime_support;
#[path = "../examples/support/order_source.rs"]
mod source;
use composition_support::{Engine, Event};
#[test]
fn opposite_arrival_programs_have_the_same_unique_success_and_explicit_failure() {
    for family in ["oldest-first", "newest-first"] {
        for n in [0, 1, 4, 8] {
            for fail_tail in [false, true] {
                for reverse in [false, true] {
                    let schema = source::Schema {
                        family,
                        resource: true,
                        fail_tail,
                        work: 4,
                        payload: 8,
                    };
                    let rules = schema.rules();
                    let q = schema.query(n, reverse);
                    let expected = schema.expected_query(n, reverse);
                    runtime_support::same_raw(
                        runtime_support::run(&rules, &q, 2_000_000),
                        expected.clone(),
                    );
                    for mode in [0, 5, 7, 8] {
                        let mut e = if mode == 8 {
                            #[cfg(feature = "head-dispatch")]
                            {
                                Engine::Conditional(chr_direct_conditional::engine::PreparedRuleset::with_head_contract(rules.clone(), None, chr_direct_conditional::engine::HeadAdmission::Optional).unwrap().start(q.clone()).unwrap())
                            }
                            #[cfg(not(feature = "head-dispatch"))]
                            {
                                continue;
                            }
                        } else {
                            Engine::new(mode, &rules, &q)
                        };
                        let mut answers = vec![];
                        let mut done = false;
                        for _ in 0..2_000_000 {
                            match e.step() {
                                Event::Answer(a) => answers.push(a),
                                Event::Exhausted => {
                                    done = true;
                                    break;
                                }
                                Event::Progress => (),
                            }
                        }
                        assert!(done);
                        runtime_support::same_raw(answers, expected.clone());
                    }
                }
            }
        }
    }
}
