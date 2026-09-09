//! Same finite-tree equation placed before or after source discrimination.
//! Choices remain four sequential binary ORs, with sixteen causal leaves.
use chr_syntax::{Answer, Query, Rule, Term, Var, and, atom, c, eq, or, t, v};

#[derive(Clone, Copy, Debug)]
pub enum Placement {
    BeforeGate,
    AfterGate,
    /// Deliberately different source: used only to refute failure-tree equivalence.
    NaiveHoist,
}

pub fn rules(placement: Placement) -> Vec<Rule> {
    let choices_tuple = t("tuple", [v(3), v(4), v(5), v(6)]);
    let mut body = Vec::new();
    if matches!(placement, Placement::NaiveHoist) {
        body.push(eq(v(0), v(1)));
    }
    body.extend((3..7).map(|i| or(eq(v(i), atom("a")), eq(v(i), atom("b")))));
    body.push(
        c(
            if matches!(placement, Placement::BeforeGate) {
                "verify"
            } else {
                "gate"
            },
            [v(0), v(1), choices_tuple, v(2)],
        )
        .into(),
    );
    let mut rules = vec![Rule::simplify(
        "choose",
        [c("start", [v(0), v(1), v(2)])],
        and(body),
    )];
    rules.push(Rule::simplify(
        "verify",
        [c("verify", [v(0), v(1), v(2), v(3)])],
        and(vec![
            eq(v(0), v(1)),
            c("gate", [v(0), v(1), v(2), v(3)]).into(),
        ]),
    ));
    for key in 0..16 {
        let tuple = tuple(key);
        let mut body = Vec::new();
        if matches!(placement, Placement::AfterGate) {
            body.push(eq(v(0), v(1)));
        }
        body.extend([
            c("witness", [tuple.clone(), v(2), v(2)]).into(),
            c("done", [v(2), v(2)]).into(),
        ]);
        rules.push(Rule::simplify(
            &format!("gate-{key}"),
            [c("gate", [v(0), v(1), tuple, v(2)])],
            and(body),
        ));
    }
    rules
}

pub fn tuple(key: u8) -> Term {
    t(
        "tuple",
        (0..4)
            .map(|bit| atom(if key & (1 << bit) == 0 { "a" } else { "b" }))
            .collect::<Vec<_>>(),
    )
}

pub fn query(depth: usize, clash: bool) -> Query {
    let left = (0..depth).fold(t("tip", [v(10), v(10), atom("end")]), |tail, _| {
        t("row", [v(10), tail])
    });
    let right = (0..depth).fold(
        t(
            "tip",
            [v(11), v(12), atom(if clash { "clash" } else { "end" })],
        ),
        |tail, _| t("row", [v(11), tail]),
    );
    Query {
        constraints: vec![c("start", [left, right, v(10)])],
        outputs: [("x", 10), ("y", 11), ("z", 12), ("unused", 99)]
            .into_iter()
            .map(|(name, id)| (name.into(), Var(id)))
            .collect(),
    }
}

/// Full structural oracle, independent of either engine's unifier/export.
/// All equation leaves alias; the output-only variable remains independent.
pub fn answer_key(answer: &Answer) -> Option<u8> {
    let [
        (x, Term::Var(a)),
        (y, Term::Var(b)),
        (z, Term::Var(d)),
        (unused, Term::Var(u)),
    ] = answer.outputs.as_slice()
    else {
        return None;
    };
    if (x.as_str(), y.as_str(), z.as_str(), unused.as_str()) != ("x", "y", "z", "unused")
        || a != b
        || a != d
        || a == u
        || answer.residual.len() != 2
    {
        return None;
    }
    let witness = answer.residual.iter().find(|c| c.name == "witness")?;
    let done = answer.residual.iter().find(|c| c.name == "done")?;
    if done.args != [Term::Var(*a), Term::Var(*a)] {
        return None;
    }
    let [Term::App(name, bits), Term::Var(h), Term::Var(j)] = witness.args.as_slice() else {
        return None;
    };
    if name != "tuple" || bits.len() != 4 || h != a || j != a {
        return None;
    }
    let mut key = 0;
    for (bit, term) in bits.iter().enumerate() {
        match term {
            Term::App(n, xs) if xs.is_empty() && n == "a" => (),
            Term::App(n, xs) if xs.is_empty() && n == "b" => key |= 1 << bit,
            _ => return None,
        }
    }
    Some(key)
}
