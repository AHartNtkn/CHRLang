//! Source contract for demand-driven three-relation discovery (S01/T068).
use chr_syntax::{Query, Rule, Term, Var, and, atom, c, eq, t, v};

/// Requests observe a live demand; updates do not themselves publish receipts.
/// Source rule order drains each pulse before the driver advances.
pub fn source_rules(consuming: bool) -> Vec<Rule> {
    let mut join = Rule::propagate(
        "join",
        [
            c("left", [v(0), t("f", [v(1)])]),
            c("middle", [v(1), t("g", [v(2)])]),
            c("right", [v(2), t("h", [v(3)])]),
            c("demand", [v(0), v(3), v(4)]),
            c("pulse", [v(4), v(5)]),
        ],
        c(
            "receipt",
            [v(4), v(5), t("f", [v(1)]), t("g", [v(2)]), t("h", [v(3)])],
        )
        .into(),
    );
    if consuming {
        join.removed = vec![join.kept.remove(2)];
    }
    let mut rules = vec![
        join,
        Rule::simplify(
            "ack",
            [c("pulse", [v(0), v(1)]), c("wait", [v(2)])],
            c("drive", [v(2)]).into(),
        ),
        Rule::simplify(
            "issue",
            [c("drive", [t("cons", [t("ask", [v(0), v(1)]), v(2)])])],
            and([c("pulse", [v(0), v(1)]).into(), c("wait", [v(2)]).into()]),
        ),
        Rule::simplify(
            "open",
            [c(
                "drive",
                [t("cons", [t("open", [v(0), v(1), v(2)]), v(3)])],
            )],
            and([
                c("demand", [v(0), v(1), v(2)]).into(),
                c("drive", [v(3)]).into(),
            ]),
        ),
        Rule::simplify(
            "close",
            [
                c("drive", [t("cons", [t("close", [v(0)]), v(1)])]),
                c("demand", [v(2), v(3), v(0)]),
            ],
            c("drive", [v(1)]).into(),
        ),
        Rule::simplify(
            "bind",
            [c("drive", [t("cons", [t("bind", [v(0), v(1)]), v(2)])])],
            and([eq(v(0), v(1)), c("drive", [v(2)]).into()]),
        ),
    ];
    for relation in ["left", "middle", "right"] {
        rules.push(Rule::simplify(
            &format!("insert_{relation}"),
            [c(
                "drive",
                [t(
                    "cons",
                    [t(&format!("insert_{relation}"), [v(0), v(1)]), v(2)],
                )],
            )],
            and([c(relation, [v(0), v(1)]).into(), c("drive", [v(2)]).into()]),
        ));
        rules.push(Rule::simplify(
            &format!("remove_{relation}"),
            [
                c(
                    "drive",
                    [t(
                        "cons",
                        [t(&format!("remove_{relation}"), [v(0), v(1)]), v(2)],
                    )],
                ),
                c(relation, [v(0), v(1)]),
            ],
            c("drive", [v(2)]).into(),
        ));
    }
    rules.push(Rule::simplify(
        "done",
        [c("drive", [atom("nil")])],
        c("done", []).into(),
    ));
    rules
}

pub fn script(ops: Vec<Term>) -> Term {
    ops.into_iter()
        .rev()
        .fold(atom("nil"), |tail, op| t("cons", [op, tail]))
}

pub fn query(mut rows: Vec<chr_syntax::Constraint>, ops: Vec<Term>, driver_first: bool) -> Query {
    let driver = c("drive", [script(ops)]);
    if driver_first {
        rows.insert(0, driver);
    } else {
        rows.push(driver);
    }
    Query {
        constraints: rows,
        outputs: vec![("x".into(), Var(50)), ("y".into(), Var(51))],
    }
}
