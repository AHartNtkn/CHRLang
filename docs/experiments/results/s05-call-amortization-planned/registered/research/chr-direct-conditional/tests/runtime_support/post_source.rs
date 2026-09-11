use chr_syntax::{Answer, Query, Rule, Var, and, atom, c, eq, v};
pub const FAMILIES: &[&str] = &[
    "post-input",
    "post-output",
    "post-forward",
    "post-miss",
    "post-duplicate",
    "post-template",
];
pub fn rules(kind: &str) -> Vec<Rule> {
    assert!(FAMILIES.contains(&kind));
    let posted = match kind {
        "post-input" => v(1),
        "post-forward" => v(9),
        _ => v(2),
    };
    let mut body = vec![c("token", [v(0), posted.clone()]).into()];
    if kind == "post-duplicate" {
        body.push(c("token", [v(0), posted]).into());
    }
    if kind == "post-forward" {
        body.push(c("end", [v(1), v(9)]).into());
    }
    body.push(eq(v(2), v(1)));
    vec![
        Rule::simplify("emit", [c("emit", [v(0), v(1), v(2)])], and(body)),
        Rule::simplify(
            "take",
            [c("take", [v(0), v(1), v(2)]), c("token", [v(0), v(1)])],
            eq(v(2), v(1)),
        ),
        Rule::simplify("end", [c("end", [v(0), v(1)])], eq(v(1), v(0))),
    ]
}
fn key(kind: &str, i: u64) -> chr_syntax::Term {
    atom(&if kind == "post-template" {
        "same".into()
    } else {
        format!("k{i}")
    })
}
pub fn query(size: u64, kind: &str, reverse: bool, value: &str) -> Query {
    assert!(size > 0 && size < 1000 && FAMILIES.contains(&kind));
    let mut constraints = vec![];
    let mut outputs = vec![];
    for i in 0..size {
        let k = key(kind, i);
        let desired = if kind == "post-miss" {
            atom("absent")
        } else {
            atom(value)
        };
        let id = 1000 + i * 3;
        constraints.push(c("take", [k.clone(), desired.clone(), v(id)]));
        outputs.push((format!("take{i}"), Var(id)));
        if kind == "post-duplicate" {
            constraints.push(c("take", [k.clone(), desired, v(id + 1)]));
            outputs.push((format!("second{i}"), Var(id + 1)));
        }
        constraints.push(c(
            "emit",
            [
                k,
                if kind == "post-template" {
                    atom(value)
                } else {
                    v(100)
                },
                v(id + 2),
            ],
        ));
        outputs.push((format!("emit{i}"), Var(id + 2)));
    }
    constraints.push(c("end", [atom(value), v(100)]));
    if reverse {
        constraints.reverse();
    }
    Query {
        constraints,
        outputs,
    }
}
pub fn expected(size: u64, kind: &str, value: &str) -> Vec<Answer> {
    let mut outputs = vec![];
    let mut residual = vec![];
    for i in 0..size {
        let id = 1000 + i * 3;
        outputs.push((
            format!("take{i}"),
            if kind == "post-miss" {
                v(id)
            } else {
                atom(value)
            },
        ));
        if kind == "post-duplicate" {
            outputs.push((format!("second{i}"), atom(value)));
        }
        outputs.push((format!("emit{i}"), atom(value)));
        if kind == "post-miss" {
            residual.push(c("take", [key(kind, i), atom("absent"), v(id)]));
            residual.push(c("token", [key(kind, i), atom(value)]));
        }
    }
    vec![Answer { outputs, residual }]
}
