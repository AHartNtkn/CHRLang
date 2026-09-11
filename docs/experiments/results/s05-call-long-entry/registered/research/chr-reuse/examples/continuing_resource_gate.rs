#[allow(dead_code, unused_imports)]
mod gate {
    include!("../tests/consumer_pressure.rs");
    pub fn run() {
        let args = std::env::args().collect::<Vec<_>>();
        assert_eq!(args.len(), 3);
        let mode = args[1].as_str();
        let resource = match args[2].as_str() {
            "pure" => false,
            "resource" => true,
            _ => panic!("source"),
        };
        let reclaiming = mode.ends_with("-reclaim");
        let base = mode.strip_suffix("-reclaim").unwrap_or(mode);
        assert!(["dependencies", "templates"].contains(&base));
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
        let p = Owner::new(base, schema, rules);
        let mut constraints = vec![c("stream", [v(10)])];
        if resource {
            constraints.extend([c("finish", [v(10), v(11)]), c("token", [])]);
        }
        let mut run = p.start(Query {
            constraints,
            outputs: vec![("answer".into(), Var(if resource { 11 } else { 10 }))],
        });
        let expected = Answer {
            outputs: vec![("answer".into(), t("pair", [v(99), v(99)]))],
            residual: vec![],
        };
        let mut held = Vec::new();
        let mut removed = 0;
        let mut claims = 0;
        let mut finished = false;
        for calls in 1..=2_000_000 {
            match run.tick() {
                Event::Answer(a) => {
                    oracle::same_raw(vec![a.clone()], vec![expected.clone()]);
                    held.push(a);
                    if reclaiming {
                        let Service::Existing(Running::Graph(g)) = &mut run else {
                            panic!("graph")
                        };
                        let r = g.reclaim_incompatible_supports();
                        removed += r.results;
                        claims += r.consumption_claims;
                    }
                    if [1, 8, 32, 64].contains(&held.len()) {
                        println!(
                            "{{\"answers\":{},\"calls\":{calls},\"removed_results\":{removed},\"removed_claims\":{claims}}}",
                            held.len()
                        );
                    }
                    if held.len() == 64 {
                        finished = true;
                        break;
                    }
                }
                Event::Done => panic!("continuing source exhausted"),
                Event::Progress => (),
            }
        }
        assert!(finished, "service cutoff");
        drop(run);
        drop(p);
        for a in held {
            oracle::same_raw(vec![a], vec![expected.clone()]);
        }
        if reclaiming {
            assert_eq!(removed > 0, resource, "reclamation opportunity");
        }
        println!("{{\"validated\":true}}");
    }
}
fn main() {
    gate::run();
}
