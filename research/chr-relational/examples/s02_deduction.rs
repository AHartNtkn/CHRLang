//! Isolated lifecycle runner. Comparative runs require prospective registration.
#[path = "support/deduction_source.rs"]
mod deduction_source;
#[cfg(feature = "deduction-profile")]
#[path = "support/deduction_profile.rs"]
mod profile;
use chr_syntax::{Answer, Query, Rule};
use deduction_source::{Lowered, Schema, Values};
use std::time::Instant;
#[cfg(feature = "alloc-meter")]
#[allow(unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
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
    Lowered(Box<Lowered>),
    Contextual(
        std::sync::Arc<chr_relational::contextual_execute::Prepared>,
        bool,
        bool,
    ),
    Relevant(
        std::sync::Arc<chr_relational::contextual_execute::Prepared>,
        bool,
        bool,
    ),
    Relational(std::sync::Arc<chr_relational::execute::Prepared>),
    Compiled(Box<chr_compiled::PreparedRuleset>, chr_compiled::Access),
}
enum Running {
    Lowered(Box<Values>),
    Contextual(Box<chr_relational::contextual_execute::Engine>),
    Relational(Box<chr_relational::execute::Engine>),
    Compiled(Box<chr_compiled::SearchEngine>),
}
const MODES: [&str; 13] = [
    "validated",
    "persistent-validated",
    "relevant",
    "persistent-relevant",
    "contextual",
    "shared",
    "persistent",
    "persistent-shared",
    "relational",
    "scan",
    "indexed",
    "sealed",
    "lowered",
];
impl Prepared {
    fn new(mode: &str, schema: Schema, rules: Vec<Rule>) -> Self {
        match mode {
            "relevant" | "persistent-relevant" | "validated" | "persistent-validated" => {
                Self::Relevant(
                    chr_relational::contextual_execute::Prepared::new(&rules).unwrap(),
                    mode.starts_with("persistent"),
                    mode.ends_with("validated"),
                )
            }
            "lowered" => Self::Lowered(Box::new(Lowered::new(schema, rules).unwrap())),
            "contextual" | "shared" | "persistent" | "persistent-shared" => Self::Contextual(
                chr_relational::contextual_execute::Prepared::new(&rules).unwrap(),
                mode == "shared" || mode == "persistent-shared",
                mode.starts_with("persistent"),
            ),
            "relational" => {
                Self::Relational(chr_relational::execute::Prepared::new(&rules).unwrap())
            }
            "scan" | "indexed" | "sealed" => Self::Compiled(
                Box::new({
                    let p = chr_compiled::PreparedRuleset::new(rules, None).unwrap();
                    if mode == "sealed" {
                        p.specialize_inferred()
                    } else {
                        p
                    }
                }),
                if mode == "scan" {
                    chr_compiled::Access::Scan
                } else {
                    chr_compiled::Access::Indexed
                },
            ),
            _ => panic!("unknown mode"),
        }
    }
    fn start(&self, input: Query) -> Running {
        match self {
            Self::Relevant(p, persistent, validated) => {
                Running::Contextual(Box::new(if *validated {
                    if *persistent {
                        p.start_persistent_validated_deductions(&input)
                    } else {
                        p.start_validated_deductions(&input)
                    }
                } else if *persistent {
                    p.start_persistent_relevant_deductions(&input)
                } else {
                    p.start_relevant_deductions(&input)
                }))
            }
            Self::Lowered(p) => Running::Lowered(Box::new(p.start(input).unwrap())),
            Self::Contextual(p, shared, persistent) => {
                Running::Contextual(Box::new(if *persistent {
                    p.start_persistent_equality(&input, *shared)
                } else if *shared {
                    p.start_shared_deductions(&input)
                } else {
                    p.start(&input)
                }))
            }
            Self::Relational(p) => Running::Relational(Box::new(p.start(&input))),
            Self::Compiled(p, a) => Running::Compiled(Box::new(
                p.start_search(input, chr_compiled::Policy::Global, *a)
                    .unwrap(),
            )),
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
                Self::Lowered(e) => match e.next() {
                    Some(a) => Some(a),
                    None => return (answers, first, true),
                },
                Self::Contextual(e) => match e.advance() {
                    chr_relational::contextual_execute::Step::Progress => None,
                    chr_relational::contextual_execute::Step::Answer(a) => Some(a),
                    chr_relational::contextual_execute::Step::Exhausted => {
                        return (answers, first, true);
                    }
                },
                Self::Relational(e) => match e.advance() {
                    chr_relational::execute::Step::Progress => None,
                    chr_relational::execute::Step::Answer(a) => Some(a),
                    chr_relational::execute::Step::Exhausted => return (answers, first, true),
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
    answer_drop: Option<Measurement>,
    answer_hold: Option<Measurement>,
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
    if cfg!(feature = "deduction-work")
        || cfg!(feature = "local-work")
        || chr_persistent::COLLECT_KERNEL_METRICS
        || chr_persistent::COLLECT_METRICS
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
    let reverse = match args[6].as_str() {
        "0" => false,
        "1" => true,
        _ => panic!("reverse must be 0/1"),
    };
    let cancel = args.get(7).map(|s| s.parse::<usize>().unwrap());
    let retain = match std::env::var("DEDUCTION_RETAIN").as_deref() {
        Ok("all") => true,
        Ok("immediate") | Err(_) => false,
        Ok(_) => panic!("unknown retention policy"),
    };
    #[cfg(feature = "deduction-profile")]
    profile::enable();
    let schema = Schema::new(family, resource);
    // Exact input fixtures and independent expected answers precede all primary phases.
    let source = schema.rules();
    let fixtures = (0..count.min(2))
        .map(|i| {
            let query = schema.query(n + i, (i % 2 == 0) == reverse);
            let expected = oracle::run(&source, &query, 2_000_000);
            assert_eq!(expected.len(), schema.branches);
            oracle::same_raw(expected.clone(), schema.values().collect());
            (query, expected)
        })
        .collect::<Vec<_>>();
    drop(source);
    let mut samples = Vec::with_capacity(count);
    let mut held = Vec::with_capacity(if retain { count } else { 0 });
    // Initialize reporting before the allocation restoration baseline.
    println!("{{\"event\":\"start\",\"mode\":\"{mode}\"}}");
    #[cfg(feature = "alloc-meter")]
    let root = meter::begin();
    let (rules, source_build) = measure(|| schema.rules());
    let (prepared, preparation) = measure(|| Prepared::new(mode, schema, rules));
    for i in 0..count {
        #[cfg(feature = "alloc-meter")]
        let query_live = meter::begin();
        let (input, input_time) = measure(|| schema.query(n + i % 2, (i % 2 == 0) == reverse));
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
        let (answer_drop, answer_hold) = if retain {
            let (_, held_time) = measure(|| held.push(answers));
            (None, Some(held_time))
        } else {
            let (_, dropped_time) = measure(|| drop(answers));
            (Some(dropped_time), None)
        };
        #[cfg(feature = "alloc-meter")]
        {
            let live = meter::end(query_live);
            if !retain {
                assert_eq!(
                    live.live_start, live.live_end,
                    "query allocations remain after disposal"
                );
            }
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
            answer_hold,
        });
    }
    let (_, prepared_drop) = measure(|| drop(prepared));
    for (i, answers) in held.iter().enumerate() {
        let expected = &fixtures[i % fixtures.len()].1;
        if samples[i].complete {
            oracle::same_raw(answers.clone(), expected.clone());
        } else {
            let mut remaining = expected.clone();
            for answer in answers {
                let pos = remaining
                    .iter()
                    .position(|a| chr_observe::equivalent(a, answer, &mut Default::default()))
                    .expect("retained cancelled answer changed after producer disposal");
                remaining.swap_remove(pos);
            }
        }
    }
    let (_, consumer_drop) = measure(|| held.clear());
    #[cfg(feature = "alloc-meter")]
    {
        let live = meter::end(root);
        assert_eq!(
            live.live_start, live.live_end,
            "prepared/source allocations remain after disposal"
        );
    }
    #[cfg(feature = "deduction-profile")]
    let attribution = profile::json();
    #[cfg(not(feature = "deduction-profile"))]
    let attribution = "null";
    let sample_json=samples.into_iter().map(|s|format!("{{\"depth\":{},\"complete\":{},\"answers\":{},\"first_answer_ns\":{},\"input_build\":{},\"setup\":{},\"execute_observe\":{},\"engine_drop\":{},\"answer_drop\":{},\"answer_hold\":{}}}",s.depth,s.complete,s.answers,s.first.map_or("null".into(),|n|n.to_string()),s.input.json(),s.setup.json(),s.execute.json(),s.engine_drop.json(),s.answer_drop.map_or("null".into(), Measurement::json),s.answer_hold.map_or("null".into(), Measurement::json))).collect::<Vec<_>>().join(",");
    println!(
        "{{\"event\":\"result\",\"mode\":\"{mode}\",\"family\":\"{family}\",\"resource\":{resource},\"meter\":{},\"counters\":false,\"source_build\":{},\"preparation\":{},\"prepared_drop\":{},\"consumer_drop\":{},\"retained\":{retain},\"attribution\":{attribution},\"samples\":[{sample_json}]}}",
        cfg!(feature = "alloc-meter"),
        source_build.json(),
        preparation.json(),
        prepared_drop.json(),
        consumer_drop.json()
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lowering_checks_the_complete_source_and_query() {
        use deduction_source::Lowered;
        let schema = Schema::new("changed", true);
        let mut source = schema.rules();
        source[0].body = chr_syntax::Goal::True;
        assert!(Lowered::new(schema, source).is_err());
        let p = Lowered::new(schema, schema.rules()).unwrap();
        let mut q = schema.query(8, false);
        q.constraints
            .push(chr_syntax::c("token", [chr_syntax::atom("key0")]));
        assert!(p.start(q).is_err());
        let mut q = schema.query(8, false);
        q.outputs.pop();
        assert!(p.start(q).is_err());
        let mut q = schema.query(8, false);
        q.constraints[0].args[0] = chr_syntax::v(9000);
        assert!(p.start(q).is_err());
        let mut q = schema.query(8, false);
        q.constraints.retain(|c| c.name != "token");
        assert!(p.start(q).is_err());
    }
    #[test]
    fn all_paths_match_source_and_survive_interruption() {
        for family in ["single", "shared", "distinct", "changed"] {
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
}
