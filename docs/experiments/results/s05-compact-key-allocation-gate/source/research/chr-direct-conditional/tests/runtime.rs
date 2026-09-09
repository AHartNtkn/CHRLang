mod runtime_support;
use chr_direct_conditional::engine::{Event, PreparedRuleset};
use chr_syntax::{Answer, Constraint, Goal, Guard, Query, Rule, Var, and, atom, c, eq, or, t, v};
fn query(constraints: Vec<Constraint>, outputs: &[u64]) -> Query {
    Query {
        constraints,
        outputs: outputs.iter().map(|&i| (format!("o{i}"), Var(i))).collect(),
    }
}
fn run(prepared: &PreparedRuleset, input: Query) -> Vec<Answer> {
    let mut engine = prepared.start(input).unwrap();
    let mut answers = vec![];
    for _ in 0..1_000_000 {
        match engine.tick() {
            Event::Progress => (),
            Event::Answer(a) => answers.push(a),
            Event::Exhausted => return answers,
        }
    }
    panic!("finite candidate cutoff");
}
fn check(name: &str, rules: Vec<Rule>, input: Query, count: usize) {
    use chr_direct_conditional::engine::Trace;
    let expected = runtime_support::run_traced(&rules, &input, 100_000);
    assert_eq!(expected.len(), count, "oracle count: {name}");
    let mut engine = PreparedRuleset::new(rules).unwrap().start(input).unwrap();
    engine.enable_trace();
    let mut actual = vec![];
    let mut done = false;
    for _ in 0..1_000_000 {
        match engine.tick() {
            Event::Progress => (),
            Event::Answer(a) => actual.push(a),
            Event::Exhausted => {
                done = true;
                break;
            }
        }
    }
    assert!(done, "candidate cutoff: {name}");
    runtime_support::same_raw(actual, expected.iter().map(|(a, _)| a.clone()).collect());
    // Projection is test-only: the execution engine never discovers work this way.
    let variables = engine.supports().variable_count();
    assert!(variables < 16, "bounded trace projection");
    let mut projected = vec![];
    for bits in 0..(1usize << variables) {
        let world: Vec<_> = (0..variables).map(|i| bits & (1 << i) != 0).collect();
        let eval = |s| engine.supports().eval(s, &world);
        if eval(engine.store().failed()) || engine.trace().iter().any(|event| {
            matches!(event, Trace::Birth {support, choice} if !eval(*support) && eval(*choice))
        }) { continue; }
        projected.push(
            engine
                .trace()
                .iter()
                .filter_map(|event| match event {
                    Trace::Application { rule, support, .. } if eval(*support) => Some(*rule),
                    _ => None,
                })
                .collect::<Vec<_>>(),
        );
    }
    let mut expected_traces: Vec<_> = expected.into_iter().map(|(_, trace)| trace).collect();
    projected.sort();
    expected_traces.sort();
    assert_eq!(
        projected, expected_traces,
        "source application projections: {name}"
    );
}
#[test]
fn independent_full_source_witnesses() {
    check(
        "nonground residual",
        vec![],
        query(vec![c("pair", [v(0), v(0)])], &[0]),
        1,
    );
    let choose = Rule::simplify(
        "choose",
        [c("choose", [v(0)])],
        or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
    );
    check(
        "correlation",
        vec![choose.clone()],
        query(vec![c("choose", [v(0)]), c("pair", [v(0), v(0)])], &[0]),
        2,
    );
    check(
        "independent",
        vec![choose.clone()],
        query(vec![c("choose", [v(0)]), c("choose", [v(1)])], &[0, 1]),
        4,
    );
    check(
        "equal arms",
        vec![Rule::simplify(
            "dup",
            [c("start", [])],
            or(Goal::True, Goal::True),
        )],
        query(vec![c("start", [])], &[]),
        2,
    );
    check(
        "inactive birth",
        vec![Rule::simplify(
            "nested",
            [c("start", [])],
            or(Goal::True, or(Goal::True, Goal::True)),
        )],
        query(vec![c("start", [])], &[]),
        3,
    );
    check(
        "partial failure",
        vec![Rule::simplify(
            "choose",
            [c("start", [v(0)])],
            or(
                and(vec![eq(v(0), atom("a")), eq(atom("a"), atom("b"))]),
                eq(v(0), atom("b")),
            ),
        )],
        query(vec![c("start", [v(0)])], &[0]),
        1,
    );
    check(
        "off-output failure",
        vec![Rule::simplify("bad", [c("bad", [])], Goal::Fail)],
        query(vec![c("out", [atom("a")]), c("bad", [])], &[]),
        0,
    );
    check(
        "aliases",
        vec![Rule::simplify(
            "alias",
            [c("start", [v(0), v(1)])],
            and(vec![eq(v(0), v(1)), c("pair", [v(0), v(1)]).into()]),
        )],
        query(vec![c("start", [v(0), v(1)])], &[0, 1]),
        1,
    );
    check(
        "fresh locals",
        vec![Rule::simplify(
            "fresh",
            [c("start", [v(0)])],
            and(vec![
                c("out", [v(0), v(1)]).into(),
                c("link", [v(1), v(1)]).into(),
            ]),
        )],
        query(vec![c("start", [atom("a")]), c("start", [atom("b")])], &[]),
        1,
    );
    check(
        "propagation once",
        vec![
            Rule::propagate("seen", [c("p", [v(0)])], c("seen", [v(0)]).into()),
            Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
        ],
        query(vec![c("p", [v(0)]), c("bind", [v(0)])], &[0]),
        1,
    );
    check(
        "late constructor",
        vec![
            Rule::simplify(
                "structural",
                [c("p", [t("f", [v(0)])])],
                c("hit", [v(0)]).into(),
            ),
            Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), t("f", [atom("a")]))),
        ],
        query(vec![c("p", [v(0)]), c("bind", [v(0)])], &[0]),
        1,
    );
    let pair = Rule::simplify(
        "pair",
        [c("p", [v(0)]), c("q", [v(0)])],
        c("out", [v(0)]).into(),
    );
    check(
        "nonbinding join",
        vec![pair.clone()],
        query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]),
        1,
    );
    check(
        "late alias",
        vec![
            pair.clone(),
            Rule::simplify("join", [c("join", [v(0), v(1)])], eq(v(0), v(1))),
        ],
        query(
            vec![c("p", [v(0)]), c("q", [v(1)]), c("join", [v(0), v(1)])],
            &[0, 1],
        ),
        1,
    );
    check(
        "conditional consumption",
        vec![
            choose,
            Rule::simplify(
                "use",
                [c("task", [v(0)]), c("token", [])],
                c("out", [v(0)]).into(),
            ),
        ],
        query(
            vec![c("choose", [v(0)]), c("task", [v(0)]), c("token", [])],
            &[0],
        ),
        2,
    );
    let two = Rule::simplify("two", [c("p", []), c("p", [])], c("done", []).into());
    check(
        "distinct occurrences",
        vec![two.clone()],
        query(vec![c("p", []), c("p", [])], &[]),
        1,
    );
    check("one is not two", vec![two], query(vec![c("p", [])], &[]), 1);
    check(
        "simpagation",
        vec![Rule {
            name: "mixed".into(),
            kept: vec![c("p", [v(0)])],
            removed: vec![c("q", [v(0)])],
            guards: vec![],
            body: c("out", [v(0)]).into(),
        }],
        query(
            vec![
                c("p", [atom("a")]),
                c("q", [atom("a")]),
                c("q", [atom("a")]),
            ],
            &[],
        ),
        1,
    );
    check(
        "disjoint tuple",
        vec![
            Rule::simplify(
                "start",
                [c("start", [])],
                or(c("p", [atom("a")]).into(), c("q", [atom("a")]).into()),
            ),
            pair,
        ],
        query(vec![c("start", [])], &[]),
        2,
    );
    check(
        "two environments",
        vec![
            Rule::propagate("view", [c("p", [t("f", [v(0)])])], c("out", [v(0)]).into()),
            Rule::simplify(
                "choose",
                [c("choose", [v(0)])],
                or(eq(v(0), t("f", [atom("a")])), eq(v(0), t("f", [atom("b")]))),
            ),
        ],
        query(vec![c("choose", [v(0)]), c("p", [v(0)])], &[0]),
        2,
    );
    check(
        "priority after binding",
        vec![
            Rule::simplify(
                "first",
                [c("p", [atom("a")]), c("token", [])],
                c("first", []).into(),
            ),
            Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
            Rule::simplify(
                "second",
                [c("p", [v(0)]), c("token", [])],
                c("second", []).into(),
            ),
        ],
        query(vec![c("bind", [v(0)]), c("p", [v(0)]), c("token", [])], &[]),
        1,
    );
    check(
        "shared tail",
        vec![Rule::simplify(
            "start",
            [c("start", [v(0)])],
            and(vec![
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
                c("tail", [v(0)]).into(),
            ]),
        )],
        query(vec![c("start", [v(0)])], &[0]),
        2,
    );
    let mut local = Rule::simplify("local", [c("start", [])], c("out", [v(9)]).into());
    local.guards = vec![Guard::Equal(v(9), v(9))];
    check(
        "guard-body local",
        vec![local],
        query(vec![c("start", [])], &[]),
        1,
    );
}
#[test]
fn repeated_dynamic_births_and_prepared_query_independence() {
    let rules = vec![
        Rule::simplify(
            "base",
            [c("build", [atom("z"), v(0)])],
            eq(v(0), atom("nil")),
        ),
        Rule::simplify(
            "step",
            [c("build", [t("s", [v(0)]), v(1)])],
            and(vec![
                or(
                    eq(v(1), t("pair", [atom("a"), v(2)])),
                    eq(v(1), t("pair", [atom("b"), v(2)])),
                ),
                c("build", [v(0), v(2)]).into(),
            ]),
        ),
    ];
    let prepared = PreparedRuleset::new(rules.clone()).unwrap();
    for depth in [0, 1, 3, 2, 0, 4] {
        let mut n = atom("z");
        for _ in 0..depth {
            n = t("s", [n]);
        }
        let input = query(vec![c("build", [n, v(0)])], &[0]);
        let expected = runtime_support::run(&rules, &input, 100_000);
        assert_eq!(expected.len(), 1 << depth);
        runtime_support::same_raw(run(&prepared, input), expected);
    }
}
#[test]
fn finite_sibling_publishes_beside_recursive_activity() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(c("loop", []).into(), eq(v(0), atom("done"))),
        ),
        Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
    ];
    let prepared = PreparedRuleset::new(rules).unwrap();
    let mut engine = prepared
        .start(query(vec![c("start", [v(0)])], &[0]))
        .unwrap();
    for _ in 0..100_000 {
        match engine.tick() {
            Event::Progress => (),
            Event::Answer(a) => {
                runtime_support::same_raw(
                    vec![a],
                    vec![Answer {
                        outputs: vec![("o0".into(), atom("done"))],
                        residual: vec![],
                    }],
                );
                return;
            }
            Event::Exhausted => panic!("divergent sibling was treated as exhausted"),
        }
    }
    panic!("finite sibling starved");
}
#[test]
fn publication_is_not_repeated_when_divergent_support_adds_births() {
    let prepared = PreparedRuleset::new(vec![
        Rule::simplify(
            "start",
            [c("start", [])],
            or(c("loop", []).into(), c("done", []).into()),
        ),
        Rule::simplify(
            "loop",
            [c("loop", [])],
            or(c("loop", []).into(), c("loop", []).into()),
        ),
    ])
    .unwrap();
    let mut engine = prepared.start(query(vec![c("start", [])], &[])).unwrap();
    let mut found = false;
    for _ in 0..100_000 {
        match engine.tick() {
            Event::Progress => (),
            Event::Answer(a) => {
                runtime_support::same_raw(
                    vec![a],
                    vec![Answer {
                        outputs: vec![],
                        residual: vec![c("done", [])],
                    }],
                );
                found = true;
                break;
            }
            Event::Exhausted => panic!(),
        }
    }
    assert!(found, "finite branch starved");
    for _ in 0..5_000 {
        assert!(
            matches!(engine.tick(), Event::Progress),
            "published support must not repeat or imply exhausted divergence"
        );
    }
}

#[test]
fn adversarial_dependency_and_priority_witnesses() {
    check(
        "conditional occurs cycle",
        vec![Rule::simplify(
            "start",
            [c("start", [v(0), v(1)])],
            or(
                and(vec![eq(v(0), t("f", [v(1)])), eq(v(1), v(0))]),
                eq(v(0), atom("ok")),
            ),
        )],
        query(vec![c("start", [v(0), v(1)])], &[0, 1]),
        1,
    );
    let mut guard = Rule::simplify("guard", [c("p", [v(0)])], c("hit", [v(0)]).into());
    guard.guards = vec![Guard::Equal(v(0), atom("a"))];
    check(
        "same-value support expansion",
        vec![
            guard,
            Rule::simplify(
                "choose",
                [c("choose", [v(0)])],
                or(eq(v(0), atom("a")), eq(v(0), atom("a"))),
            ),
        ],
        query(vec![c("p", [v(0)]), c("choose", [v(0)])], &[0]),
        2,
    );
    check(
        "ordered tuple priority",
        vec![Rule::simplify(
            "take",
            [c("p", [v(0)]), c("token", [])],
            c("taken", [v(0)]).into(),
        )],
        query(
            vec![c("p", [atom("a")]), c("p", [atom("b")]), c("token", [])],
            &[],
        ),
        1,
    );
}

#[test]
fn supported_application_projection_and_shared_consumption() {
    use chr_direct_conditional::engine::Trace;
    let rules = vec![
        Rule::simplify(
            "choose",
            [c("choose", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify(
            "use",
            [c("task", [v(0)]), c("token", [])],
            c("out", [v(0)]).into(),
        ),
    ];
    let input = query(
        vec![c("choose", [v(0)]), c("task", [v(0)]), c("token", [])],
        &[0],
    );
    let expected = runtime_support::run_traced(&rules, &input, 100_000);
    let mut engine = PreparedRuleset::new(rules).unwrap().start(input).unwrap();
    engine.enable_trace();
    let mut actual = vec![];
    let mut done = false;
    for _ in 0..100_000 {
        match engine.tick() {
            Event::Progress => (),
            Event::Answer(a) => actual.push(a),
            Event::Exhausted => {
                done = true;
                break;
            }
        }
    }
    assert!(done);
    runtime_support::same_raw(actual, expected.iter().map(|(a, _)| a.clone()).collect());
    let applications: Vec<_> = engine
        .trace()
        .iter()
        .filter_map(|event| {
            if let Trace::Application { rule, ids, support } = event {
                Some((*rule, ids, *support))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(
        applications.len(),
        2,
        "one physical consumption shared across both histories"
    );
    assert_eq!(applications[0].1, &vec![0]);
    assert_eq!(applications[1].1, &vec![1, 2]);
    let mut projected = vec![];
    for choice in [false, true] {
        projected.push(
            applications
                .iter()
                .filter(|(_, _, s)| engine.supports().eval(*s, &[choice]))
                .map(|(rule, _, _)| *rule)
                .collect::<Vec<_>>(),
        );
    }
    let mut expected_traces: Vec<_> = expected.into_iter().map(|(_, trace)| trace).collect();
    projected.sort();
    expected_traces.sort();
    assert_eq!(projected, expected_traces);
    assert_eq!(
        engine
            .trace()
            .iter()
            .filter(|e| matches!(e, Trace::Birth { .. }))
            .count(),
        1
    );
}

#[test]
fn finite_crossing_corpus() {
    let values = [
        atom("a"),
        atom("b"),
        t("f", [atom("a")]),
        t("f", [atom("b")]),
    ];
    for left in &values {
        for right in &values {
            for mode in 0..3 {
                for reverse_rules in [false, true] {
                    for reverse_input in [false, true] {
                        let mut reaction = match mode {
                            0 => Rule::simplify(
                                "join",
                                [c("p", [v(0)]), c("q", [v(0)])],
                                c("hit", [v(0)]).into(),
                            ),
                            1 => Rule::propagate(
                                "view",
                                [c("p", [t("f", [v(0)])])],
                                c("hit", [v(0)]).into(),
                            ),
                            _ => Rule::simplify(
                                "guard",
                                [c("p", [v(0)]), c("q", [v(1)])],
                                c("hit", [v(0), v(1)]).into(),
                            ),
                        };
                        if mode == 2 {
                            reaction.guards = vec![Guard::Equal(v(0), v(1))];
                        }
                        let choose = Rule::simplify(
                            "choose",
                            [c("choose", [v(0)])],
                            or(eq(v(0), left.clone()), eq(v(0), right.clone())),
                        );
                        let mut rules = vec![reaction, choose];
                        if reverse_rules {
                            rules.reverse();
                        }
                        let mut constraints = vec![
                            c("p", [v(0)]),
                            c("q", [v(1)]),
                            c("choose", [v(0)]),
                            c("choose", [v(1)]),
                        ];
                        if reverse_input {
                            constraints.reverse();
                        }
                        check("finite crossing", rules, query(constraints, &[0, 1]), 4);
                    }
                }
            }
        }
    }
}
