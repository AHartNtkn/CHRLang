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
        Phase(
            capacity::phase::Prepared<{ cfg!(feature = "capacity-diagnostics") }>,
            chr_compiled::PreparedRuleset,
        ),
        Ordinary(chr_compiled::PreparedRuleset),
    }
    fn make_query(n: usize, supply: &str, i: usize) -> Query {
        let caps = match supply {
            "empty" => [0, 0],
            "tight" => [n / 2, n - n / 2],
            _ => [n, n],
        };
        let base = 100 + i as u64 * 100;
        let mut q = query("capacity_", &vec![3; n], caps, i % 2 == 1, base);
        q.constraints[n..n + caps[0] + caps[1]].reverse();
        q.constraints.push(c("go", [v(base), v(base + 50)]));
        q.constraints.push(c("outside", [v(base + 50)]));
        q.outputs.push(("external".into(), Var(base + 50)));
        q
    }
    fn execute(
        p: &chr_compiled::PreparedRuleset,
        q: Query,
        access: chr_compiled::Access,
        first: bool,
    ) -> Vec<Answer> {
        let mut run = p
            .start_search(q, chr_compiled::Policy::Global, access)
            .unwrap();
        let mut out = vec![];
        for _ in 0..200000 {
            match run.tick() {
                chr_compiled::SearchEvent::Complete(mut branch) => {
                    out.push(branch.engine.observe().unwrap());
                    if first {
                        return out;
                    }
                }
                chr_compiled::SearchEvent::Exhausted => return out,
                _ => (),
            }
        }
        panic!("ordinary service cutoff");
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
        assert_eq!(args.len(), 9);
        let head = args[6].as_str();
        let caller = args[7].as_str();
        let stop = args[8].as_str();
        assert!(
            ["need", "token"].contains(&head)
                && ["take", "later"].contains(&caller)
                && ["complete", "first"].contains(&stop)
        );
        let mode = args[1].as_str();
        let n = args[2].parse::<usize>().unwrap();
        let supply = args[3].as_str();
        let weight = args[4].parse::<usize>().unwrap();
        let reuse = args[5].parse::<usize>().unwrap();
        assert!(
            [
                "phase",
                "scan",
                "index",
                "specialized-scan",
                "specialized-index"
            ]
            .contains(&mode)
        );
        assert!([0, 2, 4].contains(&n) && [1, 2].contains(&weight) && [1, 4].contains(&reuse));
        assert!(["empty", "tight", "spare"].contains(&supply));
        if chr_compiled::COLLECT_METRICS || chr_observe::COLLECT_METRICS {
            panic!("measurement requires engine diagnostics disabled");
        }
        let mut records = Vec::with_capacity(8 + 4 * reuse);
        let mut answers = Vec::with_capacity(reuse);
        let mut counts = Vec::with_capacity(reuse);
        #[cfg(feature = "alloc-meter")]
        let baseline = meter::end(meter::begin()).live_end;
        let rules = measure(&mut records, "source_build", 0, || {
            let mut rules = source("capacity_", weight);
            if head == "token" {
                rules[3].removed.swap(0, 1);
            }
            rules.push(if caller == "take" {
                Rule::simplify(
                    "caller",
                    [c("go", [v(0), v(1)]), c("capacity_done", [v(2)])],
                    c("chosen", [v(2), v(0), v(1)]).into(),
                )
            } else {
                Rule::simplify(
                    "caller",
                    [c("go", [v(0), v(1)])],
                    and(vec![
                        c("capacity_pick1", [v(1)]).into(),
                        c("capacity_token", [atom("a")]).into(),
                        c("later", [v(0), v(1)]).into(),
                    ]),
                )
            });
            rules
        });
        let p = measure(&mut records, "prepare", 0, || {
            if mode == "phase" {
                let phase = capacity::phase::Prepared::new(&rules, 5).unwrap();
                let mut program = vec![Rule::simplify(
                    "$phase-bind",
                    [c("$phase-bind", [v(0), v(1)])],
                    eq(v(0), v(1)),
                )];
                program.extend(rules.clone());
                let caller = chr_compiled::PreparedRuleset::new(program, None)
                    .unwrap()
                    .specialize_inferred();
                Prepared::Phase(phase, caller)
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
            let q = measure(&mut records, "input_build", i, || make_query(n, supply, i));
            let first_only = stop == "first" && i == 0;
            let out = match &p {
                Prepared::Phase(phase, caller) => {
                    let continuations = measure(&mut records, "phase_solve", i, || {
                        phase.solve(&q, Default::default()).unwrap()
                    });
                    measure(
                        &mut records,
                        "caller_transport_execute_observe_drop",
                        i,
                        || {
                            let mut answers = vec![];
                            for mut next in continuations {
                                next.query.constraints.extend(
                                    next.bindings
                                        .into_iter()
                                        .map(|(v, t)| c("$phase-bind", [Term::Var(v), t])),
                                );
                                let mut got = execute(
                                    caller,
                                    next.query,
                                    chr_compiled::Access::Scan,
                                    first_only,
                                );
                                answers.append(&mut got);
                                if first_only && !answers.is_empty() {
                                    break;
                                }
                            }
                            answers
                        },
                    )
                }
                Prepared::Ordinary(p) => {
                    measure(&mut records, "setup_execute_observe_drop", i, || {
                        execute(
                            p,
                            q.clone(),
                            if mode.ends_with("index") {
                                chr_compiled::Access::Indexed
                            } else {
                                chr_compiled::Access::Scan
                            },
                            first_only,
                        )
                    })
                }
            };
            counts.push(out.len());
            answers.push(out);
            measure(&mut records, "input_drop", i, || drop(q));
        }
        measure(&mut records, "prepared_drop", 0, || drop(p));
        #[cfg(feature = "alloc-meter")]
        let validation_live = meter::end(meter::begin()).live_end;
        for (i, out) in answers.iter().enumerate() {
            let q = make_query(n, supply, i);
            let expected = checked(&rules, &q);
            if stop == "first" && i == 0 {
                assert_eq!(out.len(), usize::from(!expected.is_empty()));
                for a in out {
                    assert!(expected.iter().any(|b| chr_observe::equivalent(
                        a,
                        b,
                        &mut Default::default()
                    )));
                }
            } else {
                runtime_support::same_raw(out.clone(), expected);
            }
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
            "{{\"mode\":\"{mode}\",\"requests\":{n},\"supply\":\"{supply}\",\"weight\":{weight},\"reuse\":{reuse},\"counts\":{counts:?},\"head\":\"{head}\",\"caller\":\"{caller}\",\"stop\":\"{stop}\",\"diagnostics\":{},\"meter\":{},\"phases\":[{phases}]}}",
            cfg!(feature = "capacity-diagnostics"),
            cfg!(feature = "alloc-meter")
        );
    }
}
fn main() {
    gate::run()
}
