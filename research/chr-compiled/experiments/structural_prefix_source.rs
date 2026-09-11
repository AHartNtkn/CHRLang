//! Shared structural-prefix source; rules depend on depth and kill presence only.
use chr_syntax::{Goal, Query, Rule, Var, atom, c, eq, t, v};
fn wrap(depth: usize, value: chr_syntax::Term) -> chr_syntax::Term {
    (0..depth).fold(value, |x, _| t("f", [x]))
}
pub fn rules(family: &str, depth: usize) -> Vec<Rule> {
    let mut rules = vec![];
    if family == "kill" {
        rules.push(Rule {
            name: "kill".into(),
            kept: vec![c("kill", [])],
            removed: vec![c("left", [v(0)])],
            guards: vec![],
            body: Goal::True,
        });
    }
    rules.push(Rule {
        name: "join".into(),
        kept: vec![
            c("left", [wrap(depth, v(0))]),
            c("middle", [wrap(depth, v(1))]),
        ],
        removed: vec![c("right", [v(0), v(1), v(2)])],
        guards: vec![],
        body: eq(v(2), atom("hit")),
    });
    rules
}
pub fn query(family: &str, n: usize, depth: usize, seed: usize) -> Query {
    let key = |i: usize| atom(&format!("k{seed}_{i}"));
    let base = 1000 + seed as u64 * 100;
    let mut facts = vec![];
    for i in 0..n {
        facts.push(c("left", [wrap(depth, key(i))]));
        if family != "miss" || depth > 0 {
            let middle = if family == "miss" {
                wrap(depth - 1, t("g", [key(i)]))
            } else {
                wrap(depth, key(i))
            };
            facts.push(c("middle", [middle]));
        }
        let target = if family == "keyed" { i } else { n - 1 };
        facts.push(c("right", [key(target), key(target), v(base + i as u64)]));
    }
    if family == "kill" {
        facts.push(c("kill", []));
    }
    Query {
        constraints: facts,
        outputs: (0..n)
            .map(|i| (format!("out{i}"), Var(base + i as u64)))
            .collect(),
    }
}
pub fn source(family: &str, n: usize, depth: usize, seed: usize) -> (Vec<Rule>, Query) {
    (rules(family, depth), query(family, n, depth, seed))
}

pub fn programs() -> Vec<Vec<Rule>> {
    [false, true]
        .into_iter()
        .flat_map(|kill| {
            [0, 8, 32]
                .into_iter()
                .map(move |depth| rules(if kill { "kill" } else { "sparse" }, depth))
        })
        .collect()
}
pub fn program_id(family: &str, depth: usize) -> usize {
    usize::from(family == "kill") * 3
        + [0, 8, 32]
            .iter()
            .position(|d| *d == depth)
            .expect("registered depth")
}
