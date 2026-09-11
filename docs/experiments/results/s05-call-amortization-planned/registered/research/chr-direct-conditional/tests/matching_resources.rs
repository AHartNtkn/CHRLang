//! Independent projected nonbinding matching and source-resource expectations.
use chr_direct_conditional::engine::PreparedRuleset;
use chr_syntax::{Term as Source, Var};
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq, Eq)]
enum Tree {
    Free(usize),
    App(String, Vec<Tree>),
}
fn source_match(pattern: &Source, value: &Tree, slots: &mut BTreeMap<u64, Tree>) -> bool {
    match pattern {
        Source::Var(Var(v)) => match slots.get(v) {
            Some(previous) => previous == value,
            None => {
                slots.insert(*v, value.clone());
                true
            }
        },
        Source::App(name, args) => match value {
            Tree::Free(_) => false,
            Tree::App(other, fields) => {
                name == other
                    && args.len() == fields.len()
                    && args
                        .iter()
                        .zip(fields)
                        .all(|(p, t)| source_match(p, t, slots))
            }
        },
    }
}
use chr_direct_conditional::{
    equality::{Store, Term, TermView, UnifyStatus},
    support::{Arena, Operation, Status, Support},
};
fn support(arena: &mut Arena, op: Operation) -> Support {
    let mut job = arena.job(op);
    for _ in 0..10000 {
        if let Status::Complete(s) = job.tick(arena) {
            return s;
        }
    }
    panic!("support cutoff")
}
fn equate(store: &mut Store, arena: &mut Arena, s: Support, a: Term, b: Term) {
    let mut job = store.unify(s, a, b);
    for _ in 0..10000 {
        match job.tick(store, arena) {
            UnifyStatus::Pending => (),
            UnifyStatus::Complete { .. } => return,
            UnifyStatus::Stale => panic!(),
        }
    }
    panic!("equality cutoff")
}
fn project(store: &Store, arena: &Arena, term: Term, world: &[bool], depth: usize) -> Tree {
    assert!(depth < 64);
    match store.inspect(term) {
        TermView::Variable(v) => {
            let mut active = store
                .bindings(v)
                .iter()
                .filter(|b| arena.eval(b.support, world));
            match active.next() {
                Some(binding) => {
                    assert!(active.next().is_none());
                    project(store, arena, binding.term, world, depth + 1)
                }
                None => Tree::Free(v),
            }
        }
        TermView::Constructor { name, args } => Tree::App(
            name.into(),
            args.iter()
                .map(|&t| project(store, arena, t, world, depth + 1))
                .collect(),
        ),
    }
}
use chr_direct_conditional::matching::{MatchStatus, Matched, Pattern, Plan};
use chr_syntax::{atom, t, v};
fn pattern(source: &Source) -> Pattern {
    match source {
        Source::Var(Var(v)) => Pattern::Slot(*v as usize),
        Source::App(name, args) => Pattern::Constructor {
            name: name.clone(),
            args: args.iter().map(pattern).collect(),
        },
    }
}
fn matches(plan: &Plan, store: &Store, arena: &mut Arena, terms: Vec<Term>) -> Vec<Matched> {
    let mut job = plan.start(store, Support::TRUE, terms).unwrap();
    let mut frames = vec![];
    for _ in 0..10000 {
        match job.tick(store, arena) {
            MatchStatus::Pending => (),
            MatchStatus::Match(frame) => frames.push(frame),
            MatchStatus::Done => return frames,
            MatchStatus::Stale => panic!("unchanged matching became stale"),
        }
    }
    panic!("matching cutoff");
}
#[test]
fn symbolic_matching_matches_independent_joint_slot_environments() {
    let patterns = [
        v(0),
        v(1),
        atom("a"),
        atom("b"),
        t("f", [v(0)]),
        t("pair", [v(0), v(0)]),
        t("pair", [v(0), v(1)]),
    ];
    for state in 0..6 {
        let mut arena = Arena::new();
        let (_, birth) = arena.fresh_variable();
        let not = support(&mut arena, Operation::Not(birth));
        let mut store = Store::new();
        let x = store.fresh_variable();
        let y = store.fresh_variable();
        let a = store.constructor("a", vec![]);
        let b = store.constructor("b", vec![]);
        let fx = store.constructor("f", vec![x]);
        let fy = store.constructor("f", vec![y]);
        let pxx = store.constructor("pair", vec![x, x]);
        let pxy = store.constructor("pair", vec![x, y]);
        match state {
            0 => (),
            1 => equate(&mut store, &mut arena, birth, x, a),
            2 => {
                equate(&mut store, &mut arena, birth, x, fy);
                equate(&mut store, &mut arena, not, x, b)
            }
            3 => equate(&mut store, &mut arena, Support::TRUE, x, y),
            4 => {
                equate(&mut store, &mut arena, birth, y, a);
                equate(&mut store, &mut arena, not, x, y)
            }
            5 => {
                equate(&mut store, &mut arena, Support::TRUE, x, fy);
                equate(&mut store, &mut arena, birth, y, a)
            }
            _ => unreachable!(),
        }
        let terms = [x, y, a, b, fx, pxx, pxy];
        let version = store.version();
        let changes = store.changes().len();
        for p in &patterns {
            for q in &patterns {
                let plan = Plan::new(vec![pattern(p), pattern(q)], 2).unwrap();
                for &left in &terms {
                    for &right in &terms {
                        let frames = matches(&plan, &store, &mut arena, vec![left, right]);
                        for world in [false, true] {
                            let mut expected = BTreeMap::new();
                            let accepts = source_match(
                                p,
                                &project(&store, &arena, left, &[world], 0),
                                &mut expected,
                            ) && source_match(
                                q,
                                &project(&store, &arena, right, &[world], 0),
                                &mut expected,
                            );
                            let active: Vec<_> = frames
                                .iter()
                                .filter(|f| arena.eval(f.support, &[world]))
                                .collect();
                            assert_eq!(
                                active.len(),
                                usize::from(accepts),
                                "state{state},patterns{p:?}/{q:?}"
                            );
                            if let Some(frame) = active.first() {
                                for slot in 0..2 {
                                    let actual = frame.slots[slot]
                                        .map(|term| project(&store, &arena, term, &[world], 0));
                                    assert_eq!(actual, expected.get(&(slot as u64)).cloned());
                                }
                            }
                        }
                        assert_eq!(store.version(), version);
                        assert_eq!(store.changes().len(), changes);
                    }
                }
            }
        }
    }
}
#[test]
fn opaque_slot_does_not_partition_conditional_values() {
    let mut arena = Arena::new();
    let (_, birth) = arena.fresh_variable();
    let not = support(&mut arena, Operation::Not(birth));
    let mut store = Store::new();
    let x = store.fresh_variable();
    let a = store.constructor("a", vec![]);
    let b = store.constructor("b", vec![]);
    equate(&mut store, &mut arena, birth, x, a);
    equate(&mut store, &mut arena, not, x, b);
    let plan = Plan::new(vec![Pattern::Slot(0)], 1).unwrap();
    let frames = matches(&plan, &store, &mut arena, vec![x]);
    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0].support, Support::TRUE);
    assert_eq!(frames[0].slots[0], Some(x));
}
use chr_direct_conditional::resources::{AckStatus, ApplicationStatus, Resources, Token};
use chr_syntax::{Goal, Guard, Rule, c};
fn authorize(
    resources: &Resources,
    store: &Store,
    arena: &mut Arena,
    rule: usize,
    ids: Vec<u64>,
    scope: Support,
) -> Option<Token> {
    let mut job = resources.prepare(rule, ids, scope, store).unwrap();
    for _ in 0..10000 {
        match job.tick(resources, store, arena) {
            ApplicationStatus::Pending => (),
            ApplicationStatus::Ready(token) => return Some(token),
            ApplicationStatus::Ineligible => return None,
            ApplicationStatus::Stale => panic!("unchanged application stale"),
            ApplicationStatus::Finished => panic!("application finished without an outcome"),
        }
    }
    panic!("application cutoff");
}
fn acknowledge_true(resources: &mut Resources, arena: &mut Arena, id: u64) {
    let mut ack = resources.begin_acknowledge_body(id).unwrap();
    for _ in 0..10000 {
        match ack.tick(resources, arena) {
            AckStatus::Pending => (),
            AckStatus::Ready(token) => {
                resources.acknowledge_body(token).unwrap();
                return;
            }
            AckStatus::Stale => panic!(),
        }
    }
    panic!("acknowledgment cutoff");
}
#[test]
fn propagation_resource_truth_masks_preserve_exact_effects_and_pending_body() {
    for live_a in 0u8..4 {
        for live_b in 0u8..4 {
            for matched in 0u8..4 {
                for failed in 0u8..4 {
                    for prior in 0u8..4 {
                        let mut arena = Arena::new();
                        let (_, birth) = arena.fresh_variable();
                        let not = support(&mut arena, Operation::Not(birth));
                        let masks = [Support::FALSE, not, birth, Support::TRUE];
                        let mut store = Store::new();
                        let x = store.fresh_variable();
                        let y = store.fresh_variable();
                        let a = store.constructor("a", vec![]);
                        let b = store.constructor("b", vec![]);
                        equate(&mut store, &mut arena, masks[matched as usize], x, a);
                        let rest = support(&mut arena, Operation::Not(masks[matched as usize]));
                        equate(&mut store, &mut arena, rest, x, b);
                        equate(&mut store, &mut arena, masks[failed as usize], a, b);
                        let mut rule =
                            Rule::propagate("seen", [c("p", [v(0)]), c("q", [v(1)])], Goal::True);
                        rule.guards = vec![Guard::Equal(v(0), atom("a"))];
                        let mut resources =
                            Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
                        let p = resources
                            .insert("p", vec![x], masks[live_a as usize], &store)
                            .unwrap();
                        let q = resources
                            .insert("q", vec![y], masks[live_b as usize], &store)
                            .unwrap();
                        if let Some(token) = authorize(
                            &resources,
                            &store,
                            &mut arena,
                            0,
                            vec![p, q],
                            masks[prior as usize],
                        ) {
                            let id = resources.commit(token, &store).unwrap();
                            acknowledge_true(&mut resources, &mut arena, id);
                        }
                        let expected = live_a & live_b & matched & !failed & !prior & 3;
                        let token =
                            authorize(&resources, &store, &mut arena, 0, vec![p, q], Support::TRUE);
                        assert_eq!(token.is_some(), expected != 0);
                        if let Some(token) = token {
                            let id = resources.commit(token, &store).unwrap();
                            let body = resources.pending_bodies().get(&id).unwrap();
                            for world in 0..2 {
                                assert_eq!(
                                    arena.eval(body.support, &[world == 1]),
                                    expected & (1 << world) != 0
                                );
                            }
                            assert_eq!(resources.pending_bodies().len(), 1);
                        }
                        for world in 0..2 {
                            assert_eq!(
                                arena.eval(resources.occurrence(p).unwrap().live, &[world == 1]),
                                live_a & (1 << world) != 0
                            );
                            assert_eq!(
                                arena.eval(resources.occurrence(q).unwrap().live, &[world == 1]),
                                live_b & (1 << world) != 0
                            );
                            let history = resources
                                .history()
                                .get(&(0, vec![p, q]))
                                .copied()
                                .unwrap_or(Support::FALSE);
                            assert_eq!(
                                arena.eval(history, &[world == 1]),
                                (live_a & live_b & matched & !failed) & (1 << world) != 0
                            );
                        }
                    }
                }
            }
        }
    }
}
#[test]
fn two_birth_consumption_intersects_crossing_supports() {
    for a in 0u8..16 {
        for b in 0u8..16 {
            let mut arena = Arena::new();
            arena.fresh_variable();
            arena.fresh_variable();
            let mut mask = |bits: u8| {
                let leaf = |i: usize| {
                    if bits & (1u8 << i) != 0u8 {
                        Support::TRUE
                    } else {
                        Support::FALSE
                    }
                };
                if cfg!(feature = "support-reverse-order") {
                    let lo = arena.mk(0, leaf(0), leaf(1));
                    let hi = arena.mk(0, leaf(2), leaf(3));
                    arena.mk(1, lo, hi)
                } else {
                    let lo = arena.mk(1, leaf(0), leaf(2));
                    let hi = arena.mk(1, leaf(1), leaf(3));
                    arena.mk(0, lo, hi)
                }
            };
            let sa = mask(a);
            let sb = mask(b);
            let store = Store::new();
            let rule = Rule::simplify("consume", [c("p", []), c("q", [])], Goal::True);
            let mut resources = Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
            let p = resources.insert("p", vec![], sa, &store).unwrap();
            let q = resources.insert("q", vec![], sb, &store).unwrap();
            let token = authorize(&resources, &store, &mut arena, 0, vec![p, q], Support::TRUE);
            assert_eq!(token.is_some(), a & b != 0);
            if let Some(token) = token {
                let id = resources.commit(token, &store).unwrap();
                assert!(resources.pending_bodies().contains_key(&id));
            }
            assert!(resources.history().is_empty());
            for world in 0..4 {
                let values = [world & 1 != 0, world & 2 != 0];
                assert_eq!(
                    arena.eval(resources.occurrence(p).unwrap().live, &values),
                    (a & !b) & (1 << world) != 0
                );
                assert_eq!(
                    arena.eval(resources.occurrence(q).unwrap().live, &values),
                    (b & !a) & (1 << world) != 0
                );
            }
        }
    }
}
#[test]
fn tokens_reject_foreign_and_stale_owners_without_partial_effects() {
    let mut arena = Arena::new();
    let mut store = Store::new();
    let x = store.fresh_variable();
    let a = store.constructor("a", vec![]);
    let rule = Rule::propagate("seen", [c("p", [v(0)])], Goal::True);
    let mut first = Resources::new(&PreparedRuleset::new(vec![rule.clone()]).unwrap(), &store);
    let p = first.insert("p", vec![x], Support::TRUE, &store).unwrap();
    let mut second = Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
    second.insert("p", vec![x], Support::TRUE, &store).unwrap();
    let foreign = authorize(&first, &store, &mut arena, 0, vec![p], Support::TRUE).unwrap();
    assert!(second.commit(foreign, &store).is_err());
    assert!(second.pending_bodies().is_empty());
    assert!(second.history().is_empty());
    let stale = authorize(&first, &store, &mut arena, 0, vec![p], Support::TRUE).unwrap();
    equate(&mut store, &mut arena, Support::TRUE, x, a);
    assert!(first.commit(stale, &store).is_err());
    assert!(first.pending_bodies().is_empty());
    assert!(first.history().is_empty());
    let stale = authorize(&first, &store, &mut arena, 0, vec![p], Support::TRUE).unwrap();
    first
        .insert("unrelated", vec![], Support::TRUE, &store)
        .unwrap();
    assert!(first.commit(stale, &store).is_err());
    assert_eq!(first.occurrence(p).unwrap().live, Support::TRUE);
    assert!(first.pending_bodies().is_empty());
    assert!(first.history().is_empty());
}
#[test]
fn occurrence_identity_and_body_readiness_are_preserved() {
    let mut arena = Arena::new();
    let store = Store::new();
    let rule = Rule::simplify("two", [c("p", []), c("p", [])], Goal::True);
    let mut resources = Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
    let first = resources
        .insert("p", vec![], Support::TRUE, &store)
        .unwrap();
    if let Ok(mut job) = resources.prepare(0, vec![first, first], Support::TRUE, &store) {
        let mut rejected = false;
        for _ in 0..1000 {
            match job.tick(&resources, &store, &mut arena) {
                ApplicationStatus::Ineligible => {
                    rejected = true;
                    break;
                }
                ApplicationStatus::Pending => (),
                _ => panic!("one occurrence cannot occupy two heads"),
            }
        }
        assert!(rejected, "duplicate tuple did not terminate");
    }
    let second = resources
        .insert("p", vec![], Support::TRUE, &store)
        .unwrap();
    assert_ne!(first, second);
    let token = authorize(
        &resources,
        &store,
        &mut arena,
        0,
        vec![first, second],
        Support::TRUE,
    )
    .unwrap();
    let body = resources.commit(token, &store).unwrap();
    assert_eq!(resources.pending_bodies().len(), 1);
    assert_eq!(resources.occurrence(first).unwrap().live, Support::FALSE);
    assert_eq!(resources.occurrence(second).unwrap().live, Support::FALSE);
    assert!(resources.pending_bodies().contains_key(&body));
    let rule = Rule::propagate("one", [c("p", [])], Goal::True);
    let mut resources = Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
    let first = resources
        .insert("p", vec![], Support::TRUE, &store)
        .unwrap();
    let second = resources
        .insert("p", vec![], Support::TRUE, &store)
        .unwrap();
    let token = authorize(
        &resources,
        &store,
        &mut arena,
        0,
        vec![first],
        Support::TRUE,
    )
    .unwrap();
    let body1 = resources.commit(token, &store).unwrap();
    assert!(
        authorize(
            &resources,
            &store,
            &mut arena,
            0,
            vec![second],
            Support::TRUE
        )
        .is_none(),
        "unfinished body blocks same-support source selection"
    );
    acknowledge_true(&mut resources, &mut arena, body1);
    let token = authorize(
        &resources,
        &store,
        &mut arena,
        0,
        vec![second],
        Support::TRUE,
    )
    .unwrap();
    let body2 = resources.commit(token, &store).unwrap();
    assert_ne!(body1, body2);
    assert_eq!(resources.history().len(), 2);
}
#[test]
fn simpagation_keeps_only_declared_kept_occurrence() {
    let mut arena = Arena::new();
    let store = Store::new();
    let rule = Rule {
        name: "mixed".into(),
        kept: vec![c("keep", [])],
        removed: vec![c("take", [])],
        guards: vec![],
        body: Goal::True,
    };
    let mut resources = Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
    let kept = resources
        .insert("keep", vec![], Support::TRUE, &store)
        .unwrap();
    let taken = resources
        .insert("take", vec![], Support::TRUE, &store)
        .unwrap();
    let token = authorize(
        &resources,
        &store,
        &mut arena,
        0,
        vec![kept, taken],
        Support::TRUE,
    )
    .unwrap();
    resources.commit(token, &store).unwrap();
    assert_eq!(resources.occurrence(kept).unwrap().live, Support::TRUE);
    assert_eq!(resources.occurrence(taken).unwrap().live, Support::FALSE);
    assert!(resources.history().is_empty());
}
#[test]
fn guard_locals_are_rigid_fresh_identities_without_head_bindings() {
    for (left, right, expected) in [
        (v(9), v(9), true),
        (t("f", [v(9)]), t("f", [v(9)]), true),
        (v(9), v(10), false),
        (v(9), atom("a"), false),
        (t("f", [v(9)]), t("f", [v(10)]), false),
    ] {
        let mut arena = Arena::new();
        let store = Store::new();
        let mut rule = Rule::propagate("local", [c("p", [])], Goal::True);
        rule.guards = vec![Guard::Equal(left, right)];
        let mut resources = Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
        let p = resources
            .insert("p", vec![], Support::TRUE, &store)
            .unwrap();
        let result = authorize(&resources, &store, &mut arena, 0, vec![p], Support::TRUE);
        assert_eq!(result.is_some(), expected);
    }
    let mut arena = Arena::new();
    let mut store = Store::new();
    store.fresh_variable();
    let query = store.fresh_variable();
    let mut rule = Rule::propagate("namespaces", [c("p", [v(0)])], Goal::True);
    rule.guards = vec![Guard::Equal(v(0), v(1))];
    let mut resources = Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
    let p = resources
        .insert("p", vec![query], Support::TRUE, &store)
        .unwrap();
    assert!(authorize(&resources, &store, &mut arena, 0, vec![p], Support::TRUE).is_none());
}
#[test]
fn propagation_keys_preserve_ordered_ids_and_pending_support_is_local() {
    let mut arena = Arena::new();
    let (_, birth) = arena.fresh_variable();
    let not = support(&mut arena, Operation::Not(birth));
    let store = Store::new();
    let rule = Rule::propagate("ordered", [c("p", []), c("p", [])], Goal::True);
    let mut resources = Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
    let a = resources
        .insert("p", vec![], Support::TRUE, &store)
        .unwrap();
    let b = resources
        .insert("p", vec![], Support::TRUE, &store)
        .unwrap();
    let token = authorize(&resources, &store, &mut arena, 0, vec![a, b], Support::TRUE).unwrap();
    let first = resources.commit(token, &store).unwrap();
    acknowledge_true(&mut resources, &mut arena, first);
    let token = authorize(&resources, &store, &mut arena, 0, vec![b, a], Support::TRUE).unwrap();
    let second = resources.commit(token, &store).unwrap();
    assert_ne!(first, second);
    assert_eq!(resources.history().len(), 2);
    acknowledge_true(&mut resources, &mut arena, second);
    assert!(authorize(&resources, &store, &mut arena, 0, vec![a, b], Support::TRUE).is_none());
    let rule = Rule::propagate("local", [c("p", [])], Goal::True);
    let mut resources = Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
    let a = resources
        .insert("p", vec![], Support::TRUE, &store)
        .unwrap();
    let b = resources
        .insert("p", vec![], Support::TRUE, &store)
        .unwrap();
    let token = authorize(&resources, &store, &mut arena, 0, vec![a], birth).unwrap();
    let first = resources.commit(token, &store).unwrap();
    assert_eq!(resources.pending_bodies()[&first].support, birth);
    let token = authorize(&resources, &store, &mut arena, 0, vec![b], Support::TRUE).unwrap();
    let second = resources.commit(token, &store).unwrap();
    assert_eq!(resources.pending_bodies()[&second].support, not);
    assert_eq!(resources.pending_bodies().len(), 2);
}
#[test]
fn acknowledgement_keeps_other_pending_support_and_cancelled_prepare_has_no_effect() {
    let mut arena = Arena::new();
    let (_, birth) = arena.fresh_variable();
    let not = support(&mut arena, Operation::Not(birth));
    let store = Store::new();
    let rule = Rule::propagate("pending", [c("p", [])], Goal::True);
    let mut resources = Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
    let a = resources
        .insert("p", vec![], Support::TRUE, &store)
        .unwrap();
    let b = resources
        .insert("p", vec![], Support::TRUE, &store)
        .unwrap();
    let c = resources
        .insert("p", vec![], Support::TRUE, &store)
        .unwrap();
    let mut cancelled = resources
        .prepare(0, vec![a], Support::TRUE, &store)
        .unwrap();
    for _ in 0..3 {
        let _ = cancelled.tick(&resources, &store, &mut arena);
    }
    drop(cancelled);
    assert!(resources.pending_bodies().is_empty());
    assert!(resources.history().is_empty());
    assert_eq!(resources.occurrence(a).unwrap().live, Support::TRUE);
    let token = authorize(&resources, &store, &mut arena, 0, vec![a], birth).unwrap();
    let first = resources.commit(token, &store).unwrap();
    let token = authorize(&resources, &store, &mut arena, 0, vec![b], not).unwrap();
    let second = resources.commit(token, &store).unwrap();
    acknowledge_true(&mut resources, &mut arena, first);
    assert!(resources.pending_bodies().contains_key(&second));
    let token = authorize(&resources, &store, &mut arena, 0, vec![c], Support::TRUE).unwrap();
    let third = resources.commit(token, &store).unwrap();
    assert_eq!(resources.pending_bodies()[&third].support, birth);
}
#[test]
fn one_frame_completion_is_distinct_from_exhaustive_ineligibility() {
    let mut arena = Arena::new();
    let (_, birth) = arena.fresh_variable();
    let not = support(&mut arena, Operation::Not(birth));
    let mut store = Store::new();
    let x = store.fresh_variable();
    let a = store.constructor("a", vec![]);
    let b = store.constructor("b", vec![]);
    let fa = store.constructor("f", vec![a]);
    let fb = store.constructor("f", vec![b]);
    equate(&mut store, &mut arena, birth, x, fa);
    equate(&mut store, &mut arena, not, x, fb);
    let rule = Rule::propagate("frames", [c("p", [t("f", [v(0)])])], Goal::True);
    let mut resources = Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
    let p = resources
        .insert("p", vec![x], Support::TRUE, &store)
        .unwrap();
    let mut job = resources
        .prepare(0, vec![p], Support::TRUE, &store)
        .unwrap();
    let mut ready = None;
    for _ in 0..10000 {
        match job.tick(&resources, &store, &mut arena) {
            ApplicationStatus::Pending => (),
            ApplicationStatus::Ready(token) => {
                ready = Some(token);
                break;
            }
            _ => panic!("expected a first frame"),
        }
    }
    let token = ready.expect("first frame did not arrive");
    assert!(matches!(
        job.tick(&resources, &store, &mut arena),
        ApplicationStatus::Finished
    ));
    let first = resources.commit(token, &store).unwrap();
    let first_support = resources.pending_bodies()[&first].support;
    assert_ne!(first_support, Support::TRUE);
    acknowledge_true(&mut resources, &mut arena, first);
    let token = authorize(&resources, &store, &mut arena, 0, vec![p], Support::TRUE).unwrap();
    let second = resources.commit(token, &store).unwrap();
    let second_support = resources.pending_bodies()[&second].support;
    assert_eq!(
        support(&mut arena, Operation::And(first_support, second_support)),
        Support::FALSE
    );
    assert_eq!(
        support(&mut arena, Operation::Or(first_support, second_support)),
        Support::TRUE
    );
    acknowledge_true(&mut resources, &mut arena, second);
    assert!(authorize(&resources, &store, &mut arena, 0, vec![p], Support::TRUE).is_none());
}

#[test]
fn rejecting_a_guard_frame_does_not_discard_an_eligible_alternative() {
    let mut arena = Arena::new();
    let (_, birth) = arena.fresh_variable();
    let not = support(&mut arena, Operation::Not(birth));
    let mut store = Store::new();
    let x = store.fresh_variable();
    let a = store.constructor("a", vec![]);
    let b = store.constructor("b", vec![]);
    let fa = store.constructor("f", vec![a]);
    let fb = store.constructor("f", vec![b]);
    equate(&mut store, &mut arena, birth, x, fb);
    equate(&mut store, &mut arena, not, x, fa);
    let mut rule = Rule::propagate("guard_frames", [c("p", [t("f", [v(0)])])], Goal::True);
    rule.guards = vec![Guard::Equal(v(0), atom("a"))];
    let mut resources = Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
    let p = resources
        .insert("p", vec![x], Support::TRUE, &store)
        .unwrap();
    let token = authorize(&resources, &store, &mut arena, 0, vec![p], Support::TRUE).unwrap();
    let body = resources.commit(token, &store).unwrap();
    assert_eq!(resources.pending_bodies()[&body].support, not);
}
