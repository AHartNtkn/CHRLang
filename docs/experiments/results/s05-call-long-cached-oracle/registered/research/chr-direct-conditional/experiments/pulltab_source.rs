use chr_syntax::{Answer, Query, Rule, Var, atom, c, eq, or, t, v};
#[derive(Clone, Copy)]
pub struct Schema {
    pub family: &'static str,
    pub consumers: u64,
    pub independent: bool,
    pub resource: bool,
}
impl Schema {
    pub fn new(family: &str, resource: bool) -> Self {
        let (family, consumers, independent) = match family {
            "direct1" => ("direct", 1, false),
            "shared4" => ("direct", 4, false),
            "direct4" => ("direct", 4, true),
            "opaque4" => ("opaque", 4, true),
            "nested4" => ("nested", 4, true),
            _ => panic!("unknown source family"),
        };
        Self {
            family,
            consumers,
            independent,
            resource,
        }
    }
    pub fn rules(self) -> Vec<Rule> {
        let mut rules = vec![
            Rule::simplify(
                "choose",
                [c("choose", [v(0)])],
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
            ),
            Rule::simplify("id", [c("id", [v(0), v(1)])], eq(v(1), v(0))),
        ];
        if self.family == "opaque" {
            rules.push(Rule::simplify(
                "use",
                [c("use", [v(0), v(1), v(2)])],
                eq(v(2), t("answer", [v(0), v(1)])),
            ));
        } else {
            for value in ["a", "b"] {
                let pattern = if self.family == "nested" {
                    t("box", [atom(value)])
                } else {
                    atom(value)
                };
                rules.push(Rule::simplify(
                    value,
                    [c("use", [pattern, v(0), v(1)])],
                    eq(v(1), t("answer", [atom(value), v(0)])),
                ));
            }
        }
        if self.resource {
            rules.push(Rule::simplify(
                "finish",
                [c("finish", [v(0), v(1)]), c("token", [])],
                eq(v(1), v(0)),
            ));
        }
        rules
    }
    pub fn answer_count(self) -> usize {
        1 << if self.independent { self.consumers } else { 1 }
    }
    pub fn query(self, depth: usize, reverse: bool) -> Query {
        // The tag and insertion order change together between reused queries.
        let tag = atom(if reverse { "key1" } else { "key0" });
        let producers = if self.independent { self.consumers } else { 1 };
        let mut constraints = vec![];
        let mut roots = vec![];
        for producer in 0..producers {
            let first = 100 + producer * 1000;
            constraints.push(c("choose", [v(first)]));
            for step in 0..depth as u64 {
                constraints.push(c("id", [v(first + step), v(first + step + 1)]));
            }
            roots.push(first + depth as u64);
        }
        let mut outputs = vec![];
        for consumer in 0..self.consumers {
            let input = v(roots[if self.independent {
                consumer as usize
            } else {
                0
            }]);
            let input = if self.family == "nested" {
                t("box", [input])
            } else {
                input
            };
            let out = 10000 + consumer;
            constraints.push(c("use", [input, tag.clone(), v(out)]));
            let out = if self.resource {
                constraints.extend([c("finish", [v(out), v(out + 100)]), c("token", [])]);
                out + 100
            } else {
                out
            };
            outputs.push((format!("out{consumer}"), Var(out)));
        }
        if reverse {
            constraints.reverse();
        }
        Query {
            constraints,
            outputs,
        }
    }
    pub fn expected(self, reverse: bool) -> Vec<Answer> {
        (0..self.answer_count())
            .map(|mask| Answer {
                outputs: (0..self.consumers)
                    .map(|consumer| {
                        let bit = if self.independent { consumer } else { 0 };
                        (
                            format!("out{consumer}"),
                            t(
                                "answer",
                                [
                                    atom(if mask & (1 << bit) == 0 { "a" } else { "b" }),
                                    atom(if reverse { "key1" } else { "key0" }),
                                ],
                            ),
                        )
                    })
                    .collect(),
                residual: vec![],
            })
            .collect()
    }
}
