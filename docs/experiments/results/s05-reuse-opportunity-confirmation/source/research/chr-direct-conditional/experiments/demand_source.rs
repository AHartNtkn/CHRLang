use chr_syntax::{Answer, Query, Rule, Term, Var, and, atom, c, eq, or, t, v};
pub fn rules(discriminate: bool, bits: usize, resource: bool) -> Vec<Rule> {
    let mut rules = vec![
        Rule::simplify(
            "choose",
            [c("choose", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify("base", [c("work", [atom("z"), v(0), v(1)])], eq(v(1), v(0))),
        Rule::simplify(
            "step",
            [c("work", [t("s", [v(0)]), v(1), v(2)])],
            and([
                c("work", [v(0), v(1), v(3)]).into(),
                eq(v(2), t("box", [v(3)])),
            ]),
        ),
    ];
    if discriminate {
        for mask in 0..1 << bits {
            let value = t(
                "pack",
                (0..bits)
                    .map(|i| atom(if mask & (1 << i) == 0 { "a" } else { "b" }))
                    .collect::<Vec<_>>(),
            );
            rules.push(Rule::simplify(
                &format!("gate{mask}"),
                [c("gate", [v(0), value.clone(), v(1)])],
                c("work", [v(0), value, v(1)]).into(),
            ));
        }
    }
    if resource {
        rules.push(Rule::simplify(
            "take",
            [c("take", [v(0), v(1)]), c("token", [])],
            eq(v(1), v(0)),
        ))
    }
    rules
}

#[derive(Clone, Copy)]
pub struct Schema {
    pub bits: usize,
    pub discriminate: bool,
    pub resource: bool,
}
impl Schema {
    pub fn new(family: &str, resource: bool) -> Self {
        assert!(["plain", "opaque", "discriminate"].contains(&family));
        Self {
            bits: if family == "plain" { 0 } else { 3 },
            discriminate: family == "discriminate",
            resource,
        }
    }
    pub fn rules(self) -> Vec<Rule> {
        rules(self.discriminate, self.bits, self.resource)
    }
    pub fn query(self, n: usize, first: bool) -> Query {
        let depth = (0..n).fold(atom("z"), |x, _| t("s", [x]));
        let work = c(
            if self.discriminate { "gate" } else { "work" },
            [
                depth,
                t(
                    "pack",
                    (0..self.bits)
                        .map(|i| v(100 + i as u64))
                        .collect::<Vec<_>>(),
                ),
                v(1000),
            ],
        );
        let mut constraints = (0..self.bits)
            .map(|i| c("choose", [v(100 + i as u64)]))
            .collect::<Vec<_>>();
        if first {
            constraints.insert(0, work)
        } else {
            constraints.push(work)
        }
        if self.resource {
            constraints.extend([c("take", [v(1000), v(1001)]), c("token", [])]);
        }
        let out = Var(if self.resource { 1001 } else { 1000 });
        Query {
            constraints,
            outputs: vec![("x".into(), out), ("again".into(), out)],
        }
    }
}
pub struct Lowered {
    schema: Schema,
}
pub struct Values {
    depth: usize,
    bits: usize,
    next: usize,
}
impl Lowered {
    pub fn new(schema: Schema, source: Vec<Rule>) -> Result<Self, String> {
        if source != schema.rules() {
            return Err("source does not match checked box schema".into());
        }
        Ok(Self { schema })
    }
    pub fn start(&self, input: Query) -> Result<Values, String> {
        let name = if self.schema.discriminate {
            "gate"
        } else {
            "work"
        };
        let mut matches = input.constraints.iter().filter(|c| c.name == name);
        let work = matches.next().ok_or("missing work")?;
        if matches.next().is_some() {
            return Err("multiple work roots".into());
        }
        let mut term = work.args.first().ok_or("missing depth")?;
        let mut depth = 0;
        loop {
            match term {
                Term::App(n, args) if n == "s" && args.len() == 1 => {
                    depth += 1;
                    term = &args[0]
                }
                Term::App(n, args) if n == "z" && args.is_empty() => break,
                _ => return Err("depth is not a closed natural".into()),
            }
        }
        if input != self.schema.query(depth, false) && input != self.schema.query(depth, true) {
            return Err("query outside checked box schema".into());
        }
        Ok(Values {
            depth,
            bits: self.schema.bits,
            next: 0,
        })
    }
}
impl Iterator for Values {
    type Item = Answer;
    fn next(&mut self) -> Option<Answer> {
        if self.next == 1 << self.bits {
            return None;
        }
        let mask = self.next;
        self.next += 1;
        let mut value = t(
            "pack",
            (0..self.bits)
                .map(|i| atom(if mask & (1 << i) == 0 { "a" } else { "b" }))
                .collect::<Vec<_>>(),
        );
        for _ in 0..self.depth {
            value = t("box", [value]);
        }
        Some(Answer {
            outputs: vec![("x".into(), value.clone()), ("again".into(), value)],
            residual: vec![],
        })
    }
}
