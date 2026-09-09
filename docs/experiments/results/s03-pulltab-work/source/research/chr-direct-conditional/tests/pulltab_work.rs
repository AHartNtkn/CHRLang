#![cfg(feature = "work-diagnostics")]
#[allow(dead_code)]
mod runtime_support;
use chr_direct_choice::demand::{Event, Prepared};
use chr_syntax::{Answer, Query, Rule, Var, atom, c, eq, or, t, v};

#[test]
fn registered_local_pulltab_work() {
    println!(
        "PULL_HEADER,family,consumers,independent,depth,reverse,pull,answers,ticks,nodes,calls,choices,births,results,obligations,lifts,expansions,force_entries,match_entries,lift_walk_entries"
    );
    let mut configurations = 0;
    for family in ["direct", "opaque", "nested"] {
        for consumers in [1, 4] {
            for independent in [false, true] {
                for depth in [0, 8, 32] {
                    for reverse in [false, true] {
                        let mut rules = vec![
                            Rule::simplify(
                                "choose",
                                [c("choose", [v(0)])],
                                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
                            ),
                            Rule::simplify("id", [c("id", [v(0), v(1)])], eq(v(1), v(0))),
                        ];
                        if family == "opaque" {
                            rules.push(Rule::simplify(
                                "use",
                                [c("use", [v(0), v(1)])],
                                eq(v(1), v(0)),
                            ));
                        } else {
                            for value in ["a", "b"] {
                                let pattern = if family == "nested" {
                                    t("box", [atom(value)])
                                } else {
                                    atom(value)
                                };
                                rules.push(Rule::simplify(
                                    value,
                                    [c("use", [pattern, v(0)])],
                                    eq(v(0), atom(value)),
                                ));
                            }
                        }
                        let producers = if independent { consumers } else { 1 };
                        let mut constraints = vec![];
                        let mut roots = vec![];
                        for producer in 0..producers {
                            let first = 100 + producer * 100;
                            constraints.push(c("choose", [v(first)]));
                            for step in 0..depth {
                                constraints.push(c("id", [v(first + step), v(first + step + 1)]));
                            }
                            roots.push(first + depth);
                        }
                        let mut outputs = vec![];
                        for consumer in 0..consumers {
                            let input = v(roots[if independent { consumer as usize } else { 0 }]);
                            let input = if family == "nested" {
                                t("box", [input])
                            } else {
                                input
                            };
                            constraints.push(c("use", [input, v(1000 + consumer)]));
                            outputs.push((format!("out{consumer}"), Var(1000 + consumer)));
                        }
                        if reverse {
                            constraints.reverse();
                        }
                        let query = Query {
                            constraints,
                            outputs,
                        };
                        let expected = (0..1 << producers)
                            .map(|mask| Answer {
                                outputs: (0..consumers)
                                    .map(|consumer| {
                                        let bit = if independent { consumer } else { 0 };
                                        (
                                            format!("out{consumer}"),
                                            atom(if mask & (1 << bit) == 0 { "a" } else { "b" }),
                                        )
                                    })
                                    .collect(),
                                residual: vec![],
                            })
                            .collect::<Vec<_>>();
                        runtime_support::same_raw(
                            runtime_support::run(&rules, &query, 200_000),
                            expected.clone(),
                        );
                        for pull in [false, true] {
                            let prepared = Prepared::new(rules.clone()).unwrap();
                            let prepared = if pull {
                                prepared.with_pull_tabs()
                            } else {
                                prepared
                            };
                            let mut run = prepared.start(query.clone()).unwrap();
                            let mut actual = vec![];
                            let mut exhausted = false;
                            let mut ticks = 0;
                            for _ in 0..100_000 {
                                ticks += 1;
                                match run.tick() {
                                    Event::Progress => {}
                                    Event::Answer(a) => actual.push(a),
                                    Event::Exhausted => {
                                        exhausted = true;
                                        break;
                                    }
                                }
                            }
                            assert!(exhausted, "candidate cutoff");
                            runtime_support::same_raw(actual, expected.clone());
                            let g = run.retained_graph();
                            let w = run.work();
                            let lifts = g.choices - g.births;
                            if !pull || family != "direct" {
                                assert_eq!(lifts, 0);
                            }
                            assert_eq!(g.calls, g.obligations);
                            println!(
                                "PULL,{family},{consumers},{independent},{depth},{reverse},{pull},{},{ticks},{},{},{},{},{},{},{lifts},{},{},{},{}",
                                expected.len(),
                                g.nodes,
                                g.calls,
                                g.choices,
                                g.births,
                                g.results,
                                g.obligations,
                                g.results - lifts,
                                w.force_entries,
                                w.match_entries,
                                w.lift_walk_entries
                            );
                        }
                        configurations += 1;
                    }
                }
            }
        }
    }
    assert_eq!(configurations, 72);
}
