#[allow(dead_code)]
mod runtime_support;
use chr_direct_conditional::engine::{Event, PreparedRuleset};
use chr_syntax::{Answer, Goal, Query, Rule, Term, and, atom, c, eq, or, t, v};

#[derive(Clone, Copy, Debug)]
enum Alphabet {
    Binary,
    Duplicate,
    Nested,
}
fn source(alphabet: Alphabet, reject: bool) -> Vec<Rule> {
    let assign = |name: &str| eq(v(1), t("cons", [atom(name), v(2)]));
    let choice = match alphabet {
        Alphabet::Binary => or(assign("a"), assign("b")),
        Alphabet::Duplicate => or(assign("a"), assign("a")),
        Alphabet::Nested => or(assign("a"), or(assign("b"), assign("c"))),
    };
    let mut rules = vec![
        Rule::propagate("watch", [c("token", [v(0)])], c("seen", [v(0)]).into()),
        Rule::simplify(
            "base",
            [c("build", [atom("z"), v(0)])],
            eq(v(0), atom("nil")),
        ),
        Rule::simplify(
            "step",
            [c("build", [t("s", [v(0)]), v(1)])],
            and([choice, c("build", [v(0), v(2)]).into()]),
        ),
        Rule::simplify(
            "use",
            [c("task", [v(0), v(1)]), c("token", [v(2)])],
            c("out", [v(0), v(0), v(1), v(1), v(2), v(2)]).into(),
        ),
    ];
    if reject {
        rules.push(Rule::simplify(
            "reject",
            [c(
                "out",
                [t("cons", [atom("b"), v(0)]), v(1), v(2), v(3), v(4), v(5)],
            )],
            Goal::Fail,
        ));
    }
    rules
}
fn query(depth: usize, two: bool, token_first: bool) -> Query {
    let n = (0..depth).fold(atom("z"), |x, _| t("s", [x]));
    let mut constraints = vec![
        c("build", [n.clone(), v(100)]),
        c("task", [v(100), v(if two { 101 } else { 100 })]),
    ];
    if two {
        constraints.push(c("build", [n, v(101)]));
    }
    if token_first {
        constraints.insert(0, c("token", [v(200)]));
    } else {
        constraints.push(c("token", [v(200)]));
    }
    Query {
        constraints,
        outputs: vec![],
    }
}
fn words(depth: usize, alphabet: Alphabet) -> Vec<Term> {
    if depth == 0 {
        return vec![atom("nil")];
    }
    let letters: &[&str] = match alphabet {
        Alphabet::Binary => &["a", "b"],
        Alphabet::Duplicate => &["a", "a"],
        Alphabet::Nested => &["a", "b", "c"],
    };
    let tails = words(depth - 1, alphabet);
    letters
        .iter()
        .flat_map(|letter| {
            tails
                .iter()
                .map(move |tail| t("cons", [atom(letter), tail.clone()]))
        })
        .collect()
}
fn expected(depth: usize, two: bool, alphabet: Alphabet, reject: bool) -> Vec<Answer> {
    let words = words(depth, alphabet);
    let mut result = vec![];
    for x in &words {
        if reject && matches!(x,Term::App(n,xs) if n=="cons" && xs[0]==atom("b")) {
            continue;
        }
        let ys = if two { words.clone() } else { vec![x.clone()] };
        for y in ys {
            result.push(Answer {
                outputs: vec![],
                residual: vec![
                    c("seen", [v(900)]),
                    c("out", [x.clone(), x.clone(), y.clone(), y, v(900), v(900)]),
                ],
            });
        }
    }
    result
}
fn run(prepared: &PreparedRuleset, input: Query) -> Vec<Answer> {
    let mut engine = prepared.start(input).unwrap();
    let mut result = vec![];
    for _ in 0..2_000_000 {
        match engine.tick() {
            Event::Progress => {}
            Event::Answer(a) => result.push(a),
            Event::Exhausted => return result,
        }
    }
    panic!("registered Conditional gate cutoff")
}
#[test]
fn dynamic_births_resources_and_complete_observations() {
    for (alphabet, reject, cases) in [
        (
            Alphabet::Binary,
            false,
            vec![
                (0, false),
                (1, false),
                (2, false),
                (3, false),
                (1, true),
                (2, true),
            ],
        ),
        (
            Alphabet::Nested,
            false,
            vec![(1, false), (2, false), (2, true)],
        ),
        (
            Alphabet::Duplicate,
            false,
            vec![(1, false), (2, false), (3, false)],
        ),
        (Alphabet::Binary, true, vec![(1, false), (2, false)]),
    ] {
        let rules = source(alphabet, reject);
        let prepared = PreparedRuleset::new(rules.clone()).unwrap();
        let compiled = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
        let lowered = chr_direct_choice::words::Prepared::new(&rules).unwrap();
        let direct = chr_direct_choice::engine::PreparedRuleset::new(rules.clone()).unwrap();
        for (depth, two) in cases {
            for first in [false, true] {
                let input = query(depth, two, first);
                let oracle = expected(depth, two, alphabet, reject);
                runtime_support::same_raw(
                    lowered.start(input.clone()).unwrap().collect(),
                    oracle.clone(),
                );
                for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                    runtime_support::same_raw(
                        run_compiled(
                            &compiled,
                            input.clone(),
                            chr_compiled::Policy::Global,
                            access,
                        ),
                        oracle.clone(),
                    );
                }
                runtime_support::same_raw(
                    runtime_support::run(&rules, &input, 200_000),
                    oracle.clone(),
                );
                runtime_support::same_raw(run_direct(&direct, input.clone()), oracle.clone());
                let actual = run(&prepared, input);
                let raw = actual.len();
                runtime_support::same_raw(actual, oracle);
                println!(
                    "alphabet={alphabet:?} reject={reject} depth={depth} two={two} token_first={first} raw={raw}"
                );
            }
        }
    }
}
#[test]
fn oracle_preserves_correlation_aliases_and_raw_multiplicity() {
    let single = expected(2, false, Alphabet::Binary, false);
    let independent = expected(2, true, Alphabet::Binary, false);
    assert_eq!(single.len(), 4);
    assert_eq!(independent.len(), 16);
    assert_eq!(expected(2, false, Alphabet::Nested, false).len(), 9);
    assert_eq!(expected(2, false, Alphabet::Duplicate, false).len(), 4);
    let good = single[0].clone();
    let mut missing = good.clone();
    missing.residual.remove(0);
    assert!(!chr_observe::equivalent(
        &good,
        &missing,
        &mut Default::default()
    ));
    let mut broken = good.clone();
    broken.residual[1].args[5] = v(901);
    assert!(!chr_observe::equivalent(
        &good,
        &broken,
        &mut Default::default()
    ));
    let failed_word = expected(1, false, Alphabet::Binary, false).pop().unwrap();
    assert!(
        expected(1, false, Alphabet::Binary, true)
            .iter()
            .all(|a| !chr_observe::equivalent(a, &failed_word, &mut Default::default()))
    );
}
#[test]
fn finite_sibling_is_observed_without_claiming_global_completion() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [])],
            or(c("loop", []).into(), c("finite", [v(0), v(0)]).into()),
        ),
        Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
    ];
    let mut engine = PreparedRuleset::new(rules)
        .unwrap()
        .start(Query {
            constraints: vec![c("start", [])],
            outputs: vec![],
        })
        .unwrap();
    let expected = Answer {
        outputs: vec![],
        residual: vec![c("finite", [v(900), v(900)])],
    };
    let mut answers = 0;
    for _ in 0..20_000 {
        match engine.tick() {
            Event::Progress => {}
            Event::Answer(a) => {
                assert!(chr_observe::equivalent(
                    &a,
                    &expected,
                    &mut Default::default()
                ));
                answers += 1;
            }
            Event::Exhausted => panic!("recursive sibling cannot exhaust"),
        }
    }
    assert_eq!(answers, 1);
}

fn run_direct(prepared: &chr_direct_choice::engine::PreparedRuleset, input: Query) -> Vec<Answer> {
    use chr_direct_choice::engine::Event;
    let mut engine = prepared.start(input).unwrap();
    let mut answers = Vec::new();
    for _ in 0..2_000_000 {
        match engine.tick() {
            Event::Progress => {}
            Event::Answer(a) => answers.push(a),
            Event::Exhausted => return answers,
        }
    }
    panic!("registered direct graph gate cutoff");
}
#[test]
fn direct_graph_publishes_finite_sibling_without_claiming_exhaustion() {
    use chr_direct_choice::engine::{Event, PreparedRuleset};
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [])],
            or(c("loop", []).into(), c("finite", [v(0), v(0)]).into()),
        ),
        Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
    ];
    let mut engine = PreparedRuleset::new(rules)
        .unwrap()
        .start(Query {
            constraints: vec![c("start", [])],
            outputs: vec![],
        })
        .unwrap();
    let expected = Answer {
        outputs: vec![],
        residual: vec![c("finite", [v(900), v(900)])],
    };
    let mut count = 0;
    for _ in 0..20_000 {
        match engine.tick() {
            Event::Progress => {}
            Event::Answer(a) => {
                assert!(chr_observe::equivalent(
                    &a,
                    &expected,
                    &mut Default::default()
                ));
                count += 1;
            }
            Event::Exhausted => panic!("continuing direct graph sibling cannot exhaust"),
        }
    }
    assert_eq!(count, 1);
}

fn check_all_source_engines(rules: Vec<Rule>, input: Query, expected: Vec<Answer>) {
    runtime_support::same_raw(
        runtime_support::run(&rules, &input, 200_000),
        expected.clone(),
    );
    let prepared = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
        runtime_support::same_raw(
            run_compiled(
                &prepared,
                input.clone(),
                chr_compiled::Policy::Global,
                access,
            ),
            expected.clone(),
        );
    }
    runtime_support::same_raw(
        run(&PreparedRuleset::new(rules.clone()).unwrap(), input.clone()),
        expected.clone(),
    );
    runtime_support::same_raw(
        run_direct(
            &chr_direct_choice::engine::PreparedRuleset::new(rules).unwrap(),
            input,
        ),
        expected,
    );
}

#[test]
fn effectful_choices_competing_consumers_and_late_arrivals() {
    let rules = vec![
        Rule::propagate("watch", [c("token", [v(0)])], c("seen", [v(0)]).into()),
        Rule::simplify(
            "seed",
            [c("seed", [v(0)])],
            or(
                and([eq(v(0), atom("a")), c("ask", [v(0)]).into()]),
                and([eq(v(0), atom("b")), c("late", [v(0)]).into()]),
            ),
        ),
        Rule::simplify(
            "take_a",
            [c("ask", [atom("a")]), c("token", [v(0)])],
            c("won", [atom("a"), v(0)]).into(),
        ),
        Rule::simplify("delay", [c("late", [v(0)])], c("ask", [v(0)]).into()),
        Rule::simplify(
            "take_b",
            [c("ask", [atom("b")]), c("token", [v(0)])],
            c("won", [atom("b"), v(0)]).into(),
        ),
    ];
    let query = Query {
        constraints: vec![
            c("seed", [v(100)]),
            c("token", [v(200)]),
            c("payload", [v(100)]),
        ],
        outputs: vec![("value".into(), chr_syntax::Var(100))],
    };
    let expected = ["a", "b"]
        .into_iter()
        .map(|value| Answer {
            outputs: vec![("value".into(), atom(value))],
            residual: vec![
                c("seen", [v(900)]),
                c("payload", [atom(value)]),
                c("won", [atom(value), v(900)]),
            ],
        })
        .collect();
    check_all_source_engines(rules, query, expected);
}

#[test]
fn nonbinding_heads_and_guards_wait_for_real_aliases() {
    let same = Rule::simplify("same", [c("pair", [v(0), v(0)])], c("same", [v(0)]).into());
    let bind = Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1)));
    for later in [false, true] {
        let mut constraints = vec![c("pair", [v(100), v(101)])];
        if later {
            constraints.push(c("bind", [v(100), v(101)]));
        }
        let expected = if later {
            vec![c("same", [v(900)])]
        } else {
            vec![c("pair", [v(900), v(901)])]
        };
        check_all_source_engines(
            vec![same.clone(), bind.clone()],
            Query {
                constraints,
                outputs: vec![],
            },
            vec![Answer {
                outputs: vec![],
                residual: expected,
            }],
        );
        let guarded = Rule {
            name: "guarded".into(),
            kept: vec![c("p", [v(0)]), c("q", [v(1)])],
            removed: vec![],
            guards: vec![chr_syntax::Guard::Equal(v(0), v(1))],
            body: c("equal", [v(0), v(1)]).into(),
        };
        let mut constraints = vec![c("p", [v(100)]), c("q", [v(101)])];
        if later {
            constraints.push(c("bind", [v(100), v(101)]));
        }
        let residual = if later {
            vec![
                c("p", [v(900)]),
                c("q", [v(900)]),
                c("equal", [v(900), v(900)]),
            ]
        } else {
            vec![c("p", [v(900)]), c("q", [v(901)])]
        };
        check_all_source_engines(
            vec![guarded, bind.clone()],
            Query {
                constraints,
                outputs: vec![],
            },
            vec![Answer {
                outputs: vec![],
                residual,
            }],
        );
    }
}

#[test]
fn propagation_respects_occurrence_identity_and_fresh_application_unknowns() {
    let pair = Rule::propagate(
        "pair",
        [c("p", [v(0)]), c("p", [v(1)])],
        c("seen", [v(0), v(1)]).into(),
    );
    for n in [1, 2] {
        let constraints = vec![c("p", [atom("a")]); n];
        let mut residual = constraints.clone();
        if n == 2 {
            residual.extend(vec![c("seen", [atom("a"), atom("a")]); 2]);
        }
        check_all_source_engines(
            vec![pair.clone()],
            Query {
                constraints,
                outputs: vec![],
            },
            vec![Answer {
                outputs: vec![],
                residual,
            }],
        );
    }
    let fresh = Rule::simplify(
        "fresh",
        [c("start", [v(0)])],
        and([c("done", [v(0), v(1), v(1)]).into(), Goal::True]),
    );
    check_all_source_engines(
        vec![fresh],
        Query {
            constraints: vec![c("start", [atom("a")]), c("start", [atom("b")])],
            outputs: vec![],
        },
        vec![Answer {
            outputs: vec![],
            residual: vec![
                c("done", [atom("a"), v(900), v(900)]),
                c("done", [atom("b"), v(901), v(901)]),
            ],
        }],
    );
}

#[test]
fn disconnected_failure_prevents_early_answer_publication() {
    for bad in [
        Goal::Fail,
        eq(v(1), t("f", [v(1)])),
        and([eq(v(1), v(2)), eq(v(2), t("f", [v(1)]))]),
    ] {
        let rules = vec![Rule::simplify(
            "start",
            [c("start", [])],
            or(
                and([c("out", [atom("bad")]).into(), bad]),
                c("out", [atom("good")]).into(),
            ),
        )];
        check_all_source_engines(
            rules,
            Query {
                constraints: vec![c("start", [])],
                outputs: vec![],
            },
            vec![Answer {
                outputs: vec![],
                residual: vec![c("out", [atom("good")])],
            }],
        );
    }
}

#[test]
fn mixed_kept_consumed_heads_preserve_joint_output_identity() {
    let rule = Rule {
        name: "serve".into(),
        kept: vec![c("token", [v(0)])],
        removed: vec![c("ask", [v(1)])],
        guards: vec![],
        body: c("served", [v(1), v(0)]).into(),
    };
    check_all_source_engines(
        vec![rule],
        Query {
            constraints: vec![
                c("token", [v(100)]),
                c("ask", [atom("a")]),
                c("ask", [atom("b")]),
            ],
            outputs: vec![("handle".into(), chr_syntax::Var(100))],
        },
        vec![Answer {
            outputs: vec![("handle".into(), v(900))],
            residual: vec![
                c("token", [v(900)]),
                c("served", [atom("a"), v(900)]),
                c("served", [atom("b"), v(900)]),
            ],
        }],
    );
}

fn run_compiled(
    prepared: &chr_compiled::PreparedRuleset,
    input: Query,
    policy: chr_compiled::Policy,
    access: chr_compiled::Access,
) -> Vec<Answer> {
    use chr_compiled::SearchEvent;
    let mut engine = prepared.start_search(input, policy, access).unwrap();
    let mut answers = Vec::new();
    for _ in 0..2_000_000 {
        match engine.tick() {
            SearchEvent::Complete(mut branch) => answers.push(
                branch
                    .engine
                    .observe()
                    .expect("complete branch must observe"),
            ),
            SearchEvent::Exhausted => return answers,
            SearchEvent::Progress | SearchEvent::Failed(_) | SearchEvent::Split { .. } => {}
        }
    }
    panic!("registered compiled source gate cutoff");
}
#[test]
fn compiled_global_controls_publish_finite_sibling_without_exhaustion() {
    use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [])],
            or(c("loop", []).into(), c("finite", [v(0), v(0)]).into()),
        ),
        Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
    ];
    let prepared = PreparedRuleset::new(rules, None).unwrap();
    for access in [Access::Scan, Access::Indexed] {
        let mut engine = prepared
            .start_search(
                Query {
                    constraints: vec![c("start", [])],
                    outputs: vec![],
                },
                Policy::Global,
                access,
            )
            .unwrap();
        let expected = Answer {
            outputs: vec![],
            residual: vec![c("finite", [v(900), v(900)])],
        };
        let mut count = 0;
        for _ in 0..20_000 {
            match engine.tick() {
                SearchEvent::Complete(mut branch) => {
                    runtime_support::same_raw(
                        vec![branch.engine.observe().unwrap()],
                        vec![expected.clone()],
                    );
                    count += 1;
                }
                SearchEvent::Exhausted => panic!("continuing compiled sibling cannot exhaust"),
                SearchEvent::Progress | SearchEvent::Failed(_) | SearchEvent::Split { .. } => {}
            }
        }
        assert_eq!(count, 1);
    }
}

fn same_answers(actual: &[Answer], expected: &[Answer]) -> bool {
    if actual.len() != expected.len() {
        return false;
    }
    let mut unmatched = expected.to_vec();
    for answer in actual {
        let Some(index) = unmatched
            .iter()
            .position(|other| chr_observe::equivalent(answer, other, &mut Default::default()))
        else {
            return false;
        };
        unmatched.swap_remove(index);
    }
    unmatched.is_empty()
}
#[test]
fn active_policy_word_screen_records_complete_answer_agreement() {
    let mut agreements = 0;
    let mut cases = 0;
    for (alphabet, reject, depths) in [
        (
            Alphabet::Binary,
            false,
            vec![
                (0, false),
                (1, false),
                (2, false),
                (3, false),
                (1, true),
                (2, true),
            ],
        ),
        (
            Alphabet::Nested,
            false,
            vec![(1, false), (2, false), (2, true)],
        ),
        (
            Alphabet::Duplicate,
            false,
            vec![(1, false), (2, false), (3, false)],
        ),
        (Alphabet::Binary, true, vec![(1, false), (2, false)]),
    ] {
        let prepared = chr_compiled::PreparedRuleset::new(source(alphabet, reject), None).unwrap();
        for (depth, two) in depths {
            for first in [false, true] {
                let oracle = expected(depth, two, alphabet, reject);
                for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                    let actual = run_compiled(
                        &prepared,
                        query(depth, two, first),
                        chr_compiled::Policy::Active,
                        access,
                    );
                    let agrees = same_answers(&actual, &oracle);
                    let mut policy_expected = oracle.clone();
                    if !first {
                        for answer in &mut policy_expected {
                            answer.residual.retain(|c| c.name != "seen");
                        }
                    }
                    assert!(
                        same_answers(&actual, &policy_expected),
                        "Active difference exceeds the token observation"
                    );
                    cases += 1;
                    agreements += usize::from(agrees);
                    if cases == 1 {
                        println!("active first difference actual={actual:?} expected={oracle:?}");
                    }
                    println!(
                        "active alphabet={alphabet:?} reject={reject} depth={depth} two={two} token_first={first} access={access:?} expected_raw={} actual_raw={} agrees={agrees}",
                        oracle.len(),
                        actual.len()
                    );
                }
            }
        }
    }
    assert_eq!(cases, 56);
    println!(
        "active agreements={agreements}/{cases}; this is a diagnostic, not admission of mismatched cases"
    );
}

#[test]
fn active_nonchoice_trace_explains_missing_token_observation() {
    use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
    let rules = source(Alphabet::Binary, false);
    let prepared = PreparedRuleset::new(rules.clone(), None).unwrap();
    for first in [false, true] {
        let mut root = prepared
            .start(query(0, false, first), Policy::Active, Access::Indexed)
            .unwrap();
        root.enable_trace();
        let mut engine = root.into_search();
        let mut trace = None;
        for _ in 0..10_000 {
            match engine.tick() {
                SearchEvent::Complete(branch) => {
                    trace = Some(
                        branch
                            .engine
                            .trace()
                            .iter()
                            .map(|(rule, _)| rules[*rule].name.clone())
                            .collect::<Vec<_>>(),
                    );
                }
                SearchEvent::Exhausted => break,
                _ => {}
            }
        }
        let trace = trace.expect("bounded no-choice source must complete");
        assert!(trace.iter().any(|name| name == "use"));
        assert_eq!(trace.iter().any(|name| name == "watch"), first);
        if first {
            assert!(
                trace.iter().position(|n| n == "watch") < trace.iter().position(|n| n == "use")
            );
        }
        println!("Active no-choice token_first={first} rule_trace={trace:?}");
    }
}

#[test]
fn word_lowering_handles_asymmetric_queries_and_rejects_uncertified_inputs() {
    for alphabet in [Alphabet::Binary, Alphabet::Duplicate, Alphabet::Nested] {
        for reject in [false, true] {
            let rules = source(alphabet, reject);
            let prepared = chr_direct_choice::words::Prepared::new(&rules).unwrap();
            for (left, right) in [(0, 2), (1, 2), (2, 1), (2, 0)] {
                for reverse in [false, true] {
                    let depth = |n| (0..n).fold(atom("z"), |x, _| t("s", [x]));
                    let mut constraints = vec![
                        c("build", [depth(left), v(100)]),
                        c("build", [depth(right), v(101)]),
                        c(
                            "task",
                            if reverse {
                                [v(101), v(100)]
                            } else {
                                [v(100), v(101)]
                            },
                        ),
                        c("token", [v(200)]),
                    ];
                    // Vary insertion order independently of the task's argument order.
                    if reverse {
                        constraints.reverse();
                    }
                    let input = Query {
                        constraints,
                        outputs: vec![],
                    };
                    let (first, second) = if reverse {
                        (right, left)
                    } else {
                        (left, right)
                    };
                    let mut expected = Vec::new();
                    for x in words(first, alphabet) {
                        if reject && matches!(&x,Term::App(n,xs) if n=="cons" && xs[0]==atom("b")) {
                            continue;
                        }
                        for y in words(second, alphabet) {
                            expected.push(Answer {
                                outputs: vec![],
                                residual: vec![
                                    c("seen", [v(900)]),
                                    c("out", [x.clone(), x.clone(), y.clone(), y, v(900), v(900)]),
                                ],
                            });
                        }
                    }
                    let actual: Vec<_> = prepared
                        .start(input.clone())
                        .unwrap()
                        .take(100_001)
                        .collect();
                    assert!(actual.len() <= 100_000);
                    runtime_support::same_raw(actual, expected.clone());
                    check_all_source_engines(rules.clone(), input, expected);
                }
            }
            // A canceled iterator cannot affect a subsequent query on the preparation.
            let input = query(2, false, false);
            let mut partial = prepared.start(input.clone()).unwrap();
            assert!(partial.next().is_some());
            drop(partial);
            runtime_support::same_raw(
                prepared.start(input).unwrap().collect(),
                expected(2, false, alphabet, reject),
            );
            let mut changed = rules.clone();
            changed[0].body = Goal::True;
            assert!(chr_direct_choice::words::Prepared::new(&changed).is_err());
            let mut changed = rules.clone();
            changed.swap(0, 1);
            assert!(chr_direct_choice::words::Prepared::new(&changed).is_err());
            let mut changed = rules.clone();
            changed.push(Rule::simplify("extra", [c("out", [v(0)])], Goal::Fail));
            assert!(chr_direct_choice::words::Prepared::new(&changed).is_err());
            let valid = query(1, false, false);
            let mut invalids = Vec::new();
            let mut q = valid.clone();
            q.constraints.push(c("extra", []));
            invalids.push(q);
            let mut q = valid.clone();
            q.constraints.push(c("build", [atom("z"), v(100)]));
            invalids.push(q);
            let mut q = valid.clone();
            q.constraints.push(c("build", [atom("z"), v(102)]));
            invalids.push(q);
            let mut q = valid.clone();
            q.constraints[0].args[0] = v(300);
            invalids.push(q);
            let mut q = valid.clone();
            q.constraints[2].args[0] = v(100);
            invalids.push(q);
            let mut q = valid.clone();
            q.constraints.pop();
            invalids.push(q);
            let mut q = valid.clone();
            q.outputs.push(("x".into(), chr_syntax::Var(100)));
            invalids.push(q);
            let mut q = valid.clone();
            q.constraints.push(c("token", [v(201)]));
            invalids.push(q);
            let mut q = valid;
            q.constraints.push(c("task", [v(100), v(100)]));
            invalids.push(q);
            for q in invalids {
                assert!(prepared.start(q).is_err());
            }
        }
    }
}

#[test]
fn common_work_discrimination_and_no_choice_have_independent_complete_answers() {
    for family in ["plain", "shared", "discriminate"] {
        let mut rules = vec![Rule::simplify(
            "choose",
            [c("choose", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        )];
        if family == "discriminate" {
            for bits in 0..16 {
                let fields = (0..4)
                    .map(|i| atom(if bits & (1 << i) == 0 { "a" } else { "b" }))
                    .collect::<Vec<_>>();
                let pack = t("pack", fields);
                rules.push(Rule::simplify(
                    &format!("gate{bits}"),
                    [c("gate", [v(0), pack.clone()])],
                    c("work", [v(0), pack]).into(),
                ));
            }
        }
        rules.push(Rule::simplify(
            "step",
            [c("work", [t("s", [v(0)]), v(1)])],
            c("work", [v(0), v(1)]).into(),
        ));
        rules.push(Rule::simplify(
            "base",
            [c("work", [atom("z"), v(0)])],
            c("result", [v(0)]).into(),
        ));
        for n in [0, 1, 8, 32] {
            let depth = (0..n).fold(atom("z"), |x, _| t("s", [x]));
            let mut constraints = Vec::new();
            if family != "plain" {
                constraints.extend((0..4).map(|i| c("choose", [v(i)])));
            }
            constraints.push(c(
                if family == "discriminate" {
                    "gate"
                } else {
                    "work"
                },
                [depth, t("pack", (0..4).map(v).collect::<Vec<_>>())],
            ));
            let input = Query {
                constraints,
                outputs: (0..4)
                    .map(|i| (format!("o{i}"), chr_syntax::Var(i)))
                    .collect(),
            };
            let values: Vec<Vec<Term>> = if family == "plain" {
                vec![(900..904).map(v).collect()]
            } else {
                (0..16)
                    .map(|bits| {
                        (0..4)
                            .map(|i| atom(if bits & (1 << i) == 0 { "a" } else { "b" }))
                            .collect()
                    })
                    .collect()
            };
            let expected: Vec<_> = values
                .into_iter()
                .map(|fields| Answer {
                    outputs: fields
                        .iter()
                        .enumerate()
                        .map(|(i, t)| (format!("o{i}"), t.clone()))
                        .collect(),
                    residual: vec![c("result", [t("pack", fields)])],
                })
                .collect();
            println!(
                "common-work family={family} depth={n} expected_raw={}",
                expected.len()
            );
            check_all_source_engines(rules.clone(), input, expected);
        }
    }
}
