use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, atom, c, eq, or, t, v};

#[derive(Clone, Copy)]
pub struct Schema {
    pub family: &'static str,
    pub resource: bool,
    pub fail_tail: bool,
    pub work: usize,
    pub payload: usize,
}
impl Schema {
    pub fn rules(self) -> Vec<Rule> {
        assert!(matches!(self.family, "repeated" | "distinct" | "aliases"));
        let next = if self.family == "distinct" {
            t("next", [v(1)])
        } else {
            v(1)
        };
        let mut rules = vec![
            Rule::simplify(
                "emit-or-continue",
                [c("stream", [t("s", [v(0)]), v(1), v(2), v(3), v(4)])],
                or(
                    c("wait", [v(2), v(1), v(3), v(4)]).into(),
                    c("stream", [v(0), next, v(2), v(3), v(4)]).into(),
                ),
            ),
            Rule::simplify(
                "last",
                [c("stream", [atom("z"), v(1), v(2), v(3), v(4)])],
                if self.fail_tail {
                    Goal::Fail
                } else {
                    c("wait", [v(2), v(1), v(3), v(4)]).into()
                },
            ),
            Rule::simplify(
                "wait-step",
                [c("wait", [t("s", [v(0)]), v(1), v(2), v(3)])],
                c("wait", [v(0), v(1), v(2), v(3)]).into(),
            ),
            Rule::simplify(
                "wait-done",
                [c("wait", [atom("z"), v(1), v(2), v(3)])],
                eq(v(3), t("item", [v(1), v(1), v(2)])),
            ),
        ];
        if self.resource {
            rules.push(Rule::simplify(
                "consume",
                [
                    c("finish", [t("item", [v(0), v(1), v(2)]), v(3)]),
                    c("token", []),
                ],
                eq(v(3), t("item", [v(0), v(1), v(2)])),
            ));
        }
        rules
    }
    pub fn query(self, n: usize, reverse: bool) -> Query {
        let mut constraints = vec![c(
            "stream",
            [
                unary("s", "z", n),
                if self.family == "aliases" {
                    v(42)
                } else {
                    atom("root")
                },
                unary("s", "z", self.work),
                unary("data", "end", self.payload),
                v(99),
            ],
        )];
        if self.resource {
            constraints.extend([c("finish", [v(99), v(100)]), c("token", [])]);
        }
        if reverse {
            constraints.reverse();
        }
        let result = Var(if self.resource { 100 } else { 99 });
        Query {
            constraints,
            outputs: vec![("result".into(), result), ("again".into(), result)],
        }
    }
    pub fn expected(self, n: usize) -> Vec<Answer> {
        (0..n + usize::from(!self.fail_tail))
            .map(|i| {
                let seed = match self.family {
                    "distinct" => unary("next", "root", i),
                    "aliases" => v(42),
                    _ => atom("root"),
                };
                let value = t(
                    "item",
                    [seed.clone(), seed, unary("data", "end", self.payload)],
                );
                Answer {
                    outputs: vec![("result".into(), value.clone()), ("again".into(), value)],
                    residual: vec![],
                }
            })
            .collect()
    }
}
fn unary(name: &str, end: &str, n: usize) -> Term {
    (0..n).fold(atom(end), |tail, _| t(name, [tail]))
}
