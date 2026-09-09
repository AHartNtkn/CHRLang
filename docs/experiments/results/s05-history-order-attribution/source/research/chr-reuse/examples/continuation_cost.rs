//! Isolated lifecycle runner. Comparative runs require prospective registration.
#[path = "support/continuation_source.rs"]
mod continuation_source;
#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use chr_syntax::{Answer, Query, Rule};
use continuation_source::Schema;
use std::time::Instant;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[derive(Clone, Copy)]
struct Measurement {
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn measure<T>(f: impl FnOnce() -> T) -> (T, Measurement) {
    #[cfg(feature = "alloc-meter")]
    let memory = meter::begin();
    let start = Instant::now();
    let value = f();
    let ns = start.elapsed().as_nanos();
    #[cfg(feature = "alloc-meter")]
    let memory = meter::end(memory);
    (
        value,
        Measurement {
            ns,
            #[cfg(feature = "alloc-meter")]
            memory,
        },
    )
}
impl Measurement {
    fn json(self) -> String {
        #[cfg(feature = "alloc-meter")]
        let memory = self.memory.json();
        #[cfg(not(feature = "alloc-meter"))]
        let memory = "null";
        format!("{{\"ns\":{},\"memory\":{}}}", self.ns, memory)
    }
}
enum Prepared {
    Demand(Box<chr_direct_choice::demand::Prepared>),
    Compiled(Box<chr_compiled::PreparedRuleset>, chr_compiled::Access),
    Reuse(chr_reuse::continuations::Prepared),
}
enum Running {
    Demand(Box<chr_direct_choice::demand::Run>),
    Compiled(Box<chr_compiled::SearchEngine>),
    Reuse(Box<chr_reuse::continuations::Search>),
}
const MODES: [&str; 7] = [
    "direct",
    "exact",
    "alpha",
    "live",
    "scan",
    "sealed",
    "dependencies",
];
impl Prepared {
    fn new(mode: &str, _schema: Schema, rules: Vec<Rule>) -> Self {
        match mode {
            "dependencies" | "templates" => {
                let p = chr_direct_choice::demand::Prepared::with_reuse(
                    rules,
                    chr_direct_choice::demand::Reuse::MatchDependencies,
                )
                .unwrap();
                Self::Demand(Box::new(if mode == "templates" {
                    p.with_derivation_templates()
                } else {
                    p
                }))
            }
            "scan" | "indexed" | "sealed" => {
                let p = chr_compiled::PreparedRuleset::new(rules, None).unwrap();
                Self::Compiled(
                    Box::new(if mode == "sealed" {
                        p.specialize_inferred()
                    } else {
                        p
                    }),
                    if mode == "indexed" {
                        chr_compiled::Access::Indexed
                    } else {
                        chr_compiled::Access::Scan
                    },
                )
            }
            "direct" | "exact" | "alpha" | "live" => {
                use chr_reuse::continuations::Mode;
                let mode = match mode {
                    "direct" => Mode::Direct,
                    "exact" => Mode::ExactIds,
                    "alpha" => Mode::Alpha,
                    _ => Mode::AlphaLive,
                };
                Self::Reuse(chr_reuse::continuations::Prepared::new(rules, mode).unwrap())
            }
            _ => panic!("unknown mode"),
        }
    }
    fn start(&self, input: Query) -> Running {
        match self {
            Self::Demand(p) => Running::Demand(Box::new(p.start(input).unwrap())),
            Self::Compiled(p, access) => Running::Compiled(Box::new(
                p.start_search(input, chr_compiled::Policy::Global, *access)
                    .unwrap(),
            )),
            Self::Reuse(p) => Running::Reuse(Box::new(p.start(input).unwrap())),
        }
    }
}
impl Running {
    fn collect(&mut self, cancel: Option<usize>) -> (Vec<Answer>, Option<u128>, bool) {
        let start = Instant::now();
        let mut first = None;
        let mut answers = vec![];
        for step in 0..2_000_000 {
            if cancel == Some(step) {
                return (answers, first, false);
            }
            let answer = match self {
                Self::Reuse(e) => {
                    let mut batch = e.advance(1);
                    if let Some(a) = batch.answers.pop() {
                        Some(a)
                    } else if batch.exhausted {
                        return (answers, first, true);
                    } else {
                        None
                    }
                }
                Self::Demand(e) => match e.tick() {
                    chr_direct_choice::demand::Event::Progress => None,
                    chr_direct_choice::demand::Event::Answer(a) => Some(a),
                    chr_direct_choice::demand::Event::Exhausted => return (answers, first, true),
                },
                Self::Compiled(e) => match e.tick() {
                    chr_compiled::SearchEvent::Complete(mut b) => Some(b.engine.observe().unwrap()),
                    chr_compiled::SearchEvent::Exhausted => return (answers, first, true),
                    _ => None,
                },
            };
            if let Some(a) = answer {
                if first.is_none() {
                    first = Some(start.elapsed().as_nanos())
                }
                answers.push(a);
            }
        }
        panic!("query service bound exceeded");
    }
}
struct Sample {
    depth: usize,
    complete: bool,
    answers: usize,
    first: Option<u128>,
    input: Measurement,
    setup: Measurement,
    execute: Measurement,
    engine_drop: Measurement,
    answer_drop: Measurement,
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.get(1).map(String::as_str) == Some("meter-check") {
        #[cfg(feature = "alloc-meter")]
        {
            meter::self_check().unwrap();
            println!("meter check passed");
            return;
        }
        #[cfg(not(feature = "alloc-meter"))]
        panic!("meter is not enabled");
    }
    if chr_reuse::continuations::COLLECT_METRICS
        || chr_direct_choice::demand::COLLECT_WORK_DIAGNOSTICS
        || cfg!(feature = "metrics")
        || chr_compiled::COLLECT_METRICS
        || chr_compiled::COLLECT_KERNEL_METRICS
        || chr_observe::COLLECT_METRICS
    {
        panic!("cost builds require counters disabled");
    }
    assert!(
        args.len() == 7 || args.len() == 8,
        "mode family depth queries resource reverse_first [cancel_ticks]"
    );
    let mode = args[1].as_str();
    assert!(MODES.contains(&mode));
    let family = args[2].as_str();
    let n = args[3].parse::<usize>().unwrap();
    let count = args[4].parse::<usize>().unwrap();
    let resource = match args[5].as_str() {
        "0" => false,
        "1" => true,
        _ => panic!("resource must be 0 or 1"),
    };
    assert!(n <= 128 && (1..=64).contains(&count));
    let reverse_first = match args[6].as_str() {
        "0" => false,
        "1" => true,
        _ => panic!("reverse_first must be 0 or 1"),
    };
    let cancel = args.get(7).map(|s| s.parse::<usize>().unwrap());
    let schema = Schema::new(family, resource);
    // Exact input fixtures and independent expected answers precede all primary phases.
    let source = schema.rules();
    let fixtures = (0..count.min(2))
        .map(|i| {
            let query = schema.query(n + i, (i % 2 == 0) == reverse_first);
            let expected = oracle::run(&source, &query, 2_000_000);
            assert_eq!(expected.len(), schema.answer_count());
            (query, expected)
        })
        .collect::<Vec<_>>();
    drop(source);
    let mut samples = Vec::with_capacity(count);
    // Initialize reporting before the allocation restoration baseline.
    println!("{{\"event\":\"start\",\"mode\":\"{mode}\"}}");
    #[cfg(feature = "alloc-meter")]
    let root = meter::begin();
    let (rules, source_build) = measure(|| schema.rules());
    let (prepared, preparation) = measure(|| Prepared::new(mode, schema, rules));
    for i in 0..count {
        #[cfg(feature = "alloc-meter")]
        let query_live = meter::begin();
        let (input, input_time) =
            measure(|| schema.query(n + i % 2, (i % 2 == 0) == reverse_first));
        assert_eq!(&input, &fixtures[i % fixtures.len()].0);
        let (mut engine, setup) = measure(|| prepared.start(input));
        let ((answers, first, complete), execute) =
            measure(|| engine.collect(if i % 2 == 0 { cancel } else { None }));
        let (_, engine_drop) = measure(|| drop(engine));
        let actual_count = answers.len();
        let expected = &fixtures[i % fixtures.len()].1;
        if complete {
            oracle::same_raw(answers.clone(), expected.clone());
        } else {
            let mut remaining = expected.clone();
            for answer in &answers {
                let pos = remaining
                    .iter()
                    .position(|a| chr_observe::equivalent(a, answer, &mut Default::default()))
                    .expect("cancelled prefix contained an invalid answer");
                remaining.swap_remove(pos);
            }
        }
        let (_, answer_drop) = measure(|| drop(answers));
        #[cfg(feature = "alloc-meter")]
        {
            let live = meter::end(query_live);
            assert_eq!(
                live.live_start, live.live_end,
                "query allocations remain after disposal"
            );
        }
        samples.push(Sample {
            depth: n + i % 2,
            complete,
            answers: actual_count,
            first,
            input: input_time,
            setup,
            execute,
            engine_drop,
            answer_drop,
        });
    }
    let (_, prepared_drop) = measure(|| drop(prepared));
    #[cfg(feature = "alloc-meter")]
    {
        let live = meter::end(root);
        assert_eq!(
            live.live_start, live.live_end,
            "prepared/source allocations remain after disposal"
        );
    }
    let sample_json=samples.into_iter().map(|s|format!("{{\"depth\":{},\"complete\":{},\"answers\":{},\"first_answer_ns\":{},\"input_build\":{},\"setup\":{},\"execute_observe\":{},\"engine_drop\":{},\"answer_drop\":{}}}",s.depth,s.complete,s.answers,s.first.map_or("null".into(),|n|n.to_string()),s.input.json(),s.setup.json(),s.execute.json(),s.engine_drop.json(),s.answer_drop.json())).collect::<Vec<_>>().join(",");
    println!(
        "{{\"event\":\"result\",\"mode\":\"{mode}\",\"family\":\"{family}\",\"resource\":{resource},\"meter\":{},\"counters\":false,\"source_build\":{},\"preparation\":{},\"prepared_drop\":{},\"samples\":[{sample_json}]}}",
        cfg!(feature = "alloc-meter"),
        source_build.json(),
        preparation.json(),
        prepared_drop.json()
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_complete_answers_and_cancellation() {
        for family in ["exact", "rename", "history", "history-early", "distinct"] {
            for resource in [false, true] {
                let schema = Schema::new(family, resource);
                let rules = schema.rules();
                for mode in MODES {
                    let prepared = Prepared::new(mode, schema, rules.clone());
                    for n in [0, 1, 8] {
                        for reverse in [false, true] {
                            let query = schema.query(n, reverse);
                            let expected = oracle::run(&rules, &query, 10000);
                            assert_eq!(expected.len(), 4);
                            let mut run = prepared.start(query.clone());
                            let (actual, _, done) = run.collect(None);
                            assert!(done);
                            oracle::same_raw(actual, expected);
                            let mut run = prepared.start(query);
                            let (_, _, done) = run.collect(Some(1));
                            assert!(!done);
                        }
                    }
                }
            }
        }
    }
}
