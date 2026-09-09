use chr_syntax::{Query, Rule, Var, atom, c, eq, or, t, v};
#[derive(Clone, Copy)]
pub struct Schema {
    pub bits: usize,
    calls: usize,
    steps: usize,
    choice: bool,
    resource: bool,
    failure: bool,
}
impl Schema {
    pub fn new(family: &str, resource: bool, steps: usize) -> Self {
        let (calls, choice) = match family {
            "plain1" => (1, false),
            "plain4" => (4, false),
            "plain16" => (16, false),
            "fail1" => (1, false),
            "fail4" => (4, false),
            "choice1" => (1, true),
            "choice4" => (4, true),
            _ => panic!("unknown family"),
        };
        assert!(steps > 0);
        Self {
            bits: if choice { calls } else { 0 },
            calls,
            steps,
            choice,
            resource,
            failure: family.starts_with("fail"),
        }
    }
    pub fn answer_count(self) -> usize {
        if self.failure { 0 } else { 1 << self.bits }
    }
    pub fn rules(self) -> Vec<Rule> {
        let mut rules = vec![];
        for i in 0..self.steps {
            let body = if i + 1 == self.steps {
                let a = eq(v(1), t("f", [v(0)]));
                if self.choice {
                    or(a, eq(v(1), t("g", [v(0)])))
                } else {
                    a
                }
            } else {
                c(&format!("stage{}", i + 1), [v(0), v(1)]).into()
            };
            rules.push(Rule::simplify(
                &format!("stage{i}"),
                [c(&format!("stage{i}"), [v(0), v(1)])],
                body,
            ));
        }
        if self.resource {
            rules.push(Rule::simplify(
                "consume",
                [c("ready", [t("f", [v(0)])]), c("token", [v(0)])],
                c("done", [v(0)]).into(),
            ));
        }
        rules
    }
    pub fn query(self, seed: usize, reverse: bool) -> Query {
        let mut constraints = vec![];
        let mut outputs = vec![];
        for i in 0..self.calls {
            let key = atom(&format!("key{seed}_{i}"));
            let id = 100 + i as u64;
            constraints.push(c(
                "stage0",
                [
                    key.clone(),
                    if self.failure { atom("clash") } else { v(id) },
                ],
            ));
            constraints.push(c("ready", [v(id)]));
            if self.resource {
                constraints.push(c("token", [key]));
            }
            outputs.push((format!("out{i}"), Var(id)));
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
