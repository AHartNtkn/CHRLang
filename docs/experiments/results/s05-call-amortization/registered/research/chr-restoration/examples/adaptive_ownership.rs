//! Requested-allocation ownership qualification; validation runs between phases.
use chr_compiled::experiment::meter;
use chr_restoration::{
    Mode, Prepared, Step,
    reunion::{AttemptPolicy, PreparedPhase},
};
use chr_syntax::{Answer, Query};
use std::sync::Arc;
#[path = "support/repeated_source.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
struct Row {
    phase: &'static str,
    query: usize,
    memory: meter::Reading,
}
fn measure<T>(rows: &mut Vec<Row>, phase: &'static str, query: usize, f: impl FnOnce() -> T) -> T {
    let start = meter::begin();
    let value = f();
    let memory = meter::end(start);
    assert!(rows.len() < rows.capacity(), "bookkeeping bound");
    rows.push(Row {
        phase,
        query,
        memory,
    });
    value
}
struct Config<'a> {
    family: &'a str,
    depth: usize,
    keep: usize,
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
fn run<P, E>(
    config: &Config<'_>,
    prepare: impl Fn() -> Arc<P>,
    start: impl Fn(&Arc<P>, &Query) -> E,
    advance: impl Fn(&mut E) -> Step,
) {
    // Whole-answer preflight is independent of measurement and retention choices.
    let warm = prepare();
    for (q, want) in config.expected.iter().enumerate() {
        let input = fixture::query(config.family, 3, config.depth, q);
        let mut engine = start(&warm, &input);
        let mut steps = 0;
        let mut actual = vec![];
        while let Some(a) = next(&mut engine, &advance, &mut steps) {
            actual.push(a);
        }
        println!("preflight query={q} answers={actual:?}");
        oracle::same_raw(actual, want.clone());
    }
    drop(warm);
    let mut used = config
        .expected
        .iter()
        .map(|v| vec![false; v.len()])
        .collect::<Vec<_>>();
    let mut rows = Vec::with_capacity(512);
    let mut consumer: Vec<(usize, Answer)> = Vec::new();
    let mut counts = [0usize; 4];
    let owner = meter::begin();
    let p = measure(&mut rows, "prepare", 0, prepare);
    let prepared_live = rows[0].memory.live_end;
    for q in 0..4 {
        let input = measure(&mut rows, "input", q, || {
            fixture::query(config.family, 3, config.depth, q)
        });
        let mut engine = measure(&mut rows, "setup", q, || start(&p, &input));
        let mut steps = 0;
        loop {
            let answer = measure(&mut rows, "service_observe", q, || {
                next(&mut engine, &advance, &mut steps)
            });
            let Some(answer) = answer else {
                break;
            };
            // Borrowed exact validation allocates transiently, outside measured phases.
            let before = meter::begin();
            let found = config.expected[q]
                .iter()
                .enumerate()
                .find(|(i, want)| {
                    !used[q][*i] && chr_observe::equivalent(want, &answer, &mut Default::default())
                })
                .map(|(i, _)| i)
                .expect("unexpected or duplicate answer");
            used[q][found] = true;
            let validated = meter::end(before);
            assert_eq!(
                validated.live_start, validated.live_end,
                "validation retained memory"
            );
            counts[q] += 1;
            measure(&mut rows, "consume", q, || {
                if config.keep == 0 {
                    drop(answer);
                } else {
                    if consumer.len() == config.keep {
                        consumer.remove(0);
                    }
                    consumer.push((q, answer));
                }
            });
            if config.cancel {
                break;
            }
        }
        assert_eq!(
            counts[q],
            if config.cancel {
                1
            } else {
                config.expected[q].len()
            }
        );
        measure(&mut rows, "engine_dispose", q, || drop(engine));
        measure(&mut rows, "input_dispose", q, || drop(input));
        assert_eq!(Arc::strong_count(&p), 1, "engine retained prepared owner");
        if config.keep == 0 {
            assert_eq!(
                rows.last().unwrap().memory.live_end,
                prepared_live,
                "query retained state"
            );
        }
    }
    measure(&mut rows, "prepared_dispose", 4, || drop(p));
    let held = meter::end(owner);
    let consumer_bytes = held.live_end - held.live_start;
    for (q, answer) in &consumer {
        assert!(
            config.expected[*q]
                .iter()
                .any(|want| chr_observe::equivalent(want, answer, &mut Default::default())),
            "consumer after producer disposal"
        );
    }
    let retained = consumer.len();
    measure(&mut rows, "consumer_dispose", 4, || drop(consumer));
    let released = meter::end(owner);
    assert_eq!(
        released.live_end, released.live_start,
        "final owner conservation"
    );
    let requested: usize = rows.iter().map(|r| r.memory.requested_bytes).sum();
    println!(
        "{{\"counts\":{counts:?},\"retained\":{retained},\"consumer_bytes\":{consumer_bytes},\"requested_bytes\":{requested},\"engine_bytes\":{},\"unreleased_bytes\":0}}",
        std::mem::size_of::<E>()
    );
    for r in rows {
        println!(
            "{{\"phase\":\"{}\",\"query\":{},\"memory\":{}}}",
            r.phase,
            r.query,
            r.memory.json()
        );
    }
}
fn main() {
    if cfg!(feature = "replay-diagnostic") {
        panic!("diagnostic-free ownership build required");
    }
    meter::self_check().unwrap();
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 6, "mode family depth retain cancel");
    let mode = &args[1];
    let family = &args[2];
    let depth = args[3].parse::<usize>().unwrap();
    assert!(matches!(depth, 0 | 4));
    let keep = match args[4].as_str() {
        "0" => 0,
        "4" => 4,
        "all" => usize::MAX,
        _ => panic!("retain"),
    };
    let cancel = match args[5].as_str() {
        "0" => false,
        "1" => true,
        _ => panic!("cancel"),
    };
    let (rules, local) = fixture::source(family, depth);
    let expected = (0..4)
        .map(|q| oracle::run(&rules, &fixture::query(family, 3, depth, q), 200_000))
        .collect();
    let config = Config {
        family,
        depth,
        keep,
        cancel,
        expected,
    };
    assert!(config.expected.iter().all(|a| a.len() == 16));
    match mode.as_str() {
        "copy" => run(
            &config,
            || Prepared::new(&rules).unwrap(),
            |p, q| p.start(q, Mode::Copy).unwrap(),
            |e| e.advance(),
        ),
        "reunion" => run(
            &config,
            || PreparedPhase::new(&rules, local).unwrap(),
            |p, q| p.start(q).unwrap(),
            |e| e.advance().unwrap(),
        ),
        "eager" => run(
            &config,
            || PreparedPhase::new(&rules, local).unwrap(),
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
                || PreparedPhase::new(&rules, local).unwrap(),
                |p, q| p.start_repeated_with_policy(q, policy).unwrap(),
                |e| e.advance().unwrap(),
            );
        }
        _ => panic!("unknown mode"),
    }
}
