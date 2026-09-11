#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../examples/support/stream_run.rs"]
mod runtime;
#[path = "../examples/support/stream_source.rs"]
mod source;
use runtime::{Event, MODES, Prepared};
use source::Schema;

#[test]
fn completed_histories_have_conditions_no_pending_task_can_select() {
    let schema = Schema {
        family: "repeated",
        resource: true,
        fail_tail: false,
        work: 3,
        payload: 4,
    };
    let p = chr_direct_choice::demand::Prepared::with_reuse(
        schema.rules(),
        chr_direct_choice::demand::Reuse::MatchDependencies,
    )
    .unwrap();
    let mut run = p.start(schema.query(16, false)).unwrap();
    let mut seen = 0;
    for _ in 0..200_000 {
        match run.tick() {
            chr_direct_choice::demand::Event::Answer(_) => {
                seen += 1;
                if seen == 4 {
                    let before = run.retained_graph();
                    let supports = run.future_supports();
                    assert!(supports.pending_tasks > 0);
                    assert!(supports.dead_results > 0);
                    assert!(supports.dead_obligations > 0);
                    assert_eq!(before, run.retained_graph());
                    assert_eq!(supports, run.future_supports());
                    let removed = run.reclaim_incompatible_supports();
                    assert_eq!(removed.results, supports.dead_results);
                    assert_eq!(removed.consumption_claims, supports.dead_consumption_claims);
                    let after = run.future_supports();
                    assert_eq!(after.results, supports.results - supports.dead_results);
                    assert_eq!(
                        after.consumption_claims,
                        supports.consumption_claims - supports.dead_consumption_claims
                    );
                    assert_eq!(after.dead_results, 0);
                    assert_eq!(after.dead_consumption_claims, 0);
                    assert_eq!(after.obligations, supports.obligations);
                    assert_eq!(after.births, supports.births);
                    assert_eq!(after.pending_tasks, supports.pending_tasks);
                    assert_eq!(run.retained_graph().nodes, before.nodes);
                    assert_eq!(run.reclaim_incompatible_supports(), Default::default());
                }
            }
            chr_direct_choice::demand::Event::Exhausted => break,
            _ => (),
        }
    }
    assert_eq!(seen, 17);
    let s = run.future_supports();
    assert_eq!(s.pending_tasks, 0);
    assert_eq!(s.dead_results, s.results);
    assert_eq!(s.dead_obligations, s.obligations);
    assert_eq!(s.dead_births, s.births);
    assert_eq!(s.dead_consumption_claims, s.consumption_claims);
}

fn compare_reclaimed_delivery(
    rules: Vec<chr_syntax::Rule>,
    query: chr_syntax::Query,
    reuse: chr_direct_choice::demand::Reuse,
    templates: bool,
    every_tick: bool,
) -> usize {
    let expected = oracle::run(&rules, &query, 200_000);
    let (removed, answers) =
        paired_graph_delivery(rules, query, reuse, templates, every_tick, false);
    oracle::same_raw(answers, expected);
    removed
}

fn paired_graph_delivery(
    rules: Vec<chr_syntax::Rule>,
    query: chr_syntax::Query,
    reuse: chr_direct_choice::demand::Reuse,
    templates: bool,
    every_tick: bool,
    pull_tabs: bool,
) -> (usize, Vec<chr_syntax::Answer>) {
    use chr_direct_choice::demand::{Event as E, Prepared as P};
    let p = P::with_reuse(rules, reuse).unwrap();
    let p = if templates {
        p.with_derivation_templates()
    } else {
        p
    };
    let p = if pull_tabs { p.with_pull_tabs() } else { p };
    let mut control = p.start(query.clone()).unwrap();
    let mut candidate = p.start(query).unwrap();
    let mut answers = vec![];
    let mut removed = 0;
    let mut done = false;
    for _ in 0..200_000 {
        let a = control.tick();
        let b = candidate.tick();
        let observation = matches!(&b, E::Answer(_));
        match (a, b) {
            (E::Answer(a), E::Answer(b)) => {
                oracle::same_raw(vec![a], vec![b.clone()]);
                answers.push(b);
            }
            (E::Progress, E::Progress) => (),
            (E::Exhausted, E::Exhausted) => {
                done = true;
                break;
            }
            _ => panic!("reclamation changed service or delivery"),
        }
        if every_tick || observation {
            let before = candidate.retained_graph();
            let s = candidate.reclaim_incompatible_supports();
            removed += s.results + s.consumption_claims;
            let after = candidate.retained_graph();
            assert_eq!(
                (
                    before.nodes,
                    before.calls,
                    before.choices,
                    before.births,
                    before.obligations
                ),
                (
                    after.nodes,
                    after.calls,
                    after.choices,
                    after.births,
                    after.obligations
                )
            );
        }
    }
    assert!(done);
    (removed, answers)
}

#[test]
fn reclamation_preserves_independent_answers_and_each_service_boundary() {
    use chr_direct_choice::demand::Reuse;
    let mut removed = 0;
    for family in ["repeated", "distinct", "aliases"] {
        for resource in [false, true] {
            for fail_tail in [false, true] {
                for n in [0, 4, 16] {
                    for work in [0, 3] {
                        for payload in [0, 4] {
                            let schema = Schema {
                                family,
                                resource,
                                fail_tail,
                                work,
                                payload,
                            };
                            for reverse in [false, true] {
                                for reuse in [
                                    Reuse::CurrentContext,
                                    Reuse::StaticBirth,
                                    Reuse::MatchDependencies,
                                ] {
                                    for templates in [false, true] {
                                        for every_tick in [false, true] {
                                            removed += compare_reclaimed_delivery(
                                                schema.rules(),
                                                schema.query(n, reverse),
                                                reuse,
                                                templates,
                                                every_tick,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(removed > 0);
}

#[test]
fn a_consumed_resource_needed_by_a_pending_answer_stays_consumed() {
    use chr_direct_choice::demand::{Event as E, Prepared as P, Reuse};
    let schema = Schema {
        family: "repeated",
        resource: true,
        fail_tail: false,
        work: 4,
        payload: 8,
    };
    let p = P::with_reuse(schema.rules(), Reuse::MatchDependencies)
        .unwrap()
        .with_derivation_templates();
    let mut run = p.start(schema.query(64, false)).unwrap();
    let mut answers = vec![];
    let mut checked = false;
    let mut done = false;
    for _ in 0..200_000 {
        match run.tick() {
            E::Answer(a) => {
                answers.push(a);
                if answers.len() == 64 {
                    let before = run.future_supports();
                    assert!(before.consumption_claims > before.dead_consumption_claims);
                    let removed = run.reclaim_incompatible_supports();
                    assert_eq!(removed.consumption_claims, before.dead_consumption_claims);
                    assert!(run.future_supports().consumption_claims > 0);
                    checked = true;
                }
            }
            E::Exhausted => {
                done = true;
                break;
            }
            E::Progress => (),
        }
    }
    assert!(checked && done);
    oracle::same_raw(
        answers,
        oracle::run(&schema.rules(), &schema.query(64, false), 200_000),
    );
}

#[test]
fn reclamation_preserves_competing_consumers_and_shared_choices() {
    use chr_direct_choice::demand::Reuse;
    use chr_syntax::{Query, Rule, Var, atom, c, eq, or, t, v};
    let rules = vec![
        Rule::simplify(
            "choose",
            [c("choose", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify(
            "take",
            [c("take", [v(0), v(1)]), c("token", [])],
            eq(v(1), t("taken", [v(0)])),
        ),
    ];
    for tokens in [0, 1, 2] {
        for reverse in [false, true] {
            let mut constraints = vec![
                c("choose", [v(0)]),
                c("take", [v(0), v(1)]),
                c("take", [atom("b"), v(2)]),
            ];
            constraints.extend((0..tokens).map(|_| c("token", [])));
            if reverse {
                constraints.reverse();
            }
            let query = Query {
                constraints,
                outputs: vec![("first".into(), Var(1)), ("second".into(), Var(2))],
            };
            for reuse in [
                Reuse::CurrentContext,
                Reuse::StaticBirth,
                Reuse::MatchDependencies,
            ] {
                for templates in [false, true] {
                    for every_tick in [false, true] {
                        let (_, answers) = paired_graph_delivery(
                            rules.clone(),
                            query.clone(),
                            reuse,
                            templates,
                            every_tick,
                            false,
                        );
                        if tokens == 1 {
                            // Round-robin graph service advances past the interrupted
                            // first consumer when its shared choice splits. The second
                            // consumer wins. The fixed scalar order differs when forward.
                            let expected = |first_wins: bool| {
                                ["a", "b"]
                                    .into_iter()
                                    .map(|tag| chr_syntax::Answer {
                                        outputs: vec![
                                            (
                                                "first".into(),
                                                if first_wins {
                                                    t("taken", [atom(tag)])
                                                } else {
                                                    v(42)
                                                },
                                            ),
                                            (
                                                "second".into(),
                                                if first_wins {
                                                    v(43)
                                                } else {
                                                    t("taken", [atom("b")])
                                                },
                                            ),
                                        ],
                                        residual: vec![c(
                                            "take",
                                            [
                                                atom(if first_wins { "b" } else { tag }),
                                                v(if first_wins { 43 } else { 42 }),
                                            ],
                                        )],
                                    })
                                    .collect::<Vec<_>>()
                            };
                            oracle::same_raw(answers, expected(false));
                            oracle::same_raw(
                                oracle::run(&rules, &query, 200_000),
                                expected(!reverse),
                            );
                        } else {
                            oracle::same_raw(answers, oracle::run(&rules, &query, 200_000));
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn reclamation_preserves_lifted_argument_choices() {
    use chr_direct_choice::demand::Reuse;
    use chr_syntax::{Query, Rule, Var, atom, c, eq, or, v};
    let rules = vec![
        Rule::simplify(
            "choose",
            [c("choose", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify(
            "a",
            [c("use", [atom("a"), v(0)]), c("token", [])],
            eq(v(0), atom("left")),
        ),
        Rule::simplify(
            "b",
            [c("use", [atom("b"), v(0)]), c("token", [])],
            eq(v(0), atom("right")),
        ),
    ];
    for reverse in [false, true] {
        let mut constraints = vec![c("choose", [v(0)]), c("use", [v(0), v(1)]), c("token", [])];
        if reverse {
            constraints.reverse();
        }
        let q = Query {
            constraints,
            outputs: vec![("result".into(), Var(1))],
        };
        let expected = oracle::run(&rules, &q, 200_000);
        for reuse in [
            Reuse::CurrentContext,
            Reuse::StaticBirth,
            Reuse::MatchDependencies,
        ] {
            for templates in [false, true] {
                for every_tick in [false, true] {
                    let (_, answers) = paired_graph_delivery(
                        rules.clone(),
                        q.clone(),
                        reuse,
                        templates,
                        every_tick,
                        true,
                    );
                    oracle::same_raw(answers, expected.clone());
                }
            }
        }
    }
}

#[test]
fn progressive_source_has_independent_complete_answers() {
    for family in ["repeated", "distinct", "aliases"] {
        for resource in [false, true] {
            for fail_tail in [false, true] {
                for n in [0, 1, 4, 16] {
                    for work in [0, 3] {
                        for payload in [0, 4] {
                            let schema = Schema {
                                family,
                                resource,
                                fail_tail,
                                work,
                                payload,
                            };
                            for reverse in [false, true] {
                                let answers = oracle::run(
                                    &schema.rules(),
                                    &schema.query(n, reverse),
                                    200_000,
                                );
                                oracle::same_raw(answers, schema.expected(n));
                                for mode in MODES {
                                    let prepared = Prepared::new(mode, schema, schema.rules());
                                    let mut run = prepared.start(schema.query(n, reverse));
                                    let mut actual = vec![];
                                    let mut done = false;
                                    for _ in 0..200_000 {
                                        match run.tick() {
                                            Event::Answer(a) => actual.push(a),
                                            Event::Done => {
                                                done = true;
                                                break;
                                            }
                                            Event::Progress => (),
                                        }
                                    }
                                    assert!(done, "{mode} {family} {n}");
                                    oracle::same_raw(actual, schema.expected(n));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn consumer_release_and_cancellation_preserve_valid_delivery() {
    for family in ["repeated", "distinct", "aliases"] {
        let schema = Schema {
            family,
            resource: true,
            fail_tail: true,
            work: 3,
            payload: 4,
        };
        let expected = oracle::run(&schema.rules(), &schema.query(16, false), 200_000);
        for mode in MODES {
            let p = Prepared::new(mode, schema, schema.rules());
            for keep in [0, 4, usize::MAX] {
                for stop in [Some(4), None] {
                    let mut run = p.start(schema.query(16, false));
                    let mut remaining = expected.clone();
                    let mut consumer = std::collections::VecDeque::new();
                    let mut count = 0;
                    let mut complete = false;
                    for _ in 0..200_000 {
                        match run.tick() {
                            Event::Answer(a) => {
                                let at = remaining
                                    .iter()
                                    .position(|e| {
                                        chr_observe::equivalent(e, &a, &mut Default::default())
                                    })
                                    .expect("invalid raw answer or multiplicity");
                                remaining.swap_remove(at);
                                count += 1;
                                consumer.push_back(a);
                                if consumer.len() > keep {
                                    consumer.pop_front();
                                }
                                if stop == Some(count) {
                                    break;
                                }
                            }
                            Event::Done => {
                                complete = true;
                                break;
                            }
                            Event::Progress => (),
                        }
                    }
                    assert_eq!(complete, stop.is_none());
                    assert_eq!(count, stop.unwrap_or(16));
                    assert_eq!(consumer.len(), count.min(keep));
                    drop(consumer);
                    drop(run);
                }
            }
        }
    }
}
