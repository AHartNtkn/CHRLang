use chr_syntax::{Goal, Query, Rule, Var, atom, c, eq, or, t, v};
#[derive(Clone, Copy)]
pub struct Schema {
    pub bits: usize,
    local: bool,
    resource: bool,
}
impl Schema {
    pub fn new(family: &str, resource: bool) -> Self {
        let (bits, local) = match family {
            "shared0" => (0, false),
            "shared3" => (3, false),
            "local0" => (0, true),
            "local3" => (3, true),
            _ => panic!("unknown family"),
        };
        Self {
            bits,
            local,
            resource,
        }
    }
    pub fn rules(self) -> Vec<Rule> {
        let mut rules = vec![
            Rule::simplify("init", [c("init", [v(0), v(1)])], eq(v(1), v(0))),
            Rule::simplify(
                "choose",
                [c("choose", [v(0)])],
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
            ),
            Rule::simplify("patch", [c("patch", [v(0), v(1)])], eq(v(1), v(0))),
        ];
        if self.resource {
            rules.push(Rule::simplify(
                "eat",
                [c("eat", [v(0)]), c("token", [v(0)])],
                Goal::True,
            ));
        }
        rules
    }
    pub fn query(self, n: usize, reverse: bool) -> Query {
        let mut payload = atom("leaf");
        for _ in 0..n {
            payload = t("f", [payload]);
        }
        let mut constraints = vec![c("init", [payload, v(1000)])];
        let mut outputs = vec![("payload".into(), Var(1000))];
        for i in 0..self.bits {
            let id = 100 + i as u64;
            constraints.push(c("choose", [v(id)]));
            outputs.push((format!("choice{i}"), Var(id)));
        }
        for i in 0..8 {
            let value = v(2000 + i as u64);
            constraints.push(c("hold", [value.clone()]));
            if self.local {
                constraints.push(c(
                    "patch",
                    [if self.bits == 0 { atom("a") } else { v(100) }, value],
                ));
            }
            if self.resource {
                let key = atom(&format!("key{i}"));
                constraints.push(c("eat", [key.clone()]));
                constraints.push(c("token", [key]));
            }
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

/// Exact-schema derivation, not a general source compiler.
pub struct Lowered {
    schema: Schema,
}
pub struct Values {
    schema: Schema,
    payload: chr_syntax::Term,
    mask: usize,
}
impl Lowered {
    pub fn new(schema: Schema, rules: Vec<Rule>) -> Result<Self, &'static str> {
        if rules != schema.rules() {
            return Err("source outside exact schema");
        }
        Ok(Self { schema })
    }
    pub fn start(&self, query: Query) -> Result<Values, &'static str> {
        let init = query
            .constraints
            .iter()
            .find(|c| c.name == "init")
            .ok_or("missing init")?;
        let mut term = init.args.first().ok_or("missing payload")?;
        let mut depth = 0;
        loop {
            match term {
                chr_syntax::Term::App(n, xs) if n == "f" && xs.len() == 1 => {
                    depth += 1;
                    term = &xs[0];
                }
                chr_syntax::Term::App(n, xs) if n == "leaf" && xs.is_empty() => break,
                _ => return Err("payload outside schema"),
            }
        }
        if query != self.schema.query(depth, false) && query != self.schema.query(depth, true) {
            return Err("query outside exact schema");
        }
        Ok(Values {
            schema: self.schema,
            payload: init.args[0].clone(),
            mask: 0,
        })
    }
}
impl Iterator for Values {
    type Item = chr_syntax::Answer;
    fn next(&mut self) -> Option<Self::Item> {
        if self.mask == 1 << self.schema.bits {
            return None;
        }
        let choice = |i: usize| {
            atom(if self.mask & (1usize << i) == 0 {
                "a"
            } else {
                "b"
            })
        };
        let mut outputs = vec![("payload".into(), self.payload.clone())];
        for i in 0..self.schema.bits {
            outputs.push((format!("choice{i}"), choice(i)));
        }
        let residual = (0..8)
            .map(|i| {
                c(
                    "hold",
                    [if self.schema.local {
                        if self.schema.bits == 0 {
                            atom("a")
                        } else {
                            choice(0)
                        }
                    } else {
                        v(2000 + i)
                    }],
                )
            })
            .collect();
        self.mask += 1;
        Some(chr_syntax::Answer { outputs, residual })
    }
}
