use chr_syntax::{Goal, Query, Rule, Var, atom, c, eq, or, t, v};
pub fn nat(n: usize) -> chr_syntax::Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
pub fn source(
    a: usize,
    b: usize,
    reverse: bool,
    mode: usize,
    offset: u64,
) -> (Vec<Rule>, usize, Query) {
    let (rs, count) = program(reverse, mode);
    (rs, count, input(a, b, offset))
}
pub fn program(reverse: bool, mode: usize) -> (Vec<Rule>, usize) {
    let mut rs = vec![Rule::simplify(
        "recurse",
        [c("wait", [t("s", [v(0)]), v(1), v(2)])],
        c("wait", [v(0), v(1), v(2)]).into(),
    )];
    if mode == 1 {
        rs.push(Rule::simplify(
            "fail_b",
            [c("wait", [atom("z"), atom("b"), v(0)])],
            Goal::Fail,
        ));
    }
    rs.push(Rule::simplify(
        "base",
        [c("wait", [atom("z"), v(0), v(1)])],
        eq(
            v(1),
            if mode == 2 {
                t("pair", [v(3), v(3)])
            } else {
                v(0)
            },
        ),
    ));
    if mode == 3 {
        let binding = rs.last().unwrap().body.clone();
        rs.last_mut().unwrap().body = Goal::And(vec![binding, c("tail", [nat(4)]).into()]);
        rs.push(Rule::simplify(
            "tail_step",
            [c("tail", [t("s", [v(0)])])],
            c("tail", [v(0)]).into(),
        ));
        rs.push(Rule::simplify(
            "tail_done",
            [c("tail", [atom("z")])],
            Goal::True,
        ));
    }
    let count = rs.len();
    let mut branches = [
        Goal::from(c("wait", [v(0), atom("a"), v(2)])),
        Goal::from(c("wait", [v(1), atom("b"), v(2)])),
    ];
    if reverse {
        branches.swap(0, 1);
    }
    rs.extend([
        Rule::propagate("history", [c("watch", [v(0)])], c("seen", [v(0)]).into()),
        Rule::simplify(
            "launch",
            [c("start", [v(0), v(1), v(2)])],
            or(branches[0].clone(), branches[1].clone()),
        ),
        Rule::simplify(
            "a_wins",
            [c("watch", [atom("a")]), c("token", [])],
            c("winner", [atom("a")]).into(),
        ),
        Rule::simplify(
            "other_wins",
            [c("token", [])],
            c("winner", [atom("other")]).into(),
        ),
    ]);
    (rs, count)
}

pub fn input(a: usize, b: usize, offset: u64) -> Query {
    Query {
        constraints: vec![
            c("start", [nat(a), nat(b), v(offset)]),
            c("watch", [v(offset)]),
            c("token", []),
        ],
        outputs: vec![("out".into(), Var(offset))],
    }
}
