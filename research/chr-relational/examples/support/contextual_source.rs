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
