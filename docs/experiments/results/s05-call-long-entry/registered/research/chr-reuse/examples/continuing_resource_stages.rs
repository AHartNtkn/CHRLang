#[allow(dead_code, unused_imports)]
mod gate {
    include!("../tests/consumer_pressure.rs");
    pub fn run() {
        let args = std::env::args().collect::<Vec<_>>();
        assert_eq!(args.len(), 3);
        let resource = args[1].parse::<bool>().unwrap();
        let target = args[2].parse::<usize>().unwrap();
        assert!([128, 256, 512].contains(&target));
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
        let schema = Schema {
            family: "aliases",
            resource: false,
            fail_tail: false,
            work: 0,
            payload: 0,
        };
        let p = Owner::new("conditional", schema, rules);
        let mut run = p.start(Query {
            constraints: {
                let mut cs = vec![c("stream", [v(10)]), c("query", [chr_syntax::atom("q0")])];
                if resource {
                    cs.extend([c("finish", [v(10), v(11)]), c("token", [])]);
                }
                cs
            },
            outputs: vec![("answer".into(), Var(if resource { 11 } else { 10 }))],
        });
        let mut answers = 0;
        let mut stages = [0usize; 7];
        let mut counts = [[0u64; 6]; 7];
        let mut held = Vec::new();
        fn values(p: chr_direct_conditional::equality::EqualityProbe) -> [u64; 6] {
            [
                p.walker_ticks,
                p.support_ticks,
                p.binding_probes,
                p.empty,
                p.full,
                p.partial,
            ]
        }
        for calls in 1..=1_000_000_000 {
            let Service::Conditional(engine) = &run else {
                unreachable!()
            };
            let stage = engine.allocation_stage();
            stages[stage] += 1;
            let before = values(engine.store().probe());
            let event = run.tick();
            let Service::Conditional(engine) = &run else {
                unreachable!()
            };
            let after = values(engine.store().probe());
            for i in 0..6 {
                counts[stage][i] += after[i] - before[i];
            }
            match event {
                Event::Answer(a) => {
                    oracle::same_raw(
                        vec![a.clone()],
                        vec![Answer {
                            outputs: vec![("answer".into(), t("pair", [v(99), v(99)]))],
                            residual: vec![c("query", [chr_syntax::atom("q0")])],
                        }],
                    );
                    held.push(a);
                    answers += 1;
                    if [1, 8, 32, 64, 128, 256, 512].contains(&answers) {
                        let Service::Conditional(engine) = &run else {
                            unreachable!()
                        };
                        println!(
                            "{{\"answers\":{answers},\"calls\":{calls},\"stages\":{stages:?},\"counts\":{counts:?},\"backlog\":{},\"supports\":{},\"variables\":{},\"occurrences\":{}}}",
                            engine.observation_backlog(),
                            engine.supports().node_count(),
                            engine.store().variable_count(),
                            engine.resources().occurrences().len()
                        );
                    }
                    if answers == target {
                        drop(run);
                        drop(p);
                        for a in held {
                            oracle::same_raw(
                                vec![a],
                                vec![Answer {
                                    outputs: vec![("answer".into(), t("pair", [v(99), v(99)]))],
                                    residual: vec![c("query", [chr_syntax::atom("q0")])],
                                }],
                            );
                        }
                        return;
                    }
                }
                Event::Done => panic!("continuing source exhausted"),
                Event::Progress => (),
            }
        }
        panic!("probe cutoff after {answers} answers");
    }
}
fn main() {
    gate::run();
}
