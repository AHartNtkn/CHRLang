//! The same conjunction has opposite construction preferences depending on the
//! order in which its variables become operands. Truth is independent of both.
use chr_direct_conditional::support::{Arena, Operation, Status, Support};
fn conjunction(width: usize, newest_first: bool) -> usize {
    let mut arena = Arena::new();
    let variables: Vec<_> = (0..width).map(|_| arena.fresh_variable().1).collect();
    let mut ids: Vec<_> = (0..width).collect();
    if newest_first {
        ids.reverse();
    }
    let mut result = Support::TRUE;
    for i in ids {
        let mut job = arena.job(Operation::And(result, variables[i]));
        let mut complete = None;
        for _ in 0..10000 {
            if let Status::Complete(value) = job.tick(&mut arena) {
                complete = Some(value);
                break;
            }
        }
        result = complete.expect("finite conjunction");
    }
    assert!(arena.eval(result, &vec![true; width]));
    for i in 0..width {
        let mut values = vec![true; width];
        values[i] = false;
        assert!(!arena.eval(result, &values));
    }
    arena.node_count()
}
#[test]
fn reversing_operand_arrival_reverses_the_smaller_diagram_construction() {
    for width in [1, 4, 16, 64] {
        let forward = conjunction(width, false);
        let backward = conjunction(width, true);
        let shared = 2 + width + width - 1;
        let rebuilt = 2 + width + width * (width - 1) / 2;
        if cfg!(feature = "support-reverse-order") {
            assert_eq!((forward, backward), (shared, rebuilt));
        } else {
            assert_eq!((forward, backward), (rebuilt, shared));
        }
        println!("width={width} forward_nodes={forward} backward_nodes={backward}");
    }
}
