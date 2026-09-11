use chr_syntax::{Answer, Query, Rule, Term, Var, atom, c, eq, or, t, v};
#[derive(Clone, Copy)]
pub struct Schema {
    pub family: &'static str,
    pub calls: u64,
    pub resource: bool,
}
impl Schema {
    pub fn new(family: &str, resource: bool) -> Self {
        let (family, calls) = match family {
            "single" => ("single", 1),
            "repeat" => ("repeat", 4),
            "distinct" => ("distinct", 4),
            "choice" => ("choice", 4),
            "grow" => ("grow", 1),
            _ => panic!("unknown family"),
        };
        Self {
            family,
            calls,
            resource,
        }
    }
    pub fn rules(self) -> Vec<Rule> {
        let value = eq(v(2), t("item", [v(1), v(0), v(99)]));
        let value = if self.family == "choice" {
            or(value, eq(v(2), t("other", [v(1), v(0), v(99)])))
        } else {
            value
        };
        let accumulator = if self.family == "grow" {
            t("pair", [v(1), v(1)])
        } else {
            v(1)
        };
        let mut rules = vec![
            Rule::simplify("base", [c("build", [atom("z"), v(0), v(1), v(2)])], value),
            Rule::simplify(
                "step",
                [c("build", [t("s", [v(0)]), v(1), v(2), v(3)])],
                c("build", [v(0), accumulator, v(2), v(3)]).into(),
            ),
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
    fn key(self, i: u64, reverse: bool) -> Term {
        atom(&format!(
            "key{}_{}",
            if self.family == "distinct" { i } else { 0 },
            u8::from(reverse)
        ))
    }
    pub fn answer_count(self) -> usize {
        if self.family == "choice" {
            1 << self.calls
        } else {
            1
        }
    }
    pub fn query(self, depth: usize, reverse: bool) -> Query {
        assert!(depth <= if self.family == "grow" { 13 } else { 129 });
        let n = (0..depth).fold(atom("z"), |x, _| t("s", [x]));
        let mut constraints = vec![];
        let mut outputs = vec![];
        for i in 0..self.calls {
            constraints.push(c(
                "build",
                [n.clone(), atom("seed"), self.key(i, reverse), v(100 + i)],
            ));
            let out = if self.resource {
                constraints.extend([c("finish", [v(100 + i), v(200 + i)]), c("token", [])]);
                200 + i
            } else {
                100 + i
            };
            outputs.push((format!("out{i}"), Var(out)));
        }
        if reverse {
            constraints.reverse();
        }
        Query {
            constraints,
            outputs,
        }
    }
    pub fn expected(self, depth: usize, reverse: bool) -> Vec<Answer> {
        self.values(depth, reverse).collect()
    }
    fn values(self, depth: usize, reverse: bool) -> Values {
        Values {
            schema: self,
            depth,
            reverse,
            next: 0,
        }
    }
}
/// Exact-schema source control; this is not a general recursive compiler.
pub struct Lowered {
    schema: Schema,
}
pub struct Values {
    schema: Schema,
    depth: usize,
    reverse: bool,
    next: usize,
}
impl Lowered {
    pub fn new(schema: Schema, rules: Vec<Rule>) -> Result<Self, String> {
        if rules != schema.rules() {
            return Err("source outside exact schema".into());
        }
        Ok(Self { schema })
    }
    pub fn start(&self, input: Query) -> Result<Values, String> {
        let call = input
            .constraints
            .iter()
            .find(|c| c.name == "build")
            .ok_or("missing build")?;
        let mut term = call.args.first().ok_or("missing depth")?;
        let mut depth = 0;
        loop {
            match term {
                Term::App(n, xs) if n == "s" && xs.len() == 1 => {
                    depth += 1;
                    term = &xs[0];
                }
                Term::App(n, xs) if n == "z" && xs.is_empty() => break,
                _ => return Err("depth is not a closed natural".into()),
            }
        }
        if depth
            > if self.schema.family == "grow" {
                13
            } else {
                129
            }
        {
            return Err("depth outside bounded schema".into());
        }
        for reverse in [false, true] {
            if input == self.schema.query(depth, reverse) {
                return Ok(self.schema.values(depth, reverse));
            }
        }
        Err("query outside exact schema".into())
    }
}
impl Iterator for Values {
    type Item = Answer;
    fn next(&mut self) -> Option<Answer> {
        if self.next == self.schema.answer_count() {
            return None;
        }
        let mask = self.next;
        self.next += 1;
        let mut value = atom("seed");
        if self.schema.family == "grow" {
            for _ in 0..self.depth {
                value = t("pair", [value.clone(), value]);
            }
        }
        Some(Answer {
            outputs: (0..self.schema.calls)
                .map(|i| {
                    let name = if self.schema.family == "choice" && mask & (1 << i) != 0 {
                        "other"
                    } else {
                        "item"
                    };
                    (
                        format!("out{i}"),
                        t(
                            name,
                            [self.schema.key(i, self.reverse), value.clone(), v(900 + i)],
                        ),
                    )
                })
                .collect(),
            residual: vec![],
        })
    }
}
