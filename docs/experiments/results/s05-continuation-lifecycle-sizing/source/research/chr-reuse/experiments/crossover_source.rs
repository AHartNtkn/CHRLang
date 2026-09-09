//! Source-size/request crossover. Choice count changes source rule count too;
//! preparation and source selection are intentionally part of the comparison.
use chr_syntax::{Answer, Goal, Query, Rule, Term, and, atom, c, eq, t, v};
#[derive(Clone, Copy)]
pub struct Config {
    pub before: bool,
    pub clash: bool,
    pub unique: bool,
    pub depth: usize,
    pub bits: usize,
}
impl Config {
    pub fn parse(name: &str) -> Option<Self> {
        if !name.starts_with("cross-") {
            return None;
        }
        let parts: Vec<_> = name.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert!(["before", "after", "unique"].contains(&parts[1]));
        assert!(["success", "clash"].contains(&parts[2]));
        let depth = parts[3].parse().unwrap();
        let bits = parts[4].parse().unwrap();
        assert!([64, 256, 1024].contains(&depth) && [2, 4, 6].contains(&bits));
        Some(Self {
            before: parts[1] == "before",
            unique: parts[1] == "unique",
            clash: parts[2] == "clash",
            depth,
            bits,
        })
    }
    fn tuple(self, key: usize) -> Term {
        t(
            "tuple",
            (0..self.bits)
                .map(|i| atom(if key & (1 << i) == 0 { "a" } else { "b" }))
                .collect::<Vec<_>>(),
        )
    }
    pub fn rules(self) -> Vec<Rule> {
        let mut body: Vec<Goal> = (3..3 + self.bits).map(|i| or_pair(i as u64)).collect();
        body.push(
            c(
                if self.before { "verify" } else { "gate" },
                [
                    v(0),
                    v(1),
                    t(
                        "tuple",
                        (3..3 + self.bits).map(|i| v(i as u64)).collect::<Vec<_>>(),
                    ),
                    v(2),
                ],
            )
            .into(),
        );
        let mut rules = vec![
            Rule::simplify("choose", [c("start", [v(0), v(1), v(2)])], and(body)),
            Rule::simplify(
                "verify",
                [c("verify", [v(0), v(1), v(2), v(3)])],
                and(vec![
                    eq(v(0), v(1)),
                    c("gate", [v(0), v(1), v(2), v(3)]).into(),
                ]),
            ),
        ];
        for key in 0..1 << self.bits {
            let tuple = self.tuple(key);
            let mut body = vec![];
            if !self.before {
                body.push(if self.unique {
                    eq(
                        t("tag", [tuple.clone(), v(0)]),
                        t("tag", [tuple.clone(), v(1)]),
                    )
                } else {
                    eq(v(0), v(1))
                });
            }
            body.extend([
                c("witness", [tuple.clone(), v(2), v(2)]).into(),
                c("done", [v(2), v(2)]).into(),
            ]);
            rules.push(Rule::simplify(
                &format!("gate-{key}"),
                [c("gate", [v(0), v(1), tuple, v(2)])],
                and(body),
            ));
        }
        rules
    }
    pub fn query(self, seed: usize) -> Query {
        let left = (0..self.depth).fold(t("tip", [v(10), v(10), atom("end")]), |tail, _| {
            t("row", [v(10), tail])
        });
        let right = (0..self.depth).fold(
            t(
                "tip",
                [v(11), v(12), atom(if self.clash { "clash" } else { "end" })],
            ),
            |tail, _| t("row", [v(11), tail]),
        );
        Query {
            constraints: vec![
                c("start", [left, right, v(10)]),
                c("query_tag", [atom(&format!("q{seed}"))]),
            ],
            outputs: [("x", 10), ("y", 11), ("z", 12), ("unused", 99)]
                .into_iter()
                .map(|(n, id)| (n.into(), chr_syntax::Var(id)))
                .collect(),
        }
    }
    pub fn expected(self, seed: usize) -> Vec<Answer> {
        if self.clash {
            return vec![];
        }
        (0..1 << self.bits)
            .map(|key| Answer {
                outputs: vec![
                    ("x".into(), v(10)),
                    ("y".into(), v(10)),
                    ("z".into(), v(10)),
                    ("unused".into(), v(99)),
                ],
                residual: vec![
                    c("query_tag", [atom(&format!("q{seed}"))]),
                    c("witness", [self.tuple(key), v(10), v(10)]),
                    c("done", [v(10), v(10)]),
                ],
            })
            .collect()
    }
}
fn or_pair(i: u64) -> Goal {
    chr_syntax::or(eq(v(i), atom("a")), eq(v(i), atom("b")))
}
