#[cfg(feature = "alloc-meter")]
#[allow(unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[path = "support/projection_cost.rs"]
mod runtime;
use std::time::Instant;
struct Row {
    phase: &'static str,
    query: usize,
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn measure<T>(rows: &mut Vec<Row>, phase: &'static str, query: usize, f: impl FnOnce() -> T) -> T {
    #[cfg(feature = "alloc-meter")]
    let begin = meter::begin();
    let clock = Instant::now();
    let value = f();
    let ns = clock.elapsed().as_nanos();
    #[cfg(feature = "alloc-meter")]
    let memory = meter::end(begin);
    rows.push(Row {
        phase,
        query,
        ns,
        #[cfg(feature = "alloc-meter")]
        memory,
    });
    value
}
fn digest(h: u64, record: &runtime::Record) -> u64 {
    let mut h = h
        .wrapping_mul(6364136223846793005)
        .wrapping_add(record.0.len() as u64);
    for &v in &record.0 {
        h = h.wrapping_mul(1099511628211).wrapping_add(v as u64 + 1);
    }
    h.wrapping_mul(1099511628211)
        .wrapping_add(record.1 as u64)
        .wrapping_add((record.1 >> 64) as u64)
}
fn restriction(query: usize) -> Option<u8> {
    match query {
        1 => Some(0),
        2 => Some(1),
        _ => None,
    }
}
fn main() {
    if cfg!(feature = "metrics") {
        panic!("metrics-off required");
    }
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 7, "mode family size queries endpoint cancel");
    let mode = &args[1];
    assert!(runtime::MODES.contains(&mode.as_str()));
    let family = &args[2];
    let n = args[3].parse().unwrap();
    let queries = args[4].parse::<usize>().unwrap();
    assert!(matches!(queries, 1 | 4));
    let expanded = match args[5].as_str() {
        "weighted" => false,
        "expanded" => true,
        _ => panic!("endpoint"),
    };
    let cancel = match args[6].as_str() {
        "0" => false,
        "8" => true,
        _ => panic!("cancel"),
    };
    let mut expected = Vec::new();
    {
        let (p, visible) = runtime::source(family, n);
        let prepared = runtime::Prepared::new(mode, &p, &visible);
        for q in 0..queries {
            let want = runtime::oracle(&p, &visible, restriction(q));
            let want = if expanded {
                runtime::expand(want)
            } else {
                want
            };
            assert_eq!(
                prepared.start(restriction(q), expanded).collect::<Vec<_>>(),
                want,
                "complete preflight"
            );
            let prefix = if cancel {
                want.len().min(8)
            } else {
                want.len()
            };
            expected.push((prefix, want[..prefix].iter().fold(0, digest)));
        }
    }
    let mut rows = Vec::with_capacity(32);
    let mut observed = Vec::with_capacity(queries);
    #[cfg(feature = "alloc-meter")]
    let owner = meter::begin();
    let (p, visible) = measure(&mut rows, "source", 0, || runtime::source(family, n));
    let prepared = measure(&mut rows, "prepare", 0, || {
        runtime::Prepared::new(mode, &p, &visible)
    });
    measure(&mut rows, "source_dispose", 0, || drop((p, visible)));
    for q in 0..queries {
        let mut output = measure(&mut rows, "query_start", q, || {
            prepared.start(restriction(q), expanded)
        });
        let (mut count, mut hash) = measure(&mut rows, "first", q, || match output.next() {
            Some(record) => (1, digest(0, &record)),
            None => (0, 0),
        });
        measure(&mut rows, "remaining", q, || {
            let limit = if cancel { 8 } else { 100_001 };
            while count < limit {
                let Some(record) = output.next() else {
                    break;
                };
                count += 1;
                hash = digest(hash, &record);
            }
            assert!(count <= 100_000, "output bound");
        });
        measure(&mut rows, "query_dispose", q, || drop(output));
        observed.push((count, hash));
    }
    measure(&mut rows, "prepare_dispose", 0, || drop(prepared));
    #[cfg(feature = "alloc-meter")]
    let total_memory = meter::end(owner);
    #[cfg(feature = "alloc-meter")]
    assert_eq!(
        total_memory.live_end, total_memory.live_start,
        "final owner conservation"
    );
    assert_eq!(observed, expected, "measured complete/prefix outputs");
    let total: u128 = rows.iter().map(|r| r.ns).sum();
    let observed_json = observed
        .iter()
        .map(|(n, h)| format!("[{n},{h}]"))
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"n\":{n},\"queries\":{queries},\"endpoint\":\"{}\",\"cancel\":{},\"total_ns\":{total},\"observed\":[{observed_json}],\"metered\":{}}}",
        args[5],
        args[6],
        cfg!(feature = "alloc-meter")
    );
    for row in rows {
        #[cfg(feature = "alloc-meter")]
        println!(
            "{{\"phase\":\"{}\",\"query\":{},\"ns\":{},\"memory\":{}}}",
            row.phase,
            row.query,
            row.ns,
            row.memory.json()
        );
        #[cfg(not(feature = "alloc-meter"))]
        println!(
            "{{\"phase\":\"{}\",\"query\":{},\"ns\":{}}}",
            row.phase, row.query, row.ns
        );
    }
}
