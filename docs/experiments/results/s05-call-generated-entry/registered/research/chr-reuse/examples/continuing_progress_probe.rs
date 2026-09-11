#[allow(dead_code, unused_imports)]
mod gate {
    include!("../tests/consumer_pressure.rs");
    pub fn run() {
        let args = std::env::args().collect::<Vec<_>>();
        assert_eq!(args.len(), 2);
        let mode = args[1].as_str();
        let reclaiming = mode.ends_with("-reclaim");
        let base = mode.strip_suffix("-reclaim").unwrap_or(mode);
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
            resource: false,
            fail_tail: false,
            work: 0,
            payload: 0,
        };
        let p = if base == "conditional-inferred" {
            Owner::Conditional(
                chr_direct_conditional::engine::PreparedRuleset::with_head_contract(
                    rules,
                    None,
                    chr_direct_conditional::engine::HeadAdmission::Optional,
                )
                .unwrap(),
            )
        } else {
            Owner::new(base, schema, rules)
        };
        let mut run = p.start(Query {
            constraints: vec![c("stream", [v(10)])],
            outputs: vec![("answer".into(), Var(10))],
        });
        let mut answers = 0;
        let mut removed = 0;
        for calls in 1..=20_000_000 {
            match run.tick() {
                Event::Answer(a) => {
                    oracle::same_raw(
                        vec![a],
                        vec![Answer {
                            outputs: vec![("answer".into(), t("pair", [v(99), v(99)]))],
                            residual: vec![],
                        }],
                    );
                    answers += 1;
                    if reclaiming {
                        let Service::Existing(Running::Graph(g)) = &mut run else {
                            panic!("graph required")
                        };
                        removed += g.reclaim_incompatible_supports().results;
                    }
                    if [1, 8, 32, 64, 128, 256, 512].contains(&answers) {
                        let equality_ticks = if let Service::Conditional(e) = &run {
                            e.stats().equality_ticks
                        } else {
                            0
                        };
                        println!(
                            "{{\"answers\":{answers},\"calls\":{calls},\"removed\":{removed},\"equality_ticks\":{equality_ticks}}}"
                        );
                    }
                    if answers == 512 {
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
