#[path = "../experiments/subscription_join.rs"]
mod join;
use chr_syntax::{Term, atom};
use join::{Join, Mode, Triple};
use std::collections::BTreeMap;
type Rows = [BTreeMap<usize, (Term, Term)>; 3];
// Declarative Cartesian product is independent of candidate indexes and incidence.
fn expected(rows: &Rows, key: &(Term, Term)) -> Vec<Triple> {
    let mut result = Vec::new();
    for (&l, (k, a)) in &rows[0] {
        for (&m, (aa, b)) in &rows[1] {
            for (&r, (bb, v)) in &rows[2] {
                if k == &key.0 && a == aa && b == bb && v == &key.1 {
                    result.push([l, m, r]);
                }
            }
        }
    }
    result
}
#[test]
fn updates_retirement_reopening_and_occurrences_match_independent_product() {
    for mode in [Mode::Indexed, Mode::Eager, Mode::Subscribed] {
        for seed in 0..32u64 {
            let mut engine = Join::new(mode);
            let mut rows: Rows = std::array::from_fn(|_| BTreeMap::new());
            let keys = [
                (atom("0"), atom("0")),
                (atom("1"), atom("2")),
                (atom("2"), atom("1")),
            ];
            for (i, k) in keys.iter().enumerate() {
                engine.open(i, k.clone());
            }
            let mut rng = seed + 1;
            for step in 0..72 {
                rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
                let rel = ((rng >> 32) % 3) as usize;
                if step % 4 == 3 && !rows[rel].is_empty() {
                    let id = *rows[rel].keys().next().unwrap();
                    rows[rel].remove(&id);
                    engine.remove(rel, id);
                } else {
                    let pair = (
                        atom(&((rng >> 16) % 3).to_string()),
                        atom(&((rng >> 24) % 3).to_string()),
                    );
                    rows[rel].insert(step, pair.clone());
                    engine.insert(rel, step, pair);
                }
                for (i, k) in keys.iter().enumerate() {
                    assert_eq!(
                        engine.request(i),
                        expected(&rows, k),
                        "{mode:?} seed{seed} step{step}"
                    );
                }
                if step % 9 == 8 {
                    engine.open(99, keys[0].clone());
                    engine.close(0);
                    assert_eq!(engine.request(99), expected(&rows, &keys[0]));
                    engine.close(99);
                    engine.open(0, keys[0].clone());
                    assert_eq!(engine.request(0), expected(&rows, &keys[0]));
                }
            }
            for i in 0..3 {
                engine.close(i);
            }
            if mode != Mode::Eager {
                assert_eq!(engine.retained(), 0);
            }
        }
    }
}
#[test]
fn inactive_demands_avoid_retention_and_repeated_requests_avoid_rediscovery() {
    for mode in [Mode::Indexed, Mode::Eager, Mode::Subscribed] {
        let mut e = Join::new(mode);
        for i in 0..8 {
            e.insert(0, i, (atom("k"), atom("a")));
            e.insert(1, i, (atom("a"), atom("b")));
            e.insert(2, i, (atom("b"), atom("v")));
        }
        assert_eq!(e.retained(), if mode == Mode::Eager { 512 } else { 0 });
        e.open(0, (atom("k"), atom("v")));
        let before = e.stats().probes;
        for _ in 0..3 {
            assert_eq!(e.request(0).len(), 512);
        }
        if chr_compiled::COLLECT_METRICS {
            assert_eq!(e.stats().probes > before, mode == Mode::Indexed);
        } else {
            assert_eq!(e.stats().probes, 0);
        }
        e.close(0);
        assert_eq!(e.retained(), if mode == Mode::Eager { 512 } else { 0 });
    }
}

#[test]
fn indexed_control_uses_selective_final_endpoint() {
    let mut e = Join::new(Mode::Indexed);
    for i in 0..128 {
        e.insert(0, i, (atom("k"), atom(&format!("a{i}"))));
        e.insert(1, i, (atom(&format!("a{i}")), atom(&format!("b{i}"))));
    }
    e.insert(2, 0, (atom("b0"), atom("v")));
    e.open(0, (atom("k"), atom("v")));
    assert_eq!(e.request(0), vec![[0, 0, 0]]);
    assert_eq!(
        e.stats().probes,
        if chr_compiled::COLLECT_METRICS { 2 } else { 0 }
    );
}
