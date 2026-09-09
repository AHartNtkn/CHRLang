//! Independent owned-term substitution oracle; no candidate equality operations.
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq, Eq)]
enum Tree {
    Var(usize),
    App(&'static str, Vec<Tree>),
}
fn deref(mut t: Tree, bindings: &BTreeMap<usize, Tree>) -> Tree {
    while let Tree::Var(v) = t {
        match bindings.get(&v) {
            Some(next) => t = next.clone(),
            None => break,
        }
    }
    t
}
fn occurs(v: usize, t: Tree, bindings: &BTreeMap<usize, Tree>) -> bool {
    match deref(t, bindings) {
        Tree::Var(x) => v == x,
        Tree::App(_, xs) => xs.into_iter().any(|x| occurs(v, x, bindings)),
    }
}
fn unify(a: Tree, b: Tree, bindings: &mut BTreeMap<usize, Tree>) -> bool {
    let mut pairs = vec![(a, b)];
    while let Some((a, b)) = pairs.pop() {
        let a = deref(a, bindings);
        let b = deref(b, bindings);
        if a == b {
            continue;
        }
        match (a, b) {
            (Tree::Var(v), t) | (t, Tree::Var(v)) => {
                if occurs(v, t.clone(), bindings) {
                    return false;
                }
                bindings.insert(v, t);
            }
            (Tree::App(a, xs), Tree::App(b, ys)) => {
                if a != b || xs.len() != ys.len() {
                    return false;
                }
                pairs.extend(xs.into_iter().zip(ys));
            }
        }
    }
    true
}
fn resolve(t: Tree, bindings: &BTreeMap<usize, Tree>) -> Tree {
    match deref(t, bindings) {
        Tree::Var(v) => Tree::Var(v),
        Tree::App(n, xs) => Tree::App(n, xs.into_iter().map(|x| resolve(x, bindings)).collect()),
    }
}
fn canonical(trees: Vec<Tree>) -> Vec<Tree> {
    fn walk(t: Tree, names: &mut BTreeMap<usize, usize>) -> Tree {
        match t {
            Tree::Var(v) => {
                let next = names.len();
                Tree::Var(*names.entry(v).or_insert(next))
            }
            Tree::App(n, xs) => Tree::App(n, xs.into_iter().map(|x| walk(x, names)).collect()),
        }
    }
    let mut names = BTreeMap::new();
    trees.into_iter().map(|t| walk(t, &mut names)).collect()
}

use chr_direct_conditional::equality::{DemandStatus, Store, Term, TermView, UnifyStatus};
use chr_direct_conditional::support::{Arena, Operation, Status, Support};
fn boolean(arena: &mut Arena, op: Operation) -> Support {
    let mut j = arena.job(op);
    for _ in 0..10000 {
        if let Status::Complete(s) = j.tick(arena) {
            return s;
        }
    }
    panic!("support job did not finish");
}
fn equate(store: &mut Store, arena: &mut Arena, s: Support, a: Term, b: Term) {
    let mut job = store.unify(s, a, b);
    for _ in 0..10000 {
        match job.tick(store, arena) {
            UnifyStatus::Pending => (),
            UnifyStatus::Complete { .. } => return,
            UnifyStatus::Stale => panic!("serial operation became stale"),
        }
    }
    panic!("equality job did not finish");
}
fn project(store: &Store, arena: &Arena, t: Term, world: &[bool], depth: usize) -> Tree {
    assert!(depth < 64, "live projected bindings must be finite trees");
    match store.inspect(t) {
        TermView::Variable(v) => {
            let active: Vec<_> = store
                .bindings(v)
                .iter()
                .filter(|b| arena.eval(b.support, world))
                .collect();
            assert!(
                active.len() <= 1,
                "overlapping guarded variable definitions"
            );
            match active.first() {
                None => Tree::Var(v),
                Some(b) => project(store, arena, b.term, world, depth + 1),
            }
        }
        TermView::Constructor { name, args } => {
            let n = match name {
                "a" => "a",
                "b" => "b",
                "f" => "f",
                _ => panic!("unexpected constructor"),
            };
            Tree::App(
                n,
                args.iter()
                    .map(|&x| project(store, arena, x, world, depth + 1))
                    .collect(),
            )
        }
    }
}
#[test]
fn supported_equation_pairs_match_independent_finite_tree_substitutions() {
    let source = [
        Tree::Var(0),
        Tree::Var(1),
        Tree::App("a", vec![]),
        Tree::App("b", vec![]),
        Tree::App("f", vec![Tree::Var(0)]),
        Tree::App("f", vec![Tree::Var(1)]),
    ];
    for a in 0..6 {
        for b in 0..6 {
            for c in 0..6 {
                for d in 0..6 {
                    for m in 0u8..4 {
                        for n in 0u8..4 {
                            let mut arena = Arena::new();
                            let (_, birth) = arena.fresh_variable();
                            let not = boolean(&mut arena, Operation::Not(birth));
                            let masks = [Support::FALSE, not, birth, Support::TRUE];
                            let mut store = Store::new();
                            let x = store.fresh_variable();
                            let y = store.fresh_variable();
                            let ca = store.constructor("a", vec![]);
                            let cb = store.constructor("b", vec![]);
                            let fx = store.constructor("f", vec![x]);
                            let fy = store.constructor("f", vec![y]);
                            let terms = [x, y, ca, cb, fx, fy];
                            equate(
                                &mut store,
                                &mut arena,
                                masks[m as usize],
                                terms[a],
                                terms[b],
                            );
                            equate(
                                &mut store,
                                &mut arena,
                                masks[n as usize],
                                terms[c],
                                terms[d],
                            );
                            for world in 0..2 {
                                let mut bindings = BTreeMap::new();
                                let mut valid = true;
                                for (mask, l, r) in [(m, a, b), (n, c, d)] {
                                    if valid && mask & (1 << world) != 0 {
                                        valid = unify(
                                            source[l].clone(),
                                            source[r].clone(),
                                            &mut bindings,
                                        );
                                    }
                                }
                                assert_eq!(
                                    arena.eval(store.failed(), &[world == 1]),
                                    !valid,
                                    "equations {a},{b}/{c},{d}, masks {m}/{n}, world{world}"
                                );
                                if valid {
                                    let expected = canonical(vec![
                                        resolve(Tree::Var(0), &bindings),
                                        resolve(Tree::Var(1), &bindings),
                                    ]);
                                    let actual = canonical(vec![
                                        project(&store, &arena, x, &[world == 1], 0),
                                        project(&store, &arena, y, &[world == 1], 0),
                                    ]);
                                    assert_eq!(
                                        actual, expected,
                                        "equations {a},{b}/{c},{d}, masks {m}/{n}, world{world}"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[test]
fn equality_demand_is_nonbinding_and_reports_support_expansion_dependencies() {
    let mut arena = Arena::new();
    let (_, birth) = arena.fresh_variable();
    let mut store = Store::new();
    let x = store.fresh_variable();
    let a = store.constructor("a", vec![]);
    let initial = store.version();
    let mut query = store.entails(Support::TRUE, x, a);
    let mut result = None;
    for _ in 0..1000 {
        match query.tick(&store, &mut arena) {
            DemandStatus::Pending => (),
            DemandStatus::Complete { entailed } => {
                result = Some(entailed);
                break;
            }
            DemandStatus::Stale => panic!(),
        }
    }
    assert_eq!(result, Some(Support::FALSE));
    assert_eq!(store.version(), initial);
    assert!(!query.dependencies().is_empty());
    equate(&mut store, &mut arena, birth, x, a);
    let mut query = store.entails(Support::TRUE, x, a);
    let mut result = None;
    for _ in 0..1000 {
        match query.tick(&store, &mut arena) {
            DemandStatus::Pending => (),
            DemandStatus::Complete { entailed } => {
                result = Some(entailed);
                break;
            }
            DemandStatus::Stale => panic!(),
        }
    }
    assert_eq!(result, Some(birth));
    let previous = store.changes().len();
    equate(&mut store, &mut arena, Support::TRUE, x, a);
    assert!(
        store.changes().len() > previous,
        "same value on more support must notify"
    );
    let mut query = store.entails(Support::TRUE, x, a);
    for _ in 0..1000 {
        match query.tick(&store, &mut arena) {
            DemandStatus::Pending => (),
            DemandStatus::Complete { entailed } => {
                assert_eq!(entailed, Support::TRUE);
                return;
            }
            DemandStatus::Stale => panic!(),
        }
    }
    panic!("demand did not finish");
}
#[test]
fn alias_dependencies_intersect_later_binding_and_new_support_notifications() {
    let mut arena = Arena::new();
    let (_, birth) = arena.fresh_variable();
    let mut store = Store::new();
    let x = store.fresh_variable();
    let y = store.fresh_variable();
    let a = store.constructor("a", vec![]);
    equate(&mut store, &mut arena, Support::TRUE, x, y);
    let mut demand = store.entails(Support::TRUE, x, a);
    let mut finished = false;
    for _ in 0..1000 {
        match demand.tick(&store, &mut arena) {
            DemandStatus::Pending => (),
            DemandStatus::Complete { entailed } => {
                assert_eq!(entailed, Support::FALSE);
                finished = true;
                break;
            }
            DemandStatus::Stale => panic!(),
        }
    }
    assert!(finished);
    let dependencies: Vec<_> = demand.dependencies().iter().map(|c| c.variable).collect();
    let before = store.changes().len();
    equate(&mut store, &mut arena, birth, y, a);
    assert!(
        store.changes()[before..]
            .iter()
            .any(|c| dependencies.contains(&c.variable) && arena.eval(c.support, &[true]))
    );
    let mut demand = store.entails(Support::TRUE, x, a);
    let mut finished = false;
    for _ in 0..1000 {
        match demand.tick(&store, &mut arena) {
            DemandStatus::Pending => (),
            DemandStatus::Complete { entailed } => {
                assert_eq!(entailed, birth);
                finished = true;
                break;
            }
            DemandStatus::Stale => panic!(),
        }
    }
    assert!(finished);
    let before = store.changes().len();
    equate(&mut store, &mut arena, Support::TRUE, y, a);
    assert!(
        store.changes()[before..]
            .iter()
            .any(|c| dependencies.contains(&c.variable) && arena.eval(c.support, &[false]))
    );
}
#[test]
fn stale_writer_must_restart_and_cannot_commit_from_old_binding_state() {
    let mut arena = Arena::new();
    let mut store = Store::new();
    let x = store.fresh_variable();
    let a = store.constructor("a", vec![]);
    let b = store.constructor("b", vec![]);
    let mut stale = store.unify(Support::TRUE, x, b);
    equate(&mut store, &mut arena, Support::TRUE, x, a);
    assert!(matches!(
        stale.tick(&mut store, &mut arena),
        UnifyStatus::Stale
    ));
    assert_eq!(store.failed(), Support::FALSE);
    equate(&mut store, &mut arena, Support::TRUE, x, b);
    assert_eq!(store.failed(), Support::TRUE);
}
#[test]
fn partial_commit_notifications_survive_interruption_and_restart() {
    let mut arena = Arena::new();
    let mut store = Store::new();
    let x = store.fresh_variable();
    let y = store.fresh_variable();
    let a = store.constructor("a", vec![]);
    let b = store.constructor("b", vec![]);
    let pair = store.constructor("pair", vec![x, y]);
    let values = store.constructor("pair", vec![a, b]);
    let mut partial = store.unify(Support::TRUE, pair, values);
    let mut interrupted = false;
    for _ in 0..10000 {
        let result = partial.tick(&mut store, &mut arena);
        if !store.changes().is_empty() && matches!(result, UnifyStatus::Pending) {
            interrupted = true;
            break;
        }
    }
    assert!(
        interrupted,
        "must interrupt after a committed prefix, before completion"
    );
    drop(partial);
    let committed = store.changes().len();
    assert!(committed > 0);
    equate(&mut store, &mut arena, Support::TRUE, pair, values);
    assert!(store.changes().len() >= committed);
    assert_eq!(project(&store, &arena, x, &[], 0), Tree::App("a", vec![]));
    assert_eq!(project(&store, &arena, y, &[], 0), Tree::App("b", vec![]));
}
#[test]
fn entails_matches_resolved_tree_equality_without_adding_bindings() {
    let source = [
        Tree::Var(0),
        Tree::Var(1),
        Tree::App("a", vec![]),
        Tree::App("b", vec![]),
        Tree::App("f", vec![Tree::Var(0)]),
        Tree::App("f", vec![Tree::Var(1)]),
    ];
    for a in 0..6 {
        for b in 0..6 {
            for mask in 0u8..4 {
                let mut arena = Arena::new();
                let (_, birth) = arena.fresh_variable();
                let not = boolean(&mut arena, Operation::Not(birth));
                let masks = [Support::FALSE, not, birth, Support::TRUE];
                let mut store = Store::new();
                let x = store.fresh_variable();
                let y = store.fresh_variable();
                let ca = store.constructor("a", vec![]);
                let cb = store.constructor("b", vec![]);
                let fx = store.constructor("f", vec![x]);
                let fy = store.constructor("f", vec![y]);
                let terms = [x, y, ca, cb, fx, fy];
                equate(
                    &mut store,
                    &mut arena,
                    masks[mask as usize],
                    terms[a],
                    terms[b],
                );
                let states: Vec<_> = (0..2)
                    .map(|world| {
                        let mut bindings = BTreeMap::new();
                        let valid = mask & (1 << world) == 0
                            || unify(source[a].clone(), source[b].clone(), &mut bindings);
                        (valid, bindings)
                    })
                    .collect();
                let version = store.version();
                let changes = store.changes().len();
                for l in 0..6 {
                    for r in 0..6 {
                        let mut query = store.entails(Support::TRUE, terms[l], terms[r]);
                        let mut done = false;
                        for _ in 0..10000 {
                            match query.tick(&store, &mut arena) {
                                DemandStatus::Pending => (),
                                DemandStatus::Complete { entailed } => {
                                    for (world, (valid, bindings)) in states.iter().enumerate() {
                                        let expected = *valid
                                            && resolve(source[l].clone(), bindings)
                                                == resolve(source[r].clone(), bindings);
                                        assert_eq!(
                                            arena.eval(entailed, &[world == 1]),
                                            expected,
                                            "bind{a},{b}@{mask}, demand{l},{r}, world{world}"
                                        );
                                    }
                                    done = true;
                                    break;
                                }
                                DemandStatus::Stale => panic!(),
                            }
                        }
                        assert!(done);
                        assert_eq!(store.version(), version);
                        assert_eq!(store.changes().len(), changes);
                    }
                }
            }
        }
    }
}
#[test]
fn three_edge_cycle_fails_only_on_intersection_and_field_clash_is_local() {
    let mut arena = Arena::new();
    let (_, b) = arena.fresh_variable();
    let (_, c) = arena.fresh_variable();
    let mut store = Store::new();
    let x = store.fresh_variable();
    let y = store.fresh_variable();
    let z = store.fresh_variable();
    let fy = store.constructor("f", vec![y]);
    equate(&mut store, &mut arena, b, x, fy);
    equate(&mut store, &mut arena, c, y, z);
    equate(&mut store, &mut arena, Support::TRUE, z, x);
    for bv in [false, true] {
        for cv in [false, true] {
            assert_eq!(arena.eval(store.failed(), &[bv, cv]), bv && cv);
        }
    }
    let mut store = Store::new();
    let x = store.fresh_variable();
    let a = store.constructor("a", vec![]);
    let value = store.constructor("b", vec![]);
    let left = store.constructor("pair", vec![x, a]);
    let right = store.constructor("pair", vec![value, value]);
    equate(&mut store, &mut arena, b, left, right);
    assert_eq!(store.failed(), b);
    assert_eq!(project(&store, &arena, x, &[false, false], 0), Tree::Var(0));
}

#[test]
fn mutation_during_deep_occurs_continuation_rejects_stale_effects() {
    let mut arena = Arena::new();
    let mut store = Store::new();
    let x = store.fresh_variable();
    let y = store.fresh_variable();
    let mut nested = y;
    for _ in 0..128 {
        nested = store.constructor("f", vec![nested]);
    }
    let mut paused = store.unify(Support::TRUE, x, nested);
    for _ in 0..100 {
        assert!(matches!(
            paused.tick(&mut store, &mut arena),
            UnifyStatus::Pending
        ));
    }
    assert!(
        store.changes().is_empty(),
        "pause during traversal before binding commit"
    );
    equate(&mut store, &mut arena, Support::TRUE, y, x);
    let changed = store.changes().len();
    let version = store.version();
    assert!(matches!(
        paused.tick(&mut store, &mut arena),
        UnifyStatus::Stale
    ));
    assert_eq!(store.changes().len(), changed);
    assert_eq!(store.version(), version);
    assert_eq!(store.failed(), Support::FALSE);
    equate(&mut store, &mut arena, Support::TRUE, x, nested);
    assert_eq!(store.failed(), Support::TRUE);
}

#[test]
fn unfinished_equality_jobs_reject_a_different_store_with_colliding_handles() {
    let mut arena = Arena::new();
    let mut first = Store::new();
    let x = first.fresh_variable();
    let a = first.constructor("a", vec![]);
    let mut second = Store::new();
    second.fresh_variable();
    second.constructor("b", vec![]);
    assert_eq!(first.version(), second.version());
    let mut writer = first.unify(Support::TRUE, x, a);
    assert!(matches!(
        writer.tick(&mut second, &mut arena),
        UnifyStatus::Stale
    ));
    assert!(second.changes().is_empty());
    assert_eq!(second.failed(), Support::FALSE);
    let mut demand = first.entails(Support::TRUE, x, x);
    assert!(matches!(
        demand.tick(&second, &mut arena),
        DemandStatus::Stale
    ));
}
