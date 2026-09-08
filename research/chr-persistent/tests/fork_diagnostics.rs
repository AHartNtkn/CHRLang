#![cfg(feature = "fork-diagnostics")]
use chr_persistent::{
    Stats,
    kernel::{Arena, ForkObserver, Term},
};
#[derive(Default)]
struct Observer {
    opened: Option<&'static str>,
    count: usize,
}
impl ForkObserver for Observer {
    fn before(&mut self, owner: &'static str) {
        assert!(self.opened.replace(owner).is_none());
    }
    fn after(&mut self, owner: &'static str) {
        assert_eq!(self.opened.take(), Some(owner));
        self.count += 1;
    }
}
#[test]
fn actual_clone_and_latest_prefix_interning_are_branch_local() {
    let mut arena = Arena::default();
    let mut stats = Stats::default();
    let a = arena.make("a", vec![], &mut stats);
    assert_eq!(arena.take_fork_interning().misses, 0);
    let mut observer = Observer::default();
    let mut sibling = arena.clone_observed(&mut observer);
    assert_eq!(observer.count, 5);
    assert!(observer.opened.is_none());
    arena.mark_fork_prefix();
    sibling.mark_fork_prefix();
    assert_eq!(arena.make("a", vec![], &mut stats), a);
    let b = arena.make("b", vec![Term::Var(8)], &mut stats);
    assert_eq!(arena.make("b", vec![Term::Var(8)], &mut stats), b);
    let counts = arena.take_fork_interning();
    assert_eq!(
        (counts.inherited_hits, counts.local_hits, counts.misses),
        (1, 1, 1)
    );
    assert_eq!(sibling.node_count(), 1);
    let segment = arena.fork_segment().unwrap();
    assert_eq!(
        (
            segment.inherited_nodes,
            segment.first_miss_nodes,
            segment.predicate_insertions
        ),
        (1, Some(1), 0)
    );
    assert_eq!(segment.requests_before_first_miss, Some(1));
    sibling.predicate("new", 0);
    sibling.predicate("new", 0);
    assert_eq!(sibling.fork_segment().unwrap().predicate_insertions, 1);
    let mut next = arena.clone_observed(&mut observer);
    arena.mark_fork_prefix();
    next.mark_fork_prefix();
    assert_eq!(next.make("b", vec![Term::Var(8)], &mut stats), b);
    assert_eq!(next.take_fork_interning().inherited_hits, 1);
    assert_eq!(arena.take_fork_interning().inherited_hits, 0);
    assert_eq!(sibling.make("c", vec![], &mut stats), Term::Node(1));
    assert_eq!(sibling.take_fork_interning().misses, 1);
}
