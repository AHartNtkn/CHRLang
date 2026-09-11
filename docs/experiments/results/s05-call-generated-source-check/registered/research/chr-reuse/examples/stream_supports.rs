//! Non-mutating support inventory; no timing or heap measurement.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "support/stream_source.rs"]
mod source;
use chr_direct_choice::demand::{Event, Prepared, Reuse, Run};
fn emit(run: &Run, label: &str, answers: usize) {
    let s = run.future_supports();
    let g = run.retained_graph();
    println!(
        "{{\"label\":\"{label}\",\"answers\":{answers},\"tasks\":{},\"nodes\":{},\"results\":{},\"dead_results\":{},\"obligations\":{},\"dead_obligations\":{},\"births\":{},\"dead_births\":{},\"claims\":{},\"dead_claims\":{}}}",
        s.pending_tasks,
        g.nodes,
        s.results,
        s.dead_results,
        s.obligations,
        s.dead_obligations,
        s.births,
        s.dead_births,
        s.consumption_claims,
        s.dead_consumption_claims
    );
}
fn main() {
    for family in ["repeated", "distinct", "aliases"] {
        for n in [16, 64] {
            for resource in [false, true] {
                for fail_tail in [false, true] {
                    for templates in [false, true] {
                        let schema = source::Schema {
                            family,
                            resource,
                            fail_tail,
                            work: 4,
                            payload: 8,
                        };
                        let p =
                            Prepared::with_reuse(schema.rules(), Reuse::MatchDependencies).unwrap();
                        let p = if templates {
                            p.with_derivation_templates()
                        } else {
                            p
                        };
                        for q in 0..2 {
                            println!(
                                "{{\"family\":\"{family}\",\"depth\":{},\"resource\":{resource},\"fail_tail\":{fail_tail},\"templates\":{templates},\"reverse\":{}}}",
                                n + q,
                                q == 1
                            );
                            let query = schema.query(n + q, q == 1);
                            let expected = oracle::run(&schema.rules(), &query, 2_000_000);
                            oracle::same_raw(expected.clone(), schema.expected(n + q));
                            let mut run = p.start(query).unwrap();
                            emit(&run, "setup", 0);
                            let mut answers = vec![];
                            let mut done = false;
                            for _ in 0..2_000_000 {
                                match run.tick() {
                                    Event::Answer(a) => {
                                        answers.push(a);
                                        if [1, 4, 16, 64].contains(&answers.len()) {
                                            emit(&run, "delivery", answers.len());
                                        }
                                    }
                                    Event::Exhausted => {
                                        done = true;
                                        break;
                                    }
                                    Event::Progress => (),
                                }
                            }
                            assert!(done);
                            emit(&run, "exhausted", answers.len());
                            oracle::same_raw(answers, expected);
                        }
                    }
                }
            }
        }
    }
}
