//! Isolated lifecycle runner. Comparative runs require prospective registration.
#[path = "support/prefix_source.rs"]
mod prefix_source;
#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use chr_syntax::{Answer, Query, Rule};
use prefix_source::Schema;
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
    Original(Box<chr_compiled::PreparedRuleset>),
    Lowered(chr_compiled::pure_prefix::Program, bool),
}
struct Running(chr_compiled::SearchEngine);
const MODES: [&str; 4] = ["original", "sealed", "lowered", "lowered-sealed"];
impl Prepared {
    fn new(mode: &str, _schema: Schema, rules: Vec<Rule>) -> Self {
        match mode {
            "original" | "sealed" => {
                let p = chr_compiled::PreparedRuleset::new(rules, None).unwrap();
                Self::Original(Box::new(if mode == "sealed" {
                    p.specialize_inferred()
                } else {
                    p
                }))
            }
            "lowered" | "lowered-sealed" => Self::Lowered(
                chr_compiled::pure_prefix::Program::new(&rules).unwrap(),
                mode == "lowered-sealed",
            ),
            _ => panic!("unknown mode"),
        }
    }
    fn start(&self, input: Query) -> Running {
        let e = match self {
            Self::Original(p) => p
                .start_search(
                    input,
                    chr_compiled::Policy::Global,
                    chr_compiled::Access::Scan,
                )
                .unwrap(),
            Self::Lowered(p, sealed) => {
                let (rules, q) = p.lower(&input).unwrap();
                let compiled = chr_compiled::PreparedRuleset::new(rules, None).unwrap();
                let compiled = if *sealed {
                    compiled.specialize_inferred()
                } else {
                    compiled
                };
                compiled
                    .start_search(q, chr_compiled::Policy::Global, chr_compiled::Access::Scan)
                    .unwrap()
            }
        };
        Running(e)
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
            let answer = match self.0.tick() {
                chr_compiled::SearchEvent::Complete(mut b) => Some(b.engine.observe().unwrap()),
                chr_compiled::SearchEvent::Exhausted => return (answers, first, true),
                _ => None,
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
    if chr_compiled::COLLECT_METRICS
        || chr_compiled::COLLECT_KERNEL_METRICS
        || chr_observe::COLLECT_METRICS
    {
        panic!("cost builds require counters disabled");
    }
    assert!(
        args.len() == 6 || args.len() == 7,
        "mode family depth queries resource [cancel_ticks]"
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
    let cancel = args.get(6).map(|s| s.parse::<usize>().unwrap());
    let schema = Schema::new(family, resource, n);
    // Exact input fixtures and independent expected answers precede all primary phases.
    let source = schema.rules();
    let fixtures = (0..count.min(2))
        .map(|i| {
            let query = schema.query(n + i, i % 2 == 0);
            let expected = oracle::run(&source, &query, 2_000_000);
            assert_eq!(expected.len(), 1 << schema.bits);
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
        let (input, input_time) = measure(|| schema.query(n + i % 2, i % 2 == 0));
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
            depth: n,
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
    fn all_paths_match_source_and_survive_interruption() {
        for family in ["plain1", "plain4", "choice1", "choice4"] {
            for resource in [false, true] {
                let schema = Schema::new(family, resource, 3);
                let rules = schema.rules();
                for mode in MODES {
                    let prepared = Prepared::new(mode, schema, rules.clone());
                    let mut cancelled = prepared.start(schema.query(8, false));
                    let (_, _, complete) = cancelled.collect(Some(1));
                    assert!(!complete);
                    drop(cancelled);
                    for n in [0, 1, 8] {
                        let query = schema.query(n, n % 2 == 0);
                        let expected = oracle::run(&rules, &query, 200_000);
                        let mut run = prepared.start(query);
                        let (answers, _, complete) = run.collect(None);
                        assert!(complete);
                        drop(run);
                        oracle::same_raw(answers, expected);
                    }
                }
            }
        }
    }
}
