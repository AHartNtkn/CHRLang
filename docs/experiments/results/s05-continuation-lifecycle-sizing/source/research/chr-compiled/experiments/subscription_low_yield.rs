use super::source;
use chr_syntax::{Query, atom, c, t};
pub const FAMILIES: [&str; 3] = ["low-stable", "low-reopen", "low-churn"];
pub fn query(f: &str, n: usize, rounds: usize, seed: usize) -> Query {
    assert!(FAMILIES.contains(&f) && [4, 8, 16].contains(&n) && [2, 16, 64].contains(&rounds));
    let mut rows = Vec::new();
    for i in 0..n {
        rows.push(c("left", [atom("k"), t("f", [atom(&format!("a{i}"))])]));
        rows.push(c(
            "right",
            [atom(&format!("b{i}")), t("h", [atom("value")])],
        ));
        for j in 0..n {
            rows.push(c(
                "middle",
                [
                    atom(&format!("a{i}")),
                    t("g", [atom(&format!("dead_b{j}"))]),
                ],
            ));
            rows.push(c(
                "middle",
                [
                    atom(&format!("dead_a{i}")),
                    t("g", [atom(&format!("b{j}"))]),
                ],
            ));
        }
    }
    rows.push(c("middle", [atom("a0"), t("g", [atom("b0")])]));
    rows.push(c("done", [atom(&format!("query{seed}"))]));
    let open = || t("open", [atom("k"), atom("value"), atom("d")]);
    let close = || t("close", [atom("d")]);
    let mut ops = Vec::new();
    if f != "low-reopen" {
        ops.push(open());
    }
    for r in 0..rounds {
        if f == "low-reopen" {
            ops.push(open());
        }
        if f == "low-churn" {
            ops.push(t("remove_middle", [atom("a0"), t("g", [atom("b0")])]));
            ops.push(t("insert_middle", [atom("a0"), t("g", [atom("b0")])]));
        }
        ops.push(t("ask", [atom("d"), atom(&format!("r{r}"))]));
        if f == "low-reopen" {
            ops.push(close());
        }
    }
    if f != "low-reopen" {
        ops.push(close());
    }
    source::query(rows, ops, false)
}
