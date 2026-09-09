use chr_syntax::{Goal, Query, Rule, Var, and, atom, c, eq, or, t, v};
#[derive(Clone, Copy)]
pub struct Schema {
    pub family: &'static str,
    pub resource: bool,
}
impl Schema {
    pub fn new(family: &str, resource: bool) -> Self {
        Self {
            family: match family {
                "exact" => "exact",
                "rename" => "rename",
                "history" => "history",
                "distinct" => "distinct",
                _ => panic!("family"),
            },
            resource,
        }
    }
    pub fn rules(self) -> Vec<Rule> {
        let branches = (0..4)
            .map(|i| {
                let seed = if self.family == "distinct" {
                    atom(&format!("tag{i}"))
                } else {
                    v(if self.family == "exact" { 10 } else { 10 + i })
                };
                let call: Goal = c("work", [v(0), seed, v(1)]).into();
                if self.family == "history" {
                    let mut gs = (0..i)
                        .map(|j| c("trash", [v(30 + i), v(40 + i * 4 + j)]).into())
                        .collect::<Vec<_>>();
                    gs.push(call);
                    and(gs)
                } else {
                    call
                }
            })
            .collect::<Vec<_>>();
        let body = or(
            or(branches[0].clone(), branches[1].clone()),
            or(branches[2].clone(), branches[3].clone()),
        );
        let terminal = eq(v(1), t("box", [v(0)]));

        let mut rules = vec![
            Rule::simplify("start", [c("start", [v(0), v(1)])], body),
            Rule::simplify(
                "step",
                [c("work", [t("s", [v(0)]), v(1), v(2)])],
                c("work", [v(0), v(1), v(2)]).into(),
            ),
            Rule::simplify("base", [c("work", [atom("z"), v(0), v(1)])], terminal),
            Rule::simplify("trash", [c("trash", [v(0), v(1)])], eq(v(1), atom("unit"))),
        ];
        if self.resource {
            rules.push(Rule::simplify(
                "finish",
                [c("finish", [v(0), v(1)]), c("token", [])],
                eq(v(1), v(0)),
            ));
        }
        rules
    }
    pub fn query(self, n: usize, reverse: bool) -> Query {
        let depth = (0..n).fold(atom("z"), |a, _| t("s", [a]));
        let mut constraints = vec![c("start", [depth, v(99)])];
        if self.resource {
            constraints.extend([c("finish", [v(99), v(100)]), c("token", [])]);
        }
        if reverse {
            constraints.reverse();
        }
        Query {
            constraints,
            outputs: vec![("result".into(), Var(if self.resource { 100 } else { 99 }))],
        }
    }
    pub fn answer_count(self) -> usize {
        4
    }
}
