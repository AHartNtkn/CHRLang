//! Registered isolated-process dependency ownership experiment.
#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../tests/runtime_support/mod.rs"]
mod oracle;
#[allow(dead_code)]
#[path = "support/post_continuation_source.rs"]
mod source;
#[path = "support/static_posts.rs"]
mod static_posts;
#[path = "support/value_choices.rs"]
mod value_choices;
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
    Conditional(Box<chr_direct_conditional::engine::PreparedRuleset>),
    Compiled(
        Box<chr_compiled::PreparedRuleset>,
        chr_compiled::Access,
        chr_compiled::Policy,
    ),
}
enum Running {
    Demand(Box<Run>),
    Conditional(Box<chr_direct_conditional::engine::Engine>),
    Compiled(Box<chr_compiled::SearchEngine>),
}
impl Prepared {
    fn new(
        mode: &str,
        rules: Vec<Rule>,
        kind: &str,
        choices: usize,
        history: bool,
        initialize: bool,
    ) -> Self {
        if mode == "conditional" {
            return Self::Conditional(Box::new(
                chr_direct_conditional::engine::PreparedRuleset::new(rules).unwrap(),
            ));
        }
        if [
            "scan",
            "indexed",
            "sealed-scan",
            "sealed-indexed",
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
                        let id = source::FAMILIES.iter().position(|f| *f == kind).unwrap() * 6
                            + [0, 1, 3].iter().position(|k| *k == choices).unwrap() * 2
                            + usize::from(history);
                        Some(if initialize {
                            chr_compiled::access_initialized_continuation_bundled(id)
                        } else {
                            chr_compiled::access_continuation_bundled(id)
                        })
                    } else {
                        None
                    };
                    let p = chr_compiled::PreparedRuleset::new(rules, code).unwrap();
                    if mode.starts_with("sealed-") {
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
            Self::Conditional(p) => Running::Conditional(Box::new(p.start(q).unwrap())),
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
                Self::Conditional(r) => match r.tick() {
                    chr_direct_conditional::engine::Event::Answer(a) => answers.push(a),
                    chr_direct_conditional::engine::Event::Exhausted => {
                        return (answers, true, tick, first);
                    }
                    _ => (),
                },
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
    let first_clock = false;
    assert_eq!(args.len(), 11);
    let initialize = args[10].parse::<bool>().unwrap();
    let mode = &args[1];
    let kind = &args[2];
    let choices = args[3].parse::<usize>().unwrap();
    let depth = args[4].parse::<usize>().unwrap();
    let history = args[5].parse::<bool>().unwrap();
    let reverse = args[6].parse::<bool>().unwrap();
    let retention = &args[7];
    let counted = args[8].parse::<bool>().unwrap();
    let cancel = args[9].parse::<bool>().unwrap();
    assert!(["immediate", "window", "all"].contains(&retention.as_str()));
    let rules = source::rules(kind, choices, history);
    let expected = [0, 1].map(|seed| {
        let answers = oracle::run(
            &rules,
            &source::query(kind, choices, depth, seed, reverse),
            2_000_000,
        );
        oracle::same_raw(
            answers.clone(),
            source::expected(kind, choices, seed, history),
        );
        answers
    });
    drop(rules);
    let mut records = Vec::with_capacity(64);
    let mut held = Vec::with_capacity(4);
    let mut endpoints = Vec::with_capacity(4);
    println!("{{\"event\":\"start\"}}");
    #[cfg(feature = "alloc-meter")]
    let root = meter::begin();
    let (rules, m) = measure(|| source::rules(kind, choices, history));
    records.push(("source", 0, m));
    let mut rules = Some(rules);
    let (count_program, m) = measure(|| {
        counted
            .then(|| chr_compiled::resource_count::Program::infer(rules.as_ref().unwrap()).unwrap())
    });
    records.push(("count_prepare", 0, m));
    let (static_program, m) = measure(|| {
        (mode.starts_with("birth") || initialize)
            .then(|| static_posts::Prepared::infer(rules.as_ref().unwrap()).unwrap())
    });
    records.push(("static_prepare", 0, m));
    let (engine_rules, m) = measure(|| {
        if let Some(p) = &static_program {
            if mode.starts_with("birth") {
                value_choices::lower(p.rules())
            } else {
                p.rules().to_vec()
            }
        } else {
            rules.take().unwrap()
        }
    });
    records.push(("value_prepare", 0, m));
    let (prepared, m) =
        measure(|| Prepared::new(mode, engine_rules, kind, choices, history, initialize));
    records.push(("engine_prepare", 0, m));
    let (_, m) = measure(|| drop(rules));
    records.push(("source_drop", 0, m));
    for i in 0..4 {
        let (q, m) = measure(|| source::query(kind, choices, depth, i % 2, reverse));
        records.push(("input", i, m));
        let (q, m) = measure(|| {
            if let Some(p) = &count_program {
                p.lower(&q).unwrap()
            } else {
                q
            }
        });
        records.push(("count_query", i, m));
        let (q, m) = measure(|| {
            if let Some(p) = &static_program {
                p.initialize(&q)
            } else {
                q
            }
        });
        records.push(("static_query", i, m));
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
    let (_, m) = measure(|| drop((prepared, static_program, count_program)));
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
}
