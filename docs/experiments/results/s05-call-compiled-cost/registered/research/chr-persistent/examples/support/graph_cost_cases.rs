//! Actual-source A6 fixtures. Expected trees are built independently of exports.
//! DAG leaves remain free. Tags follow payloads in the output list. Symmetric
//! repeated streams contain two classes; distinct tags can bypass residual search.
use chr_cases::Case;
use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, and, atom, c, eq, or, t, v};
pub fn ids() -> Vec<String> {
    [
        ("dag", 4),
        ("dag", 8),
        ("unary", 16),
        ("unary", 64),
        ("symmetric", 4),
        ("symmetric", 8),
        ("retention", 64),
        ("retention", 1024),
    ]
    .into_iter()
    .flat_map(|(f, n)| [format!("{f}-{n}-repeat"), format!("{f}-{n}-distinct")])
    .collect()
}
fn balanced(mut goals: Vec<Goal>) -> Goal {
    assert!(!goals.is_empty() && goals.len().is_power_of_two());
    while goals.len() > 1 {
        let mut next = Vec::with_capacity(goals.len() / 2);
        let mut iter = goals.into_iter();
        while let Some(a) = iter.next() {
            next.push(or(a, iter.next().unwrap()));
        }
        goals = next;
    }
    goals.pop().unwrap()
}
fn unary(n: usize) -> Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
fn tree(n: usize) -> Term {
    if n == 0 {
        v(900)
    } else {
        t("pair", [tree(n - 1), tree(n - 1)])
    }
}
fn residual(n: usize, split: bool) -> Vec<chr_syntax::Constraint> {
    (0..n)
        .map(|i| {
            let width = if split { n / 2 } else { n };
            let base = (i / width) * width;
            c(
                "edge",
                [v(100 + i as u64), v(100 + (base + (i + 1) % width) as u64)],
            )
        })
        .collect()
}
fn parameters(id: &str) -> (&str, usize, bool) {
    let parts = id.split('-').collect::<Vec<_>>();
    assert_eq!(parts.len(), 3, "unknown graph cost case {id}");
    let n: usize = parts[1].parse().expect("size");
    let distinct = match parts[2] {
        "repeat" => false,
        "distinct" => true,
        _ => panic!("unknown case {id}"),
    };
    assert!(
        matches!(
            (parts[0], n),
            ("dag", 4 | 8) | ("unary", 16 | 64) | ("symmetric", 4 | 8) | ("retention", 64 | 1024)
        ),
        "unknown case {id}"
    );
    (parts[0], n, distinct)
}
fn tag(distinct: bool, i: usize) -> Term {
    if distinct {
        t("tag", [atom(&format!("id{i}"))])
    } else {
        atom("repeat")
    }
}
/// Source syntax preparation only: no expected-answer construction or execution.
pub fn build_source(id: &str) -> (Vec<Rule>, Query) {
    let (family, n, distinct) = parameters(id);
    let mut initial = vec![];
    let (input, head) = match family {
        "dag" => {
            for i in 0..n {
                initial.push(eq(
                    v(if i == 0 { 0 } else { (i + 1) as u64 }),
                    t("pair", [v((i + 2) as u64), v((i + 2) as u64)]),
                ));
            }
            (c("start", [v(0), v(1)]), c("start", [v(0), v(1)]))
        }
        "unary" => {
            initial.push(eq(v(0), unary(n)));
            (c("start", [v(0), v(1)]), c("start", [v(0), v(1)]))
        }
        "symmetric" => {
            initial.push(eq(v(0), atom("unit")));
            (c("start", [v(0), v(1)]), c("start", [v(0), v(1)]))
        }
        "retention" => {
            initial.push(eq(v(0), atom("unit")));
            let payload = t(
                "payload",
                (0..n)
                    .map(|i| atom(&format!("junk{i}")))
                    .collect::<Vec<_>>(),
            );
            (
                c("start", [v(0), v(1), payload]),
                c("start", [v(0), v(1), v(2)]),
            )
        }
        _ => unreachable!(),
    };
    let alternatives = (0..32)
        .map(|i| {
            let mut goals = vec![eq(v(1), tag(distinct, i))];
            if family == "symmetric" {
                goals.extend(residual(n, i % 2 == 1).into_iter().map(Goal::Constraint));
            }
            and(goals)
        })
        .collect();
    initial.push(balanced(alternatives));
    (
        vec![Rule::simplify("start", [head], and(initial))],
        Query {
            constraints: vec![input],
            outputs: vec![("payload".into(), Var(0)), ("tag".into(), Var(1))],
        },
    )
}
/// Independent full expected answers; does not use either source evaluator or exporter.
pub fn expected(id: &str) -> Vec<Answer> {
    let (family, n, distinct) = parameters(id);
    let payload = match family {
        "dag" => tree(n),
        "unary" => unary(n),
        _ => atom("unit"),
    };
    let count = if distinct {
        32
    } else if family == "symmetric" {
        2
    } else {
        1
    };
    (0..count)
        .map(|i| Answer {
            outputs: vec![
                ("payload".into(), payload.clone()),
                ("tag".into(), tag(distinct, i)),
            ],
            residual: if family == "symmetric" {
                residual(n, i % 2 == 1)
            } else {
                vec![]
            },
        })
        .collect()
}
pub fn case(id: &str) -> Case {
    let (rules, query) = build_source(id);
    Case {
        id: id.into(),
        rules,
        query,
        expected: expected(id),
        exhausted: true,
        raw_answers: 32,
        budget: 100_000,
        answer_limit: None,
    }
}
