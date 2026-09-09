use chr_syntax::{Query, Rule, Var, and, atom, c, eq, or, t, v};
#[derive(Clone, Copy)]
pub struct Schema {
    family: &'static str,
    resource: bool,
}
impl Schema {
    pub fn new(family: &str, resource: bool, _: usize) -> Self {
        let family = match family {
            "pass" => "pass",
            "unary" => "unary",
            "nested" => "nested",
            "open" => "open",
            "late" => "late",
            "malformed" => "malformed",
            "choice" => "choice",
            "multi" => "multi",
            "multi-choice" => "multi-choice",
            "fail" => "fail",
            _ => panic!("unknown recursive family"),
        };
        Self { family, resource }
    }
    fn calls(self) -> usize {
        if self.family.starts_with("multi") {
            2
        } else {
            1
        }
    }
    pub fn answer_count(self) -> usize {
        if self.family == "fail" {
            0
        } else if self.family.contains("choice") {
            1 << self.calls()
        } else {
            1
        }
    }
    pub fn rules(self) -> Vec<Rule> {
        let update = match self.family {
            "pass" => v(1),
            "nested" => t("pair", [t("f", [v(1)]), atom("tag")]),
            _ => t("f", [v(1)]),
        };
        let equation = eq(v(1), v(0));
        let equation = if self.family.contains("choice") {
            or(equation, eq(v(1), t("alt", [v(0)])))
        } else {
            equation
        };
        let terminal = if self.resource {
            and(vec![equation, c("claim", [v(0)]).into()])
        } else {
            equation
        };
        let mut rules = vec![
            Rule::simplify(
                "step",
                [c("fold", [t("s", [v(0)]), v(1), v(2)])],
                c("fold", [v(0), update, v(2)]).into(),
            ),
            Rule::simplify("base", [c("fold", [atom("z"), v(0), v(1)])], terminal),
        ];
        if self.resource {
            rules.push(Rule::simplify(
                "take",
                [c("claim", [v(0)]), c("token", [])],
                c("done", [v(0)]).into(),
            ));
        }
        if self.family == "late" {
            rules.push(Rule::simplify(
                "bind",
                [c("bind", [v(0)])],
                eq(v(0), atom("z")),
            ));
        }
        rules
    }
    pub fn query(self, depth: usize, reverse: bool) -> Query {
        let mut constraints = vec![];
        let mut outputs = vec![];
        for i in 0..self.calls() {
            let id = 100 + i as u64 * 3;
            let tail = match self.family {
                "open" | "late" => v(id),
                "malformed" => atom("bad"),
                _ => atom("z"),
            };
            let input = (0..depth + i).fold(tail, |n, _| t("s", [n]));
            let result = if self.family == "fail" {
                atom("clash")
            } else {
                v(id + 2)
            };
            let seed = if self.family == "fail" {
                atom("seed")
            } else {
                v(id + 1)
            };
            constraints.push(c("fold", [input, seed, result]));
            if self.family == "late" {
                constraints.push(c("bind", [v(id)]));
            }
            outputs.extend([
                (format!("tail{i}"), Var(id)),
                (format!("seed{i}"), Var(id + 1)),
                (format!("result{i}"), Var(id + 2)),
            ]);
        }
        // One token forces competing completions to retain source arbitration.
        if self.resource {
            constraints.push(c("token", []));
        }
        if reverse {
            constraints.reverse();
        }
        Query {
            constraints,
            outputs,
        }
    }
}
