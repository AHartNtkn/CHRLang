use chr_syntax::{Goal, Query, Rule, Var, and, atom, c, eq, t, v};
fn nat(n: usize) -> chr_syntax::Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
pub fn rules(family: &str) -> Vec<Rule> {
    match family {
        "cold" => vec![Rule::simplify(
            "cold",
            [c("request", [t("f", [v(0)])]), c("partner", [v(0)])],
            Goal::True,
        )],
        "dense" => vec![Rule::simplify(
            "dense",
            [c("left", [v(0)]), c("right", [t("f", [v(0)])])],
            Goal::True,
        )],
        "three" => vec![
            Rule {
                name: "three".into(),
                kept: vec![c("left", [t("f", [v(0)])]), c("middle", [v(0)])],
                removed: vec![c("right", [v(0)])],
                guards: vec![],
                body: c("hit", [v(0), v(1), v(1)]).into(),
            },
            Rule::simplify(
                "go",
                [c("go", [v(0)])],
                and(vec![
                    eq(v(0), t("f", [atom("a")])),
                    c("middle", [atom("a")]).into(),
                ]),
            ),
        ],
        _ => vec![
            Rule::simplify(
                "consume",
                [
                    c(
                        "request",
                        [if family == "nested" {
                            t("pair", [v(0), v(0)])
                        } else {
                            t("f", [v(0)])
                        }],
                    ),
                    c("permit", []),
                ],
                c("hit", [v(0)]).into(),
            ),
            Rule::simplify(
                "tick",
                [c("tick", [t("s", [v(0)])])],
                c("tick", [v(0)]).into(),
            ),
            Rule::simplify("end", [c("tick", [atom("z")])], Goal::True),
            Rule::simplify("link", [c("link", [v(0), v(1)])], eq(v(0), v(1))),
            Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), t("f", [atom("a")]))),
        ],
    }
}
pub fn query(family: &str, n: usize, seed: usize) -> Query {
    let base = 100 + seed as u64 * 10000;
    let mut xs = vec![];
    match family {
        "cold" => {
            for i in 0..n {
                xs.push(c("request", [v(base + i as u64)]));
                xs.push(c("partner", [atom("a")]));
            }
        }
        "dense" => {
            for i in 0..n {
                xs.push(c("left", [v(base)]));
                xs.push(c("right", [v(base + 1000 + i as u64)]));
            }
        }
        "three" => {
            xs.extend([c("left", [v(base)]), c("go", [v(base)])]);
            for _ in 0..n {
                xs.push(c("right", [atom("a")]));
            }
        }
        _ => {
            xs.extend([c("permit", []), c("tick", [nat(n)])]);
            for i in 0..n {
                xs.push(c(
                    "request",
                    [if family == "nested" {
                        t("pair", [v(base + i as u64), v(base + 1000 + i as u64)])
                    } else {
                        v(base + i as u64)
                    }],
                ));
            }
            if family == "broad" {
                for i in 1..n {
                    xs.push(c("link", [v(base), v(base + i as u64)]));
                }
            }
            if family == "nested" {
                xs.push(c(
                    "link",
                    [v(base + n as u64 / 2), v(base + 1000 + n as u64 / 2)],
                ));
            } else {
                xs.push(c("bind", [v(base + n as u64 / 2)]));
            }
        }
    }
    xs.push(c("seed", [nat(seed)]));
    Query {
        constraints: xs,
        outputs: vec![("x".into(), Var(base))],
    }
}
