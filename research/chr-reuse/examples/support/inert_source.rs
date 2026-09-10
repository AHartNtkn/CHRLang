use chr_syntax::{Query, Rule, Var, and, atom, c, eq, or, t, v};
fn nat(n: usize) -> chr_syntax::Term {
    (0..n).fold(atom("z"), |n, _| t("s", [n]))
}
pub fn source(kind: usize, depth: usize, distinct: bool, offset: u64) -> (Vec<Rule>, Query) {
    use chr_syntax::Goal;
    let body = |tag: &str, fail: bool| {
        let marker = if kind == 2 { v(1) } else { atom(tag) };
        let mut parts = vec![c("caller", [marker]).into()];
        if fail {
            parts.push(Goal::Fail);
        } else {
            parts.push(c("work", [nat(depth), v(0), v(1)]).into());
            if kind == 2 {
                parts.push(c("bind", [v(1), atom(tag)]).into());
            }
        }
        and(parts)
    };
    let mut step = vec![c("work", [v(1), v(0), v(2)]).into()];
    if kind == 4 {
        step.extend([
            c("note", [t("s", [v(1)])]).into(),
            c("note", [t("s", [v(1)])]).into(),
        ]);
    }
    let mut rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(
                body("a", kind == 5),
                body(if distinct { "b" } else { "a" }, false),
            ),
        ),
        Rule::simplify("step", [c("work", [t("s", [v(1)]), v(0), v(2)])], and(step)),
        Rule::simplify(
            "finish",
            [c("work", [atom("z"), v(0), v(2)])],
            eq(
                v(0),
                if kind == 2 {
                    t("pair", [v(2), v(99), v(99)])
                } else {
                    t("pair", [v(99), v(99)])
                },
            ),
        ),
    ];
    if kind == 1 {
        rules.push(Rule::propagate(
            "read-caller",
            [c("caller", [v(3)])],
            c("seen", [v(3)]).into(),
        ));
    }
    if kind == 2 {
        rules.push(Rule::simplify(
            "late-bind",
            [c("bind", [v(0), v(1)])],
            eq(v(0), v(1)),
        ));
    }
    if kind == 3 {
        rules.push(Rule::simplify(
            "read-other-arity",
            [c("caller", [v(0), v(1)])],
            Goal::Fail,
        ));
    }
    (
        rules,
        Query {
            constraints: vec![c("start", [v(offset)])],
            outputs: vec![("out".into(), Var(offset))],
        },
    )
}
