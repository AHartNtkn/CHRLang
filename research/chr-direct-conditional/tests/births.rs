use chr_direct_conditional::births::{Births, HistoryEvent};
use chr_direct_conditional::support::{Arena, Support};
fn histories(births: &Births, arena: &Arena) -> Vec<Vec<bool>> {
    let mut cursor = births.histories();
    let mut result = vec![];
    for _ in 0..10000 {
        match cursor.tick(births, arena) {
            HistoryEvent::Progress => (),
            HistoryEvent::History(h) => result.push(h),
            HistoryEvent::Exhausted => return result,
        }
    }
    panic!("finite birth enumeration failed to finish");
}
#[test]
fn nested_inactive_birth_has_three_histories_and_equal_arms_still_count() {
    let mut arena = Arena::new();
    let mut births = Births::new();
    assert_eq!(histories(&births, &arena), vec![vec![]]);
    let outer = births.create(&mut arena, Support::TRUE);
    assert_eq!(histories(&births, &arena), vec![vec![false], vec![true]]);
    births.create(&mut arena, outer);
    assert_eq!(
        histories(&births, &arena),
        vec![vec![false, false], vec![true, false], vec![true, true]]
    );
}
#[test]
fn independent_births_are_cartesian_and_cancelled_cursor_does_not_consume_them() {
    let mut arena = Arena::new();
    let mut births = Births::new();
    births.create(&mut arena, Support::TRUE);
    births.create(&mut arena, Support::TRUE);
    let mut partial = births.histories();
    for _ in 0..3 {
        partial.tick(&births, &arena);
    }
    drop(partial);
    assert_eq!(
        histories(&births, &arena),
        vec![
            vec![false, false],
            vec![false, true],
            vec![true, false],
            vec![true, true]
        ]
    );
}
#[test]
fn a_cursor_keeps_its_birth_prefix_when_other_work_introduces_births() {
    let mut arena = Arena::new();
    let mut births = Births::new();
    births.create(&mut arena, Support::TRUE);
    let mut cursor = births.histories();
    births.create(&mut arena, Support::TRUE);
    let mut actual = vec![];
    for _ in 0..100 {
        match cursor.tick(&births, &arena) {
            HistoryEvent::History(h) => actual.push(h),
            HistoryEvent::Exhausted => break,
            HistoryEvent::Progress => (),
        }
    }
    assert_eq!(actual, vec![vec![false], vec![true]]);
}
#[test]
fn all_three_birth_causal_guards_match_independent_reachable_histories() {
    for first_active in [false, true] {
        for second_table in 0..4 {
            for third_table in 0..16 {
                let mut arena = Arena::new();
                let mut births = Births::new();
                let leaf = |b| if b { Support::TRUE } else { Support::FALSE };
                births.create(&mut arena, leaf(first_active));
                let second = arena.mk(0, leaf(second_table & 1 != 0), leaf(second_table & 2 != 0));
                births.create(&mut arena, second);
                let lo = arena.mk(1, leaf(third_table & 1 != 0), leaf(third_table & 4 != 0));
                let hi = arena.mk(1, leaf(third_table & 2 != 0), leaf(third_table & 8 != 0));
                let third = arena.mk(0, lo, hi);
                births.create(&mut arena, third);
                let mut expected = vec![];
                for a in [false, true] {
                    for b in [false, true] {
                        for c in [false, true] {
                            let active_b = second_table & (1 << (a as usize)) != 0;
                            let active_c =
                                third_table & (1 << ((a as usize) + 2 * (b as usize))) != 0;
                            if (!a || first_active) && (!b || active_b) && (!c || active_c) {
                                expected.push(vec![a, b, c]);
                            }
                        }
                    }
                }
                assert_eq!(histories(&births, &arena), expected);
            }
        }
    }
}
