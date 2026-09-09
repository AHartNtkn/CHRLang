//! Isolated lifecycle runner. Comparative runs require prospective registration.
mod demand_source;
use chr_syntax::{Answer, Query, Rule};
use demand_source::{Lowered, Schema, Values};
use std::time::Instant;
#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../tests/runtime_support/mod.rs"]
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
    Graph(Box<chr_direct_choice::engine::PreparedRuleset>),
    Conditional(Box<chr_direct_conditional::engine::PreparedRuleset>),
    Compiled(Box<chr_compiled::PreparedRuleset>, chr_compiled::Access),
    Lowered(Box<Lowered>),
}
enum Running {
    Demand(Box<chr_direct_choice::demand::Run>),
    Graph(Box<chr_direct_choice::engine::Engine>),
    Conditional(Box<chr_direct_conditional::engine::Engine>),
    Compiled(Box<chr_compiled::SearchEngine>),
    Lowered(Box<Values>),
}
const MODES: [&str; 7] = [
    "demand",
    "current",
    "graph",
    "conditional",
    "scan",
    "indexed",
    "lowered",
];
impl Prepared {
    fn new(mode: &str, schema: Schema, rules: Vec<Rule>) -> Self {
        match mode {
            "demand" | "current" => Self::Demand(Box::new(
                chr_direct_choice::demand::Prepared::with_reuse(
                    rules,
                    if mode == "current" {
                        chr_direct_choice::demand::Reuse::CurrentContext
                    } else {
                        chr_direct_choice::demand::Reuse::StaticBirth
                    },
                )
                .unwrap(),
            )),
            "graph" => Self::Graph(Box::new(
                chr_direct_choice::engine::PreparedRuleset::new(rules).unwrap(),
            )),
            "conditional" => Self::Conditional(Box::new(
                chr_direct_conditional::engine::PreparedRuleset::new(rules).unwrap(),
            )),
            "scan" | "indexed" => Self::Compiled(
                Box::new(chr_compiled::PreparedRuleset::new(rules, None).unwrap()),
                if mode == "scan" {
                    chr_compiled::Access::Scan
                } else {
                    chr_compiled::Access::Indexed
                },
            ),
            "lowered" => Self::Lowered(Box::new(Lowered::new(schema, rules).unwrap())),
            _ => panic!("unknown mode"),
        }
    }
    fn start(&self, input: Query) -> Running {
        match self {
            Self::Demand(p) => Running::Demand(Box::new(p.start(input).unwrap())),
            Self::Graph(p) => Running::Graph(Box::new(p.start(input).unwrap())),
            Self::Conditional(p) => Running::Conditional(Box::new(p.start(input).unwrap())),
            Self::Compiled(p, access) => Running::Compiled(Box::new(
                p.start_search(input, chr_compiled::Policy::Global, *access)
                    .unwrap(),
            )),
            Self::Lowered(p) => Running::Lowered(Box::new(p.start(input).unwrap())),
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
                Self::Demand(e) => match e.tick() {
                    chr_direct_choice::demand::Event::Progress => None,
                    chr_direct_choice::demand::Event::Answer(a) => Some(a),
                    chr_direct_choice::demand::Event::Exhausted => return (answers, first, true),
                },
                Self::Graph(e) => match e.tick() {
                    chr_direct_choice::engine::Event::Progress => None,
                    chr_direct_choice::engine::Event::Answer(a) => Some(a),
                    chr_direct_choice::engine::Event::Exhausted => return (answers, first, true),
                },
                Self::Conditional(e) => match e.tick() {
                    chr_direct_conditional::engine::Event::Progress => None,
                    chr_direct_conditional::engine::Event::Answer(a) => Some(a),
                    chr_direct_conditional::engine::Event::Exhausted => {
                        return (answers, first, true);
                    }
                },
                Self::Compiled(e) => match e.tick() {
                    chr_compiled::SearchEvent::Complete(mut b) => Some(b.engine.observe().unwrap()),
                    chr_compiled::SearchEvent::Exhausted => return (answers, first, true),
                    _ => None,
                },
                Self::Lowered(e) => match e.next() {
                    Some(a) => Some(a),
                    None => return (answers, first, true),
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
    if cfg!(feature = "metrics")
        || chr_compiled::COLLECT_METRICS
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
    let schema = Schema::new(family, resource);
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
    fn all_paths_and_lowering_match_source_and_survive_interruption() {
        for family in ["plain", "opaque", "discriminate"] {
            for resource in [false, true] {
                let schema = Schema::new(family, resource);
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
    #[test]
    fn lowering_rejects_source_and_query_near_misses() {
        let schema = Schema::new("opaque", true);
        let mut rules = schema.rules();
        rules[0].body = chr_syntax::Goal::True;
        assert!(Lowered::new(schema, rules).is_err());
        let p = Lowered::new(schema, schema.rules()).unwrap();
        let mut q = schema.query(1, false);
        q.constraints.push(c("token", []));
        assert!(p.start(q).is_err());
        let mut q = schema.query(1, false);
        q.outputs.pop();
        assert!(p.start(q).is_err());
        let mut q = schema.query(1, false);
        q.constraints[3].args[0] = chr_syntax::v(9999);
        assert!(p.start(q).is_err());
    }
    use chr_syntax::c;
}
