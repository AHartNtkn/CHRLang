//! Full-source witnesses for head multiplicity, overlap, and unary reformulation.
#[allow(dead_code)]
mod composition_support;
#[allow(dead_code)]
mod runtime_support;
use chr_syntax::{Answer, Query, Rule, atom, c, eq, v};
fn check(
    rules: &[Rule],
    constraints: Vec<chr_syntax::Constraint>,
    residual: Vec<chr_syntax::Constraint>,
) {
    let q = Query {
        constraints,
        outputs: vec![],
    };
    let expected = vec![Answer {
        outputs: vec![],
        residual,
    }];
    runtime_support::same_raw(runtime_support::run(rules, &q, 100_000), expected.clone());
    #[cfg(feature = "head-dispatch")]
    {
        let p = chr_direct_conditional::engine::PreparedRuleset::with_head_contract(
            rules.to_vec(),
            None,
            chr_direct_conditional::engine::HeadAdmission::Optional,
        )
        .unwrap();
        runtime_support::same_raw(
            composition_support::Engine::Conditional(p.start(q.clone()).unwrap()).collect(),
            expected.clone(),
        );
    }
    for mode in [0, 1, 5, 7] {
        runtime_support::same_raw(
            composition_support::Engine::new(mode, rules, &q).collect(),
            expected.clone(),
        );
    }
}
#[test]
fn unary_rules_can_compete_and_source_priority_changes_the_winner() {
    let general = Rule::simplify("general", [c("p", [v(0)])], c("general_won", []).into());
    let specific = Rule::simplify(
        "specific",
        [c("p", [atom("a")])],
        c("specific_won", []).into(),
    );
    check(
        &[general.clone(), specific.clone()],
        vec![c("p", [atom("a")])],
        vec![c("general_won", [])],
    );
    check(
        &[specific, general],
        vec![c("p", [atom("a")])],
        vec![c("specific_won", [])],
    );
}
#[test]
fn disjoint_unary_patterns_still_need_late_activation_and_multiplicity() {
    let rules = vec![
        Rule::simplify("a", [c("p", [atom("a")])], c("done_a", []).into()),
        Rule::simplify("b", [c("p", [atom("b")])], c("done_b", []).into()),
        Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("b"))),
    ];
    check(
        &rules,
        vec![
            c("p", [v(100)]),
            c("bind", [v(100)]),
            c("p", [atom("a")]),
            c("p", [atom("a")]),
        ],
        vec![c("done_a", []), c("done_a", []), c("done_b", [])],
    );
}
#[test]
fn a_single_unary_propagation_rule_still_requires_once_per_occurrence_history() {
    let rule = Rule::propagate("visit", [c("p", [v(0)])], c("mark", [v(0)]).into());
    check(
        &[rule],
        vec![c("p", [atom("a")]), c("p", [atom("a")])],
        vec![
            c("p", [atom("a")]),
            c("p", [atom("a")]),
            c("mark", [atom("a")]),
            c("mark", [atom("a")]),
        ],
    );
}
#[test]
fn one_multihead_rule_has_conflicting_instances() {
    use chr_relational::contextual::Store;
    let mut store = Store::default();
    for x in ["a", "b", "c"] {
        let value = store.constructor(x, &[]);
        store.post("p", &[value]);
    }
    let heads = [c("p", [v(0)]), c("p", [v(1)])];
    let matches = store.matches(&[], &heads);
    assert_eq!(matches.len(), 6);
    assert!(matches.iter().enumerate().any(|(i, a)| {
        matches
            .iter()
            .skip(i + 1)
            .any(|b| a.removed.iter().any(|id| b.removed.contains(id)))
    }));
    let rule = Rule::simplify("pair", heads, c("pair", [v(0), v(1)]).into());
    check(
        &[rule],
        vec![
            c("p", [atom("a")]),
            c("p", [atom("b")]),
            c("p", [atom("c")]),
        ],
        vec![c("pair", [atom("a"), atom("b")]), c("p", [atom("c")])],
    );
}
#[test]
fn sequential_reservation_changes_atomic_join_behavior() {
    let rival = Rule::simplify("rival", [c("p", [])], c("rival_won", []).into());
    let atomic = vec![
        Rule::simplify("join", [c("p", []), c("q", [])], c("join_won", []).into()),
        rival.clone(),
    ];
    let sequential = vec![
        Rule::simplify("reserve", [c("p", [])], c("waiting", []).into()),
        Rule::simplify(
            "finish",
            [c("waiting", []), c("q", [])],
            c("join_won", []).into(),
        ),
        rival,
    ];
    for rules in [&atomic, &sequential] {
        check(rules, vec![c("p", []), c("q", [])], vec![c("join_won", [])]);
    }
    check(&atomic, vec![c("p", [])], vec![c("rival_won", [])]);
    check(&sequential, vec![c("p", [])], vec![c("waiting", [])]);
}

#[test]
fn bundling_moves_the_join_to_an_input_and_update_contract() {
    let atomic = vec![
        Rule::simplify("join", [c("p", []), c("q", [])], c("join_won", []).into()),
        Rule::simplify("rival", [c("p", [])], c("rival_won", []).into()),
    ];
    let bundled = vec![
        Rule::simplify(
            "join",
            [c("state", [atom("both")])],
            c("join_won", []).into(),
        ),
        Rule::simplify(
            "rival",
            [c("state", [atom("only_p")])],
            c("rival_won", []).into(),
        ),
        Rule::simplify("q", [c("state", [atom("only_q")])], c("q", []).into()),
    ];
    for (state, input, output) in [
        (
            "both",
            vec![c("p", []), c("q", [])],
            vec![c("join_won", [])],
        ),
        ("only_p", vec![c("p", [])], vec![c("rival_won", [])]),
        ("only_q", vec![c("q", [])], vec![c("q", [])]),
    ] {
        check(&atomic, input, output.clone());
        check(&bundled, vec![c("state", [atom(state)])], output);
    }
    // A linked producer operates on the public q occurrence in the original.
    let producer = Rule::simplify("produce", [c("start", [])], c("q", []).into());
    let linked = vec![atomic[0].clone(), producer.clone(), atomic[1].clone()];
    check(
        &linked,
        vec![c("p", []), c("start", [])],
        vec![c("join_won", [])],
    );
    let incorrectly_linked = vec![
        bundled[0].clone(),
        producer,
        bundled[1].clone(),
        bundled[2].clone(),
    ];
    check(
        &incorrectly_linked,
        vec![c("state", [atom("only_p")]), c("start", [])],
        vec![c("rival_won", []), c("q", [])],
    );
    // A coherent unary update instead owns the entire state and its command.
    let owned = vec![
        Rule::simplify(
            "produce",
            [c("owned", [atom("only_p"), atom("produce_q")])],
            c("state", [atom("both")]).into(),
        ),
        bundled[0].clone(),
    ];
    check(
        &owned,
        vec![c("owned", [atom("only_p"), atom("produce_q")])],
        vec![c("join_won", [])],
    );
    // Source alternatives preserve their complete observations after encoding.
    let choice = Rule::simplify(
        "choose",
        [c("choose", [])],
        chr_syntax::or(
            c("state", [atom("both")]).into(),
            c("state", [atom("only_p")]).into(),
        ),
    );
    let q = Query {
        constraints: vec![c("choose", [])],
        outputs: vec![],
    };
    let rules = vec![choice, bundled[0].clone(), bundled[1].clone()];
    let expected = vec![
        Answer {
            outputs: vec![],
            residual: vec![c("join_won", [])],
        },
        Answer {
            outputs: vec![],
            residual: vec![c("rival_won", [])],
        },
    ];
    let original_choice = Rule::simplify(
        "choose",
        [c("choose", [])],
        chr_syntax::or(
            chr_syntax::and([c("p", []).into(), c("q", []).into()]),
            c("p", []).into(),
        ),
    );
    let original = vec![original_choice, atomic[0].clone(), atomic[1].clone()];
    for rules in [rules, original] {
        runtime_support::same_raw(runtime_support::run(&rules, &q, 100_000), expected.clone());
        for mode in [0, 1, 5, 7] {
            runtime_support::same_raw(
                composition_support::Engine::new(mode, &rules, &q).collect(),
                expected.clone(),
            );
        }
    }
}

#[test]
fn a_unary_constructor_head_can_have_multiple_intermediate_environments() {
    use chr_relational::contextual::Store;
    use chr_syntax::t;
    let mut store = Store::default();
    let x = store.unknown();
    let y = store.unknown();
    let fx = store.constructor("f", &[x]);
    let fy = store.constructor("f", &[y]);
    store.equate(fx, fy);
    assert!(store.step());
    assert!(store.pending() > 0);
    store.post("p", &[fx]);
    let matches = store.matches(&[], &[c("p", [t("f", [v(0)])])]);
    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0].removed, matches[1].removed);
    assert_ne!(matches[0].bindings, matches[1].bindings);
    assert_ne!(store.root(x), store.root(y));
    assert!(store.step());
    assert_eq!(store.root(x), store.root(y));
}
