#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "support/continuation_source.rs"]
mod source;
use chr_compiled::experiment::meter;
use chr_persistent::continuations::{Machine, Step};
use std::collections::VecDeque;
fn main() {
    if chr_reuse::continuations::COLLECT_METRICS || chr_persistent::COLLECT_METRICS {
        panic!("allocation attribution requires counters disabled");
    }
    meter::self_check().unwrap();
    for family in ["exact", "rename", "history", "history-early", "distinct"] {
        for depth in [0, 16, 64] {
            for resource in [false, true] {
                for mode in ["exact", "alpha", "live"] {
                    let schema = source::Schema::new(family, resource);
                    let expected =
                        oracle::run(&schema.rules(), &schema.query(depth, false), 2_000_000);
                    assert_eq!(expected.len(), schema.answer_count());
                    let root = meter::begin();
                    let (mut machine, cursor) =
                        Machine::new(schema.rules(), schema.query(depth, false)).unwrap();
                    let mut queue = VecDeque::from([cursor]);
                    let mut answers = vec![];
                    let (mut steps, mut export, mut canonical, mut source_step) = (0, 0, 0, 0);
                    while let Some(cursor) = queue.pop_front() {
                        steps += 1;
                        assert!(steps < 2_000_000);
                        let start = meter::begin();
                        let key = machine.key(&cursor);
                        export += meter::end(start).requested_bytes;
                        let start = meter::begin();
                        let key = match mode {
                            "alpha" => key.alpha(),
                            "live" => key.alpha_live_history(),
                            _ => key,
                        };
                        canonical += meter::end(start).requested_bytes;
                        drop(key);
                        let start = meter::begin();
                        let event = machine.step(cursor);
                        source_step += meter::end(start).requested_bytes;
                        match event {
                            Step::Continue(c) => queue.push_back(c),
                            Step::Split(a, b) => {
                                queue.push_back(a);
                                queue.push_back(b);
                            }
                            Step::Answer(a) => answers.push(a),
                            Step::Failed => {}
                        }
                    }
                    oracle::same_raw(answers, expected.clone());
                    drop(queue);
                    drop(machine);
                    let done = meter::end(root);
                    assert_eq!(done.live_start, done.live_end);
                    println!(
                        "{{\"family\":\"{family}\",\"depth\":{depth},\"resource\":{resource},\"mode\":\"{mode}\",\"steps\":{steps},\"key_export_requested\":{export},\"canonical_requested\":{canonical},\"source_step_requested\":{source_step}}}"
                    );
                }
            }
        }
    }
}
