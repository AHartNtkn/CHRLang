use chr_reference::Search;
use chr_syntax::{Answer, Constraint, Query, Term, Var, atom, c, t, v};
fn run(constraints: Vec<Constraint>, outputs: Vec<(String, Var)>) -> Vec<Answer> {
    let mut s = Search::new(
        chr_programs::lambda(),
        Query {
            constraints,
            outputs,
        },
    )
    .unwrap();
    let batch = s.advance(20_000);
    assert!(
        batch.exhausted,
        "finite literal step/constraint case must terminate"
    );
    batch.answers
}
fn step(input: Term) -> Vec<Answer> {
    run(vec![c("step", [input, v(0)])], vec![("out".into(), Var(0))])
}
#[test]
fn identity_beta_returns_the_argument() {
    assert_eq!(
        step(t("app", [t("lam", [atom("x"), atom("x")]), atom("a")])),
        vec![Answer {
            outputs: vec![("out".into(), atom("a"))],
            residual: vec![]
        }]
    );
}
#[test]
fn distinct_names_remain_an_explicit_residual_restriction() {
    assert_eq!(
        step(t("app", [t("lam", [atom("x"), atom("y")]), atom("a")])),
        vec![Answer {
            outputs: vec![("out".into(), atom("y"))],
            residual: vec![c("neq", [atom("x"), atom("y")])]
        }]
    );
}
#[test]
fn norm_requires_the_actual_companion_occurrence() {
    assert_eq!(
        run(vec![c("norm", [atom("x")])], vec![]),
        vec![Answer {
            outputs: vec![],
            residual: vec![c("norm", [atom("x")])]
        }]
    );
    assert_eq!(
        run(vec![c("norm", [atom("x")]), c("var", [atom("x")])], vec![]),
        vec![Answer {
            outputs: vec![],
            residual: vec![c("var", [atom("x")])]
        }]
    );
}
#[test]
fn invalid_names_and_equal_disequalities_fail() {
    for constraint in [
        c("var", [t("app", [atom("x"), atom("y")])]),
        c("neq", [atom("x"), atom("x")]),
    ] {
        assert!(run(vec![constraint], vec![]).is_empty());
    }
}
#[test]
fn literal_substitution_does_not_invent_capture_avoidance() {
    let input = t(
        "app",
        [
            t("lam", [atom("a"), t("lam", [atom("b"), atom("a")])]),
            atom("b"),
        ],
    );
    assert_eq!(
        step(input),
        vec![Answer {
            outputs: vec![(
                "out".into(),
                t(
                    "lam",
                    [
                        atom("b"),
                        t("app", [t("lam", [atom("a"), atom("a")]), atom("b")])
                    ]
                )
            )],
            residual: vec![c("neq", [atom("a"), atom("b")])]
        }]
    );
}

#[test]
fn norm_reintroduction_gives_propagation_a_new_occurrence() {
    use chr_syntax::Rule;
    let mut rules = vec![Rule::propagate(
        "observe-var",
        [c("var", [v(0)])],
        c("seen", [v(0)]).into(),
    )];
    rules.extend(chr_programs::lambda());
    let mut search = Search::new(
        rules,
        Query {
            constraints: vec![c("var", [atom("x")]), c("norm", [atom("x")])],
            outputs: vec![],
        },
    )
    .unwrap();
    let batch = search.advance(1000);
    assert!(batch.exhausted);
    assert_eq!(
        batch.answers,
        vec![Answer {
            outputs: vec![],
            residual: vec![
                c("seen", [atom("x")]),
                c("seen", [atom("x")]),
                c("var", [atom("x")])
            ]
        }]
    );
}
