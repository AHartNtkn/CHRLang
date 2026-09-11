use chr_syntax::{Query, Rule, Var, and, atom, c, eq, or, v};
pub fn rules(family: &str) -> Vec<Rule> {
    let effect = |color| {
        and(vec![
            eq(v(1), atom(color)),
            c("out", [v(0), v(1), v(10), v(10), v(11)]).into(),
        ])
    };
    let body = match family {
        "choices" | "shared" => or(effect("a"), effect("b")),
        "duplicates" => or(effect("a"), effect("a")),
        _ => effect("a"),
    };
    vec![
        Rule::simplify(
            "consume",
            [
                c("middle", [v(0), v(1)]),
                c("permit", [v(0)]),
                c("permit", [v(0)]),
            ],
            body,
        ),
        Rule::simplify(
            "produce",
            [c("seed", [v(10), v(11)]), c("fuel", [v(10)])],
            c("middle", [v(10), v(11)]).into(),
        ),
    ]
}
pub fn query(family: &str, n: usize, seed: usize) -> Query {
    let base = 100 + seed as u64 * 1000;
    let owner = atom(&format!("owner{seed}"));
    let mut xs = (0..n)
        .map(|i| {
            c(
                "seed",
                [
                    owner.clone(),
                    v(base + if family == "shared" { 0 } else { i as u64 }),
                ],
            )
        })
        .collect::<Vec<_>>();
    xs.extend((0..n + if family == "spare" { 2 } else { 0 }).map(|_| c("fuel", [owner.clone()])));
    xs.extend(
        (0..2 * n + if family == "spare" { 4 } else { 0 }).map(|_| c("permit", [owner.clone()])),
    );
    xs.push(c("query", [atom(&format!("q{seed}"))]));
    Query {
        constraints: xs,
        outputs: (0..n)
            .map(|i| {
                (
                    format!("x{i}"),
                    Var(base + if family == "shared" { 0 } else { i as u64 }),
                )
            })
            .collect(),
    }
}
