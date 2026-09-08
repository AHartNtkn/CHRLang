#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../experiments/reusable_regions.rs"]
mod regions;
#[allow(dead_code)]
#[path = "../experiments/reusable_workers.rs"]
mod workers;
use chr_syntax::{Answer, Goal, Query, Rule, atom, c, eq, or, v};
use regions::{Mode, Runtime};
fn modes() -> Vec<Mode> {
    vec![
        Mode::Inline,
        Mode::Threads(1),
        Mode::Threads(2),
        Mode::Threads(4),
        #[cfg(feature = "worker-lowering")]
        Mode::Contracted,
    ]
}
fn rules() -> Vec<Rule> {
    ["p", "q"]
        .into_iter()
        .map(|name| {
            Rule::simplify(
                name,
                [c(name, [v(0)])],
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
            )
        })
        .collect()
}
fn check(runtime: &mut Runtime, rules: &[Rule], q: Query, factors: usize) {
    check_answers(runtime, rules, q, Some(factors));
}
fn check_answers(runtime: &mut Runtime, rules: &[Rule], q: Query, factors: Option<usize>) {
    let raw = oracle::run(rules, &q, 100_000);
    let mut expected = Vec::<Answer>::new();
    for a in &raw {
        if !expected
            .iter()
            .any(|b| chr_observe::equivalent(a, b, &mut Default::default()))
        {
            expected.push(a.clone());
        }
    }
    let mut session = runtime.start(q).unwrap();
    if let Some(factors) = factors {
        assert_eq!(session.factor_count(), factors);
    }
    let _ = &session.certificate;
    assert!(session.advance(0).unwrap().answers.is_empty());
    let mut actual = Vec::new();
    for _ in 0..100_000 {
        let batch = session.advance(1).unwrap();
        actual.extend(batch.answers);
        if batch.exhausted {
            break;
        }
    }
    assert!(session.exhausted());
    assert_eq!(session.raw_count(), Some(raw.len() as u128));
    oracle::same_raw(actual, expected);
    session.close().unwrap();
}
#[test]
fn certified_products_reuse_runtime_with_empty_shared_and_independent_outputs() {
    let rs = rules();
    for mode in modes() {
        for quantum in [1, 7] {
            for window in [1, 4] {
                let mut r = Runtime::new(rs.clone(), mode, quantum, window).unwrap();
                check(&mut r, &rs, chr_cases::query(vec![], &[]), 1);
                check(
                    &mut r,
                    &rs,
                    chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]),
                    2,
                );
                check(
                    &mut r,
                    &rs,
                    chr_cases::query(vec![c("p", [v(0)]), c("q", [v(0)])], &[0]),
                    1,
                );
                check(
                    &mut r,
                    &rs,
                    chr_cases::query(vec![c("left", [v(0)]), c("right", [v(1)])], &[0, 1]),
                    2,
                );
                check(
                    &mut r,
                    &rs,
                    chr_cases::query(vec![c("p", [atom("c")]), c("q", [v(1)])], &[1]),
                    2,
                );
                check(
                    &mut r,
                    &rs,
                    chr_cases::query(vec![c("p", [v(0)])], &[0, 9]),
                    2,
                );
                r.shutdown().unwrap();
                assert!(r.start(chr_cases::query(vec![], &[])).is_err());
            }
        }
    }
}
#[test]
fn product_raw_multiplicity_and_cancellation_do_not_leak_across_queries() {
    let rs = ["p", "q"]
        .into_iter()
        .map(|n| {
            Rule::simplify(
                n,
                [c(n, [v(0)])],
                or(eq(v(0), atom("a")), eq(v(0), atom("a"))),
            )
        })
        .collect::<Vec<_>>();
    for mode in modes() {
        let mut r = Runtime::new(rs.clone(), mode, 1, 4).unwrap();
        let q = chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]);
        for prefix in [0, 1, 5, 20] {
            let mut session = r.start(q.clone()).unwrap();
            session.advance(prefix).unwrap();
            drop(session);
            check(&mut r, &rs, q.clone(), 2);
        }
    }
}

#[test]
fn finite_source_registry_agrees_after_partition_and_product() {
    for case in chr_cases::registry().into_iter().filter(|c| c.exhausted) {
        for mode in modes() {
            let mut runtime = Runtime::new(case.rules.clone(), mode, 7, 4).unwrap();
            check_answers(&mut runtime, &case.rules, case.query.clone(), None);
            runtime.shutdown().unwrap();
        }
    }
}

#[test]
fn finite_products_publish_beside_continuing_source_and_runtime_reuses() {
    let rs = vec![
        Rule::simplify(
            "p",
            [c("p", [v(0)])],
            or(Goal::Constraint(c("loop", [v(0)])), eq(v(0), atom("a"))),
        ),
        Rule::simplify(
            "loop",
            [c("loop", [v(0)])],
            Goal::Constraint(c("loop", [v(0)])),
        ),
        Rule::simplify(
            "q",
            [c("q", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
    ];
    for mode in modes() {
        for quantum in [1, 7] {
            for window in [1, 4] {
                let mut runtime = Runtime::new(rs.clone(), mode, quantum, window).unwrap();
                let mut session = runtime
                    .start(chr_cases::query(
                        vec![c("p", [v(0)]), c("q", [v(1)])],
                        &[0, 1],
                    ))
                    .unwrap();
                assert_eq!(session.factor_count(), 2);
                let mut actual = Vec::new();
                for _ in 0..256 {
                    let batch = session.advance(1).unwrap();
                    assert!(!batch.exhausted);
                    actual.extend(batch.answers);
                }
                assert_eq!(session.raw_count(), None);
                oracle::same_raw(
                    actual,
                    vec![
                        chr_cases::answer(vec![atom("a"), atom("a")], vec![]),
                        chr_cases::answer(vec![atom("a"), atom("b")], vec![]),
                    ],
                );
                session.close().unwrap();
                assert!(session.advance(1).is_err());
                drop(session);
                check(
                    &mut runtime,
                    &rs,
                    chr_cases::query(vec![c("q", [v(0)])], &[0]),
                    1,
                );
                runtime.shutdown().unwrap();
            }
        }
    }
}

#[cfg(feature = "worker-lowering")]
#[test]
fn factored_contraction_matches_full_worker_source_products() {
    #[path = "../experiments/worker_cases.rs"]
    mod workload;
    for mode in [Mode::Inline, Mode::Threads(4), Mode::Contracted] {
        for quantum in [1, 128] {
            let mut runtime = Runtime::new(workload::source(), mode, quantum, 4).unwrap();
            for count in [1, 2, 4] {
                let expected = workload::expected(count);
                for depth in [0, 8, 64, 256] {
                    for skew in [false, true] {
                        let mut session =
                            runtime.start(workload::query(count, depth, skew)).unwrap();
                        assert_eq!(session.factor_count(), count);
                        let mut actual = Vec::new();
                        for _ in 0..100_000 {
                            let batch = session.advance(1).unwrap();
                            actual.extend(batch.answers);
                            if batch.exhausted {
                                break;
                            }
                        }
                        assert!(session.exhausted());
                        assert_eq!(session.raw_count(), Some(expected.len() as u128));
                        actual.sort_unstable_by(|a, b| a.outputs.cmp(&b.outputs));
                        assert_eq!(actual, expected);
                        session.close().unwrap();
                    }
                }
            }
            runtime.shutdown().unwrap();
        }
    }
}
