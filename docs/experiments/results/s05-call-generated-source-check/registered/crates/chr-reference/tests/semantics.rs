use chr_reference::Search;
use chr_syntax::*;

fn query(constraints: Vec<Constraint>, vars: &[u64]) -> Query {
    Query {
        constraints,
        outputs: vars.iter().map(|id| (format!("v{id}"), Var(*id))).collect(),
    }
}
fn solve(rules: Vec<Rule>, q: Query) -> Vec<Answer> {
    let mut search = Search::new(rules, q).unwrap();
    let batch = search.advance(2000);
    assert!(
        batch.exhausted,
        "finite test did not exhaust: {:?}",
        search.stats()
    );
    batch.answers
}
fn values(answer: &Answer) -> Vec<Term> {
    answer.outputs.iter().map(|(_, t)| t.clone()).collect()
}

#[test]
fn simplification_resolves_outputs_and_entire_residual_store() {
    let rules = vec![Rule::simplify(
        "p",
        [c("p", [v(0)])],
        and([eq(v(0), atom("a")), c("q", [v(0)]).into()]),
    )];
    let a = solve(
        rules,
        query(vec![c("p", [v(7)]), c("untouched", [v(7)])], &[7]),
    );
    assert_eq!(a.len(), 1);
    assert_eq!(values(&a[0]), vec![atom("a")]);
    assert_eq!(
        a[0].residual,
        vec![c("q", [atom("a")]), c("untouched", [atom("a")])]
    );
}
#[test]
fn constructor_heads_do_not_instantiate_store_variables() {
    let rules = vec![Rule::simplify("p", [c("p", [t("s", [v(0)])])], Goal::Fail)];
    let a = solve(rules, query(vec![c("p", [v(8)])], &[8]));
    assert_eq!(a.len(), 1);
    assert_eq!(values(&a[0]), vec![v(0)]);
    assert_eq!(a[0].residual, vec![c("p", [v(0)])]);
}
#[test]
fn repeated_head_variables_need_entailed_equality() {
    let rules = vec![Rule::simplify(
        "same",
        [c("p", [v(0)]), c("q", [v(0)])],
        Goal::Fail,
    )];
    assert_eq!(
        solve(
            rules.clone(),
            query(vec![c("p", [v(3)]), c("q", [v(4)])], &[])
        )
        .len(),
        1
    );
    assert!(solve(rules, query(vec![c("p", [v(3)]), c("q", [v(3)])], &[])).is_empty());
}
#[test]
fn rule_heads_require_distinct_occurrences() {
    let rules = vec![Rule::simplify("two", [c("p", []), c("p", [])], Goal::Fail)];
    assert_eq!(solve(rules.clone(), query(vec![c("p", [])], &[])).len(), 1);
    assert!(solve(rules, query(vec![c("p", []), c("p", [])], &[])).is_empty());
}
#[test]
fn simpagation_keeps_one_resource_and_consumes_each_partner() {
    let rule = Rule {
        name: "join".into(),
        kept: vec![c("a", [v(0)])],
        removed: vec![c("b", [v(0)])],
        guards: vec![],
        body: c("seen", [v(0)]).into(),
    };
    let a = solve(
        vec![rule],
        query(
            vec![
                c("a", [atom("x")]),
                c("b", [atom("x")]),
                c("b", [atom("x")]),
            ],
            &[],
        ),
    );
    assert_eq!(a.len(), 1);
    assert_eq!(
        a[0].residual,
        vec![
            c("a", [atom("x")]),
            c("seen", [atom("x")]),
            c("seen", [atom("x")])
        ]
    );
}
#[test]
fn propagation_fires_once_per_occurrence_and_history_survives_binding() {
    let rules = vec![
        Rule::propagate("p", [c("p", [v(0)])], c("q", [v(0)]).into()),
        Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
    ];
    let a = solve(
        rules,
        query(vec![c("p", [v(7)]), c("p", [v(7)]), c("bind", [v(7)])], &[]),
    );
    assert_eq!(
        a[0].residual,
        vec![
            c("p", [atom("a")]),
            c("p", [atom("a")]),
            c("q", [atom("a")]),
            c("q", [atom("a")])
        ]
    );
}
#[test]
fn ordered_propagation_tuples_are_distinct() {
    let rules = vec![Rule::propagate(
        "pairs",
        [c("p", [v(0)]), c("p", [v(1)])],
        c("pair", [v(0), v(1)]).into(),
    )];
    let a = solve(
        rules,
        query(vec![c("p", [atom("a")]), c("p", [atom("b")])], &[]),
    );
    assert_eq!(
        a[0].residual
            .iter()
            .filter(|c| c.name == "pair")
            .cloned()
            .collect::<Vec<_>>(),
        vec![
            c("pair", [atom("a"), atom("b")]),
            c("pair", [atom("b"), atom("a")])
        ]
    );
}
#[test]
fn guard_waits_without_binding_then_rechecks_after_aliasing() {
    let mut guarded = Rule::simplify("guarded", [c("p", [v(0), v(1)])], c("hit", []).into());
    guarded.guards = vec![Guard::Equal(v(0), v(1))];
    let a = solve(
        vec![guarded.clone()],
        query(vec![c("p", [v(4), v(5)])], &[4, 5]),
    );
    assert_eq!(values(&a[0]), vec![v(0), v(1)]);
    assert_eq!(a[0].residual, vec![c("p", [v(0), v(1)])]);
    let a = solve(
        vec![
            guarded,
            Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
        ],
        query(vec![c("p", [v(4), v(5)]), c("bind", [v(4), v(5)])], &[4, 5]),
    );
    assert_eq!(values(&a[0]), vec![v(0), v(0)]);
    assert_eq!(a[0].residual, vec![c("hit", [])]);
}
#[test]
fn fresh_body_variables_are_separate_per_application() {
    let a = solve(
        vec![Rule::simplify(
            "fresh",
            [c("p", [])],
            c("q", [v(99)]).into(),
        )],
        query(vec![c("p", []), c("p", [])], &[99]),
    );
    assert_eq!(values(&a[0]), vec![v(0)]);
    assert_eq!(a[0].residual, vec![c("q", [v(1)]), c("q", [v(2)])]);
}
#[test]
fn ordinary_competing_rules_commit_instead_of_creating_search() {
    let rules = vec![
        Rule::simplify("first", [c("p", [v(0)])], eq(v(0), atom("a"))),
        Rule::simplify("second", [c("p", [v(0)])], eq(v(0), atom("b"))),
    ];
    let a = solve(rules, query(vec![c("p", [v(7)])], &[7]));
    assert_eq!(a.len(), 1);
    assert_eq!(values(&a[0]), vec![atom("a")]);
}
#[test]
fn independently_executed_choices_produce_all_four_combinations() {
    let rule = Rule::simplify(
        "pick",
        [c("pick", [v(0)])],
        or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
    );
    let a = solve(
        vec![rule],
        query(vec![c("pick", [v(8)]), c("pick", [v(9)])], &[8, 9]),
    );
    let mut got = a.iter().map(values).collect::<Vec<_>>();
    got.sort();
    assert_eq!(
        got,
        vec![
            vec![atom("a"), atom("a")],
            vec![atom("a"), atom("b")],
            vec![atom("b"), atom("a")],
            vec![atom("b"), atom("b")]
        ]
    );
}
#[test]
fn occurs_failure_is_isolated_to_its_alternative() {
    let rule = Rule::simplify(
        "pick",
        [c("pick", [v(0)])],
        or(eq(v(0), t("s", [v(0)])), eq(v(0), atom("z"))),
    );
    let a = solve(vec![rule], query(vec![c("pick", [v(8)])], &[8]));
    assert_eq!(a.len(), 1);
    assert_eq!(values(&a[0]), vec![atom("z")]);
}
#[test]
fn propagation_history_is_inherited_at_a_split() {
    let rules = vec![
        Rule::propagate("p", [c("p", [])], c("q", []).into()),
        Rule::simplify(
            "pick",
            [c("pick", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
    ];
    let a = solve(rules, query(vec![c("p", []), c("pick", [v(8)])], &[8]));
    assert_eq!(a.len(), 2);
    for a in a {
        assert_eq!(a.residual, vec![c("p", []), c("q", [])]);
    }
}
#[test]
fn post_split_matching_and_history_are_branch_local() {
    let rules = vec![
        Rule::propagate("p", [c("p", [atom("a")])], c("q", []).into()),
        Rule::simplify(
            "pick",
            [c("pick", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
    ];
    let mut a = solve(rules, query(vec![c("p", [v(8)]), c("pick", [v(8)])], &[8]));
    a.sort();
    assert_eq!(a.len(), 2);
    assert_eq!(a[0].residual, vec![c("p", [atom("a")]), c("q", [])]);
    assert_eq!(a[1].residual, vec![c("p", [atom("b")])]);
}
#[test]
fn active_failure_prevents_publishing_even_a_ground_output() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            and([eq(v(0), atom("done")), c("bad", []).into()]),
        ),
        Rule::simplify("bad", [c("bad", [])], Goal::Fail),
    ];
    assert!(solve(rules, query(vec![c("start", [v(8)])], &[8])).is_empty());
}
#[test]
fn finite_answer_survives_an_infinite_sibling_and_search_can_resume() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(c("loop", []).into(), eq(v(0), atom("done"))),
        ),
        Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
    ];
    let mut s = Search::new(rules, query(vec![c("start", [v(8)])], &[8])).unwrap();
    let zero = s.advance(0);
    assert!(!zero.exhausted);
    assert!(zero.answers.is_empty());
    let batch = s.advance(100);
    assert!(!batch.exhausted);
    assert_eq!(batch.answers.len(), 1);
    assert_eq!(values(&batch.answers[0]), vec![atom("done")]);
    assert!(s.pending_alternatives() > 0);
    let batch = s.advance(100);
    assert!(!batch.exhausted);
    assert!(batch.answers.is_empty());
}
#[test]
fn alpha_equivalent_residual_answers_deduplicate_but_multiplicity_does_not() {
    let rule = Rule::simplify(
        "start",
        [c("start", [])],
        or(
            c("p", [v(0)]).into(),
            or(
                c("p", [v(1)]).into(),
                and([c("p", [v(2)]).into(), c("p", [v(2)]).into()]),
            ),
        ),
    );
    let mut s = Search::new(vec![rule], query(vec![c("start", [])], &[])).unwrap();
    let mut a = s.advance(200).answers;
    a.sort();
    assert_eq!(a.len(), 2);
    assert_eq!(a[0].residual, vec![c("p", [v(0)])]);
    assert_eq!(a[1].residual, vec![c("p", [v(0)]), c("p", [v(0)])]);
    assert_eq!(s.stats().completed_branches, 3);
    assert_eq!(s.stats().duplicate_answers, 1);
}
#[test]
fn answer_aliases_are_not_erased_by_deduplication() {
    let rule = Rule::simplify(
        "start",
        [c("start", [v(0), v(1)])],
        or(eq(v(0), v(1)), Goal::True),
    );
    let mut a = solve(vec![rule], query(vec![c("start", [v(8), v(9)])], &[8, 9]));
    a.sort();
    assert_eq!(a.len(), 2);
    assert_eq!(values(&a[0]), vec![v(0), v(0)]);
    assert_eq!(values(&a[1]), vec![v(0), v(1)]);
}
#[test]
fn empty_query_returns_one_answer_with_unused_output_variables() {
    let a = solve(vec![], query(vec![], &[42]));
    assert_eq!(a.len(), 1);
    assert_eq!(values(&a[0]), vec![v(0)]);
}
#[test]
fn malformed_rule_sets_are_rejected() {
    assert!(
        Search::new(
            vec![Rule::simplify("empty", [], Goal::True)],
            query(vec![], &[])
        )
        .is_err()
    );
    let r = Rule::simplify("duplicate", [c("p", [])], Goal::True);
    assert!(Search::new(vec![r.clone(), r], query(vec![], &[])).is_err());
}

#[test]
fn residual_alpha_dedup_handles_permutations_and_preserves_cross_constraint_aliases() {
    let rule = Rule::simplify(
        "start",
        [c("start", [])],
        or(
            and([c("node", [v(1)]).into(), c("edge", [v(0), v(1)]).into()]),
            or(
                and([c("edge", [v(4), v(5)]).into(), c("node", [v(5)]).into()]),
                and([c("edge", [v(6), v(7)]).into(), c("node", [v(6)]).into()]),
            ),
        ),
    );
    let mut s = Search::new(vec![rule], query(vec![c("start", [])], &[99])).unwrap();
    let mut a = s.advance(200).answers;
    a.sort();
    assert_eq!(a.len(), 2);
    assert_eq!(values(&a[0]), vec![v(0)]);
    assert_eq!(
        a[0].residual,
        vec![c("edge", [v(1), v(2)]), c("node", [v(1)])]
    );
    assert_eq!(
        a[1].residual,
        vec![c("edge", [v(1), v(2)]), c("node", [v(2)])]
    );
    assert_eq!(s.stats().duplicate_answers, 1);
}
