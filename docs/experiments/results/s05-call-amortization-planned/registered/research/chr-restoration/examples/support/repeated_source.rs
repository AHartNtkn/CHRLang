use chr_syntax::{Query, Rule, Var, and, atom, c, eq, or, t, v};
fn nat(n: usize) -> chr_syntax::Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
pub fn source(family: &str, depth: usize) -> (Vec<Rule>, usize) {
    let mut rules = vec![];
    for key in ["left", "right"] {
        rules.push(Rule::simplify(
            "choose",
            [c("job", [atom(key), v(0), v(1)])],
            or(
                c("walk", [atom(key), v(0), v(1), atom("a")]).into(),
                c("walk", [atom(key), v(0), v(1), atom("b")]).into(),
            ),
        ));
        rules.push(Rule::simplify(
            "walk",
            [c("walk", [atom(key), t("s", [v(0)]), v(1), v(2)])],
            c("walk", [atom(key), v(0), v(1), v(2)]).into(),
        ));
        rules.push(Rule::simplify(
            "ready",
            [c("walk", [atom(key), atom("z"), v(0), v(1)])],
            and(vec![
                eq(v(0), v(1)),
                c("ready", [atom(key), v(0), v(2)]).into(),
            ]),
        ));
        if family == "history" {
            rules.push(Rule {
                name: "remember".into(),
                kept: vec![c("stamp", [atom(key), v(0)])],
                removed: vec![],
                guards: vec![],
                body: c("alias", [atom(key), v(0), v(1), v(1)]).into(),
            });
        }
    }
    let local = rules.len();
    let heads = vec![
        c("ready", [atom("left"), v(0), v(2)]),
        c("ready", [atom("right"), v(1), v(3)]),
    ];
    let mut next_heads = heads.clone();
    next_heads.push(c("round", [atom("left"), t("s", [v(4)])]));
    let mut effects = vec![
        eq(v(0), v(1)),
        c("record", [atom("left"), v(0), v(2), v(2)]).into(),
        c("round", [atom("left"), v(4)]).into(),
        c("job", [atom("left"), nat(depth), v(5)]).into(),
        c("job", [atom("right"), nat(depth), v(6)]).into(),
    ];
    if family == "late" {
        effects.push(eq(v(5), v(6)));
    }
    rules.push(Rule::simplify("next", next_heads, and(effects)));
    rules.push(Rule::simplify(
        "finish",
        heads,
        and(vec![
            eq(v(0), v(1)),
            eq(v(2), v(3)),
            c("joined", [atom("left"), v(0), v(2)]).into(),
        ]),
    ));
    (rules, local)
}
pub fn query(family: &str, rounds: usize, depth: usize, seed: usize) -> Query {
    let x = 1000 + seed as u64 * 10;
    let mut constraints = vec![
        c("job", [atom("left"), nat(depth), v(x)]),
        c("job", [atom("right"), nat(depth), v(x + 1)]),
        c("round", [atom("left"), nat(rounds)]),
        c("seed", [atom("left"), nat(seed)]),
    ];
    if family == "history" {
        for key in ["left", "right"] {
            for stamp in 0..16 {
                constraints.push(c("stamp", [atom(key), nat(stamp)]));
            }
        }
    }
    Query {
        constraints,
        outputs: vec![("x".into(), Var(x)), ("y".into(), Var(x + 1))],
    }
}
