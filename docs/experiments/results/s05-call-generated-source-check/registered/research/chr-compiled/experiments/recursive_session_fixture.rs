//! Source shared by the external session gate's three backends.
use chr_syntax::{Rule, and, atom, c, eq, t, v};
pub fn rules(kind: &str) -> Vec<Rule> {
    assert!(matches!(kind, "add" | "fresh"), "unknown fixture");
    if kind == "fresh" {
        return vec![
            Rule::simplify(
                "base",
                [c("fresh", [atom("z"), v(1), v(2)])],
                eq(v(1), v(2)),
            ),
            Rule::simplify(
                "step",
                [c("fresh", [t("s", [v(0)]), v(1), v(2)])],
                c("fresh", [v(0), t("pair", [v(1), v(3)]), v(2)]).into(),
            ),
        ];
    }
    vec![
        Rule::simplify("base", [c("add", [atom("z"), v(1), v(2)])], eq(v(1), v(2))),
        Rule::simplify(
            "step",
            [c("add", [t("s", [v(0)]), v(1), v(2)])],
            and(vec![
                eq(v(2), t("s", [v(3)])),
                c("add", [v(0), v(1), v(3)]).into(),
            ]),
        ),
    ]
}
