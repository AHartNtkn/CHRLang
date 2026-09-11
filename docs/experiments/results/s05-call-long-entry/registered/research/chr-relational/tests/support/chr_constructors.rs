use chr_syntax::{Constraint, Goal, Rule, and, c, v};
fn keep(name: &str, kept: Vec<Constraint>, removed: Vec<Constraint>, body: Goal) -> Rule {
    Rule {
        name: name.into(),
        kept,
        removed,
        guards: vec![],
        body,
    }
}
pub fn rules() -> Vec<Rule> {
    let mut rules = super::forest::rules(true);
    // Every exposed node-valued column follows source-level parent edges.
    for (pred, arity) in [
        ("d_a", 1),
        ("d_b", 1),
        ("d_f", 2),
        ("below", 2),
        ("take", 2),
    ] {
        for column in 0..arity {
            let args = (0..arity).map(|i| v(i as u64)).collect::<Vec<_>>();
            let mut updated = args.clone();
            updated[column] = v(arity as u64);
            rules.push(keep(
                &format!("repair-{pred}-{column}"),
                vec![c("edge", [args[column].clone(), v(arity as u64)])],
                vec![c(pred, args)],
                c(pred, updated).into(),
            ));
        }
    }
    for pred in ["d_a", "d_b"] {
        rules.push(keep(
            &format!("dedup-{pred}"),
            vec![c(pred, [v(0)])],
            vec![c(pred, [v(0)])],
            Goal::True,
        ));
    }
    rules.push(keep(
        "decompose-f",
        vec![c("d_f", [v(0), v(1)])],
        vec![c("d_f", [v(0), v(2)])],
        c("union", [v(1), v(2)]).into(),
    ));
    for (a, b, arity) in [("d_a", "d_b", 1), ("d_a", "d_f", 2), ("d_b", "d_f", 2)] {
        rules.push(Rule::simplify(
            &format!("clash-{a}-{b}"),
            [
                c(a, [v(0)]),
                c(b, (0..arity).map(|i| v(i as u64)).collect::<Vec<_>>()),
            ],
            Goal::Fail,
        ));
    }
    rules.push(Rule::simplify(
        "occurs",
        [c("below", [v(0), v(0)])],
        Goal::Fail,
    ));
    rules.push(keep(
        "dedup-below",
        vec![c("below", [v(0), v(1)])],
        vec![c("below", [v(0), v(1)])],
        Goal::True,
    ));
    rules.push(Rule::propagate(
        "constructor-child",
        [c("d_f", [v(0), v(1)])],
        c("below", [v(0), v(1)]).into(),
    ));
    rules.push(Rule::propagate(
        "transitive-below",
        [c("below", [v(0), v(1)]), c("below", [v(1), v(2)])],
        c("below", [v(0), v(2)]).into(),
    ));
    rules.push(keep(
        "take-f",
        vec![c("d_f", [v(0), v(1)])],
        vec![c("take", [v(0), v(2)]), c("token", [])],
        and(vec![c("union", [v(2), v(1)]).into()]),
    ));
    rules
}
