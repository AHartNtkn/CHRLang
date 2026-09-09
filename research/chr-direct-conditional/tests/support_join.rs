#[allow(dead_code)]
mod composition_support;
#[allow(dead_code)]
mod runtime_support;
use chr_direct_conditional::engine::{Event, PreparedRuleset};
use chr_syntax::{Answer, Query, Rule, and, atom, c, or, v};

#[test]
fn correlation_controls_which_occurrences_can_join() {
    for family in ["incompatible", "correlated", "independent"] {
        let post = |p: &str, x: &str| c(p, [atom(x)]).into();
        let body = match family {
            "incompatible" => or(post("p", "a"), post("q", "b")),
            "correlated" => or(
                and([post("p", "a"), post("q", "a")]),
                and([post("p", "b"), post("q", "b")]),
            ),
            _ => and([
                or(post("p", "a"), post("p", "b")),
                or(post("q", "a"), post("q", "b")),
            ]),
        };
        let rules = vec![
            Rule::simplify("start", [c("start", [])], body),
            Rule::propagate(
                "join",
                [c("p", [v(0)]), c("q", [v(1)])],
                c("seen", [v(0), v(1)]).into(),
            ),
        ];
        let query = Query {
            constraints: vec![c("start", [])],
            outputs: vec![],
        };
        let answer = |residual| Answer {
            outputs: vec![],
            residual,
        };
        let expected = match family {
            "incompatible" => vec![
                answer(vec![c("p", [atom("a")])]),
                answer(vec![c("q", [atom("b")])]),
            ],
            "correlated" => ["a", "b"]
                .into_iter()
                .map(|x| {
                    answer(vec![
                        c("p", [atom(x)]),
                        c("q", [atom(x)]),
                        c("seen", [atom(x), atom(x)]),
                    ])
                })
                .collect(),
            _ => ["a", "b"]
                .into_iter()
                .flat_map(|x| {
                    ["a", "b"].into_iter().map(move |y| {
                        answer(vec![
                            c("p", [atom(x)]),
                            c("q", [atom(y)]),
                            c("seen", [atom(x), atom(y)]),
                        ])
                    })
                })
                .collect(),
        };
        runtime_support::same_raw(
            runtime_support::run(&rules, &query, 500_000),
            expected.clone(),
        );
        runtime_support::same_raw(
            composition_support::Engine::new(0, &rules, &query).collect(),
            expected.clone(),
        );
        let mut engine = PreparedRuleset::new(rules).unwrap().start(query).unwrap();
        let mut answers = vec![];
        let mut exhausted = false;
        for _ in 0..500_000 {
            match engine.tick() {
                Event::Answer(a) => answers.push(a),
                Event::Exhausted => {
                    exhausted = true;
                    break;
                }
                Event::Progress => (),
            }
        }
        assert!(exhausted);
        runtime_support::same_raw(answers, expected);
        #[cfg(feature = "metrics")]
        assert_eq!(
            engine.stats().discovered_tuples,
            match family {
                "incompatible" =>
                    if cfg!(feature = "support-join") {
                        1
                    } else {
                        2
                    },
                "correlated" =>
                    if cfg!(feature = "support-join") {
                        3
                    } else {
                        5
                    },
                _ => 5,
            }
        );
        println!(
            "family={family} tuples={} ticks={}",
            engine.stats().discovered_tuples,
            engine.stats().ticks
        );
    }
}

#[test]
fn three_head_suffix_products_preserve_full_answers() {
    for family in ["incompatible", "correlated", "independent", "dense"] {
        for n in [0, 1, 8] {
            let post = |p: &str, x: &str| c(p, [atom(x)]).into();
            let body = match family {
                "incompatible" => or(post("p", "a"), post("q", "b")),
                "correlated" => or(
                    and([post("p", "a"), post("q", "a")]),
                    and([post("p", "b"), post("q", "b")]),
                ),
                "independent" => and([
                    or(post("p", "a"), post("p", "b")),
                    or(post("q", "a"), post("q", "b")),
                ]),
                _ => and([post("p", "a"), post("q", "a")]),
            };
            let rules = vec![
                Rule::simplify("start", [c("start", [])], body),
                Rule::propagate(
                    "join",
                    [c("p", [v(0)]), c("q", [v(1)]), c("r", [v(2)])],
                    c("seen", [v(0), v(1), v(2)]).into(),
                ),
            ];
            let suffix = (0..n)
                .map(|i| c("r", [atom(&format!("r{i}"))]))
                .collect::<Vec<_>>();
            let mut constraints = suffix.clone();
            constraints.push(c("start", []));
            let query = Query {
                constraints,
                outputs: vec![],
            };
            let pairs = match family {
                "incompatible" => vec![(Some("a"), None), (None, Some("b"))],
                "correlated" => vec![(Some("a"), Some("a")), (Some("b"), Some("b"))],
                "independent" => vec![
                    (Some("a"), Some("a")),
                    (Some("a"), Some("b")),
                    (Some("b"), Some("a")),
                    (Some("b"), Some("b")),
                ],
                _ => vec![(Some("a"), Some("a"))],
            };
            let expected = pairs
                .into_iter()
                .map(|(x, y)| {
                    let mut residual = suffix.clone();
                    if let Some(x) = x {
                        residual.push(c("p", [atom(x)]));
                    }
                    if let Some(y) = y {
                        residual.push(c("q", [atom(y)]));
                    }
                    if let (Some(x), Some(y)) = (x, y) {
                        for i in 0..n {
                            residual.push(c("seen", [atom(x), atom(y), atom(&format!("r{i}"))]));
                        }
                    }
                    Answer {
                        outputs: vec![],
                        residual,
                    }
                })
                .collect::<Vec<_>>();
            runtime_support::same_raw(
                runtime_support::run(&rules, &query, 500_000),
                expected.clone(),
            );
            runtime_support::same_raw(
                composition_support::Engine::new(0, &rules, &query).collect(),
                expected.clone(),
            );
            let mut e = PreparedRuleset::new(rules).unwrap().start(query).unwrap();
            let mut actual = vec![];
            let mut exhausted = false;
            for _ in 0..500_000 {
                match e.tick() {
                    Event::Answer(a) => actual.push(a),
                    Event::Exhausted => {
                        exhausted = true;
                        break;
                    }
                    Event::Progress => (),
                }
            }
            assert!(exhausted);
            runtime_support::same_raw(actual, expected);
            println!(
                "suffix family={family} n={n} tuples={} ticks={}",
                e.stats().discovered_tuples,
                e.stats().ticks
            );
        }
    }
}
