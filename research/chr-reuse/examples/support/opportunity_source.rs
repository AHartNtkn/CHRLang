use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, and, atom, c, eq, or, t, v};
#[derive(Clone, Copy)]
pub struct Schema {
    pub family: &'static str,
    pub resource: bool,
    pub alternatives: usize,
    pub payload: usize,
}
impl Schema {
    pub fn new(family: &str, resource: bool, alternatives: usize, payload: usize) -> Self {
        assert!(matches!(alternatives, 1 | 4 | 16 | 64));
        assert!(payload <= 128);
        Self {
            family: match family {
                "exact" => "exact",
                "rename" => "rename",
                "history" => "history",
                "history-early" => "history-early",
                "distinct" => "distinct",
                _ => panic!("family"),
            },
            resource,
            alternatives,
            payload,
        }
    }
    pub fn rules(self) -> Vec<Rule> {
        let branches = (0..self.alternatives)
            .map(|i| {
                let local = 1000 + 8 * i as u64;
                let seed = if self.family == "distinct" {
                    atom(&format!("tag{i}"))
                } else {
                    v(if self.family == "exact" { 100 } else { local })
                };
                let call: Goal = c("work", [v(0), seed, v(1), v(2)]).into();
                if matches!(self.family, "history" | "history-early") {
                    let mut gs = (0..i % 4)
                        .map(|j| c("trash", [v(local + 1), v(local + 2 + j as u64)]).into())
                        .collect::<Vec<_>>();
                    gs.push(call);
                    and(gs)
                } else {
                    call
                }
            })
            .collect::<Vec<_>>();
        fn choice(xs: &[Goal]) -> Goal {
            if xs.len() == 1 {
                xs[0].clone()
            } else {
                let mid = xs.len() / 2;
                or(choice(&xs[..mid]), choice(&xs[mid..]))
            }
        }
        let mut rules = vec![
            Rule::simplify("start", [c("start", [v(0), v(1), v(2)])], choice(&branches)),
            Rule::simplify(
                "step",
                [c("work", [t("s", [v(0)]), v(1), v(2), v(3)])],
                c("work", [v(0), v(1), v(2), v(3)]).into(),
            ),
            Rule::simplify(
                "base",
                [c("work", [atom("z"), v(0), v(1), v(2)])],
                eq(v(2), t("box", [v(0)])),
            ),
            Rule::simplify("trash", [c("trash", [v(0), v(1)])], eq(v(1), atom("unit"))),
        ];
        if self.family == "history-early" {
            let trash = rules.remove(3);
            rules.insert(1, trash);
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
    pub fn query(self, n: usize, reverse: bool) -> Query {
        let depth = (0..n).fold(atom("z"), |a, _| t("s", [a]));
        let payload = (0..self.payload).fold(atom("end"), |a, _| t("data", [a]));
        let mut constraints = vec![c("start", [depth, payload, v(99)])];
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
        self.alternatives
    }
    pub fn expected(self) -> Vec<Answer> {
        (0..self.alternatives)
            .map(|i| Answer {
                outputs: vec![(
                    "result".into(),
                    t(
                        "box",
                        [if self.family == "distinct" {
                            atom(&format!("tag{i}"))
                        } else {
                            v(i as u64)
                        }],
                    ),
                )],
                residual: vec![],
            })
            .collect()
    }
}
pub struct Lowered(Schema);
impl Lowered {
    pub fn new(schema: Schema, rules: Vec<Rule>) -> Self {
        assert_eq!(
            rules,
            schema.rules(),
            "exact-source control requires registered rules"
        );
        Self(schema)
    }
    pub fn start(&self, input: Query) -> std::vec::IntoIter<Answer> {
        let start = input
            .constraints
            .iter()
            .find(|c| c.name == "start")
            .expect("start");
        let mut term = &start.args[0];
        let mut n = 0;
        while let Term::App(name, args) = term {
            if name == "s" && args.len() == 1 {
                n += 1;
                term = &args[0];
            } else {
                break;
            }
        }
        assert!(n <= 129);
        assert_eq!(term, &atom("z"));
        assert!(
            input == self.0.query(n, false) || input == self.0.query(n, true),
            "exact-source query shape"
        );
        self.0.expected().into_iter()
    }
}
