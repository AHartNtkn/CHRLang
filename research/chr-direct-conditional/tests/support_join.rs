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
