//! Continuous source service with separate preparation, first output and disposal.
#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use chr_restoration::{
    Mode, Prepared, Step,
    reunion::{AttemptPolicy, PreparedPhase},
};
use chr_syntax::{Answer, Query, Rule};
use std::{sync::Arc, time::Instant};
#[path = "support/repeated_source.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
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
    assert!(rows.len() < rows.capacity());
    rows.push(Row {
        phase,
        query,
        ns,
        #[cfg(feature = "alloc-meter")]
        memory,
    });
    value
}
struct Config<'a> {
    family: &'a str,
    depth: usize,
    reuse: usize,
    keep: bool,
    cancel: bool,
    expected: Vec<Vec<Answer>>,
}
fn next<E>(engine: &mut E, advance: &impl Fn(&mut E) -> Step, steps: &mut usize) -> Option<Answer> {
    while *steps < 200_000 {
        *steps += 1;
        match advance(engine) {
            Step::Progress => (),
            Step::Answer(a) => return Some(a),
            Step::Exhausted => return None,
        }
    }
    panic!("service bound")
}
fn consume(answer: Answer, query: usize, keep: bool, retained: &mut Vec<(usize, Answer)>) {
    let answer = std::hint::black_box(answer);
    if keep {
        retained.push((query, answer));
    }
}
fn run<P, E>(
    config: &Config<'_>,
    prepare: impl Fn(&[Rule], usize) -> Arc<P>,
    start: impl Fn(&Arc<P>, &Query) -> E,
    advance: impl Fn(&mut E) -> Step,
) {
    {
        let (rules, local) = fixture::source(config.family, config.depth);
        let warm = prepare(&rules, local);
        for q in 0..config.reuse {
            let input = fixture::query(config.family, 3, config.depth, q);
            let mut engine = start(&warm, &input);
            let mut steps = 0;
            let mut actual = vec![];
            while let Some(a) = next(&mut engine, &advance, &mut steps) {
                actual.push(a);
            }
            oracle::same_raw(actual, config.expected[q].clone());
        }
    }
    let mut rows = Vec::with_capacity(64);
    let mut retained = Vec::new();
    let mut counts = [0usize; 4];
    #[cfg(feature = "alloc-meter")]
    let owner = meter::begin();
    let (rules, local) = measure(&mut rows, "source", 0, || {
        fixture::source(config.family, config.depth)
    });
    let p = measure(&mut rows, "prepare", 0, || prepare(&rules, local));
    measure(&mut rows, "source_dispose", 0, || drop(rules));
    #[cfg(feature = "alloc-meter")]
    let prepared_live = rows.last().unwrap().memory.live_end;
    for (q, count) in counts.iter_mut().enumerate().take(config.reuse) {
        let input = measure(&mut rows, "input", q, || {
            fixture::query(config.family, 3, config.depth, q)
        });
        let mut engine = measure(&mut rows, "setup", q, || start(&p, &input));
        let mut steps = 0;
        measure(&mut rows, "first", q, || {
            let answer =
                next(&mut engine, &advance, &mut steps).expect("finite source has answers");
            consume(answer, q, config.keep, &mut retained);
            *count += 1;
        });
        measure(&mut rows, "remaining", q, || {
            if !config.cancel {
                while let Some(answer) = next(&mut engine, &advance, &mut steps) {
                    consume(answer, q, config.keep, &mut retained);
                    *count += 1;
                }
            }
        });
        assert_eq!(
            *count,
            if config.cancel {
                1
            } else {
                config.expected[q].len()
            }
        );
        measure(&mut rows, "engine_dispose", q, || drop(engine));
        measure(&mut rows, "input_dispose", q, || drop(input));
        assert_eq!(Arc::strong_count(&p), 1);
        #[cfg(feature = "alloc-meter")]
        if !config.keep {
            assert_eq!(
                rows.last().unwrap().memory.live_end,
                prepared_live,
                "query retained state"
            );
        }
    }
    measure(&mut rows, "prepared_dispose", config.reuse, || drop(p));
    // Retained measured observations receive independent validation after producer disposal.
    for (q, &count) in counts.iter().enumerate().take(config.reuse) {
        let mut used = vec![false; config.expected[q].len()];
        for (_, a) in retained.iter().filter(|(i, _)| *i == q) {
            let found = config.expected[q]
                .iter()
                .enumerate()
                .position(|(i, w)| {
                    !used[i] && chr_observe::equivalent(a, w, &mut Default::default())
                })
                .expect("retained measured answer differs");
            used[found] = true;
        }
        if config.keep {
            assert_eq!(used.iter().filter(|&&x| x).count(), count);
        }
    }
    let retained_count = retained.len();
    #[cfg(feature = "alloc-meter")]
    let held = meter::end(owner);
    measure(&mut rows, "consumer_dispose", config.reuse, || {
        drop(retained)
    });
    #[cfg(feature = "alloc-meter")]
    let released = meter::end(owner);
    #[cfg(feature = "alloc-meter")]
    assert_eq!(
        released.live_end, released.live_start,
        "final owner conservation"
    );
    let total_ns: u128 = rows.iter().map(|r| r.ns).sum();
    println!(
        "{{\"counts\":{:?},\"retained\":{retained_count},\"total_ns\":{total_ns},\"engine_bytes\":{},\"metered\":{}}}",
        &counts[..config.reuse],
        std::mem::size_of::<E>(),
        cfg!(feature = "alloc-meter")
    );
    #[cfg(feature = "alloc-meter")]
    println!(
        "{{\"consumer_bytes\":{},\"unreleased_bytes\":0}}",
        held.live_end - held.live_start
    );
    for r in rows {
        #[cfg(feature = "alloc-meter")]
        println!(
            "{{\"phase\":\"{}\",\"query\":{},\"ns\":{},\"memory\":{}}}",
            r.phase,
            r.query,
            r.ns,
            r.memory.json()
        );
        #[cfg(not(feature = "alloc-meter"))]
        println!(
            "{{\"phase\":\"{}\",\"query\":{},\"ns\":{}}}",
            r.phase, r.query, r.ns
        );
    }
}
fn main() {
    if cfg!(feature = "replay-diagnostic") || cfg!(feature = "compiled-work") {
        panic!("diagnostic counters must be disabled");
    }
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 7, "mode family depth reuse retain cancel");
    let mode = &args[1];
    let family = &args[2];
    assert!(["plain", "history", "late"].contains(&family.as_str()));
    let depth = args[3].parse::<usize>().unwrap();
    assert!(matches!(depth, 0 | 4));
    let reuse = args[4].parse::<usize>().unwrap();
    assert!(matches!(reuse, 1 | 4));
    let keep = match args[5].as_str() {
        "0" => false,
        "all" => true,
        _ => panic!("retain"),
    };
    let cancel = match args[6].as_str() {
        "0" => false,
        "1" => true,
        _ => panic!("cancel"),
    };
    let expected = {
        let (rules, _) = fixture::source(family, depth);
        (0..reuse)
            .map(|q| oracle::run(&rules, &fixture::query(family, 3, depth, q), 200_000))
            .collect::<Vec<_>>()
    };
    assert!(expected.iter().all(|x| x.len() == 16));
    let config = Config {
        family,
        depth,
        reuse,
        keep,
        cancel,
        expected,
    };
    match mode.as_str() {
        "copy" => run(
            &config,
            |r, _| Prepared::new(r).unwrap(),
            |p, q| p.start(q, Mode::Copy).unwrap(),
            |e| e.advance(),
        ),
        "reunion" => run(
            &config,
            |r, n| PreparedPhase::new(r, n).unwrap(),
            |p, q| p.start(q).unwrap(),
            |e| e.advance().unwrap(),
        ),
        "eager" => run(
            &config,
            |r, n| PreparedPhase::new(r, n).unwrap(),
            |p, q| p.start_repeated(q).unwrap(),
            |e| e.advance().unwrap(),
        ),
        "scheduled" | "fixed1" | "fixed8" | "backoff1" | "backoff8" => {
            let policy = match mode.as_str() {
                "scheduled" => AttemptPolicy::EveryBoundary,
                "fixed1" => AttemptPolicy::FixedSkip(1),
                "fixed8" => AttemptPolicy::FixedSkip(8),
                "backoff1" => AttemptPolicy::FailedCheckBackoff { max_skip: 1 },
                _ => AttemptPolicy::FailedCheckBackoff { max_skip: 8 },
            };
            run(
                &config,
                |r, n| PreparedPhase::new(r, n).unwrap(),
                |p, q| p.start_repeated_with_policy(q, policy).unwrap(),
                |e| e.advance().unwrap(),
            );
        }
        _ => panic!("unknown mode"),
    }
}
