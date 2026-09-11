mod equation_support;
#[cfg(feature = "experiment")]
#[allow(dead_code)]
#[path = "../../chr-compiled/tests/search_support/mod.rs"]
mod search_support;
use equation_support::{Placement, answer_key, query, rules};
const LIMIT: usize = 2_000_000;

fn record(answer: &chr_syntax::Answer, seen: &mut u16) {
    let key =
        answer_key(answer).expect("full answer violates independent alias/correlation oracle");
    assert_eq!(*seen & (1 << key), 0, "duplicate choice answer");
    *seen |= 1 << key;
}

#[test]
fn conditional_equation_placements_complete_and_preserve_joint_aliases() {
    use chr_direct_conditional::engine::{Event, PreparedRuleset};
    for placement in [
        Placement::BeforeGate,
        Placement::AfterGate,
        Placement::NaiveHoist,
    ] {
        let prepared = PreparedRuleset::new(rules(placement)).unwrap();
        for (depth, clash) in [
            (0, false),
            (1, true),
            (8, false),
            (8, true),
            (1, false),
            (0, true),
            (0, false),
        ] {
            let mut engine = prepared.start(query(depth, clash)).unwrap();
            let mut seen = 0;
            let mut retained = vec![];
            let mut exhausted = false;
            for _ in 0..LIMIT {
                match engine.tick() {
                    Event::Progress => (),
                    Event::Answer(answer) => {
                        record(&answer, &mut seen);
                        retained.push(answer);
                    }
                    Event::Exhausted => {
                        exhausted = true;
                        break;
                    }
                }
            }
            assert!(exhausted, "{placement:?} depth={depth} clash={clash}");
            assert_eq!(seen, if clash { 0 } else { u16::MAX });
            assert_eq!(retained.len(), if clash { 0 } else { 16 });
            drop(engine);
            assert!(retained.iter().all(|a| answer_key(a).is_some()));
        }
    }
}

#[cfg(feature = "experiment")]
fn explicit(placement: Placement, depth: usize, clash: bool) -> (usize, usize) {
    use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
    use std::collections::BTreeSet;
    let source = rules(placement);
    let prepared = PreparedRuleset::new(source.clone(), None)
        .unwrap()
        .specialize_inferred();
    // Same immutable preparation also serves a second changed query.
    for input in [query(0, false), query(depth, clash)] {
        let target = input == query(depth, clash);
        let mut engine = prepared
            .start_search(input.clone(), Policy::Global, Access::Indexed)
            .unwrap();
        engine.enable_trace();
        let mut frontier = BTreeSet::from([vec![]]);
        let mut seen = 0;
        let mut failed = 0;
        let mut complete = 0;
        let mut exhausted = false;
        for _ in 0..LIMIT {
            match engine.tick() {
                SearchEvent::Progress => (),
                SearchEvent::Split { lineage, .. } => {
                    assert!(frontier.remove(&lineage));
                    for choice in [false, true] {
                        let mut child = lineage.clone();
                        child.push(choice);
                        assert!(frontier.insert(child));
                    }
                }
                event @ (SearchEvent::Failed(_) | SearchEvent::Complete(_)) => {
                    let is_failed = matches!(&event, SearchEvent::Failed(_));
                    let (SearchEvent::Failed(mut branch) | SearchEvent::Complete(mut branch)) =
                        event
                    else {
                        unreachable!()
                    };
                    assert!(frontier.remove(&branch.lineage));
                    let mut replay = search_support::Replay::new(&input, &branch.lineage);
                    let mut error = None;
                    for (rule, ids) in branch.engine.trace() {
                        assert!(error.is_none(), "source fired after branch failure");
                        error = replay.fire(*rule, &source[*rule], ids).err();
                    }
                    assert!(replay.choices_consumed());
                    if is_failed {
                        assert!(
                            error.as_deref().is_some_and(|e| e.contains("clash")),
                            "unexpected replay failure: {error:?}"
                        );
                        failed += 1;
                    } else {
                        assert!(error.is_none());
                        assert!(replay.terminal(&source));
                        assert_eq!(replay.history(), branch.engine.view().history);
                        let answer = branch.engine.observe().unwrap();
                        assert_eq!(answer_key(&answer), answer_key(&replay.answer()));
                        record(&answer, &mut seen);
                        complete += 1;
                    }
                }
                SearchEvent::Exhausted => {
                    exhausted = true;
                    break;
                }
            }
        }
        assert!(exhausted);
        assert!(frontier.is_empty());
        if target {
            return (complete, failed);
        }
        assert_eq!((complete, failed, seen), (16, 0, u16::MAX));
    }
    unreachable!()
}

#[cfg(feature = "experiment")]
#[test]
fn specialized_source_replay_conserves_success_and_failed_lineages() {
    for placement in [Placement::BeforeGate, Placement::AfterGate] {
        for depth in [0, 1, 8] {
            assert_eq!(explicit(placement, depth, false), (16, 0));
            assert_eq!(explicit(placement, depth, true), (0, 16));
        }
    }
}

#[cfg(feature = "experiment")]
#[test]
fn naive_hoist_does_not_preserve_failed_source_tree() {
    assert_eq!(explicit(Placement::BeforeGate, 8, true), (0, 16));
    // Equal empty answer multisets do not imply equal source failure trees.
    assert_eq!(explicit(Placement::NaiveHoist, 8, true), (0, 1));
}

#[test]
fn full_oracle_rejects_alias_tuple_and_multiplicity_mutations() {
    use chr_syntax::{Answer, atom, c, v};
    for key in 0..16 {
        let answer = Answer {
            outputs: vec![
                ("x".into(), v(7)),
                ("y".into(), v(7)),
                ("z".into(), v(7)),
                ("unused".into(), v(8)),
            ],
            residual: vec![
                c("witness", [equation_support::tuple(key), v(7), v(7)]),
                c("done", [v(7), v(7)]),
            ],
        };
        assert_eq!(answer_key(&answer), Some(key));
        for index in 0..5 {
            let mut bad = answer.clone();
            match index {
                0 => bad.outputs[1].1 = v(8),
                1 => bad.outputs[3].1 = v(7),
                2 => bad.residual[0].args[0] = atom("tuple"),
                3 => bad.residual.push(bad.residual[1].clone()),
                _ => bad.residual[1].args[0] = atom("ground"),
            }
            assert_eq!(answer_key(&bad), None);
        }
    }
}
