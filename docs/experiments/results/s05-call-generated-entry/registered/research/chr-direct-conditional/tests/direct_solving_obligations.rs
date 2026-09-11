//! Source-semantic probes, not a general solver or a performance benchmark.
#[allow(dead_code)]
mod composition_support;
#[allow(dead_code)]
mod runtime_support;
#[allow(dead_code)]
#[path = "../examples/support/order_source.rs"]
mod source;
use chr_syntax::{Answer, Goal, Query, Rule, Var, atom, c, eq, or, v};

fn checked(rules: &[Rule], query: &Query) -> Vec<Answer> {
    let a = runtime_support::run(rules, query, 2_000_000);
    runtime_support::same_raw(
        composition_support::Engine::new(0, rules, query).collect(),
        a.clone(),
    );
    a
}
fn query(cs: Vec<chr_syntax::Constraint>) -> Query {
    Query {
        constraints: cs,
        outputs: vec![("x".into(), Var(100)), ("again".into(), Var(100))],
    }
}
fn basic(duplicate: bool) -> Vec<Rule> {
    let yes = eq(v(0), atom("t"));
    vec![
        Rule::simplify(
            "coin",
            [c("coin", [v(0)])],
            or(
                yes.clone(),
                if duplicate {
                    or(yes, eq(v(0), atom("f")))
                } else {
                    eq(v(0), atom("f"))
                },
            ),
        ),
        Rule::simplify("accept", [c("check", [atom("t")])], c("done", []).into()),
        Rule::simplify("reject", [c("check", [atom("f")])], Goal::Fail),
    ]
}
#[test]
fn closed_arrival_pruning_preserves_full_answers() {
    let mut count = 0;
    for family in ["oldest-first", "newest-first"] {
        for n in 0..=6 {
            for work in [0, 2] {
                for resource in [false, true] {
                    for fail_tail in [false, true] {
                        for tag in [false, true] {
                            let s = source::Schema {
                                family,
                                work,
                                resource,
                                fail_tail,
                                payload: 2,
                            };
                            let rules = s.rules();
                            let q = s.query(n, tag);
                            let original = checked(&rules, &q);
                            runtime_support::same_raw(original.clone(), s.expected_query(n, tag));
                            let mut pruned = rules;
                            // Known-source probe only. Future inference must derive and certify this.
                            pruned[0].body = eq(v(0), atom("t"));
                            runtime_support::same_raw(checked(&pruned, &q), original);
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(count, 224);
    println!("closed_source_configurations={count}");
}
#[test]
fn linked_consumer_can_rescue_the_apparently_failing_alternative() {
    let mut rules = basic(false);
    rules.insert(
        1,
        Rule::simplify(
            "rescue",
            [c("watch", [atom("f")]), c("check", [v(0)])],
            c("rescued", []).into(),
        ),
    );
    let q = query(vec![
        c("coin", [v(100)]),
        c("watch", [v(100)]),
        c("check", [v(100)]),
    ]);
    let original = checked(&rules, &q);
    assert_eq!(original.len(), 2);
    assert!(
        original
            .iter()
            .any(|a| a.residual == vec![c("rescued", [])] && a.outputs[0].1 == atom("f"))
    );
    rules[0].body = eq(v(0), atom("t"));
    let pruned = checked(&rules, &q);
    assert_eq!(pruned.len(), 1);
    println!("linked_consumer_answers=2 pruned_answers=1");
}
#[test]
fn early_binding_changes_a_competing_writer_and_resource_winner() {
    let mut rules = basic(false);
    rules[0].removed.push(c("token", []));
    rules.insert(
        0,
        Rule::simplify(
            "writer",
            [c("watch", [atom("t"), v(0)]), c("token", [])],
            eq(v(0), atom("f")),
        ),
    );
    rules.insert(0, Rule::simplify("entry", [c("start", [v(0)])], Goal::True));
    let mut q = query(vec![
        c("start", [v(100)]),
        c("coin", [v(100)]),
        c("token", []),
        c("watch", [v(100), v(101)]),
        c("check", [v(100)]),
    ]);
    q.outputs.push(("y".into(), Var(101)));
    let original = checked(&rules, &q);
    assert_eq!(original.len(), 1);
    assert!(matches!(original[0].outputs[2].1, chr_syntax::Term::Var(_)));
    assert!(!original[0].residual.iter().any(|c| c.name == "coin"));
    rules[2].body = eq(v(0), atom("t"));
    runtime_support::same_raw(checked(&rules, &q), original);
    // Same successful binding, but published at entry instead of choice execution.
    rules[0].body = eq(v(0), atom("t"));
    let early = checked(&rules, &q);
    assert_eq!(early.len(), 1);
    assert_eq!(early[0].outputs[2].1, atom("f"));
    assert!(early[0].residual.iter().any(|c| c.name == "coin"));
    println!("early_binding_changes_full_answer=true local_pruning_preserves=true");
}
#[test]
fn duplicate_derivations_must_survive_finite_solving() {
    for copies in 1..=3 {
        let mut cs = vec![c("coin", [v(100)]); copies];
        cs.push(c("check", [v(100)]));
        let q = query(cs);
        let original = checked(&basic(true), &q);
        let collapsed = checked(&basic(false), &q);
        assert_eq!(original.len(), 1 << copies);
        assert_eq!(collapsed.len(), 1);
        for answer in &original {
            runtime_support::same_raw(vec![answer.clone()], collapsed.clone());
        }
        println!(
            "aliased_requests={copies} duplicate_answers={} distinct_values=1",
            original.len()
        );
    }
}
