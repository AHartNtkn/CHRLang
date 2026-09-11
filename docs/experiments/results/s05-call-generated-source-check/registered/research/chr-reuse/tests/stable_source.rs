use chr_reuse::{stable::Policy, stable_search::Search};
use chr_syntax::{Query, Rule, Var, and, atom, c, eq, or, t, v};
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;

#[test]
fn successful_hits_enable_consumption_and_preserve_duplicate_alternatives() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("a"))),
        ),
        Rule::simplify(
            "consume",
            [c("item", [atom("a")])],
            c("seen", [atom("a")]).into(),
        ),
    ];
    let q = Query {
        constraints: vec![c("item", [v(0)]), c("start", [v(0)])],
        outputs: vec![("x".into(), Var(0))],
    };
    let expected = oracle::run(&rules, &q, 1000);
    assert_eq!(expected.len(), 2);
    for policy in [Policy::Direct, Policy::Exact, Policy::Dependencies] {
        let mut s = Search::new(rules.clone(), q.clone(), policy, 16).unwrap();
        let batch = s.advance(1000);
        assert!(batch.exhausted);
        oracle::same_raw(batch.answers, expected.clone());
        if cfg!(feature = "metrics") && policy != Policy::Direct {
            assert!(s.hits() > 0);
        }
    }
}
#[test]
fn common_failure_does_not_fail_a_finite_sibling() {
    let clash = eq(t("pair", [v(0), v(0)]), t("pair", [atom("a"), atom("b")]));
    let rules = vec![Rule::simplify(
        "start",
        [c("start", [v(0)])],
        or(or(clash.clone(), clash), c("ok", [v(0)]).into()),
    )];
    let q = Query {
        constraints: vec![c("start", [v(0)])],
        outputs: vec![("x".into(), Var(0))],
    };
    let expected = oracle::run(&rules, &q, 1000);
    assert_eq!(expected.len(), 1);
    for policy in [Policy::Exact, Policy::Dependencies] {
        let mut s = Search::new(rules.clone(), q.clone(), policy, 16).unwrap();
        let b = s.advance(1000);
        assert!(b.exhausted);
        oracle::same_raw(b.answers, expected.clone());
        if cfg!(feature = "metrics") {
            assert!(s.hits() > 0);
        }
    }
}
#[test]
fn changing_binding_invalidates_cached_success() {
    let rules = vec![Rule::simplify(
        "start",
        [c("start", [v(0), v(1)])],
        or(
            eq(v(0), t("f", [v(1)])),
            and(vec![eq(v(0), v(1)), eq(v(0), t("f", [v(1)]))]),
        ),
    )];
    let q = Query {
        constraints: vec![c("start", [v(0), v(1)])],
        outputs: vec![("x".into(), Var(0)), ("y".into(), Var(1))],
    };
    let expected = oracle::run(&rules, &q, 1000);
    assert_eq!(expected.len(), 1);
    for policy in [Policy::Direct, Policy::Exact, Policy::Dependencies] {
        let mut s = Search::new(rules.clone(), q.clone(), policy, 16).unwrap();
        let b = s.advance(1000);
        assert!(b.exhausted);
        oracle::same_raw(b.answers, expected.clone());
    }
}

#[test]
#[should_panic(expected = "cursor belongs to another machine")]
fn foreign_cursor_is_rejected_before_equation_service() {
    use chr_persistent::continuations::Machine;
    let q = chr_cases::query(vec![c("a", [])], &[]);
    let (mut a, _) = Machine::new(vec![], q.clone()).unwrap();
    let (_, b) = Machine::new(vec![], q).unwrap();
    a.step_with_equation(b, |_| panic!("foreign equation reached cache"));
}

#[test]
fn registered_sources_preserve_full_observations_raw_multiplicity_and_progress() {
    let cases = chr_cases::registry();
    println!(
        "registered source cases: {}; configurations: {}",
        cases.len(),
        cases.len() * 9
    );
    for case in cases {
        for policy in [Policy::Direct, Policy::Exact, Policy::Dependencies] {
            for capacity in [0, 1, 32] {
                let mut s =
                    Search::new(case.rules.clone(), case.query.clone(), policy, capacity).unwrap();
                let mut raw = vec![];
                let mut unique = chr_observe::AnswerSet::default();
                let mut count = 0;
                let mut exhausted = false;
                for _ in 0..case.budget {
                    let b = s.advance(1);
                    exhausted = b.exhausted;
                    for a in b.answers {
                        if unique.insert(a.clone()) {
                            count += 1
                        };
                        raw.push(a);
                    }
                    if exhausted || case.answer_limit.is_some_and(|n| count >= n) {
                        break;
                    }
                }
                assert_eq!(exhausted, case.exhausted, "{} {policy:?}", case.id);
                assert_eq!(raw.len() as u64, case.raw_answers, "{} {policy:?}", case.id);
                assert_eq!(count, case.expected.len(), "{} {policy:?}", case.id);
                for a in &raw {
                    assert!(
                        case.expected.iter().any(|b| chr_observe::equivalent(
                            a,
                            b,
                            &mut Default::default()
                        )),
                        "{}",
                        case.id
                    );
                }
            }
        }
    }
}

#[test]
fn one_cache_cannot_accept_another_machines_constructor_numbers() {
    use chr_persistent::continuations::{Machine, Step};
    use chr_reuse::stable::EquationCache;
    let mut cache = EquationCache::new(8);
    for (index, name) in ["a", "b"].into_iter().enumerate() {
        let rules = vec![Rule::simplify(
            "start",
            [c("start", [])],
            eq(atom(name), atom("a")),
        )];
        let (mut machine, mut cursor) =
            Machine::new(rules, chr_cases::query(vec![c("start", [])], &[])).unwrap();
        while !machine.has_pending_equation(&cursor) {
            cursor = match machine.step(cursor) {
                Step::Continue(c) => c,
                _ => panic!("expected pending source equation"),
            };
        }
        machine.step_with_equation(cursor, |equation| {
            let result = cache.solve(equation, Policy::Exact);
            if index == 0 {
                assert!(result.unwrap().success);
            } else {
                assert!(result.is_err());
            }
            true
        });
    }
}

#[test]
fn prepared_rules_are_reused_across_changed_queries_without_cache_identity_leakage() {
    use chr_reuse::stable_search::Prepared;
    let rules = vec![Rule::simplify(
        "bind",
        [c("start", [v(0), v(1)])],
        or(eq(v(0), v(1)), eq(v(0), v(1))),
    )];
    for policy in [Policy::Exact, Policy::Dependencies] {
        let p = Prepared::new(rules.clone(), policy, 16).unwrap();
        for name in ["a", "b", "a"] {
            let q = chr_cases::query(vec![c("start", [v(0), atom(name)])], &[0]);
            let expected = oracle::run(&rules, &q, 1000);
            let mut s = p.start(q).unwrap();
            let b = s.advance(1000);
            assert!(b.exhausted);
            oracle::same_raw(b.answers, expected);
        }
    }
}
