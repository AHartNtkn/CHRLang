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
