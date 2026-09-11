#[allow(dead_code, unused_imports)]
mod gate {
    include!("../tests/consumer_pressure.rs");
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

    pub fn run() {
        let args = std::env::args().collect::<Vec<_>>();
        assert_eq!(args.len(), 5);
        let mode = args[1].as_str();
        assert!(["dependencies", "templates"].contains(&mode));
        let resource = args[2].parse::<bool>().unwrap();
        let marker = args[3].parse::<bool>().unwrap();
        let target = args[4].parse::<usize>().unwrap();
        assert!([128, 512].contains(&target));
        let p = Owner::new(
            mode,
            Schema {
                family: "aliases",
                resource: false,
                fail_tail: false,
                work: 0,
                payload: 0,
            },
            rules(resource),
        );
        let mut q = query(resource, 0);
        let mut want = expected(0);
        if !marker {
            q.constraints.retain(|c| c.name != "query");
            want.residual.clear();
        }
        let mut run = p.start(q);
        let mut held = Vec::new();
        for calls in 1..=200_000_000 {
            match run.tick() {
                Event::Answer(a) => {
                    oracle::same_raw(vec![a.clone()], vec![want.clone()]);
                    held.push(a);
                    let answers = held.len();
                    if [1, 8, 32, 64, 128, 512].contains(&answers) {
                        let Service::Existing(Running::Graph(g)) = &run else {
                            unreachable!()
                        };
                        let w = g.work();
                        let r = g.retained_graph();
                        println!(
                            "{{\"answers\":{answers},\"calls\":{calls},\"force\":{},\"validation\":{},\"matches\":{},\"dependencies\":{},\"candidates\":{},\"template_hits\":{},\"nodes\":{},\"results\":{},\"obligations\":{}}}",
                            w.force_entries,
                            w.validation_entries,
                            w.match_entries,
                            w.dependency_entries,
                            w.resource_candidates,
                            w.template_hits,
                            r.nodes,
                            r.results,
                            r.obligations
                        );
                    }
                    if answers == target {
                        drop(run);
                        drop(p);
                        for a in held {
                            oracle::same_raw(vec![a], vec![want.clone()]);
                        }
                        return;
                    }
                }
                Event::Done => panic!("continuing source exhausted"),
                Event::Progress => (),
            }
        }
        panic!("registered service cutoff")
    }
}
fn main() {
    gate::run();
}
