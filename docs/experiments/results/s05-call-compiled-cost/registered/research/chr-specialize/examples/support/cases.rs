pub fn cases() -> Vec<chr_cases::Case> {
    use chr_syntax::{c, v};
    let mut cases = chr_cases::registry()
        .into_iter()
        .filter(|c| chr_specialize::specialize(&c.rules, &c.query, 0).is_ok())
        .collect::<Vec<_>>();
    for size in [0, 1, 4, 16, 64] {
        for mode in [
            "forward",
            "left",
            "right",
            "decompose",
            "repeated",
            "failure",
        ] {
            let n = chr_programs::unary;
            let (args, outputs, values) = match mode {
                "forward" => (
                    vec![n(size), n(size), v(0)],
                    vec![0],
                    vec![vec![n(size * 2)]],
                ),
                "left" => (
                    vec![v(0), n(size), n(size * 2)],
                    vec![0],
                    vec![vec![n(size)]],
                ),
                "right" => (
                    vec![n(size), v(0), n(size * 2)],
                    vec![0],
                    vec![vec![n(size)]],
                ),
                "decompose" => (
                    vec![v(0), v(1), n(size)],
                    vec![0, 1],
                    (0..=size).map(|i| vec![n(i), n(size - i)]).collect(),
                ),
                "repeated" => (vec![v(0), v(0), n(size * 2)], vec![0], vec![vec![n(size)]]),
                _ => (vec![n(size + 1), v(0), n(size)], vec![0], vec![]),
            };
            let expected = values
                .into_iter()
                .map(|v| chr_cases::answer(v, vec![]))
                .collect::<Vec<_>>();
            cases.push(chr_cases::Case {
                id: format!("specialize-{mode}-n{size}"),
                rules: chr_programs::arithmetic(),
                query: chr_cases::query(vec![c("add", args)], &outputs),
                raw_answers: expected.len() as u64,
                expected,
                exhausted: true,
                budget: 1_000_000,
                answer_limit: None,
            });
        }
    }
    cases
}
