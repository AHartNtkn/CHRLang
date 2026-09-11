#![cfg(feature = "work-diagnostics")]
#[allow(dead_code)]
mod runtime_support;
use chr_direct_choice::demand::{Event, Prepared, Reuse};
#[path = "runtime_support/dependency_source.rs"]
mod source;
use source::{query, rules};

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
