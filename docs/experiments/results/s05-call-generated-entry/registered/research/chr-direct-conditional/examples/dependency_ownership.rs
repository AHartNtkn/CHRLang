//! Registered isolated-process dependency ownership experiment.
#[cfg(feature = "candidate-profile")]
#[path = "support/candidate_profile.rs"]
mod candidate_profile;
#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../tests/runtime_support/post_source.rs"]
mod post_source;
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
    Compiled(
        Box<chr_compiled::PreparedRuleset>,
        chr_compiled::Access,
        chr_compiled::Policy,
    ),
}
enum Running {
    Demand(Box<Run>),
    Compiled(Box<chr_compiled::SearchEngine>),
}
impl Prepared {
    fn new(mode: &str, rules: Vec<Rule>, kind: &str) -> Self {
        if [
            "scan",
            "indexed",
            "sealed",
            "active-scan",
            "active-indexed",
            "native-scan",
            "native-indexed",
            "active-native-scan",
            "active-native-indexed",
        ]
        .contains(&mode)
        {
            Self::Compiled(
                Box::new({
                    let code = if mode.contains("native") {
                        let id = post_source::FAMILIES
                            .iter()
                            .position(|f| *f == kind)
                            .expect("native post source");
                        Some(chr_compiled::access_post_bundled(id))
                    } else {
                        None
                    };
                    let p = chr_compiled::PreparedRuleset::new(rules, code).unwrap();
                    if mode == "sealed" {
                        p.specialize_inferred()
                    } else {
                        p
                    }
                }),
                if mode.ends_with("scan") {
                    chr_compiled::Access::Scan
                } else {
                    chr_compiled::Access::Indexed
                },
                if mode.starts_with("active-") {
                    chr_compiled::Policy::Active
                } else {
                    chr_compiled::Policy::Global
                },
            )
        } else {
            let base = mode.strip_suffix("-template").unwrap_or(mode);
            let reuse = match base.strip_suffix("-miss").unwrap_or(base) {
                "current" => Reuse::CurrentContext,
                "birth" => Reuse::StaticBirth,
                "dependencies" => Reuse::MatchDependencies,
                _ => panic!("unknown mode"),
            };
            let mut p = Demand::with_reuse(rules, reuse).unwrap();
            if base.ends_with("-miss") {
                p = p.with_miss_reuse();
            }
            if mode.ends_with("-template") {
                p = p.with_derivation_templates();
            }
            Self::Demand(Box::new(p))
        }
    }
    fn start(&self, q: Query) -> Running {
        match self {
            Self::Demand(p) => Running::Demand(Box::new(p.start(q).unwrap())),
            Self::Compiled(p, access, policy) => {
                Running::Compiled(Box::new(p.start_search(q, *policy, *access).unwrap()))
            }
        }
    }
}
impl Running {
    fn collect(
        &mut self,
        cancel: bool,
        first_clock: bool,
    ) -> (Vec<Answer>, bool, usize, Option<u128>) {
        let start = first_clock.then(Instant::now);
        let mut first = None;
        let mut answers = vec![];
        for tick in 1..=2_000_000 {
            match self {
                Self::Demand(r) => match r.tick() {
                    Event::Answer(a) => {
                        answers.push(a);
                        if first.is_none() {
                            first = start.map(|clock| clock.elapsed().as_nanos());
                        }
                    }
                    Event::Exhausted => return (answers, true, tick, first),
                    Event::Progress => (),
                },
                Self::Compiled(r) => match r.tick() {
                    chr_compiled::SearchEvent::Complete(mut b) => {
                        answers.push(b.engine.observe().unwrap());
                        if first.is_none() {
                            first = start.map(|clock| clock.elapsed().as_nanos());
                        }
                    }
                    chr_compiled::SearchEvent::Exhausted => return (answers, true, tick, first),
                    _ => (),
                },
            }
            if cancel {
                return (answers, false, tick, first);
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
fn source_rules(kind: &str) -> Vec<Rule> {
    if post_source::FAMILIES.contains(&kind) {
        post_source::rules(kind)
    } else {
        source::rules(kind == "plain")
    }
}
fn source_query(size: u64, kind: &str, reverse: bool, value: &str) -> Query {
    if post_source::FAMILIES.contains(&kind) {
        post_source::query(size, kind, reverse, value)
    } else {
        source::query(size, kind, reverse, value)
    }
}
#[allow(clippy::assertions_on_constants)] // Reject instrumented measurements at runtime.
fn main() {
    assert!(
        !cfg!(feature = "metrics")
            && !cfg!(feature = "work-diagnostics")
            && !chr_direct_choice::demand::COLLECT_WORK_DIAGNOSTICS
            && !chr_compiled::COLLECT_METRICS,
        "diagnostic work counters enabled"
    );
    let args = std::env::args().collect::<Vec<_>>();
    if args.get(1).map(String::as_str) == Some("meter-check") {
        #[cfg(feature = "alloc-meter")]
        meter::self_check().unwrap();
        println!("meter-check passed");
        return;
    }
    if args.get(1).map(String::as_str) == Some("clock-check") {
        assert!(
            !cfg!(feature = "alloc-meter"),
            "clock calibration requires ordinary allocator"
        );
        let mut readings = Vec::with_capacity(100_000);
        for _ in 0..100_000 {
            let (_, reading) = measure(|| std::hint::black_box(()));
            readings.push(reading.ns);
        }
        readings.sort_unstable();
        println!(
            "{{\"event\":\"clock\",\"samples\":100000,\"min\":{},\"median\":{},\"p99\":{},\"max\":{}}}",
            readings[0], readings[50_000], readings[99_000], readings[99_999]
        );
        return;
    }
    let first_clock = match std::env::var("DEPENDENCY_FIRST_CLOCK").as_deref() {
        Ok("on") => true,
        Ok("off") | Err(_) => false,
        _ => panic!("unknown first-observation clock mode"),
    };
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
            || post_source::FAMILIES.contains(&kind.as_str())
    );
    assert!(["immediate", "window", "all"].contains(&retention.as_str()));
    let rules = source_rules(kind);
    let expected = ["a", "b"]
        .map(|value| oracle::run(&rules, &source_query(size, kind, reverse, value), 2_000_000));
    if post_source::FAMILIES.contains(&kind.as_str()) {
        for (i, value) in ["a", "b"].iter().enumerate() {
            oracle::same_raw(
                expected[i].clone(),
                post_source::expected(size, kind, value),
            );
        }
    }
    drop(rules);
    let mut records = Vec::with_capacity(32);
    let mut held = Vec::with_capacity(4);
    let mut endpoints = Vec::with_capacity(4);
    #[cfg(feature = "candidate-profile")]
    candidate_profile::enable();
    println!("{{\"event\":\"start\"}}");
    #[cfg(feature = "alloc-meter")]
    let root = meter::begin();
    let (rules, m) = measure(|| source_rules(kind));
    records.push(("source", 0, m));
    let (prepared, m) = measure(|| Prepared::new(mode, rules, kind));
    records.push(("prepare", 0, m));
    for i in 0..4 {
        let (q, m) =
            measure(|| source_query(size, kind, reverse, if i % 2 == 0 { "a" } else { "b" }));
        records.push(("input", i, m));
        let (mut run, m) = measure(|| prepared.start(q));
        records.push(("setup", i, m));
        let ((answers, complete, ticks, first), m) =
            measure(|| run.collect(cancel && i % 2 == 0, first_clock));
        assert_eq!(first.is_some(), first_clock && !answers.is_empty());
        assert!(first.is_none_or(|ns| ns <= m.ns));
        records.push(("execute_observe", i, m));
        let (_, m) = measure(|| drop(run));
        records.push(("engine_drop", i, m));
        validate(&answers, &expected[i % 2], complete);
        endpoints.push((complete, ticks, answers.len(), first));
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
        .map(|(complete, ticks, answers, first)| {
            format!("{{\"complete\":{complete},\"ticks\":{ticks},\"answers\":{answers},\"first_ns\":{}}}",first.map_or("null".into(), |ns|ns.to_string()))
        })
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "{{\"event\":\"result\",\"meter\":{},\"endpoints\":[{ends}],\"phases\":[{rows}]}}",
        cfg!(feature = "alloc-meter")
    );
    #[cfg(feature = "candidate-profile")]
    println!("{}", candidate_profile::json());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn post_families_preserve_complete_answers_and_prepared_reuse() {
        for kind in post_source::FAMILIES {
            for size in [1, 8] {
                let rules = post_source::rules(kind);
                for mode in [
                    "birth",
                    "birth-miss",
                    "birth-miss-template",
                    "scan",
                    "indexed",
                    "sealed",
                    "active-scan",
                    "active-indexed",
                    "native-scan",
                    "native-indexed",
                    "active-native-scan",
                    "active-native-indexed",
                ] {
                    let p = Prepared::new(mode, rules.clone(), kind);
                    let mut held = vec![];
                    for (value, reverse) in [("a", false), ("b", true)] {
                        let q = post_source::query(size, kind, reverse, value);
                        let expected = post_source::expected(size, kind, value);
                        oracle::same_raw(oracle::run(&rules, &q, 2_000_000), expected.clone());
                        let mut partial = p.start(q.clone());
                        let (a, done, _, _) = partial.collect(true, false);
                        assert!(!done);
                        drop(partial);
                        validate(&a, &expected, false);
                        let mut r = p.start(q);
                        let (actual, done, _, _) = r.collect(false, false);
                        assert!(done);
                        drop(r);
                        held.push((actual, expected));
                    }
                    drop(p);
                    for (actual, expected) in held {
                        validate(&actual, &expected, true);
                    }
                }
            }
        }
    }
}
