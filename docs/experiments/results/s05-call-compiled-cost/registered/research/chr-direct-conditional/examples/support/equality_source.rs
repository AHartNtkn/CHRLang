#[path = "resumable_source.rs"]
mod prior;
use chr_syntax::{Answer, Query, Rule, and, atom, c, eq, t, v};
pub const FAMILIES: [&str; 6] = [
    "stable_history",
    "reset_history",
    "constructor",
    "dynamic",
    "delayed",
    "independent",
];
pub fn rules(family: &str) -> Vec<Rule> {
    match family {
        "constructor" => vec![Rule::propagate(
            "visit",
            [c("item", [v(0)])],
            and([
                c("mark", [v(0)]).into(),
                eq(t("box", [v(0)]), t("box", [v(0)])),
            ]),
        )],
        "dynamic" => vec![
            Rule::simplify("alias", [c("alias", [v(0), v(1)])], eq(v(0), v(1))),
            Rule::propagate(
                "visit",
                [c("item", [v(0), v(1)])],
                and([c("mark", [v(0)]).into(), eq(v(0), v(1))]),
            ),
        ],
        _ => {
            assert!(prior::FAMILIES.contains(&family));
            prior::rules(family)
        }
    }
}
pub fn query(family: &str, n: usize, seed: usize) -> Query {
    match family {
        "constructor" => prior::query("reset_history", n, seed),
        "dynamic" => {
            let mut constraints = vec![c("query", [atom(&format!("q{seed}"))])];
            for i in 0..16 * n {
                let x = v((100 + i) as u64);
                let a = atom(&format!("h{seed}_{i}"));
                constraints.push(c("alias", [x.clone(), a.clone()]));
                constraints.push(c("item", [x, a]));
            }
            Query {
                constraints,
                outputs: vec![],
            }
        }
        _ => prior::query(family, n, seed),
    }
}
pub fn expected(family: &str, n: usize, seed: usize) -> Vec<Answer> {
    match family {
        "constructor" => prior::expected("reset_history", n, seed),
        "dynamic" => {
            let mut residual = vec![c("query", [atom(&format!("q{seed}"))])];
            for i in 0..16 * n {
                let a = atom(&format!("h{seed}_{i}"));
                residual.push(c("item", [a.clone(), a.clone()]));
                residual.push(c("mark", [a]));
            }
            vec![Answer {
                outputs: vec![],
                residual,
            }]
        }
        _ => prior::expected(family, n, seed),
    }
}
