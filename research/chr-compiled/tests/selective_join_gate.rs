#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_compiled::selective_join::{Mode, Prepared, source_rules};
use chr_syntax::{Answer, Query, Term, Var, atom, c, t, v};
fn script(ops: Vec<Term>) -> Term {
    ops.into_iter()
        .rev()
        .fold(atom("nil"), |tail, op| t("cons", [op, tail]))
}
fn query(mut rows: Vec<chr_syntax::Constraint>, ops: Vec<Term>, first: bool) -> Query {
    let driver = c("drive", [script(ops)]);
    if first {
        rows.insert(0, driver);
    } else {
        rows.push(driver);
    }
    Query {
        constraints: rows,
        outputs: vec![("x".into(), Var(50)), ("y".into(), Var(51))],
    }
}
fn run(prepared: &Prepared, q: &Query, mode: Mode, quantum: usize) -> Answer {
    let mut e = prepared.start(q.clone(), mode).unwrap();
    assert!(!e.advance(0));
    assert!(e.observe().is_err());
    for _ in 0..2_000_000 / quantum {
        if e.advance(quantum) {
            return e.observe().unwrap();
        }
    }
    panic!("lowering cutoff");
}
fn check(q: &Query, consuming: bool) {
    let rules = source_rules(consuming);
    let expected = oracle::run(&rules, q, 2_000_000);
    assert_eq!(expected.len(), 1);
    let p = Prepared::new(&rules).unwrap();
    for mode in [Mode::Direct, Mode::Retained] {
        oracle::same_raw(vec![run(&p, q, mode, 37)], expected.clone());
        oracle::same_raw(vec![run(&p, q, mode, 1)], expected.clone());
    }
    let control = chr_compiled::PreparedRuleset::new(rules, None).unwrap();
    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
        let mut e = control
            .start(q.clone(), chr_compiled::Policy::Global, access)
            .unwrap();
        let end = e.advance(2_000_000);
        assert!(end.exhausted && !end.failed);
        oracle::same_raw(vec![e.observe().unwrap()], expected.clone());
    }
}
#[test]
fn key_fanout_selectivity_updates_and_consumption_match_source() {
    let mut count = 0;
    for n in [1, 4, 8] {
        for groups in [1, n] {
            for payloads in [1, n] {
                for changed in [0, 1, n] {
                    for consuming in [false, true] {
                        for first in [false, true] {
                            let key = |i| atom(&format!("k{}", i % groups));
                            let value = |i| atom(&format!("v{}", i % payloads));
                            let mut rows = Vec::new();
                            for i in 0..n {
                                rows.push(c("left", [key(i), t("f", [value(i)])]));
                                rows.push(c("right", [key(i), t("g", [value(i)])]));
                            }
                            let mut ops = vec![t("req", [key(0), atom("before")])];
                            for i in 0..changed {
                                ops.push(t(
                                    "replace",
                                    [key(i), t("g", [value(i)]), t("g", [atom("new")])],
                                ));
                            }
                            ops.push(t("req", [key(0), atom("after")]));
                            ops.push(t("insert", [key(0), t("g", [value(0)])]));
                            ops.push(t("req", [key(0), atom("inserted")]));
                            check(&query(rows, ops, first), consuming);
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(count, 144);
}
#[test]
fn structural_unknowns_broad_aliases_and_late_receipt_values() {
    for consuming in [false, true] {
        for first in [false, true] {
            for broad in [false, true] {
                let mut rows = Vec::new();
                for i in 0..4 {
                    rows.push(c(
                        "left",
                        [
                            atom("k"),
                            t("f", [if broad || i == 0 { v(50) } else { atom("a") }]),
                        ],
                    ));
                    rows.push(c("right", [atom("k"), t("g", [atom("a")])]));
                }
                check(
                    &query(
                        rows,
                        vec![
                            t("req", [atom("k"), atom("before")]),
                            t("bind", [v(50), atom("a")]),
                            t("req", [atom("k"), atom("after")]),
                        ],
                        first,
                    ),
                    consuming,
                );
            }
        }
        check(
            &query(
                vec![
                    c("left", [v(50), v(51)]),
                    c("right", [atom("k"), t("g", [atom("a")])]),
                ],
                vec![
                    t("req", [atom("k"), atom("before")]),
                    t("bind", [v(50), atom("k")]),
                    t("bind", [v(51), t("f", [atom("a")])]),
                    t("req", [atom("k"), atom("after")]),
                ],
                false,
            ),
            consuming,
        );
        check(
            &query(
                vec![
                    c("left", [atom("k"), t("f", [v(50)])]),
                    c("right", [atom("k"), t("g", [v(50)])]),
                ],
                vec![
                    t("req", [atom("k"), atom("before")]),
                    t("bind", [v(50), atom("a")]),
                ],
                false,
            ),
            consuming,
        );
    }
}
#[test]
fn retained_consumption_invalidates_all_shared_partners_and_reuses_preparation() {
    let rows = vec![c("left", [atom("k"), t("f", [atom("a")])]); 4];
    let mut rows = rows;
    rows.extend(vec![c("right", [atom("k"), t("g", [atom("a")])]); 3]);
    let q = query(
        rows,
        vec![
            t("req", [atom("k"), atom("one")]),
            t("req", [atom("k"), atom("empty")]),
        ],
        false,
    );
    let p = Prepared::new(&source_rules(true)).unwrap();
    let mut e = p.start(q.clone(), Mode::Retained).unwrap();
    assert_eq!(e.retained_pairs(), 12);
    assert!(e.advance(100));
    assert_eq!(e.retained_pairs(), 0);
    let answer = e.observe().unwrap();
    assert_eq!(
        answer
            .residual
            .iter()
            .filter(|c| c.name == "receipt")
            .count(),
        3
    );
    assert!(!answer.residual.iter().any(|c| c.name == "right"));
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(e.stats().constructed_pairs, 12);
        assert_eq!(e.stats().invalidated_pairs, 12);
        assert_eq!(e.stats().emitted, 3);
    } else {
        assert_eq!(e.stats().constructed_pairs, 0);
        assert_eq!(e.stats().invalidated_pairs, 0);
        assert_eq!(e.stats().emitted, 0);
    }
    let mut canceled = p.start(q.clone(), Mode::Retained).unwrap();
    canceled.advance(1);
    drop(canceled);
    oracle::same_raw(vec![run(&p, &q, Mode::Retained, 1)], vec![answer]);
    check(&q, true);
}
#[test]
fn zero_requests_missing_matches_and_certificate_boundaries() {
    for consuming in [false, true] {
        let rules = source_rules(consuming);
        let p = Prepared::new(&rules).unwrap();
        check(
            &query(
                vec![
                    c("left", [atom("k"), atom("wrong")]),
                    c("right", [atom("k"), t("g", [atom("a")])]),
                ],
                vec![],
                false,
            ),
            consuming,
        );
        check(
            &query(vec![], vec![t("req", [atom("missing"), atom("r")])], true),
            consuming,
        );
        let mut changed = rules.clone();
        changed.swap(0, 1);
        assert!(Prepared::new(&changed).is_err());
        let q = query(
            vec![],
            vec![t("bind", [v(50), atom("a")]), t("bind", [v(50), atom("b")])],
            false,
        );
        assert!(p.start(q, Mode::Direct).is_err());
    }
}

#[test]
fn selective_work_diagnostics_separate_lookup_from_retention() {
    let mut rows = Vec::new();
    for i in 0..8 {
        rows.push(c("left", [atom("k"), t("f", [atom(&format!("v{i}"))])]));
    }
    for _ in 0..4 {
        rows.push(c("right", [atom("k"), t("g", [atom("v0")])]));
    }
    let q = query(
        rows,
        (0..3)
            .map(|i| t("req", [atom("k"), atom(&format!("r{i}"))]))
            .collect(),
        false,
    );
    let p = Prepared::new(&source_rules(false)).unwrap();
    for mode in [Mode::Direct, Mode::Retained] {
        let mut e = p.start(q.clone(), mode).unwrap();
        assert!(e.advance(100));
        assert_eq!(
            e.observe()
                .unwrap()
                .residual
                .iter()
                .filter(|c| c.name == "receipt")
                .count(),
            12
        );
        let s = e.stats();
        if chr_compiled::COLLECT_METRICS {
            if mode == Mode::Direct {
                assert_eq!(s.constructed_pairs, 0);
                assert_eq!(s.direct_pairs, 12);
            } else {
                assert_eq!(s.constructed_pairs, 4);
                assert_eq!(s.retained_visits, 12);
            }
        } else {
            assert_eq!(
                s.constructed_pairs + s.direct_pairs + s.direct_lookups + s.retained_visits,
                0
            );
        }
        println!(
            "S01_SELECTIVE_WORK {mode:?} constructed={} invalidated={} lookups={} direct_pairs={} retained={} emitted={}",
            s.constructed_pairs,
            s.invalidated_pairs,
            s.direct_lookups,
            s.direct_pairs,
            s.retained_visits,
            s.emitted
        );
    }
}

#[test]
fn blocked_tail_after_binding_and_consumption_preserves_source_residual() {
    let q = query(
        vec![
            c("left", [atom("k"), t("f", [v(50)])]),
            c("right", [atom("k"), t("g", [atom("a")])]),
        ],
        vec![
            t("bind", [v(50), atom("a")]),
            t("req", [atom("k"), atom("first")]),
            t("replace", [atom("k"), t("g", [v(50)]), t("g", [atom("b")])]),
            t("insert", [atom("k"), t("g", [atom("a")])]),
            t("req", [atom("k"), atom("unreached")]),
        ],
        false,
    );
    check(&q, true);
}
