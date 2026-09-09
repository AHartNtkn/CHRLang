#[cfg(feature = "alloc-meter")]
#[allow(unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[path = "support/finite_oracle.rs"]
mod oracle;
#[path = "support/finite_cost.rs"]
mod runtime;
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
    if chr_structural::finite::COLLECT_METRICS {
        panic!("counter-free measurement required");
    }
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 6, "mode family width retain cancel");
    let mode = &args[1];
    let family = *runtime::FAMILIES.iter().find(|&&f| f == args[2]).unwrap();
    let width = args[3].parse().unwrap();
    assert!(matches!(width, 0 | 4 | 6));
    let retain = args[4] == "1";
    let cancel = args[5] == "1";
    let schema = Schema { width, family };
    let mut expected_counts = [0; 4];
    {
        let p = Prepared::new(mode, schema);
        for (q, count) in expected_counts.iter_mut().enumerate() {
            let expected = oracle::expected(schema, q);
            *count = expected.len();
            let mut run = p.start(p.request(q));
            let mut actual = vec![];
            let mut done = false;
            for _ in 0..2_000_000 {
                match run.tick() {
                    Event::Progress => (),
                    Event::Answer(a) => {
                        assert_eq!(a.multiplicity, 1);
                        actual.push(a.term);
                    }
                    Event::Done => {
                        done = true;
                        break;
                    }
                }
            }
            assert!(done, "preflight cutoff");
            actual.sort();
            assert_eq!(actual, expected);
        }
    }
    let mut rows = Vec::with_capacity(32);
    let mut retained = Vec::with_capacity(1 << width);
    let mut first = [0u128; 4];
    let mut counts = [0usize; 4];
    #[cfg(feature = "alloc-meter")]
    let owner = meter::begin();
    let (p, row) = measure("prepare", 0, || Prepared::new(mode, schema));
    rows.push(row);
    for q in 0..4 {
        let (request, row) = measure("input", q, || p.request(q));
        rows.push(row);
        let (mut run, row) = measure("setup", q, || p.start(request));
        rows.push(row);
        let ((), row) = measure("execute_observe", q, || {
            let start = Instant::now();
            let mut done = false;
            for _ in 0..2_000_000 {
                match run.tick() {
                    Event::Progress => (),
                    Event::Answer(a) => {
                        assert_eq!(a.multiplicity, 1);
                        if counts[q] == 0 {
                            first[q] = start.elapsed().as_nanos();
                        }
                        counts[q] += 1;
                        if retain {
                            retained.push(a);
                        } else {
                            drop(a);
                        }
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
            assert!(done || cancel && counts[q] == 1, "measured cutoff");
        });
        rows.push(row);
        assert_eq!(
            counts[q],
            if cancel {
                expected_counts[q].min(1)
            } else {
                expected_counts[q]
            }
        );
        let ((), row) = measure("query_drop", q, || drop(run));
        rows.push(row);
        let ((), row) = measure("consumer_drop", q, || retained.clear());
        rows.push(row);
    }
    let ((), row) = measure("prepared_drop", 0, || drop(p));
    rows.push(row);
    #[cfg(feature = "alloc-meter")]
    {
        let reading = meter::end(owner);
        assert_eq!(reading.live_start, reading.live_end, "owner leak");
    }
    print!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"width\":{width},\"retain\":{retain},\"cancel\":{cancel},\"answers\":{counts:?},\"first\":{first:?},\"rows\":["
    );
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
