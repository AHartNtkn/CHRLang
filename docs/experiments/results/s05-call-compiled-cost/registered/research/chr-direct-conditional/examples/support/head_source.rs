#[path = "broad_mixed_source.rs"]
mod broad;
use chr_syntax::{Answer, Query, Rule, Var, and, atom, c, eq, or, t, v};
pub const FAMILIES: [&str; 7] = [
    "history",
    "chain",
    "wide",
    "choices",
    "mixed_same",
    "independent",
    "delayed",
];
fn width(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 4,
        _ => 32,
    }
}
pub fn rules(family: &str, n: usize) -> Vec<Rule> {
    match family {
        "history" => vec![Rule::propagate(
            "visit",
            [c("p", [v(0)])],
            c("mark", [v(0)]).into(),
        )],
        "chain" => vec![
            Rule::simplify("step", [c("p", [t("s", [v(0)])])], c("p", [v(0)]).into()),
            Rule::simplify("end", [c("p", [atom("z")])], c("done", []).into()),
        ],
        "wide" => (0..width(n))
            .map(|i| {
                Rule::simplify(
                    &format!("r{i}"),
                    [c(&format!("p{i}"), [v(0)])],
                    c("done", [v(0)]).into(),
                )
            })
            .collect(),
        "choices" => vec![Rule::simplify(
            "choose",
            [c("start", [v(0)])],
            or(
                and([eq(v(0), atom("a")), c("done", [v(0)]).into()]),
                and([eq(v(0), atom("b")), c("done", [v(0)]).into()]),
            ),
        )],
        "mixed_same" => vec![
            Rule::simplify("unary", [c("p", [atom("a")])], c("done_a", []).into()),
            Rule::simplify(
                "join",
                [c("p", [atom("b")]), c("q", [])],
                c("done_b", []).into(),
            ),
        ],
        _ => broad::rules(family),
    }
}
pub fn query(family: &str, n: usize, seed: usize) -> Query {
    if matches!(family, "independent" | "delayed") {
        return broad::query(family, n, seed);
    }
    let mut constraints = vec![c("query", [atom(&format!("q{seed}"))])];
    let mut outputs = vec![];
    match family {
        "history" => {
            for i in 0..16 * n {
                constraints.push(c("p", [atom(&format!("v{seed}_{i}"))]));
            }
        }
        "chain" => {
            let mut x = atom("z");
            for _ in 0..16 * n {
                x = t("s", [x]);
            }
            constraints.push(c("p", [x]));
        }
        "wide" => {
            for i in 0..width(n) {
                constraints.push(c(&format!("p{i}"), [atom(&format!("v{seed}_{i}"))]));
            }
        }
        "choices" => {
            for i in 0..n {
                let id = (100 + seed * 10 + i) as u64;
                constraints.push(c("start", [v(id)]));
                outputs.push((format!("o{i}"), Var(id)));
            }
        }
        "mixed_same" => {
            for _ in 0..n {
                constraints.extend([c("p", [atom("a")]), c("p", [atom("b")]), c("q", [])]);
            }
        }
        _ => panic!("source"),
    }
    Query {
        constraints,
        outputs,
    }
}
pub fn expected(family: &str, n: usize, seed: usize) -> Vec<Answer> {
    if matches!(family, "independent" | "delayed") {
        return broad::expected(family, n, seed);
    }
    let count = if family == "choices" { 1usize << n } else { 1 };
    (0..count)
        .map(|bits| {
            let mut residual = vec![c("query", [atom(&format!("q{seed}"))])];
            let mut outputs = vec![];
            match family {
                "history" => {
                    for i in 0..16 * n {
                        let x = atom(&format!("v{seed}_{i}"));
                        residual.extend([c("p", [x.clone()]), c("mark", [x])]);
                    }
                }
                "chain" => residual.push(c("done", [])),
                "wide" => {
                    for i in 0..width(n) {
                        residual.push(c("done", [atom(&format!("v{seed}_{i}"))]));
                    }
                }
                "choices" => {
                    for i in 0..n {
                        let x = atom(if bits & (1 << i) == 0 { "a" } else { "b" });
                        outputs.push((format!("o{i}"), x.clone()));
                        residual.push(c("done", [x]));
                    }
                }
                "mixed_same" => {
                    for _ in 0..n {
                        residual.extend([c("done_a", []), c("done_b", [])]);
                    }
                }
                _ => panic!("source"),
            }
            Answer { outputs, residual }
        })
        .collect()
}
