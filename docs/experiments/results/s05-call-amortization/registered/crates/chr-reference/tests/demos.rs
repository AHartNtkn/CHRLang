use chr_programs::{arithmetic, sk, typing, unary};
use chr_reference::Search;
use chr_syntax::{Answer, Query, Rule, Term, Var, atom, c, t, v};

fn run(
    rules: Vec<Rule>,
    constraints: Vec<chr_syntax::Constraint>,
    outputs: &[&str],
    steps: usize,
) -> (Vec<Answer>, bool) {
    let query = Query {
        constraints,
        outputs: outputs
            .iter()
            .enumerate()
            .map(|(i, n)| ((*n).into(), Var(i as u64)))
            .collect(),
    };
    let mut search = Search::new(rules, query).unwrap();
    let batch = search.advance(steps);
    (batch.answers, batch.exhausted)
}
fn app(f: Term, x: Term) -> Term {
    t("a", [f, x])
}
fn fun(a: Term, b: Term) -> Term {
    t("fun", [a, b])
}
fn eval(program: Term, expected: Term) {
    let (answers, exhausted) = run(
        sk(),
        vec![c("eval", [t("p", [program, atom("nil")]), v(0)])],
        &["result"],
        100_000,
    );
    assert!(exhausted, "ground evaluation exceeded budget");
    assert_eq!(
        answers,
        vec![Answer {
            outputs: vec![("result".into(), expected)],
            residual: vec![]
        }]
    );
}
#[test]
fn arithmetic_runs_in_each_direction_and_subtraction_reuses_addition() {
    for args in [
        [unary(2), unary(3), v(0)],
        [unary(2), v(0), unary(5)],
        [v(0), unary(3), unary(5)],
    ] {
        let expected = if args[2] == v(0) {
            unary(5)
        } else if args[1] == v(0) {
            unary(3)
        } else {
            unary(2)
        };
        let (answers, exhausted) = run(arithmetic(), vec![c("add", args)], &["n"], 10_000);
        assert!(exhausted);
        assert_eq!(
            answers,
            vec![Answer {
                outputs: vec![("n".into(), expected)],
                residual: vec![]
            }]
        );
    }
    let (answers, exhausted) = run(
        arithmetic(),
        vec![c("sub", [unary(5), unary(2), v(0)])],
        &["n"],
        10_000,
    );
    assert!(exhausted);
    assert_eq!(answers[0].outputs[0].1, unary(3));
}
#[test]
fn sk_identity_and_duplication_evaluate_forward() {
    let x = t("c", [atom("z")]);
    let y = t("c", [t("s", [atom("z")])]);
    let identity = app(app(atom("s"), atom("k")), atom("k"));
    eval(app(identity, x.clone()), x.clone());
    let duplication = app(app(atom("s"), atom("s")), app(atom("s"), atom("k")));
    eval(
        app(app(duplication, x.clone()), y.clone()),
        app(app(x, y.clone()), y),
    );
}
#[test]
fn typing_infers_k_s_and_identity() {
    for (program, expected) in [
        (atom("k"), fun(v(0), fun(v(1), v(0)))),
        (
            atom("s"),
            fun(
                fun(v(0), fun(v(1), v(2))),
                fun(fun(v(0), v(1)), fun(v(0), v(2))),
            ),
        ),
        (app(app(atom("s"), atom("k")), atom("k")), fun(v(0), v(0))),
    ] {
        let (answers, exhausted) = run(
            typing(),
            vec![c("infer", [program, v(0)])],
            &["type"],
            100_000,
        );
        assert!(exhausted);
        assert_eq!(
            answers,
            vec![Answer {
                outputs: vec![("type".into(), expected)],
                residual: vec![]
            }]
        );
    }
}
#[test]
fn type_directed_synthesis_can_resume() {
    let target = fun(atom("u"), fun(atom("v"), atom("u")));
    let mut search = Search::new(
        typing(),
        Query {
            constraints: vec![c("infer", [v(0), target])],
            outputs: vec![("program".into(), Var(0))],
        },
    )
    .unwrap();
    let mut found_k = false;
    for _ in 0..100 {
        let batch = search.advance(100);
        found_k |= batch
            .answers
            .iter()
            .any(|a| a.outputs[0].1 == atom("k") && a.residual.is_empty());
        if found_k {
            break;
        }
    }
    assert!(found_k, "K must be a synthesized inhabitant");
    assert!(
        search.pending_alternatives() > 0,
        "recursive synthesis remains open"
    );
    let wrapper = app(app(atom("k"), atom("k")), atom("k"));
    let mut found_wrapper = false;
    for _ in 0..100 {
        let continued = search.advance(100);
        found_wrapper |= continued
            .answers
            .iter()
            .any(|answer| answer.outputs[0].1 == wrapper && answer.residual.is_empty());
        if found_wrapper {
            break;
        }
    }
    assert!(
        found_wrapper,
        "resuming must enumerate another known inhabitant, K K K"
    );
}

#[test]
fn addition_enumerates_all_decompositions_of_a_known_sum() {
    let (answers, exhausted) = run(
        arithmetic(),
        vec![c("add", [v(0), v(1), unary(3)])],
        &["left", "right"],
        10_000,
    );
    assert!(exhausted);
    let mut pairs: Vec<_> = answers
        .into_iter()
        .map(|a| {
            assert!(a.residual.is_empty());
            (a.outputs[0].1.clone(), a.outputs[1].1.clone())
        })
        .collect();
    pairs.sort();
    let mut expected: Vec<_> = (0..=3).map(|left| (unary(left), unary(3 - left))).collect();
    expected.sort();
    assert_eq!(pairs, expected);
}

#[test]
fn no_c_keeps_unused_holes_and_rejects_hidden_constants() {
    let program = app(app(atom("k"), atom("s")), v(0));
    let (answers, exhausted) = run(
        sk(),
        vec![
            c("no_c", [program.clone()]),
            c("eval", [t("p", [program, atom("nil")]), v(1)]),
        ],
        &["hole", "result"],
        100_000,
    );
    assert!(exhausted);
    assert_eq!(
        answers,
        vec![Answer {
            outputs: vec![("hole".into(), v(0)), ("result".into(), atom("s"))],
            residual: vec![c("no_c", [v(0)])],
        }]
    );
    let program = app(app(atom("k"), atom("s")), t("c", [atom("z")]));
    let (answers, exhausted) = run(
        sk(),
        vec![
            c("no_c", [program.clone()]),
            c("eval", [t("p", [program, atom("nil")]), v(0)]),
        ],
        &["result"],
        100_000,
    );
    assert!(exhausted);
    assert!(answers.is_empty());
}

#[test]
fn sk_evaluator_runs_backwards_with_the_constant_restriction() {
    let mut search = Search::new(
        sk(),
        Query {
            constraints: vec![
                c("no_c", [v(0)]),
                c("eval", [t("p", [v(0), atom("nil")]), atom("k")]),
            ],
            outputs: vec![("program".into(), Var(0))],
        },
    )
    .unwrap();
    let mut found_k = false;
    for _ in 0..100 {
        let batch = search.advance(100);
        found_k |= batch
            .answers
            .iter()
            .any(|answer| answer.outputs[0].1 == atom("k") && answer.residual.is_empty());
        if found_k {
            break;
        }
    }
    assert!(
        found_k,
        "backward eval should synthesize K as a preimage of K"
    );
    assert!(search.pending_alternatives() > 0);
}
