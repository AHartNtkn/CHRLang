#[cfg(feature = "alloc-meter")]
#[allow(unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[cfg(feature = "alloc-meter")]
mod probe {
    use super::meter;
    use chr_structural::projection::{Problem, Relation, Semantics};
    use std::collections::BTreeMap;
    use std::mem::size_of;
    struct Row {
        phase: &'static str,
        query: usize,
        reading: meter::Reading,
    }
    fn measure<T>(
        rows: &mut Vec<Row>,
        phase: &'static str,
        query: usize,
        f: impl FnOnce() -> T,
    ) -> T {
        let start = meter::begin();
        let value = f();
        let reading = meter::end(start);
        rows.push(Row {
            phase,
            query,
            reading,
        });
        value
    }
    fn problem(family: &str) -> (Problem, Vec<usize>) {
        match family {
            "independent" => (
                Problem {
                    domains: vec![vec![0, 1]; 12],
                    filters: vec![],
                },
                vec![0],
            ),
            "star" => (
                Problem {
                    domains: vec![vec![0, 1]; 8],
                    filters: (1..8)
                        .map(|i| Relation {
                            scope: vec![0, i],
                            rows: vec![vec![0, 0], vec![0, 1], vec![1, 0]],
                        })
                        .collect(),
                },
                vec![6, 7],
            ),
            "dense" => (
                Problem {
                    domains: vec![vec![0, 1]; 10],
                    filters: vec![],
                },
                (0..10).collect(),
            ),
            _ => panic!("unknown family"),
        }
    }
    fn oracle(
        p: &Problem,
        visible: &[usize],
        restriction: &[(usize, u8)],
    ) -> BTreeMap<Vec<u8>, u128> {
        let mut answer = BTreeMap::new();
        for mut index in 0..p.domains.iter().map(Vec::len).product::<usize>() {
            let mut values = vec![0; p.domains.len()];
            for i in (0..values.len()).rev() {
                values[i] = p.domains[i][index % p.domains[i].len()];
                index /= p.domains[i].len();
            }
            if restriction.iter().any(|&(i, v)| values[i] != v)
                || p.filters.iter().any(|r| {
                    !r.rows
                        .iter()
                        .any(|row| r.scope.iter().zip(row).all(|(&i, &v)| values[i] == v))
                })
            {
                continue;
            }
            *answer
                .entry(visible.iter().map(|&i| values[i]).collect())
                .or_insert(0) += 1;
        }
        answer
    }
    pub fn main() {
        assert!(!cfg!(feature = "metrics"), "metrics-off required");
        meter::self_check().unwrap();
        let args = std::env::args().collect::<Vec<_>>();
        assert_eq!(args.len(), 4, "family retain(0/4/all) cancel(0/1)");
        let family = &args[1];
        let keep = match args[2].as_str() {
            "0" => 0,
            "4" => 4,
            "all" => usize::MAX,
            _ => panic!("retain"),
        };
        let cancel = match args[3].as_str() {
            "0" => false,
            "1" => true,
            _ => panic!("cancel"),
        };
        let mut totals = [0usize; 4];
        {
            let (p, visible) = problem(family);
            let q = p
                .project(
                    &visible,
                    &[],
                    &p.elimination_order(&visible).unwrap(),
                    Semantics::Counted,
                    100_000,
                )
                .unwrap();
            for (i, total) in totals.iter_mut().enumerate() {
                let r = if i == 1 || i == 2 {
                    vec![(visible[0], (i - 1) as u8)]
                } else {
                    vec![]
                };
                let expected = oracle(&p, &visible, &r);
                assert_eq!(q.answers(&r, 100_000).unwrap(), expected);
                let mut actual = BTreeMap::new();
                for row in q.expanded_answers(&r, 100_000, 100_000).unwrap() {
                    *actual.entry(row).or_insert(0) += 1;
                }
                assert_eq!(actual, expected);
                *total = expected.values().sum::<u128>() as usize;
            }
        }
        let mut rows = Vec::with_capacity(64);
        let mut consumer: Vec<Vec<u8>> = Vec::new();
        let owner = meter::begin();
        let (p, visible) = measure(&mut rows, "source", 0, || problem(family));
        let order = measure(&mut rows, "order", 0, || {
            p.elimination_order(&visible).unwrap()
        });
        let q = measure(&mut rows, "project", 0, || {
            p.project(&visible, &[], &order, Semantics::Counted, 100_000)
                .unwrap()
        });
        let coordinate = visible[0];
        measure(&mut rows, "source_order_dispose", 0, || {
            drop((p, visible, order))
        });
        let mut q = Some(q);
        let mut delivered = [0usize; 4];
        for i in 0usize..4 {
            let restriction = [(coordinate, i.saturating_sub(1) as u8)];
            let r = if i == 1 || i == 2 {
                &restriction[..]
            } else {
                &[]
            };
            let mut iter = measure(&mut rows, "weighted_iterator_setup", i, || {
                q.as_ref()
                    .unwrap()
                    .expanded_answers(r, 100_000, 100_000)
                    .unwrap()
            });
            if i == 3 {
                measure(&mut rows, "producer_dispose", i, || drop(q.take()));
            }
            measure(&mut rows, "expand_consume", i, || {
                let limit = if cancel { 8.min(totals[i]) } else { totals[i] };
                for _ in 0..limit {
                    let row = iter.next().unwrap();
                    delivered[i] += 1;
                    if keep > 0 {
                        if consumer.len() == keep {
                            consumer.remove(0);
                        }
                        consumer.push(row);
                    }
                }
                if !cancel {
                    assert!(iter.next().is_none());
                }
            });
            measure(&mut rows, "iterator_dispose", i, || drop(iter));
        }
        assert!(q.is_none());
        let consumer_bytes = consumer.capacity() * size_of::<Vec<u8>>()
            + consumer.iter().map(Vec::capacity).sum::<usize>();
        let held = meter::end(owner);
        assert_eq!(
            held.live_end - held.live_start,
            consumer_bytes,
            "unaccounted producer retention"
        );
        measure(&mut rows, "consumer_dispose", 4, || drop(consumer));
        let released = meter::end(owner);
        assert_eq!(released.live_end, released.live_start, "owner disposal");
        println!(
            "{{\"family\":\"{family}\",\"retain\":\"{}\",\"cancel\":{cancel},\"delivered\":{delivered:?},\"expected\":{totals:?},\"consumer_bytes\":{consumer_bytes},\"unreleased_bytes\":0,\"requested_bytes\":{}}}",
            args[2], released.requested_bytes
        );
        for row in rows {
            println!(
                "{{\"phase\":\"{}\",\"query\":{},\"memory\":{}}}",
                row.phase,
                row.query,
                row.reading.json()
            );
        }
    }
}
fn main() {
    #[cfg(feature = "alloc-meter")]
    probe::main();
    #[cfg(not(feature = "alloc-meter"))]
    panic!("alloc-meter feature required");
}
