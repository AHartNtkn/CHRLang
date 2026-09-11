use chr_syntax::{Answer, Query, Rule, and, atom, c, eq, v};
pub const FAMILIES: [&str; 1] = ["dynamic_alias"];
pub fn rules(_: &str) -> Vec<Rule> {
    vec![
        Rule::simplify("alias", [c("alias", [v(0), v(1)])], eq(v(0), v(1))),
        Rule::propagate(
            "visit",
            [c("item", [v(0), v(1)])],
            and([c("mark", [v(0)]).into(), eq(v(0), v(1))]),
        ),
    ]
}
pub fn query(_: &str, n: usize, seed: usize) -> Query {
    let mut constraints = vec![c("query", [atom(&format!("q{seed}"))])];
    for i in 0..16 * n {
        let x = v((100 + 2 * i) as u64);
        let y = v((101 + 2 * i) as u64);
        constraints.push(c("alias", [x.clone(), y.clone()]));
        constraints.push(c("item", [x, y]));
    }
    Query {
        constraints,
        outputs: vec![],
    }
}
pub fn expected(_: &str, n: usize, seed: usize) -> Vec<Answer> {
    let mut residual = vec![c("query", [atom(&format!("q{seed}"))])];
    for i in 0..16 * n {
        let x = v((900 + i) as u64);
        residual.push(c("item", [x.clone(), x.clone()]));
        residual.push(c("mark", [x]));
    }
    vec![Answer {
        outputs: vec![],
        residual,
    }]
}
