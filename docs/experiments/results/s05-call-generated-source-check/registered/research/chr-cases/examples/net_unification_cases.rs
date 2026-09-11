use chr_syntax::{Query, Rule, Term, Var, and, atom, c, eq, t, v};
fn array(xs: impl IntoIterator<Item = String>) -> String {
    format!("[{}]", xs.into_iter().collect::<Vec<_>>().join(","))
}
fn term(x: &Term) -> String {
    match x {
        Term::Var(v) => v.0.to_string(),
        Term::App(n, xs) => format!("[\"{n}\",{}]", array(xs.iter().map(term))),
    }
}
fn main() {
    let terms = vec![
        atom("a"),
        atom("b"),
        v(0),
        v(1),
        v(2),
        t("f", [atom("a")]),
        t("f", [v(0)]),
        t("f", [v(1)]),
        t("p", [v(0), v(0)]),
        t("p", [v(0), v(1)]),
        t("f", [t("f", [v(0)])]),
        t("p", [t("f", [v(0)]), v(2)]),
    ];
    let initial = [vec![], vec![(v(0), v(1))], vec![(v(0), t("f", [v(1)]))]];
    for (environment, bindings) in initial.iter().enumerate() {
        for (i, a) in terms.iter().enumerate() {
            for (j, b) in terms.iter().enumerate() {
                let mut goals = bindings
                    .iter()
                    .map(|(a, b)| eq(a.clone(), b.clone()))
                    .collect::<Vec<_>>();
                goals.push(eq(a.clone(), b.clone()));
                let rule =
                    Rule::simplify("request", [c("request", [v(0), v(1), v(2)])], and(goals));
                let query = Query {
                    constraints: vec![c("request", [v(0), v(1), v(2)])],
                    outputs: (0..3).map(|i| (format!("v{i}"), Var(i))).collect(),
                };
                let mut reference = chr_reference::Search::new(vec![rule], query).unwrap();
                let batch = reference.advance(100_000);
                assert!(batch.exhausted);
                assert!(batch.answers.len() <= 1);
                assert!(batch.answers.iter().all(|a| a.residual.is_empty()));
                println!(
                    "{{\"id\":\"{environment}-{i}-{j}\",\"initial\":{},\"equation\":{},\"answers\":{}}}",
                    array(bindings.iter().map(|(a, b)| array([term(a), term(b)]))),
                    array([term(a), term(b)]),
                    array(
                        batch
                            .answers
                            .iter()
                            .map(|a| array(a.outputs.iter().map(|(_, t)| term(t))))
                    )
                );
            }
        }
    }
}
