#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_relational::contextual_execute::{Prepared, Step};
use chr_syntax::{Answer, Goal, Guard, Query, Rule, Var, and, atom, c, eq, or, t, v};
fn collect(p: &std::sync::Arc<Prepared>, q: &Query) -> Vec<Answer> {
    let mut e = p.start(q);
    let mut answers = vec![];
    for _ in 0..10000 {
        match e.advance() {
            Step::Answer(a) => answers.push(a),
            Step::Exhausted => {
                let mut control = chr_relational::execute::Prepared::new(p.rules())
                    .unwrap()
                    .start(q);
                let mut other = vec![];
                for _ in 0..10000 {
                    match control.advance() {
                        chr_relational::execute::Step::Answer(a) => other.push(a),
                        chr_relational::execute::Step::Exhausted => {
                            oracle::same_raw(answers.clone(), other);
                            return answers;
                        }
                        chr_relational::execute::Step::Progress => (),
                    }
                }
                panic!("relational control bound");
            }
            Step::Progress => (),
        }
    }
    panic!("finite bound");
}
#[test]
fn common_fresh_prefix_and_consuming_siblings_have_complete_answers() {
    for duplicate in [false, true] {
        for fail in [false, true] {
            let branch = |name: &str| {
                and([
                    eq(v(1), atom(name)),
                    c("take", [v(1), v(2)]).into(),
                    eq(v(3), v(2)),
                    if fail && name == "b" {
                        Goal::Fail
                    } else {
                        Goal::True
                    },
                ])
            };
            let rules = vec![
                Rule::simplify(
                    "start",
                    [c("start", [v(0), v(3)])],
                    and([
                        eq(v(0), t("f", [v(1)])),
                        or(branch("a"), branch(if duplicate { "a" } else { "b" })),
                    ]),
                ),
                Rule {
                    name: "take".into(),
                    kept: vec![c("permit", [])],
                    removed: vec![c("take", [v(0), v(1)]), c("ticket", [v(0)])],
                    guards: vec![Guard::Equal(v(0), v(0))],
                    body: and([eq(v(1), t("done", [v(0)])), c("fresh", [v(2), v(2)]).into()]),
                },
            ];
            let p = Prepared::new(&rules).unwrap();
            for extra in [false, true] {
                let mut constraints = vec![
                    c("start", [v(10), v(11)]),
                    c("permit", []),
                    c("ticket", [atom("a")]),
                    c("ticket", [atom("b")]),
                ];
                if extra {
                    constraints.push(c("ticket", [atom("a")]));
                }
                let q = Query {
                    constraints,
                    outputs: vec![("x".into(), Var(10)), ("result".into(), Var(11))],
                };
                let expected = oracle::run(&rules, &q, 10000);
                assert_eq!(expected.len(), if fail && !duplicate { 1 } else { 2 });
                let actual = collect(&p, &q);
                oracle::same_raw(actual, expected);
            }
        }
    }
}
#[test]
fn propagation_history_and_late_guard_enablement_are_context_local() {
    let mut watch = Rule::propagate(
        "watch",
        [c("watch", [v(0), v(1)])],
        c("seen", [v(0)]).into(),
    );
    watch.guards.push(Guard::Equal(v(0), v(1)));
    let rules = vec![
        watch,
        Rule::simplify(
            "start",
            [c("start", [v(0), v(1)])],
            or(
                eq(v(0), v(1)),
                and([eq(v(0), atom("a")), eq(v(1), atom("b"))]),
            ),
        ),
    ];
    let q = Query {
        constraints: vec![c("watch", [v(10), v(11)]), c("start", [v(10), v(11)])],
        outputs: vec![("x".into(), Var(10)), ("y".into(), Var(11))],
    };
    oracle::same_raw(
        collect(&Prepared::new(&rules).unwrap(), &q),
        oracle::run(&rules, &q, 10000),
    );
}
#[test]
fn finite_sibling_publishes_while_other_source_keeps_running() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(
                c("loop", []).into(),
                and([eq(v(0), atom("a")), c("done", []).into()]),
            ),
        ),
        Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
    ];
    let q = Query {
        constraints: vec![c("start", [v(10)])],
        outputs: vec![("x".into(), Var(10))],
    };
    let mut e = Prepared::new(&rules).unwrap().start(&q);
    let mut published = false;
    for _ in 0..500 {
        match e.advance() {
            Step::Answer(a) => {
                assert_eq!(
                    a,
                    Answer {
                        outputs: vec![("x".into(), atom("a"))],
                        residual: vec![c("done", [])]
                    }
                );
                published = true;
                break;
            }
            Step::Exhausted => panic!("ongoing branch lost"),
            Step::Progress => (),
        }
    }
    assert!(published);
    for _ in 0..32 {
        assert!(matches!(e.advance(), Step::Progress));
    }
    drop(e);
}
