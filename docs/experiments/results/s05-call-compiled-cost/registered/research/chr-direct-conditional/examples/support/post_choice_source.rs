use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, and, atom, c, eq, or, t, v};
pub const FAMILIES: [&str; 3] = ["common", "independent", "early"];
fn depth(n: usize, mut base: Term) -> Term {
    for _ in 0..n {
        base = t("s", [base]);
    }
    base
}
fn independent(i: usize, k: usize, extra: usize) -> Goal {
    if i == k {
        return c("run", [depth(extra, v(0)), v(1)]).into();
    }
    or(
        and([eq(v(2 + i as u64), atom("a")), independent(i + 1, k, extra)]),
        and([
            eq(v(2 + i as u64), atom("b")),
            independent(i + 1, k, extra + 1),
        ]),
    )
}
pub fn rules(family: &str, k: usize) -> Vec<Rule> {
    assert!(FAMILIES.contains(&family));
    let body = if family == "independent" {
        independent(0, k, 0)
    } else {
        let mut gs = (0..k)
            .map(|i| {
                or(
                    eq(v(2 + i as u64), atom("a")),
                    if family == "early" && i == 0 {
                        Goal::Fail
                    } else {
                        eq(v(2 + i as u64), atom("b"))
                    },
                )
            })
            .collect::<Vec<_>>();
        gs.push(c("run", [v(0), v(1)]).into());
        and(gs)
    };
    vec![
        Rule::propagate("permit-history", [c("permit", [])], c("mark", []).into()),
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
            and([eq(v(0), t("done", [v(1)])), c("fresh", [v(1), v(1)]).into()]),
        ),
        Rule::simplify(
            "start",
            [c(
                "start",
                [
                    v(0),
                    v(1),
                    t(
                        "colors",
                        (0..k).map(|i| v(2 + i as u64)).collect::<Vec<_>>(),
                    ),
                ],
            )],
            body,
        ),
    ]
}
pub fn query(family: &str, k: usize, d: usize, seed: usize) -> Query {
    let base = 100 + seed as u64 * 1000;
    let mut constraints = vec![
        c("permit", []),
        c(
            "start",
            [
                depth(d, atom("z")),
                v(base),
                t(
                    "colors",
                    (0..k).map(|i| v(base + 1 + i as u64)).collect::<Vec<_>>(),
                ),
            ],
        ),
        c("query", [atom(&format!("q{seed}"))]),
    ];
    constraints.extend((0..d + if family == "independent" { k } else { 0 }).map(|_| c("fuel", [])));
    if seed % 2 == 1 {
        constraints.reverse();
    }
    let mut outputs = vec![("result".into(), Var(base))];
    outputs.extend((0..k).map(|i| (format!("x{i}"), Var(base + 1 + i as u64))));
    Query {
        constraints,
        outputs,
    }
}
pub fn expected(family: &str, k: usize, _d: usize, seed: usize) -> Vec<Answer> {
    let mut out = vec![];
    for bits in 0..1usize << k {
        if family == "early" && k > 0 && bits & 1 != 0 {
            continue;
        }
        let mut outputs = vec![("result".into(), t("done", [v(10000)]))];
        outputs.extend((0..k).map(|i| {
            (
                format!("x{i}"),
                atom(if bits & (1 << i) == 0 { "a" } else { "b" }),
            )
        }));
        let mut residual = vec![
            c("permit", []),
            c("mark", []),
            c("fresh", [v(10000), v(10000)]),
            c("query", [atom(&format!("q{seed}"))]),
        ];
        if family == "independent" {
            residual.extend((0..k - bits.count_ones() as usize).map(|_| c("fuel", [])));
        }
        out.push(Answer { outputs, residual });
    }
    out
}
pub fn explicit_steps(family: &str, k: usize, d: usize) -> usize {
    match family {
        "early" if k > 0 => (1usize << (k - 1)) * d,
        "independent" if k > 0 => (1usize << k) * d + k * (1usize << (k - 1)),
        _ => (1usize << k) * d,
    }
}
