#[allow(dead_code)]
mod search_support;
mod update_join_support;
use chr_compiled::{Access, Policy, PreparedRuleset};
use chr_syntax::{Answer, Query, Rule, Var, and, atom, c, v};
use update_join_support::{Case, Event, rules};

fn equivalent(a: &chr_syntax::Answer, b: &chr_syntax::Answer) -> bool {
    chr_observe::equivalent(a, b, &mut Default::default())
}
fn execute(
    case: &Case,
    policy: Policy,
    access: Access,
    first: bool,
    quantum: usize,
) -> chr_syntax::Answer {
    execute_source(rules(), case.query(first), policy, access, quantum)
}
fn execute_source(
    source: Vec<Rule>,
    query: Query,
    policy: Policy,
    access: Access,
    quantum: usize,
) -> Answer {
    let mut engine = PreparedRuleset::new(source.clone(), None)
        .unwrap()
        .start(query.clone(), policy, access)
        .unwrap();
    engine.enable_trace();
    assert!(!engine.advance(0).exhausted);
    let mut exhausted = false;
    for _ in 0..(2_000_000 / quantum) {
        let status = engine.advance(quantum);
        assert!(!status.failed && !status.pending_split);
        if status.exhausted {
            exhausted = true;
            break;
        }
    }
    assert!(exhausted, "control exceeded source bound");
    let mut replay = search_support::Replay::new(&query, &[]);
    let mut retired_right = None;
    for (index, ids) in engine.trace() {
        if source[*index].name == "join"
            && let Some(retired) = retired_right
        {
            let mut stale = ids.clone();
            stale[1] = retired;
            assert!(
                replay
                    .clone()
                    .fire(*index, &source[*index], &stale)
                    .is_err()
            );
        }
        replay.fire(*index, &source[*index], ids).unwrap();
        if source[*index].name == "replace" {
            retired_right = Some(ids[1]);
        }
    }
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(engine.stats().applications as usize, engine.trace().len());
    } else {
        assert_eq!(engine.stats().source_steps, 0);
        assert_eq!(engine.stats().applications, 0);
    }
    assert!(replay.terminal(&source));
    let answer = engine.observe().unwrap();
    assert!(equivalent(&answer, &replay.answer()));
    answer
}
fn check(case: &Case) {
    let expected = case.expected();
    for mode in [
        chr_compiled::update_join::Mode::Direct,
        chr_compiled::update_join::Mode::Retained,
    ] {
        for first in [false, true] {
            let prepared = chr_compiled::update_join::Prepared::new(&rules()).unwrap();
            let mut engine = prepared.start(case.query(first), mode).unwrap();
            assert!(!engine.advance(0));
            assert!(engine.observe().is_err());
            assert!(engine.advance(2_000_000));
            assert!(equivalent(&engine.observe().unwrap(), &expected));
            if chr_compiled::COLLECT_METRICS {
                assert_eq!(
                    engine.stats().emitted as usize,
                    expected
                        .residual
                        .iter()
                        .filter(|r| r.name == "receipt")
                        .count()
                );
            } else {
                assert_eq!(engine.stats().emitted, 0);
                assert_eq!(engine.stats().constructed_pairs, 0);
            }
        }
    }
    for access in [Access::Scan, Access::Indexed] {
        for first in [false, true] {
            assert!(equivalent(
                &execute(case, Policy::Global, access, first, 4096),
                &expected
            ));
        }
    }
}
#[test]
fn global_cartesian_grid_and_adverse_empty_requests() {
    for n in [1, 2, 4, 8] {
        for groups in [1, n] {
            for rounds in [0, 1, 2, 4] {
                check(&Case::grid(n, rounds, groups));
            }
        }
    }
    let mut case = Case::grid(4, 0, 1);
    case.events
        .push(Event::Request(atom("absent"), atom("round0")));
    check(&case);
}
#[test]
fn source_mutation_bindings_duplicates_and_joint_aliases() {
    for n in [2, 4, 8] {
        for payload in [atom("r0"), atom("changed")] {
            let mut case = Case::grid(n, 1, 1);
            case.events.extend([
                Event::Replace(atom("k0"), atom("r0"), payload),
                Event::Request(atom("k0"), atom("later")),
            ]);
            check(&case);
        }
        let mut late = Case::grid(n, 1, 1);
        for (key, _) in late.left.iter_mut().chain(late.right.iter_mut()) {
            *key = v(100);
        }
        late.events.extend([
            Event::Bind(Var(100), atom("k0")),
            Event::Request(atom("k0"), atom("later")),
        ]);
        check(&late);
        let mut duplicate = Case::grid(n, 2, 1);
        duplicate.left[1] = duplicate.left[0].clone();
        duplicate.right[1] = duplicate.right[0].clone();
        check(&duplicate);
        for shared in [false, true] {
            let mut case = Case::grid(n, 2, 1);
            case.left[0].1 = v(10);
            case.right[0].1 = v(if shared { 10 } else { 11 });
            check(&case);
        }
    }
}
#[test]
fn bounded_resume_and_oracle_mutations() {
    let case = Case::grid(2, 2, 1);
    let expected = case.expected();
    assert!(equivalent(
        &execute(&case, Policy::Global, Access::Indexed, false, 1),
        &expected
    ));
    let mut missing = expected.clone();
    missing.residual.remove(0);
    assert!(!equivalent(&missing, &expected));
    let mut duplicate = expected.clone();
    duplicate.residual.push(expected.residual[0].clone());
    assert!(!equivalent(&duplicate, &expected));
    let mut premature = expected.clone();
    premature.residual.retain(|x| x.name != "receipt");
    assert!(!equivalent(&premature, &expected));
    let mut alias = Case::grid(2, 1, 1);
    alias.left[0].1 = v(10);
    alias.right[0].1 = v(11);
    let before = alias.expected();
    alias.right[0].1 = v(10);
    assert!(!equivalent(&before, &alias.expected()));
}
#[test]
fn active_policy_propagation_completion_screen() {
    let case = Case::grid(2, 1, 1);
    let expected = case.expected();
    for access in [Access::Scan, Access::Indexed] {
        let actual = execute(&case, Policy::Active, access, false, 4096);
        let count = actual
            .residual
            .iter()
            .filter(|x| x.name == "receipt")
            .count();
        assert_eq!(count, 1);
        assert!(!equivalent(&actual, &expected));
        println!(
            "Active {access:?}: receipts={count}, complete_expected={}",
            equivalent(&actual, &expected)
        );
    }
}

#[test]
fn unique_consumed_partner_preserves_kept_resource() {
    let source = vec![Rule {
        name: "consume".into(),
        kept: vec![c("left", [v(0), v(1)])],
        removed: vec![c("right", [v(0), v(2)]), c("request", [v(0), v(3)])],
        guards: vec![],
        body: and([
            c("receipt", [v(3), v(1), v(2)]).into(),
            c("done", []).into(),
        ]),
    }];
    let query = Query {
        constraints: vec![
            c("left", [atom("k"), v(10)]),
            c("right", [atom("k"), v(11)]),
            c("request", [atom("k"), atom("r")]),
        ],
        outputs: vec![],
    };
    let expected = Answer {
        outputs: vec![],
        residual: vec![
            c("left", [atom("k"), v(10)]),
            c("receipt", [atom("r"), v(10), v(11)]),
            c("done", []),
        ],
    };
    for access in [Access::Scan, Access::Indexed] {
        assert!(equivalent(
            &execute_source(source.clone(), query.clone(), Policy::Global, access, 1),
            &expected
        ));
    }
}

#[test]
fn lowerings_reject_source_changes_and_preserve_blocked_replacement() {
    use chr_compiled::update_join::{Mode, Prepared};
    let mut wrong = rules();
    wrong.swap(0, 1);
    assert!(Prepared::new(&wrong).is_err());
    let mut wrong = rules();
    wrong[0].body = chr_syntax::Goal::True;
    assert!(Prepared::new(&wrong).is_err());
    let prepared = Prepared::new(&rules()).unwrap();
    let mut blocked = Case::grid(2, 0, 1);
    blocked
        .events
        .push(Event::Replace(atom("k0"), atom("missing"), atom("new")));
    for mode in [Mode::Direct, Mode::Retained] {
        let query = blocked.query(false);
        let mut engine = prepared.start(query.clone(), mode).unwrap();
        assert!(engine.advance(100));
        assert!(equivalent(
            &engine.observe().unwrap(),
            &execute(&blocked, Policy::Global, Access::Indexed, false, 4096)
        ));
        let mut bad = query;
        bad.constraints.push(c("external", []));
        assert!(prepared.start(bad, mode).is_err());
    }
}
#[test]
fn retained_updates_are_incident_and_repeated_requests_do_not_rebuild_pairs() {
    use chr_compiled::update_join::{Mode, Prepared};
    let n = 8;
    let rounds = 4;
    let base = Case::grid(n, rounds, 1);
    let prepared = Prepared::new(&rules()).unwrap();
    for mode in [Mode::Direct, Mode::Retained] {
        let mut engine = prepared.start(base.query(false), mode).unwrap();
        assert_eq!(
            engine.retained_pairs(),
            if mode == Mode::Retained { n * n } else { 0 }
        );
        let mut steps = 0;
        while !engine.advance(1) {
            steps += 1;
            assert!(steps < 2000);
        }
        assert!(equivalent(&engine.observe().unwrap(), &base.expected()));
        if chr_compiled::COLLECT_METRICS {
            println!(
                "mode={mode:?} n={n} rounds={rounds} stats={:?}",
                engine.stats()
            );
            assert_eq!(
                engine.stats().constructed_pairs,
                if mode == Mode::Retained {
                    (n * n) as u64
                } else {
                    0
                }
            );
            assert_eq!(
                engine.stats().direct_pairs,
                if mode == Mode::Direct {
                    (n * n * rounds) as u64
                } else {
                    0
                }
            );
            assert_eq!(
                engine.stats().retained_visits,
                if mode == Mode::Retained {
                    (n * n * rounds) as u64
                } else {
                    0
                }
            );
        }
    }
    let mut changed = Case::grid(n, 1, 1);
    changed.events.extend([
        Event::Replace(atom("k0"), atom("r0"), atom("new")),
        Event::Request(atom("k0"), atom("later")),
    ]);
    let mut engine = prepared
        .start(changed.query(false), Mode::Retained)
        .unwrap();
    assert!(engine.advance(2000));
    assert!(equivalent(&engine.observe().unwrap(), &changed.expected()));
    assert_eq!(engine.retained_pairs(), n * n);
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(engine.stats().constructed_pairs, (n * n + n) as u64);
        assert_eq!(engine.stats().invalidated_pairs, n as u64);
    }
}

#[test]
fn sparse_replacement_and_broad_binding_have_distinct_maintenance() {
    use chr_compiled::update_join::{Mode, Prepared};
    let prepared = Prepared::new(&rules()).unwrap();
    let mut sparse = Case::grid(8, 1, 8);
    sparse.events.extend([
        Event::Replace(atom("k0"), atom("r0"), atom("r0")),
        Event::Request(atom("k0"), atom("again")),
    ]);
    let mut engine = prepared.start(sparse.query(false), Mode::Retained).unwrap();
    assert!(engine.advance(2000));
    assert!(equivalent(&engine.observe().unwrap(), &sparse.expected()));
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(engine.stats().constructed_pairs, 9);
        assert_eq!(engine.stats().invalidated_pairs, 1);
        println!("sparse replacement stats={:?}", engine.stats());
    }
    let mut broad = Case::grid(8, 1, 1);
    for (key, _) in broad.left.iter_mut().chain(broad.right.iter_mut()) {
        *key = v(100);
    }
    broad.events.extend([
        Event::Bind(Var(100), atom("k0")),
        Event::Request(atom("k0"), atom("later")),
    ]);
    let mut engine = prepared.start(broad.query(false), Mode::Retained).unwrap();
    assert!(engine.advance(2000));
    assert!(equivalent(&engine.observe().unwrap(), &broad.expected()));
    if chr_compiled::COLLECT_METRICS {
        assert_eq!(engine.stats().constructed_pairs, 128);
        assert_eq!(engine.stats().invalidated_pairs, 64);
        println!("broad binding stats={:?}", engine.stats());
    }
}

#[test]
fn prepared_reuse_cancellation_and_output_roots() {
    use chr_compiled::update_join::{Mode, Prepared};
    let prepared = Prepared::new(&rules()).unwrap();
    for mode in [Mode::Direct, Mode::Retained] {
        let case = Case::grid(2, 2, 1);
        let mut first = prepared.start(case.query(false), mode).unwrap();
        assert!(!first.advance(2));
        assert!(first.observe().is_err());
        drop(first);
        let mut query = case.query(false);
        query.outputs.push(("unused".into(), Var(500)));
        let mut expected = case.expected();
        expected.outputs.push(("unused".into(), v(500)));
        let mut second = prepared.start(query.clone(), mode).unwrap();
        assert!(second.advance(100));
        assert!(equivalent(&second.observe().unwrap(), &expected));
        query.outputs.push(("unused".into(), Var(501)));
        assert!(prepared.start(query, mode).is_err());
    }
}
