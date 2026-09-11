use chr_syntax::{Query, Rule, Var, atom, c, eq, v};
pub fn rules(plain: bool) -> Vec<Rule> {
    let mut rules = vec![Rule::simplify(
        "end",
        [c("end", [v(0), v(1)])],
        eq(v(1), v(0)),
    )];
    if !plain {
        rules.push(Rule::simplify(
            "step",
            [c("step", [v(0), v(1), v(2)]), c("link", [v(0), v(1)])],
            eq(v(2), v(1)),
        ));
    }
    rules
}
pub fn query(size: u64, kind: &str, reverse: bool, value: &str) -> Query {
    let desired = atom(value);
    let yielded = if kind.ends_with("miss") {
        atom("miss")
    } else {
        desired.clone()
    };
    let mut constraints = vec![];
    for i in 0..size {
        if kind == "plain" {
            constraints.push(c("end", [desired.clone(), v(100 + i)]));
        } else {
            let index = atom(&format!("i{i}"));
            constraints.push(c("step", [index.clone(), desired.clone(), v(100 + i)]));
            constraints.push(c(
                "link",
                [
                    index,
                    if kind.starts_with("delayed") {
                        v(101 + i)
                    } else {
                        yielded.clone()
                    },
                ],
            ));
        }
    }
    constraints.push(c("end", [yielded, v(100 + size)]));
    if reverse {
        constraints.reverse();
    }
    Query {
        constraints,
        outputs: (0..=size)
            .map(|i| (format!("out{i}"), Var(100 + i)))
            .collect(),
    }
}
