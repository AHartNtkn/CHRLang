use chr_syntax::{Goal, Rule, Term, and, atom, c, eq, or, t, v};
#[allow(dead_code)]
#[path = "factor_cases.rs"]
mod originals;

pub fn ids() -> Vec<String> {
    [
        "one-zero",
        "one-work",
        "two-work",
        "owner-product",
        "asym-first",
        "asym-last",
        "duplicate-eight",
        "refute-loop",
        "stream-prefix",
        "mixed-add-infer",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
fn unary(n: usize) -> Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
pub fn case(id: &str) -> chr_cases::Case {
    let original = match id {
        "one-zero" => "product-k1-w0-n0",
        "one-work" => "product-k1-w256-n0",
        "two-work" => "product-k2-w256-n0",
        "owner-product" | "duplicate-eight" => "product-k8-w0-n0",
        "asym-first" | "asym-last" => "product-k4-w0-n0",
        "mixed-add-infer" => id,
        "refute-loop" => {
            return chr_cases::Case {
                id: id.into(),
                rules: vec![
                    Rule::simplify("loop", [c("loop", [v(0)])], c("loop", [v(0)]).into()),
                    Rule::simplify("bad", [c("bad", [])], Goal::Fail),
                ],
                query: chr_cases::query(vec![c("loop", [atom("a")]), c("bad", [])], &[]),
                expected: vec![],
                exhausted: true,
                raw_answers: 0,
                budget: 10_000,
                answer_limit: None,
            };
        }
        "stream-prefix" => {
            return chr_cases::Case {
                id: id.into(),
                rules: vec![
                    Rule::simplify(
                        "nums",
                        [c("nums", [v(0)])],
                        or(
                            eq(v(0), atom("z")),
                            and(vec![eq(v(0), t("s", [v(1)])), c("nums", [v(1)]).into()]),
                        ),
                    ),
                    Rule::simplify(
                        "pick",
                        [c("pick", [v(0)])],
                        or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
                    ),
                ],
                query: chr_cases::query(vec![c("nums", [v(0)]), c("pick", [v(1)])], &[0, 1]),
                // Prefix membership is checked mathematically; order is schedule-specific.
                expected: vec![],
                exhausted: false,
                raw_answers: 8,
                budget: 100_000,
                answer_limit: Some(8),
            };
        }
        _ => panic!("unknown regional cost case: {id}"),
    };
    let mut result = originals::input(original);
    result.id = id.into();
    if id == "asym-first" || id == "asym-last" {
        let index = if id == "asym-first" { 0 } else { 3 };
        result.query.constraints[index * 2 + 1].args[1] = unary(256);
    }
    if id == "duplicate-eight" {
        for rule in result.rules.iter_mut().take(8) {
            rule.body = or(eq(v(0), atom("a")), eq(v(0), atom("a")));
        }
        result.expected.truncate(1);
    }
    result
}
