#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code, unused_imports)]
mod gate {
    include!("../tests/resource_capacity_entry.rs");
    #[cfg(feature = "alloc-meter")]
    use super::meter;
    use std::time::Instant;
    struct Phase {
        name: &'static str,
        query: usize,
        ns: u128,
        #[cfg(feature = "alloc-meter")]
        memory: meter::Reading,
    }
    fn measure<T>(
        records: &mut Vec<Phase>,
        name: &'static str,
        query: usize,
        f: impl FnOnce() -> T,
    ) -> T {
        assert!(records.len() < records.capacity());
        #[cfg(feature = "alloc-meter")]
        let memory_start = meter::begin();
        let start = Instant::now();
        let result = f();
        let ns = start.elapsed().as_nanos();
        #[cfg(feature = "alloc-meter")]
        let memory = meter::end(memory_start);
        records.push(Phase {
            name,
            query,
            ns,
            #[cfg(feature = "alloc-meter")]
            memory,
        });
        result
    }
    enum Prepared {
        Capacity(capacity::Prepared<{ cfg!(feature = "capacity-diagnostics") }>),
        Ordinary(chr_compiled::PreparedRuleset),
    }
    pub fn run() {
        let args = std::env::args().collect::<Vec<_>>();
        if args.get(1).map(String::as_str) == Some("meter-check") {
            #[cfg(feature = "alloc-meter")]
            {
                meter::self_check().unwrap();
                println!("meter-check passed");
                return;
            }
            #[cfg(not(feature = "alloc-meter"))]
            panic!("allocation meter not enabled");
        }
        assert_eq!(args.len(), 6);
        let mode = args[1].as_str();
        let n = args[2].parse::<usize>().unwrap();
        let supply = args[3].as_str();
        let weight = args[4].parse::<usize>().unwrap();
        let reuse = args[5].parse::<usize>().unwrap();
        assert!(
            [
                "capacity",
                "scan",
                "index",
                "specialized-scan",
                "specialized-index"
            ]
            .contains(&mode)
        );
        assert!([0, 2, 4].contains(&n) && [1, 2].contains(&weight) && [1, 4].contains(&reuse));
        assert!(["empty", "tight", "spare"].contains(&supply));
        assert!(!chr_compiled::COLLECT_METRICS && !chr_observe::COLLECT_METRICS);
        let mut records = Vec::with_capacity(8 + 3 * reuse);
        let mut answers = Vec::with_capacity(reuse);
        let mut work = Vec::with_capacity(reuse);
        let mut counts = Vec::with_capacity(reuse);
        #[cfg(feature = "alloc-meter")]
        let baseline = meter::end(meter::begin()).live_end;
        let rules = measure(&mut records, "source_build", 0, || {
            source("capacity_", weight)
        });
        let p = measure(&mut records, "prepare", 0, || {
            if mode == "capacity" {
                Prepared::Capacity(capacity::Prepared::new(&rules).unwrap())
            } else {
                let p = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
                Prepared::Ordinary(if mode.starts_with("specialized") {
                    p.specialize_inferred()
                } else {
                    p
                })
            }
        });
        for i in 0..reuse {
            let q = measure(&mut records, "input_build", i, || {
                query(
                    "capacity_",
                    &vec![3; n],
                    match supply {
                        "empty" => [0, 0],
                        "tight" => [n / 2, n - n / 2],
                        _ => [n, n],
                    },
                    i % 2 == 1,
                    100 + i as u64 * 100,
                )
            });
            let (out, stats) =
                measure(&mut records, "setup_execute_observe_drop", i, || match &p {
                    Prepared::Capacity(p) => {
                        let r = p.solve(&q, Default::default()).unwrap();
                        (r.answers, [r.states, r.branches, r.capacity_prunes])
                    }
                    Prepared::Ordinary(p) => {
                        let mut run = p
                            .start_search(
                                q.clone(),
                                chr_compiled::Policy::Global,
                                if mode.ends_with("index") {
                                    chr_compiled::Access::Indexed
                                } else {
                                    chr_compiled::Access::Scan
                                },
                            )
                            .unwrap();
                        let mut out = Vec::new();
                        let mut ended = false;
                        for _ in 0..200000 {
                            match run.tick() {
                                chr_compiled::SearchEvent::Complete(mut branch) => {
                                    out.push(branch.engine.observe().expect("complete observation"))
                                }
                                chr_compiled::SearchEvent::Exhausted => {
                                    ended = true;
                                    break;
                                }
                                _ => (),
                            }
                        }
                        assert!(ended, "ordinary service cutoff");
                        (out, [0; 3])
                    }
                });
            counts.push(out.len());
            answers.push(out);
            work.push(stats);
            measure(&mut records, "input_drop", i, || drop(q));
        }
        measure(&mut records, "prepared_drop", 0, || drop(p));
        #[cfg(feature = "alloc-meter")]
        let validation_live = meter::end(meter::begin()).live_end;
        for (i, out) in answers.iter().enumerate() {
            let q = query(
                "capacity_",
                &vec![3; n],
                match supply {
                    "empty" => [0, 0],
                    "tight" => [n / 2, n - n / 2],
                    _ => [n, n],
                },
                i % 2 == 1,
                100 + i as u64 * 100,
            );
            runtime_support::same_raw(out.clone(), checked(&rules, &q));
            runtime_support::same_raw(
                out.clone(),
                relation(
                    "capacity_",
                    &vec![3; n],
                    match supply {
                        "empty" => [0, 0],
                        "tight" => [n / 2, n - n / 2],
                        _ => [n, n],
                    },
                    i % 2 == 1,
                    weight,
                    100 + i as u64 * 100,
                ),
            );
        }
        #[cfg(feature = "alloc-meter")]
        assert_eq!(meter::end(meter::begin()).live_end, validation_live);
        measure(&mut records, "consumer_drop", 0, || answers.clear());
        measure(&mut records, "source_drop", 0, || drop(rules));
        #[cfg(feature = "alloc-meter")]
        assert_eq!(meter::end(meter::begin()).live_end, baseline);
        let phases = records
            .iter()
            .map(|p| {
                #[cfg(feature = "alloc-meter")]
                let memory = p.memory.json();
                #[cfg(not(feature = "alloc-meter"))]
                let memory = "null";
                format!(
                    "{{\"name\":\"{}\",\"query\":{},\"ns\":{},\"memory\":{memory}}}",
                    p.name, p.query, p.ns
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        println!(
            "{{\"mode\":\"{mode}\",\"requests\":{n},\"supply\":\"{supply}\",\"weight\":{weight},\"reuse\":{reuse},\"counts\":{counts:?},\"work\":{work:?},\"diagnostics\":{},\"meter\":{},\"phases\":[{phases}]}}",
            cfg!(feature = "capacity-diagnostics"),
            cfg!(feature = "alloc-meter")
        );
    }
}
fn main() {
    gate::run()
}
