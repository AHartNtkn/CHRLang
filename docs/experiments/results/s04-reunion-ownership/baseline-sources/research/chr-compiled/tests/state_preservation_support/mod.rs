//! State-preservation fixture: real source-built payload, cheap right-spine choices.
//!
//! Alternatives counts leaves (>=1); one leaf is the no-choice control. Each
//! right arm retains all payload. Left arms equate a candidate key with a query
//! wanted key; a free wanted variable admits all keys, z only the final key.
//! Failed equations terminate before cleanup. Successful
//! arms consume n payload occurrences through indexed two-head source rules.
//! All-success therefore includes n+1 cleanup applications per leaf, whereas
//! mostly-failing service pays that cleanup once. This is not a wide balanced tree.
use chr_syntax::{Answer, Query, Rule, Term, Var, and, atom, c, eq, or, t, v};
pub fn unary(n: usize) -> Term {
    (0..n).fold(atom("z"), |rest, _| t("s", [rest]))
}
pub fn rules() -> Vec<Rule> {
    vec![
        Rule::simplify(
            "build-step",
            [c("build", [t("s", [v(0)]), v(1), v(2), v(3), v(4)])],
            and(vec![
                eq(v(5), t("cell", [t("s", [v(0)]), v(3), v(3)])),
                c("payload", [t("s", [v(0)]), v(5)]).into(),
                c("build", [v(0), v(1), v(2), v(3), v(4)]).into(),
            ]),
        ),
        Rule::simplify(
            "build-done",
            [c("build", [atom("z"), v(1), v(2), v(3), v(4)])],
            c("choose", [v(2), v(1), v(3), v(4)]).into(),
        ),
        Rule::simplify(
            "choose",
            [c("choose", [t("s", [v(0)]), v(1), v(2), v(3)])],
            or(
                and(vec![
                    eq(v(3), t("s", [v(0)])),
                    c("clean", [v(1), v(2)]).into(),
                ]),
                c("choose", [v(0), v(1), v(2), v(3)]).into(),
            ),
        ),
        Rule::simplify(
            "choose-last",
            [c("choose", [atom("z"), v(1), v(2), v(3)])],
            and(vec![eq(v(3), atom("z")), c("clean", [v(1), v(2)]).into()]),
        ),
        Rule::simplify(
            "consume-payload",
            [
                c("clean", [t("s", [v(0)]), v(1)]),
                c(
                    "payload",
                    [t("s", [v(0)]), t("cell", [t("s", [v(0)]), v(1), v(1)])],
                ),
            ],
            c("clean", [v(0), v(1)]).into(),
        ),
        Rule::simplify(
            "finish",
            [c("clean", [atom("z"), v(0)])],
            and(vec![
                c("done", [v(0), v(0)]).into(),
                c("done", [v(0), v(0)]).into(),
            ]),
        ),
    ]
}
pub fn query(state_size: usize, alternatives: usize, all_success: bool) -> Query {
    assert!(
        alternatives >= 1,
        "alternatives counts leaves; at least one required"
    );
    Query {
        constraints: vec![c(
            "build",
            [
                unary(state_size),
                unary(state_size),
                unary(alternatives - 1),
                v(0),
                if all_success { v(2) } else { atom("z") },
            ],
        )],
        outputs: vec![
            ("x".into(), Var(0)),
            ("alias".into(), Var(0)),
            ("unused".into(), Var(1)),
        ],
    }
}
pub fn expected_raw(alternatives: usize, all_success: bool) -> usize {
    if all_success { alternatives } else { 1 }
}
/// Complete structural oracle; accepts arbitrary joint renaming, no allocations.
pub fn answer_matches(answer: &Answer) -> bool {
    let [
        (x, Term::Var(a)),
        (alias, Term::Var(b)),
        (unused, Term::Var(u)),
    ] = answer.outputs.as_slice()
    else {
        return false;
    };
    x == "x"
        && alias == "alias"
        && unused == "unused"
        && a == b
        && a != u
        && answer.residual.len() == 2
        && answer
            .residual
            .iter()
            .all(|c| c.name == "done" && c.args.as_slice() == [Term::Var(*a), Term::Var(*a)])
}
