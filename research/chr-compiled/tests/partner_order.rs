//! Source-order discrimination, with a scalar complete ground-answer oracle.
use chr_compiled::{Access, Policy, PreparedRuleset};
use chr_syntax::{Constraint, Query, Rule, atom, c, v};
fn key(prefix: &str, i: usize) -> chr_syntax::Term {
    atom(&format!("{prefix}{i}"))
}
fn rules(right_first: bool, bound_right: bool) -> Vec<Rule> {
    let mut kept = vec![c("left", [v(0), v(1)]), c("right", [v(1), v(2)])];
    if right_first {
        kept.reverse();
    }
    vec![Rule {
        name: "join".into(),
        kept,
        removed: vec![c("request", [v(if bound_right { 2 } else { 0 })])],
        guards: vec![],
        body: c("receipt", [v(0), v(1), v(2)]).into(),
    }]
}
fn input(
    n: usize,
    reverse: bool,
    bound_right: bool,
    missing: bool,
    target: usize,
) -> (Query, Vec<Constraint>) {
    let mut facts = Vec::new();
    for i in 0..n {
        facts.push(c("left", [key("a", i), key("x", i)]));
        facts.push(c("right", [key("x", i), key("b", i)]));
    }
    if reverse {
        facts.reverse();
    }
    let req = c(
        "request",
        [if missing {
            atom("absent")
        } else {
            key(if bound_right { "b" } else { "a" }, target)
        }],
    );
    let mut expected = facts.clone();
    expected.push(if missing {
        req.clone()
    } else {
        c(
            "receipt",
            [key("a", target), key("x", target), key("b", target)],
        )
    });
    facts.push(req);
    (
        Query {
            constraints: facts,
            outputs: vec![],
        },
        expected,
    )
}
#[test]
fn opposite_keys_preserve_complete_unique_answers() {
    let mut cases = 0;
    for n in [8, 32, 128] {
        for right in [false, true] {
            for missing in [false, true] {
                for reverse in [false, true] {
                    for policy in [Policy::Global, Policy::Active] {
                        for access in [Access::Scan, Access::Indexed] {
                            for order in [false, true] {
                                let prepared =
                                    PreparedRuleset::new(rules(order, right), None).unwrap();
                                for target in [0, n - 1] {
                                    let (query, mut expected) =
                                        input(n, reverse, right, missing, target);
                                    let mut engine = prepared.start(query, policy, access).unwrap();
                                    let status = engine.advance(2_000_000);
                                    assert!(
                                        status.exhausted && !status.failed && !status.pending_split,
                                        "case {cases}"
                                    );
                                    let mut answer = engine.observe().unwrap();
                                    assert!(answer.outputs.is_empty());
                                    answer.residual.sort();
                                    expected.sort();
                                    assert_eq!(answer.residual, expected, "case {cases}");
                                    let s = engine.stats();
                                    if chr_compiled::COLLECT_METRICS {
                                        assert_eq!(s.applications, u64::from(!missing));
                                    }
                                    println!(
                                        "PARTNER,id={cases},n={n},right={right},missing={missing},reverse={reverse},policy={policy:?},access={access:?},order={order},target={target},metrics={},candidate={},pool={},structural={},index={}",
                                        chr_compiled::COLLECT_METRICS,
                                        s.candidate_visits,
                                        s.pool_visits,
                                        s.structural_tests,
                                        s.index_lookups
                                    );
                                    cases += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(cases, 384);
}
#[test]
fn competing_matches_expose_order_contract() {
    let query = Query {
        constraints: vec![
            c("left", [atom("a"), key("x", 0)]),
            c("left", [atom("a"), key("x", 1)]),
            c("right", [key("x", 1), atom("b")]),
            c("right", [key("x", 0), atom("b")]),
            c("request", [atom("b")]),
        ],
        outputs: vec![],
    };
    for access in [Access::Scan, Access::Indexed] {
        for order in [false, true] {
            let prepared = PreparedRuleset::new(rules(order, true), None).unwrap();
            let mut engine = prepared
                .start(query.clone(), Policy::Global, access)
                .unwrap();
            let status = engine.advance(10000);
            assert!(status.exhausted && !status.failed);
            let mut answer = engine.observe().unwrap();
            let mut expected = query.constraints[..4].to_vec();
            expected.push(c(
                "receipt",
                [atom("a"), key("x", usize::from(order)), atom("b")],
            ));
            answer.residual.sort();
            expected.sort();
            assert_eq!(answer.residual, expected);
        }
    }
    println!("PARTNER_SENTINEL,validated=4");
}
