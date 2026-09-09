use chr_syntax::{and, c, eq, t, v, Goal, Rule};
pub fn rules(compression: bool) -> Vec<Rule> {
    fn simpagation(
        name: &str,
        kept: Vec<chr_syntax::Constraint>,
        removed: Vec<chr_syntax::Constraint>,
        body: Goal,
    ) -> Rule {
        Rule {
            name: name.into(),
            kept,
            removed,
            guards: vec![],
            body,
        }
    }
    let mut rules = vec![
        Rule::simplify(
            "union-requests",
            [c("union", [v(0), v(1)])],
            and(vec![
                c("find", [v(0), v(2)]).into(),
                c("find", [v(1), v(3)]).into(),
                c("link", [v(2), v(3)]).into(),
            ]),
        ),
        simpagation(
            "find-root",
            vec![c("root", [v(0)])],
            vec![c("find", [v(0), v(1)])],
            eq(v(1), t("found", [v(0)])),
        ),
        simpagation(
            "find-edge",
            vec![c("edge", [v(0), v(1)])],
            vec![c("find", [v(0), v(2)])],
            if compression {
                and(vec![
                    c("find", [v(1), v(2)]).into(),
                    c("compress", [v(0), v(2)]).into(),
                ])
            } else {
                c("find", [v(1), v(2)]).into()
            },
        ),
        Rule::simplify(
            "link-self",
            [c("link", [t("found", [v(0)]), t("found", [v(0)])])],
            Goal::True,
        ),
        simpagation(
            "redirect-left",
            vec![c("edge", [v(0), v(1)])],
            vec![c("link", [t("found", [v(0)]), t("found", [v(2)])])],
            c("link", [t("found", [v(1)]), t("found", [v(2)])]).into(),
        ),
        simpagation(
            "redirect-right",
            vec![c("edge", [v(0), v(1)])],
            vec![c("link", [t("found", [v(2)]), t("found", [v(0)])])],
            c("link", [t("found", [v(2)]), t("found", [v(1)])]).into(),
        ),
        simpagation(
            "link-roots",
            vec![c("root", [v(1)])],
            vec![
                c("root", [v(0)]),
                c("link", [t("found", [v(0)]), t("found", [v(1)])]),
            ],
            c("edge", [v(0), v(1)]).into(),
        ),
    ];
    if compression {
        rules.push(Rule::simplify(
            "compress-path",
            [
                c("compress", [v(0), t("found", [v(1)])]),
                c("edge", [v(0), v(2)]),
            ],
            c("edge", [v(0), v(1)]).into(),
        ));
    }
    rules
}
