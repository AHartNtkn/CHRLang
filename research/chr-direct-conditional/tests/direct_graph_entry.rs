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
        let direct = chr_direct_choice::engine::PreparedRuleset::new(rules.clone()).unwrap();
        for (depth, two) in cases {
            for first in [false, true] {
                let input = query(depth, two, first);
                let oracle = expected(depth, two, alphabet, reject);
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
