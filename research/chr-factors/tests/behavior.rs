use chr_factors::{Mode, Search};
#[test]
fn finite_registry_observations_survive_factorization() {
    for case in chr_cases::registry() {
        if !case.exhausted {
            continue;
        }
        for mode in [Mode::Scalar, Mode::Factored] {
            let mut s = Search::new(case.rules.clone(), case.query.clone(), mode).unwrap();
            let b = s.advance(case.budget * 8);
            assert!(b.exhausted, "{} {mode:?}", case.id);
            assert_eq!(b.answers.len(), case.expected.len(), "{} {mode:?}", case.id);
            for e in &case.expected {
                assert!(
                    b.answers.iter().any(|a| chr_observe::equivalent(
                        a,
                        e,
                        &mut Default::default()
                    )),
                    "{} {mode:?}",
                    case.id
                );
            }
            assert_eq!(s.raw_count(), Some(case.raw_answers as u128));
        }
    }
}
#[test]
fn distinct_closed_predicates_form_a_product_without_aliasing() {
    use chr_syntax::{Rule, atom, c, eq, or, v};
    let rules = vec![
        Rule::simplify(
            "p",
            [c("p", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify(
            "q",
            [c("q", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
    ];
    let mut s = Search::new(
        rules,
        chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]),
        Mode::Factored,
    )
    .unwrap();
    assert_eq!(s.factor_count(), 2);
    let b = s.advance(100);
    assert!(b.exhausted);
    assert_eq!(b.answers.len(), 4);
    assert_eq!(s.raw_count(), Some(4));
    assert_eq!(s.source_applications(), 2);
}

#[test]
fn coupling_and_future_resource_rules_prevent_partitioning() {
    use chr_syntax::{Goal, Rule, atom, c, eq, or, v};
    let rules = vec![
        Rule::simplify(
            "p",
            [c("p", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify(
            "q",
            [c("q", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
    ];
    let mut shared = Search::new(
        rules.clone(),
        chr_cases::query(vec![c("p", [v(0)]), c("q", [v(0)])], &[0]),
        Mode::Factored,
    )
    .unwrap();
    assert_eq!(shared.factor_count(), 1);
    assert_eq!(shared.advance(100).answers.len(), 2);
    let mut linked = rules;
    linked.push(Rule::simplify(
        "joint",
        [c("p", [v(0)]), c("q", [v(1)])],
        Goal::True,
    ));
    let s = Search::new(
        linked,
        chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]),
        Mode::Factored,
    )
    .unwrap();
    assert_eq!(s.factor_count(), 1);
}
#[test]
fn regional_existentials_are_renamed_apart_and_free_outputs_keep_aliases() {
    use chr_syntax::{c, v};
    let mut s = Search::new(
        vec![],
        chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[3, 3]),
        Mode::Factored,
    )
    .unwrap();
    assert_eq!(s.factor_count(), 3);
    let b = s.advance(100);
    assert!(b.exhausted);
    assert_eq!(b.answers.len(), 1);
    let expected = chr_cases::answer(vec![v(2), v(2)], vec![c("p", [v(0)]), c("q", [v(1)])]);
    assert!(chr_observe::equivalent(
        &b.answers[0],
        &expected,
        &mut Default::default()
    ));
}
#[test]
fn an_exhausted_empty_factor_refutes_without_waiting_for_a_loop() {
    use chr_syntax::{Goal, Rule, c, v};
    let rules = vec![
        Rule::simplify("loop", [c("loop", [v(0)])], c("loop", [v(0)]).into()),
        Rule::simplify("reject", [c("bad", [])], Goal::Fail),
    ];
    let query = chr_cases::query(vec![c("loop", [v(0)]), c("bad", [])], &[]);
    let mut factored = Search::new(rules.clone(), query.clone(), Mode::Factored).unwrap();
    let b = factored.advance(100);
    assert!(b.exhausted);
    assert!(b.answers.is_empty());
    assert_eq!(factored.raw_count(), Some(0));
    let mut scalar = Search::new(rules, query, Mode::Scalar).unwrap();
    assert!(!scalar.advance(100).exhausted);
}
#[test]
fn product_service_does_not_wait_for_an_infinite_factor_to_finish() {
    use chr_syntax::{Rule, and, atom, c, eq, or, t, v};
    let rules = vec![
        Rule::simplify(
            "nums",
            [c("nums", [v(0)])],
            or(
                eq(v(0), atom("z")),
                and([eq(v(0), t("s", [v(1)])), c("nums", [v(1)]).into()]),
            ),
        ),
        Rule::simplify(
            "pick",
            [c("pick", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
    ];
    let mut s = Search::new(
        rules,
        chr_cases::query(vec![c("nums", [v(0)]), c("pick", [v(1)])], &[0, 1]),
        Mode::Factored,
    )
    .unwrap();
    let b = s.advance(500);
    assert!(!b.exhausted);
    assert_eq!(s.raw_count(), None);
    for n in 0..4 {
        for letter in ["a", "b"] {
            let n = (0..n).fold(atom("z"), |n, _| t("s", [n]));
            let expected = chr_cases::answer(vec![n, atom(letter)], vec![]);
            assert!(b.answers.iter().any(|a| chr_observe::equivalent(
                a,
                &expected,
                &mut Default::default()
            )));
        }
    }
}

#[test]
fn exact_regional_dedup_preserves_finite_raw_multiplicity_accounting() {
    use chr_syntax::{Rule, atom, c, eq, or, v};
    let rules = vec![
        Rule::simplify(
            "p",
            [c("p", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("a"))),
        ),
        Rule::simplify(
            "q",
            [c("q", [v(0)])],
            or(eq(v(0), atom("b")), eq(v(0), atom("b"))),
        ),
    ];
    for mode in [Mode::Scalar, Mode::Factored] {
        let mut s = Search::new(
            rules.clone(),
            chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]),
            mode,
        )
        .unwrap();
        let b = s.advance(100);
        assert!(b.exhausted);
        assert_eq!(b.answers.len(), 1);
        assert_eq!(s.raw_count(), Some(4));
    }
}
