#[allow(dead_code)]
#[path = "support/local_ports.rs"]
mod local;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_syntax::{Answer, Goal, Query, Rule, Var, and, atom, c, eq, t, v};
fn check(rules: &[Rule], q: &Query) -> Vec<Answer> {
    let p = local::multihead::Program::compile(rules).unwrap();
    let mut e = p.start(q);
    let mut done = false;
    for _ in 0..200_000 {
        if e.advance() {
            done = true;
            break;
        }
    }
    assert!(done, "unfinished multihead source");
    let got = e.answer().into_iter().collect::<Vec<_>>();
    let mut activated = p.start_mode::<true>(q, true);
    let mut done = false;
    for _ in 0..200_000 {
        if activated.advance() {
            done = true;
            break;
        }
    }
    assert!(done, "unfinished selective source");
    oracle::same_raw(activated.answer().into_iter().collect(), got.clone());
    let mut partial = p.start_partial::<true>(q);
    let mut done = false;
    for _ in 0..200_000 {
        partial.assert_cache_integrity();
        if partial.advance() {
            done = true;
            break;
        }
    }
    assert!(done, "unfinished partial join");
    oracle::same_raw(partial.answer().into_iter().collect(), got.clone());
    fn intermediate<const METRICS: bool>(
        p: &std::sync::Arc<local::multihead::Program>,
        q: &Query,
        expected: &[Answer],
    ) {
        let mut e = p.start_intermediate::<METRICS>(q);
        let mut done = false;
        for _ in 0..200_000 {
            e.assert_cache_integrity();
            if e.advance() {
                done = true;
                break;
            }
        }
        assert!(done, "unfinished intermediate join");
        oracle::same_raw(e.answer().into_iter().collect(), expected.to_vec());
    }
    intermediate::<true>(&p, q, &got);
    intermediate::<false>(&p, q, &got);
    println!(
        "INTERMEDIATE_SOURCE,rules={},facts={}",
        rules.len(),
        q.constraints.len()
    );
    oracle::same_raw(got.clone(), oracle::run(rules, q, 200_000));
    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
        let p = chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap();
        let mut search = p
            .start_search(q.clone(), chr_compiled::Policy::Global, access)
            .unwrap();
        let mut expected = vec![];
        let mut done = false;
        for _ in 0..200_000 {
            match search.tick() {
                chr_compiled::SearchEvent::Complete(mut b) => {
                    expected.push(b.engine.observe().unwrap())
                }
                chr_compiled::SearchEvent::Exhausted => {
                    done = true;
                    break;
                }
                _ => (),
            }
        }
        assert!(done);
        oracle::same_raw(got.clone(), expected);
    }
    got
}
#[test]
fn ordered_multiheads_history_and_late_aliases() {
    for duplicate in 1..=3 {
        for late in [false, true] {
            for reverse in [false, true] {
                for nested in [false, true] {
                    let pattern = if nested { t("f", [v(0)]) } else { v(0) };
                    let value = |x| if nested { t("f", [x]) } else { x };
                    let mut rules = vec![
                        Rule {
                            name: "remember".into(),
                            kept: vec![c("item", [pattern.clone()]), c("item", [pattern.clone()])],
                            removed: vec![],
                            guards: vec![],
                            body: c("pair", [v(0), v(10), v(10)]).into(),
                        },
                        Rule::simplify(
                            "consume",
                            [c("item", [pattern.clone()]), c("permit", [v(0)])],
                            c("winner", [atom("first"), v(0)]).into(),
                        ),
                        Rule::simplify(
                            "consume_other",
                            [c("item", [pattern]), c("permit", [v(0)])],
                            c("winner", [atom("second"), v(0)]).into(),
                        ),
                        Rule::simplify("link", [c("link", [v(0), v(1)])], eq(v(0), v(1))),
                    ];
                    if reverse {
                        rules.swap(1, 2);
                    }
                    let mut constraints = (0..duplicate)
                        .map(|_| c("item", [value(v(100))]))
                        .collect::<Vec<_>>();
                    constraints.push(c("item", [value(if late { v(200) } else { v(100) })]));
                    if !late {
                        constraints.push(c("permit", [v(200)]));
                    }
                    if late {
                        rules[3].body = and(vec![eq(v(0), v(1)), c("permit", [v(0)]).into()]);
                    }
                    constraints.push(c("link", [v(100), v(200)]));
                    if reverse {
                        constraints.reverse();
                    }
                    let q = Query {
                        constraints,
                        outputs: vec![("x".into(), Var(100)), ("y".into(), Var(200))],
                    };
                    let got = check(&rules, &q);
                    assert_eq!(got.len(), 1);
                    let winner = got[0].residual.iter().find(|c| c.name == "winner").unwrap();
                    assert_eq!(
                        winner.args[0],
                        atom(if reverse { "second" } else { "first" })
                    );
                }
            }
        }
    }
}
#[test]
fn body_barrier_freshness_and_failure() {
    for effect in 0..4 {
        let tail = match effect {
            0 => eq(v(0), atom("a")),
            1 => eq(v(0), atom("b")),
            2 => eq(v(0), t("f", [v(0)])),
            _ => Goal::Fail,
        };
        let rules = vec![
            Rule::simplify(
                "early",
                [c("ready", [v(0)])],
                c("winner", [atom("early")]).into(),
            ),
            Rule::simplify(
                "start",
                [c("start", [v(0)])],
                and(vec![
                    c("ready", [v(0)]).into(),
                    eq(v(0), atom("a")),
                    tail,
                    c("fresh", [v(1), v(1), v(2)]).into(),
                ]),
            ),
        ];
        let q = Query {
            constraints: vec![c("start", [v(100)])],
            outputs: vec![("x".into(), Var(100))],
        };
        let got = check(&rules, &q);
        assert_eq!(got.len(), usize::from(effect == 0));
    }
}
#[test]
fn unsupported_search_and_guards_are_rejected() {
    let r = Rule::simplify(
        "choice",
        [c("x", [])],
        chr_syntax::or(Goal::True, Goal::Fail),
    );
    assert!(local::multihead::Program::compile(&[r]).is_err());
    let mut guarded = Rule::simplify("guarded", [c("x", [v(0)])], Goal::True);
    guarded
        .guards
        .push(chr_syntax::Guard::Equal(v(0), atom("a")));
    assert!(local::multihead::Program::compile(&[guarded]).is_err());
    assert!(
        local::multihead::Program::compile(&[Rule::simplify("empty", [], Goal::True)]).is_err()
    );
}

#[test]
fn kept_and_removed_heads_require_distinct_occurrences_and_keep_identity() {
    let rules = vec![Rule {
        name: "keep_one".into(),
        kept: vec![c("item", [v(0)])],
        removed: vec![c("item", [v(0)])],
        guards: vec![],
        body: c("used", [v(0)]).into(),
    }];
    let prepared = local::multihead::Program::compile(&rules).unwrap();
    for count in 1..=4 {
        for seed in 0..3 {
            let q = Query {
                constraints: (0..count).map(|_| c("item", [v(100 + seed)])).collect(),
                outputs: vec![("x".into(), Var(100 + seed))],
            };
            let got = check(&rules, &q);
            let a = &got[0];
            assert_eq!(a.residual.iter().filter(|c| c.name == "item").count(), 1);
            assert_eq!(
                a.residual.iter().filter(|c| c.name == "used").count(),
                count - 1
            );
            let mut run = prepared.start(&q);
            for _ in 0..10 {
                if run.advance() {
                    break;
                }
            }
            oracle::same_raw(run.answer().into_iter().collect(), got);
        }
    }
}
#[test]
fn duplicate_kept_occurrences_generate_ordered_history_entries() {
    let rules = vec![Rule {
        name: "pairs".into(),
        kept: vec![c("item", [v(0)]), c("item", [v(0)])],
        removed: vec![],
        guards: vec![],
        body: c("pair", [v(0)]).into(),
    }];
    for count in 1..=4 {
        let q = Query {
            constraints: (0..count).map(|_| c("item", [atom("a")])).collect(),
            outputs: vec![],
        };
        let got = check(&rules, &q);
        assert_eq!(
            got[0].residual.iter().filter(|c| c.name == "pair").count(),
            count * (count - 1)
        );
    }
}

#[test]
fn body_completion_and_occurrence_order_select_the_actual_consumer() {
    let rules = vec![
        Rule::simplify(
            "late",
            [c("ready", [v(0)]), c("permit", [])],
            c("winner", [atom("late"), v(0)]).into(),
        ),
        Rule::simplify(
            "early",
            [c("ready", [v(0)])],
            c("winner", [atom("early"), v(0)]).into(),
        ),
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            and(vec![c("ready", [v(0)]).into(), c("permit", []).into()]),
        ),
    ];
    let q = Query {
        constraints: vec![c("start", [atom("a")])],
        outputs: vec![],
    };
    let got = check(&rules, &q);
    assert_eq!(
        got[0].residual,
        vec![c("winner", [atom("late"), atom("a")])]
    );
    let rules = vec![Rule::simplify(
        "choose_first",
        [c("item", [v(0)]), c("permit", [])],
        c("winner", [v(0)]).into(),
    )];
    for reverse in [false, true] {
        let mut constraints = vec![c("item", [atom("a")]), c("item", [atom("b")])];
        if reverse {
            constraints.reverse();
        }
        constraints.push(c("permit", []));
        let got = check(
            &rules,
            &Query {
                constraints,
                outputs: vec![],
            },
        );
        assert!(
            got[0]
                .residual
                .contains(&c("winner", [atom(if reverse { "b" } else { "a" })]))
        );
    }
}

#[test]
fn selective_sparse_broad_and_nested_work() {
    fn nat(n: usize) -> chr_syntax::Term {
        (0..n).fold(atom("z"), |x, _| t("s", [x]))
    }
    for family in ["sparse", "broad", "nested", "cold", "dense"] {
        for width in [4, 16, 64] {
            let pattern = if family == "nested" {
                t("pair", [v(0), v(0)])
            } else {
                t("f", [v(0)])
            };
            let mut rules = vec![
                Rule::simplify(
                    "consume",
                    [c("request", [pattern]), c("permit", [])],
                    c("hit", [v(0)]).into(),
                ),
                Rule::simplify(
                    "tick",
                    [c("tick", [t("s", [v(0)])])],
                    c("tick", [v(0)]).into(),
                ),
                Rule::simplify("end_tick", [c("tick", [atom("z")])], Goal::True),
                Rule::simplify("link", [c("link", [v(0), v(1)])], eq(v(0), v(1))),
                Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), t("f", [atom("a")]))),
            ];
            let mut constraints = vec![c("permit", []), c("tick", [nat(width)])];
            for i in 0..width {
                let input = if family == "nested" {
                    t("pair", [v(100 + i as u64), v(1000 + i as u64)])
                } else {
                    v(100 + i as u64)
                };
                constraints.push(c("request", [input]));
            }
            if family == "broad" {
                for i in 1..width {
                    constraints.push(c("link", [v(100), v(100 + i as u64)]));
                }
            }
            if family == "nested" {
                constraints.push(c(
                    "link",
                    [v(100 + width as u64 / 2), v(1000 + width as u64 / 2)],
                ));
            } else {
                constraints.push(c("bind", [v(100 + width as u64 / 2)]));
            }
            if family == "cold" {
                rules = vec![Rule::simplify(
                    "cold",
                    [c("request", [t("f", [v(0)])]), c("partner", [v(0)])],
                    Goal::True,
                )];
                constraints.retain(|c| c.name == "request");
                for _ in 0..width {
                    constraints.push(c("partner", [atom("a")]));
                }
            }
            if family == "dense" {
                rules = vec![Rule::simplify(
                    "dense",
                    [c("left", [v(0)]), c("right", [t("f", [v(0)])])],
                    Goal::True,
                )];
                constraints.clear();
                for i in 0..width {
                    constraints.push(c("left", [v(100)]));
                    constraints.push(c("right", [v(1000 + i as u64)]));
                }
            }
            let q = Query {
                constraints,
                outputs: vec![],
            };
            let expected = check(&rules, &q);
            let p = local::multihead::Program::compile(&rules).unwrap();
            for mode in 0..3 {
                let selective = mode > 0;
                let partial = mode == 2;
                let mut e = if partial {
                    p.start_partial::<true>(&q)
                } else {
                    p.start_mode::<true>(&q, selective)
                };
                let mut done = false;
                for _ in 0..200_000 {
                    e.assert_cache_integrity();
                    if e.advance() {
                        done = true;
                        break;
                    }
                }
                assert!(done);
                e.assert_cache_integrity();
                oracle::same_raw(e.answer().into_iter().collect(), expected.clone());
                if family == "cold" {
                    assert_eq!(
                        e.work.head_attempts.get(),
                        if selective && !partial {
                            width * width
                        } else {
                            width
                        }
                    );
                }
                if family == "dense" && selective {
                    assert_eq!(
                        e.work.peak_tuples,
                        width * width + if partial { width } else { 0 }
                    );
                }
                println!(
                    "{{\"family\":\"{family}\",\"width\":{width},\"selective\":{selective},\"partial\":{partial},\"heads\":{},\"fact_visits\":{},\"combinations\":{},\"inspections\":{},\"registrations\":{},\"notifications\":{},\"wakeups\":{},\"changed_handles\":{},\"peak_tuples\":{}}}",
                    e.work.head_attempts.get(),
                    e.work.fact_visits.get(),
                    e.work.combinations.get(),
                    e.work.inspections,
                    e.work.registrations,
                    e.work.notifications,
                    e.work.wakeups,
                    e.work.changed_handles,
                    e.work.peak_tuples
                );
                let mut off = if partial {
                    p.start_partial::<false>(&q)
                } else {
                    p.start_mode::<false>(&q, selective)
                };
                let mut done = false;
                for _ in 0..200_000 {
                    if off.advance() {
                        done = true;
                        break;
                    }
                }
                assert!(done);
                oracle::same_raw(off.answer().into_iter().collect(), expected.clone());
                assert_eq!(off.work.head_attempts.get(), 0);
                assert_eq!(off.work.registrations, 0);
            }
        }
    }
}

#[test]
fn three_head_prefixes_accept_new_partners_and_consumption_invalidates_waiters() {
    for initially_known in [false, true] {
        for killed in [false, true] {
            for middle_count in [1, 3] {
                let rules = vec![
                    Rule {
                        name: "join_three".into(),
                        kept: vec![c("left", [t("f", [v(0)])]), c("middle", [v(0)])],
                        removed: vec![c("right", [v(0)])],
                        guards: vec![],
                        body: c("hit", [v(0), v(1), v(1)]).into(),
                    },
                    Rule::simplify("kill", [c("left", [v(0)]), c("kill", [])], Goal::True),
                    Rule::simplify(
                        "go",
                        [c("go", [v(0)])],
                        and(vec![
                            eq(v(0), t("f", [atom("a")])),
                            c("middle", [atom("a")]).into(),
                            c("right", [atom("a")]).into(),
                            c("right", [atom("a")]).into(),
                        ]),
                    ),
                ];
                let mut constraints = vec![
                    c(
                        "left",
                        [if initially_known {
                            t("f", [atom("a")])
                        } else {
                            v(100)
                        }],
                    ),
                    c("go", [v(100)]),
                ];
                for _ in 0..middle_count {
                    constraints.push(c("middle", [atom("a")]));
                }
                if killed {
                    constraints.push(c("kill", []));
                }
                let got = check(
                    &rules,
                    &Query {
                        constraints,
                        outputs: vec![("x".into(), Var(100))],
                    },
                );
                assert_eq!(
                    got[0].residual.iter().filter(|c| c.name == "hit").count(),
                    if killed { 0 } else { 2 }
                );
            }
        }
    }
}

#[test]
fn proper_intermediates_do_not_retain_the_final_partner_product() {
    let rules = vec![Rule {
        name: "join".into(),
        kept: vec![c("left", [v(0)]), c("middle", [v(1)])],
        removed: vec![c("right", [v(0), v(1), v(2)])],
        guards: vec![],
        body: eq(v(2), atom("hit")),
    }];
    for n in [2, 4, 8] {
        let mut facts = vec![];
        for i in 0..n {
            facts.push(c("left", [atom(&format!("k{i}"))]));
            facts.push(c("middle", [atom(&format!("k{i}"))]));
            facts.push(c(
                "right",
                [
                    atom(&format!("k{}", n - 1)),
                    atom(&format!("k{}", n - 1)),
                    v(i as u64),
                ],
            ));
        }
        let q = Query {
            constraints: facts,
            outputs: (0..n).map(|i| (format!("v{i}"), Var(i as u64))).collect(),
        };
        let p = local::multihead::Program::compile(&rules).unwrap();
        let e = p.start_intermediate::<true>(&q);
        assert_eq!(
            e.work.peak_tuples,
            n + n * n,
            "final partners must not be retained"
        );
        let expected = check(&rules, &q);
        for (name, mut e) in [
            ("scan", p.start_mode::<true>(&q, false)),
            ("partial", p.start_partial::<true>(&q)),
            ("intermediate", e),
        ] {
            let mut done = false;
            for _ in 0..200_000 {
                e.assert_cache_integrity();
                if e.advance() {
                    done = true;
                    break;
                }
            }
            assert!(done);
            oracle::same_raw(e.answer().into_iter().collect(), expected.clone());
            println!(
                "PROPER_JOIN,size={n},mode={name},heads={},facts={},intermediates={},peak={},firings={}",
                e.work.head_attempts.get(),
                e.work.fact_visits.get(),
                e.work.intermediate_visits.get(),
                e.work.peak_tuples,
                e.firings
            );
        }
    }
}

#[test]
fn consuming_many_prefix_occurrences_invalidates_intermediates() {
    for n in [2, 4, 8] {
        let rules = vec![
            Rule {
                name: "invalidate".into(),
                kept: vec![c("kill", [])],
                removed: vec![c("left", [v(0)])],
                guards: vec![],
                body: Goal::True,
            },
            Rule {
                name: "join".into(),
                kept: vec![c("left", [v(0)]), c("middle", [v(1)])],
                removed: vec![c("right", [v(0), v(1)])],
                guards: vec![],
                body: c("hit", []).into(),
            },
        ];
        let mut facts = vec![c("kill", [])];
        for i in 0..n {
            facts.push(c("left", [atom(&format!("k{i}"))]));
            facts.push(c("middle", [atom(&format!("k{i}"))]));
            facts.push(c("right", [atom(&format!("k{i}")), atom(&format!("k{i}"))]));
        }
        let answer = check(
            &rules,
            &Query {
                constraints: facts,
                outputs: vec![],
            },
        );
        assert!(
            answer[0]
                .residual
                .iter()
                .all(|c| c.name != "left" && c.name != "hit")
        );
        assert_eq!(
            answer[0]
                .residual
                .iter()
                .filter(|c| c.name == "right")
                .count(),
            n
        );
    }
}
