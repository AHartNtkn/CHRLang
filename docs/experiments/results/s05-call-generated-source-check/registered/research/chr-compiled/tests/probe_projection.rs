use chr_compiled::{Access, Policy, PreparedRuleset};
use chr_syntax::{Query, Rule, atom, c, v};
#[test]
fn repeated_projection_preserves_occurrences_and_records_each_step() {
    for n in [16, 64, 256] {
        for duplicates in [false, true] {
            let rules = vec![Rule {
                name: "join".into(),
                kept: vec![
                    c("request", [v(2)]),
                    c("left", [v(0), v(1)]),
                    c("right", [v(1), v(2)]),
                ],
                removed: vec![c("token", [])],
                guards: vec![],
                body: c("receipt", [v(0), v(1), v(2)]).into(),
            }];
            let prepared = PreparedRuleset::new(rules, None).unwrap();
            for query in 0..2 {
                let b = atom(&format!("b{query}"));
                let mut facts = vec![c("request", [b.clone()])];
                for i in 0..n {
                    facts.push(c("left", [atom(&format!("a{i}")), atom("x")]));
                }
                let m = if duplicates { n - 1 } else { 1 };
                for _ in 0..m {
                    facts.push(c("right", [atom("x"), b.clone()]));
                }
                let mut expected = facts.clone();
                expected.push(c("receipt", [atom("a0"), atom("x"), b]));
                expected.sort();
                let token = facts.len() as u64;
                facts.push(c("token", []));
                let mut engine = prepared
                    .start(
                        Query {
                            constraints: facts,
                            outputs: vec![],
                        },
                        Policy::Global,
                        Access::Indexed,
                    )
                    .unwrap();
                engine.enable_trace();
                let mut previous = [0; 5];
                let mut step = 0;
                let mut max_bucket = 0;
                while !engine.status().exhausted {
                    assert!(step < 2_000_000);
                    engine.step();
                    step += 1;
                    let s = engine.stats();
                    let now = [
                        s.probe_visits,
                        s.index_bucket_entries,
                        s.pool_visits,
                        s.candidate_visits,
                        s.empty_head_checks,
                    ];
                    println!(
                        "PROJECT_STEP,n={n},duplicates={duplicates},query={query},step={step},probe={},bucket={},pool={},candidate={},empty={}",
                        now[0] - previous[0],
                        now[1] - previous[1],
                        now[2] - previous[2],
                        now[3] - previous[3],
                        now[4] - previous[4]
                    );
                    max_bucket = max_bucket.max(now[1] - previous[1]);
                    previous = now;
                }
                if chr_compiled::COLLECT_METRICS && cfg!(feature = "selective-probe") {
                    assert!(
                        max_bucket <= 2 * n as u64,
                        "max bucket entries {max_bucket} at n={n}"
                    );
                }
                let mut answer = engine.observe().unwrap();
                answer.residual.sort();
                assert_eq!(answer.residual, expected);
                assert_eq!(engine.trace(), &[(0, vec![0, 1, (n + 1) as u64, token])]);
                println!(
                    "PROJECT_DONE,n={n},duplicates={duplicates},query={query},steps={step},metrics={}",
                    chr_compiled::COLLECT_METRICS
                );
            }
        }
    }
}

#[test]
fn empty_head_checks_allow_source_reactivation_and_distinct_nullary_heads() {
    for policy in [Policy::Global, Policy::Active] {
        let p = PreparedRuleset::new(
            vec![
                Rule::simplify("consume", [c("token", [])], c("receipt", []).into()),
                Rule::simplify("issue", [c("seed", [])], c("token", []).into()),
            ],
            None,
        )
        .unwrap();
        let mut e = p
            .start(
                Query {
                    constraints: vec![c("seed", [])],
                    outputs: vec![],
                },
                policy,
                Access::Indexed,
            )
            .unwrap();
        e.enable_trace();
        assert!(e.advance(1000).exhausted);
        assert_eq!(e.observe().unwrap().residual, vec![c("receipt", [])]);
        assert_eq!(e.trace(), &[(1, vec![0]), (0, vec![1])]);
        for n in [1, 2, 3] {
            let p = PreparedRuleset::new(
                vec![Rule::simplify(
                    "pair",
                    [c("token", []), c("token", [])],
                    c("paired", []).into(),
                )],
                None,
            )
            .unwrap();
            let mut e = p
                .start(
                    Query {
                        constraints: vec![c("token", []); n],
                        outputs: vec![],
                    },
                    policy,
                    Access::Indexed,
                )
                .unwrap();
            assert!(e.advance(1000).exhausted);
            let mut actual = e.observe().unwrap().residual;
            actual.sort();
            let mut expected = if n == 1 {
                vec![c("token", [])]
            } else {
                vec![c("paired", [])]
            };
            if n == 3 {
                expected.push(c("token", []));
            }
            expected.sort();
            assert_eq!(actual, expected);
        }
    }
}
