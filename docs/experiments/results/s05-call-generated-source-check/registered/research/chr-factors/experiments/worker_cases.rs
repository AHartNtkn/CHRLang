//! Independent recursive regions with hand-derived complete products.
use chr_syntax::{Answer, Goal, Query, Rule, atom, c, eq, or, t, v};
pub fn source() -> Vec<Rule> {
    (0..4)
        .flat_map(|i| {
            let p = format!("p{i}");
            [
                Rule::simplify(
                    &format!("base{i}"),
                    [c(&p, [atom("z"), v(0)])],
                    or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
                ),
                Rule::simplify(
                    &format!("step{i}"),
                    [c(&p, [t("s", [v(0)]), v(1)])],
                    Goal::Constraint(c(&p, [v(0), v(1)])),
                ),
            ]
        })
        .collect()
}
pub fn query(count: usize, depth: usize, skew: bool) -> Query {
    chr_cases::query(
        (0..count)
            .map(|i| {
                let mut n = atom("z");
                for _ in 0..depth * if skew { i + 1 } else { 1 } {
                    n = t("s", [n]);
                }
                c(&format!("p{i}"), [n, v(i as u64)])
            })
            .collect(),
        &(0..count as u64).collect::<Vec<_>>(),
    )
}
pub fn expected(count: usize) -> Vec<Answer> {
    let mut answers = (0..1usize << count)
        .map(|bits| {
            chr_cases::answer(
                (0..count)
                    .map(|i| atom(if bits & (1 << i) == 0 { "a" } else { "b" }))
                    .collect(),
                vec![],
            )
        })
        .collect::<Vec<_>>();
    answers.sort_unstable_by(|a, b| a.outputs.cmp(&b.outputs));
    answers
}
