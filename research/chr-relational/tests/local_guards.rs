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
fn guarded(a: chr_syntax::Term, b: chr_syntax::Term) -> Rule {
    let mut r = Rule::simplify(
        "guarded",
        [c("request", [v(0), v(1), v(2)]), c("ticket", [])],
        eq(v(2), atom("won")),
    );
    r.guards.push(chr_syntax::Guard::Equal(a, b));
    r
}
#[test]
fn guards_wait_for_branch_local_equality_without_binding_unknowns() {
    let mut cases = 0;
    for form in 0..4 {
        for setup in 0..4 {
            for choice in [false, true] {
                for reverse in [false, true] {
                    let (a, b) = match form {
                        0 => (v(0), v(1)),
                        1 => (t("f", [v(0)]), t("f", [v(1)])),
                        2 => (v(0), t("f", [v(1)])),
                        _ => (t("pair", [v(0), v(0)]), t("pair", [v(1), v(1)])),
                    };
                    let left_value = if form == 2 {
                        t("f", [atom("a")])
                    } else {
                        atom("a")
                    };
                    let conflict = and([eq(v(0), left_value.clone()), eq(v(1), atom("b"))]);
                    let main = match setup {
                        0 => Goal::True,
                        1 => eq(v(0), v(1)),
                        2 => and([eq(v(0), left_value), eq(v(1), atom("a"))]),
                        _ => conflict.clone(),
                    };
                    let body = if choice {
                        if reverse {
                            or(conflict, main)
                        } else {
                            or(main, conflict)
                        }
                    } else {
                        main
                    };
                    let rules = vec![
                        guarded(a, b),
                        Rule::simplify("setup", [c("setup", [v(0), v(1)])], body),
                    ];
                    let q = Query {
                        constraints: vec![
                            c("request", [v(10), v(11), v(12)]),
                            c("ticket", []),
                            c("setup", [v(10), v(11)]),
                        ],
                        outputs: vec![
                            ("x".into(), Var(10)),
                            ("y".into(), Var(11)),
                            ("out".into(), Var(12)),
                        ],
                    };
                    check(&rules, &q);
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 64);
    println!("guard_source_configurations={cases}");
}
#[test]
fn guard_failure_continues_to_later_matching_tuples() {
    let rules = vec![guarded(v(0), v(1))];
    let q = Query {
        constraints: vec![
            c("request", [atom("a"), atom("b"), v(10)]),
            c("request", [atom("a"), atom("a"), v(11)]),
            c("ticket", []),
        ],
        outputs: vec![("later".into(), Var(11))],
    };
    let answers = check(&rules, &q);
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0].outputs, vec![("later".into(), atom("won"))]);
    assert_eq!(answers[0].residual.len(), 1);
    let q = Query {
        constraints: vec![c("request", [v(10), v(11), v(12)]), c("ticket", [])],
        outputs: vec![("x".into(), Var(10)), ("y".into(), Var(11))],
    };
    let answers = check(&rules, &q);
    let expected = Answer {
        outputs: vec![("x".into(), v(10)), ("y".into(), v(11))],
        residual: q.constraints.clone(),
    };
    ordered(&answers, &[expected]);
}
#[test]
fn guard_local_variables_are_fresh_and_conjunctions_are_required() {
    let mut fresh = Rule::simplify("fresh", [c("go", [v(0)])], c("fresh", [v(9), v(9)]).into());
    fresh.guards.push(chr_syntax::Guard::Equal(
        t("pair", [v(9), v(9)]),
        t("pair", [v(9), v(9)]),
    ));
    let q = Query {
        constraints: vec![c("go", [v(9)])],
        outputs: vec![("query".into(), Var(9))],
    };
    let expected = Answer {
        outputs: vec![("query".into(), v(50))],
        residual: vec![c("fresh", [v(51), v(51)])],
    };
    ordered(&check(&[fresh.clone()], &q), &[expected]);
    fresh.guards.push(chr_syntax::Guard::Equal(v(9), v(0)));
    ordered(
        &check(&[fresh], &q),
        &[Answer {
            outputs: vec![("query".into(), v(9))],
            residual: q.constraints.clone(),
        }],
    );
    let mut r = guarded(v(0), v(0));
    r.guards.push(chr_syntax::Guard::Equal(v(0), v(1)));
    let q = Query {
        constraints: vec![c("request", [atom("a"), atom("b"), v(2)]), c("ticket", [])],
        outputs: vec![],
    };
    ordered(
        &check(&[r], &q),
        &[Answer {
            outputs: vec![],
            residual: q.constraints.clone(),
        }],
    );
}
#[test]
fn completed_body_and_failed_siblings_control_guarded_consumption() {
    let mut consume = Rule::simplify(
        "consume",
        [c("request", [v(0), v(1), v(2)]), c("ticket", [])],
        eq(v(1), v(2)),
    );
    consume
        .guards
        .push(chr_syntax::Guard::Equal(v(0), t("f", [atom("a")])));
    let good = and([
        c("request", [t("f", [atom("a")]), v(1), atom("newer")]).into(),
        eq(v(0), t("f", [atom("a")])),
    ]);
    let bad = and([good.clone(), eq(v(8), t("cycle", [v(8)]))]);
    let rules = vec![
        consume,
        Rule::simplify(
            "trigger",
            [c("trigger", [v(0), v(1)]), c("ticket", [])],
            or(good, bad),
        ),
    ];
    let q = Query {
        constraints: vec![
            c("request", [v(10), v(11), atom("older")]),
            c("trigger", [v(10), v(11)]),
            c("ticket", []),
            c("ticket", []),
        ],
        outputs: vec![("winner".into(), Var(11))],
    };
    let answer = check(&rules, &q);
    assert_eq!(answer.len(), 1);
    assert_eq!(answer[0].outputs, vec![("winner".into(), atom("older"))]);
}
