#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "support/call_cost.rs"]
mod runtime;
#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use runtime::{Event, Prepared, Schema};
use std::time::Instant;
struct Row {
    phase: &'static str,
    query: usize,
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn measure<T>(phase: &'static str, query: usize, f: impl FnOnce() -> T) -> (T, Row) {
    #[cfg(feature = "alloc-meter")]
    let begin = meter::begin();
    let clock = Instant::now();
    let value = f();
    let ns = clock.elapsed().as_nanos();
    (
        value,
        Row {
            phase,
            query,
            ns,
            #[cfg(feature = "alloc-meter")]
            memory: meter::end(begin),
        },
    )
}
fn main() {
    if chr_reuse::continuations::COLLECT_METRICS
        || chr_compiled::COLLECT_METRICS
        || chr_compiled::COLLECT_KERNEL_METRICS
        || chr_observe::COLLECT_METRICS
    {
        panic!("counter-free runner required");
    }
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(
        args.len(),
        9,
        "mode build depth tail payload queries distinct cancel"
    );
    let mode = &args[1];
    let schema = Schema {
        build: args[2] == "1",
        depth: args[3].parse().unwrap(),
        tail: args[4].parse().unwrap(),
        payload: args[5].parse().unwrap(),
        distinct: args[7] == "1",
    };
    let queries: usize = args[6].parse().unwrap();
    let cancel = args[8] == "1";
    assert!(
        matches!(schema.depth, 0 | 32)
            && matches!(schema.tail, 0 | 8)
            && matches!(schema.payload, 0 | 8)
            && matches!(queries, 1 | 8)
    );
    // Independent full-answer validation and one unmeasured preparation/query pass.
    {
        let mut p = Prepared::new(mode, schema);
        for q in 0..queries {
            let query = schema.query(q);
            let expected = oracle::run(&schema.rules(), &query, 2_000_000);
            let mut run = p.start(query);
            let mut actual = vec![];
            let mut done = false;
            for _ in 0..2_000_000 {
                match p.tick(&mut run) {
                    Event::Progress => (),
                    Event::Answer(a) => actual.push(a),
                    Event::Done => {
                        done = true;
                        break;
                    }
                }
            }
            assert!(done);
            oracle::same_raw(actual, expected);
        }
    }
    let mut rows = Vec::with_capacity(64);
    let mut answers = Vec::with_capacity(2);
    let mut first = [0u128; 8];
    let mut counts = [0usize; 8];
    #[cfg(feature = "alloc-meter")]
    let owner = meter::begin();
    let (mut p, row) = measure("prepare", 0, || Prepared::new(mode, schema));
    rows.push(row);
    for q in 0..queries {
        let (query, row) = measure("input", q, || schema.query(q));
        rows.push(row);
        let (mut run, row) = measure("setup", q, || p.start(query));
        rows.push(row);
        let ((), row) = measure("execute_observe", q, || {
            let clock = Instant::now();
            let mut done = false;
            for _ in 0..2_000_000 {
                match p.tick(&mut run) {
                    Event::Progress => (),
                    Event::Answer(a) => {
                        if answers.is_empty() {
                            first[q] = clock.elapsed().as_nanos();
                        }
                        answers.push(a);
                        if cancel {
                            break;
                        }
                    }
                    Event::Done => {
                        done = true;
                        break;
                    }
                }
            }
            assert!(done || cancel);
        });
        rows.push(row);
        counts[q] = answers.len();
        assert_eq!(counts[q], if cancel { 1 } else { 2 });
        let ((), row) = measure("caller_drop", q, || drop(run));
        rows.push(row);
        let ((), row) = measure("consumer_drop", q, || answers.clear());
        rows.push(row);
    }
    let ((), row) = measure("prepared_drop", 0, || drop(p));
    rows.push(row);
    #[cfg(feature = "alloc-meter")]
    {
        let end = meter::end(owner);
        assert_eq!(end.live_start, end.live_end, "lifecycle owner leak");
    }
    print!("{{\"mode\":\"{mode}\",\"build\":{},\"depth\":{},\"tail\":{},\"payload\":{},\"queries\":{queries},\"distinct\":{},\"cancel\":{cancel},\"first\":{:?},\"answers\":{:?},\"rows\":[",schema.build,schema.depth,schema.tail,schema.payload,schema.distinct,&first[..queries],&counts[..queries]);
    for (i, row) in rows.iter().enumerate() {
        if i > 0 {
            print!(",");
        }
        #[cfg(feature = "alloc-meter")]
        let memory = row.memory.json();
        #[cfg(not(feature = "alloc-meter"))]
        let memory = "null";
        print!(
            "{{\"phase\":\"{}\",\"query\":{},\"ns\":{},\"memory\":{memory}}}",
            row.phase, row.query, row.ns
        );
    }
    println!("]}}");
}
