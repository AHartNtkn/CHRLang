use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent, search_bundled, search_fixtures};
use std::collections::BTreeSet;
#[allow(dead_code)]
mod search_support;
fn check_branch(
    rules: &[chr_syntax::Rule],
    query: &chr_syntax::Query,
    lineage: &[bool],
    engine: &mut chr_compiled::Engine,
    failed: bool,
) {
    let mut replay = search_support::Replay::new(query, lineage);
    let mut error = None;
    for (r, ids) in engine.trace() {
        assert!(error.is_none(), "source fired after failure");
        error = replay.fire(*r, &rules[*r], ids).err();
    }
    if let Some(error) = &error {
        assert!(
            [
                "explicit source failure",
                "finite-tree cycle",
                "constructor clash"
            ]
            .contains(&error.as_str()),
            "invalid source replay: {error}"
        );
    }
    assert_eq!(
        error.is_some(),
        failed,
        "independent failure agreement: {error:?}"
    );
    assert!(replay.choices_consumed());
    if !failed {
        assert!(replay.terminal(rules), "missed enabled source work");
        let propagation_history = engine
            .view()
            .history
            .into_iter()
            .filter(|(r, _)| rules[*r].removed.is_empty())
            .collect::<Vec<_>>();
        assert_eq!(replay.history(), propagation_history);
        assert!(chr_observe::equivalent(
            &replay.answer(),
            &engine.observe().unwrap(),
            &mut Default::default()
        ));
        assert_eq!(
            replay.live_ids(),
            engine
                .view()
                .store
                .iter()
                .map(|(id, _)| *id)
                .collect::<Vec<_>>()
        );
    }
}
#[test]
fn branch_tree_has_exact_independent_observations_and_resource_effects() {
    for id in 0..10 {
        for policy in [Policy::Global, Policy::Active] {
            for access in [Access::Scan, Access::Indexed] {
                for generated in 0..3 {
                    let rules = search_fixtures::programs()[id].clone();
                    let (query, mut expected, finite) = search_fixtures::case(id);
                    let prepared = PreparedRuleset::new(
                        rules.clone(),
                        match generated {
                            0 => None,
                            1 => Some(search_bundled(id)),
                            _ => Some(chr_compiled::access_search_bundled(id)),
                        },
                    )
                    .unwrap();
                    let mut search = prepared
                        .start_search(query.clone(), policy, access)
                        .unwrap();
                    search.enable_trace();
                    let mut live = BTreeSet::from([vec![]]);
                    let mut exhausted = false;
                    let mut completions = 0;
                    for _ in 0..20_000 {
                        match search.tick() {
                            SearchEvent::Progress => (),
                            SearchEvent::Split { lineage, .. } => {
                                assert!(live.remove(&lineage));
                                for arm in [false, true] {
                                    let mut child = lineage.clone();
                                    child.push(arm);
                                    assert!(live.insert(child));
                                }
                            }
                            SearchEvent::Complete(mut branch) => {
                                assert!(live.remove(&branch.lineage));
                                check_branch(
                                    &rules,
                                    &query,
                                    &branch.lineage,
                                    &mut branch.engine,
                                    false,
                                );
                                let actual = branch.engine.observe().unwrap();
                                let index = expected
                                    .iter()
                                    .position(|e| {
                                        chr_observe::equivalent(e, &actual, &mut Default::default())
                                    })
                                    .unwrap_or_else(|| panic!("unexpected {id} {actual:?}"));
                                expected.remove(index);
                                completions += 1;
                            }
                            SearchEvent::Failed(mut branch) => {
                                assert!(live.remove(&branch.lineage));
                                check_branch(
                                    &rules,
                                    &query,
                                    &branch.lineage,
                                    &mut branch.engine,
                                    true,
                                );
                            }
                            SearchEvent::Exhausted => {
                                exhausted = true;
                                break;
                            }
                        }
                    }
                    assert!(
                        expected.is_empty(),
                        "missing answers {id} {policy:?} {access:?} native={generated}"
                    );
                    assert_eq!(exhausted, finite);
                    if finite {
                        assert!(live.is_empty());
                    } else {
                        assert_eq!(completions, 1);
                        assert!(!live.is_empty());
                    }
                }
            }
        }
    }
}
#[test]
fn prepared_search_reuse_and_direct_branch_split_are_explicit() {
    let rules = search_fixtures::programs()[1].clone();
    let prepared = PreparedRuleset::new(rules, Some(search_bundled(1))).unwrap();
    for count in [1, 2, 1] {
        let query = chr_syntax::Query {
            constraints: (0..count)
                .map(|i| chr_syntax::c("choose", [chr_syntax::v(i)]))
                .collect(),
            outputs: vec![],
        };
        let mut branch = prepared
            .start(query, Policy::Active, Access::Indexed)
            .unwrap();
        let status = branch.advance(1000);
        assert!(status.pending_split);
        assert!(!status.exhausted);
        assert!(branch.observe().is_none());
        let mut search = branch.into_search();
        let mut raw = 0;
        let mut exhausted = false;
        for _ in 0..10000 {
            match search.tick() {
                SearchEvent::Complete(_) => raw += 1,
                SearchEvent::Exhausted => {
                    exhausted = true;
                    break;
                }
                _ => (),
            }
        }
        assert!(exhausted);
        assert_eq!(raw, 1 << count);
    }
}
