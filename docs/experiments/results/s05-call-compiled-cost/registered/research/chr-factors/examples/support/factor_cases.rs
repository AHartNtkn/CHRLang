use chr_syntax::{Rule, Term, Var, atom, c, eq, or, t, v};
fn unary(n: usize) -> Term {
    (0..n).fold(atom("z"), |n, _| t("s", [n]))
}
pub fn names() -> Vec<String> {
    let mut names = chr_cases::registry()
        .into_iter()
        .filter(|c| c.exhausted)
        .map(|c| c.id)
        .collect::<Vec<_>>();
    for k in [1, 2, 4, 8] {
        for w in [0, 1, 8, 64, 256] {
            for n in [0, 16] {
                names.push(format!("product-k{k}-w{w}-n{n}"));
            }
        }
    }
    names.push("mixed-add-infer".into());
    names
}
fn shift(t: &Term) -> Term {
    match t {
        Term::Var(Var(v)) => chr_syntax::v(v + 1000),
        Term::App(n, args) => chr_syntax::t(n, args.iter().map(shift).collect::<Vec<_>>()),
    }
}
pub fn input(id: &str) -> chr_cases::Case {
    if let Some(case) = chr_cases::registry().into_iter().find(|c| c.id == id) {
        return case;
    }
    if id == "mixed-add-infer" {
        let mut add = chr_cases::application_cases()
            .into_iter()
            .find(|c| c.id == "app-add-forward")
            .unwrap();
        let infer = chr_cases::application_cases()
            .into_iter()
            .find(|c| c.id == "app-infer-identity")
            .unwrap();
        add.id = id.into();
        add.rules.extend(infer.rules);
        add.query
            .constraints
            .extend(
                infer
                    .query
                    .constraints
                    .into_iter()
                    .map(|c| chr_syntax::Constraint {
                        name: c.name,
                        args: c.args.iter().map(shift).collect(),
                    }),
            );
        add.query.outputs.push(("out1".into(), Var(1000)));
        add.expected = vec![chr_cases::answer(
            vec![unary(5), t("fun", [v(0), v(0)])],
            vec![],
        )];
        add.budget = 100000;
        return add;
    }
    for k in [1, 2, 4, 8] {
        for w in [0, 1, 8, 64, 256] {
            for n in [0, 16] {
                if id == format!("product-k{k}-w{w}-n{n}") {
                    let mut rules = vec![];
                    let mut constraints = vec![];
                    for i in 0..k {
                        rules.push(Rule::simplify(
                            &format!("pick-{i}"),
                            [c(&format!("pick{i}"), [v(0)])],
                            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
                        ));
                    }
                    for i in 0..k {
                        rules.push(Rule::simplify(
                            &format!("carry-{i}"),
                            [c(&format!("carry{i}"), [v(0), t("s", [v(1)])])],
                            c(&format!("carry{i}"), [v(0), v(1)]).into(),
                        ));
                    }
                    for i in 0..k {
                        constraints.push(c(&format!("pick{i}"), [v(i)]));
                        constraints.push(c(&format!("carry{i}"), [v(i), unary(w)]));
                    }
                    let noise = (0..n)
                        .map(|i| c("noise", [unary(i + 1)]))
                        .collect::<Vec<_>>();
                    constraints.extend(noise.clone());
                    let expected = (0..1u64 << k)
                        .map(|bits| {
                            let values = (0..k)
                                .map(|i| atom(if bits & (1 << i) == 0 { "a" } else { "b" }))
                                .collect::<Vec<_>>();
                            let mut residual = (0..k)
                                .map(|i| {
                                    c(
                                        &format!("carry{i}"),
                                        [values[i as usize].clone(), atom("z")],
                                    )
                                })
                                .collect::<Vec<_>>();
                            residual.extend(noise.clone());
                            chr_cases::answer(values, residual)
                        })
                        .collect();
                    return chr_cases::Case {
                        id: id.into(),
                        rules,
                        query: chr_cases::query(constraints, &(0..k).collect::<Vec<_>>()),
                        expected,
                        exhausted: true,
                        raw_answers: 1 << k,
                        budget: 5_000_000,
                        answer_limit: None,
                    };
                }
            }
        }
    }
    panic!("unknown case");
}
