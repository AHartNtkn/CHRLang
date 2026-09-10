#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[allow(dead_code)]
#[path = "../examples/support/deduction_source.rs"]
mod source;
use chr_relational::{
    contextual::Store,
    contextual_execute::{Prepared, Step},
};
use chr_syntax::{atom, c, t, v};
fn settle(store: &mut Store) {
    for _ in 0..1000 {
        if !store.step() {
            return;
        }
    }
    panic!("store service cutoff");
}
#[test]
fn reuse_across_unrelated_bindings_preserves_caller_state_and_claims() {
    for persistent in [false, true] {
        let mut base = Store::default().with_relevant_deductions();
        if persistent {
            base = base.with_persistent_equality();
        }
        let x = base.unknown();
        let y = base.unknown();
        let context = base.unknown();
        let a = base.constructor("a", &[]);
        let b = base.constructor("b", &[]);
        let u = base.constructor("u", &[]);
        let w = base.constructor("w", &[]);
        let fx = base.constructor("f", &[x]);
        let fa = base.constructor("f", &[a]);
        base.post("open", &[fx]);
        base.post("ticket", &[x]);
        base.post("ticket", &[x]);
        let mut left = base.clone();
        let mut right = base.clone();
        left.equate(context, u);
        right.equate(context, w);
        settle(&mut left);
        settle(&mut right);
        left.equate(fx, fa);
        left.equate(y, a);
        settle(&mut left);
        #[cfg(feature = "deduction-work")]
        let before = base.relevant_deduction_hits();
        right.equate(fx, fa);
        right.equate(y, b);
        settle(&mut right);
        #[cfg(feature = "deduction-work")]
        assert!(
            base.relevant_deduction_hits() > before,
            "must reuse a new deduction across distinct maps"
        );
        assert_eq!(
            left.export(&[x, y, context]),
            Some(vec![atom("a"), atom("a"), atom("u")])
        );
        assert_eq!(
            right.export(&[x, y, context]),
            Some(vec![atom("a"), atom("b"), atom("w")])
        );
        assert!(!left.shares_equality(&right));
        let heads = [c("open", [t("f", [v(0)])]), c("ticket", [v(0)])];
        let claims = left.matches(&[], &heads);
        let tuples: std::collections::BTreeSet<_> =
            claims.iter().map(|m| m.removed.clone()).collect();
        assert_eq!(tuples.len(), 2);
        assert!(left.consume(&claims[0]));
        let right_tuples: std::collections::BTreeSet<_> = right
            .matches(&[], &heads)
            .into_iter()
            .map(|m| m.removed)
            .collect();
        assert_eq!(right_tuples.len(), 2);
        assert_eq!(base.matches(&[], &heads).len(), 2);
    }
}
#[test]
fn relevant_changes_and_cycles_do_not_replay_compatible_results() {
    for persistent in [false, true] {
        let mut base = Store::default().with_relevant_deductions();
        if persistent {
            base = base.with_persistent_equality();
        }
        let x = base.unknown();
        let context = base.unknown();
        let a = base.constructor("a", &[]);
        let b = base.constructor("b", &[]);
        let fx = base.constructor("f", &[x]);
        let fa = base.constructor("f", &[a]);
        let mut good = base.clone();
        good.equate(fx, fa);
        settle(&mut good);
        let mut changed = base.clone();
        changed.equate(x, b);
        settle(&mut changed);
        changed.equate(fx, fa);
        settle(&mut changed);
        assert!(changed.failed());
        assert!(!good.failed());
        assert!(!base.failed());
        for value in [a, b] {
            let mut cycle = base.clone();
            cycle.equate(context, value);
            settle(&mut cycle);
            cycle.equate(x, fx);
            settle(&mut cycle);
            assert!(cycle.failed());
        }
        assert_eq!(good.export(&[x]), Some(vec![atom("a")]));
    }
}
#[test]
fn complete_sources_match_independent_controls() {
    let mut cases = 0;
    #[cfg(feature = "deduction-work")]
    let mut changed_hits = 0;
    for family in ["single", "shared", "distinct", "changed"] {
        for depth in [0, 4, 16] {
            for resource in [false, true] {
                for reverse in [false, true] {
                    let schema = source::Schema::new(family, resource);
                    let rules = schema.rules();
                    let query = schema.query(depth, reverse);
                    let expected = oracle::run(&rules, &query, 200_000);
                    let p = Prepared::new(&rules).unwrap();
                    for (mode, mut engine) in [
                        ("plain", p.start(&query)),
                        ("exact", p.start_shared_deductions(&query)),
                        ("relevant", p.start_relevant_deductions(&query)),
                    ] {
                        let mut actual = vec![];
                        let mut done = false;
                        #[cfg(feature = "deduction-work")]
                        let mut hits = 0;
                        for _ in 0..200_000 {
                            match engine.advance() {
                                Step::Answer(a) => actual.push(a),
                                Step::Exhausted => {
                                    done = true;
                                    break;
                                }
                                Step::Progress => {}
                            }
                            #[cfg(feature = "deduction-work")]
                            {
                                hits = hits.max(engine.relevant_deduction_hits());
                            }
                        }
                        assert!(done, "source cutoff {family} {depth} {mode}");
                        oracle::same_raw(actual, expected.clone());
                        #[cfg(feature = "deduction-work")]
                        {
                            println!(
                                "READ_REUSE,{family},{depth},{resource},{reverse},{mode},{hits}"
                            );
                            if family == "changed" && mode == "relevant" {
                                changed_hits += hits;
                            }
                        }
                    }
                    let p = chr_compiled::PreparedRuleset::new(rules, None).unwrap();
                    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                        let mut engine = p
                            .start_search(query.clone(), chr_compiled::Policy::Global, access)
                            .unwrap();
                        let mut actual = vec![];
                        let mut done = false;
                        for _ in 0..200_000 {
                            match engine.tick() {
                                chr_compiled::SearchEvent::Complete(mut b) => {
                                    actual.push(b.engine.observe().unwrap())
                                }
                                chr_compiled::SearchEvent::Exhausted => {
                                    done = true;
                                    break;
                                }
                                _ => {}
                            }
                        }
                        assert!(done);
                        oracle::same_raw(actual, expected.clone());
                    }
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 48);
    #[cfg(feature = "deduction-work")]
    assert!(
        changed_hits > 0,
        "complete source must exercise cross-state deduction reuse"
    );
}

#[test]
fn descendant_reads_distinguish_cached_occurs_failure_from_valid_merge() {
    let mut base = Store::default().with_relevant_deductions();
    let x = base.unknown();
    let y = base.unknown();
    let fx = base.constructor("f", &[x]);
    let mut failed = base.clone();
    failed.equate(y, x);
    settle(&mut failed);
    failed.equate(fx, y);
    settle(&mut failed);
    assert!(failed.failed());
    let mut valid = base.clone();
    valid.equate(fx, y);
    settle(&mut valid);
    assert!(!valid.failed());
    let values = valid.export(&[x, y]).unwrap();
    assert_eq!(values[1], t("f", [values[0].clone()]));
}
