use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, and, atom, c, eq, or, t, v};
#[derive(Clone, Copy)]
pub struct Schema {
    pub family: &'static str,
    pub branches: usize,
    pairs: usize,
    resource: bool,
}
impl Schema {
    pub fn new(family: &str, resource: bool) -> Self {
        let (family, branches, pairs) = match family {
            "single" => ("single", 1, 1),
            "shared" => ("shared", 4, 1),
            "distinct" => ("distinct", 4, 4),
            "changed" => ("changed", 4, 1),
            _ => panic!("unknown family"),
        };
        Self {
            family,
            branches,
            pairs,
            resource,
        }
    }
    pub fn rules(self) -> Vec<Rule> {
        let context = v((self.pairs * 2) as u64);
        let branches = (0..self.branches)
            .map(|i| {
                let pair = if self.family == "distinct" { i } else { 0 };
                let tag = atom(&format!("branch{i}"));
                let mut goals = vec![c("tag", [tag.clone()]).into()];
                if self.family == "changed" {
                    goals.push(eq(context.clone(), tag));
                }
                goals.push(eq(v((pair * 2) as u64), v((pair * 2 + 1) as u64)));
                if self.resource {
                    goals.push(c("take", []).into());
                }
                and(goals)
            })
            .collect::<Vec<_>>();
        let body = if self.branches == 1 {
            branches[0].clone()
        } else {
            or(
                or(branches[0].clone(), branches[1].clone()),
                or(branches[2].clone(), branches[3].clone()),
            )
        };
        let mut rules = vec![Rule::simplify(
            "start",
            [c(
                "start",
                (0..=self.pairs * 2)
                    .map(|i| v(i as u64))
                    .collect::<Vec<_>>(),
            )],
            body,
        )];
        if self.resource {
            rules.push(Rule::simplify(
                "take",
                [c("take", []), c("token", [])],
                Goal::True,
            ));
        }
        rules
    }
    pub fn query(self, depth: usize, reverse: bool) -> Query {
        assert!(depth <= 129);
        let mut args = vec![];
        let mut outputs = vec![];
        for i in 0..self.pairs {
            let id = 1000 + i as u64;
            let left = (0..depth).fold(v(id), |x, _| t("f", [x]));
            let right = (0..depth).fold(atom("a"), |x, _| t("f", [x]));
            args.extend([left, right]);
            outputs.push((format!("x{i}"), Var(id)));
        }
        args.push(v(9000));
        outputs.push(("context".into(), Var(9000)));
        let mut constraints = vec![c("start", args)];
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
    pub fn values(self) -> Values {
        Values {
            schema: self,
            next: 0,
        }
    }
}
pub struct Lowered {
    schema: Schema,
}
pub struct Values {
    schema: Schema,
    next: usize,
}
impl Lowered {
    pub fn new(schema: Schema, rules: Vec<Rule>) -> Result<Self, &'static str> {
        if rules != schema.rules() {
            return Err("source outside exact schema");
        }
        Ok(Self { schema })
    }
    pub fn start(&self, query: Query) -> Result<Values, &'static str> {
        let start = query
            .constraints
            .iter()
            .find(|c| c.name == "start")
            .ok_or("missing start")?;
        let mut at = start.args.first().ok_or("missing argument")?;
        let mut depth = 0;
        while let Term::App(n, xs) = at {
            if n != "f" || xs.len() != 1 {
                return Err("source input shape");
            }
            depth += 1;
            if depth > 129 {
                return Err("depth bound");
            }
            at = &xs[0];
        }
        if query != self.schema.query(depth, false) && query != self.schema.query(depth, true) {
            return Err("query outside exact schema");
        }
        Ok(self.schema.values())
    }
}
impl Iterator for Values {
    type Item = Answer;
    fn next(&mut self) -> Option<Answer> {
        if self.next == self.schema.branches {
            return None;
        }
        let branch = self.next;
        self.next += 1;
        let tag = atom(&format!("branch{branch}"));
        let mut outputs = (0..self.schema.pairs)
            .map(|i| {
                (
                    format!("x{i}"),
                    if self.schema.family != "distinct" || i == branch {
                        atom("a")
                    } else {
                        v(1000 + i as u64)
                    },
                )
            })
            .collect::<Vec<_>>();
        outputs.push((
            "context".into(),
            if self.schema.family == "changed" {
                tag.clone()
            } else {
                v(9000)
            },
        ));
        Some(Answer {
            outputs,
            residual: vec![c("tag", [tag])],
        })
    }
}
