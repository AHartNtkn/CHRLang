#[allow(dead_code)]
mod search_support;
mod state_preservation_support;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
use state_preservation_support::{answer_matches, expected_raw, query, rules};
#[test]
fn independent_live_state_and_complete_branch_gate() {
    let source = rules();
    let prepared = PreparedRuleset::new(source.clone(), None)
        .unwrap()
        .specialize_inferred();
    for n in [0, 1, 4, 16] {
        for alternatives in [1, 2, 5] {
            for all_success in [false, true] {
                let q = query(n, alternatives, all_success);
                let mut e = prepared
                    .start_search(q.clone(), Policy::Global, Access::Indexed)
                    .unwrap();
                e.enable_trace();
                let mut live = std::collections::BTreeSet::from([vec![]]);
                let mut complete = 0;
                let mut failed = 0;
                let mut splits = 0;
                let mut exhausted = false;
                for _ in 0..200_000 {
                    match e.tick() {
                        SearchEvent::Split { lineage, .. } => {
                            assert!(live.remove(&lineage));
                            for side in [false, true] {
                                let mut child = lineage.clone();
                                child.push(side);
                                assert!(live.insert(child));
                            }
                            splits += 1;
                        }
                        SearchEvent::Complete(mut b) | SearchEvent::Failed(mut b) => {
                            assert!(live.remove(&b.lineage));
                            let failure = b.engine.status().failed;
                            let mut replay = search_support::Replay::new(&q, &b.lineage);
                            let mut error = None;
                            for (r, ids) in b.engine.trace() {
                                assert!(error.is_none());
                                error = replay.fire(*r, &source[*r], ids).err();
                            }
                            assert_eq!(error.is_some(), failure);
                            assert!(replay.choices_consumed());
                            if failure {
                                assert_eq!(error.as_deref(), Some("constructor clash"));
                                check_payload(&b.engine.view(), n);
                                failed += 1;
                            } else {
                                assert!(replay.terminal(&source));
                                let a = b.engine.observe().unwrap();
                                assert!(answer_matches(&a));
                                assert!(chr_observe::equivalent(
                                    &a,
                                    &replay.answer(),
                                    &mut Default::default()
                                ));
                                complete += 1;
                            }
                        }
                        SearchEvent::Exhausted => {
                            exhausted = true;
                            break;
                        }
                        SearchEvent::Progress => (),
                    }
                }
                assert!(exhausted);
                assert!(live.is_empty());
                assert_eq!(splits, alternatives - 1);
                assert_eq!(complete, expected_raw(alternatives, all_success));
                assert_eq!(failed, if all_success { 0 } else { alternatives - 1 });
            }
        }
    }
}

fn check_payload(view: &chr_compiled::View, n: usize) {
    use chr_syntax::{Term, c, t};
    let shared = view.outputs[0].1.clone();
    assert!(matches!(shared, Term::Var(_)));
    assert_eq!(
        view.store.len(),
        n,
        "payload must exist as live source occurrences"
    );
    for ((_, actual), key) in view.store.iter().zip((1..=n).rev()) {
        let key = state_preservation_support::unary(key);
        assert_eq!(
            *actual,
            c(
                "payload",
                [
                    key.clone(),
                    t("cell", [key, shared.clone(), shared.clone()])
                ]
            )
        );
    }
}
#[test]
fn source_payload_is_live_and_bound_before_first_split() {
    let source = rules();
    let prepared = PreparedRuleset::new(source.clone(), None)
        .unwrap()
        .specialize_inferred();
    for n in [0, 1, 8, 32] {
        for all_success in [false, true] {
            let q = query(n, 3, all_success);
            let mut engine = prepared.start(q, Policy::Global, Access::Indexed).unwrap();
            engine.enable_trace();
            let status = engine.advance(100_000);
            assert!(status.pending_split && !status.exhausted && !status.failed);
            assert!(engine.observe().is_none());
            check_payload(&engine.view(), n);
            assert_eq!(
                engine
                    .trace()
                    .iter()
                    .filter(|(rule, _)| source[*rule].name == "build-step")
                    .count(),
                n
            );
            assert_eq!(
                engine
                    .trace()
                    .iter()
                    .filter(|(rule, _)| source[*rule].name == "build-done")
                    .count(),
                1
            );
            // Dropping pending continuation must leave the reusable prepared ruleset usable.
            drop(engine);
            let mut fresh = prepared
                .start_search(query(1, 1, false), Policy::Global, Access::Indexed)
                .unwrap();
            let mut answers = 0;
            let mut exhausted = false;
            for _ in 0..10_000 {
                match fresh.tick() {
                    SearchEvent::Complete(mut b) => {
                        assert!(answer_matches(&b.engine.observe().unwrap()));
                        answers += 1
                    }
                    SearchEvent::Exhausted => {
                        exhausted = true;
                        break;
                    }
                    _ => (),
                }
            }
            assert!(exhausted);
            assert_eq!(answers, 1);
        }
    }
}
#[test]
fn payload_omission_or_broken_alias_prevents_small_answer() {
    use chr_syntax::{Goal, t, v};
    for omit in [true, false] {
        let mut source = rules();
        let Goal::And(body) = &mut source[0].body else {
            panic!("build conjunction")
        };
        if omit {
            body.remove(1);
        } else {
            body[0] = chr_syntax::eq(v(5), t("cell", [t("s", [v(0)]), v(3), v(6)]));
        }
        let prepared = PreparedRuleset::new(source, None)
            .unwrap()
            .specialize_inferred();
        let mut search = prepared
            .start_search(query(2, 1, false), Policy::Global, Access::Indexed)
            .unwrap();
        let mut complete = false;
        let mut exhausted = false;
        for _ in 0..20_000 {
            match search.tick() {
                SearchEvent::Complete(mut b) => {
                    assert!(!answer_matches(&b.engine.observe().unwrap()));
                    complete = true
                }
                SearchEvent::Exhausted => {
                    exhausted = true;
                    break;
                }
                _ => (),
            }
        }
        assert!(complete && exhausted);
    }
}
#[test]
fn complete_answer_checker_rejects_alias_and_multiplicity_changes() {
    use chr_syntax::{Answer, c, v};
    let good = Answer {
        outputs: vec![
            ("x".into(), v(44)),
            ("alias".into(), v(44)),
            ("unused".into(), v(45)),
        ],
        residual: vec![c("done", [v(44), v(44)]), c("done", [v(44), v(44)])],
    };
    assert!(answer_matches(&good));
    let mut bad = good.clone();
    bad.residual.pop();
    assert!(!answer_matches(&bad));
    let mut bad = good.clone();
    bad.residual.push(bad.residual[0].clone());
    assert!(!answer_matches(&bad));
    let mut bad = good.clone();
    bad.outputs[2].1 = v(44);
    assert!(!answer_matches(&bad));
    let mut bad = good;
    bad.residual[1].args[1] = v(46);
    assert!(!answer_matches(&bad));
}
#[test]
fn requested_key_selects_early_late_or_no_success_without_changing_rules() {
    let source = rules();
    let prepared = PreparedRuleset::new(source.clone(), None)
        .unwrap()
        .specialize_inferred();
    for wanted in 0..6 {
        let mut q = query(3, 5, false);
        q.constraints[0].args[4] = state_preservation_support::unary(wanted);
        let mut search = prepared
            .start_search(q.clone(), Policy::Global, Access::Indexed)
            .unwrap();
        search.enable_trace();
        let mut successes = 0;
        let mut failures = 0;
        let mut exhausted = false;
        for _ in 0..100_000 {
            match search.tick() {
                SearchEvent::Complete(mut branch) => {
                    assert!(answer_matches(&branch.engine.observe().unwrap()));
                    successes += 1;
                }
                SearchEvent::Failed(branch) => {
                    let mut replay = search_support::Replay::new(&q, &branch.lineage);
                    let mut error = None;
                    for (r, ids) in branch.engine.trace() {
                        assert!(error.is_none());
                        error = replay.fire(*r, &source[*r], ids).err();
                    }
                    assert_eq!(error.as_deref(), Some("constructor clash"));
                    failures += 1;
                }
                SearchEvent::Exhausted => {
                    exhausted = true;
                    break;
                }
                _ => (),
            }
        }
        assert!(exhausted);
        assert_eq!(successes, usize::from(wanted < 5));
        assert_eq!(successes + failures, 5);
    }
}
