//! Qualified sources shared by semantic checks and lifecycle measurement.
use chr_syntax::{Goal, Query, Rule, Term, Var, atom, c, eq, t, v};
pub fn nest(depth: usize, mut leaf: Term) -> Term {
    for _ in 0..depth {
        leaf = t("f", [leaf]);
    }
    leaf
}
pub fn separated_source(
    depth: usize,
    possible: bool,
    shared: bool,
    outcome: &str,
    tokens: usize,
) -> (Vec<Rule>, Query) {
    let tail_leaf = if outcome == "clash" {
        atom("bad")
    } else {
        v(20)
    };
    let target = if shared {
        if possible { "b" } else { "no" }
    } else {
        "tail-end"
    };
    let mut older = Rule::simplify(
        "older",
        [c("use", [v(0), v(1), v(2)]), c("token", [])],
        if outcome == "fail" {
            Goal::Fail
        } else {
            eq(v(2), atom("older"))
        },
    );
    older.guards.extend([
        chr_syntax::Guard::Equal(v(0), atom("a")),
        chr_syntax::Guard::Equal(v(1), atom("b")),
    ]);
    let mut newer = Rule::simplify(
        "newer",
        [c("use", [v(0), v(1), v(2)]), c("token", [])],
        if outcome == "fail" {
            Goal::Fail
        } else {
            eq(v(2), atom("newer"))
        },
    );
    newer.guards.push(chr_syntax::Guard::Equal(v(0), atom("a")));
    let rules = vec![
        Rule::simplify(
            "bind",
            [c("bind", [v(0), v(1), v(2), v(3)])],
            chr_syntax::and([
                eq(v(0), atom("a")),
                eq(v(1), atom(if possible { "b" } else { "no" })),
                eq(v(2), v(3)),
            ]),
        ),
        older,
        newer,
    ];
    let mut constraints = vec![
        c(
            "bind",
            [
                v(10),
                v(11),
                nest(depth, tail_leaf.clone()),
                nest(depth, atom(target)),
            ],
        ),
        c(
            "use",
            [v(10), if shared { tail_leaf } else { v(11) }, v(12)],
        ),
    ];
    constraints.extend((0..tokens).map(|_| c("token", [])));
    (
        rules,
        Query {
            constraints,
            outputs: vec![("winner".into(), Var(12)), ("tail".into(), Var(20))],
        },
    )
}

/// Many distinct occurrences converge on one value; constructor facts may coalesce.
pub fn broad_source(
    width: usize,
    reverse_insert: bool,
    reverse_merge: bool,
    tokens: usize,
) -> (Vec<Rule>, Query) {
    let rules = vec![
        Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
        Rule::simplify(
            "consume",
            [c("edge", [v(0), v(0), v(1)])],
            c("receipt", [v(1)]).into(),
        ),
    ];
    let mut constraints = vec![];
    let mut order = (0..width).collect::<Vec<_>>();
    if reverse_insert {
        order.reverse();
    }
    for i in order {
        for _ in 0..tokens {
            constraints.push(c(
                "edge",
                [
                    v(i as u64),
                    v(i as u64),
                    t(&format!("f{}", i % 4), [v(i as u64), v(i as u64)]),
                ],
            ));
        }
    }
    let mut order = (0..width).collect::<Vec<_>>();
    if reverse_merge {
        order.reverse();
    }
    for i in order {
        constraints.push(c("bind", [v(i as u64)]));
    }
    (
        rules,
        Query {
            constraints,
            outputs: vec![],
        },
    )
}
