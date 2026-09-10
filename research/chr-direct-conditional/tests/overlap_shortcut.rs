use chr_direct_conditional::{
    equality::{DemandStatus, Store, UnifyStatus},
    support::{Arena, Operation, Status, Support},
};
fn op(arena: &mut Arena, op: Operation) -> Support {
    let mut j = arena.job(op);
    for _ in 0..10000 {
        if let Status::Complete(s) = j.tick(arena) {
            return s;
        }
    }
    panic!("support bound")
}
fn unify(
    store: &mut Store,
    arena: &mut Arena,
    r: Support,
    a: chr_direct_conditional::equality::Term,
    b: chr_direct_conditional::equality::Term,
) -> usize {
    let mut job = store.unify(r, a, b);
    for ticks in 1..10000 {
        match job.tick(store, arena) {
            UnifyStatus::Complete { failed } => {
                assert_eq!(failed, Support::FALSE);
                return ticks;
            }
            UnifyStatus::Pending => (),
            UnifyStatus::Stale => panic!("stale"),
        }
    }
    panic!("equality bound")
}
#[test]
fn disjoint_bindings_preserve_both_contexts_without_redundant_remainder_work() {
    let mut arena = Arena::new();
    let (_, p) = arena.fresh_variable();
    let not_p = op(&mut arena, Operation::Difference(Support::TRUE, p));
    let mut store = Store::new();
    let x = store.fresh_variable();
    let a = store.constructor("a", vec![]);
    let b = store.constructor("b", vec![]);
    unify(&mut store, &mut arena, p, x, a);
    let ticks = unify(&mut store, &mut arena, not_p, x, b);
    for (r, value, wanted) in [
        (p, a, p),
        (p, b, Support::FALSE),
        (not_p, b, not_p),
        (not_p, a, Support::FALSE),
    ] {
        let mut job = store.entails(r, x, value);
        let mut done = false;
        for _ in 0..10000 {
            match job.tick(&store, &mut arena) {
                DemandStatus::Complete { entailed } => {
                    assert_eq!(entailed, wanted);
                    done = true;
                    break;
                }
                DemandStatus::Pending => (),
                DemandStatus::Stale => panic!("stale"),
            }
        }
        assert!(done);
    }
    #[cfg(feature = "equality-overlap-shortcut")]
    assert!(
        ticks < 21,
        "must avoid some of the recorded baseline remainder work"
    );
    println!("disjoint writer ticks={ticks}");
}
