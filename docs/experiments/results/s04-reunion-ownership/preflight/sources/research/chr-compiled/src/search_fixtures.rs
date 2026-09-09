//! Explicit-search programs and independent analytic observations for T030.
use chr_syntax::{Answer, Goal, Guard, Query, Rule, Var, and, atom, c, eq, or, t, v};
pub const FINITE_START: usize = 10;
pub fn finite_rules(order: usize) -> Vec<Rule> {
    let choose = Rule::simplify(
        "choose",
        [c("choose", [v(0)])],
        or(
            eq(v(0), atom("a0")),
            or(eq(v(0), atom("a1")), eq(v(0), atom("a2"))),
        ),
    );
    let given = Rule::simplify("given", [c("given", [v(0), v(1)])], eq(v(0), v(1)));
    let mut forbid = Rule::propagate(
        "forbid",
        [c("forbid", [v(0), v(1), v(2), v(3)])],
        Goal::Fail,
    );
    forbid.guards = vec![Guard::Equal(v(0), v(2)), Guard::Equal(v(1), v(3))];
    let all = [choose, given, forbid];
    [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ][order]
        .iter()
        .map(|&i| all[i].clone())
        .collect()
}
pub fn programs() -> Vec<Vec<Rule>> {
    let mut p = vec![
        vec![Rule::simplify(
            "choose",
            [c("start", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("a"))),
        )],
        vec![Rule::simplify(
            "choose",
            [c("choose", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        )],
        vec![Rule::simplify(
            "fresh",
            [c("start", [v(0)])],
            and(vec![
                eq(v(0), t("pair", [v(1), v(1)])),
                or(eq(v(1), atom("a")), eq(v(1), atom("b"))),
            ]),
        )],
        vec![Rule::simplify(
            "alias",
            [c("start", [v(0), v(1)])],
            or(eq(v(0), v(1)), Goal::True),
        )],
        vec![
            Rule::simplify(
                "choose",
                [c("start", [])],
                or(c("use", []).into(), c("use", []).into()),
            ),
            Rule::simplify("use", [c("use", []), c("token", [])], c("done", []).into()),
        ],
        vec![
            Rule::propagate("seen", [c("p", [v(0)])], c("seen", [v(0)]).into()),
            Rule::simplify(
                "choose",
                [c("start", [v(0)])],
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
            ),
        ],
        vec![Rule::simplify(
            "continue",
            [c("start", [v(0)])],
            and(vec![
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
                eq(v(0), atom("a")),
                c("done", []).into(),
            ]),
        )],
        vec![Rule::simplify(
            "failure",
            [c("start", [v(0)])],
            or(
                eq(v(0), atom("ok")),
                or(eq(v(1), t("f", [v(1)])), Goal::Fail),
            ),
        )],
        vec![
            Rule::simplify(
                "fork",
                [c("start", [v(0)])],
                or(c("loop", []).into(), eq(v(0), atom("done"))),
            ),
            Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
        ],
        vec![
            Rule::simplify("first", [c("start", [v(0)])], eq(v(0), atom("a"))),
            Rule::simplify("second", [c("start", [v(0)])], eq(v(0), atom("b"))),
        ],
    ];
    p.extend((0..6).map(finite_rules));
    p
}
pub fn case(id: usize) -> (Query, Vec<Answer>, bool) {
    let mut q = Query {
        constraints: vec![c("start", [v(0)])],
        outputs: vec![("x".into(), Var(0))],
    };
    let answer = |x| Answer {
        outputs: vec![("x".into(), x)],
        residual: vec![],
    };
    let expected = match id {
        0 => vec![answer(atom("a")), answer(atom("a"))],
        1 => {
            q.constraints = vec![c("choose", [v(0)]), c("choose", [v(1)])];
            q.outputs.push(("y".into(), Var(1)));
            ["a", "b"]
                .iter()
                .flat_map(|a| {
                    ["a", "b"].iter().map(move |b| Answer {
                        outputs: vec![("x".into(), atom(a)), ("y".into(), atom(b))],
                        residual: vec![],
                    })
                })
                .collect()
        }
        2 => vec![
            answer(t("pair", [atom("a"), atom("a")])),
            answer(t("pair", [atom("b"), atom("b")])),
        ],
        3 => {
            q.constraints = vec![c("start", [v(0), v(1)])];
            q.outputs.push(("y".into(), Var(1)));
            vec![
                Answer {
                    outputs: vec![("x".into(), v(0)), ("y".into(), v(0))],
                    residual: vec![],
                },
                Answer {
                    outputs: vec![("x".into(), v(0)), ("y".into(), v(1))],
                    residual: vec![],
                },
            ]
        }
        4 => {
            q.constraints = vec![c("start", []), c("token", [])];
            q.outputs.clear();
            vec![
                Answer {
                    outputs: vec![],
                    residual: vec![c("done", [])]
                };
                2
            ]
        }
        5 => {
            q.constraints = vec![c("p", [v(0)]), c("start", [v(0)])];
            ["a", "b"]
                .iter()
                .map(|x| Answer {
                    outputs: vec![("x".into(), atom(x))],
                    residual: vec![c("p", [atom(x)]), c("seen", [atom(x)])],
                })
                .collect()
        }
        6 => vec![Answer {
            outputs: vec![("x".into(), atom("a"))],
            residual: vec![c("done", [])],
        }],
        7 => vec![answer(atom("ok"))],
        8 => vec![answer(atom("done"))],
        9 => vec![answer(atom("a"))],
        _ => panic!("unknown search case"),
    };
    (q, expected, id != 8)
}
