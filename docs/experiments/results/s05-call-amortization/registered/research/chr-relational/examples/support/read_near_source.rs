use chr_syntax::{Answer, Goal, Query, Rule, Var, and, atom, c, eq, or, t, v};
pub fn rules(n: usize, kind: &str, token: bool) -> Vec<Rule> {
    let branches = (0..n)
        .map(|i| {
            let a = atom(&format!("a{:02}", if kind == "repeated" { 0 } else { i }));
            let b = if kind == "mixed" && i % 2 == 1 {
                atom("incompatible")
            } else {
                a.clone()
            };
            let mut gs = vec![
                eq(v(0), a),
                eq(v(1), b),
                eq(v(2), v(3)),
                c("tag", [atom(&format!("branch{i}"))]).into(),
            ];
            if token {
                gs.push(c("take", []).into());
            }
            and(gs)
        })
        .collect::<Vec<_>>();
    fn choice(xs: &[Goal]) -> Goal {
        if xs.len() == 1 {
            xs[0].clone()
        } else {
            let m = xs.len() / 2;
            or(choice(&xs[..m]), choice(&xs[m..]))
        }
    }
    let mut rules = vec![Rule::simplify(
        "start",
        [c("start", [v(0), v(1), v(2), v(3)])],
        choice(&branches),
    )];
    if token {
        rules.push(Rule::simplify(
            "take",
            [c("take", []), c("token", [])],
            Goal::True,
        ));
    }
    rules
}
pub fn query(depth: usize, token: bool, reverse: bool) -> Query {
    let deep = |id| (0..depth).fold(v(id), |x, _| t("f", [x]));
    let mut constraints = vec![c("start", [v(100), v(101), deep(100), deep(101)])];
    if token {
        constraints.push(c("token", []));
    }
    if reverse {
        constraints.reverse();
    }
    Query {
        constraints,
        outputs: vec![("x".into(), Var(100)), ("y".into(), Var(101))],
    }
}
pub fn expected(n: usize, kind: &str) -> Vec<Answer> {
    (0..n)
        .filter(|i| kind != "mixed" || i % 2 == 0)
        .map(|i| {
            let a = atom(&format!("a{:02}", if kind == "repeated" { 0 } else { i }));
            Answer {
                outputs: vec![("x".into(), a.clone()), ("y".into(), a)],
                residual: vec![c("tag", [atom(&format!("branch{i}"))])],
            }
        })
        .collect()
}
pub fn source(
    n: usize,
    depth: usize,
    kind: &str,
    token: bool,
    reverse: bool,
) -> (Vec<Rule>, Query, Vec<Answer>) {
    (
        rules(n, kind, token),
        query(depth, token, reverse),
        expected(n, kind),
    )
}
