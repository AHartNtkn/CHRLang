use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
use chr_reuse::continuations::{Mode, Search};
use chr_syntax::Answer;
#[allow(dead_code)]
#[path = "../examples/support/call_trace_source.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
fn ordered(a: &[Answer], b: &[Answer]) {
    assert_eq!(a.len(), b.len());
    for (a, b) in a.iter().zip(b) {
        assert!(
            chr_observe::equivalent(a, b, &mut Default::default()),
            "order differs: {a:?} / {b:?}"
        );
    }
}
#[test]
fn compiled_sources_keep_complete_answers_order_and_cancellation() {
    let mut queries = 0;
    let mut cancellations = 0;
    let mut held = vec![];
    for mode in ["scan", "indexed", "sealed"] {
        for reverse in [false, true] {
            for family in 0..4 {
                let (rs, _) = fixture::program(reverse, family);
                let mut prepared = PreparedRuleset::new(rs.clone(), None).unwrap();
                if mode == "sealed" {
                    prepared = prepared.specialize_inferred();
                }
                let access = if mode == "scan" {
                    Access::Scan
                } else {
                    Access::Indexed
                };
                for a in [0, 1, 4, 8] {
                    for b in [0, 1, 4, 8] {
                        for offset in [10, 1000] {
                            eprintln!("case={mode}/{reverse}/{family}/{a}/{b}/{offset}");
                            let q = fixture::input(a, b, offset);
                            let mut direct =
                                Search::new(rs.clone(), q.clone(), Mode::Direct).unwrap();
                            let expected = direct.advance(100_000);
                            assert!(expected.exhausted);
                            scalar::same_raw(
                                expected.answers.clone(),
                                scalar::run(&rs, &q, 100_000),
                            );
                            for cutoff in [0, 1, 5] {
                                let mut e = prepared
                                    .start_search(q.clone(), Policy::Global, access)
                                    .unwrap();
                                for _ in 0..cutoff {
                                    if let SearchEvent::Complete(mut branch) = e.tick() {
                                        let _ = branch.engine.observe().unwrap();
                                    }
                                }
                                drop(e);
                                cancellations += 1;
                            }
                            for first in [false, true] {
                                let mut e = prepared
                                    .start_search(q.clone(), Policy::Global, access)
                                    .unwrap();
                                let mut answers = vec![];
                                let mut terminal = false;
                                for _ in 0..100_000 {
                                    match e.tick() {
                                        SearchEvent::Complete(mut branch) => {
                                            answers.push(branch.engine.observe().unwrap());
                                            if first {
                                                terminal = true;
                                                break;
                                            }
                                        }
                                        SearchEvent::Exhausted => {
                                            terminal = true;
                                            break;
                                        }
                                        _ => (),
                                    }
                                }
                                assert!(terminal);
                                drop(e);
                                ordered(
                                    &answers,
                                    &expected.answers[..if first {
                                        expected.answers.len().min(1)
                                    } else {
                                        expected.answers.len()
                                    }],
                                );
                                held.push((answers, expected.answers.clone(), first));
                            }
                            queries += 1;
                        }
                    }
                }
            }
        }
    }
    for (a, b, first) in held {
        ordered(&a, &b[..if first { b.len().min(1) } else { b.len() }]);
    }
    assert_eq!(queries, 768);
    assert_eq!(cancellations, 2304);
    eprintln!(
        "complete_queries={queries} cancellation_restarts={cancellations} retained_outputs_checked=true"
    );
}
