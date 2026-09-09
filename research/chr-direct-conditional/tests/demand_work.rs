#[allow(dead_code)]
mod runtime_support;
use chr_syntax::{Answer, Query, Var, atom, c, t, v};
#[path = "../experiments/demand_source.rs"]
#[allow(dead_code)]
mod source;
use source::rules;
#[test]
fn registered_work_attribution() {
    println!(
        "WORK_HEADER,policy,family,bits,depth,work_first,resource,answers,graph_boxes,demand_boxes,demand_work_results"
    );
    let mut cases = 0;
    for (family, bits) in [
        ("plain", 0),
        ("opaque", 1),
        ("opaque", 3),
        ("discriminate", 1),
        ("discriminate", 3),
    ] {
        for n in [0, 1, 8, 32] {
            for first in [false, true] {
                for resource in [false, true] {
                    let rules = rules(family == "discriminate", bits, resource);
                    let depth = (0..n).fold(atom("z"), |x, _| t("s", [x]));
                    let work = c(
                        if family == "discriminate" {
                            "gate"
                        } else {
                            "work"
                        },
                        [
                            depth,
                            t(
                                "pack",
                                (0..bits).map(|i| v(100 + i as u64)).collect::<Vec<_>>(),
                            ),
                            v(1000),
                        ],
                    );
                    let mut constraints = (0..bits)
                        .map(|i| c("choose", [v(100 + i as u64)]))
                        .collect::<Vec<_>>();
                    if first {
                        constraints.insert(0, work)
                    } else {
                        constraints.push(work)
                    }
                    if resource {
                        constraints.extend([c("take", [v(1000), v(1001)]), c("token", [])]);
                    }
                    let output = Var(if resource { 1001 } else { 1000 });
                    let query = Query {
                        constraints,
                        outputs: vec![("x".into(), output), ("again".into(), output)],
                    };
                    let expected = (0..1 << bits)
                        .map(|mask| {
                            let pack = t(
                                "pack",
                                (0..bits)
                                    .map(|i| atom(if mask & (1 << i) == 0 { "a" } else { "b" }))
                                    .collect::<Vec<_>>(),
                            );
                            let value = (0..n).fold(pack, |x, _| t("box", [x]));
                            Answer {
                                outputs: vec![("x".into(), value.clone()), ("again".into(), value)],
                                residual: vec![],
                            }
                        })
                        .collect::<Vec<_>>();
                    runtime_support::same_raw(
                        runtime_support::run(&rules, &query, 200_000),
                        expected.clone(),
                    );
                    let mut graph = chr_direct_choice::engine::PreparedRuleset::new(rules.clone())
                        .unwrap()
                        .start(query.clone())
                        .unwrap();
                    let mut answers = vec![];
                    let mut done = false;
                    for _ in 0..100_000 {
                        match graph.tick() {
                            chr_direct_choice::engine::Event::Progress => {}
                            chr_direct_choice::engine::Event::Answer(a) => answers.push(a),
                            chr_direct_choice::engine::Event::Exhausted => {
                                done = true;
                                break;
                            }
                        }
                    }
                    assert!(done);
                    runtime_support::same_raw(answers, expected.clone());
                    for policy in [
                        chr_direct_choice::demand::Reuse::CurrentContext,
                        chr_direct_choice::demand::Reuse::StaticBirth,
                    ] {
                        let mut demand =
                            chr_direct_choice::demand::Prepared::with_reuse(rules.clone(), policy)
                                .unwrap()
                                .start(query.clone())
                                .unwrap();
                        let mut answers = vec![];
                        let mut done = false;
                        for _ in 0..100_000 {
                            match demand.tick() {
                                chr_direct_choice::demand::Event::Progress => {}
                                chr_direct_choice::demand::Event::Answer(a) => answers.push(a),
                                chr_direct_choice::demand::Event::Exhausted => {
                                    done = true;
                                    break;
                                }
                            }
                        }
                        assert!(done);
                        runtime_support::same_raw(answers, expected.clone());
                        if family == "opaque"
                            && policy == chr_direct_choice::demand::Reuse::StaticBirth
                        {
                            assert_eq!(
                                demand.retained_constructors().get("box"),
                                graph.retained_constructors().get("box"),
                                "opaque static matches should not repeat a common call expansion in sibling contexts"
                            );
                        }
                        println!(
                            "WORK,{policy:?},{family},{bits},{n},{first},{resource},{},{},{},{}",
                            expected.len(),
                            graph
                                .retained_constructors()
                                .get("box")
                                .copied()
                                .unwrap_or(0),
                            demand
                                .retained_constructors()
                                .get("box")
                                .copied()
                                .unwrap_or(0),
                            demand
                                .retained_application_results()
                                .get("work")
                                .copied()
                                .unwrap_or(0)
                        );
                    }
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 80);
}
