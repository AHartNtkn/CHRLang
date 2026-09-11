//! Owned source fixture and independent, allocation-free complete-answer oracle.
use chr_syntax::{Answer, Goal, Query, Rule, Term, and, atom, c, eq, or, t, v};

pub const RAW_ANSWERS: usize = 16;

pub fn rules() -> Vec<Rule> {
    let tuple = t("tuple", [v(2), v(3), v(4), v(5)]);
    let mut choices = (2..6)
        .map(|id| or(eq(v(id), atom("a")), eq(v(id), atom("b"))))
        .collect::<Vec<_>>();
    choices.push(c("pre", [v(0), v(1), tuple, v(6)]).into());
    let mut rules = vec![
        Rule::simplify("choose", [c("start", [v(0), v(1)])], and(choices)),
        Rule::simplify(
            "pre-step",
            [c("pre", [t("s", [v(0)]), v(1), v(2), v(3)])],
            c("pre", [v(0), v(1), v(2), v(3)]).into(),
        ),
        Rule::simplify(
            "pre-done",
            [c("pre", [atom("z"), v(1), v(2), v(3)])],
            c("gate", [v(1), v(2), v(3)]).into(),
        ),
    ];
    for key in 0..RAW_ANSWERS {
        let tuple = t(
            "tuple",
            (0..4)
                .map(|bit| atom(if key & (1 << bit) == 0 { "a" } else { "b" }))
                .collect::<Vec<_>>(),
        );
        rules.push(Rule::simplify(
            &format!("gate-{key}"),
            [c("gate", [v(0), tuple.clone(), v(1)])],
            and(vec![
                c("post", [v(0), v(1)]).into(),
                c("witness", [tuple, v(1), v(1)]).into(),
            ]),
        ));
    }
    rules.extend([
        Rule::simplify(
            "post-step",
            [c("post", [t("s", [v(0)]), v(1)])],
            c("post", [v(0), v(1)]).into(),
        ),
        Rule::simplify(
            "post-done",
            [c("post", [atom("z"), v(1)])],
            Goal::from(c("done", [v(1), v(1)])),
        ),
    ]);
    rules
}

pub fn query(pre: usize, post: usize) -> Query {
    let depth = |n| (0..n).fold(atom("z"), |rest, _| t("s", [rest]));
    Query {
        constraints: vec![c("start", [depth(pre), depth(post)])],
        outputs: vec![],
    }
}

/// Returns the four-bit source choice only after checking the entire answer,
/// including the unknown shared across both residuals. Residual order is free.
pub fn answer_key(answer: &Answer) -> Option<u8> {
    if !answer.outputs.is_empty() || answer.residual.len() != 2 {
        return None;
    }
    let witness = answer.residual.iter().find(|c| c.name == "witness")?;
    let done = answer.residual.iter().find(|c| c.name == "done")?;
    let [tuple, Term::Var(shared), Term::Var(alias)] = witness.args.as_slice() else {
        return None;
    };
    if shared != alias || done.args.as_slice() != [Term::Var(*shared), Term::Var(*shared)] {
        return None;
    }
    let Term::App(name, bits) = tuple else {
        return None;
    };
    if name != "tuple" || bits.len() != 4 {
        return None;
    }
    let mut key = 0;
    for (bit, value) in bits.iter().enumerate() {
        match value {
            Term::App(name, fields) if fields.is_empty() && name == "a" => (),
            Term::App(name, fields) if fields.is_empty() && name == "b" => key |= 1 << bit,
            _ => return None,
        }
    }
    Some(key)
}
