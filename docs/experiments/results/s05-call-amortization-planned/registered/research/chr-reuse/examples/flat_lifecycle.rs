//! Finite flat-choice adverse ownership control for completed traversal reuse.
#[allow(dead_code, unused_imports)]
mod gate {
    include!("../tests/consumer_pressure.rs");
    #[cfg(feature = "alloc-meter")]
    use chr_compiled::experiment::meter;
    use std::time::Instant;
    struct Reading {
        ns: u128,
        #[cfg(feature = "alloc-meter")]
        memory: meter::Reading,
    }
    impl Reading {
        fn json(&self) -> String {
            #[cfg(feature = "alloc-meter")]
            let mem = self.memory.json();
            #[cfg(not(feature = "alloc-meter"))]
            let mem = "null";
            format!("{{\"ns\":{},\"memory\":{mem}}}", self.ns)
        }
    }
    fn measured<T>(f: impl FnOnce() -> T) -> (T, Reading) {
        #[cfg(feature = "alloc-meter")]
        let begin = meter::begin();
        let clock = Instant::now();
        let out = f();
        let ns = clock.elapsed().as_nanos();
        #[cfg(feature = "alloc-meter")]
        let memory = meter::end(begin);
        (
            out,
            Reading {
                ns,
                #[cfg(feature = "alloc-meter")]
                memory,
            },
        )
    }

    fn rss() -> usize {
        use std::io::Read;
        let mut bytes = [0; 4096];
        let n = std::fs::File::open("/proc/self/smaps_rollup")
            .unwrap()
            .read(&mut bytes)
            .unwrap();
        std::str::from_utf8(&bytes[..n])
            .unwrap()
            .lines()
            .find_map(|s| s.strip_prefix("Rss:"))
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap()
    }
    fn rules(resource: bool, n: usize) -> Vec<Rule> {
        let body = (1..n).fold(eq(v(0), t("pair", [v(1), v(1)])), |tail, _| {
            or(eq(v(0), t("pair", [v(1), v(1)])), tail)
        });
        let mut rs = vec![Rule::simplify("flat", [c("flat", [v(0)])], body)];
        if resource {
            rs.push(Rule::simplify(
                "finish",
                [c("finish", [t("pair", [v(0), v(1)]), v(2)]), c("token", [])],
                eq(v(2), t("pair", [v(0), v(1)])),
            ));
        }
        rs
    }
    fn query(resource: bool, seed: usize) -> Query {
        let base = seed as u64 * 1000 + 10;
        let mut constraints = vec![
            c("flat", [v(base)]),
            c("query", [chr_syntax::atom(&format!("q{seed}"))]),
        ];
        if resource {
            constraints.extend([c("finish", [v(base), v(base + 1)]), c("token", [])]);
        }
        Query {
            constraints,
            outputs: vec![("answer".into(), Var(base + u64::from(resource)))],
        }
    }
    fn expected(seed: usize) -> Answer {
        Answer {
            outputs: vec![("answer".into(), t("pair", [v(99), v(99)]))],
            residual: vec![c("query", [chr_syntax::atom(&format!("q{seed}"))])],
        }
    }
    #[allow(clippy::assertions_on_constants)]
    pub fn run() {
        let args = std::env::args().collect::<Vec<_>>();
        assert!(
            [5, 6, 7].contains(&args.len()),
            "mode resource alternatives keep [rss] [complete-sessions]"
        );
        let resident = args.get(5).is_some_and(|s| s.parse::<bool>().unwrap());
        assert!(
            !chr_direct_choice::demand::COLLECT_WORK_DIAGNOSTICS
                && !chr_reuse::continuations::COLLECT_METRICS
                && !chr_compiled::COLLECT_METRICS
                && !chr_compiled::COLLECT_KERNEL_METRICS
                && !chr_observe::COLLECT_METRICS
        );
        let mode = args[1].as_str();
        assert!(["direct", "dependencies", "templates"].contains(&mode));
        let resource = args[2].parse::<bool>().unwrap();
        let n = args[3].parse::<usize>().unwrap();
        assert!([1, 32].contains(&n));
        let keep = match args[4].as_str() {
            "0" => false,
            "all" => true,
            _ => panic!("keep"),
        };
        let wanted = [expected(0), expected(1)];
        for (seed, answer) in wanted.iter().enumerate() {
            oracle::same_raw(
                oracle::run(&rules(resource, n), &query(resource, seed), 2_000_000),
                vec![answer.clone(); n],
            );
        }
        let mut records = Vec::with_capacity(8 * n + 32);
        records.resize_with(8 * n + 32, || ("", 0, 0, measured(|| ()).1));
        std::hint::black_box(&records);
        records.clear();
        let mut snapshots = Vec::with_capacity(12);
        if resident {
            snapshots.push(("root", 0, rss()));
        }
        #[cfg(feature = "alloc-meter")]
        let root = meter::begin();
        let (owner, m) = measured(|| {
            Owner::new(
                mode,
                Schema {
                    family: "aliases",
                    resource: false,
                    fail_tail: false,
                    work: 0,
                    payload: 0,
                },
                rules(resource, n),
            )
        });
        records.push(("prepare", 0, 0, m));
        if resident {
            snapshots.push(("prepared", 0, rss()));
        }
        let mut held = Vec::new();
        for (seed, answer) in wanted.iter().enumerate() {
            let (mut running, m) = measured(|| owner.start(query(resource, seed)));
            records.push(("setup", seed, 0, m));
            if resident {
                snapshots.push(("setup", seed, rss()));
            }
            let mut delivered = 0;
            let mut calls = 0;
            loop {
                let (next, m) = measured(|| {
                    loop {
                        calls += 1;
                        assert!(calls <= 2_000_000, "finite service cutoff");
                        match running.tick() {
                            Event::Progress => (),
                            other => break other,
                        }
                    }
                });
                let phase = if matches!(next, Event::Done) {
                    "exhaustion"
                } else {
                    "produce"
                };
                records.push((phase, seed, delivered, m));
                match next {
                    Event::Answer(a) => {
                        oracle::same_raw(vec![a.clone()], vec![answer.clone()]);
                        let (_, m) = measured(|| {
                            if keep {
                                held.push((seed, a));
                            } else {
                                drop(a);
                            }
                        });
                        records.push(("consumer", seed, delivered, m));
                        delivered += 1;
                    }
                    Event::Done => break,
                    Event::Progress => unreachable!(),
                }
            }
            assert_eq!(delivered, n);
            if resident {
                snapshots.push(("exhausted", seed, rss()));
            }
            let (_, m) = measured(|| drop(running));
            records.push(("producer_drop", seed, delivered, m));
            if resident {
                snapshots.push(("producer_drop", seed, rss()));
            }
        }
        let (_, m) = measured(|| drop(owner));
        records.push(("prepared_drop", 0, 0, m));
        if resident {
            snapshots.push(("prepared_drop", 0, rss()));
        }
        for (seed, a) in &held {
            oracle::same_raw(vec![a.clone()], vec![wanted[*seed].clone()]);
        }
        let (_, m) = measured(|| drop(held));
        records.push(("consumer_drop", 0, 0, m));
        #[cfg(feature = "alloc-meter")]
        {
            let end = meter::end(root);
            assert_eq!(end.live_start, end.live_end, "owned heap remains");
        }
        if resident {
            snapshots.push(("disposed", 0, rss()));
        }
        let points = snapshots
            .iter()
            .map(|(stage, query, size)| {
                format!("{{\"stage\":\"{stage}\",\"query\":{query},\"rss_kib\":{size}}}")
            })
            .collect::<Vec<_>>()
            .join(",");
        let rows=records.iter().map(|(phase,query,answer,m)|format!("{{\"phase\":\"{phase}\",\"query\":{query},\"answer\":{answer},\"reading\":{}}}",m.json())).collect::<Vec<_>>().join(",");
        println!(
            "{{\"meter\":{},\"validated\":true,\"records\":[{rows}],\"snapshots\":[{points}]}}",
            cfg!(feature = "alloc-meter")
        );
    }
}
fn main() {
    let sessions = std::env::args()
        .nth(6)
        .map_or(1, |n| n.parse::<usize>().unwrap());
    assert!(sessions > 0);
    for _ in 0..sessions {
        gate::run();
    }
}
