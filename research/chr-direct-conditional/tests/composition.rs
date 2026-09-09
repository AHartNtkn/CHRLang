mod runtime_support;
use chr_syntax::{Answer, Goal, Guard, Query, Rule, Var, and, atom, c, eq, or, t, v};

#[test]
fn mixed_resources_aliases_history_and_choice() {
    for history_first in [false, true] {
        let mut rules = vec![
            Rule::simplify(
                "start",
                [c("start", [v(0), v(1)])],
                or(
                    and([eq(v(0), atom("a")), c("take", [v(0), v(1)]).into()]),
                    c("second", [v(0), v(1)]).into(),
                ),
            ),
            Rule::simplify(
                "second",
                [c("second", [v(0), v(1)]), c("config", [v(2), v(3)])],
                and([
                    eq(v(0), v(2)),
                    c("take", [v(0), v(1)]).into(),
                    c("check", [v(3)]).into(),
                ]),
            ),
            Rule::simplify("fail", [c("check", [atom("fail")])], Goal::Fail),
            Rule::simplify("pass", [c("check", [atom("pass")])], Goal::True),
            Rule {
                name: "take".into(),
                kept: vec![c("permit", [])],
                removed: vec![c("take", [v(0), v(1)]), c("ticket", [t("box", [v(3)])])],
                guards: vec![Guard::Equal(v(0), v(3))],
                body: and([
                    eq(v(1), t("done", [v(0), v(2)])),
                    c("fresh", [v(2), v(2)]).into(),
                ]),
            },
        ];
        let history = Rule {
            name: "history".into(),
            kept: vec![c("permit", [])],
            removed: vec![],
            guards: vec![],
            body: c("mark", []).into(),
        };
        if history_first {
            rules.insert(0, history);
        } else {
            rules.push(history);
        }
        let compiled = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
        let contextual = chr_relational::contextual_execute::Prepared::new(&rules).unwrap();
        let conditional =
            chr_direct_conditional::engine::PreparedRuleset::new(rules.clone()).unwrap();
        for duplicate in [false, true] {
            for fail in [false, true] {
                for extra in [false, true] {
                    for reverse in [false, true] {
                        let second = if duplicate { "a" } else { "b" };
                        let config = c(
                            "config",
                            [atom(second), atom(if fail { "fail" } else { "pass" })],
                        );
                        let mut constraints = vec![
                            c("start", [v(10), v(11)]),
                            c("permit", []),
                            c("ticket", [t("box", [atom("a")])]),
                            c("ticket", [t("box", [atom("b")])]),
                            config.clone(),
                        ];
                        if extra {
                            constraints.push(c("ticket", [t("box", [atom("a")])]));
                        }
                        if reverse {
                            constraints.reverse();
                        }
                        let q = Query {
                            constraints,
                            outputs: vec![("x".into(), Var(10)), ("result".into(), Var(11))],
                        };
                        let mut expected = vec![];
                        for (arm, value) in [(false, "a"), (true, second)] {
                            if arm && fail {
                                continue;
                            }
                            let mut residual =
                                vec![c("permit", []), c("mark", []), c("fresh", [v(100), v(100)])];
                            if !arm {
                                residual.push(config.clone());
                            }
                            let mut tickets = vec!["a", "b"];
                            if extra {
                                tickets.push("a");
                            }
                            tickets.remove(tickets.iter().position(|x| *x == value).unwrap());
                            residual.extend(
                                tickets
                                    .into_iter()
                                    .map(|x| c("ticket", [t("box", [atom(x)])])),
                            );
                            expected.push(Answer {
                                outputs: vec![
                                    ("x".into(), atom(value)),
                                    ("result".into(), t("done", [atom(value), v(100)])),
                                ],
                                residual,
                            });
                        }
                        runtime_support::same_raw(
                            runtime_support::run(&rules, &q, 200000),
                            expected.clone(),
                        );
                        let mut e = compiled
                            .start_search(
                                q.clone(),
                                chr_compiled::Policy::Global,
                                chr_compiled::Access::Scan,
                            )
                            .unwrap();
                        let mut answers = vec![];
                        let mut exhausted = false;
                        for _ in 0..200000 {
                            match e.tick() {
                                chr_compiled::SearchEvent::Complete(mut b) => {
                                    answers.push(b.engine.observe().unwrap())
                                }
                                chr_compiled::SearchEvent::Exhausted => {
                                    exhausted = true;
                                    break;
                                }
                                _ => (),
                            }
                        }
                        assert!(exhausted);
                        runtime_support::same_raw(answers, expected.clone());
                        for mut e in [
                            contextual.start(&q),
                            contextual.start_shared_deductions(&q),
                            contextual.start_persistent_equality(&q, false),
                            contextual.start_persistent_equality(&q, true),
                        ] {
                            let mut answers = vec![];
                            let mut exhausted = false;
                            for _ in 0..200000 {
                                match e.advance() {
                                    chr_relational::contextual_execute::Step::Answer(a) => {
                                        answers.push(a)
                                    }
                                    chr_relational::contextual_execute::Step::Exhausted => {
                                        exhausted = true;
                                        break;
                                    }
                                    _ => (),
                                }
                            }
                            assert!(exhausted);
                            runtime_support::same_raw(answers, expected.clone());
                        }
                        let mut e = conditional.start(q).unwrap();
                        let mut answers = vec![];
                        let mut exhausted = false;
                        for _ in 0..200000 {
                            match e.tick() {
                                chr_direct_conditional::engine::Event::Answer(a) => answers.push(a),
                                chr_direct_conditional::engine::Event::Exhausted => {
                                    exhausted = true;
                                    break;
                                }
                                _ => (),
                            }
                        }
                        assert!(exhausted);
                        runtime_support::same_raw(answers, expected);
                    }
                }
            }
        }
    }
}
