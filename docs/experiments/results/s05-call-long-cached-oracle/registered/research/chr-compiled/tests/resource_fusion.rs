#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent, resource_fusion::Program};
use chr_syntax::{Answer, Goal, Query, Rule, Var, and, atom, c, eq, or, v};
fn source(prefix: &str, offset: u64, duplicate: bool) -> Vec<Rule> {
    let name = |x: &str| format!("{prefix}{x}");
    let v = |x| v(x + offset);
    let effect = |color| {
        and(vec![
            eq(v(1), atom(color)),
            c(&name("out"), [v(0), v(1), v(10), v(10), v(11)]).into(),
        ])
    };
    vec![
        Rule::simplify(
            "consume",
            [
                c(&name("middle"), [v(0), v(1)]),
                c(&name("permit"), [v(0)]),
                c(&name("permit"), [v(0)]),
            ],
            or(effect("a"), effect(if duplicate { "a" } else { "b" })),
        ),
        Rule::simplify(
            "observe_final",
            [c(&name("out"), [v(0), v(1), v(2), v(3), v(4)])],
            c(&name("seen"), [v(0), v(1), v(2), v(3), v(4)]).into(),
        ),
        Rule::simplify(
            "produce",
            [c(&name("seed"), [v(10), v(11)]), c(&name("fuel"), [v(10)])],
            c(&name("middle"), [v(10), v(11)]).into(),
        ),
    ]
}
fn query(prefix: &str, seeds: usize, fuel: usize, permits: usize, shared: bool) -> Query {
    let name = |x: &str| format!("{prefix}{x}");
    let mut xs = (0..seeds)
        .map(|i| {
            c(
                &name("seed"),
                [atom("owner"), v(if shared { 100 } else { 100 + i as u64 })],
            )
        })
        .collect::<Vec<_>>();
    xs.extend((0..fuel).map(|_| c(&name("fuel"), [atom("owner")])));
    xs.extend((0..permits).map(|_| c(&name("permit"), [atom("owner")])));
    Query {
        constraints: xs,
        outputs: (0..seeds)
            .map(|i| {
                (
                    format!("x{i}"),
                    Var(if shared { 100 } else { 100 + i as u64 }),
                )
            })
            .collect(),
    }
}
fn native(r: &[Rule], q: &Query, special: bool) -> Vec<Answer> {
    let p = PreparedRuleset::new(r.to_vec(), None).unwrap();
    let p = if special { p.specialize_inferred() } else { p };
    let mut e = p
        .start_search(q.clone(), Policy::Global, Access::Scan)
        .unwrap();
    let mut out = vec![];
    for _ in 0..200_000 {
        match e.tick() {
            SearchEvent::Complete(mut b) => out.push(b.engine.observe().unwrap()),
            SearchEvent::Exhausted => return out,
            _ => (),
        }
    }
    panic!("unfinished resource source")
}
#[test]
fn renamed_resource_counts_choices_freshness_and_aliases() {
    for (prefix, offset) in [("", 0), ("renamed_", 70)] {
        for duplicate in [false, true] {
            let rules = source(prefix, offset, duplicate);
            let plan = Program::infer(&rules).unwrap();
            assert_eq!(plan.fused_rules, (2, 0));
            for seeds in 0..=2 {
                for fuel in 0..=2 {
                    for permits in 0..=4 {
                        for shared in [false, true] {
                            let q = query(prefix, seeds, fuel, permits, shared);
                            if permits < 2 * seeds.min(fuel) {
                                assert!(plan.lower(&q).is_err());
                                continue;
                            }
                            let lowered = plan.lower(&q).unwrap();
                            let expected = oracle::run(&rules, &q, 200_000);
                            oracle::same_raw(oracle::run(lowered, &q, 200_000), expected.clone());
                            for special in [false, true] {
                                oracle::same_raw(native(&rules, &q, special), expected.clone());
                                oracle::same_raw(native(lowered, &q, special), expected.clone());
                            }
                        }
                    }
                }
            }
        }
    }
}
#[test]
fn insufficient_resources_are_a_semantic_counterexample_not_an_empty_result() {
    let rules = source("", 0, false);
    let plan = Program::infer(&rules).unwrap();
    let q = query("", 1, 1, 0, false);
    assert!(plan.lower(&q).is_err());
    let original = oracle::run(&rules, &q, 200_000);
    assert_eq!(original.len(), 1);
    assert!(original[0].residual.iter().any(|c| c.name == "middle"));
    // The emitted program is obtained through a separately certified query to
    // expose why applying it without this query's certificate would be wrong.
    let eligible = query("", 1, 1, 2, false);
    let wrong = oracle::run(plan.lower(&eligible).unwrap(), &q, 200_000);
    assert!(wrong[0].residual.iter().any(|c| c.name == "seed"));
    assert!(!chr_observe::equivalent(
        &original[0],
        &wrong[0],
        &mut Default::default()
    ));
}
#[test]
fn unknown_keys_initial_intermediates_and_competing_owners_are_rejected() {
    let rules = source("", 0, false);
    let plan = Program::infer(&rules).unwrap();
    let mut q = query("", 1, 1, 2, false);
    q.constraints[0].args[0] = v(999);
    assert!(plan.lower(&q).is_err());
    let mut q = query("", 1, 1, 2, false);
    q.constraints.push(c("middle", [atom("owner"), v(200)]));
    assert!(plan.lower(&q).is_err());
    for head in [
        c("middle", [v(0), v(1)]),
        c("permit", [v(0)]),
        c("seed", [v(0), v(1)]),
    ] {
        let mut changed = rules.clone();
        changed.push(Rule::simplify("external", [head], Goal::True));
        assert!(Program::infer(&changed).is_err());
    }
    let mut changed = rules.clone();
    changed.swap(0, 2);
    assert!(Program::infer(&changed).is_err());
}
#[test]
fn actual_resource_applications_are_eliminated() {
    let mut rules = source("", 0, false);
    rules.remove(1);
    rules[0].body = c("done", [v(0), v(1), v(10), v(10)]).into();
    let q = query("", 3, 3, 6, false);
    let plan = Program::infer(&rules).unwrap();
    for (r, want) in [(&rules[..], 6), (plan.lower(&q).unwrap(), 3)] {
        oracle::same_raw(
            oracle::run(r, &q, 200_000),
            oracle::run(&rules, &q, 200_000),
        );
        let p = PreparedRuleset::new(r.to_vec(), None).unwrap();
        let mut e = p
            .start_search(q.clone(), Policy::Global, Access::Scan)
            .unwrap();
        e.enable_trace();
        let mut seen = false;
        for _ in 0..200_000 {
            match e.tick() {
                SearchEvent::Complete(mut b) => {
                    assert_eq!(b.engine.trace().len(), want);
                    assert_eq!(b.engine.observe().unwrap().residual.len(), 3);
                    seen = true;
                }
                SearchEvent::Exhausted => break,
                _ => (),
            }
        }
        assert!(seen);
    }
}

#[test]
fn keys_partition_resources_and_late_payload_aliases_remain_ordinary_effects() {
    let rules = source("", 0, false);
    let plan = Program::infer(&rules).unwrap();
    let mut wrong = query("", 1, 1, 2, false);
    for c in &mut wrong.constraints {
        if c.name == "permit" {
            c.args[0] = atom("other");
        }
    }
    assert!(plan.lower(&wrong).is_err());
    for reverse in [false, true] {
        let mut q = query("", 1, 1, 2, false);
        let mut other = query("", 1, 1, 3, false);
        for c in &mut other.constraints {
            c.args[0] = atom("other");
            if c.name == "seed" {
                c.args[1] = v(200);
            }
        }
        q.constraints.extend(other.constraints);
        q.outputs.push(("other".into(), Var(200)));
        if reverse {
            q.constraints.reverse();
        }
        let expected = oracle::run(&rules, &q, 200_000);
        let lowered = plan.lower(&q).unwrap();
        oracle::same_raw(oracle::run(lowered, &q, 200_000), expected.clone());
        oracle::same_raw(native(lowered, &q, true), expected);
    }
    for bind_constant in [false, true] {
        let mut rules = rules.clone();
        rules.push(Rule::simplify(
            "late_alias",
            [c("alias", [v(0), v(1)])],
            eq(v(0), v(1)),
        ));
        let plan = Program::infer(&rules).unwrap();
        let mut q = query("", 2, 2, 4, false);
        q.constraints.push(c(
            "alias",
            [v(100), if bind_constant { atom("a") } else { v(101) }],
        ));
        let expected = oracle::run(&rules, &q, 200_000);
        let lowered = plan.lower(&q).unwrap();
        oracle::same_raw(oracle::run(lowered, &q, 200_000), expected.clone());
        for special in [false, true] {
            oracle::same_raw(native(lowered, &q, special), expected.clone());
        }
        assert_eq!(expected.len(), 2);
    }
}

#[test]
fn checked_variable_bounds_and_source_restrictions() {
    let mut rules = source("", 0, false);
    rules[0].body = eq(v(u64::MAX), atom("a"));
    assert!(matches!(
        Program::infer(&rules),
        Err("variable identity exhausted")
    ));
    let mut rules = source("", 0, false);
    rules[2].removed[0].args[1] = atom("a");
    assert!(Program::infer(&rules).is_err());
    let mut rules = source("", 0, false);
    rules[0].removed[1].args.push(v(1));
    assert!(Program::infer(&rules).is_err());
    let mut rules = source("", 0, false);
    rules[0].body = c("fuel", [v(0)]).into();
    assert!(Program::infer(&rules).is_err());
}
