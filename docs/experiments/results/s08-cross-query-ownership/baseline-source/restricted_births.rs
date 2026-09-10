//! Integer-mask expectations are independent of support evaluation and traversal.
use chr_direct_conditional::births::{Births, HistoryEvent};
use chr_direct_conditional::support::{Arena, Support};
fn diagram(arena: &mut Arena, width: usize, mask: u8) -> Support {
    fn node(arena: &mut Arena, width: usize, variable: usize, prefix: usize, mask: u8) -> Support {
        if variable == width {
            return if mask & (1 << prefix) != 0 {
                Support::TRUE
            } else {
                Support::FALSE
            };
        }
        let low = node(arena, width, variable + 1, prefix, mask);
        let high = node(arena, width, variable + 1, prefix | (1 << variable), mask);
        arena.mk(variable, low, high)
    }
    node(arena, width, 0, 0, mask)
}
fn collect(births: &Births, arena: &Arena, region: Support) -> Vec<Vec<bool>> {
    let mut cursor = births.histories(region, arena);
    let mut actual = vec![];
    for _ in 0..1000 {
        match cursor.tick(births, arena) {
            HistoryEvent::Progress => (),
            HistoryEvent::History(h) => actual.push(h),
            HistoryEvent::Exhausted => {
                assert!(matches!(
                    cursor.tick(births, arena),
                    HistoryEvent::Exhausted
                ));
                return actual;
            }
        }
    }
    panic!("small restricted cursor exceeded finite service bound");
}
#[test]
fn all_causal_guard_combinations_and_support_masks_preserve_exact_ordered_histories() {
    for first in [false, true] {
        for second in 0u8..4 {
            for third in 0u8..16 {
                let mut arena = Arena::new();
                let mut births = Births::new();
                births.create(
                    &mut arena,
                    if first { Support::TRUE } else { Support::FALSE },
                );
                let second_guard = diagram(&mut arena, 1, second);
                births.create(&mut arena, second_guard);
                let third_guard = diagram(&mut arena, 2, third);
                births.create(&mut arena, third_guard);
                for mask in 0u16..256 {
                    let region = diagram(&mut arena, 3, mask as u8);
                    let mut expected = vec![];
                    for a in [false, true] {
                        for b in [false, true] {
                            for c in [false, true] {
                                let ab = usize::from(a) + 2 * usize::from(b);
                                let abc = ab + 4 * usize::from(c);
                                let active_b = second & (1 << usize::from(a)) != 0;
                                let active_c = third & (1 << ab) != 0;
                                if (!a || first)
                                    && (!b || active_b)
                                    && (!c || active_c)
                                    && mask & (1 << abc) != 0
                                {
                                    expected.push(vec![a, b, c]);
                                }
                            }
                        }
                    }
                    assert_eq!(
                        collect(&births, &arena, region),
                        expected,
                        "first={first},second={second},third={third},region={mask}"
                    );
                }
            }
        }
    }
}
#[test]
fn empty_prefix_handles_false_and_true_regions() {
    let arena = Arena::new();
    let births = Births::new();
    assert_eq!(
        collect(&births, &arena, Support::FALSE),
        Vec::<Vec<bool>>::new()
    );
    assert_eq!(
        collect(&births, &arena, Support::TRUE),
        vec![Vec::<bool>::new()]
    );
}
#[test]
fn a_restricted_cursor_keeps_its_frozen_prefix_when_later_births_are_added() {
    let mut arena = Arena::new();
    let mut births = Births::new();
    let first = births.create(&mut arena, Support::TRUE);
    let mut cursor = births.histories(first, &arena);
    let second = births.create(&mut arena, Support::TRUE);
    births.create(&mut arena, second);
    let mut actual = vec![];
    let mut exhausted = false;
    for _ in 0..100 {
        match cursor.tick(&births, &arena) {
            HistoryEvent::History(h) => actual.push(h),
            HistoryEvent::Exhausted => {
                exhausted = true;
                break;
            }
            HistoryEvent::Progress => (),
        }
    }
    assert!(exhausted);
    assert_eq!(actual, vec![vec![true]]);
}
#[test]
#[should_panic(expected = "outside the frozen birth prefix")]
fn region_cannot_reference_a_variable_outside_the_birth_ledger() {
    let mut arena = Arena::new();
    let births = Births::new();
    let (_, foreign) = arena.fresh_variable();
    births.histories(foreign, &arena);
}
