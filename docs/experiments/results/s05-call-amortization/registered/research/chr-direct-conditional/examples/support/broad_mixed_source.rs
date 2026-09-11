use chr_syntax::{Answer, Goal, Guard, Query, Rule, Var, and, atom, c, eq, or, t, v};
pub const FAMILIES: [&str; 7] = [
    "independent",
    "shared",
    "delayed",
    "early",
    "late",
    "history",
    "deep",
];
fn payload(family: &str, mut x: chr_syntax::Term) -> chr_syntax::Term {
    if family == "deep" {
        for _ in 0..16 {
            x = t("box", [x]);
        }
    }
    x
}
pub fn rules(family: &str) -> Vec<Rule> {
    assert!(FAMILIES.contains(&family));
    let arm = |color| {
        if family == "early" && color == "b" {
            return Goal::Fail;
        }
        let mut effects = if family == "delayed" {
            vec![
                c("ready", [v(0), v(1), v(2)]).into(),
                c("bind", [v(1), atom(color)]).into(),
            ]
        } else {
            vec![eq(v(1), atom(color)), c("ready", [v(0), v(1), v(2)]).into()]
        };
        if family == "late" && color == "b" {
            effects.push(c("abort", []).into());
        }
        and(effects)
    };
    vec![
        Rule::propagate(
            "permit-history",
            [c("permit", [v(0)])],
            c("mark", [v(0)]).into(),
        ),
        Rule::propagate(
            "item-history",
            [c("item", [v(0)])],
            c("seen", [v(0)]).into(),
        ),
        Rule {
            name: "consume".into(),
            kept: vec![c("permit", [v(0)])],
            removed: vec![
                c("ready", [v(0), v(1), v(2)]),
                c("ticket", [v(0), t("box", [v(3)])]),
            ],
            guards: vec![Guard::Equal(v(1), v(3))],
            body: and([
                eq(v(2), t("done", [v(1), payload(family, v(4))])),
                c("fresh", [v(0), v(4), v(4)]).into(),
            ]),
        },
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
        Rule::simplify("abort", [c("abort", [])], Goal::Fail),
        Rule::simplify(
            "start",
            [c("start", [v(0), v(1), v(2)])],
            or(arm("a"), arm("b")),
        ),
    ]
}
pub fn query(family: &str, n: usize, seed: usize) -> Query {
    let base = 100 + seed as u64 * 1000;
    let mut constraints = vec![c("query", [atom(&format!("q{seed}"))])];
    let mut outputs = vec![];
    for i in 0..n {
        let key = atom(&format!("k{seed}_{i}"));
        let x = base + if family == "shared" { 0 } else { 2 * i as u64 };
        let y = base + 2 * i as u64 + 1;
        constraints.extend([
            c("start", [key.clone(), v(x), v(y)]),
            c("permit", [key.clone()]),
            c("ticket", [key.clone(), t("box", [atom("a")])]),
            c("ticket", [key, t("box", [atom("b")])]),
        ]);
        outputs.extend([(format!("x{i}"), Var(x)), (format!("y{i}"), Var(y))]);
    }
    if family == "history" {
        for i in 0..16 * n {
            constraints.push(c("item", [atom(&format!("h{seed}_{i}"))]));
        }
    }
    if seed % 2 == 1 {
        constraints.reverse();
    }
    Query {
        constraints,
        outputs,
    }
}
pub fn expected(family: &str, n: usize, seed: usize) -> Vec<Answer> {
    let mut answers = vec![];
    for bits in 0..(1usize << n) {
        if matches!(family, "early" | "late") && bits != 0 {
            continue;
        }
        if family == "shared" && bits != 0 && bits != (1usize << n) - 1 {
            continue;
        }
        let mut residual = vec![c("query", [atom(&format!("q{seed}"))])];
        let mut outputs = vec![];
        for i in 0..n {
            let color = if bits & (1 << i) == 0 { "a" } else { "b" };
            let other = if color == "a" { "b" } else { "a" };
            let key = atom(&format!("k{seed}_{i}"));
            let fresh = v(10000 + i as u64);
            outputs.extend([
                (format!("x{i}"), atom(color)),
                (
                    format!("y{i}"),
                    t("done", [atom(color), payload(family, fresh.clone())]),
                ),
            ]);
            residual.extend([
                c("permit", [key.clone()]),
                c("mark", [key.clone()]),
                c("ticket", [key.clone(), t("box", [atom(other)])]),
                c("fresh", [key, fresh.clone(), fresh]),
            ]);
        }
        if family == "history" {
            for i in 0..16 * n {
                let x = atom(&format!("h{seed}_{i}"));
                residual.extend([c("item", [x.clone()]), c("seen", [x])]);
            }
        }
        answers.push(Answer { outputs, residual });
    }
    answers
}
