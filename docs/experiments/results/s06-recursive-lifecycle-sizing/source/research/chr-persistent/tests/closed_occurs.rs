use chr_persistent::{
    Stats,
    kernel::{Arena, Bindings, Term, deref},
};

#[test]
fn occurs_skips_closed_suffixes_but_checks_open_fields() {
    let mut visits = vec![];
    for depth in [1_u64, 8, 32] {
        let mut arena = Arena::default();
        let mut stats = Stats::default();
        let mut bindings = Bindings::default();
        let mut suffix = arena.make("z", vec![], &mut stats);
        let mut changed = vec![];
        for k in 1..=depth {
            suffix = arena.make("s", vec![suffix], &mut stats);
            let cell = arena.make("cell", vec![suffix, Term::Var(0), Term::Var(0)], &mut stats);
            assert!(arena.unify_record(
                Term::Var(k),
                cell,
                &mut bindings,
                &mut stats,
                &mut changed
            ));
            assert!(arena.equal(Term::Var(k), cell, &bindings, &mut stats));
        }
        assert_eq!(changed, (1..=depth).collect::<Vec<_>>());
        // Each equation visits the open cell, its closed child once, and
        // both free-variable fields. Closed descendants add no work.
        visits.push(stats.occurs_visits);
    }
    assert_eq!(
        visits,
        if chr_persistent::COLLECT_KERNEL_METRICS {
            vec![4, 32, 128]
        } else {
            vec![0; 3]
        }
    );
}

#[test]
fn occurs_open_aliases_forks_and_transactional_notifications() {
    let mut arena = Arena::default();
    let mut stats = Stats::default();
    let a = arena.make("a", vec![], &mut stats);
    let b = arena.make("b", vec![], &mut stats);
    let closed = arena.make("closed", vec![a], &mut stats);
    let open = arena.make("open", vec![closed, Term::Var(1)], &mut stats);
    let mut bindings = Bindings::default();
    let mut changes = vec![];
    assert!(arena.unify_record(Term::Var(0), open, &mut bindings, &mut stats, &mut changes));
    assert_eq!(changes, vec![0]);
    let mut sibling = bindings.clone();
    let fork_arena = arena.clone();
    assert!(arena.unify_record(Term::Var(1), a, &mut bindings, &mut stats, &mut changes));
    assert_eq!(changes, vec![0, 1]);
    // The sibling still has the unresolved alias: binding it back through X
    // creates a cycle, even though the other branch now resolves X fully.
    let mut sibling_changes = vec![99];
    assert!(!fork_arena.unify_record(
        Term::Var(1),
        Term::Var(0),
        &mut sibling,
        &mut stats,
        &mut sibling_changes
    ));
    assert_eq!(sibling_changes, vec![99]);
    assert!(fork_arena.equal(Term::Var(1), Term::Var(1), &sibling, &mut stats));
    assert_eq!(deref(Term::Var(1), &sibling, &mut stats), Term::Var(1));
    assert!(fork_arena.unify_record(
        Term::Var(1),
        b,
        &mut sibling,
        &mut stats,
        &mut sibling_changes
    ));
    assert_eq!(sibling_changes, vec![99, 1]);
    assert_ne!(
        arena.export(Term::Var(0), &bindings, &mut stats),
        fork_arena.export(Term::Var(0), &sibling, &mut stats)
    );
    // The last field binds first. A later occurs failure must roll that
    // tentative binding back and must not publish its notification.
    let recursive = arena.make("f", vec![Term::Var(3)], &mut stats);
    let left = arena.make("pair", vec![recursive, Term::Var(2)], &mut stats);
    let right = arena.make("pair", vec![Term::Var(3), closed], &mut stats);
    assert!(!arena.unify_record(left, right, &mut bindings, &mut stats, &mut changes));
    assert_eq!(changes, vec![0, 1]);
    assert_eq!(deref(Term::Var(2), &bindings, &mut stats), Term::Var(2));
    assert_eq!(deref(Term::Var(3), &bindings, &mut stats), Term::Var(3));
}
