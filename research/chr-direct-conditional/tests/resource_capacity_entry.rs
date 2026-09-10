//! Independent capacity relation versus full consuming source semantics.
#[allow(dead_code)]
mod composition_support;
#[allow(dead_code)]
mod runtime_support;
use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, and, atom, c, eq, or, v};

fn source(tag: &str, weight: usize) -> Vec<Rule> {
    let mut rules = Vec::new();
    for mask in 1..=3 {
        let arms: Vec<_> = ["a", "b"]
            .into_iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .flat_map(|(_, name)| (0..weight).map(move |_| eq(v(0), atom(name))))
            .collect();
        rules.push(Rule::simplify(
            &format!("pick{mask}"),
            [c(&format!("{tag}pick{mask}"), [v(0)])],
            and(vec![
                arms.into_iter().reduce(or).unwrap(),
                c(&format!("{tag}need"), [v(0)]).into(),
            ]),
        ));
    }
    rules.push(Rule::simplify(
        "consume",
        [
            c(&format!("{tag}need"), [v(0)]),
            c(&format!("{tag}token"), [v(0)]),
        ],
        c(&format!("{tag}done"), [v(0)]).into(),
    ));
    rules.push(Rule::simplify(
        "reject",
        [c(&format!("{tag}need"), [v(0)])],
        Goal::Fail,
    ));
    rules
}
fn query(tag: &str, domains: &[usize], caps: [usize; 2], alias: bool, base: u64) -> Query {
    let mut constraints = domains
        .iter()
        .enumerate()
        .map(|(i, m)| {
            c(
                &format!("{tag}pick{m}"),
                [v(base + if alias { 0 } else { i as u64 })],
            )
        })
        .collect::<Vec<_>>();
    for (name, count) in ["a", "b"].into_iter().zip(caps) {
        for _ in 0..count {
            constraints.push(c(&format!("{tag}token"), [atom(name)]));
        }
    }
    constraints.push(c("noise", [atom("retained")]));
    let mut outputs = (0..domains.len())
        .map(|i| {
            (
                format!("x{i}"),
                Var(base + if alias { 0 } else { i as u64 }),
            )
        })
        .collect::<Vec<_>>();
    outputs.push(("unused".into(), Var(base + 100)));
    Query {
        constraints,
        outputs,
    }
}
// Exhaustive mathematical oracle only; it executes no source rule or CHR kernel.
fn relation(
    tag: &str,
    domains: &[usize],
    caps: [usize; 2],
    alias: bool,
    weight: usize,
    base: u64,
) -> Vec<Answer> {
    let mut answers = Vec::new();
    for bits in 0..(1usize << domains.len()) {
        let values = (0..domains.len())
            .map(|i| (bits >> i) & 1)
            .collect::<Vec<_>>();
        if values.iter().zip(domains).any(|(x, m)| m & (1 << x) == 0)
            || (alias && values.windows(2).any(|w| w[0] != w[1]))
        {
            continue;
        }
        let used = [
            values.iter().filter(|x| **x == 0).count(),
            values.iter().filter(|x| **x == 1).count(),
        ];
        if (0..2).any(|i| used[i] > caps[i]) {
            continue;
        }
        let names = ["a", "b"];
        let mut outputs = values
            .iter()
            .enumerate()
            .map(|(i, x)| (format!("x{i}"), atom(names[*x])))
            .collect::<Vec<_>>();
        outputs.push(("unused".into(), Term::Var(Var(base + 100))));
        let mut residual = values
            .iter()
            .map(|x| c(&format!("{tag}done"), [atom(names[*x])]))
            .collect::<Vec<_>>();
        for i in 0..2 {
            for _ in used[i]..caps[i] {
                residual.push(c(&format!("{tag}token"), [atom(names[i])]));
            }
        }
        residual.push(c("noise", [atom("retained")]));
        for _ in 0..weight.pow(domains.len() as u32) {
            answers.push(Answer {
                outputs: outputs.clone(),
                residual: residual.clone(),
            });
        }
    }
    answers
}
fn checked(rules: &[Rule], q: &Query) -> Vec<Answer> {
    let expected = runtime_support::run(rules, q, 200000);
    runtime_support::same_raw(
        composition_support::Engine::new(0, rules, q).collect(),
        expected.clone(),
    );
    expected
}
#[test]
fn capacity_relation_preserves_complete_consuming_answers() {
    let mut cases = 0;
    for n in 0..=3 {
        for profile in 0..3 {
            for a in 0..=3 {
                for b in 0..=3 {
                    for alias in [false, true] {
                        for weight in [1, 2] {
                            for (tag, base) in [("r", 10), ("renamed", 1000)] {
                                let domains = (0..n)
                                    .map(|i| match profile {
                                        0 => 3,
                                        1 => [1, 3, 2][i % 3],
                                        _ => 1,
                                    })
                                    .collect::<Vec<_>>();
                                let rules = source(tag, weight);
                                let q = query(tag, &domains, [a, b], alias, base);
                                runtime_support::same_raw(
                                    relation(tag, &domains, [a, b], alias, weight, base),
                                    checked(&rules, &q),
                                );
                                cases += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(cases, 1536);
    println!("validated {cases} complete source/capacity cases");
}
#[test]
fn alias_groups_cannot_be_split_across_resource_values() {
    let rules = source("r", 1);
    assert_eq!(
        checked(&rules, &query("r", &[3, 3], [1, 1], false, 10)).len(),
        2
    );
    assert!(checked(&rules, &query("r", &[3, 3], [1, 1], true, 10)).is_empty());
    // Both have two slots and two tokens; the aliased pair demands two of one value.
}
#[test]
fn capacity_failure_requires_a_sink_and_exclusive_resource_ownership() {
    let mut rules = source("r", 1);
    let q = query("r", &[1], [0, 0], false, 10);
    assert!(checked(&rules, &q).is_empty());
    rules.pop();
    let suspended = checked(&rules, &q);
    assert_eq!(suspended.len(), 1);
    assert!(suspended[0].residual.iter().any(|c| c.name == "rneed"));
    let mut rules = source("r", 1);
    rules.insert(
        0,
        Rule::simplify("steal", [c("rtoken", [v(0)])], Goal::True),
    );
    let q = query("r", &[1], [1, 0], false, 10);
    assert_eq!(relation("r", &[1], [1, 0], false, 1, 10).len(), 1);
    assert!(checked(&rules, &q).is_empty());
}
