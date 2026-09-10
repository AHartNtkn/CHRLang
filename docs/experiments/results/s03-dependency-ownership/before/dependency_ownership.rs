//! Registered isolated-process dependency ownership experiment.
#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../tests/runtime_support/dependency_source.rs"]
mod source;
use chr_direct_choice::demand::{Event, Prepared as Demand, Reuse, Run};
use chr_syntax::{Answer, Query, Rule};
use std::time::Instant;

struct Reading {
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn measure<T>(f: impl FnOnce() -> T) -> (T, Reading) {
    #[cfg(feature = "alloc-meter")]
    let start = meter::begin();
    let clock = Instant::now();
    let value = f();
    let ns = clock.elapsed().as_nanos();
    #[cfg(feature = "alloc-meter")]
    let memory = meter::end(start);
    (
        value,
        Reading {
            ns,
            #[cfg(feature = "alloc-meter")]
            memory,
        },
    )
}
impl Reading {
    fn json(&self) -> String {
        #[cfg(feature = "alloc-meter")]
        let memory = self.memory.json();
        #[cfg(not(feature = "alloc-meter"))]
        let memory = "null";
        format!("{{\"ns\":{},\"memory\":{memory}}}", self.ns)
    }
}
enum Prepared {
    Demand(Box<Demand>),
    Compiled(Box<chr_compiled::PreparedRuleset>, chr_compiled::Access),
}
enum Running {
    Demand(Box<Run>),
    Compiled(Box<chr_compiled::SearchEngine>),
}
impl Prepared {
    fn new(mode: &str, rules: Vec<Rule>) -> Self {
        if mode == "scan" || mode == "indexed" {
            Self::Compiled(
                Box::new(chr_compiled::PreparedRuleset::new(rules, None).unwrap()),
                if mode == "scan" {
                    chr_compiled::Access::Scan
                } else {
                    chr_compiled::Access::Indexed
                },
            )
        } else {
            let reuse = match mode.strip_suffix("-miss").unwrap_or(mode) {
                "current" => Reuse::CurrentContext,
                "birth" => Reuse::StaticBirth,
                "dependencies" => Reuse::MatchDependencies,
                _ => panic!("unknown mode"),
            };
            let p = Demand::with_reuse(rules, reuse).unwrap();
            Self::Demand(Box::new(if mode.ends_with("-miss") {
                p.with_miss_reuse()
            } else {
                p
            }))
        }
    }
    fn start(&self, q: Query) -> Running {
        match self {
            Self::Demand(p) => Running::Demand(Box::new(p.start(q).unwrap())),
            Self::Compiled(p, access) => Running::Compiled(Box::new(
                p.start_search(q, chr_compiled::Policy::Global, *access)
                    .unwrap(),
            )),
        }
    }
}
impl Running {
    fn collect(&mut self, cancel: bool) -> (Vec<Answer>, bool, usize) {
        let mut answers = vec![];
        for tick in 1..=2_000_000 {
            match self {
                Self::Demand(r) => match r.tick() {
                    Event::Answer(a) => answers.push(a),
                    Event::Exhausted => return (answers, true, tick),
                    Event::Progress => (),
                },
                Self::Compiled(r) => match r.tick() {
                    chr_compiled::SearchEvent::Complete(mut b) => {
                        answers.push(b.engine.observe().unwrap())
                    }
                    chr_compiled::SearchEvent::Exhausted => return (answers, true, tick),
                    _ => (),
                },
            }
            if cancel {
                return (answers, false, tick);
            }
        }
        panic!("service cutoff")
    }
}
fn validate(actual: &[Answer], expected: &[Answer], complete: bool) {
    if complete {
        oracle::same_raw(actual.to_vec(), expected.to_vec());
    } else {
        let mut remaining = expected.to_vec();
        for answer in actual {
            let i = remaining
                .iter()
                .position(|candidate| {
                    chr_observe::equivalent(candidate, answer, &mut Default::default())
                })
                .expect("invalid canceled output");
            remaining.swap_remove(i);
        }
    }
}
fn main() {
    assert!(
        !cfg!(feature = "metrics") && !cfg!(feature = "work-diagnostics"),
        "diagnostic work counters enabled"
    );
    let args = std::env::args().collect::<Vec<_>>();
    if args.get(1).map(String::as_str) == Some("meter-check") {
        #[cfg(feature = "alloc-meter")]
        meter::self_check().unwrap();
        println!("meter-check passed");
        return;
    }
    assert_eq!(args.len(), 7);
    let mode = &args[1];
    let kind = &args[2];
    let size = args[3].parse::<u64>().unwrap();
    let reverse = args[4].parse::<bool>().unwrap();
    let retention = &args[5];
    let cancel = args[6].parse::<bool>().unwrap();
    assert!(
        [
            "plain",
            "known-hit",
            "known-miss",
            "delayed-hit",
            "delayed-miss"
        ]
        .contains(&kind.as_str())
    );
    assert!(["immediate", "window", "all"].contains(&retention.as_str()));
    let rules = source::rules(kind == "plain");
    let expected = ["a", "b"].map(|value| {
        oracle::run(
            &rules,
            &source::query(size, kind, reverse, value),
            2_000_000,
        )
    });
    drop(rules);
    let mut records = Vec::with_capacity(32);
    let mut held = Vec::with_capacity(4);
    let mut endpoints = Vec::with_capacity(4);
    println!("{{\"event\":\"start\"}}");
    #[cfg(feature = "alloc-meter")]
    let root = meter::begin();
    let (rules, m) = measure(|| source::rules(kind == "plain"));
    records.push(("source", 0, m));
    let (prepared, m) = measure(|| Prepared::new(mode, rules));
    records.push(("prepare", 0, m));
    for i in 0..4 {
        let (q, m) =
            measure(|| source::query(size, kind, reverse, if i % 2 == 0 { "a" } else { "b" }));
        records.push(("input", i, m));
        let (mut run, m) = measure(|| prepared.start(q));
        records.push(("setup", i, m));
        let ((answers, complete, ticks), m) = measure(|| run.collect(cancel && i % 2 == 0));
        records.push(("execute_observe", i, m));
        let (_, m) = measure(|| drop(run));
        records.push(("engine_drop", i, m));
        validate(&answers, &expected[i % 2], complete);
        endpoints.push((complete, ticks, answers.len()));
        let (_, m) = measure(|| {
            if retention == "immediate" {
                drop(answers);
            } else {
                held.push((i, complete, answers));
                if retention == "window" && held.len() > 2 {
                    drop(held.remove(0));
                }
            }
        });
        records.push(("consumer", i, m));
    }
    let (_, m) = measure(|| drop(prepared));
    records.push(("prepared_drop", 0, m));
    for (i, complete, answers) in &held {
        validate(answers, &expected[i % 2], *complete);
    }
    let (_, m) = measure(|| held.clear());
    records.push(("consumer_drop", 0, m));
    #[cfg(feature = "alloc-meter")]
    {
        let end = meter::end(root);
        assert_eq!(end.live_start, end.live_end, "unreleased task-owned heap");
    }
    let rows = records
        .iter()
        .map(|(phase, query, r)| {
            format!(
                "{{\"phase\":\"{phase}\",\"query\":{query},\"reading\":{}}}",
                r.json()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let ends = endpoints
        .iter()
        .map(|(complete, ticks, answers)| {
            format!("{{\"complete\":{complete},\"ticks\":{ticks},\"answers\":{answers}}}")
        })
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "{{\"event\":\"result\",\"meter\":{},\"endpoints\":[{ends}],\"phases\":[{rows}]}}",
        cfg!(feature = "alloc-meter")
    );
}
