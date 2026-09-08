//! Source shared by the external session gate's three backends.
use chr_syntax::{Rule, and, atom, c, eq, t, v};
pub fn rules() -> Vec<Rule> {
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
