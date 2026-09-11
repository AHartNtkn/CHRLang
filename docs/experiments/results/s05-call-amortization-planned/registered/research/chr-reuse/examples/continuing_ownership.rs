//! Requested-heap trajectories for the qualified continuing emitter.
use chr_compiled::experiment::meter;
#[allow(dead_code, unused_imports)]
mod gate {
    include!("../tests/consumer_pressure.rs");
    use super::meter;
    struct Snapshot {
        label: &'static str,
        delivered: usize,
        retained: usize,
        queued: usize,
        memory: meter::Reading,
    }
    fn reclaim(d: &mut Delivery, enabled: bool) -> usize {
        if !enabled {
            return 0;
        }
        match &mut d.producer {
            Service::Existing(Running::Graph(g)) => g.reclaim_incompatible_supports().results,
            _ => panic!("reclamation requires graph"),
        }
    }
    fn consume(
        d: &mut Delivery,
        retained: &mut VecDeque<Answer>,
        keep: usize,
        delivered: &mut usize,
        target: usize,
        reclaiming: bool,
        root: Option<meter::Start>,
    ) -> usize {
        let mut removed = 0;
        while d.queued.len() < d.capacity {
            let queued = d.queued.len();
            d.pump();
            if d.queued.len() > queued {
                removed += reclaim(d, reclaiming);
            }
            assert!(!d.exhausted, "continuing source exhausted");
        }
        let before = root.map(meter::end);
        for _ in 0..8 {
            d.pump();
        }
        if let (Some(before), Some(root)) = (before, root) {
            let after = meter::end(root);
            assert_eq!(before.live_end, after.live_end);
            assert_eq!(before.requested_bytes, after.requested_bytes);
            assert_eq!(before.allocation_calls, after.allocation_calls);
            assert_eq!(before.deallocation_calls, after.deallocation_calls);
        }
        for _ in 0..if d.capacity == 4 { 3 } else { 1 } {
            if *delivered == target {
                break;
            }
            let a = d.queued.pop_front().unwrap();
            if root.is_none() {
                oracle::same_raw(
                    vec![a.clone()],
                    vec![Answer {
                        outputs: vec![("answer".into(), t("pair", [v(99), v(99)]))],
                        residual: vec![],
                    }],
                );
            }
            retained.push_back(a);
            if retained.len() > keep {
                retained.pop_front();
            }
            *delivered += 1;
        }
        removed
    }
    fn prepare(mode: &str) -> Owner {
        let rules = vec![Rule::simplify(
            "emit-or-recur",
            [c("stream", [v(0)])],
            or(
                eq(v(0), t("pair", [v(1), v(1)])),
                c("stream", [v(0)]).into(),
            ),
        )];
        let schema = Schema {
            family: "aliases",
            fail_tail: false,
            resource: false,
            work: 0,
            payload: 0,
        };
        Owner::new(mode, schema, rules)
    }
    fn start(p: &Owner, capacity: usize) -> Delivery {
        Delivery::new(
            p.start(Query {
                constraints: vec![c("stream", [v(10)])],
                outputs: vec![("answer".into(), Var(10))],
            }),
            capacity,
        )
    }
    pub fn run() {
        let args = std::env::args().collect::<Vec<_>>();
        if args.get(1).is_some_and(|x| x == "meter-check") {
            meter::self_check().unwrap();
            println!("meter-check passed");
            return;
        }
        assert_eq!(args.len(), 5, "mode demand keep capacity");
        assert!(
            !chr_reuse::continuations::COLLECT_METRICS
                && !chr_direct_choice::demand::COLLECT_WORK_DIAGNOSTICS
                && !chr_compiled::COLLECT_METRICS
                && !chr_compiled::COLLECT_KERNEL_METRICS
                && !chr_observe::COLLECT_METRICS
        );
        let mode = args[1].as_str();
        let reclaiming = mode.ends_with("-reclaim");
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
        assert!(!reclaiming || ["dependencies", "templates"].contains(&base));
        let demand = args[2].parse::<usize>().unwrap();
        assert!([32, 128, 512].contains(&demand));
        let keep = match args[3].as_str() {
            "0" => 0,
            "4" => 4,
            "all" => usize::MAX,
            _ => panic!("keep"),
        };
        let capacity = args[4].parse::<usize>().unwrap();
        assert!([1, 4].contains(&capacity));
        // Independent replay and its retained observation owners end before accounting.
        {
            let p = prepare(base);
            let mut d = start(&p, capacity);
            let mut held = VecDeque::new();
            let mut delivered = 0;
            for target in [1, 8, 32, 128, 512].into_iter().filter(|n| *n <= demand) {
                while delivered < target {
                    consume(
                        &mut d,
                        &mut held,
                        usize::MAX,
                        &mut delivered,
                        target,
                        reclaiming,
                        None,
                    );
                }
            }
            drop(d);
            drop(p);
            for a in held {
                oracle::same_raw(
                    vec![a],
                    vec![Answer {
                        outputs: vec![("answer".into(), t("pair", [v(99), v(99)]))],
                        residual: vec![],
                    }],
                );
            }
        }
        let mut snapshots = Vec::with_capacity(16);
        let mut held = VecDeque::with_capacity(demand + 1);
        let root = meter::begin();
        let baseline = meter::end(root).live_end;
        let p = prepare(base);
        let prepared_live = meter::end(root).live_end;
        snapshots.push(Snapshot {
            label: "prepared",
            delivered: 0,
            retained: 0,
            queued: 0,
            memory: meter::end(root),
        });
        let mut d = start(&p, capacity);
        snapshots.push(Snapshot {
            label: "setup",
            delivered: 0,
            retained: 0,
            queued: 0,
            memory: meter::end(root),
        });
        let mut delivered = 0;
        let mut removed = 0;
        for target in [1, 8, 32, 128, 512].into_iter().filter(|n| *n <= demand) {
            while delivered < target {
                removed += consume(
                    &mut d,
                    &mut held,
                    keep,
                    &mut delivered,
                    target,
                    reclaiming,
                    Some(root),
                );
            }
            snapshots.push(Snapshot {
                label: "prefix",
                delivered,
                retained: held.len(),
                queued: d.queued.len(),
                memory: meter::end(root),
            });
        }
        assert_eq!(delivered, demand);
        let blocked = d.blocked;
        snapshots.push(Snapshot {
            label: "before-engine-drop",
            delivered,
            retained: held.len(),
            queued: d.queued.len(),
            memory: meter::end(root),
        });
        drop(d);
        snapshots.push(Snapshot {
            label: "engine-dropped",
            delivered,
            retained: held.len(),
            queued: 0,
            memory: meter::end(root),
        });
        held.clear();
        assert_eq!(
            meter::end(root).live_end,
            prepared_live,
            "query/consumer ownership remains"
        );
        snapshots.push(Snapshot {
            label: "consumer-released",
            delivered,
            retained: 0,
            queued: 0,
            memory: meter::end(root),
        });
        drop(p);
        assert_eq!(
            meter::end(root).live_end,
            baseline,
            "prepared ownership remains"
        );
        snapshots.push(Snapshot {
            label: "prepared-dropped",
            delivered,
            retained: 0,
            queued: 0,
            memory: meter::end(root),
        });
        let rows=snapshots.iter().map(|s|format!("{{\"label\":\"{}\",\"delivered\":{},\"retained\":{},\"queued\":{},\"memory\":{}}}",s.label,s.delivered,s.retained,s.queued,s.memory.json())).collect::<Vec<_>>().join(",");
        println!(
            "{{\"mode\":\"{mode}\",\"demand\":{demand},\"keep\":\"{}\",\"capacity\":{capacity},\"baseline\":{baseline},\"removed_results\":{removed},\"blocked\":{blocked},\"snapshots\":[{rows}]}}",
            args[3]
        );
    }
}
fn main() {
    gate::run();
}
