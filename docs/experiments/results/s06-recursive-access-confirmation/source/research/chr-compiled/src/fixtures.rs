//! Source programs and analytic finite observations, independent of engine code.
use chr_syntax::{Answer, Constraint, Goal, Guard, Query, Rule, Var, and, atom, c, eq, t, v};

pub const ANALYTIC_PROGRAMS: usize = 12;

pub fn programs() -> Vec<Vec<Rule>> {
    let build = vec![
        Rule::simplify(
            "zero",
            [c("build", [atom("z"), v(0)])],
            eq(v(0), atom("nil")),
        ),
        Rule::simplify(
            "step",
            [c("build", [t("s", [v(0)]), v(1)])],
            and(vec![
                eq(v(1), t("cons", [atom("item"), v(2)])),
                c("build", [v(0), v(2)]).into(),
            ]),
        ),
    ];
    let reach = vec![
        Rule::propagate(
            "reach",
            [c("reach", [v(0)]), c("edge", [v(0), v(1)])],
            c("reach", [v(1)]).into(),
        ),
        Rule {
            name: "job".into(),
            kept: vec![c("reach", [v(0)])],
            removed: vec![c("job", [v(0), v(1)])],
            guards: vec![],
            body: and(vec![eq(v(1), t("seen", [v(0)])), c("done", [v(0)]).into()]),
        },
    ];
    let mut delayed = reach.clone();
    delayed[1].guards.push(Guard::Equal(v(0), v(0)));
    delayed.push(Rule::simplify(
        "bind",
        [c("bind", [v(0), v(1)])],
        eq(v(0), v(1)),
    ));
    let mut guard = Rule::simplify("guard", [c("p", [v(0), v(1)])], c("hit", []).into());
    guard.guards.push(Guard::Equal(v(0), v(1)));
    let wake = vec![
        guard,
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
    ];
    let ordered = vec![
        Rule::propagate(
            "ordered",
            [c("p", [v(0)]), c("p", [v(0)])],
            c("hit", []).into(),
        ),
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
    ];
    let middle = vec![
        Rule::simplify(
            "three",
            [c("left", [v(0)]), c("middle", [v(0)]), c("right", [v(0)])],
            c("hit", []).into(),
        ),
        Rule::simplify("seed", [c("seed", [v(0)])], c("middle", [v(0)]).into()),
    ];
    let mut fail = vec![Rule::simplify(
        "fail",
        [c("start", [v(0)])],
        and(vec![eq(v(0), t("f", [v(0)])), c("wrong", []).into()]),
    )];
    fail.push(Rule::simplify("skip", [c("skip", [])], Goal::True));
    fail.push(Rule::simplify(
        "late",
        [c("late", [v(0)])],
        and(vec![eq(v(0), atom("a")), Goal::Fail]),
    ));
    let fresh = vec![Rule::simplify(
        "fresh",
        [c("start", [v(0)])],
        and(vec![eq(v(0), t("box", [v(1)])), c("hole", [v(1)]).into()]),
    )];
    let arrival = vec![
        Rule::propagate(
            "pairs",
            [c("p", [v(0)]), c("q", [v(1)])],
            c("hit", [v(0), v(1)]).into(),
        ),
        Rule::simplify("spawn", [c("seed", [])], c("q", [atom("a")]).into()),
    ];
    let self_wake = vec![
        Rule::propagate(
            "alias",
            [c("p", [v(0)]), c("trigger", [atom("a")])],
            eq(v(0), atom("a")),
        ),
        Rule::propagate("hit", [c("p", [atom("a")])], c("hit", []).into()),
    ];
    let suspended = vec![Rule::simplify(
        "known",
        [c("p", [atom("a")])],
        c("hit", []).into(),
    )];
    let nested = vec![
        Rule::simplify(
            "known",
            [c("p", [t("f", [atom("a")])])],
            c("hit", []).into(),
        ),
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
    ];
    let conflict = vec![
        Rule::simplify("join", [c("p", []), c("q", [])], c("hit", []).into()),
        Rule::simplify("single", [c("p", [])], c("single", []).into()),
        Rule::simplify("seed", [c("seed", [])], c("q", []).into()),
    ];
    let collisions = vec![
        Rule::propagate(
            "keyed",
            [c("p", [v(0)]), c("q", [v(0)])],
            c("hit", [v(0), v(0)]).into(),
        ),
        Rule::simplify("spawn", [c("seed", [])], c("q", [atom("a")]).into()),
    ];
    let anchored = vec![Rule::simplify(
        "anchored",
        [c("left", [v(0)]), c("right", [t("f", [v(0)]), v(0)])],
        c("hit", []).into(),
    )];
    vec![
        build, reach, delayed, wake, ordered, middle, fail, fresh, arrival, self_wake, suspended,
        nested, conflict, collisions, anchored,
    ]
}
pub fn unary(n: usize) -> chr_syntax::Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
fn node(n: usize) -> chr_syntax::Term {
    t("node", [unary(n)])
}
pub struct Case {
    pub query: Query,
    pub expected: Option<Answer>,
    pub failed: bool,
}
fn success(
    input: Vec<Constraint>,
    outputs: Vec<(String, Var)>,
    values: Vec<chr_syntax::Term>,
    residual: Vec<Constraint>,
) -> Case {
    Case {
        query: Query {
            constraints: input,
            outputs: outputs.clone(),
        },
        expected: Some(Answer {
            outputs: outputs
                .into_iter()
                .zip(values)
                .map(|((n, _), v)| (n, v))
                .collect(),
            residual,
        }),
        failed: false,
    }
}
pub fn case(id: usize, n: usize) -> Case {
    match id {
        0 => success(
            vec![c("build", [unary(n), v(0)])],
            vec![("out".into(), Var(0))],
            vec![(0..n).fold(atom("nil"), |tail, _| t("cons", [atom("item"), tail]))],
            vec![],
        ),
        1 | 2 => {
            let mut input = vec![c("reach", [node(0)])];
            let mut residual = vec![];
            let mut outputs = vec![];
            let mut values = vec![];
            for i in 0..n {
                let edge = c("edge", [node(i), node(i + 1)]);
                residual.push(edge.clone());
                if id == 2 {
                    input.push(c("edge", [t("node", [v((n + 1 + i) as u64)]), node(i + 1)]));
                } else {
                    input.push(edge);
                }
            }
            for i in 0..=n {
                input.push(c("job", [node(i), v(i as u64)]));
                outputs.push((format!("o{i}"), Var(i as u64)));
                values.push(t("seen", [node(i)]));
                residual.push(c("reach", [node(i)]));
                residual.push(c("done", [node(i)]));
            }
            if id == 2 {
                for i in 0..n {
                    input.push(c("bind", [v((n + 1 + i) as u64), unary(i)]));
                }
            }
            success(input, outputs, values, residual)
        }
        3 => success(
            vec![
                c("p", [t("f", [v(0)]), t("f", [v(1)])]),
                c("bind", [v(0), v(1)]),
            ],
            vec![("x".into(), Var(0)), ("y".into(), Var(1))],
            vec![v(0), v(0)],
            vec![c("hit", [])],
        ),
        4 => success(
            vec![c("p", [v(0)]), c("p", [v(1)]), c("bind", [v(0), v(1)])],
            vec![("x".into(), Var(0)), ("y".into(), Var(1))],
            vec![v(0), v(0)],
            vec![c("p", [v(0)]), c("p", [v(0)]), c("hit", []), c("hit", [])],
        ),
        5 => success(
            vec![
                c("left", [atom("a")]),
                c("right", [atom("a")]),
                c("seed", [atom("a")]),
            ],
            vec![],
            vec![],
            vec![c("hit", [])],
        ),
        6 => Case {
            query: Query {
                constraints: vec![c("start", [v(0)])],
                outputs: vec![("x".into(), Var(0))],
            },
            expected: None,
            failed: true,
        },
        7 => success(
            vec![c("start", [v(0)]), c("start", [v(1)])],
            vec![("x".into(), Var(0)), ("y".into(), Var(1))],
            vec![t("box", [v(2)]), t("box", [v(3)])],
            vec![c("hole", [v(2)]), c("hole", [v(3)])],
        ),
        8 => success(
            vec![c("p", [atom("a")]), c("q", [atom("a")]), c("seed", [])],
            vec![],
            vec![],
            vec![
                c("p", [atom("a")]),
                c("q", [atom("a")]),
                c("q", [atom("a")]),
                c("hit", [atom("a"), atom("a")]),
                c("hit", [atom("a"), atom("a")]),
            ],
        ),
        9 => success(
            vec![c("p", [v(0)]), c("trigger", [atom("a")])],
            vec![("x".into(), Var(0))],
            vec![atom("a")],
            vec![c("p", [atom("a")]), c("trigger", [atom("a")]), c("hit", [])],
        ),
        10 => success(
            vec![c("p", [v(0)])],
            vec![("x".into(), Var(0))],
            vec![v(0)],
            vec![c("p", [v(0)])],
        ),
        11 => success(
            vec![
                c("p", [v(0)]),
                c("bind", [v(0), t("f", [v(1)])]),
                c("bind", [v(1), atom("a")]),
            ],
            vec![("x".into(), Var(0)), ("y".into(), Var(1))],
            vec![t("f", [atom("a")]), atom("a")],
            vec![c("hit", [])],
        ),
        _ => panic!("unknown fixture"),
    }
}

/// Depth-zero keys with fixed-width names. Rules are program 1 (ground) or 2 (delayed).
pub fn flat_chain_case(n: usize, delayed: bool) -> Case {
    let key = |i: usize| atom(&format!("k{i:016x}"));
    let mut input = vec![c("reach", [key(0)])];
    let mut residual = vec![];
    let mut outputs = vec![];
    let mut values = vec![];
    for i in 0..n {
        residual.push(c("edge", [key(i), key(i + 1)]));
        input.push(c(
            "edge",
            [
                if delayed {
                    v((n + 1 + i) as u64)
                } else {
                    key(i)
                },
                key(i + 1),
            ],
        ));
    }
    for i in 0..=n {
        input.push(c("job", [key(i), v(i as u64)]));
        outputs.push((format!("o{i}"), Var(i as u64)));
        values.push(t("seen", [key(i)]));
        residual.push(c("reach", [key(i)]));
        residual.push(c("done", [key(i)]));
    }
    if delayed {
        for i in 0..n {
            input.push(c("bind", [v((n + 1 + i) as u64), key(i)]));
        }
    }
    success(input, outputs, values, residual)
}
/// Program 13: n initial equal-key q occurrences plus one new occurrence.
pub fn collision_case(n: usize) -> Case {
    let mut input = vec![c("p", [atom("a")])];
    for _ in 0..n {
        input.push(c("q", [atom("a")]));
    }
    input.push(c("seed", []));
    let mut residual = vec![c("p", [atom("a")])];
    for _ in 0..=n {
        residual.push(c("q", [atom("a")]));
        residual.push(c("hit", [atom("a"), atom("a")]));
    }
    success(input, vec![], vec![], residual)
}
/// Program 11: every initially unknown nested key becomes f(b), which never enables p(f(a)).
pub fn repair_case(n: usize) -> Case {
    let mut input = vec![];
    let mut outputs = vec![];
    let mut values = vec![];
    let mut residual = vec![];
    for i in 0..n {
        input.push(c("p", [t("f", [v(i as u64)])]));
        outputs.push((format!("v{i}"), Var(i as u64)));
        values.push(atom("b"));
        residual.push(c("p", [t("f", [atom("b")])]));
    }
    for i in 0..n {
        input.push(c("bind", [v(i as u64), atom("b")]));
    }
    success(input, outputs, values, residual)
}
