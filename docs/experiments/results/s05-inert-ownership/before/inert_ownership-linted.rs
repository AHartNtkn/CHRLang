use chr_compiled::experiment::meter;
use chr_reuse::continuations::{Batch, Mode, Prepared as Whole};
use chr_reuse::residuals::Prepared as Separated;
use chr_syntax::{Answer, Query, Rule, Var, c, v};
#[path = "support/inert_source.rs"]
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
    let s = meter::begin();
    let result = f();
    let memory = meter::end(s);
    assert!(rows.len() < rows.capacity());
    rows.push(Row {
        phase,
        query,
        memory,
    });
    result
}
fn input(q: usize) -> Query {
    let offset = (q * 1000 + 7) as u64;
    Query {
        constraints: vec![c("start", [v(offset)])],
        outputs: vec![("out".into(), Var(offset))],
    }
}
fn next<E>(e: &mut E, advance: &impl Fn(&mut E) -> Batch, steps: &mut usize) -> Option<Answer> {
    while *steps < 200_000 {
        *steps += 1;
        let mut b = advance(e);
        assert!(b.answers.len() <= 1);
        if let Some(a) = b.answers.pop() {
            return Some(a);
        }
        if b.exhausted {
            return None;
        }
    }
    panic!("service bound");
}
struct Config {
    family: usize,
    depth: usize,
    reuse: usize,
    keep: usize,
    cancel: bool,
    expected: Vec<Vec<Answer>>,
}
fn run<P, E>(
    cfg: &Config,
    prepare: impl Fn(Vec<Rule>) -> P,
    start: impl Fn(&P, Query) -> E,
    advance: impl Fn(&mut E) -> Batch,
) {
    let warm = prepare(fixture::source(cfg.family, cfg.depth, true, 0).0);
    for q in 0..cfg.reuse {
        let mut e = start(&warm, input(q));
        let mut steps = 0;
        let mut actual = vec![];
        while let Some(a) = next(&mut e, &advance, &mut steps) {
            actual.push(a);
        }
        assert_eq!(actual.len(), cfg.expected[q].len());
        for (a, b) in actual.iter().zip(&cfg.expected[q]) {
            assert!(chr_observe::equivalent(a, b, &mut Default::default()));
        }
    }
    drop(warm);
    let mut rows = Vec::with_capacity(256);
    let mut consumer: Vec<(usize, usize, Answer)> = vec![];
    let mut counts = [0usize; 4];
    let owner = meter::begin();
    let rules = measure(&mut rows, "source", 0, || {
        fixture::source(cfg.family, cfg.depth, true, 0).0
    });
    let p = measure(&mut rows, "prepare", 0, || prepare(rules.clone()));
    measure(&mut rows, "source_dispose", 0, || drop(rules));
    let prepared_live = rows.last().unwrap().memory.live_end;
    for (q, count) in counts.iter_mut().enumerate().take(cfg.reuse) {
        let query = measure(&mut rows, "input", q, || input(q));
        let mut e = measure(&mut rows, "setup", q, || start(&p, query.clone()));
        let mut steps = 0;
        loop {
            let a = measure(&mut rows, "service_observe", q, || {
                next(&mut e, &advance, &mut steps)
            });
            let Some(a) = a else { break };
            let check = meter::begin();
            assert!(chr_observe::equivalent(
                &a,
                &cfg.expected[q][*count],
                &mut Default::default()
            ));
            let checked = meter::end(check);
            assert_eq!(checked.live_start, checked.live_end);
            let index = *count;
            *count += 1;
            measure(&mut rows, "consume", q, || {
                if cfg.keep == 0 {
                    drop(a);
                } else {
                    if consumer.len() == cfg.keep {
                        consumer.remove(0);
                    }
                    consumer.push((q, index, a));
                }
            });
            if cfg.cancel {
                break;
            }
        }
        assert_eq!(*count, if cfg.cancel { 1 } else { cfg.expected[q].len() });
        measure(&mut rows, "engine_dispose", q, || drop(e));
        measure(&mut rows, "input_dispose", q, || drop(query));
        if cfg.keep == 0 {
            assert_eq!(
                rows.last().unwrap().memory.live_end,
                prepared_live,
                "query owner retained"
            );
        }
    }
    measure(&mut rows, "prepared_dispose", cfg.reuse, || drop(p));
    let held = meter::end(owner);
    let consumer_bytes = held.live_end - held.live_start;
    for (q, i, a) in &consumer {
        assert!(chr_observe::equivalent(
            a,
            &cfg.expected[*q][*i],
            &mut Default::default()
        ));
    }
    let retained = consumer.len();
    measure(&mut rows, "consumer_dispose", cfg.reuse, || drop(consumer));
    let released = meter::end(owner);
    assert_eq!(released.live_end, released.live_start, "unreleased owner");
    let requested_bytes: usize = rows.iter().map(|r| r.memory.requested_bytes).sum();
    println!(
        "{{\"counts\":{:?},\"retained\":{retained},\"consumer_bytes\":{consumer_bytes},\"requested_bytes\":{requested_bytes},\"unreleased_bytes\":0}}",
        &counts[..cfg.reuse]
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
    if chr_reuse::continuations::COLLECT_METRICS {
        panic!("metrics-off ownership build required");
    }
    meter::self_check().unwrap();
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 7);
    let family = args[2].parse().unwrap();
    let depth = args[3].parse().unwrap();
    let reuse = args[4].parse().unwrap();
    assert!(family < 6 && matches!(depth, 0 | 4) && matches!(reuse, 1 | 4));
    let keep = match args[5].as_str() {
        "0" => 0,
        "1" => 1,
        "all" => usize::MAX,
        _ => panic!("keep"),
    };
    let cancel = match args[6].as_str() {
        "0" => false,
        "1" => true,
        _ => panic!("cancel"),
    };
    let rules = fixture::source(family, depth, true, 0).0;
    let direct = Whole::new(rules.clone(), Mode::Direct).unwrap();
    let mut expected = vec![];
    for q in 0..reuse {
        let input = input(q);
        let want = oracle::run(&rules, &input, 200_000);
        let mut e = direct.start(input).unwrap();
        let b = e.advance(200_000);
        assert!(b.exhausted);
        oracle::same_raw(b.answers.clone(), want);
        expected.push(b.answers);
    }
    drop(direct);
    drop(rules);
    let cfg = Config {
        family,
        depth,
        reuse,
        keep,
        cancel,
        expected,
    };
    match args[1].as_str() {
        "direct" | "whole" => {
            let mode = if args[1] == "direct" {
                Mode::Direct
            } else {
                Mode::AlphaLive
            };
            run(
                &cfg,
                |r| Whole::new(r, mode).unwrap(),
                |p, q| p.start(q).unwrap(),
                |e| e.advance(1),
            );
        }
        "separate" | "memo" => {
            let memo = args[1] == "memo";
            run(
                &cfg,
                |r| Separated::new(r, memo).unwrap(),
                |p, q| p.start(q).unwrap(),
                |e| e.advance(1),
            );
        }
        _ => panic!("mode"),
    }
}
