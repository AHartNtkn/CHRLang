#[path = "broad_mixed_source.rs"]
mod broad;
use chr_syntax::{Answer, Query, Rule, and, atom, c, or, v};
pub const FAMILIES: [&str; 7] = [
    "incompatible",
    "correlated",
    "independent_suffix",
    "dense_suffix",
    "independent",
    "delayed",
    "early",
];
fn suffix(n: usize) -> usize {
    if n == 3 { 8 } else { n }
}
pub fn rules(family: &str) -> Vec<Rule> {
    if broad::FAMILIES.contains(&family) {
        return broad::rules(family);
    }
    let post = |p: &str, x: &str| c(p, [atom(x)]).into();
    let body = match family {
        "incompatible" => or(post("p", "a"), post("q", "b")),
        "correlated" => or(
            and([post("p", "a"), post("q", "a")]),
            and([post("p", "b"), post("q", "b")]),
        ),
        "independent_suffix" => and([
            or(post("p", "a"), post("p", "b")),
            or(post("q", "a"), post("q", "b")),
        ]),
        "dense_suffix" => and([post("p", "a"), post("q", "a")]),
        _ => panic!("unknown source"),
    };
    vec![
        Rule::simplify("start", [c("start", [])], body),
        Rule::propagate(
            "join",
            [c("p", [v(0)]), c("q", [v(1)]), c("r", [v(2)])],
            c("seen", [v(0), v(1), v(2)]).into(),
        ),
    ]
}
pub fn query(family: &str, n: usize, seed: usize) -> Query {
    if broad::FAMILIES.contains(&family) {
        return broad::query(family, n, seed);
    }
    let mut constraints = vec![c("query", [atom(&format!("q{seed}"))])];
    constraints.extend((0..suffix(n)).map(|i| c("r", [atom(&format!("r{seed}_{i}"))])));
    constraints.push(c("start", []));
    Query {
        constraints,
        outputs: vec![],
    }
}
pub fn expected(family: &str, n: usize, seed: usize) -> Vec<Answer> {
    if broad::FAMILIES.contains(&family) {
        return broad::expected(family, n, seed);
    }
    let pairs = match family {
        "incompatible" => vec![(Some("a"), None), (None, Some("b"))],
        "correlated" => vec![(Some("a"), Some("a")), (Some("b"), Some("b"))],
        "independent_suffix" => vec![
            (Some("a"), Some("a")),
            (Some("a"), Some("b")),
            (Some("b"), Some("a")),
            (Some("b"), Some("b")),
        ],
        _ => vec![(Some("a"), Some("a"))],
    };
    pairs
        .into_iter()
        .map(|(x, y)| {
            let mut residual = vec![c("query", [atom(&format!("q{seed}"))])];
            for i in 0..suffix(n) {
                residual.push(c("r", [atom(&format!("r{seed}_{i}"))]));
            }
            if let Some(x) = x {
                residual.push(c("p", [atom(x)]));
            }
            if let Some(y) = y {
                residual.push(c("q", [atom(y)]));
            }
            if let (Some(x), Some(y)) = (x, y) {
                for i in 0..suffix(n) {
                    residual.push(c("seen", [atom(x), atom(y), atom(&format!("r{seed}_{i}"))]));
                }
            }
            Answer {
                outputs: vec![],
                residual,
            }
        })
        .collect()
}
