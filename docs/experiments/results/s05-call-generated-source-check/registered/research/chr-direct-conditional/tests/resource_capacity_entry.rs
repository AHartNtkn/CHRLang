#[path = "../../chr-compiled/experiments/resource_capacity.rs"]
mod capacity;
// Independent capacity relation versus full consuming source semantics.
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
                                let prepared = capacity::Prepared::<true>::new(&rules).unwrap();
                                let report =
                                    prepared.solve(&q, capacity::Limits::default()).unwrap();
                                runtime_support::same_raw(
                                    report.answers,
                                    relation(tag, &domains, [a, b], alias, weight, base),
                                );

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

#[test]
fn solver_checks_source_and_query_boundaries() {
    let rules = source("r", 1);
    let p = capacity::Prepared::<true>::new(&rules).unwrap();
    let mut q = query("r", &[3], [1, 1], false, 10);
    q.constraints
        .push(c("payload", [chr_syntax::t("box", [v(10)])]));
    runtime_support::same_raw(
        p.solve(&q, Default::default()).unwrap().answers,
        checked(&rules, &q),
    );
    q.constraints[0].args[0] = atom("a");
    runtime_support::same_raw(
        p.solve(&q, Default::default()).unwrap().answers,
        checked(&rules, &q),
    );
    let mut bad = rules.clone();
    bad.pop();
    assert!(capacity::Prepared::<true>::new(&bad).is_err());
    let mut bad = rules.clone();
    bad.insert(
        0,
        Rule::simplify("steal", [c("rtoken", [v(0)])], Goal::True),
    );
    assert!(capacity::Prepared::<true>::new(&bad).is_err());
    let mut q = query("r", &[3], [1, 1], false, 10);
    q.constraints.push(c("rneed", [v(10)]));
    assert!(p.solve(&q, Default::default()).is_err());
    let mut q = query("r", &[3], [1, 1], false, 10);
    q.constraints[1].args[0] = v(20);
    assert!(p.solve(&q, Default::default()).is_err());
    let q = query("r", &[3, 3], [2, 2], false, 10);
    assert!(
        p.solve(
            &q,
            capacity::Limits {
                states: 0,
                answers: 100
            }
        )
        .is_err()
    );
    assert!(
        p.solve(
            &q,
            capacity::Limits {
                states: 100,
                answers: 1
            }
        )
        .is_err()
    );
    let mut partial = query("r", &[3, 3, 3], [1, 2], false, 10);
    partial.constraints[2].args[0] = v(10);
    partial.outputs[2].1 = Var(10);
    let answer = p.solve(&partial, Default::default()).unwrap();
    assert_eq!(answer.answers.len(), 1);
    runtime_support::same_raw(answer.answers, checked(&rules, &partial));
    let many = query("r", &[3; 65], [65, 65], false, 10);
    assert!(matches!(
        p.solve(&many, Default::default()),
        Err(capacity::Error::Limit)
    ));
    let mut duplicate = q.clone();
    duplicate.outputs.push(duplicate.outputs[0].clone());
    assert!(p.solve(&duplicate, Default::default()).is_err());
    let mut body_owner = rules.clone();
    body_owner[0].body = and(vec![
        body_owner[0].body.clone(),
        c("rtoken", [atom("a")]).into(),
    ]);
    assert!(capacity::Prepared::<true>::new(&body_owner).is_err());
    let mut ambiguous = rules.clone();
    ambiguous[1].removed[0].name = ambiguous[0].removed[0].name.clone();
    assert!(capacity::Prepared::<true>::new(&ambiguous).is_err());
    let mut reversed = rules.clone();
    reversed[3].removed.reverse();
    runtime_support::same_raw(
        capacity::Prepared::<true>::new(&reversed)
            .unwrap()
            .solve(&q, Default::default())
            .unwrap()
            .answers,
        checked(&reversed, &q),
    );
}

#[test]
fn capacity_subsets_prune_before_branching_and_successes_are_complete() {
    let mut rules = Vec::new();
    for (pred, a, b) in [
        ("choose_left", "alpha", "beta"),
        ("choose_right", "gamma", "delta"),
    ] {
        rules.push(Rule::simplify(
            pred,
            [c(pred, [v(9)])],
            and(vec![
                or(eq(v(9), atom(a)), eq(v(9), atom(b))),
                c("rneed", [v(9)]).into(),
            ]),
        ));
    }
    rules.extend(source("r", 1).into_iter().skip(3));
    let p = capacity::Prepared::<true>::new(&rules).unwrap();
    let make = |half: usize, caps: [usize; 4]| {
        let mut constraints = Vec::new();
        let mut outputs = Vec::new();
        for i in 0..2 * half {
            let id = 100 + i as u64;
            constraints.push(c(
                if i < half {
                    "choose_left"
                } else {
                    "choose_right"
                },
                [v(id)],
            ));
            outputs.push((format!("v{i}"), Var(id)));
        }
        for (name, count) in ["alpha", "beta", "gamma", "delta"].into_iter().zip(caps) {
            for _ in 0..count {
                constraints.push(c("rtoken", [atom(name)]));
            }
        }
        Query {
            constraints,
            outputs,
        }
    };
    let small = make(2, [0, 1, 1, 2]);
    assert!(checked(&rules, &small).is_empty());
    let r = p.solve(&small, Default::default()).unwrap();
    assert!(r.answers.is_empty());
    assert_eq!((r.states, r.branches, r.capacity_prunes), (1, 0, 1));
    // 24 independent binary choices, total supply 24, left-domain capacity 11 < 12.
    let r = p
        .solve(&make(12, [5, 6, 6, 7]), Default::default())
        .unwrap();
    assert!(r.answers.is_empty());
    assert_eq!((r.states, r.branches, r.capacity_prunes), (1, 0, 1));
    let q = make(2, [2, 2, 2, 2]);
    let r = p.solve(&q, Default::default()).unwrap();
    assert_eq!(r.answers.len(), 16);
    assert!(r.branches >= 16);
    runtime_support::same_raw(r.answers, checked(&rules, &q));
    assert!(
        p.solve(
            &q,
            capacity::Limits {
                states: 1,
                answers: 4096
            }
        )
        .is_err()
    );
    assert_eq!(p.solve(&q, Default::default()).unwrap().answers.len(), 16);
    println!(
        "capacity bottleneck: 24 binary choices, states=1 branches=0; unselective control: 16 raw answers"
    );
}

#[test]
fn capacity_diagnostics_do_not_change_answers_or_operational_limits() {
    let rules = source("r", 2);
    let primary = capacity::Prepared::<false>::new(&rules).unwrap();
    let diagnostic = capacity::Prepared::<true>::new(&rules).unwrap();
    for caps in [[0, 0], [1, 1], [2, 2]] {
        let q = query("r", &[3, 3], caps, false, 10);
        let a = primary.solve(&q, Default::default()).unwrap();
        let b = diagnostic.solve(&q, Default::default()).unwrap();
        assert_eq!((a.branches, a.capacity_prunes), (0, 0));
        assert_eq!(a.states, b.states);
        runtime_support::same_raw(a.answers, b.answers);
        assert!(
            primary
                .solve(
                    &q,
                    capacity::Limits {
                        states: 0,
                        answers: 100
                    }
                )
                .is_err()
        );
        assert!(
            diagnostic
                .solve(
                    &q,
                    capacity::Limits {
                        states: 0,
                        answers: 100
                    }
                )
                .is_err()
        );
    }
}
