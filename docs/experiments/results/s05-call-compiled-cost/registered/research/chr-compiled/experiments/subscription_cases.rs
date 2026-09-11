use super::source;
use chr_syntax::{Query, atom, c, t, v};
pub const FAMILIES: [&str; 9] = [
    "selective",
    "dense",
    "inactive",
    "churn-sparse",
    "churn-broad",
    "reopen",
    "unique",
    "binding",
    "consuming",
];
pub fn query(f: &str, n: usize, rounds: usize, seed: usize) -> Query {
    assert!(FAMILIES.contains(&f) && [2, 4].contains(&n) && [2, 8].contains(&rounds));
    let groups = if ["selective", "dense", "consuming"].contains(&f) {
        1
    } else {
        8
    };
    let mut rows = Vec::new();
    for g in 0..groups {
        for i in 0..n {
            let index = if f == "selective" { i } else { 0 };
            let a = if f == "binding" && g == 0 {
                v(50)
            } else {
                atom(&format!("a{g}_{index}"))
            };
            rows.push(c("left", [atom(&format!("k{g}")), t("f", [a])]));
            rows.push(c(
                "middle",
                [
                    atom(&format!("a{g}_{index}")),
                    t("g", [atom(&format!("b{g}_{index}"))]),
                ],
            ));
            rows.push(c(
                "right",
                [
                    atom(&format!("b{g}_{index}")),
                    t("h", [atom(&format!("value{g}_{index}"))]),
                ],
            ));
        }
    }
    // An inert source constraint distinguishes successive queries under reused preparation.
    rows.push(c("done", [atom(&format!("query{seed}"))]));
    let open = |g| {
        t(
            "open",
            [
                atom(&format!("k{g}")),
                atom(&format!("value{g}_0")),
                atom("d"),
            ],
        )
    };
    let ask = |r: usize| t("ask", [atom("d"), atom(&format!("r{r}"))]);
    let close = || t("close", [atom("d")]);
    let short = ["reopen", "unique"].contains(&f);
    let mut ops = Vec::new();
    if !short {
        ops.push(open(0));
    }
    if f == "binding" {
        ops.push(t("ask", [atom("d"), atom("before")]));
        ops.push(t("bind", [v(50), atom("a0_0")]));
    }
    for r in 0..rounds {
        if short {
            ops.push(open(if f == "unique" { r } else { 0 }));
        }
        if f.starts_with("churn") {
            for _ in 0..if f == "churn-broad" { n } else { 1 } {
                ops.push(t("remove_middle", [atom("a0_0"), t("g", [atom("b0_0")])]));
                ops.push(t("insert_middle", [atom("a0_0"), t("g", [atom("b0_0")])]));
            }
        }
        if f == "consuming" && r > 0 {
            for _ in 0..n {
                ops.push(t(
                    "insert_right",
                    [atom("b0_0"), t("h", [atom("value0_0")])],
                ));
            }
        }
        ops.push(ask(r));
        if short {
            ops.push(close());
        }
    }
    if !short {
        ops.push(close());
    }
    source::query(rows, ops, false)
}
pub fn receipts(f: &str, n: usize, rounds: usize) -> usize {
    rounds
        * if f == "selective" {
            1
        } else if f == "consuming" {
            n
        } else {
            n * n * n
        }
}
