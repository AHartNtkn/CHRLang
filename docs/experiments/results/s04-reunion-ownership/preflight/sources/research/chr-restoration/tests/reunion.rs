#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_restoration::{Mode, Prepared, Step, reunion};
use chr_syntax::{Answer, Query, Rule, Var, and, atom, c, eq, or, t, v};
fn run(rules: &[Rule], n: usize, q: &Query, limit: usize) -> Result<reunion::Outcome, String> {
    reunion::PreparedPhase::new(rules, n)?.run(q, limit)
}
fn nat(n: usize) -> chr_syntax::Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
fn source(depth: usize, duplicate: bool) -> (Vec<Rule>, usize, Query) {
    let mut rules = vec![];
    for key in ["left", "right"] {
        rules.push(Rule::simplify(
            "choose",
            [c("job", [atom(key), v(0), v(1)])],
            or(
                c("walk", [atom(key), v(0), v(1), atom("a")]).into(),
                c(
                    "walk",
                    [
                        atom(key),
                        v(0),
                        v(1),
                        atom(if duplicate { "a" } else { "b" }),
                    ],
                )
                .into(),
            ),
        ));
        rules.push(Rule::simplify(
            "walk",
            [c("walk", [atom(key), t("s", [v(0)]), v(1), v(2)])],
            c("walk", [atom(key), v(0), v(1), v(2)]).into(),
        ));
        rules.push(Rule::simplify(
            "ready",
            [c("walk", [atom(key), atom("z"), v(0), v(1)])],
            and(vec![
                eq(v(0), v(1)),
                c("ready", [atom(key), v(0), v(2)]).into(),
            ]),
        ));
    }
    let local = rules.len();
    rules.push(Rule::simplify(
        "join",
        [
            c("ready", [atom("left"), v(0), v(2)]),
            c("ready", [atom("right"), v(1), v(3)]),
        ],
        and(vec![
            eq(v(0), v(1)),
            eq(v(2), v(3)),
            c("joined", [atom("left"), v(0), v(2)]).into(),
        ]),
    ));
    let query = Query {
        constraints: vec![
            c("job", [atom("left"), nat(depth), v(100)]),
            c("job", [atom("right"), nat(depth), v(200)]),
        ],
        outputs: vec![("x".into(), Var(100)), ("y".into(), Var(200))],
    };
    (rules, local, query)
}
fn ordinary(rules: &[Rule], q: &Query) -> (Vec<Answer>, usize) {
    let p = Prepared::new(rules).unwrap();
    let mut e = p.start(q, Mode::Copy).unwrap();
    let mut out = vec![];
    for steps in 1..100_000 {
        match e.advance() {
            Step::Answer(a) => out.push(a),
            Step::Exhausted => return (out, steps - 1),
            Step::Progress => (),
        }
    }
    panic!("ordinary cutoff")
}
#[test]
fn reunited_binding_consumes_resources_preserves_freshness_and_avoids_repeated_work() {
    for depth in [0, 1, 4, 12] {
        for duplicate in [false, true] {
            let (rules, n, q) = source(depth, duplicate);
            let got = run(&rules, n, &q, 100_000).unwrap();
            oracle::same_raw(got.answers.clone(), oracle::run(&rules, &q, 100_000));
            let (expected, _steps) = ordinary(&rules, &q);
            oracle::same_raw(got.answers.clone(), expected);
            assert_eq!(
                chr_factors::partition::regions(rules.clone(), q.clone(), &mut Default::default())
                    .len(),
                1
            );
            assert_eq!(got.components, 2);
            #[cfg(feature = "replay-diagnostic")]
            assert_eq!(got.products, 4);
            assert_eq!(got.answers.len(), if duplicate { 4 } else { 2 });
            assert!(
                got.answers
                    .iter()
                    .all(|a| a.residual.len() == 1 && a.residual[0].name == "joined")
            );
            #[cfg(feature = "replay-diagnostic")]
            if depth == 12 {
                println!(
                    "duplicate={duplicate} local={} resumed={} coupled={_steps} products={}",
                    got.local_steps, got.reunion_steps, got.products
                );
                assert!(
                    got.local_steps + got.reunion_steps < _steps,
                    "private work was repeated"
                );
            }
        }
    }
}
#[test]
fn rejects_shared_variables_unknown_ownership_and_local_escape() {
    let (mut rules, n, mut q) = source(1, false);
    q.constraints[1].args[2] = v(100);
    assert!(run(&rules, n, &q, 100).is_err());
    q.constraints[1].args[2] = v(200);
    q.constraints[0].args[0] = v(900);
    assert!(run(&rules, n, &q, 100).is_err());
    q.constraints[0].args[0] = atom("left");
    rules[0].body = c("escape", [atom("right")]).into();
    assert!(run(&rules, n, &q, 100).is_err());
}
#[test]
fn ongoing_local_work_returns_unfinished_error() {
    let (mut rules, n, q) = source(1, false);
    rules[0].body = c("job", [atom("left"), v(0), v(1)]).into();
    let error = run(&rules, n, &q, 20).err().unwrap();
    assert!(error.contains("budget"), "{error}");
}

#[test]
fn history_fresh_aliases_and_competing_consumers_survive_reunion() {
    for depth in 0..=4 {
        for variant in 0..4 {
            let (mut rules, n, mut q) = source(depth, false);
            // Propagation creates a fresh alias pair which must remain private.
            let mut propagation = vec![];
            for key in ["left", "right"] {
                propagation.push(Rule {
                    name: "remember".into(),
                    kept: vec![c("stamp", [atom(key)])],
                    removed: vec![],
                    guards: vec![],
                    body: c("alias", [atom(key), v(500), v(500)]).into(),
                });
                q.constraints.push(c("stamp", [atom(key)]));
            }
            rules.splice(n..n, propagation);
            if variant == 1 {
                rules[0].body = or(
                    chr_syntax::Goal::Fail,
                    c("walk", [atom("left"), v(0), v(1), atom("a")]).into(),
                );
            }
            if variant == 2 {
                // Fresh values from both private phases remain observably distinct.
                rules[n + 2].body = and(vec![
                    c("joined", [atom("left"), v(0), v(2)]).into(),
                    c("joined", [atom("right"), v(1), v(3)]).into(),
                ]);
            }
            if variant == 3 {
                q.constraints.push(c("permit", [atom("left")]));
                let mut first = rules[n + 2].clone();
                first.removed.push(c("permit", [atom("left")]));
                first.body = c("winner", [atom("left"), atom("first")]).into();
                let mut second = first.clone();
                second.body = c("winner", [atom("left"), atom("second")]).into();
                rules.splice(n + 2.., [first, second]);
            }
            let got = run(&rules, n + 2, &q, 100_000).unwrap();
            oracle::same_raw(got.answers.clone(), oracle::run(&rules, &q, 100_000));
            oracle::same_raw(got.answers.clone(), ordinary(&rules, &q).0);
            for a in got.answers {
                assert_eq!(a.residual.iter().filter(|c| c.name == "alias").count(), 2);
                if variant == 3 {
                    assert!(
                        a.residual
                            .iter()
                            .any(|c| c.name == "winner" && c.args[1] == atom("first"))
                    );
                }
            }
        }
    }
}

#[test]
fn a_late_join_binding_reactivates_private_rules() {
    for depth in 0..=8 {
        let mut rules = vec![];
        for key in ["left", "right"] {
            rules.push(Rule::simplify(
                "walk",
                [c("job", [atom(key), t("s", [v(0)]), v(1)])],
                c("job", [atom(key), v(0), v(1)]).into(),
            ));
            rules.push(Rule::simplify(
                "ready",
                [c("job", [atom(key), atom("z"), v(0)])],
                c("ready", [atom(key), v(0)]).into(),
            ));
            rules.push(Rule::simplify(
                "wake",
                [c("wait", [atom(key), t("done", [v(0)])])],
                c("awake", [atom(key), v(0)]).into(),
            ));
        }
        let n = rules.len();
        rules.push(Rule::simplify(
            "join",
            [
                c("ready", [atom("left"), v(0)]),
                c("ready", [atom("right"), v(1)]),
            ],
            and(vec![
                eq(v(0), t("done", [v(2)])),
                eq(v(1), t("done", [v(2)])),
            ]),
        ));
        let q = Query {
            constraints: vec![
                c("job", [atom("left"), nat(depth), v(100)]),
                c("wait", [atom("left"), v(100)]),
                c("job", [atom("right"), nat(depth), v(200)]),
                c("wait", [atom("right"), v(200)]),
            ],
            outputs: vec![("x".into(), Var(100)), ("y".into(), Var(200))],
        };
        let got = run(&rules, n, &q, 100_000).unwrap();
        oracle::same_raw(got.answers.clone(), oracle::run(&rules, &q, 100_000));
        oracle::same_raw(got.answers.clone(), ordinary(&rules, &q).0);
        assert_eq!(got.answers.len(), 1);
        assert_eq!(
            got.answers[0]
                .residual
                .iter()
                .filter(|c| c.name == "awake")
                .count(),
            2
        );
        #[cfg(feature = "replay-diagnostic")]
        assert!(got.reunion_steps > 1);
    }
}

#[test]
fn finite_sibling_is_published_while_private_work_continues_and_cancel_is_owned() {
    let (mut rules, n, q) = source(1, false);
    rules[0].body = or(
        c("job", [atom("left"), v(0), v(1)]).into(),
        c("walk", [atom("left"), v(0), v(1), atom("a")]).into(),
    );
    let p = Prepared::new(&rules).unwrap();
    let mut ordinary = p.start(&q, Mode::Copy).unwrap();
    let control_first = (0..1000)
        .find_map(|_| match ordinary.advance() {
            Step::Answer(a) => Some(a),
            _ => None,
        })
        .expect("ordinary finite sibling");
    let mut e = reunion::PreparedPhase::new(&rules, n)
        .unwrap()
        .start(&q)
        .unwrap();
    let mut first = None;
    for _ in 0..1000 {
        match e.advance().unwrap() {
            Step::Answer(a) => {
                first = Some(a);
                break;
            }
            Step::Exhausted => panic!("unfinished source exhausted"),
            Step::Progress => (),
        }
    }
    let first = first.expect("finite sibling was blocked");
    oracle::same_raw(vec![first.clone()], vec![control_first]);
    assert_eq!(first.residual[0].name, "joined");
    drop(e);
    let (rules, n, q) = source(1, false);
    oracle::same_raw(
        run(&rules, n, &q, 100_000).unwrap().answers,
        oracle::run(&rules, &q, 100_000),
    );
}

#[test]
fn lazy_products_preserve_multiplicity_with_more_than_two_components() {
    for components in 2..=4 {
        for duplicate in [false, true] {
            let mut rules = vec![];
            let mut constraints = vec![];
            let mut heads = vec![];
            let mut outputs = vec![];
            let mut effects = vec![];
            for i in 0..components {
                let key = atom(&format!("owner{i}"));
                rules.push(Rule::simplify(
                    "choose",
                    [c("job", [key.clone(), v(0)])],
                    or(
                        c("ready", [key.clone(), v(0), atom("a")]).into(),
                        c(
                            "ready",
                            [key.clone(), v(0), atom(if duplicate { "a" } else { "b" })],
                        )
                        .into(),
                    ),
                ));
                constraints.push(c("job", [key.clone(), v(100 + i)]));
                outputs.push((format!("out{i}"), Var(100 + i)));
                heads.push(c("ready", [key, v(i), v(10 + i)]));
                effects.push(eq(v(i), v(10 + i)));
                effects.push(eq(v(10), v(10 + i)));
            }
            rules.push(Rule::simplify("join", heads, and(effects)));
            let q = Query {
                constraints,
                outputs,
            };
            let got = run(&rules, components as usize, &q, 100_000).unwrap();
            #[cfg(feature = "replay-diagnostic")]
            assert_eq!(got.products, 1 << components);
            assert_eq!(
                got.answers.len(),
                if duplicate { 1 << components } else { 2 }
            );
            oracle::same_raw(got.answers.clone(), oracle::run(&rules, &q, 100_000));
            oracle::same_raw(got.answers, ordinary(&rules, &q).0);
        }
    }
}

#[test]
fn source_priority_and_initial_occurrence_order_are_preserved_within_owners() {
    for depth in [0, 1, 4] {
        for reverse_rules in [false, true] {
            for reverse_query in [false, true] {
                let (mut rules, n, mut q) = source(depth, false);
                if reverse_rules {
                    rules[..n].reverse();
                }
                if reverse_query {
                    q.constraints.reverse();
                    rules[n].removed.reverse();
                }
                let got = run(&rules, n, &q, 100_000).unwrap();
                oracle::same_raw(got.answers.clone(), oracle::run(&rules, &q, 100_000));
                oracle::same_raw(got.answers, ordinary(&rules, &q).0);
            }
        }
    }
}

#[test]
fn prepared_phase_reuses_rules_across_changed_and_rejected_queries() {
    let (mut rules, n, q) = source(4, false);
    let prepared = reunion::PreparedPhase::new(&rules, n).unwrap();
    rules[0].body = chr_syntax::Goal::Fail;
    for seed in 0..4 {
        let mut changed = q.clone();
        changed.constraints[0].args[1] = nat(seed);
        changed.constraints[0].args[2] = v(1000 + seed as u64);
        changed.outputs[0].1 = Var(1000 + seed as u64);
        let original = source(4, false).0;
        let got = prepared.run(&changed, 100_000).unwrap();
        oracle::same_raw(got.answers, oracle::run(&original, &changed, 100_000));
        let mut invalid = changed.clone();
        invalid.constraints[1].args[2] = changed.constraints[0].args[2].clone();
        assert!(prepared.start(&invalid).is_err());
    }
}
