//! Deterministic source and work witness, independently of execution strategy.
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
use chr_syntax::{Answer, Query, Rule, atom, c, eq, t, v};
fn unary(n: usize) -> chr_syntax::Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
#[derive(Debug)]
struct Counts {
    applications: u64,
    dependencies: u64,
    keys: u64,
    requests: u64,
    nodes: u64,
    inserts: u64,
    removes: u64,
}
fn run(
    rules: Vec<Rule>,
    query: Query,
    policy: Policy,
    access: Access,
    specialized: bool,
) -> (Answer, Vec<usize>, Counts) {
    let prepared = PreparedRuleset::new(rules, None).unwrap();
    let prepared = if specialized {
        prepared.specialize_inferred()
    } else {
        prepared
    };
    let mut engine = prepared.start_search(query, policy, access).unwrap();
    engine.enable_trace();
    let mut result = None;
    for _ in 0..1_000_000 {
        match engine.tick() {
            SearchEvent::Complete(mut b) => {
                assert!(result.is_none(), "unexpected additional raw answer");
                let answer = b.engine.observe().unwrap();
                let s = b.engine.stats();
                let counts = Counts {
                    applications: s.applications,
                    dependencies: s.dependency_visits,
                    keys: s.key_visits,
                    requests: s.key_normalization_requests,
                    nodes: s.key_normalization_allocations,
                    inserts: s.index_inserts,
                    removes: s.index_removes,
                };
                result = Some((
                    answer,
                    b.engine.trace().iter().map(|x| x.0).collect(),
                    counts,
                ));
            }
            SearchEvent::Exhausted => return result.expect("missing answer"),
            SearchEvent::Progress => {}
            _ => panic!("unexpected failure or source alternative"),
        }
    }
    panic!("finite maintenance gate cutoff")
}
#[test]
fn immutable_suffix_work_is_distinct_from_source_applications() {
    for n in [8, 16, 32] {
        for specialized in [false, true] {
            for access in [Access::Scan, Access::Indexed] {
                let rules = vec![Rule::simplify(
                    "carry",
                    [c("carry", [v(0), t("s", [v(1)])])],
                    c("carry", [v(0), v(1)]).into(),
                )];
                let query = Query {
                    constraints: vec![c("carry", [atom("a"), unary(n)])],
                    outputs: vec![],
                };
                let (answer, trace, counts) =
                    run(rules, query, Policy::Global, access, specialized);
                assert_eq!(
                    answer,
                    Answer {
                        outputs: vec![],
                        residual: vec![c("carry", [atom("a"), atom("z")])]
                    }
                );
                assert_eq!(trace, vec![0; n]);
                if chr_compiled::COLLECT_METRICS {
                    let visits = (2 * (n + 1)) as u64;
                    assert_eq!(counts.applications, n as u64);
                    assert_eq!(
                        counts.dependencies,
                        if access == Access::Indexed { visits } else { 0 }
                    );
                    if access == Access::Indexed {
                        assert_eq!(counts.keys, visits);
                        assert_eq!(counts.requests, 0);
                        assert_eq!(counts.nodes, 0);
                        assert_eq!(counts.inserts, 2 * (n + 1) as u64);
                        assert_eq!(counts.removes, 2 * n as u64);
                    } else {
                        assert_eq!(counts.keys, 0);
                    }
                    println!("depth={n} specialized={specialized} access={access:?} {counts:?}");
                }
            }
        }
    }
}
#[test]
fn open_parent_dependencies_survive_alias_then_binding() {
    for policy in [Policy::Global, Policy::Active] {
        for access in [Access::Scan, Access::Indexed] {
            for specialized in [false, true] {
                let rules = vec![
                    Rule::simplify(
                        "match",
                        [c("p", [t("f", [atom("a")]), unary(8), v(0)])],
                        c("done", [v(0), v(0)]).into(),
                    ),
                    Rule::simplify("alias", [c("alias", [v(0), v(1)])], eq(v(0), v(1))),
                    Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
                ];
                let query = Query {
                    constraints: vec![
                        c("p", [t("f", [v(0)]), unary(8), v(2)]),
                        c("alias", [v(0), v(1)]),
                        c("bind", [v(1)]),
                    ],
                    outputs: vec![],
                };
                if specialized && policy == Policy::Active {
                    assert!(
                        PreparedRuleset::new(rules, None)
                            .unwrap()
                            .specialize_inferred()
                            .start_search(query, policy, access)
                            .is_err()
                    );
                    continue;
                }
                let (answer, trace, _) = run(rules, query, policy, access, specialized);
                let expected = Answer {
                    outputs: vec![],
                    residual: vec![c("done", [v(0), v(0)])],
                };
                assert!(chr_observe::equivalent(
                    &answer,
                    &expected,
                    &mut Default::default()
                ));
                assert_eq!(trace, vec![1, 2, 0]);
            }
        }
    }
}
