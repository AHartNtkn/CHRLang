//! Continuing active-state lifecycle with matched source and consumer controls.
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
    fn compact_term(term: &mut Term) {
        if let Term::App(name, args) = term {
            for arg in args.iter_mut() {
                compact_term(arg);
            }
            args.shrink_to_fit();
            name.shrink_to_fit();
        }
    }
    fn compact(a: &mut Answer) {
        for (name, term) in &mut a.outputs {
            name.shrink_to_fit();
            compact_term(term);
        }
        for c in &mut a.residual {
            c.name.shrink_to_fit();
            for arg in &mut c.args {
                compact_term(arg);
            }
            c.args.shrink_to_fit();
        }
        a.outputs.shrink_to_fit();
        a.residual.shrink_to_fit();
    }
    fn rules(resource: bool) -> Vec<Rule> {
        let mut rules = vec![Rule::simplify(
            "emit-or-recur",
            [c("stream", [v(0)])],
            or(
                eq(v(0), t("pair", [v(1), v(1)])),
                c("stream", [v(0)]).into(),
            ),
        )];
        if resource {
            rules.push(Rule::simplify(
                "finish",
                [c("finish", [t("pair", [v(0), v(1)]), v(2)]), c("token", [])],
                eq(v(2), t("pair", [v(0), v(1)])),
            ));
        }
        rules
    }
    fn query(resource: bool, seed: usize) -> Query {
        let base = 1000 * seed as u64 + 10;
        let mut constraints = vec![
            c("stream", [v(base)]),
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
    fn pump(d: &mut Delivery) {
        if d.queued.len() == d.capacity {
            d.blocked += 1;
            return;
        }
        assert!(!d.exhausted);
        d.calls += 1;
        assert!(d.calls <= 200_000_000, "registered service cutoff");
        match d.producer.tick() {
            Event::Answer(a) => d.queued.push_back(a),
            Event::Progress => (),
            Event::Done => panic!("continuing source exhausted"),
        }
    }
    #[allow(clippy::assertions_on_constants)]
    pub fn run() {
        let args = std::env::args().collect::<Vec<_>>();
        if args.get(1).is_some_and(|s| s == "meter-check") {
            #[cfg(feature = "alloc-meter")]
            meter::self_check().unwrap();
            return;
        }
        assert_eq!(
            args.len(),
            8,
            "mode resource demand keep capacity compact rss"
        );
        assert!(
            !chr_reuse::continuations::COLLECT_METRICS
                && !chr_direct_choice::demand::COLLECT_WORK_DIAGNOSTICS
                && !chr_compiled::COLLECT_METRICS
                && !chr_compiled::COLLECT_KERNEL_METRICS
                && !chr_observe::COLLECT_METRICS
        );
        let mode = &args[1];
        let reclaim = mode.ends_with("-reclaim");
        let base = mode.strip_suffix("-reclaim").unwrap_or(mode);
        assert!(
            [
                "direct",
                "compact-live",
                "scan",
                "sealed",
                "conditional",
                "dependencies",
                "templates"
            ]
            .contains(&base)
        );
        assert!(!reclaim || ["dependencies", "templates"].contains(&base));
        let resource = args[2].parse::<bool>().unwrap();
        let demand = args[3].parse::<usize>().unwrap();
        assert!([32, 64, 128, 512].contains(&demand));
        let keep = match args[4].as_str() {
            "0" => 0,
            "4" => 4,
            "all" => usize::MAX,
            _ => panic!("keep"),
        };
        let capacity = args[5].parse::<usize>().unwrap();
        assert!([1, 4].contains(&capacity));
        let packing = args[6].parse::<bool>().unwrap();
        let resident = args[7].parse::<bool>().unwrap();
        let mut records = Vec::with_capacity(12 * demand + 64);
        let mut snapshots = Vec::with_capacity(32);
        records.resize_with(12 * demand + 64, || ("", 0, 0, measured(|| ()).1));
        std::hint::black_box(&records);
        records.clear();
        snapshots.resize(32, ("", 0, 0, 0, 0, 0, 0, 0));
        std::hint::black_box(&snapshots);
        snapshots.clear();
        let wanted = [expected(0), expected(1)];
        if resident {
            snapshots.push(("root", 0, 0, 0, 0, 0, 0, rss()));
        }
        #[cfg(feature = "alloc-meter")]
        let root = meter::begin();
        let (p, m) = measured(|| {
            Owner::new(
                base,
                Schema {
                    family: "aliases",
                    resource: false,
                    fail_tail: false,
                    work: 0,
                    payload: 0,
                },
                rules(resource),
            )
        });
        records.push(("prepare", 0, 0, m));
        let mut held = VecDeque::new();
        for (seed, expected_answer) in wanted.iter().enumerate() {
            let (mut d, m) = measured(|| Delivery::new(p.start(query(resource, seed)), capacity));
            records.push(("setup", seed, 0, m));
            let mut delivered = 0;
            let mut removed = 0;
            let mut claims = 0;
            for target in [1, 8, 32, 64, 128, 512]
                .into_iter()
                .filter(|n| *n <= demand)
            {
                while delivered < target {
                    let (_, m) = measured(|| {
                        while d.queued.len() < d.capacity {
                            let count = d.queued.len();
                            pump(&mut d);
                            if reclaim && d.queued.len() > count {
                                let Service::Existing(Running::Graph(g)) = &mut d.producer else {
                                    unreachable!()
                                };
                                let r = g.reclaim_incompatible_supports();
                                removed += r.results;
                                claims += r.consumption_claims;
                            }
                        }
                    });
                    records.push(("produce", seed, delivered, m));
                    let calls = d.calls;
                    let (_, m) = measured(|| {
                        for _ in 0..8 {
                            pump(&mut d);
                        }
                    });
                    records.push(("pause", seed, delivered, m));
                    assert_eq!(calls, d.calls);
                    let (mut a, m) = measured(|| d.queued.pop_front().unwrap());
                    records.push(("deliver", seed, delivered, m));
                    {
                        oracle::same_raw(vec![a.clone()], vec![expected_answer.clone()]);
                    }
                    let (_, m) = measured(|| {
                        if packing {
                            compact(&mut a);
                        }
                    });
                    records.push(("export", seed, delivered, m));
                    {
                        oracle::same_raw(vec![a.clone()], vec![expected_answer.clone()]);
                    }
                    let (_, m) = measured(|| {
                        if keep == 0 {
                            drop(a);
                        } else {
                            held.push_back((seed, a));
                            if held.len() > keep {
                                held.pop_front();
                            }
                        }
                    });
                    records.push(("consumer", seed, delivered, m));
                    delivered += 1;
                }
                snapshots.push((
                    "prefix",
                    seed,
                    delivered,
                    held.len(),
                    d.queued.len(),
                    removed,
                    claims,
                    if resident { rss() } else { 0 },
                ));
            }
            if reclaim {
                assert_eq!(removed > 0, resource, "intended reclamation opportunity");
            }
            snapshots.push((
                "before_cancel",
                seed,
                delivered,
                held.len(),
                d.queued.len(),
                removed,
                claims,
                if resident { rss() } else { 0 },
            ));
            let (_, m) = measured(|| drop(d));
            records.push(("cancel", seed, delivered, m));
        }
        let (_, m) = measured(|| drop(p));
        records.push(("prepared_drop", 0, 0, m));
        {
            for (seed, a) in &held {
                oracle::same_raw(vec![a.clone()], vec![wanted[*seed].clone()]);
            }
        }
        let (_, m) = measured(|| drop(held));
        records.push(("consumer_drop", 0, 0, m));
        #[cfg(feature = "alloc-meter")]
        {
            let end = meter::end(root);
            assert_eq!(
                end.live_start, end.live_end,
                "owned heap remains after disposal"
            );
        }
        if resident {
            snapshots.push(("disposed", 0, 0, 0, 0, 0, 0, rss()));
        }
        let rows=records.iter().map(|(phase,query,answer,m)|format!("{{\"phase\":\"{phase}\",\"query\":{query},\"answer\":{answer},\"reading\":{}}}",m.json())).collect::<Vec<_>>().join(",");
        let points=snapshots.iter().map(|(stage,query,delivered,held,queued,removed,claims,rss)|format!("{{\"stage\":\"{stage}\",\"query\":{query},\"delivered\":{delivered},\"held\":{held},\"queued\":{queued},\"removed\":{removed},\"claims\":{claims},\"rss_kib\":{rss}}}")).collect::<Vec<_>>().join(",");
        println!(
            "{{\"meter\":{},\"validated\":true,\"records\":[{rows}],\"snapshots\":[{points}]}}",
            cfg!(feature = "alloc-meter")
        );
    }
}
fn main() {
    gate::run();
}
