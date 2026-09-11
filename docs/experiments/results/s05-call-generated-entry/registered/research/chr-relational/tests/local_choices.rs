#[allow(dead_code)]
#[path = "support/local_ports.rs"]
mod local;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_syntax::{Answer, Goal, Query, Rule, Var, and, atom, c, eq, or, t, v};
use local::multihead::search::{Event, Prepared};
const METRICS: bool = cfg!(feature = "local-work");
fn ordered(actual: &[Answer], expected: &[Answer]) {
    assert_eq!(actual.len(), expected.len());
    for (a, b) in actual.iter().zip(expected) {
        assert!(
            chr_observe::equivalent(a, b, &mut Default::default()),
            "{a:?} != {b:?}"
        );
    }
}
fn run(prepared: &Prepared, q: &Query) -> Vec<Answer> {
    let mut s = prepared.start::<METRICS>(q);
    let mut out = vec![];
    for _ in 0..200_000 {
        match s.tick() {
            Event::Answer(a) => out.push(a),
            Event::Progress => (),
            Event::Exhausted => return out,
        }
    }
    panic!("local source cutoff")
}
fn check(rules: &[Rule], q: &Query) -> Vec<Answer> {
    let p = Prepared::compile(rules).unwrap();
    let got = run(&p, q);
    assert_eq!(p.owners(), 1);
    ordered(&got, &oracle::run(rules, q, 200_000));
    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
        let p = chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap();
        let mut s = p
            .start_search(q.clone(), chr_compiled::Policy::Global, access)
            .unwrap();
        let mut out = vec![];
        let mut done = false;
        for _ in 0..200_000 {
            match s.tick() {
                chr_compiled::SearchEvent::Complete(mut b) => out.push(b.engine.observe().unwrap()),
                chr_compiled::SearchEvent::Exhausted => {
                    done = true;
                    break;
                }
                _ => (),
            }
        }
        assert!(done);
        oracle::same_raw(got.clone(), out);
    }
    let p = chr_relational::contextual_execute::Prepared::new(rules).unwrap();
    let mut s = p.start(q);
    let mut out = vec![];
    let mut done = false;
    for _ in 0..200_000 {
        match s.advance() {
            chr_relational::contextual_execute::Step::Answer(a) => out.push(a),
            chr_relational::contextual_execute::Step::Exhausted => {
                done = true;
                break;
            }
            _ => (),
        }
    }
    assert!(done);
    oracle::same_raw(got.clone(), out);
    got
}
#[test]
fn choices_preserve_resources_aliases_history_and_complete_answers() {
    let mut count = 0;
    for tokens in 0..=2 {
        for duplicate in [false, true] {
            for alias in [false, true] {
                for fail in [false, true] {
                    for reverse in [false, true] {
                        let left = and([
                            c("request", [v(2), v(1), atom("left")]).into(),
                            if fail {
                                eq(v(2), t("cycle", [v(2)]))
                            } else {
                                Goal::True
                            },
                        ]);
                        let right = c(
                            "request",
                            [v(2), v(1), atom(if duplicate { "left" } else { "right" })],
                        )
                        .into();
                        let choice = if reverse {
                            or(right, left)
                        } else {
                            or(left, right)
                        };
                        let rules = vec![
                            Rule::simplify(
                                "split",
                                [c("start", [v(0), v(1)])],
                                and([eq(v(0), t("pair", [v(2), v(2)])), choice]),
                            ),
                            Rule::propagate(
                                "observe-token",
                                [c("token", [])],
                                c("seen", []).into(),
                            ),
                            Rule {
                                name: "consume".into(),
                                kept: vec![c("permission", [])],
                                removed: vec![c("request", [v(0), v(1), v(2)]), c("token", [])],
                                guards: vec![],
                                body: and([eq(v(0), atom("value")), eq(v(1), v(2))]),
                            },
                        ];
                        let mut constraints = vec![
                            c("start", [v(10), v(if alias { 10 } else { 11 })]),
                            c("permission", []),
                        ];
                        constraints.extend((0..tokens).map(|_| c("token", [])));
                        let q = Query {
                            constraints,
                            outputs: vec![
                                ("pair".into(), Var(10)),
                                ("result".into(), Var(if alias { 10 } else { 11 })),
                            ],
                        };
                        check(&rules, &q);
                        count += 1;
                    }
                }
            }
        }
    }
    assert_eq!(count, 48);
    println!("source_configurations={count}");
}
#[test]
fn a_choice_does_not_expose_an_unfinished_body_to_competing_consumers() {
    let branch = |marker| {
        and([
            c("request", [t("f", [atom("a")]), v(1), atom("newer")]).into(),
            eq(v(0), t("f", [atom("a")])),
            c(marker, []).into(),
        ])
    };
    let rules = vec![
        Rule::simplify(
            "consume",
            [c("request", [t("f", [v(0)]), v(1), v(2)]), c("token", [])],
            eq(v(1), v(2)),
        ),
        Rule::simplify(
            "trigger",
            [c("trigger", [v(0), v(1)]), c("token", [])],
            or(branch("left"), branch("right")),
        ),
    ];
    let q = Query {
        constraints: vec![
            c("request", [v(10), v(11), atom("older")]),
            c("trigger", [v(10), v(11)]),
            c("token", []),
            c("token", []),
        ],
        outputs: vec![("winner".into(), Var(11))],
    };
    let answers = check(&rules, &q);
    assert_eq!(answers.len(), 2);
    assert!(
        answers
            .iter()
            .all(|a| a.outputs == vec![("winner".into(), atom("older"))])
    );
}
#[test]
fn finite_sibling_progress_cancellation_and_prepared_reuse() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(c("loop", []).into(), eq(v(0), atom("answer"))),
        ),
        Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
    ];
    let p = Prepared::compile(&rules).unwrap();
    for variable in [0, 17] {
        let q = Query {
            constraints: vec![c("start", [v(variable)])],
            outputs: vec![("out".into(), Var(variable))],
        };
        let mut s = p.start::<METRICS>(&q);
        let mut found = false;
        for _ in 0..500 {
            match s.tick() {
                Event::Answer(a) => {
                    assert_eq!(
                        a,
                        Answer {
                            outputs: vec![("out".into(), atom("answer"))],
                            residual: vec![]
                        }
                    );
                    found = true;
                    break;
                }
                Event::Progress => (),
                Event::Exhausted => panic!("continuing source exhausted"),
            }
        }
        assert!(found);
        for _ in 0..32 {
            assert!(matches!(s.tick(), Event::Progress));
        }
        assert!(p.owners() > 1);
        drop(s);
        assert_eq!(p.owners(), 1);
        let empty = Query {
            constraints: vec![],
            outputs: vec![("fresh".into(), Var(variable + 100))],
        };
        let answer = run(&p, &empty);
        assert_eq!(answer.len(), 1);
        assert!(matches!(answer[0].outputs[0].1, chr_syntax::Term::Var(_)));
        assert_eq!(p.owners(), 1);
    }
}
