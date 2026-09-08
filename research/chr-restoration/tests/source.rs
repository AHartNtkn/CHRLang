#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_restoration::{Mode, Prepared, Step};
use chr_syntax::{Answer, Goal, Guard, Query, Rule, Var, and, atom, c, eq, or, t, v};
fn modes() -> [Mode; 6] {
    [
        Mode::Copy,
        Mode::Trail,
        Mode::Replay,
        Mode::Checkpoint(1.try_into().unwrap()),
        Mode::Checkpoint(4.try_into().unwrap()),
        Mode::Checkpoint(16.try_into().unwrap()),
    ]
}
fn run(p: &std::sync::Arc<Prepared>, q: &Query, mode: Mode) -> Vec<Answer> {
    let mut e = p.start(q, mode).unwrap();
    let mut answers = Vec::new();
    for _ in 0..100000 {
        match e.advance() {
            Step::Answer(a) => answers.push(a),
            Step::Exhausted => return answers,
            Step::Progress => (),
        }
    }
    panic!("finite restoration gate cutoff");
}
fn control(rules: &[Rule], q: &Query, access: chr_compiled::Access) -> Vec<Answer> {
    let prepared = chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap();
    let mut engine = prepared
        .start_search(q.clone(), chr_compiled::Policy::Global, access)
        .unwrap();
    let mut answers = Vec::new();
    for _ in 0..100000 {
        match engine.tick() {
            chr_compiled::SearchEvent::Complete(mut branch) => {
                answers.push(branch.engine.observe().unwrap())
            }
            chr_compiled::SearchEvent::Exhausted => return answers,
            _ => (),
        }
    }
    panic!("dedicated source control cutoff");
}
fn compare(rules: &[Rule], q: &Query, expected: &[Answer]) {
    let prepared = Prepared::new(rules).unwrap();
    for mode in modes() {
        oracle::same_raw(run(&prepared, q, mode), expected.to_vec());
    }
    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
        oracle::same_raw(control(rules, q, access), expected.to_vec());
    }
}
#[test]
fn complete_source_modes_preserve_rollback_and_multiplicity() {
    let mut cases = Vec::new();
    for body in [
        or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        or(
            eq(
                t("pair", [atom("a"), v(0)]),
                t("pair", [atom("b"), atom("c")]),
            ),
            Goal::True,
        ),
        or(eq(v(0), t("f", [v(0)])), eq(v(0), atom("safe"))),
        or(Goal::True, Goal::True),
        and(vec![
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
            c("tail", [v(0), v(1), v(1)]).into(),
        ]),
    ] {
        cases.push((
            vec![Rule::simplify("start", [c("start", [v(0)])], body)],
            vec![c("start", [v(50)])],
        ));
    }
    cases.push((
        vec![
            Rule::propagate("watch", [c("token", [v(0)])], c("seen", [v(0)]).into()),
            Rule::simplify(
                "start",
                [c("start", [v(0)])],
                or(
                    c("token", [atom("a")]).into(),
                    c("token", [atom("b")]).into(),
                ),
            ),
        ],
        vec![c("start", [v(50)])],
    ));
    cases.push((
        vec![
            Rule::simplify(
                "take",
                [c("token", [v(0)]), c("task", [v(1)])],
                eq(v(1), v(0)),
            ),
            Rule::simplify(
                "start",
                [c("start", [v(0)])],
                or(c("task", [v(0)]).into(), c("task", [v(0)]).into()),
            ),
        ],
        vec![c("token", [atom("a")]), c("start", [v(50)])],
    ));
    let mut guard = Rule::simplify(
        "guard",
        [c("pair", [v(0), v(1)])],
        c("equal", [v(0)]).into(),
    );
    guard.guards = vec![Guard::Equal(v(0), v(1))];
    cases.push((
        vec![
            guard,
            Rule::simplify(
                "start",
                [c("start", [v(0)])],
                or(
                    and(vec![
                        eq(v(0), atom("a")),
                        c("pair", [v(0), atom("a")]).into(),
                    ]),
                    c("pair", [v(0), atom("a")]).into(),
                ),
            ),
        ],
        vec![c("start", [v(50)])],
    ));
    cases.push((
        vec![
            Rule {
                name: "keep".into(),
                kept: vec![c("token", [v(0)])],
                removed: vec![c("task", [v(1)])],
                guards: vec![],
                body: and(vec![eq(v(1), v(0)), c("fresh", [v(2), v(2)]).into()]),
            },
            Rule::simplify(
                "start",
                [c("start", [v(0)])],
                or(
                    and(vec![c("task", [v(0)]).into(), or(Goal::True, Goal::Fail)]),
                    c("task", [v(0)]).into(),
                ),
            ),
        ],
        vec![c("token", [v(51)]), c("start", [v(50)])],
    ));
    assert_eq!(cases.len(), 9);
    for (rules, rows) in cases {
        for extra in [false, true] {
            let mut rows = rows.clone();
            if extra {
                rows.push(c("passive", [v(50), v(51), v(51)]));
            }
            let q = Query {
                constraints: rows,
                outputs: vec![("x".into(), Var(50)), ("y".into(), Var(51))],
            };
            let expected = oracle::run(&rules, &q, 100000);
            compare(&rules, &q, &expected);
            let reversed: Vec<_> = rules.iter().rev().cloned().collect();
            oracle::same_raw(oracle::run(&reversed, &q, 100000), expected.clone());
            compare(&reversed, &q, &expected);
        }
    }
}
#[test]
fn nested_choices_restore_pending_tails_and_failed_branches() {
    for depth in 0..=4 {
        for duplicate in [false, true] {
            let mut choice = eq(v(0), atom("leaf"));
            for _ in 0..depth {
                choice = or(
                    choice.clone(),
                    if duplicate {
                        choice.clone()
                    } else {
                        Goal::Fail
                    },
                );
            }
            let rules = vec![Rule::simplify(
                "start",
                [c("start", [v(0)])],
                and(vec![choice, c("after", [v(0)]).into()]),
            )];
            let q = Query {
                constraints: vec![c("start", [v(50)])],
                outputs: vec![("x".into(), Var(50))],
            };
            let expected = vec![
                Answer {
                    outputs: vec![("x".into(), atom("leaf"))],
                    residual: vec![c("after", [atom("leaf")])]
                };
                if duplicate { 1 << depth } else { 1 }
            ];
            oracle::same_raw(oracle::run(&rules, &q, 100000), expected.clone());
            compare(&rules, &q, &expected);
        }
    }
}
#[test]
fn fair_finite_sibling_and_cancelled_query_reuse() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(c("loop", []).into(), eq(v(0), atom("finite"))),
        ),
        Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
    ];
    let p = Prepared::new(&rules).unwrap();
    let q = Query {
        constraints: vec![c("start", [v(50)])],
        outputs: vec![("x".into(), Var(50))],
    };
    for mode in modes() {
        let mut e = p.start(&q, mode).unwrap();
        let mut answers = Vec::new();
        for _ in 0..100 {
            match e.advance() {
                Step::Answer(a) => answers.push(a),
                Step::Progress => (),
                Step::Exhausted => panic!("ongoing branch exhausted"),
            }
        }
        assert_eq!(
            answers,
            vec![Answer {
                outputs: vec![("x".into(), atom("finite"))],
                residual: vec![]
            }]
        );
        drop(e);
        let q = Query {
            constraints: vec![c("passive", [v(99)])],
            outputs: vec![("fresh".into(), Var(99))],
        };
        assert_eq!(
            run(&p, &q, mode),
            vec![Answer {
                outputs: vec![("fresh".into(), v(99))],
                residual: q.constraints.clone()
            }]
        );
    }
}

#[test]
fn unsupported_rules_and_exhausted_initial_identity_are_explicit_errors() {
    assert!(Prepared::new(&[Rule::simplify("empty", [], Goal::True)]).is_err());
    let p = Prepared::new(&[]).unwrap();
    let q = Query {
        constraints: vec![],
        outputs: vec![("limit".into(), Var(u64::MAX))],
    };
    for mode in modes() {
        assert!(p.start(&q, mode).is_err());
    }
}
