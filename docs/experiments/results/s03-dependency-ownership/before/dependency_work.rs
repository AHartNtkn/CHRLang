#![cfg(feature = "work-diagnostics")]
#[allow(dead_code)]
mod runtime_support;
use chr_direct_choice::demand::{Event, Prepared, Reuse};
use chr_syntax::{Query, Rule, Var, atom, c, eq, v};

fn rules(plain: bool) -> Vec<Rule> {
    let mut rules = vec![Rule::simplify(
        "end",
        [c("end", [v(0), v(1)])],
        eq(v(1), v(0)),
    )];
    if !plain {
        rules.push(Rule::simplify(
            "step",
            [c("step", [v(0), v(1), v(2)]), c("link", [v(0), v(1)])],
            eq(v(2), v(1)),
        ));
    }
    rules
}
fn query(size: u64, kind: &str, reverse: bool, value: &str) -> Query {
    let desired = atom(value);
    let yielded = if kind.ends_with("miss") {
        atom("miss")
    } else {
        desired.clone()
    };
    let mut constraints = vec![];
    for i in 0..size {
        if kind == "plain" {
            constraints.push(c("end", [desired.clone(), v(100 + i)]));
        } else {
            let index = atom(&format!("i{i}"));
            constraints.push(c("step", [index.clone(), desired.clone(), v(100 + i)]));
            constraints.push(c(
                "link",
                [
                    index,
                    if kind.starts_with("delayed") {
                        v(101 + i)
                    } else {
                        yielded.clone()
                    },
                ],
            ));
        }
    }
    constraints.push(c("end", [yielded, v(100 + size)]));
    if reverse {
        constraints.reverse();
    }
    Query {
        constraints,
        outputs: (0..=size)
            .map(|i| (format!("out{i}"), Var(100 + i)))
            .collect(),
    }
}
#[test]
fn dependency_validation_work() {
    println!(
        "DEPENDENCY_HEADER,kind,size,reverse,reuse,query,answers,ticks,force,match,validation_passes,validation_entries"
    );
    let mut sessions = 0;
    for size in [8, 32, 128] {
        for reverse in [false, true] {
            for kind in [
                "plain",
                "known-hit",
                "known-miss",
                "delayed-hit",
                "delayed-miss",
            ] {
                let rules = rules(kind == "plain");
                let prepared: Vec<_> = [
                    Reuse::CurrentContext,
                    Reuse::StaticBirth,
                    Reuse::MatchDependencies,
                ]
                .into_iter()
                .map(|reuse| {
                    (reuse, {
                        let p = Prepared::with_reuse(rules.clone(), reuse).unwrap();
                        if std::env::var("MISS_REUSE").as_deref() == Ok("on") {
                            p.with_miss_reuse()
                        } else {
                            p
                        }
                    })
                })
                .collect();
                let compiled = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
                for value in ["a", "b"] {
                    let query = query(size, kind, reverse, value);
                    let expected = runtime_support::run(&rules, &query, 2_000_000);
                    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                        let mut search = compiled
                            .start_search(query.clone(), chr_compiled::Policy::Global, access)
                            .unwrap();
                        let mut actual = vec![];
                        let mut done = false;
                        let mut control_ticks = 0;
                        for _ in 0..2_000_000 {
                            control_ticks += 1;
                            match search.tick() {
                                chr_compiled::SearchEvent::Complete(mut b) => {
                                    actual.push(b.engine.observe().unwrap())
                                }
                                chr_compiled::SearchEvent::Exhausted => {
                                    done = true;
                                    break;
                                }
                                _ => {}
                            }
                        }
                        assert!(done, "compiled cutoff {kind} {size}");
                        runtime_support::same_raw(actual, expected.clone());
                        println!(
                            "CONTROL,{kind},{size},{reverse},{access:?},{value},{control_ticks}"
                        );
                    }
                    for (reuse, prepared) in &prepared {
                        let mut run = prepared.start(query.clone()).unwrap();
                        let mut actual = vec![];
                        let mut done = false;
                        let mut ticks = 0;
                        for _ in 0..2_000_000 {
                            ticks += 1;
                            match run.tick() {
                                Event::Answer(a) => actual.push(a),
                                Event::Exhausted => {
                                    done = true;
                                    break;
                                }
                                Event::Progress => {}
                            }
                        }
                        assert!(done, "demand cutoff {kind} {size}");
                        let answers = actual.len();
                        runtime_support::same_raw(actual, expected.clone());
                        let work = run.work();
                        println!(
                            "DEPENDENCY,{kind},{size},{reverse},{reuse:?},{value},{answers},{ticks},{},{},{},{}",
                            work.force_entries,
                            work.match_entries,
                            work.validation_passes,
                            work.validation_entries
                        );
                        println!(
                            "MISS,{kind},{size},{reverse},{reuse:?},{value},{},{},{}",
                            work.miss_lookups, work.miss_hits, work.miss_inserts
                        );
                        sessions += 1;
                    }
                }
            }
        }
    }
    assert_eq!(sessions, 180);
}
