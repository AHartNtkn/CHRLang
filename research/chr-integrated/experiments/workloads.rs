//! Source-level workloads and independently constructed complete observations.
use chr_syntax::{Answer, Query, Rule, Term, Var, and, atom, c, eq, t, v};
#[derive(Clone, Copy, Debug)]
pub enum Family {
    Independent,
    Fanout,
    Repair,
    Build,
    Batch,
    Nested,
}
impl Family {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "independent" => Some(Self::Independent),
            "fanout" => Some(Self::Fanout),
            "repair" => Some(Self::Repair),
            "build" => Some(Self::Build),
            "batch" => Some(Self::Batch),
            "nested" => Some(Self::Nested),
            _ => None,
        }
    }
    pub fn program(self) -> usize {
        match self {
            Self::Build => 0,
            Self::Nested => 2,
            _ => 1,
        }
    }
}
pub fn programs() -> Vec<Vec<Rule>> {
    let mut programs = vec![
        vec![
            Rule::simplify(
                "zero",
                [c("build", [atom("z"), v(0)])],
                eq(v(0), atom("nil")),
            ),
            Rule::simplify(
                "step",
                [c("build", [t("s", [v(0)]), v(1)])],
                and(vec![
                    eq(v(1), t("cons", [atom("item"), v(2)])),
                    c("build", [v(0), v(2)]).into(),
                ]),
            ),
        ],
        vec![
            Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
            Rule::simplify(
                "open",
                [c("open", [v(0), t("f", [v(1)])]), c("ticket", [v(0)])],
                c("result", [v(0), v(1)]).into(),
            ),
            Rule::propagate(
                "seen",
                [c("result", [v(0), v(1)])],
                c("seen", [v(0), v(1)]).into(),
            ),
        ],
    ];
    let mut structural = programs[1].clone();
    structural[1].removed[0].args[1] = t("f", [t("g", [v(1)])]);
    programs.push(structural);
    programs
}
fn nested(depth: usize) -> Term {
    (0..depth).fold(atom("a"), |tail, _| t("f", [tail]))
}
pub fn case(family: Family, n: usize, depth: usize) -> (Query, Answer) {
    assert!(depth > 0);
    if matches!(family, Family::Build) {
        return (
            Query {
                constraints: vec![c(
                    "build",
                    [(0..n).fold(atom("z"), |tail, _| t("s", [tail])), v(0)],
                )],
                outputs: vec![("out".into(), Var(0))],
            },
            Answer {
                outputs: vec![(
                    "out".into(),
                    (0..n).fold(atom("nil"), |tail, _| t("cons", [atom("item"), tail])),
                )],
                residual: vec![],
            },
        );
    }
    if matches!(family, Family::Batch | Family::Nested) {
        let structural = matches!(family, Family::Nested);
        let value = if structural {
            (0..depth).fold(atom("a"), |tail, _| t("g", [tail]))
        } else {
            nested(depth)
        };
        let result = if structural {
            (0..depth - 1).fold(atom("a"), |tail, _| t("g", [tail]))
        } else {
            nested(depth - 1)
        };
        let mut constraints = vec![];
        let mut outputs = vec![];
        let mut values = vec![];
        let mut residual = vec![];
        for i in 0..n {
            let key = atom(&format!("k{i:016x}"));
            let var = v(i as u64);
            constraints.extend([
                c(
                    "open",
                    [key.clone(), if structural { t("f", [var]) } else { var }],
                ),
                c("ticket", [key.clone()]),
            ]);
            outputs.push((format!("o{i}"), Var(i as u64)));
            values.push((format!("o{i}"), value.clone()));
            residual.extend([
                c("result", [key.clone(), result.clone()]),
                c("seen", [key, result.clone()]),
            ]);
        }
        constraints.push(c(
            "bind",
            [
                t("tuple", (0..n).map(|i| v(i as u64)).collect::<Vec<_>>()),
                t("tuple", vec![value; n]),
            ],
        ));
        return (
            Query {
                constraints,
                outputs,
            },
            Answer {
                outputs: values,
                residual,
            },
        );
    }
    let fanout = matches!(family, Family::Fanout | Family::Repair);
    let yields = !matches!(family, Family::Repair);
    let value = if yields {
        nested(depth)
    } else {
        t("g", [nested(depth - 1)])
    };
    let mut constraints = vec![];
    let mut outputs = vec![];
    let mut values = vec![];
    let mut residual = vec![];
    for i in 0..n {
        let key = atom(&format!("k{i:016x}"));
        let var = if fanout { 0 } else { i as u64 };
        constraints.push(c("open", [key.clone(), v(var)]));
        constraints.push(c("ticket", [key.clone()]));
        outputs.push((format!("o{i}"), Var(var)));
        values.push((format!("o{i}"), value.clone()));
        if yields {
            residual.extend([
                c("result", [key.clone(), nested(depth - 1)]),
                c("seen", [key, nested(depth - 1)]),
            ]);
        } else {
            residual.extend([c("open", [key.clone(), value.clone()]), c("ticket", [key])]);
        }
    }
    for i in 0..if fanout { 1 } else { n } {
        constraints.push(c("bind", [v(i as u64), value.clone()]));
    }
    (
        Query {
            constraints,
            outputs,
        },
        Answer {
            outputs: values,
            residual,
        },
    )
}
