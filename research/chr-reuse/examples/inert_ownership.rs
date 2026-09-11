#[cfg(feature = "alloc-meter")]
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
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
    ns: Option<u128>,
}
fn measure<T>(rows: &mut Vec<Row>, phase: &'static str, query: usize, f: impl FnOnce() -> T) -> T {
    #[cfg(feature = "alloc-meter")]
    let s = meter::begin();
    #[cfg(not(feature = "alloc-meter"))]
    let s = std::time::Instant::now();
    let result = std::hint::black_box(f());
    #[cfg(feature = "alloc-meter")]
    let memory = meter::end(s);
    #[cfg(feature = "alloc-meter")]
    let ns = None;
    #[cfg(not(feature = "alloc-meter"))]
    let ns = Some(s.elapsed().as_nanos());
    assert!(rows.len() < rows.capacity());
    rows.push(Row {
        phase,
        query,
        #[cfg(feature = "alloc-meter")]
        memory,
        ns,
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
    while *steps < 2_000_000 {
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
    distinct: bool,
}
fn run<P, E>(
    cfg: &Config,
    prepare: impl Fn(Vec<Rule>) -> P,
    start: impl Fn(&P, Query) -> E,
    advance: impl Fn(&mut E) -> Batch,
) {
    let warm = prepare(fixture::source(cfg.family, cfg.depth, cfg.distinct, 0).0);
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
    #[cfg(feature = "alloc-meter")]
    let owner = meter::begin();
    let rules = measure(&mut rows, "source", 0, || {
        fixture::source(cfg.family, cfg.depth, cfg.distinct, 0).0
    });
    let p = measure(&mut rows, "prepare", 0, || prepare(rules.clone()));
    measure(&mut rows, "source_dispose", 0, || drop(rules));
    #[cfg(feature = "alloc-meter")]
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
            #[cfg(feature = "alloc-meter")]
            let check = meter::begin();
            assert!(chr_observe::equivalent(
                &a,
                &cfg.expected[q][*count],
                &mut Default::default()
            ));
            #[cfg(feature = "alloc-meter")]
            let checked = meter::end(check);
            #[cfg(feature = "alloc-meter")]
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
        #[cfg(feature = "alloc-meter")]
        if cfg.keep == 0 {
            assert_eq!(
                rows.last().unwrap().memory.live_end,
                prepared_live,
                "query owner retained"
            );
        }
    }
    measure(&mut rows, "prepared_dispose", cfg.reuse, || drop(p));
    #[cfg(feature = "alloc-meter")]
    let consumer_bytes = Some({
        let held = meter::end(owner);
        held.live_end - held.live_start
    });
    #[cfg(not(feature = "alloc-meter"))]
    let consumer_bytes: Option<usize> = None;
    for (q, i, a) in &consumer {
        assert!(chr_observe::equivalent(
            a,
            &cfg.expected[*q][*i],
            &mut Default::default()
        ));
    }
    let retained = consumer.len();
    measure(&mut rows, "consumer_dispose", cfg.reuse, || drop(consumer));
    #[cfg(feature = "alloc-meter")]
    let requested_bytes = Some({
        let released = meter::end(owner);
        assert_eq!(released.live_end, released.live_start, "unreleased owner");
        rows.iter().map(|r| r.memory.requested_bytes).sum::<usize>()
    });
    #[cfg(not(feature = "alloc-meter"))]
    let requested_bytes: Option<usize> = None;
    let json = |v: Option<usize>| v.map_or_else(|| "null".into(), |v| v.to_string());
    println!(
        "{{\"counts\":{:?},\"retained\":{retained},\"consumer_bytes\":{},\"requested_bytes\":{},\"unreleased_bytes\":{}}}",
        &counts[..cfg.reuse],
        json(consumer_bytes),
        json(requested_bytes),
        json(cfg!(feature = "alloc-meter").then_some(0))
    );
    for r in rows {
        #[cfg(feature = "alloc-meter")]
        let memory = r.memory.json();
        #[cfg(not(feature = "alloc-meter"))]
        let memory = "null";
        let ns = r.ns.map_or_else(|| "null".into(), |v| v.to_string());
        println!(
            "{{\"phase\":\"{}\",\"query\":{},\"memory\":{memory},\"ns\":{ns}}}",
            r.phase, r.query
        );
    }
}
fn main() {
    if chr_reuse::continuations::COLLECT_METRICS {
        panic!("metrics-off ownership build required");
    }
    if cfg!(feature = "stage-alloc") {
        panic!("profile build is not lifecycle measurement");
    }
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    let args = std::env::args().collect::<Vec<_>>();
    if args.get(1).is_some_and(|x| x == "clock-check") {
        if cfg!(feature = "alloc-meter") {
            panic!("clock calibration needs ordinary allocator");
        }
        let mut samples = Vec::with_capacity(10000);
        for _ in 0..10000 {
            let t = std::time::Instant::now();
            std::hint::black_box(());
            samples.push(t.elapsed().as_nanos());
        }
        samples.sort_unstable();
        println!(
            "{{\"samples\":10000,\"median_ns\":{},\"p99_ns\":{}}}",
            samples[5000], samples[9900]
        );
        return;
    }
    assert!([7, 8].contains(&args.len()));
    let family = args[2].parse().unwrap();
    let depth = args[3].parse().unwrap();
    let reuse = args[4].parse().unwrap();
    assert!(family < 6 && matches!(depth, 0 | 4 | 32 | 128) && matches!(reuse, 1 | 4));
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
    execute(
        &args[1],
        configure(
            family,
            depth,
            reuse,
            keep,
            cancel,
            args.get(7).is_none_or(|x| x.parse().unwrap()),
        ),
    );
}
fn configure(
    family: usize,
    depth: usize,
    reuse: usize,
    keep: usize,
    cancel: bool,
    distinct: bool,
) -> Config {
    let rules = fixture::source(family, depth, distinct, 0).0;
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
    Config {
        family,
        depth,
        reuse,
        keep,
        cancel,
        expected,
        distinct,
    }
}
fn execute(mode: &str, cfg: Config) {
    match mode {
        "direct" | "whole" | "compact" => {
            let mode = if mode == "direct" {
                Mode::Direct
            } else if mode == "whole" {
                Mode::AlphaLive
            } else {
                Mode::CompactLive
            };
            run(
                &cfg,
                |r| Whole::new(r, mode).unwrap(),
                |p, q| p.start(q).unwrap(),
                |e| e.advance(1),
            );
        }
        "separate" | "memo" => {
            let memo = mode == "memo";
            run(
                &cfg,
                |r| Separated::new(r, memo).unwrap(),
                |p, q| p.start(q).unwrap(),
                |e| e.advance(1),
            );
        }
        "scan" | "indexed" | "sealed" | "active-scan" | "active-indexed" => {
            let access = if mode.ends_with("indexed") {
                chr_compiled::Access::Indexed
            } else {
                chr_compiled::Access::Scan
            };
            let policy = if mode.starts_with("active-") {
                chr_compiled::Policy::Active
            } else {
                chr_compiled::Policy::Global
            };
            run(
                &cfg,
                |r| {
                    let p = chr_compiled::PreparedRuleset::new(r, None).unwrap();
                    if mode == "sealed" {
                        p.specialize_inferred()
                    } else {
                        p
                    }
                },
                |p, q| p.start_search(q, policy, access).unwrap(),
                |e| match e.tick() {
                    chr_compiled::SearchEvent::Complete(mut b) => Batch {
                        answers: vec![b.engine.observe().unwrap()],
                        exhausted: false,
                    },
                    chr_compiled::SearchEvent::Exhausted => Batch {
                        answers: vec![],
                        exhausted: true,
                    },
                    _ => Batch {
                        answers: vec![],
                        exhausted: false,
                    },
                },
            );
        }
        _ => panic!("mode"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn lifecycle_controls_preserve_answers_and_consumer_lifetimes() {
        for family in 0..6 {
            for mode in [
                "direct",
                "whole",
                "compact",
                "separate",
                "memo",
                "scan",
                "indexed",
                "sealed",
                "active-scan",
                "active-indexed",
            ] {
                for keep in [0, 1, usize::MAX] {
                    for cancel in [false, true] {
                        let cfg = super::configure(family, 4, 4, keep, cancel, true);
                        super::execute(mode, cfg);
                    }
                }
            }
        }
    }
}
