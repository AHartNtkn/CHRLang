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

fn declaration() -> chr_compiled::resource_contract::Declaration {
    use chr_compiled::resource_contract::{Declaration, Predicate};
    Declaration {
        resource: Predicate {
            name: "fuel".into(),
            arity: 0,
        },
        access_rules: vec![1],
        entry: Predicate {
            name: "start".into(),
            arity: 3,
        },
        ground_argument: 0,
    }
}

#[test]
fn checked_properties_allow_queries_that_counting_cannot_lower() {
    use chr_compiled::resource_contract::CheckedSource;
    let rules = source::rules("common", 0);
    let checked = CheckedSource::check(&rules, declaration()).unwrap();
    for d in [1, 4] {
        let mut q = source::query("common", 0, d, 0);
        let index = q.constraints.iter().position(|c| c.name == "fuel").unwrap();
        q.constraints.remove(index);
        checked.check_query(&q).unwrap();
        assert!(Program::infer(checked.rules()).unwrap().lower(&q).is_err());
        check(
            checked.rules(),
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
    let mut q = source::query("common", 0, 1, 0);
    checked.check_query(&q).unwrap();
    q.constraints
        .iter_mut()
        .find(|c| c.name == "start")
        .unwrap()
        .args[0] = t("s", [v(900)]);
    assert!(checked.check_query(&q).is_err());
    let mut multiple = source::query("common", 0, 1, 0);
    let entry = multiple
        .constraints
        .iter()
        .find(|c| c.name == "start")
        .unwrap()
        .clone();
    multiple.constraints.push(entry);
    checked.check_query(&multiple).unwrap();
    assert!(
        Program::infer(checked.rules())
            .unwrap()
            .lower(&multiple)
            .is_err()
    );
    multiple.constraints.last_mut().unwrap().args[0] = v(901);
    assert!(checked.check_query(&multiple).is_err());
    multiple.constraints.retain(|c| c.name != "start");
    checked.check_query(&multiple).unwrap();
    assert!(
        Program::infer(checked.rules())
            .unwrap()
            .lower(&multiple)
            .is_err()
    );
    // A ground value outside the traversal grammar still satisfies groundness.
    q.constraints
        .iter_mut()
        .find(|c| c.name == "start")
        .unwrap()
        .args[0] = atom("other");
    checked.check_query(&q).unwrap();
    assert!(Program::infer(checked.rules()).unwrap().lower(&q).is_err());
    check(
        checked.rules(),
        &q,
        vec![Answer {
            outputs: vec![("result".into(), v(100))],
            residual: vec![
                c("permit", []),
                c("mark", []),
                c("query", [atom("q0")]),
                c("fuel", []),
                c("run", [atom("other"), v(100)]),
            ],
        }],
    );
}

#[test]
fn declarations_are_rechecked_against_the_complete_linked_rule_set() {
    use chr_compiled::resource_contract::CheckedSource;
    let rules = source::rules("common", 0);
    let checked = CheckedSource::check(&rules, declaration()).unwrap();
    let mut extension = rules.clone();
    extension.push(Rule::propagate(
        "observe",
        [c("fuel", [])],
        c("seen", []).into(),
    ));
    assert!(CheckedSource::check(&extension, declaration()).is_err());
    let mut extension = rules.clone();
    extension.push(Rule::simplify(
        "produce",
        [c("make", [])],
        chr_syntax::or(Goal::True, and([Goal::True, c("fuel", []).into()])),
    ));
    assert!(CheckedSource::check(&extension, declaration()).is_err());
    // Merely mentioning the same name at another arity is not resource access.
    let mut extension = rules.clone();
    extension.push(Rule::simplify("other", [c("fuel", [v(0)])], Goal::True));
    CheckedSource::check(&extension, declaration()).unwrap();
    // The previous certificate continues to reference exactly its original rules.
    let q = source::query("common", 0, 4, 0);
    checked.check_query(&q).unwrap();
    check(checked.rules(), &q, source::expected("common", 0, 4, 0));
    for bad in 0..5 {
        let mut d = declaration();
        match bad {
            0 => d.access_rules = vec![999],
            1 => d.access_rules = vec![],
            2 => d.resource.name = "missing".into(),
            3 => d.entry.name = "missing".into(),
            _ => d.ground_argument = 3,
        }
        assert!(CheckedSource::check(&rules, d).is_err());
    }
}

#[test]
fn ground_entry_does_not_restrict_result_unknowns_or_branch_resource_outcomes() {
    use chr_compiled::resource_contract::CheckedSource;
    let rules = source::rules("independent", 2);
    let checked = CheckedSource::check(&rules, declaration()).unwrap();
    let mut q = source::query("independent", 2, 4, 0);
    let mut removed = 0;
    q.constraints.retain(|c| {
        if c.name == "fuel" && removed < 2 {
            removed += 1;
            false
        } else {
            true
        }
    });
    checked.check_query(&q).unwrap();
    let lowered = Program::infer(checked.rules()).unwrap().lower(&q).unwrap();
    let mut expected = vec![];
    for bits in 0..4usize {
        let mut residual = vec![c("permit", []), c("mark", []), c("query", [atom("q0")])];
        let result = if bits == 0 {
            residual.push(c("fresh", [v(10000), v(10000)]));
            t("done", [v(10000)])
        } else {
            let mut depth = atom("z");
            for _ in 0..bits.count_ones() {
                depth = t("s", [depth]);
            }
            residual.push(c("run", [depth, v(100)]));
            v(100)
        };
        expected.push(Answer {
            outputs: vec![
                ("result".into(), result),
                ("x0".into(), atom(if bits & 1 == 0 { "a" } else { "b" })),
                ("x1".into(), atom(if bits & 2 == 0 { "a" } else { "b" })),
            ],
            residual,
        });
    }
    check(checked.rules(), &q, expected.clone());
    check(checked.rules(), &lowered, expected);
}

#[test]
fn optional_and_required_admission_enforce_the_same_declared_boundary() {
    use chr_compiled::resource_contract::{Admission, PreparedContract};
    let rules = source::rules("common", 0);
    assert!(PreparedContract::new(rules.clone(), None, Admission::Required, false).is_err());
    let unrestricted =
        PreparedContract::new(rules.clone(), None, Admission::Optional, false).unwrap();
    let mut bad = declaration();
    bad.access_rules = vec![];
    for admission in [Admission::Optional, Admission::Required] {
        assert!(PreparedContract::new(rules.clone(), Some(bad.clone()), admission, false).is_err());
        let prepared =
            PreparedContract::new(rules.clone(), Some(declaration()), admission, false).unwrap();
        for d in [0, 1, 4] {
            let q = source::query("common", 0, d, 0);
            let mut engine = prepared.start(q, Policy::Global, Access::Scan).unwrap();
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
            assert!(exhausted);
            scalar::same_raw(answers, source::expected("common", 0, d, 0));
        }
        let mut q = source::query("common", 0, 1, 0);
        q.constraints
            .iter_mut()
            .find(|c| c.name == "start")
            .unwrap()
            .args[0] = v(900);
        assert!(
            prepared
                .start(q.clone(), Policy::Global, Access::Scan)
                .is_err()
        );
        let mut engine = unrestricted.start(q, Policy::Global, Access::Scan).unwrap();
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
        assert!(exhausted);
        scalar::same_raw(
            answers,
            vec![Answer {
                outputs: vec![("result".into(), v(100))],
                residual: vec![
                    c("permit", []),
                    c("mark", []),
                    c("query", [atom("q0")]),
                    c("fuel", []),
                    c("run", [v(900), v(100)]),
                ],
            }],
        );
    }
}

#[test]
fn counted_contract_checks_original_input_and_reuses_its_own_source() {
    use chr_compiled::resource_contract::{Admission, PreparedContract};
    for family in ["common", "independent"] {
        for declared in [false, true] {
            let rules = source::rules(family, 2);
            let p =
                PreparedContract::new(rules, declared.then(declaration), Admission::Optional, true)
                    .unwrap();
            for depth in [0, 1, 4] {
                let mut search = p
                    .start(
                        source::query(family, 2, depth, 0),
                        Policy::Global,
                        Access::Scan,
                    )
                    .unwrap();
                let mut answers = vec![];
                let mut exhausted = false;
                for _ in 0..200_000 {
                    match search.tick() {
                        SearchEvent::Complete(mut b) => answers.push(b.engine.observe().unwrap()),
                        SearchEvent::Exhausted => {
                            exhausted = true;
                            break;
                        }
                        _ => (),
                    }
                }
                assert!(exhausted);
                scalar::same_raw(answers, source::expected(family, 2, depth, 0));
            }
            let mut unknown = source::query(family, 2, 4, 0);
            unknown
                .constraints
                .iter_mut()
                .find(|c| c.name == "start")
                .unwrap()
                .args[0] = v(900);
            assert_eq!(
                p.start(unknown, Policy::Global, Access::Scan).is_err(),
                declared
            );
        }
    }
}
