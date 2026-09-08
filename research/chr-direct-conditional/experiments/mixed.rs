//! Registered mixed-phase lifecycle runner. No runtime policy changes.
use chr_syntax::{Answer, Query};
use std::time::Instant;
#[path = "../tests/mixed_support/mod.rs"]
mod fixtures;
#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
const TICK_LIMIT: usize = 20_000_000;
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
#[derive(Default)]
struct Work {
    applications: u64,
    choices: u64,
    candidates: u64,
    specialized_applications: u64,
    cursor_steps: u64,
    history_checks: u64,
}
impl Work {
    fn add(&mut self, stats: &chr_compiled::Stats) {
        if cfg!(feature = "metrics") {
            self.applications += stats.applications;
            self.candidates += stats.candidate_visits + stats.specialized_candidates;
            self.specialized_applications += stats.specialized_applications;
            self.cursor_steps += stats.cursor_steps;
            self.history_checks += stats.history_checks;
        }
    }
    fn json(&self) -> String {
        if cfg!(feature = "metrics") {
            format!(
                "{{\"applications\":{},\"choices\":{},\"candidates\":{},\"specialized_applications\":{},\"cursor_steps\":{},\"history_checks\":{}}}",
                self.applications,
                self.choices,
                self.candidates,
                self.specialized_applications,
                self.cursor_steps,
                self.history_checks
            )
        } else {
            "null".into()
        }
    }
}
#[derive(Clone, Copy)]
enum Backend {
    Conditional,
    Specialized,
}
impl Backend {
    fn name(self) -> &'static str {
        match self {
            Self::Conditional => "conditional",
            Self::Specialized => "specialized",
        }
    }
}
#[derive(Clone, Copy)]
struct Config {
    backend: Backend,
    pre: usize,
    post: usize,
    queries: usize,
}
enum Prepared {
    Conditional(chr_direct_conditional::engine::PreparedRuleset),
    Specialized(chr_compiled::PreparedRuleset),
}
#[allow(clippy::large_enum_variant)]
enum Running {
    Conditional(chr_direct_conditional::engine::Engine),
    Specialized(chr_compiled::SearchEngine),
}
impl Prepared {
    fn start(&self, query: Query) -> Result<Running, String> {
        match self {
            Self::Conditional(p) => p.start(query).map(Running::Conditional),
            Self::Specialized(p) => p
                .start_search(
                    query,
                    chr_compiled::Policy::Global,
                    chr_compiled::Access::Indexed,
                )
                .map(Running::Specialized),
        }
    }
}
struct Collected {
    answers: Vec<Answer>,
    ticks: usize,
    first_answer_ns: Option<u128>,
    exhausted: bool,
    failed: bool,
    work: Work,
}
impl Running {
    // Full observation and completed explicit branch disposal belong to service.
    // Diagnostic segment counters are enabled only in the work build.
    fn collect(&mut self) -> Collected {
        let start = Instant::now();
        let mut result = Collected {
            answers: vec![],
            ticks: 0,
            first_answer_ns: None,
            exhausted: false,
            failed: false,
            work: Work::default(),
        };
        while result.ticks < TICK_LIMIT {
            result.ticks += 1;
            let answer = match self {
                Self::Conditional(e) => match e.tick() {
                    chr_direct_conditional::engine::Event::Progress => None,
                    chr_direct_conditional::engine::Event::Answer(a) => Some(a),
                    chr_direct_conditional::engine::Event::Exhausted => {
                        result.exhausted = true;
                        break;
                    }
                },
                Self::Specialized(e) => match e.tick() {
                    chr_compiled::SearchEvent::Complete(mut b) => {
                        let answer = b
                            .engine
                            .observe()
                            .expect("successful branch has full answer");
                        result.work.add(b.engine.stats());
                        drop(b);
                        Some(answer)
                    }
                    chr_compiled::SearchEvent::Split { work, .. } => {
                        if let Some(work) = work {
                            result.work.add(&work);
                            result.work.choices += 1;
                        }
                        None
                    }
                    chr_compiled::SearchEvent::Failed(b) => {
                        result.work.add(b.engine.stats());
                        result.failed = true;
                        break;
                    }
                    chr_compiled::SearchEvent::Progress => None,
                    chr_compiled::SearchEvent::Exhausted => {
                        result.exhausted = true;
                        break;
                    }
                },
            };
            if let Some(answer) = answer {
                if result.first_answer_ns.is_none() {
                    result.first_answer_ns = Some(start.elapsed().as_nanos());
                }
                result.answers.push(answer);
                // More answers violate this finite source; stop with evidence.
                if result.answers.len() > fixtures::RAW_ANSWERS {
                    break;
                }
            }
        }
        if cfg!(feature = "metrics")
            && let Self::Conditional(e) = self
        {
            result.work.applications = e.stats().applications;
            result.work.choices = e.stats().births;
            result.work.candidates = e.stats().discovered_tuples;
        }
        result
    }
}
struct Sample {
    pre: usize,
    post: usize,
    setup: Measurement,
    execution_observation: Measurement,
    first_answer_ns: Option<u128>,
    validation_ns: u128,
    engine_drop: Measurement,
    outputs_drop: Measurement,
    ticks: usize,
    answers: usize,
    exhausted: bool,
    cutoff: bool,
    work: Work,
}
struct Report {
    config: Config,
    harness_setup_ns: u128,
    baseline: Measurement,
    prepare: Measurement,
    source_clone_ns: u128,
    samples: Vec<Sample>,
    prepared_drop: Measurement,
    final_reading: Measurement,
}
fn run(config: Config) -> Result<Report, String> {
    if cfg!(feature = "metrics") != chr_compiled::COLLECT_METRICS {
        return Err("compiled and conditional metrics differ".into());
    }
    if config.queries == 0 || config.queries > 1000 || config.pre > 4096 || config.post > 4096 {
        return Err("queries must be 1..1000 and work depths at most 4096".into());
    }
    if cfg!(feature = "metrics") != chr_observe::COLLECT_METRICS {
        return Err("observer and conditional metrics differ".into());
    }
    let harness = Instant::now();
    let source = fixtures::rules();
    let queries = (0..config.queries)
        .map(|i| fixtures::query(config.pre + i % 2, config.post + i % 2))
        .collect::<Vec<_>>();
    // Fixture and record storage remain alive from baseline through final reading.
    let mut samples = Vec::with_capacity(config.queries);
    let harness_setup_ns = harness.elapsed().as_nanos();
    let (_, baseline) = measure(|| ());
    let ((prepared, source_clone_ns), prepare) = measure(|| {
        let start = Instant::now();
        let rules = source.clone();
        let source_clone_ns = start.elapsed().as_nanos();
        let prepared = match config.backend {
            Backend::Conditional => chr_direct_conditional::engine::PreparedRuleset::new(rules)
                .map(Prepared::Conditional),
            Backend::Specialized => chr_compiled::PreparedRuleset::new(rules, None)
                .map(|p| Prepared::Specialized(p.specialize_inferred())),
        };
        (prepared, source_clone_ns)
    });
    let prepared = prepared?;
    for (i, query) in queries.iter().enumerate() {
        let (engine, setup) = measure(|| prepared.start(query.clone()));
        let mut engine = engine?;
        let (collected, execution_observation) = measure(|| engine.collect());
        let validation = Instant::now();
        let mut seen = 0u16;
        for answer in &collected.answers {
            let key =
                fixtures::answer_key(answer).ok_or("complete answer violates source contract")?;
            let bit = 1 << key;
            if seen & bit != 0 {
                return Err("duplicate raw choice assignment".into());
            }
            seen |= bit;
        }
        if collected.failed {
            return Err("nonfailing source failed".into());
        }
        if collected.exhausted
            && (seen != u16::MAX || collected.answers.len() != fixtures::RAW_ANSWERS)
        {
            return Err("source exhausted with incorrect full raw answer multiset".into());
        }
        let validation_ns = validation.elapsed().as_nanos();
        let answers = collected.answers.len();
        let (_, engine_drop) = measure(|| drop(engine));
        let (_, outputs_drop) = measure(|| drop(collected.answers));
        let cutoff = !collected.exhausted;
        samples.push(Sample {
            pre: config.pre + i % 2,
            post: config.post + i % 2,
            setup,
            execution_observation,
            first_answer_ns: collected.first_answer_ns,
            validation_ns,
            engine_drop,
            outputs_drop,
            ticks: collected.ticks,
            answers,
            exhausted: collected.exhausted,
            cutoff,
            work: collected.work,
        });
        if cutoff {
            break;
        }
    }
    let (_, prepared_drop) = measure(|| drop(prepared));
    let (_, final_reading) = measure(|| ());
    std::hint::black_box(&source);
    std::hint::black_box(&queries);
    std::hint::black_box(&samples);
    Ok(Report {
        config,
        harness_setup_ns,
        baseline,
        prepare,
        source_clone_ns,
        samples,
        prepared_drop,
        final_reading,
    })
}
fn optional(value: Option<u128>) -> String {
    value.map_or_else(|| "null".into(), |v| v.to_string())
}
impl Report {
    fn json(&self) -> String {
        let samples = self.samples.iter().map(|s| format!(
            "{{\"pre\":{},\"post\":{},\"setup\":{},\"execution_observation\":{},\"first_answer_ns\":{},\"first_answer_request_ns\":{},\"validation_ns\":{},\"engine_drop\":{},\"outputs_drop\":{},\"ticks\":{},\"answers\":{},\"exhausted\":{},\"cutoff\":{},\"work\":{}}}",
            s.pre,s.post,s.setup.json(),s.execution_observation.json(),optional(s.first_answer_ns),optional(s.first_answer_ns.map(|ns| s.setup.ns + ns)),s.validation_ns,s.engine_drop.json(),s.outputs_drop.json(),s.ticks,s.answers,s.exhausted,s.cutoff,s.work.json()
        )).collect::<Vec<_>>().join(",");
        let measured_ns = self.prepare.ns
            + self.prepared_drop.ns
            + self
                .samples
                .iter()
                .map(|s| {
                    s.setup.ns + s.execution_observation.ns + s.engine_drop.ns + s.outputs_drop.ns
                })
                .sum::<u128>();
        format!(
            concat!(
                "{{\"schema_version\":1,\"backend\":\"{}\",\"pre\":{},\"post\":{},\"queries\":{},\"metrics\":{},\"compiled_metrics\":{},\"observer_metrics\":{},\"allocator_meter\":{},",
                "\"tick_limit\":{},\"completed\":{},\"complete\":{},\"harness_setup_ns\":{},\"baseline\":{},\"prepare\":{},\"source_clone_ns\":{},\"samples\":[{}],\"prepared_drop\":{},\"final\":{},\"measured_ns\":{},\"validation_ns\":{}}}"
            ),
            self.config.backend.name(),
            self.config.pre,
            self.config.post,
            self.config.queries,
            cfg!(feature = "metrics"),
            chr_compiled::COLLECT_METRICS,
            chr_observe::COLLECT_METRICS,
            cfg!(feature = "alloc-meter"),
            TICK_LIMIT,
            self.samples.iter().filter(|s| s.exhausted).count(),
            self.samples.len() == self.config.queries && self.samples.iter().all(|s| s.exhausted),
            self.harness_setup_ns,
            self.baseline.json(),
            self.prepare.json(),
            self.source_clone_ns,
            samples,
            self.prepared_drop.json(),
            self.final_reading.json(),
            measured_ns,
            self.samples.iter().map(|s| s.validation_ns).sum::<u128>()
        )
    }
}
fn main() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.get(1).map(String::as_str) == Some("meter-check") {
        #[cfg(feature = "alloc-meter")]
        {
            meter::self_check()?;
            println!("{{\"meter_check\":true}}");
            return Ok(());
        }
        #[cfg(not(feature = "alloc-meter"))]
        return Err("meter-check requires alloc-meter".into());
    }
    if args.len() != 5 {
        return Err("expected: conditional|specialized PRE POST QUERIES".into());
    }
    let backend = match args[1].as_str() {
        "conditional" => Backend::Conditional,
        "specialized" => Backend::Specialized,
        _ => return Err("unknown backend".into()),
    };
    let parse = |i: usize| {
        args[i]
            .parse::<usize>()
            .map_err(|_| "invalid integer".to_owned())
    };
    let report = run(Config {
        backend,
        pre: parse(2)?,
        post: parse(3)?,
        queries: parse(4)?,
    })?;
    println!("{}", report.json());
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn observer_diagnostics_follow_measurement_configuration() {
        assert_eq!(chr_observe::COLLECT_METRICS, cfg!(feature = "metrics"));
    }
    #[test]
    fn reusable_preparation_validates_complete_queries_and_lifecycle_records() {
        for backend in [Backend::Conditional, Backend::Specialized] {
            let report = run(Config {
                backend,
                pre: 0,
                post: 0,
                queries: 2,
            })
            .unwrap();
            assert_eq!(report.samples.len(), 2);
            for (i, s) in report.samples.iter().enumerate() {
                assert_eq!((s.pre, s.post), (i, i));
                assert!(s.exhausted && !s.cutoff);
                assert_eq!(s.answers, 16);
                assert!(s.ticks > 0 && s.first_answer_ns.is_some());
                if !cfg!(feature = "metrics") {
                    assert_eq!(s.work.applications, 0);
                }
            }
            assert!(report.json().contains("\"complete\":true"));
        }
    }
}
