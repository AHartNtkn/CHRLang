#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use chr_reuse::calls::trace::caller::Caller;
use chr_reuse::continuations::{Batch, Mode, Prepared as Whole};
use chr_reuse::residuals::Prepared as Separated;
use chr_syntax::{Answer, Query, Rule};
#[allow(dead_code)]
#[path = "support/call_trace_source.rs"]
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
fn query_depth(cfg: &Config, q: usize) -> usize {
    cfg.depth
        + match cfg.query_policy {
            0 => 0,
            1 => 2 * q,
            2 => 2 * (q % 4),
            3 => 2 * (q % 16),
            _ => unreachable!("validated query policy"),
        }
}
fn input(cfg: &Config, q: usize) -> Query {
    let n = query_depth(cfg, q);
    fixture::input(n, n + 1, (1000 * q + 7) as u64)
}
fn next<P, E>(
    p: &mut P,
    e: &mut E,
    advance: &impl Fn(&mut P, &mut E) -> Batch,
    steps: &mut usize,
) -> Option<Answer> {
    while *steps < 100_000 {
        *steps += 1;
        let mut b = advance(p, e);
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
    expected: std::collections::BTreeMap<usize, Vec<Answer>>,
    query_policy: usize,
}
impl Config {
    fn expected(&self, q: usize) -> &[Answer] {
        &self.expected[&query_depth(self, q)]
    }
}
fn run<P, E>(
    cfg: &Config,
    prepare: impl Fn(Vec<Rule>) -> P,
    start: impl Fn(&mut P, Query) -> E,
    advance: impl Fn(&mut P, &mut E) -> Batch,
    maintain: impl Fn(&mut P, usize),
) {
    let mut warm = prepare(fixture::program(false, cfg.family).0);
    for (&n, expected) in &cfg.expected {
        let mut e = start(&mut warm, fixture::input(n, n + 1, 7));
        let mut steps = 0;
        let mut actual = vec![];
        while let Some(a) = next(&mut warm, &mut e, &advance, &mut steps) {
            actual.push(a);
        }
        assert_eq!(actual.len(), expected.len());
        for (a, b) in actual.iter().zip(expected) {
            assert!(chr_observe::equivalent(a, b, &mut Default::default()));
        }
    }
    drop(warm);
    let capacity = 5
        + (0..cfg.reuse)
            .map(|q| 2 * cfg.expected(q).len() + 6)
            .sum::<usize>();
    let mut rows = Vec::with_capacity(capacity);
    let mut consumer: Vec<(usize, usize, Answer)> = vec![];
    let mut counts = vec![0usize; cfg.reuse];
    #[cfg(feature = "alloc-meter")]
    let owner = meter::begin();
    let rules = measure(&mut rows, "source", 0, || {
        fixture::program(false, cfg.family).0
    });
    let mut p = measure(&mut rows, "prepare", 0, || prepare(rules.clone()));
    measure(&mut rows, "source_dispose", 0, || drop(rules));
    for (q, count) in counts.iter_mut().enumerate().take(cfg.reuse) {
        let query = measure(&mut rows, "input", q, || input(cfg, q));
        let mut e = measure(&mut rows, "setup", q, || start(&mut p, query.clone()));
        let mut steps = 0;
        loop {
            let a = measure(&mut rows, "service_observe", q, || {
                next(&mut p, &mut e, &advance, &mut steps)
            });
            let Some(a) = a else { break };
            #[cfg(feature = "alloc-meter")]
            let check = meter::begin();
            assert!(chr_observe::equivalent(
                &a,
                &cfg.expected(q)[*count],
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
        assert_eq!(*count, if cfg.cancel { 1 } else { cfg.expected(q).len() });
        measure(&mut rows, "engine_dispose", q, || drop(e));
        measure(&mut rows, "input_dispose", q, || drop(query));
        measure(&mut rows, "maintenance", q, || maintain(&mut p, q + 1));
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
            &cfg.expected(*q)[*i],
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
        "{{\"oracle_classes\":{},\"counts\":{:?},\"retained\":{retained},\"consumer_bytes\":{},\"requested_bytes\":{},\"unreleased_bytes\":{}}}",
        cfg.expected.len(),
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
    entry(None);
}
pub fn entry(code: Option<chr_compiled::Compiled>) {
    if chr_reuse::continuations::COLLECT_METRICS {
        panic!("counter-free runner required");
    }
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    let a = std::env::args().collect::<Vec<_>>();
    assert_eq!(a.len(), 8);
    let family = a[2].parse().unwrap();
    let depth = a[3].parse().unwrap();
    let reuse = a[4].parse().unwrap();
    assert!(family < 4 && (1..=2048).contains(&depth) && (1..=32768).contains(&reuse));
    let query_policy = a[7].parse().unwrap();
    assert!(query_policy <= 3);
    let keep = match &*a[5] {
        "0" => 0,
        "all" => usize::MAX,
        _ => panic!("keep"),
    };
    let flag = |i: usize| match &*a[i] {
        "0" => false,
        "1" => true,
        _ => panic!("flag"),
    };
    let mut cfg = Config {
        family,
        depth,
        reuse,
        keep,
        cancel: flag(6),
        query_policy,
        expected: Default::default(),
    };
    let (rules, count) = fixture::program(false, family);
    let direct = Whole::new(rules.clone(), Mode::Direct).unwrap();
    for q in 0..reuse {
        let n = query_depth(&cfg, q);
        if cfg.expected.contains_key(&n) {
            continue;
        }
        let query = input(&cfg, q);
        let mut search = direct.start(query.clone()).unwrap();
        let mut answers = vec![];
        for _ in 0..100_000 {
            let b = search.advance(1);
            answers.extend(b.answers);
            if b.exhausted {
                break;
            }
        }
        oracle::same_raw(answers.clone(), oracle::run(&rules, &query, 100_000));
        cfg.expected.insert(n, answers);
    }
    drop((direct, rules));
    match &*a[1] {
        "trace" | "trace1" | "trace4" | "trace16" => run(
            &cfg,
            |rs| Caller::new(rs, count).unwrap(),
            |p, q| p.start(q).unwrap(),
            |p, e| p.advance(e, 1).unwrap(),
            |p, q| {
                let window = match &*a[1] {
                    "trace1" => 1,
                    "trace4" => 4,
                    "trace16" => 16,
                    _ => 0,
                };
                if window != 0 && q % window == 0 {
                    p.clear_traces().unwrap();
                }
            },
        ),
        "direct" => run(
            &cfg,
            |rs| Whole::new(rs, Mode::Direct).unwrap(),
            |p, q| p.start(q).unwrap(),
            |_, e| e.advance(1),
            |_, _| {},
        ),
        "scan" | "indexed" | "sealed" | "planned" | "planned-sealed" | "generated"
        | "generated-sealed" => {
            assert_eq!(code.is_some(), a[1].starts_with("generated"));
            let access = if a[1] == "scan" {
                chr_compiled::Access::Scan
            } else {
                chr_compiled::Access::Indexed
            };
            run(
                &cfg,
                |rs| {
                    let p = if a[1].starts_with("planned") {
                        chr_compiled::PreparedRuleset::new_with_update_plan(rs, code).unwrap()
                    } else {
                        chr_compiled::PreparedRuleset::new(rs, code).unwrap()
                    };
                    if matches!(&*a[1], "sealed" | "planned-sealed" | "generated-sealed") {
                        p.specialize_inferred()
                    } else {
                        p
                    }
                },
                |p, q| {
                    p.start_search(q, chr_compiled::Policy::Global, access)
                        .unwrap()
                },
                |_, e| match e.tick() {
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
                |_, _| {},
            );
        }
        "memo" | "memo16" => {
            let stride = if a[1] == "memo" { 1 } else { 16 };
            run(
                &cfg,
                |rs| {
                    Separated::new(rs, true)
                        .unwrap()
                        .with_recognition_stride(stride)
                },
                |p, q| p.start(q).unwrap(),
                |_, e| e.advance(1),
                |_, _| {},
            );
        }
        _ => panic!("mode"),
    }
}

#[test]
fn cached_fixture_expectations_match_actual_renamed_queries() {
    for family in 0..4 {
        let rules = fixture::program(false, family).0;
        for query_policy in 0..=3 {
            let mut cfg = Config {
                family,
                depth: 32,
                reuse: 16,
                keep: 0,
                cancel: false,
                expected: Default::default(),
                query_policy,
            };
            for q in 0..16 {
                let n = query_depth(&cfg, q);
                cfg.expected
                    .entry(n)
                    .or_insert_with(|| oracle::run(&rules, &fixture::input(n, n + 1, 0), 100_000));
                oracle::same_raw(
                    cfg.expected(q).to_vec(),
                    oracle::run(&rules, &input(&cfg, q), 100_000),
                );
            }
            assert_eq!(
                cfg.expected.len(),
                match query_policy {
                    0 => 1,
                    1 | 3 => 16,
                    2 => 4,
                    _ => unreachable!(),
                }
            );
            if query_policy != 1 {
                oracle::same_raw(
                    cfg.expected(8191).to_vec(),
                    oracle::run(&rules, &input(&cfg, 8191), 100_000),
                );
            }
        }
    }
}
