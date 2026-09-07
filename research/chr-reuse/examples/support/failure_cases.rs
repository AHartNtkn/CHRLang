pub fn cases() -> Vec<chr_cases::Case> {
    use chr_syntax::{Goal, Rule, and, atom, c, eq, or, t, v};
    let mut cases = chr_cases::registry();
    for k in [0, 1, 4, 8] {
        for depth in [0, 8, 64] {
            for kind in ["early", "late", "occurs"] {
                fn chain(depth: usize, leaf: chr_syntax::Term) -> chr_syntax::Term {
                    (0..depth).fold(leaf, |t0, _| t("s", [t0]))
                }
                let equation = match kind {
                    "early" => eq(
                        t("pair", [atom("a"), chain(depth, atom("z"))]),
                        t("pair", [atom("b"), chain(depth, atom("z"))]),
                    ),
                    "late" => eq(
                        t("pair", [chain(depth, v(0)), atom("a")]),
                        t("pair", [chain(depth, v(1)), atom("b")]),
                    ),
                    _ => eq(v(0), t("pair", [chain(depth, atom("z")), v(0)])),
                };
                let mut goals = (0..k)
                    .map(|_| or(Goal::True, Goal::True))
                    .collect::<Vec<_>>();
                goals.push(equation);
                cases.push(chr_cases::Case {
                    id: format!("failure-{kind}-k{k}-d{depth}"),
                    rules: vec![Rule::simplify(
                        "start",
                        [c("start", [v(0), v(1)])],
                        and(goals),
                    )],
                    query: chr_cases::query(vec![c("start", [v(0), v(1)])], &[0, 1]),
                    expected: vec![],
                    exhausted: true,
                    raw_answers: 0,
                    budget: 1_000_000,
                    answer_limit: None,
                });
            }
        }
    }
    cases
}
