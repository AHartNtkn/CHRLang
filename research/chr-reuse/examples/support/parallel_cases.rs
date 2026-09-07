use chr_cases::{Case, answer, query};
use chr_syntax::{Goal, Rule, Term, and, atom, c, eq, or, t, v};

pub fn ids() -> Vec<String> {
    [
        "wide-cheap",
        "wide-medium",
        "wide-large",
        "wide-identity",
        "chain-large",
        "skew-first",
        "skew-last",
        "mixed",
        "prefix-drain",
        "app-sk-duplication",
        "app-type-synthesis-prefix",
    ]
    .into_iter()
    .map(str::to_owned)
    .chain(distinct_ids())
    .collect()
}
fn tree(depth: usize, leaf: Term) -> Term {
    if depth == 0 {
        leaf
    } else {
        let child = tree(depth - 1, leaf);
        t("node", [child.clone(), child])
    }
}
fn choices(mut goals: Vec<Goal>) -> Goal {
    assert!(!goals.is_empty());
    if goals.len() == 1 {
        return goals.pop().unwrap();
    }
    let right = goals.split_off(goals.len() / 2);
    or(choices(goals), choices(right))
}
fn distinct_tree(depth: usize, next: &mut u64) -> Term {
    if depth == 0 {
        let result = v(*next);
        *next += 1;
        result
    } else {
        t(
            "node",
            [
                distinct_tree(depth - 1, next),
                distinct_tree(depth - 1, next),
            ],
        )
    }
}
fn distinct_case(id: &str) -> Case {
    let parts = id.split('-').collect::<Vec<_>>();
    assert_eq!(parts.len(), 4);
    let chain = match parts[1] {
        "chain" => true,
        "wide" => false,
        _ => panic!("shape"),
    };
    assert_eq!(parts[2], "d");
    let depth: usize = parts[3].parse().unwrap();
    assert!([4, 6, 8].contains(&depth));
    let mut goals = Vec::new();
    let mut expected = Vec::new();
    let mut next = 1;
    for i in 0..8 {
        if !chain {
            next = 1;
        }
        let equation = eq(distinct_tree(depth, &mut next), tree(depth, atom("a")));
        let tag = atom(&format!("answer{i}"));
        if chain {
            goals.push(equation);
        } else {
            goals.push(and([eq(v(0), tag.clone()), equation]));
            expected.push(answer(vec![tag, atom("a")], vec![]));
        }
    }
    let body = if chain {
        goals.insert(0, eq(v(0), atom("chain")));
        expected.push(answer(vec![atom("chain"), atom("a")], vec![]));
        and(goals)
    } else {
        choices(goals)
    };
    Case {
        id: id.into(),
        rules: vec![Rule::simplify("start", [c("start", [v(0), v(1)])], body)],
        query: query(vec![c("start", [v(0), v(1)])], &[0, 1]),
        raw_answers: expected.len() as u64,
        expected,
        exhausted: true,
        budget: 100_000,
        answer_limit: None,
    }
}
pub fn distinct_ids() -> Vec<String> {
    [4, 6, 8]
        .into_iter()
        .flat_map(|d| ["wide", "chain"].map(|s| format!("distinct-{s}-d-{d}")))
        .collect()
}
pub fn case(id: &str) -> Case {
    if id.starts_with("distinct-") {
        return distinct_case(id);
    }
    if id.starts_with("app-") {
        // These unchanged E00 fixtures are selected before measurement. Registry
        // construction affects process RSS; the pilot must report this limitation.
        return chr_cases::registry()
            .into_iter()
            .find(|c| c.id == id)
            .expect("application fixture");
    }
    assert!(ids().iter().any(|name| name == id));
    let mut expected = Vec::new();
    let mut goals = Vec::new();
    let width = 8;
    for i in 0..width {
        let tag = atom(&format!("answer{i}"));
        let depth = match id {
            "wide-cheap" | "prefix-drain" => 0,
            "wide-medium" | "mixed" => 6,
            "skew-first" => {
                if i == 0 {
                    10
                } else {
                    0
                }
            }
            "skew-last" => {
                if i == width - 1 {
                    10
                } else {
                    0
                }
            }
            _ => 10,
        };
        let left = tree(depth, v(1));
        let right = if id == "wide-identity" {
            tree(depth, v(1))
        } else {
            tree(depth, atom("a"))
        };
        let equation = if id == "mixed" && i % 3 == 1 {
            eq(t("pair", [left, atom("x")]), t("pair", [right, atom("y")]))
        } else if id == "mixed" && i % 3 == 2 {
            eq(v(1), t("loop", [v(1)]))
        } else {
            eq(left, right)
        };
        let mut branch = vec![eq(v(0), tag.clone()), equation];
        if id == "prefix-drain" && i != 0 {
            branch.push(eq(v(1), atom("a")));
        }
        goals.push(and(branch));
        if id != "mixed" || i % 3 == 0 {
            expected.push(answer(
                vec![
                    tag,
                    if id == "wide-identity" {
                        v(0)
                    } else {
                        atom("a")
                    },
                ],
                vec![],
            ));
        }
    }
    let body = if id == "chain-large" {
        // All equations are on one continuation; no branch service can overlap.
        let operations = (0..width).map(|i| eq(tree(10, v(i as u64 + 1)), tree(10, atom("a"))));
        expected = vec![answer(vec![atom("chain"), atom("a")], vec![])];
        and(std::iter::once(eq(v(0), atom("chain")))
            .chain(operations)
            .collect::<Vec<_>>())
    } else {
        choices(goals)
    };
    let prefix = id == "prefix-drain";
    if prefix {
        expected.truncate(1);
    }
    Case {
        id: id.into(),
        rules: vec![Rule::simplify("start", [c("start", [v(0), v(1)])], body)],
        query: query(vec![c("start", [v(0), v(1)])], &[0, 1]),
        raw_answers: expected.len() as u64,
        expected,
        exhausted: !prefix,
        budget: 100_000,
        answer_limit: prefix.then_some(1),
    }
}
