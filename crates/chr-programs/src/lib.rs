//! Reusable CHR programs, independent of every execution engine.
//!
//! Relational clauses are explicit disjunctions inside a single catchall rule.
//! Ordinary CHR rule competition is therefore never used as search.
use chr_syntax::{Goal, Rule, Term, and, atom, c, eq, or, t, v};

/// A finite unary natural number, using `z` and `s`.
pub fn unary(n: usize) -> Term {
    (0..n).fold(atom("z"), |term, _| t("s", [term]))
}
fn call(name: &str, args: impl Into<Vec<Term>>) -> Goal {
    c(name, args).into()
}
fn choices(arms: Vec<Goal>) -> Goal {
    arms.into_iter()
        .rev()
        .reduce(|tail, arm| or(arm, tail))
        .unwrap_or(Goal::Fail)
}
fn a(f: Term, x: Term) -> Term {
    t("a", [f, x])
}
fn p(term: Term, spine: Term) -> Term {
    t("p", [term, spine])
}
fn cons(head: Term, tail: Term) -> Term {
    t("cons", [head, tail])
}
fn fun(arg: Term, result: Term) -> Term {
    t("fun", [arg, result])
}

/// Addition in any direction; `sub(X,Y,Z)` delegates to `add(Y,Z,X)`.
pub fn arithmetic() -> Vec<Rule> {
    vec![
        Rule::simplify(
            "addition",
            [c("add", [v(0), v(1), v(2)])],
            or(
                and([eq(v(0), atom("z")), eq(v(1), v(2))]),
                and([
                    eq(v(0), t("s", [v(3)])),
                    eq(v(2), t("s", [v(4)])),
                    call("add", [v(3), v(1), v(4)]),
                ]),
            ),
        ),
        Rule::simplify(
            "subtraction",
            [c("sub", [v(0), v(1), v(2)])],
            call("add", [v(1), v(2), v(0)]),
        ),
    ]
}

/// Spine evaluator `eval(Input,Output)`, its `fold` relation, and `no_c`.
///
/// Inputs to `eval` are `p(Term,Spine)`. The evaluator can run backwards;
/// `no_c(Program)` excludes the test constants while preserving residual holes.
pub fn sk() -> Vec<Rule> {
    // 0/1 are the relation interface. Other variables are local to an application.
    let nil = atom("nil");
    let eval = choices(vec![
        and([
            eq(v(0), p(t("c", [v(2)]), v(3))),
            call("fold", [p(t("c", [v(2)]), v(3)), v(1)]),
        ]),
        and([eq(v(0), p(atom("k"), nil.clone())), eq(v(1), atom("k"))]),
        and([
            eq(v(0), p(atom("k"), cons(v(2), nil.clone()))),
            call("eval", [p(v(2), nil.clone()), v(3)]),
            eq(v(1), a(atom("k"), v(3))),
        ]),
        and([eq(v(0), p(atom("s"), nil.clone())), eq(v(1), atom("s"))]),
        and([
            eq(v(0), p(atom("s"), cons(v(2), nil.clone()))),
            call("eval", [p(v(2), nil.clone()), v(3)]),
            eq(v(1), a(atom("s"), v(3))),
        ]),
        and([
            eq(v(0), p(atom("s"), cons(v(2), cons(v(3), nil.clone())))),
            call("eval", [p(v(2), nil.clone()), v(4)]),
            call("eval", [p(v(3), nil.clone()), v(5)]),
            eq(v(1), a(a(atom("s"), v(4)), v(5))),
        ]),
        and([
            eq(v(0), p(a(v(2), v(3)), v(4))),
            call("eval", [p(v(2), cons(v(3), v(4))), v(1)]),
        ]),
        and([
            eq(v(0), p(atom("k"), cons(v(2), cons(v(3), v(4))))),
            call("eval", [p(v(2), v(4)), v(1)]),
        ]),
        and([
            eq(v(0), p(atom("s"), cons(v(2), cons(v(3), cons(v(4), v(5)))))),
            call(
                "eval",
                [p(v(2), cons(v(4), cons(a(v(3), v(4)), v(5)))), v(1)],
            ),
        ]),
    ]);
    let fold = or(
        and([eq(v(0), p(v(2), nil.clone())), eq(v(1), v(2))]),
        and([
            eq(v(0), p(v(2), cons(v(3), v(4)))),
            call("eval", [p(v(3), nil), v(5)]),
            call("fold", [p(a(v(2), v(5)), v(4)), v(1)]),
        ]),
    );
    vec![
        Rule::simplify("sk-evaluate", [c("eval", [v(0), v(1)])], eval),
        Rule::simplify("sk-fold", [c("fold", [v(0), v(1)])], fold),
        Rule::simplify("no-constant-k", [c("no_c", [atom("k")])], Goal::True),
        Rule::simplify("no-constant-s", [c("no_c", [atom("s")])], Goal::True),
        Rule::simplify(
            "no-constant-application",
            [c("no_c", [a(v(0), v(1))])],
            and([call("no_c", [v(0)]), call("no_c", [v(1)])]),
        ),
        Rule::simplify("reject-constant", [c("no_c", [t("c", [v(0)])])], Goal::Fail),
    ]
}

/// Simple types for SK terms. `infer(Program,Type)` also synthesizes inhabitants.
pub fn typing() -> Vec<Rule> {
    vec![Rule::simplify(
        "infer-sk",
        [c("infer", [v(0), v(1)])],
        choices(vec![
            and([eq(v(0), atom("k")), eq(v(1), fun(v(2), fun(v(3), v(2))))]),
            and([
                eq(v(0), atom("s")),
                eq(
                    v(1),
                    fun(
                        fun(v(2), fun(v(3), v(4))),
                        fun(fun(v(2), v(3)), fun(v(2), v(4))),
                    ),
                ),
            ]),
            and([
                eq(v(0), a(v(2), v(3))),
                call("infer", [v(3), v(4)]),
                call("infer", [v(2), fun(v(4), v(1))]),
            ]),
        ]),
    )]
}

/// Literal named-term relation from the pinned rwlog lambda notebook.
pub fn lambda() -> Vec<Rule> {
    fn app(x: Term, y: Term) -> Term {
        t("app", [x, y])
    }
    fn lam(x: Term, y: Term) -> Term {
        t("lam", [x, y])
    }
    let step = choices(vec![
        and([eq(v(0), app(lam(v(2), v(2)), v(3))), eq(v(1), v(3))]),
        and([
            eq(v(0), app(lam(v(2), v(3)), v(4))),
            call("neq", [v(2), v(3)]),
            eq(v(1), v(3)),
        ]),
        and([
            eq(v(0), app(lam(v(2), lam(v(2), v(3))), v(4))),
            eq(v(1), lam(v(2), v(3))),
        ]),
        and([
            eq(v(0), app(lam(v(2), lam(v(3), v(4))), v(5))),
            call("neq", [v(2), v(3)]),
            eq(v(1), lam(v(3), app(lam(v(2), v(4)), v(5)))),
        ]),
        and([
            eq(v(0), app(lam(v(2), app(v(3), v(4))), v(5))),
            eq(
                v(1),
                app(app(lam(v(2), v(3)), v(5)), app(lam(v(2), v(4)), v(5))),
            ),
        ]),
        and([
            eq(v(0), lam(v(2), v(3))),
            call("step", [v(3), v(4)]),
            eq(v(1), lam(v(2), v(4))),
        ]),
        and([
            eq(v(0), app(v(2), v(3))),
            call("step", [v(2), v(4)]),
            eq(v(1), app(v(4), v(3))),
        ]),
        and([
            eq(v(0), app(v(2), v(3))),
            call("norm", [v(2)]),
            call("step", [v(3), v(4)]),
            eq(v(1), app(v(2), v(4))),
        ]),
    ]);
    vec![
        Rule::simplify("neq-equal", [c("neq", [v(0), v(0)])], Goal::Fail),
        Rule::simplify(
            "neq-lam-left",
            [c("neq", [lam(v(0), v(1)), v(2)])],
            Goal::Fail,
        ),
        Rule::simplify(
            "neq-lam-right",
            [c("neq", [v(2), lam(v(0), v(1))])],
            Goal::Fail,
        ),
        Rule::simplify(
            "neq-app-left",
            [c("neq", [app(v(0), v(1)), v(2)])],
            Goal::Fail,
        ),
        Rule::simplify(
            "neq-app-right",
            [c("neq", [v(2), app(v(0), v(1))])],
            Goal::Fail,
        ),
        Rule::simplify("var-app", [c("var", [app(v(0), v(1))])], Goal::Fail),
        Rule::simplify("var-lam", [c("var", [lam(v(0), v(1))])], Goal::Fail),
        Rule::simplify(
            "norm-redex",
            [c("norm", [app(lam(v(0), v(1)), v(2))])],
            Goal::Fail,
        ),
        Rule::simplify(
            "norm-spine",
            [c("norm", [app(app(v(0), v(1)), v(2))])],
            and([call("norm", [app(v(0), v(1))]), call("norm", [v(2)])]),
        ),
        // Literal notebook rules consume and reintroduce var, including its occurrence identity.
        Rule::simplify(
            "norm-app-var",
            [c("norm", [app(v(0), v(1))]), c("var", [v(0)])],
            and([call("var", [v(0)]), call("norm", [v(1)])]),
        ),
        Rule::simplify(
            "norm-lam",
            [c("norm", [lam(v(0), v(1))])],
            call("norm", [v(1)]),
        ),
        Rule::simplify(
            "norm-var",
            [c("norm", [v(0)]), c("var", [v(0)])],
            call("var", [v(0)]),
        ),
        Rule::simplify("lambda-step", [c("step", [v(0), v(1)])], step),
        Rule::simplify(
            "lambda-equality",
            [c("lamEq", [v(0), v(1)])],
            or(
                and([eq(v(0), v(1)), call("norm", [v(0)])]),
                and([call("step", [v(0), v(2)]), call("lamEq", [v(2), v(1)])]),
            ),
        ),
    ]
}
