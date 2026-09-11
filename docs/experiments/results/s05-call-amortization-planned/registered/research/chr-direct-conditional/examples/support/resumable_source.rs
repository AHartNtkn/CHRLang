#[path = "broad_mixed_source.rs"]
mod broad;
use chr_syntax::{Answer, Query, Rule, and, atom, c, eq, v};
pub const FAMILIES: [&str; 10] = [
    "independent",
    "shared",
    "delayed",
    "early",
    "late",
    "history",
    "deep",
    "stable_history",
    "reset_history",
    "dense",
];
pub fn rules(family: &str) -> Vec<Rule> {
    match family {
        "stable_history" | "reset_history" => vec![Rule::propagate(
            "visit",
            [c("item", [v(0)])],
            if family == "reset_history" {
                and([c("mark", [v(0)]).into(), eq(v(0), v(0))])
            } else {
                c("mark", [v(0)]).into()
            },
        )],
        "dense" => vec![Rule::propagate(
            "pair",
            [c("p", [v(0)]), c("p", [v(0)])],
            c("seen", [v(0), v(0)]).into(),
        )],
        _ => broad::rules(family),
    }
}
pub fn query(family: &str, n: usize, seed: usize) -> Query {
    if broad::FAMILIES.contains(&family) {
        return broad::query(family, n, seed);
    }
    let mut constraints = vec![c("query", [atom(&format!("q{seed}"))])];
    if family == "dense" {
        constraints.extend(vec![c("p", [atom(&format!("x{seed}"))]); 2 * n]);
    } else {
        for i in 0..16 * n {
            constraints.push(c("item", [atom(&format!("h{seed}_{i}"))]));
        }
    }
    Query {
        constraints,
        outputs: vec![],
    }
}
pub fn expected(family: &str, n: usize, seed: usize) -> Vec<Answer> {
    if broad::FAMILIES.contains(&family) {
        return broad::expected(family, n, seed);
    }
    let mut residual = vec![c("query", [atom(&format!("q{seed}"))])];
    if family == "dense" {
        let x = atom(&format!("x{seed}"));
        residual.extend(vec![c("p", [x.clone()]); 2 * n]);
        for i in 0..2 * n {
            for j in 0..2 * n {
                if i != j {
                    residual.push(c("seen", [x.clone(), x.clone()]));
                }
            }
        }
    } else {
        for i in 0..16 * n {
            let x = atom(&format!("h{seed}_{i}"));
            residual.push(c("item", [x.clone()]));
            residual.push(c("mark", [x]));
        }
    }
    vec![Answer {
        outputs: vec![],
        residual,
    }]
}
