#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../experiments/subscription_source.rs"]
mod source;
use chr_syntax::{Answer, Constraint, Query, Term, atom, c, t, v};
use source::{query, source_rules};
fn open() -> Term {
    t("open", [atom("k"), atom("yes"), atom("d")])
}
fn ask(round: &str) -> Term {
    t("ask", [atom("d"), atom(round)])
}
fn close() -> Term {
    t("close", [atom("d")])
}
fn rows() -> Vec<Constraint> {
    vec![
        c("left", [atom("k"), t("f", [atom("a")])]),
        c("middle", [atom("a"), t("g", [atom("b")])]),
        c("right", [atom("b"), t("h", [atom("yes")])]),
    ]
}
fn receipt(round: &str) -> Constraint {
    c(
        "receipt",
        [
            atom("d"),
            atom(round),
            t("f", [atom("a")]),
            t("g", [atom("b")]),
            t("h", [atom("yes")]),
        ],
    )
}
fn check(q: &Query, consuming: bool, expected: Vec<Constraint>) -> Answer {
    let rules = source_rules(consuming);
    let mut answers = oracle::run(&rules, q, 200_000);
    assert_eq!(answers.len(), 1);
    let answer = answers.pop().unwrap();
    let mut actual: Vec<_> = answer
        .residual
        .iter()
        .filter(|x| x.name == "receipt")
        .cloned()
        .collect();
    let mut expected = expected;
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
    let p = chr_compiled::PreparedRuleset::new(rules, None).unwrap();
    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
        let mut engine = p
            .start(q.clone(), chr_compiled::Policy::Global, access)
            .unwrap();
        let end = engine.advance(200_000);
        assert!(end.exhausted && !end.failed);
        oracle::same_raw(vec![engine.observe().unwrap()], vec![answer.clone()]);
    }
    answer
}
#[test]
fn live_demand_repeated_pulses_retirement_and_reopening() {
    for first in [false, true] {
        let q = query(
            rows(),
            vec![
                ask("absent"),
                open(),
                ask("one"),
                ask("two"),
                close(),
                ask("retired"),
                open(),
                ask("three"),
                close(),
            ],
            first,
        );
        let answer = check(
            &q,
            false,
            vec![receipt("one"), receipt("two"), receipt("three")],
        );
        let mut expected = rows();
        expected.extend([
            receipt("one"),
            receipt("two"),
            receipt("three"),
            c("done", []),
        ]);
        oracle::same_raw(
            vec![answer],
            vec![Answer {
                outputs: vec![("x".into(), v(50)), ("y".into(), v(51))],
                residual: expected,
            }],
        );
    }
}
#[test]
fn occurrence_multiplicity_selectivity_and_consuming_shared_partners() {
    for nl in [1, 3] {
        for nm in [1, 2] {
            for nr in [1, 3] {
                for consuming in [false, true] {
                    for first in [false, true] {
                        let base = rows();
                        let mut data = vec![base[0].clone(); nl];
                        data.extend(vec![base[1].clone(); nm]);
                        data.extend(vec![base[2].clone(); nr]);
                        data.push(c("right", [atom("b"), t("h", [atom("no")])]));
                        data.push(c("middle", [atom("a"), t("g", [atom("disconnected")])]));
                        let q = query(data, vec![open(), ask("one"), ask("two"), close()], first);
                        let expected = if consuming {
                            vec![receipt("one"); nr]
                        } else {
                            let mut r = vec![receipt("one"); nl * nm * nr];
                            r.extend(vec![receipt("two"); nl * nm * nr]);
                            r
                        };
                        let answer = check(&q, consuming, expected);
                        assert_eq!(
                            answer.residual.iter().filter(|c| c.name == "right").count(),
                            if consuming { 1 } else { nr + 1 }
                        );
                    }
                }
            }
        }
    }
}
#[test]
fn updates_to_each_relation_and_demand_retirement() {
    for relation in ["left", "middle", "right"] {
        for first in [false, true] {
            let base = rows();
            let r = base.iter().find(|r| r.name == relation).unwrap();
            let q = query(
                base.clone(),
                vec![
                    open(),
                    ask("before"),
                    t(&format!("remove_{relation}"), r.args.clone()),
                    ask("missing"),
                    t(&format!("insert_{relation}"), r.args.clone()),
                    ask("restored"),
                    close(),
                    t(&format!("insert_{relation}"), r.args.clone()),
                    ask("retired"),
                ],
                first,
            );
            check(&q, false, vec![receipt("before"), receipt("restored")]);
        }
    }
}
#[test]
fn aliases_and_structural_binding_enable_matches_without_binding_during_match() {
    for first in [false, true] {
        let mut data = rows();
        data[0].args[1] = v(50);
        let q = query(
            data,
            vec![
                open(),
                ask("unknown"),
                t("bind", [v(50), v(51)]),
                ask("alias"),
                t("bind", [v(51), t("f", [atom("a")])]),
                ask("enabled"),
                close(),
            ],
            first,
        );
        let answer = check(&q, false, vec![receipt("enabled")]);
        assert_eq!(
            answer.outputs,
            vec![
                ("x".into(), t("f", [atom("a")])),
                ("y".into(), t("f", [atom("a")]))
            ]
        );
    }
}
#[test]
fn duplicate_demand_occurrences_have_independent_history_and_retirement() {
    let q = query(
        rows(),
        vec![
            open(),
            open(),
            ask("both"),
            close(),
            ask("one"),
            close(),
            ask("none"),
        ],
        false,
    );
    check(
        &q,
        false,
        vec![receipt("both"), receipt("both"), receipt("one")],
    );
}
#[test]
fn unmatched_resource_update_blocks_driver_with_exact_residual() {
    let ops = vec![
        t("remove_right", [atom("missing"), t("h", [atom("yes")])]),
        open(),
        ask("unreached"),
    ];
    let q = query(rows(), ops, false);
    let a = check(&q, false, vec![]);
    oracle::same_raw(
        vec![a],
        vec![Answer {
            outputs: vec![("x".into(), v(50)), ("y".into(), v(51))],
            residual: q.constraints,
        }],
    );
}
