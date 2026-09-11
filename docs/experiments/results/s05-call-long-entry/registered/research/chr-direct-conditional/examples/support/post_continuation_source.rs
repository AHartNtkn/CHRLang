use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, and, atom, c, eq, or, t, v};
pub const FAMILIES: [&str; 3] = ["common", "independent", "early"];
fn depth(n: usize) -> Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
pub fn rules(family: &str, k: usize, history: bool) -> Vec<Rule> {
    assert!(FAMILIES.contains(&family));
    fn branch(i: usize, k: usize, extra: usize, colors: Vec<Term>) -> Goal {
        if i == k {
            return and([
                c("run", [(0..extra).fold(v(0), |n, _| t("s", [n])), v(3)]).into(),
                eq(v(1), t("answer", [t("colors", colors), v(3)])),
            ]);
        }
        let mut left = colors.clone();
        left.push(atom("a"));
        let mut right = colors;
        right.push(atom("b"));
        or(
            branch(i + 1, k, extra, left),
            branch(i + 1, k, extra + 1, right),
        )
    }
    let body = if family == "independent" {
        branch(0, k, 0, vec![])
    } else {
        let mut goals = (0..k)
            .map(|i| {
                or(
                    eq(v(10 + i as u64), atom("a")),
                    if family == "early" && i == 0 {
                        Goal::Fail
                    } else {
                        eq(v(10 + i as u64), atom("b"))
                    },
                )
            })
            .collect::<Vec<_>>();
        goals.push(c("run", [v(0), v(3)]).into());
        goals.push(eq(
            v(1),
            t(
                "answer",
                [
                    t(
                        "colors",
                        (0..k).map(|i| v(10 + i as u64)).collect::<Vec<_>>(),
                    ),
                    v(3),
                ],
            ),
        ));
        and(goals)
    };
    let mut rules = vec![
        Rule::simplify("start", [c("start", [v(0), v(1)])], body),
        Rule::simplify(
            "emit",
            [c("emit", [v(0)])],
            and([c("token", [v(0)]).into(), eq(v(0), atom("a"))]),
        ),
        Rule::simplify(
            "consume",
            [c("consume", [v(1), v(2)]), c("token", [v(1)])],
            and([c("fresh", [v(3), v(3)]).into(), eq(v(2), t("done", [v(3)]))]),
        ),
        Rule {
            name: "step".into(),
            kept: vec![c("permit", [])],
            removed: vec![c("run", [t("s", [v(0)]), v(1)]), c("fuel", [])],
            guards: vec![],
            body: c("run", [v(0), v(1)]).into(),
        },
        Rule::simplify(
            "done",
            [c("run", [atom("z"), v(0)])],
            and([c("emit", [v(1)]).into(), c("consume", [v(1), v(0)]).into()]),
        ),
        // Establish the passive resource signature without enabling its consumer.
        Rule::simplify(
            "sink",
            [c("sink", [v(0)]), c("fresh", [v(1), v(2)])],
            eq(v(0), atom("ok")),
        ),
    ];
    if history {
        rules.push(Rule::propagate(
            "permit-history",
            [c("permit", [])],
            c("mark", []).into(),
        ));
    }
    rules
}
pub fn query(family: &str, k: usize, d: usize, seed: usize, reverse: bool) -> Query {
    let base = 100 + 1000 * seed as u64;
    let mut constraints = vec![
        c("start", [depth(d), v(base)]),
        c("permit", []),
        c("query", [atom(&format!("q{seed}"))]),
    ];
    constraints.extend((0..d + if family == "independent" { k } else { 0 }).map(|_| c("fuel", [])));
    if reverse {
        constraints.reverse();
    }
    Query {
        constraints,
        outputs: vec![("result".into(), Var(base))],
    }
}
pub fn expected(family: &str, k: usize, seed: usize, history: bool) -> Vec<Answer> {
    (0..1usize << k)
        .filter(|bits| !(family == "early" && k > 0 && bits & 1 != 0))
        .map(|bits| {
            let mut residual = vec![
                c("permit", []),
                c("query", [atom(&format!("q{seed}"))]),
                c("fresh", [v(9000), v(9000)]),
            ];
            if history {
                residual.push(c("mark", []));
            }
            if family == "independent" {
                residual.extend((0..k - bits.count_ones() as usize).map(|_| c("fuel", [])));
            }
            Answer {
                outputs: vec![(
                    "result".into(),
                    t(
                        "answer",
                        [
                            t(
                                "colors",
                                (0..k)
                                    .map(|i| atom(if bits & (1 << i) == 0 { "a" } else { "b" }))
                                    .collect::<Vec<_>>(),
                            ),
                            t("done", [v(9000)]),
                        ],
                    ),
                )],
                residual,
            }
        })
        .collect()
}
pub fn explicit_steps(family: &str, k: usize, d: usize) -> usize {
    if family == "early" && k > 0 {
        (1 << (k - 1)) * d
    } else if family == "independent" && k > 0 {
        (1 << k) * d + k * (1 << (k - 1))
    } else {
        (1 << k) * d
    }
}
