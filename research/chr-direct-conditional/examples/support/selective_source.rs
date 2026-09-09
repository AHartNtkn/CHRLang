#[allow(dead_code)]
#[path = "post_choice_source.rs"]
mod traversal;
use chr_syntax::{Answer, Query, Rule, atom, c, v};
pub const FAMILIES: [&str; 4] = ["common", "independent", "early", "dense"];
pub fn rules(family: &str, k: usize) -> Vec<Rule> {
    if family != "dense" {
        return traversal::rules(family, k);
    }
    vec![Rule::propagate(
        "pair",
        [c("p", [v(0)]), c("p", [v(1)])],
        c("seen", [v(0), v(1)]).into(),
    )]
}
pub fn query(family: &str, k: usize, d: usize, seed: usize) -> Query {
    if family != "dense" {
        return traversal::query(family, k, d, seed);
    }
    let n = if k == 0 { 1 } else { 6 };
    Query {
        constraints: (0..n)
            .map(|i| c("p", [atom(&format!("v{seed}_{i}"))]))
            .collect(),
        outputs: vec![],
    }
}
pub fn expected(family: &str, k: usize, d: usize, seed: usize) -> Vec<Answer> {
    if family != "dense" {
        return traversal::expected(family, k, d, seed);
    }
    let n = if k == 0 { 1 } else { 6 };
    let mut residual = (0..n)
        .map(|i| c("p", [atom(&format!("v{seed}_{i}"))]))
        .collect::<Vec<_>>();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                residual.push(c(
                    "seen",
                    [atom(&format!("v{seed}_{i}")), atom(&format!("v{seed}_{j}"))],
                ));
            }
        }
    }
    vec![Answer {
        outputs: vec![],
        residual,
    }]
}
