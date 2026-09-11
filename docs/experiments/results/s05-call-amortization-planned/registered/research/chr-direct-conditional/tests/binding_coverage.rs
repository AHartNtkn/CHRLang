use chr_direct_conditional::{
    equality::{DemandStatus, Store, Term, UnifyStatus},
    support::{Arena, Operation, Status, Support},
};
fn op(a: &mut Arena, o: Operation) -> Support {
    let mut j = a.job(o);
    loop {
        if let Status::Complete(s) = j.tick(a) {
            return s;
        }
    }
}
fn bind(s: &mut Store, a: &mut Arena, r: Support, x: Term, y: Term) {
    let mut j = s.unify(r, x, y);
    for _ in 0..1_000_000 {
        match j.tick(s, a) {
            UnifyStatus::Pending => (),
            UnifyStatus::Complete { failed } => {
                assert_eq!(failed, Support::FALSE);
                return;
            }
            UnifyStatus::Stale => panic!("serial writer stale"),
        }
    }
    panic!("writer cutoff")
}
fn entails(s: &Store, a: &mut Arena, r: Support, x: Term, y: Term) -> Support {
    let mut j = s.entails(r, x, y);
    for _ in 0..1_000_000 {
        match j.tick(s, a) {
            DemandStatus::Pending => (),
            DemandStatus::Complete { entailed } => return entailed,
            DemandStatus::Stale => panic!("serial reader stale"),
        }
    }
    panic!("reader cutoff")
}
#[test]
fn disjoint_binding_population_is_skipped_without_losing_new_binding() {
    let mut a = Arena::new();
    let mut s = Store::new();
    let x = s.fresh_variable();
    let old = s.constructor("old", vec![]);
    let new = s.constructor("new", vec![]);
    let mut region = Support::TRUE;
    let mut committed = vec![];
    for _ in 0..32 {
        let (_, choice) = a.fresh_variable();
        let leaf = op(&mut a, Operation::And(region, choice));
        bind(&mut s, &mut a, leaf, x, old);
        committed.push(leaf);
        region = op(&mut a, Operation::Difference(region, choice));
    }
    let before = s.probe().binding_probes;
    bind(&mut s, &mut a, region, x, new);
    let visits = s.probe().binding_probes - before;
    assert_eq!(entails(&s, &mut a, region, x, new), region);
    for leaf in committed {
        assert_eq!(entails(&s, &mut a, leaf, x, old), leaf);
        assert_eq!(entails(&s, &mut a, leaf, x, new), Support::FALSE);
    }
    assert_eq!(
        visits, 0,
        "wholly disjoint binding population should not be traversed"
    );
}
