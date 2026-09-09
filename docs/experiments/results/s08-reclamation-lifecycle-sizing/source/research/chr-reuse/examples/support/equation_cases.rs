pub fn cases() -> Vec<chr_cases::Case> {
    use chr_syntax::{Goal, Rule, and, atom, c, eq, t, v};
    fn unary(n: usize) -> chr_syntax::Term {
        (0..n).fold(atom("z"), |a, _| t("s", [a]))
    }
    fn chain(n: usize, leaf: chr_syntax::Term) -> chr_syntax::Term {
        (0..n).fold(leaf, |a, _| t("f", [a]))
    }
    let mut cases = chr_cases::registry();
    for count in [1, 8, 64, 256] {
        for depth in [0, 4, 16, 64] {
            for kind in ["repeat", "unique", "identity"] {
                // Rule variables: 0 counter, 1 caller hole, 2 tail, 3 fresh local hole.
                let equation = match kind {
                    "identity" => eq(chain(depth, v(1)), chain(depth, v(1))),
                    "unique" => eq(
                        t("triple", [chain(depth, v(1)), v(1), t("tag", [v(0)])]),
                        t("triple", [chain(depth, v(3)), atom("a"), t("tag", [v(0)])]),
                    ),
                    _ => eq(
                        t("pair", [chain(depth, v(1)), v(1)]),
                        t("pair", [chain(depth, v(3)), atom("a")]),
                    ),
                };
                let held = if kind == "identity" {
                    c("held", [v(1), v(1)])
                } else {
                    c("held", [v(1), v(3)])
                };
                let mut vars = atom("nil");
                for i in (0..count).rev() {
                    vars = t("cons", [v(i as u64), vars]);
                }
                let values = (0..count)
                    .map(|i| {
                        if kind == "identity" {
                            v(i as u64)
                        } else {
                            atom("a")
                        }
                    })
                    .collect::<Vec<_>>();
                let residual = values
                    .iter()
                    .map(|x| c("held", [x.clone(), x.clone()]))
                    .collect();
                cases.push(chr_cases::Case {
                    id: format!("equation-{kind}-r{count}-d{depth}"),
                    rules: vec![
                        Rule::simplify(
                            "run",
                            [c("run", [t("s", [v(0)]), t("cons", [v(1), v(2)])])],
                            and([equation, held.into(), c("run", [v(0), v(2)]).into()]),
                        ),
                        Rule::simplify("done", [c("run", [atom("z"), atom("nil")])], Goal::True),
                    ],
                    query: chr_cases::query(
                        vec![c("run", [unary(count), vars])],
                        &(0..count as u64).collect::<Vec<_>>(),
                    ),
                    expected: vec![chr_cases::answer(values, residual)],
                    exhausted: true,
                    raw_answers: 1,
                    budget: 1_000_000,
                    answer_limit: None,
                });
            }
        }
    }
    cases
}
