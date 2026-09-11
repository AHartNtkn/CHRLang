use chr_graph::{Mode, Search, eligible};
#[test]
fn accepted_registry_cases_preserve_full_answers() {
    let mut accepted = 0;
    for case in chr_cases::registry() {
        if eligible(&case.rules).is_err() {
            continue;
        }
        accepted += 1;
        for mode in [Mode::General, Mode::Local, Mode::Cached] {
            let mut s = Search::new(case.rules.clone(), case.query.clone(), mode).unwrap();
            let mut actual = vec![];
            let mut exhausted = false;
            for _ in 0..case.budget {
                let b = s.advance(1);
                actual.extend(b.answers);
                exhausted = b.exhausted;
                if exhausted || case.answer_limit.is_some_and(|n| actual.len() >= n) {
                    break;
                }
            }
            assert_eq!(actual.len(), case.expected.len(), "{} {mode:?}", case.id);
            for expected in &case.expected {
                assert!(
                    actual.iter().any(|a| chr_observe::equivalent(
                        a,
                        expected,
                        &mut Default::default()
                    )),
                    "{} {mode:?}: {actual:?}",
                    case.id
                );
            }
            assert_eq!(exhausted, case.exhausted, "{} {mode:?}", case.id);
            assert_eq!(
                s.stats().completed,
                case.raw_answers,
                "{} {mode:?}",
                case.id
            );
        }
    }
    assert!(
        accepted > 24,
        "must include applications beyond the carry grid"
    );
}
#[test]
fn caching_reuses_descendants_but_local_dispatch_alone_does_not() {
    for mode in [Mode::General, Mode::Local, Mode::Cached] {
        let c = chr_cases::carry_case(3, 4, 16);
        let mut s = Search::new(c.rules, c.query, mode).unwrap();
        let b = s.advance(1000);
        assert!(b.exhausted);
        assert_eq!(b.answers.len(), 8);
        assert_eq!(s.stats().applications, 39);
        assert_eq!(
            s.stats().expansions,
            if matches!(mode, Mode::Cached) { 7 } else { 39 }
        );
    }
}

#[test]
fn event_locals_are_fresh_for_distinct_occurrences_and_guard_locals() {
    use chr_syntax::{Guard, Rule, and, c, eq, v};
    let mut rule = Rule::simplify(
        "make",
        [c("make", [v(0)])],
        and([eq(v(0), v(1)), c("pair", [v(1), v(2)]).into()]),
    );
    rule.guards = vec![Guard::Equal(v(1), v(1))];
    for mode in [Mode::General, Mode::Local, Mode::Cached] {
        let mut s = Search::new(
            vec![rule.clone()],
            chr_cases::query(vec![c("make", [v(0)]), c("make", [v(1)])], &[0, 1]),
            mode,
        )
        .unwrap();
        let b = s.advance(100);
        assert!(b.exhausted);
        let expected = chr_cases::answer(
            vec![v(0), v(1)],
            vec![c("pair", [v(0), v(2)]), c("pair", [v(1), v(3)])],
        );
        assert!(
            chr_observe::equivalent(&b.answers[0], &expected, &mut Default::default()),
            "{:?}",
            b.answers
        );
        assert_eq!(s.stats().expansions, 2);
        assert_eq!(s.stats().cache_hits, 0);
    }
}
#[test]
fn contextual_demand_and_active_failure_are_not_cached_unconditionally() {
    use chr_syntax::{Goal, Rule, atom, c, eq, or, v};
    let rules = vec![
        Rule::simplify(
            "pick",
            [c("pick", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify("a", [c("check", [atom("a")])], Goal::True),
        Rule::simplify("b", [c("check", [atom("b")])], Goal::Fail),
    ];
    for mode in [Mode::General, Mode::Local, Mode::Cached] {
        let mut s = Search::new(
            rules.clone(),
            chr_cases::query(vec![c("check", [v(0)]), c("pick", [v(0)])], &[]),
            mode,
        )
        .unwrap();
        let b = s.advance(100);
        assert!(b.exhausted);
        assert_eq!(b.answers.len(), 1);
        assert!(b.answers[0].residual.is_empty());
        assert_eq!(s.stats().failed, 1);
    }
}
#[test]
fn linking_rules_requires_rechecking_region_eligibility() {
    use chr_syntax::{Goal, Rule, c, v};
    let mut rules = vec![Rule::simplify("p", [c("p", [v(0)])], Goal::True)];
    assert!(eligible(&rules).is_ok());
    rules.push(Rule::simplify(
        "linked",
        [c("p", [v(0)]), c("q", [v(0)])],
        Goal::True,
    ));
    assert!(eligible(&rules).unwrap_err().contains("single-head"));
    rules.pop();
    rules.push(Rule::simplify("overlap", [c("p", [v(1)])], Goal::True));
    assert!(eligible(&rules).unwrap_err().contains("overlap"));
}

#[test]
fn alias_binding_rechecks_a_previously_unknown_constructor() {
    use chr_syntax::{Goal, Rule, atom, c, eq, v};
    let rules = vec![
        Rule::simplify("check", [c("check", [atom("a")])], Goal::True),
        Rule::simplify("alias", [c("alias", [v(0), v(1)])], eq(v(0), v(1))),
        Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
    ];
    for mode in [Mode::General, Mode::Local, Mode::Cached] {
        let mut s = Search::new(
            rules.clone(),
            chr_cases::query(
                vec![
                    c("check", [v(0)]),
                    c("alias", [v(0), v(1)]),
                    c("bind", [v(1)]),
                ],
                &[0, 1],
            ),
            mode,
        )
        .unwrap();
        let b = s.advance(100);
        assert!(b.exhausted);
        assert_eq!(b.answers.len(), 1);
        let expected = chr_cases::answer(vec![atom("a"), atom("a")], vec![]);
        assert!(chr_observe::equivalent(
            &b.answers[0],
            &expected,
            &mut Default::default()
        ));
    }
}

#[test]
fn cached_expansion_can_be_committed_by_a_delayed_context() {
    use chr_syntax::{Goal, Rule, atom, c, or, t, v};
    fn unary(n: usize) -> chr_syntax::Term {
        (0..n).fold(atom("z"), |x, _| t("s", [x]))
    }
    let rules = vec![
        Rule::simplify(
            "choose",
            [c("choose", [])],
            or(Goal::True, c("delay", [unary(3)]).into()),
        ),
        Rule::simplify(
            "delay",
            [c("delay", [t("s", [v(0)])])],
            c("delay", [v(0)]).into(),
        ),
        Rule::simplify(
            "carry",
            [c("carry", [t("s", [v(0)])])],
            c("carry", [v(0)]).into(),
        ),
    ];
    let mut s = Search::new(
        rules,
        chr_cases::query(vec![c("carry", [unary(4)]), c("choose", [])], &[]),
        Mode::Cached,
    )
    .unwrap();
    let b = s.advance(200);
    assert!(b.exhausted);
    assert_eq!(b.answers.len(), 2);
    assert_eq!(s.stats().completed, 2);
    assert_eq!(s.stats().applications, 12);
    assert_eq!(s.stats().expansions, 8);
    assert_eq!(s.stats().cache_hits, 4);
    let expected = [
        chr_cases::answer(vec![], vec![c("carry", [atom("z")])]),
        chr_cases::answer(
            vec![],
            vec![c("carry", [atom("z")]), c("delay", [atom("z")])],
        ),
    ];
    for e in expected {
        assert!(
            b.answers
                .iter()
                .any(|a| chr_observe::equivalent(a, &e, &mut Default::default()))
        );
    }
}
