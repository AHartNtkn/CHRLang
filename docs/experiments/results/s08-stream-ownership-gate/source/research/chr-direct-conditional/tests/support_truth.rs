use chr_direct_conditional::support::{Arena, Operation, Status, Support};

fn finish(arena: &mut Arena, op: Operation) -> Support {
    let mut job = arena.job(op);
    for _ in 0..10000 {
        if let Status::Complete(result) = job.tick(arena) {
            return result;
        }
    }
    panic!("finite support job did not finish");
}
fn table(arena: &Arena, s: Support) -> u8 {
    let mut bits = 0;
    for row in 0..8 {
        let assignment = [row & 1 != 0, row & 2 != 0, row & 4 != 0];
        if arena.eval(s, &assignment) {
            bits |= 1 << row;
        }
    }
    bits
}
#[test]
fn every_three_variable_function_and_pair_matches_independent_truth_tables() {
    let mut arena = Arena::new();
    for _ in 0..3 {
        arena.fresh_variable();
    }
    // Build by Shannon branches directly; expected results use integer bit operations.
    let mut handles = vec![];
    for mask in 0u16..256 {
        let mut leaves: Vec<_> = (0..8)
            .map(|row| {
                if mask & (1 << row) != 0 {
                    Support::TRUE
                } else {
                    Support::FALSE
                }
            })
            .collect();
        for variable in (0..3).rev() {
            let half = leaves.len() / 2;
            leaves = (0..half)
                .map(|i| arena.mk(variable, leaves[i], leaves[i + half]))
                .collect();
        }
        handles.push(leaves[0]);
        assert_eq!(table(&arena, leaves[0]), mask as u8);
    }
    for (a, &left) in handles.iter().enumerate() {
        let neg = finish(&mut arena, Operation::Not(left));
        assert_eq!(table(&arena, neg), !(a as u8));
        for (b, &right) in handles.iter().enumerate() {
            for (op, expected) in [
                (Operation::And(left, right), (a & b) as u8),
                (Operation::Or(left, right), (a | b) as u8),
                (Operation::Difference(left, right), (a & !b) as u8),
            ] {
                let result = finish(&mut arena, op);
                assert_eq!(table(&arena, result), expected);
                assert_eq!(
                    result, handles[expected as usize],
                    "canonical result differs"
                );
            }
        }
    }
}

#[test]
fn cancelled_boolean_job_leaves_valid_nodes_for_interleaving_and_restart() {
    let mut arena = Arena::new();
    for _ in 0..6 {
        arena.fresh_variable();
    }
    let mut conjunction = Support::TRUE;
    for variable in (0..6).rev() {
        conjunction = arena.mk(variable, Support::FALSE, conjunction);
    }
    let before = arena.node_count();
    let mut cancelled = arena.job(Operation::Not(conjunction));
    let mut reached = false;
    for _ in 0..100 {
        let state = cancelled.tick(&mut arena);
        if arena.node_count() > before && matches!(state, Status::Pending) {
            reached = true;
            break;
        }
    }
    assert!(
        reached,
        "exercise cancellation after real intermediate construction"
    );
    drop(cancelled);
    let unrelated = finish(&mut arena, Operation::Or(conjunction, Support::TRUE));
    assert_eq!(unrelated, Support::TRUE);
    let restarted = finish(&mut arena, Operation::Not(conjunction));
    for row in 0..64 {
        let assignment: Vec<_> = (0..6).map(|bit| row & (1 << bit) != 0).collect();
        assert_eq!(arena.eval(restarted, &assignment), row != 63);
    }
}
