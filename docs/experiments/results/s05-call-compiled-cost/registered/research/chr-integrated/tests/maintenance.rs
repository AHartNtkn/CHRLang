use chr_integrated::{Engine, PreparedRuleset, Step};
use chr_syntax::{Answer, Query, Rule, Var, atom, c, eq, t, v};

fn matcher_entries(engine: &Engine, predicates: usize, arguments: usize) {
    let audit = engine.audit();
    assert!(audit.indexes_valid);
    assert_eq!(audit.matcher_predicate_entries, predicates);
    assert_eq!(audit.matcher_argument_entries, arguments);
    assert_eq!(audit.matcher_occurrence_incidence, arguments);
}

fn complete(engine: &mut Engine, expected: &Answer) {
    assert_eq!(engine.run(10_000), Step::Complete);
    assert!(engine.audit().complete);
    assert!(chr_observe::equivalent(
        &engine.answer().unwrap(),
        expected,
        &mut Default::default()
    ));
}

#[test]
fn output_only_occurrences_preserve_nested_binding_and_joint_aliases() {
    let prepared = PreparedRuleset::new(&[Rule::simplify(
        "bind",
        [c("bind", [v(0), v(1)])],
        eq(v(0), v(1)),
    )])
    .unwrap();
    let mut engine = prepared.start(&Query {
        constraints: vec![
            c("data", [v(0), v(1)]),
            c("nested", [t("box", [v(0)]), v(2)]),
            c("data", [v(0), v(1)]),
            c("bind", [v(0), v(1)]),
            c("bind", [v(1), t("pair", [atom("a"), v(2)])]),
        ],
        outputs: vec![
            ("x".into(), Var(0)),
            ("y".into(), Var(1)),
            ("free".into(), Var(2)),
        ],
    });
    // Only the two bind occurrences belong to matcher storage. The output-only
    // constructor still owns child incidence needed by equality maintenance.
    matcher_entries(&engine, 2, 4);
    let pair = t("pair", [atom("a"), v(9)]);
    complete(
        &mut engine,
        &Answer {
            outputs: vec![
                ("x".into(), pair.clone()),
                ("y".into(), pair.clone()),
                ("free".into(), v(9)),
            ],
            residual: vec![
                c("data", [pair.clone(), pair.clone()]),
                c("nested", [t("box", [pair.clone()]), v(9)]),
                c("data", [pair.clone(), pair]),
            ],
        },
    );
    matcher_entries(&engine, 0, 0);
    assert_eq!(engine.audit().live_occurrences, 3);
}

#[test]
fn mixed_source_and_output_signatures_keep_only_live_head_memberships() {
    let prepared = PreparedRuleset::new(&[
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
        Rule::propagate("read", [c("p", [t("f", [v(0)])])], c("out", [v(0)]).into()),
    ])
    .unwrap();
    let mut engine = prepared.start(&Query {
        constraints: vec![
            c("p", [v(0)]),
            c("p", [v(0), v(1)]),
            c("out", [v(1)]),
            c("bind", [v(0), t("f", [v(1)])]),
            c("bind", [v(1), atom("a")]),
        ],
        outputs: vec![("value".into(), Var(0))],
    });
    matcher_entries(&engine, 3, 5);
    complete(
        &mut engine,
        &Answer {
            outputs: vec![("value".into(), t("f", [atom("a")]))],
            residual: vec![
                c("p", [t("f", [atom("a")])]),
                c("p", [t("f", [atom("a")]), atom("a")]),
                c("out", [atom("a")]),
                c("out", [atom("a")]),
            ],
        },
    );
    matcher_entries(&engine, 1, 1);
    assert_eq!(engine.audit().propagation_tokens, 1);
}
