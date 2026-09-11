#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_reuse::calls::{Caller, CallerEvent, Fresh, Table};
use chr_syntax::{Answer, Goal, Query, Rule, Var, and, atom, c, eq, or, v};

fn source(accepted: u16, weight: usize, depth: usize) -> Vec<Rule> {
    let mut rules = vec![];
    for mask in 1..8 {
        let arms = ["a", "b", "c"]
            .into_iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .flat_map(|(_, a)| (0..weight).map(move |_| eq(v(0), atom(a))));
        rules.push(Rule::simplify(
            &format!("d{mask}"),
            [c(&format!("d{mask}"), [v(0)])],
            arms.reduce(or).unwrap(),
        ));
    }
    for i in 0..=depth {
        let from = if i == 0 {
            "entry".into()
        } else {
            format!("prefix{i}")
        };
        let to = if i == depth {
            "pair".into()
        } else {
            format!("prefix{}", i + 1)
        };
        rules.push(Rule::simplify(
            &from,
            [c(&from, [v(0), v(1)])],
            c(&to, [v(0), v(1)]).into(),
        ));
    }
    for (i, a) in ["a", "b", "c"].iter().enumerate() {
        for (j, b) in ["a", "b", "c"].iter().enumerate() {
            if accepted & (1 << (3 * i + j)) != 0 {
                rules.push(Rule::simplify(
                    &format!("yes{i}{j}"),
                    [c("pair", [atom(a), atom(b)])],
                    Goal::True,
                ));
            }
        }
    }
    rules.push(Rule::simplify("no", [c("pair", [v(0), v(1)])], Goal::Fail));
    rules.push(Rule::simplify(
        "caller",
        [c("out", [v(0)]), c("token", [])],
        c("result", [v(0)]).into(),
    ));
    rules
}
fn query(left: usize, right: usize, alias: bool, base: u64) -> Query {
    let y = if alias { base } else { base + 1 };
    Query {
        constraints: vec![
            c(&format!("d{left}"), [v(base)]),
            c(&format!("d{right}"), [v(y)]),
            c("entry", [v(base), v(y)]),
            c("out", [v(base)]),
            c("token", []),
            c("noise", [atom(&format!("marker{base}")), v(base + 9)]),
        ],
        outputs: vec![
            ("x".into(), Var(base)),
            ("y".into(), Var(y)),
            ("unused".into(), Var(base + 9)),
        ],
    }
}
fn bundled_rules(original: &[Rule]) -> Vec<Rule> {
    let mut rules = vec![];
    for left in [1, 3, 7] {
        for right in [1, 3, 7] {
            rules.push(Rule::simplify(
                &format!("bundle{left}{right}"),
                [c(
                    "bundle",
                    [
                        atom(&format!("m{left}")),
                        atom(&format!("m{right}")),
                        v(0),
                        v(1),
                    ],
                )],
                and([
                    c(&format!("d{left}"), [v(0)]).into(),
                    c(&format!("d{right}"), [v(1)]).into(),
                    c("entry", [v(0), v(1)]).into(),
                ]),
            ));
        }
    }
    rules.extend_from_slice(original);
    rules
}
fn bundled_query(q: &Query, left: usize, right: usize) -> Query {
    let mut q = q.clone();
    let args = q.constraints[2].args.clone();
    q.constraints.splice(
        ..3,
        [c(
            "bundle",
            [
                atom(&format!("m{left}")),
                atom(&format!("m{right}")),
                args[0].clone(),
                args[1].clone(),
            ],
        )],
    );
    q
}
fn collect(caller: &mut Caller, q: Query, bound: usize) -> Result<Vec<Answer>, String> {
    let mut run = caller.start(q, bound, 200_000);
    let mut answers = vec![];
    loop {
        match caller.step(&mut run)? {
            CallerEvent::Progress => (),
            CallerEvent::Answer(a) => answers.push(a),
            CallerEvent::Done => return Ok(answers),
        }
    }
}
#[test]
fn bundled_phase_preserves_domains_aliases_weights_and_changed_callers() {
    for memo in [false, true] {
        for accepted in [0, 484, 511] {
            for weight in [1, 2] {
                for depth in [0, 4] {
                    let original = source(accepted, weight, depth);
                    let wrapped = bundled_rules(&original);
                    let mut caller = Caller::new(wrapped.clone(), wrapped.len() - 1, memo).unwrap();
                    for base in [10, 1000] {
                        for left in [1, 3, 7] {
                            for right in [1, 3, 7] {
                                for alias in [false, true] {
                                    let q = query(left, right, alias, base);
                                    let expected = oracle::run(&original, &q, 200_000);
                                    let count = (0..3)
                                        .flat_map(|i| (0..3).map(move |j| (i, j)))
                                        .filter(|&(i, j)| {
                                            left & (1 << i) != 0
                                                && right & (1 << j) != 0
                                                && (!alias || i == j)
                                                && accepted & (1 << (3 * i + j)) != 0
                                        })
                                        .count()
                                        * weight
                                        * weight;
                                    assert_eq!(expected.len(), count);
                                    oracle::same_raw(
                                        collect(
                                            &mut caller,
                                            bundled_query(&q, left, right),
                                            200_000,
                                        )
                                        .unwrap(),
                                        expected,
                                    );
                                }
                            }
                        }
                    }
                    if chr_reuse::continuations::COLLECT_METRICS {
                        assert_eq!(caller.stats().computed, if memo { 18 } else { 36 });
                        assert_eq!(caller.stats().hits, if memo { 18 } else { 0 });
                    }
                }
            }
        }
    }
}
#[test]
fn separate_unbound_calls_are_not_a_substitute_for_the_complete_phase() {
    let rules = source(511, 1, 0);
    let q = query(3, 3, false, 10);
    assert_eq!(oracle::run(&rules, &q, 200_000).len(), 4);
    let mut table = Table::new(rules[..rules.len() - 1].to_vec(), true).unwrap();
    assert!(table.expand_query(&q, 0, &rules, 200_000).is_err());
    let mut fresh = Fresh::for_query(&q);
    assert!(
        table
            .expand(&q.constraints[2], &mut fresh, 200_000)
            .unwrap()
            .is_empty()
    );
}
#[test]
fn an_unfinished_bundle_does_not_become_a_cached_failure() {
    let original = source(511, 2, 4);
    let rules = bundled_rules(&original);
    let mut caller = Caller::new(rules.clone(), rules.len() - 1, true).unwrap();
    let q = query(7, 7, false, 10);
    assert!(collect(&mut caller, bundled_query(&q, 7, 7), 0).is_err());
    let expected = oracle::run(&original, &q, 200_000);
    assert_eq!(expected.len(), 36);
    oracle::same_raw(
        collect(&mut caller, bundled_query(&q, 7, 7), 200_000).unwrap(),
        expected,
    );
}
