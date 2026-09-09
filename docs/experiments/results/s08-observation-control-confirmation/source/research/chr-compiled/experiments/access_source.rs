//! Source witnesses for generated rollback, late aliases and guarded consumption.
use chr_syntax::{Goal, Guard, Query, Rule, Var, atom, c, eq, t, v};
pub fn rules(guarded: bool) -> Vec<Rule> {
    let mut join = Rule::simplify(
        "join",
        [
            c("p", [v(0)]),
            c("q", [v(0), t("f", [v(1)]), v(1)]),
            c("r", [v(1)]),
        ],
        c("out", [v(0), v(1), v(2), v(2)]).into(),
    );
    if guarded {
        join.guards.push(Guard::Equal(v(1), atom("yes")));
    }
    vec![
        join,
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
        Rule::simplify("fail", [c("fail", [])], Goal::Fail),
    ]
}
pub fn query(rotation: usize, guarded: bool, delayed: bool) -> Query {
    let key = if delayed { v(10) } else { atom("key") };
    let mut constraints = vec![
        c("p", [atom("decoy")]),
        c("p", [key.clone()]),
        c(
            "q",
            [atom("key"), t("f", [atom("wrong")]), atom("different")],
        ),
        c("q", [atom("key"), t("f", [atom("no")]), atom("no")]),
        c("q", [atom("key"), t("f", [atom("yes")]), atom("yes")]),
        c("r", [atom("no")]),
        c("r", [atom("yes")]),
    ];
    constraints.rotate_left(rotation % 7);
    if delayed {
        constraints.push(c("bind", [v(10), atom("key")]));
    }
    if !guarded {
        constraints.push(c("p", [atom("key")]));
    }
    Query {
        constraints,
        outputs: vec![("unused".into(), Var(99)), ("key".into(), Var(10))],
    }
}

/// A binding-irrelevant wake still changes Active queue competition.
pub fn wake_rules() -> Vec<Rule> {
    vec![
        Rule::simplify("left", [c("p", [v(0)]), c("q", [])], c("left", []).into()),
        Rule::simplify("right", [c("r", []), c("q", [])], c("right", []).into()),
        Rule::simplify(
            "seed",
            [c("seed", [v(0)])],
            chr_syntax::and([eq(v(0), atom("a")), c("r", []).into(), c("q", []).into()]),
        ),
    ]
}
pub fn wake_query() -> Query {
    Query {
        constraints: vec![c("p", [v(10)]), c("seed", [v(10)])],
        outputs: vec![("x".into(), Var(10))],
    }
}

pub fn payload_rules() -> Vec<Rule> {
    vec![
        Rule::propagate(
            "publish",
            [c("p", [v(0), v(1)]), c("q", [v(0)])],
            c("out", [v(1)]).into(),
        ),
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
    ]
}
pub fn payload_query(width: usize, late_key: bool) -> Query {
    let key = if late_key { v(10) } else { atom("k") };
    let payload = t(
        "payload",
        (0..width).map(|i| v(100 + i as u64)).collect::<Vec<_>>(),
    );
    let mut constraints = vec![c("p", [key, payload]), c("q", [atom("k")])];
    for i in 0..width {
        constraints.push(c("bind", [v(100 + i as u64), atom("value")]));
    }
    if late_key {
        constraints.push(c("bind", [v(10), atom("k")]));
    }
    Query {
        constraints,
        outputs: vec![("key".into(), Var(10))],
    }
}
