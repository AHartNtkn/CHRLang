#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "support/effectful_probe.rs"]
mod probe;
use chr_syntax::{Query, Rule, Var, atom, c, eq, v};
use probe::{Key, Probe};

#[test]
fn current_resource_values_and_fresh_aliases_survive_cached_transport() {
    let rules = vec![Rule::simplify(
        "take",
        [c("work", [v(0)]), c("token", [v(1)])],
        eq(v(0), chr_syntax::t("pair", [v(1), v(99), v(99)])),
    )];
    let mut p = Probe::new(rules.clone(), vec!["work", "token"], Key::Region);
    for (offset, tag) in [(0, "a"), (100, "a"), (200, "b")] {
        let q = Query {
            constraints: vec![
                c("work", [v(offset)]),
                c("token", [atom(tag)]),
                c("outside", [v(offset + 1000)]),
            ],
            outputs: vec![("out".into(), Var(offset))],
        };
        oracle::same_raw(p.run(&rules, &q), oracle::run(&rules, &q, 200_000));
    }
    assert_eq!(p.computed, 2);
    assert_eq!(p.hits, 1);
}

fn equivalent(a: &[chr_syntax::Answer], b: &[chr_syntax::Answer]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut used = vec![false; b.len()];
    a.iter().all(|a| {
        match b
            .iter()
            .enumerate()
            .position(|(i, b)| !used[i] && chr_observe::equivalent(a, b, &mut Default::default()))
        {
            Some(i) => {
                used[i] = true;
                true
            }
            None => false,
        }
    })
}
fn nat(n: usize) -> chr_syntax::Term {
    (0..n).fold(atom("z"), |n, _| chr_syntax::t("s", [n]))
}
fn phase(duplicate: bool) -> Vec<Rule> {
    use chr_syntax::{Goal, and, or, t};
    let body = and([
        eq(v(1), t("result", [v(2), v(3), v(99), v(99)])),
        c("effect", [v(2)]).into(),
    ]);
    vec![
        Rule::simplify(
            "step",
            [c("work", [t("s", [v(0)]), v(1)]), c("token", [v(2)])],
            and([c("work", [v(0), v(1)]).into(), c("token", [v(2)]).into()]),
        ),
        Rule {
            name: "finish".into(),
            kept: vec![c("fact", [v(3)])],
            removed: vec![c("work", [atom("z"), v(1)]), c("token", [v(2)])],
            guards: vec![],
            body: if duplicate {
                or(body.clone(), body)
            } else {
                body
            },
        },
        Rule::simplify("unused", [c("never", [])], Goal::True),
    ]
}
#[test]
fn resource_phase_matrix_distinguishes_stale_keys_from_transport() {
    let mut mismatches = 0;
    let mut checked = 0;
    let mut region_hits = 0;
    for depth in [0, 1, 4] {
        for duplicate in [false, true] {
            let rules = phase(duplicate);
            let mut uncached = Probe::new(rules.clone(), vec!["work", "token", "fact"], Key::None);
            let mut calls = Probe::new(rules.clone(), vec!["work", "token", "fact"], Key::Call);
            let mut regions = Probe::new(rules.clone(), vec!["work", "token", "fact"], Key::Region);
            let mut retained = vec![];
            for offset in [0, 100, 10000] {
                for tokens in [vec!["a"], vec!["b"], vec![], vec!["a", "b"]] {
                    for fact in [true, false] {
                        for tag in ["left", "right"] {
                            let mut constraints = vec![c("work", [nat(depth), v(offset)])];
                            constraints.extend(tokens.iter().map(|k| c("token", [atom(k)])));
                            if fact {
                                constraints.push(c("fact", [atom("ready")]))
                            }
                            constraints.push(c("caller", [atom(tag), v(offset), v(offset + 70)]));
                            let q = Query {
                                constraints,
                                outputs: vec![
                                    ("result".into(), Var(offset)),
                                    ("outside".into(), Var(offset + 70)),
                                ],
                            };
                            let expected = oracle::run(&rules, &q, 200_000);
                            oracle::same_raw(probe::direct(&rules, &q), expected.clone());
                            oracle::same_raw(uncached.run(&rules, &q), expected.clone());
                            let got = regions.run(&rules, &q);
                            oracle::same_raw(got.clone(), expected.clone());
                            mismatches +=
                                usize::from(!equivalent(&calls.run(&rules, &q), &expected));
                            retained.push((got, expected));
                            checked += 1;
                        }
                    }
                }
            }
            region_hits += regions.hits;
            drop(regions);
            drop(uncached);
            drop(calls);
            for (got, expected) in retained {
                oracle::same_raw(got, expected);
            }
        }
    }
    assert_eq!(checked, 288);
    assert!(mismatches > 0);
    assert!(region_hits > 0);
    println!(
        "resource matrix: queries={checked}, direct/uncached/region independent comparisons={}, call-only mismatches={mismatches}, region hits={region_hits}",
        checked * 3
    );
}

fn challenge(
    name: &str,
    source: Vec<Rule>,
    phase: Vec<Rule>,
    names: Vec<&str>,
    q: Query,
    should_match: bool,
) {
    let traced = oracle::run_traced(&source, &q, 200_000);
    let expected = traced.iter().map(|(a, _)| a.clone()).collect::<Vec<_>>();
    oracle::same_raw(probe::direct(&source, &q), expected.clone());
    for mode in [Key::None, Key::Region] {
        let mut p = Probe::new(phase.clone(), names.clone(), mode);
        let actual = p.run(&source, &q);
        assert_eq!(
            equivalent(&actual, &expected),
            should_match,
            "{name} {mode:?}: {actual:?} != {expected:?}"
        );
        let second = p.run(&source, &q);
        assert_eq!(
            equivalent(&second, &expected),
            should_match,
            "reused {name}"
        );
        oracle::same_raw(second, actual.clone());
        println!(
            "challenge={name} mode={mode:?} agrees={should_match} direct={traced:?} contracted={actual:?}"
        );
    }
}
#[test]
fn competing_consumption_requires_a_valid_scheduling_boundary() {
    use chr_syntax::{Goal, and};
    let step = Rule::simplify(
        "step",
        [c("work", [])],
        and([c("ready", []).into(), c("pulse", []).into()]),
    );
    let finish = Rule::simplify(
        "finish",
        [c("ready", []), c("token", [])],
        c("won", []).into(),
    );
    let steal = Rule::simplify(
        "compete",
        [c("pulse", []), c("token", [])],
        c("stolen", []).into(),
    );
    let q = Query {
        constraints: vec![c("work", []), c("token", [])],
        outputs: vec![],
    };
    challenge(
        "competing-consumer",
        vec![step.clone(), steal, finish.clone()],
        vec![step.clone(), finish.clone()],
        vec!["work", "token"],
        q.clone(),
        false,
    );
    challenge(
        "no-competitor",
        vec![
            step.clone(),
            finish.clone(),
            Rule::simplify("ignore", [c("pulse", [])], Goal::True),
        ],
        vec![step, finish],
        vec!["work", "token"],
        q,
        true,
    );
}
#[test]
fn intermediate_effects_cannot_be_observed_only_after_contraction() {
    use chr_syntax::{Goal, and};
    let step = Rule::simplify(
        "step",
        [c("work", [])],
        and([c("pulse", []).into(), c("ready", []).into()]),
    );
    let observe = Rule::propagate("observe", [c("pulse", [])], c("observed", []).into());
    let finish = Rule::simplify("finish", [c("ready", []), c("pulse", [])], Goal::True);
    let q = Query {
        constraints: vec![c("work", [])],
        outputs: vec![],
    };
    challenge(
        "intermediate-observer",
        vec![step.clone(), observe, finish.clone()],
        vec![step.clone(), finish.clone()],
        vec!["work"],
        q.clone(),
        false,
    );
    challenge(
        "no-observer",
        vec![step.clone(), finish.clone()],
        vec![step, finish],
        vec!["work"],
        q,
        true,
    );
}

#[test]
fn late_binding_changes_which_rule_may_execute() {
    let known = Rule::simplify(
        "known",
        [c("work", [atom("a"), v(0)])],
        eq(v(0), atom("known")),
    );
    let generic = Rule::simplify(
        "generic",
        [c("work", [v(1), v(0)])],
        eq(v(0), atom("generic")),
    );
    let supply = Rule::simplify("supply", [c("supply", [v(0)])], eq(v(0), atom("a")));
    let q = Query {
        constraints: vec![c("work", [v(7), v(8)]), c("supply", [v(7)])],
        outputs: vec![("result".into(), Var(8))],
    };
    challenge(
        "binding-before-call",
        vec![supply.clone(), known.clone(), generic.clone()],
        vec![known.clone(), generic.clone()],
        vec!["work"],
        q.clone(),
        false,
    );
    challenge(
        "binding-after-call",
        vec![known.clone(), generic.clone(), supply],
        vec![known, generic],
        vec!["work"],
        q,
        true,
    );
}
#[test]
fn residual_replay_does_not_transport_propagation_history() {
    let record = Rule::propagate("record", [c("fact", [])], c("recorded", []).into());
    let q = Query {
        constraints: vec![c("fact", [])],
        outputs: vec![],
    };
    challenge(
        "surviving-history",
        vec![record.clone()],
        vec![record],
        vec!["fact"],
        q,
        false,
    );
}
#[test]
fn reconstruction_must_preserve_relative_occurrence_order() {
    use chr_syntax::Goal;
    let no_op = Rule::simplify("idle", [c("work", [])], Goal::True);
    let choose = Rule::simplify(
        "choose",
        [c("token", [v(0)]), c("ask", [v(1)])],
        eq(v(1), v(0)),
    );
    let q = Query {
        constraints: vec![
            c("token", [atom("first")]),
            c("token", [atom("second")]),
            c("ask", [v(9)]),
        ],
        outputs: vec![("selected".into(), Var(9))],
    };
    // A region selecting all tokens preserves their order. This is the positive
    // boundary control; a per-occurrence region is tested separately below.
    challenge(
        "all-token-occurrences",
        vec![no_op.clone(), choose.clone()],
        vec![no_op],
        vec!["token"],
        q.clone(),
        true,
    );
    let mut reordered = q.clone();
    reordered.constraints.swap(0, 1);
    let a = oracle::run(std::slice::from_ref(&choose), &q, 200_000);
    let b = oracle::run(&[choose], &reordered, 200_000);
    assert!(!equivalent(&a, &b));
    println!("relative occurrence order: original={a:?}, reordered={b:?}");
}
#[test]
fn held_answers_remain_valid_after_queries_and_table_disposal() {
    let rules = phase(true);
    let mut p = Probe::new(rules.clone(), vec!["work", "token", "fact"], Key::Region);
    let mut retained = vec![];
    for offset in [0, 100, 1000] {
        let q = Query {
            constraints: vec![
                c("work", [nat(4), v(offset)]),
                c("token", [atom("a")]),
                c("fact", [atom("ready")]),
            ],
            outputs: vec![("result".into(), Var(offset))],
        };
        retained.push((p.run(&rules, &q), oracle::run(&rules, &q, 200_000)));
    }
    assert_eq!(p.computed, 1);
    assert_eq!(p.hits, 2);
    drop(p);
    for (actual, expected) in retained {
        oracle::same_raw(actual, expected);
    }
}

#[test]
fn shared_resource_interfaces_are_distinct_keys_and_reconnect_the_caller() {
    let rules = phase(false);
    let mut p = Probe::new(rules.clone(), vec!["work", "token", "fact"], Key::Region);
    for offset in [0, 100, 1000] {
        for shared in [true, false] {
            let fact = if shared { offset + 1 } else { offset + 2 };
            let q = Query {
                constraints: vec![
                    c("work", [nat(1), v(offset)]),
                    c("token", [v(offset + 1)]),
                    c("fact", [v(fact)]),
                    c("outside", [v(offset + 1), v(fact)]),
                ],
                outputs: vec![
                    ("out".into(), Var(offset)),
                    ("a".into(), Var(offset + 1)),
                    ("b".into(), Var(fact)),
                ],
            };
            oracle::same_raw(p.run(&rules, &q), oracle::run(&rules, &q, 200_000));
        }
    }
    assert_eq!(p.computed, 2);
    assert_eq!(p.hits, 4);
}
#[test]
fn interrupted_phase_collection_cannot_publish_partial_results() {
    let rules = phase(true);
    for bound in [0, 1] {
        let mut p = Probe::new(rules.clone(), vec!["work", "token", "fact"], Key::Region);
        let q = Query {
            constraints: vec![
                c("work", [nat(4), v(7)]),
                c("token", [atom("a")]),
                c("fact", [atom("ready")]),
            ],
            outputs: vec![("out".into(), Var(7))],
        };
        p.phase_budget = bound;
        assert!(p.try_run(&rules, &q).unwrap_err().contains("service bound"));
        assert_eq!(p.computed, 1);
        assert_eq!(p.hits, 0);
        p.phase_budget = 200_000;
        oracle::same_raw(p.run(&rules, &q), oracle::run(&rules, &q, 200_000));
        assert_eq!(p.computed, 2);
        assert_eq!(p.hits, 0);
        oracle::same_raw(p.run(&rules, &q), oracle::run(&rules, &q, 200_000));
        assert_eq!(p.computed, 2);
        assert_eq!(p.hits, 1);
    }
}
