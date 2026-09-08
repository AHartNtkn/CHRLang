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
