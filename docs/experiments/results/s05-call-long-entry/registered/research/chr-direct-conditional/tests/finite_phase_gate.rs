#[allow(dead_code)]
mod composition_support;
#[path = "../../chr-compiled/experiments/finite_phase.rs"]
mod finite_phase;
#[allow(dead_code)]
mod runtime_support;
#[allow(dead_code)]
#[path = "../examples/support/order_source.rs"]
mod source;
use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, and, atom, c, eq, or, v};
use finite_phase::{Limits, Prepared, Solution};
use std::collections::BTreeSet;

fn vars(t: &Term, out: &mut BTreeSet<Var>) {
    match t {
        Term::Var(x) => {
            out.insert(*x);
        }
        Term::App(_, xs) => {
            for x in xs {
                vars(x, out)
            }
        }
    }
}
fn resume(rules: &[Rule], solutions: Vec<Solution>) -> Vec<Answer> {
    let mut out = vec![];
    for s in solutions {
        let mut ids = BTreeSet::new();
        for (a, b) in &s.equations {
            vars(a, &mut ids);
            vars(b, &mut ids);
        }
        let args: Vec<_> = ids.into_iter().map(Term::Var).collect();
        let mut name = "$finite-resume".to_string();
        while rules
            .iter()
            .flat_map(|r| r.kept.iter().chain(&r.removed))
            .chain(&s.query.constraints)
            .any(|c| c.name == name)
        {
            name.push('_');
        }
        let entry = c(&name, args);
        let mut lowered = vec![Rule::simplify(
            "resume",
            [entry.clone()],
            and(s
                .equations
                .into_iter()
                .map(|(a, b)| eq(a, b))
                .collect::<Vec<_>>()),
        )];
        lowered.extend_from_slice(rules);
        let mut q = s.query;
        q.constraints.insert(0, entry);
        let a = runtime_support::run(&lowered, &q, 2_000_000);
        runtime_support::same_raw(
            composition_support::Engine::new(0, &lowered, &q).collect(),
            a.clone(),
        );
        assert!(s.multiplicity <= 4096);
        for _ in 0..s.multiplicity {
            out.extend(a.clone());
        }
    }
    out
}
#[test]
fn source_rules_drive_selective_solving_and_caller_resumption() {
    let s = source::Schema {
        family: "oldest-first",
        work: 2,
        payload: 2,
        resource: true,
        fail_tail: false,
    };
    let rules = s.rules();
    let p = Prepared::new(&rules, 6).unwrap();
    for n in 0..=6 {
        let q = s.query(n, false);
        let solved = p.solve(&q, Limits::default()).unwrap();
        runtime_support::same_raw(
            resume(&rules, solved.solutions),
            runtime_support::run(&rules, &q, 2_000_000),
        );
    }
    let solved = p
        .solve(
            &s.query(64, false),
            Limits {
                steps: 20_000,
                ..Limits::default()
            },
        )
        .unwrap();
    assert_eq!(solved.solutions.len(), 1);
    runtime_support::same_raw(
        resume(&rules, solved.solutions),
        s.expected_query(64, false),
    );
    println!(
        "selective_64_steps={} partitions={}",
        solved.steps, solved.partitions
    );
}

fn compare(rules: &[Rule], prefix: usize, q: &Query) {
    let original = runtime_support::run(rules, q, 2_000_000);
    runtime_support::same_raw(
        composition_support::Engine::new(0, rules, q).collect(),
        original.clone(),
    );
    let solved = Prepared::new(rules, prefix)
        .unwrap()
        .solve(q, Limits::default())
        .unwrap();
    runtime_support::same_raw(resume(rules, solved.solutions), original);
}
#[test]
fn entire_arrival_matrix_has_independent_complete_answers() {
    let mut count = 0;
    for family in ["oldest-first", "newest-first"] {
        for work in [0, 2] {
            for resource in [false, true] {
                for fail_tail in [false, true] {
                    let s = source::Schema {
                        family,
                        work,
                        resource,
                        fail_tail,
                        payload: 2,
                    };
                    let rules = s.rules();
                    let p = Prepared::new(&rules, 6).unwrap();
                    for n in 0..=6 {
                        for tag in [false, true] {
                            let q = s.query(n, tag);
                            let original = runtime_support::run(&rules, &q, 2_000_000);
                            runtime_support::same_raw(original.clone(), s.expected_query(n, tag));
                            runtime_support::same_raw(
                                composition_support::Engine::new(0, &rules, &q).collect(),
                                original.clone(),
                            );
                            runtime_support::same_raw(
                                resume(&rules, p.solve(&q, Limits::default()).unwrap().solutions),
                                original,
                            );
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(count, 224);
    println!("arrival_configurations={count}");
}
fn producer(name: &str, values: &[&str]) -> Rule {
    let mut gs = values.iter().map(|s| eq(v(0), atom(s)));
    let body = gs
        .next()
        .map(|first| gs.fold(first, or))
        .unwrap_or(Goal::Fail);
    Rule::simplify(name, [c(name, [v(0)])], body)
}
#[test]
fn source_derived_relations_handle_names_values_aliases_and_priority() {
    let values = ["oak", "elm", "ash"];
    let mut count = 0;
    for duplicate in [false, true] {
        for alias in [false, true] {
            for reverse in [false, true] {
                for mask in 0..8 {
                    let domain = if duplicate {
                        vec!["oak", "elm", "oak", "ash"]
                    } else {
                        values.to_vec()
                    };
                    let mut rules = vec![
                        producer("choose-tree", &domain),
                        producer("choose-other", &["elm", "oak", "ash"]),
                    ];
                    let mut checks = vec![];
                    for (i, a) in values.iter().enumerate() {
                        for (j, b) in values.iter().enumerate() {
                            checks.push(Rule::simplify(
                                &format!("relation-{i}-{j}"),
                                [c(
                                    "relation",
                                    [chr_syntax::t("pair", [atom(a), atom(b)]), v(7)],
                                )],
                                if (mask >> ((i + j) % 3)) & 1 == 1 {
                                    eq(v(7), chr_syntax::t("chosen", [atom(a), atom(b)]))
                                } else {
                                    Goal::Fail
                                },
                            ));
                        }
                    }
                    if reverse {
                        checks.reverse();
                    }
                    rules.extend(checks);
                    // A lower-priority general rule must not contribute additional derivations.
                    rules.push(Rule::simplify(
                        "fallback-case",
                        [c("relation", [v(3), v(4)])],
                        eq(v(4), atom("unreachable")),
                    ));
                    let y = if alias { 100 } else { 101 };
                    let q = Query {
                        constraints: vec![
                            c("choose-tree", [v(100)]),
                            c("choose-other", [v(y)]),
                            c("relation", [chr_syntax::t("pair", [v(100), v(y)]), v(102)]),
                            c("visible", [v(100), v(y)]),
                        ],
                        outputs: vec![
                            ("out".into(), Var(102)),
                            ("again".into(), Var(102)),
                            ("x".into(), Var(100)),
                        ],
                    };
                    compare(&rules, rules.len(), &q);
                    count += 1;
                }
            }
        }
    }
    assert_eq!(count, 64);
    println!("relation_configurations={count}");
}
#[test]
fn repeated_choice_factors_preserve_raw_multiplicity() {
    for copies in 1..=5 {
        let rules = vec![
            producer("sample", &["a", "b", "a"]),
            Rule::simplify("a", [c("accept", [atom("a")])], Goal::True),
            Rule::simplify("b", [c("accept", [atom("b")])], Goal::Fail),
        ];
        let mut cs = vec![c("sample", [v(100)]); copies];
        cs.push(c("accept", [v(100)]));
        let q = Query {
            constraints: cs,
            outputs: vec![("x".into(), Var(100))],
        };
        let r = Prepared::new(&rules, 3)
            .unwrap()
            .solve(&q, Limits::default())
            .unwrap();
        assert_eq!(r.solutions.len(), 1);
        assert_eq!(r.solutions[0].multiplicity, 1 << copies);
        compare(&rules, 3, &q);
    }
}
#[test]
fn equality_can_merge_domains_and_constructor_outputs_without_capture() {
    for same in [false, true] {
        for reverse in [false, true] {
            let mut rules = vec![
                producer("left", &["a", "a", "b"]),
                producer("right", if same { &["a", "b"] } else { &["b", "c"] }),
            ];
            // Query ids intentionally overlap rule-local ids. Output contains an alias.
            rules.push(Rule::simplify(
                "join",
                [c("join", [v(0), v(1), v(2)])],
                and(vec![
                    eq(v(0), v(1)),
                    eq(v(2), chr_syntax::t("box", [v(0), v(1)])),
                ]),
            ));
            let mut cs = vec![
                c("left", [v(1)]),
                c("right", [v(0)]),
                c("join", [v(0), v(1), v(2)]),
                c("keep", [v(0), v(1), v(2)]),
            ];
            if reverse {
                cs.reverse();
            }
            compare(
                &rules,
                3,
                &Query {
                    constraints: cs,
                    outputs: vec![
                        ("out".into(), Var(2)),
                        ("x".into(), Var(0)),
                        ("y".into(), Var(1)),
                    ],
                },
            );
        }
    }
    let rules = vec![
        producer("choice", &["a", "b"]),
        Rule::simplify(
            "cycle",
            [c("cycle", [v(0)])],
            eq(v(0), chr_syntax::t("s", [v(0)])),
        ),
    ];
    compare(
        &rules,
        2,
        &Query {
            constraints: vec![c("choice", [v(100)]), c("cycle", [v(101)])],
            outputs: vec![],
        },
    );
}
#[test]
fn complement_partition_preserves_later_cases_and_unknown_outputs() {
    let rules = vec![
        producer("choice", &["a", "b", "c"]),
        Rule::simplify(
            "first",
            [c("check", [atom("a"), atom("b"), v(0)])],
            eq(v(0), atom("first")),
        ),
        Rule::simplify(
            "rest",
            [c("check", [v(0), v(1), v(2)])],
            eq(v(2), chr_syntax::t("rest", [v(0), v(1)])),
        ),
    ];
    for alias in [false, true] {
        let y = if alias { 100 } else { 101 };
        let q = Query {
            constraints: vec![
                c("choice", [v(100)]),
                c("choice", [v(y)]),
                c("check", [v(100), v(y), v(102)]),
            ],
            outputs: vec![("out".into(), Var(102)), ("unused".into(), Var(999))],
        };
        compare(&rules, 3, &q);
    }
}
#[test]
fn caller_can_observe_bindings_and_start_another_phase() {
    let mut rules = vec![producer("choice", &["a", "b"])];
    rules.push(Rule::simplify(
        "later",
        [c("later", [v(0), v(1)])],
        and(vec![
            c("choice", [v(1)]).into(),
            c("pair", [v(0), v(1)]).into(),
        ]),
    ));
    let q = Query {
        constraints: vec![c("choice", [v(100)]), c("later", [v(100), v(101)])],
        outputs: vec![("x".into(), Var(100)), ("y".into(), Var(101))],
    };
    compare(&rules, 1, &q);
}
#[test]
fn interference_and_unsupported_source_fail_admission() {
    use finite_phase::Error;
    let p = producer("choice", &["a", "b"]);
    let external = Rule::simplify("steal", [c("choice", [v(0)]), c("token", [])], Goal::True);
    assert!(matches!(
        Prepared::new(&[p.clone(), external.clone()], 1),
        Err(Error::Source(_))
    ));
    assert!(Prepared::new(&[external, p.clone()], 2).is_err());
    let dynamic = Rule::simplify("spawn", [c("spawn", [v(0)])], c("choice", [v(0)]).into());
    assert!(Prepared::new(&[p.clone(), dynamic], 2).is_err());
    let fresh = Rule::simplify("fresh", [c("fresh", [v(0)])], eq(v(0), v(1)));
    assert!(Prepared::new(&[p.clone(), fresh], 2).is_err());
    let mut guarded = Rule::simplify("guard", [c("guard", [v(0)])], Goal::True);
    guarded
        .guards
        .push(chr_syntax::Guard::Equal(v(0), atom("a")));
    assert!(Prepared::new(&[p.clone(), guarded], 2).is_err());
    let nonlinear = Rule::simplify("same", [c("same", [v(0), v(0)])], Goal::True);
    assert!(Prepared::new(&[p.clone(), nonlinear], 2).is_err());
    let ordinary = Rule::simplify("step", [c("step", [])], Goal::True);
    assert!(Prepared::new(&[ordinary, p.clone()], 2).is_err());
    assert!(Prepared::new(&[p.clone(), p], 2).is_err());
}
#[test]
fn suspension_and_all_limits_are_explicit_without_partial_answers() {
    use finite_phase::Error;
    let rules = vec![
        producer("choice", &["a", "b"]),
        Rule::simplify("known", [c("check", [atom("a")])], Goal::True),
    ];
    let p = Prepared::new(&rules, 2).unwrap();
    let q = Query {
        constraints: vec![c("choice", [v(100)]), c("check", [v(100)])],
        outputs: vec![],
    };
    // The a branch succeeds, the b branch suspends: the whole call must reject.
    assert_eq!(
        p.solve(&q, Limits::default()).unwrap_err(),
        Error::Suspended
    );
    let unknown = Query {
        constraints: vec![c("choice", [v(100)]), c("check", [v(101)])],
        outputs: vec![],
    };
    assert_eq!(
        p.solve(&unknown, Limits::default()).unwrap_err(),
        Error::Suspended
    );
    let rules = vec![producer("choice", &["a", "b"])];
    let p = Prepared::new(&rules, 1).unwrap();
    let q = Query {
        constraints: vec![c("choice", [v(100)])],
        outputs: vec![("x".into(), Var(100))],
    };
    for limits in [
        Limits {
            steps: 0,
            ..Limits::default()
        },
        Limits {
            partitions: 0,
            ..Limits::default()
        },
        Limits {
            solutions: 1,
            ..Limits::default()
        },
        Limits {
            term_nodes: 0,
            ..Limits::default()
        },
    ] {
        assert!(matches!(p.solve(&q, limits), Err(Error::Limit(_))));
    }
    let deep = (0..130).fold(atom("z"), |t, _| chr_syntax::t("s", [t]));
    let mut deep_q = q.clone();
    deep_q.constraints.push(c("outside", [deep]));
    assert_eq!(
        p.solve(&deep_q, Limits::default()).unwrap_err(),
        Error::Limit("term depth")
    );
    let rules = vec![producer("choice", &["a", "a"])];
    let p = Prepared::new(&rules, 1).unwrap();
    let q = Query {
        constraints: vec![c("choice", [v(100)]); 128],
        outputs: vec![],
    };
    assert_eq!(
        p.solve(&q, Limits::default()).unwrap_err(),
        Error::MultiplicityOverflow
    );
    let rules = vec![
        producer("choice", &["a", "b"]),
        Rule::simplify("spin", [c("spin", [])], c("spin", []).into()),
    ];
    let p = Prepared::new(&rules, 2).unwrap();
    let q = Query {
        constraints: vec![c("choice", [v(100)]), c("spin", [])],
        outputs: vec![],
    };
    assert_eq!(
        p.solve(
            &q,
            Limits {
                steps: 32,
                ..Limits::default()
            }
        )
        .unwrap_err(),
        Error::Limit("steps")
    );
}

#[test]
fn source_names_and_unknown_recursive_boundaries_are_checked() {
    let p = producer("choice", &["a", "b"]);
    let mut other = Rule::simplify("choice", [c("other", [])], Goal::True);
    assert!(Prepared::new(&[p.clone(), other.clone()], 2).is_err());
    other.name = "other".into();
    assert!(Prepared::new(&[p, other], 2).is_ok());
    let s = source::Schema {
        family: "oldest-first",
        work: 0,
        payload: 0,
        resource: true,
        fail_tail: false,
    };
    let mut q = s.query(0, false);
    q.constraints
        .iter_mut()
        .find(|c| c.name == "check")
        .unwrap()
        .args[0] = v(123);
    assert_eq!(
        Prepared::new(&s.rules(), 6)
            .unwrap()
            .solve(&q, Limits::default())
            .unwrap_err(),
        finite_phase::Error::Suspended
    );
    let rules = vec![
        producer("choice", &["a", "b"]),
        Rule::simplify("done", [c("run", [atom("a")])], Goal::True),
        Rule::simplify(
            "loop",
            [c("run", [atom("b")])],
            c("run", [atom("b")]).into(),
        ),
    ];
    let q = Query {
        constraints: vec![c("choice", [v(100)]), c("run", [v(100)])],
        outputs: vec![("x".into(), Var(100))],
    };
    assert_eq!(
        Prepared::new(&rules, 3)
            .unwrap()
            .solve(
                &q,
                Limits {
                    steps: 32,
                    ..Limits::default()
                }
            )
            .unwrap_err(),
        finite_phase::Error::Limit("steps")
    );
}

#[test]
fn finite_phase_service_can_cancel_without_publishing_partial_answers() {
    use finite_phase::Event;
    let s = source::Schema {
        family: "oldest-first",
        work: 2,
        payload: 1,
        resource: true,
        fail_tail: false,
    };
    let rules = s.rules();
    let p = Prepared::new(&rules, 6).unwrap();
    for stop in [0, 1, 16, 100] {
        let q = s.query(64, false);
        let mut machine = p.start(&q, Limits::default()).unwrap();
        for _ in 0..stop {
            assert!(matches!(machine.advance().unwrap(), Event::Progress));
        }
        assert!(machine.retained().0 > 0);
        drop(machine);
        let fresh = s.query(2, true);
        runtime_support::same_raw(
            resume(
                &rules,
                p.solve(&fresh, Limits::default()).unwrap().solutions,
            ),
            runtime_support::run(&rules, &fresh, 2_000_000),
        );
    }
    let q = s.query(4, false);
    let mut machine = p.start(&q, Limits::default()).unwrap();
    let report = loop {
        match machine.advance().unwrap() {
            Event::Progress => (),
            Event::Complete(r) => break r,
            Event::Exhausted => panic!("missing completion"),
        }
    };
    assert_eq!(machine.retained(), (0, 0));
    assert!(matches!(machine.advance().unwrap(), Event::Exhausted));
    runtime_support::same_raw(resume(&rules, report.solutions), s.expected_query(4, false));
    let mut machine = p
        .start(
            &s.query(64, false),
            Limits {
                steps: 1,
                ..Limits::default()
            },
        )
        .unwrap();
    assert!(matches!(machine.advance().unwrap(), Event::Progress));
    assert!(machine.advance().is_err());
    assert_eq!(machine.retained(), (0, 0));
    assert!(matches!(machine.advance().unwrap(), Event::Exhausted));
}
