//! Complete equation lifecycle; failure leaves are semantic counts in every build.
use chr_syntax::{Answer, Query};
use std::time::Instant;
#[allow(dead_code)]
#[path = "../tests/equation_support/mod.rs"]
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
    conditional_equality_jobs: Option<u64>,
    conditional_equality_ticks: Option<u64>,
    pairs: Option<u64>,
    occurs_visits: Option<u64>,
    invalidations: u64,
    selected_candidates: u64,
    deferred_candidates: u64,
}
impl Work {
    fn add(&mut self, stats: &chr_compiled::Stats) {
        if cfg!(feature = "metrics") {
            self.applications += stats.applications;
            self.candidates += stats.candidate_visits + stats.specialized_candidates;
            self.specialized_applications += stats.specialized_applications;
            self.cursor_steps += stats.cursor_steps;
            self.history_checks += stats.history_checks;
            *self.pairs.get_or_insert(0) += stats.kernel.pairs;
            *self.occurs_visits.get_or_insert(0) += stats.kernel.occurs_visits;
        }
    }
    fn json(&self) -> String {
        if cfg!(feature = "metrics") {
            format!(
                "{{\"applications\":{},\"choices\":{},\"candidates\":{},\"specialized_applications\":{},\"cursor_steps\":{},\"history_checks\":{},\"conditional_equality_jobs\":{},\"conditional_equality_ticks\":{},\"pairs\":{},\"occurs_visits\":{},\"invalidations\":{},\"selected_candidates\":{},\"deferred_candidates\":{}}}",
                self.applications,
                self.choices,
                self.candidates,
                self.specialized_applications,
                self.cursor_steps,
                self.history_checks,
                optional(self.conditional_equality_jobs.map(u128::from)),
                optional(self.conditional_equality_ticks.map(u128::from)),
                optional(self.pairs.map(u128::from)),
                optional(self.occurs_visits.map(u128::from)),
                self.invalidations,
                self.selected_candidates,
                self.deferred_candidates
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
    depth: usize,
    queries: usize,
    placement: fixtures::Placement,
    clash: bool,
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
    failure_leaves: Option<usize>,
    splits: Option<usize>,
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
            failure_leaves: if matches!(self, Self::Specialized(_)) {
                Some(0)
            } else {
                None
            },
            splits: if matches!(self, Self::Specialized(_)) {
                Some(0)
            } else {
                None
            },
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
                        if result.first_answer_ns.is_none() {
                            result.first_answer_ns = Some(start.elapsed().as_nanos());
                        }
                        result.work.add(b.engine.stats());
                        drop(b);
                        Some(answer)
                    }
                    chr_compiled::SearchEvent::Split { work, .. } => {
                        *result.splits.as_mut().unwrap() += 1;
                        if let Some(work) = work {
                            result.work.add(&work);
                            result.work.choices += 1;
                        }
                        None
                    }
                    chr_compiled::SearchEvent::Failed(b) => {
                        result.work.add(b.engine.stats());
                        *result.failure_leaves.as_mut().unwrap() += 1;
                        drop(b);
                        None
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
                if result.answers.len() > 16 {
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
            #[cfg(feature = "metrics")]
            {
                result.work.conditional_equality_jobs = Some(e.stats().equality_jobs);
                result.work.conditional_equality_ticks = Some(e.stats().equality_ticks);
            }
            result.work.invalidations = e.stats().invalidations;
            result.work.selected_candidates = e.stats().selected_candidates;
            result.work.deferred_candidates = e.stats().deferred_candidates;
        }
        result
    }
}
struct Sample {
    depth: usize,
    query: Measurement,
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
    failure_leaves: Option<usize>,
    splits: Option<usize>,
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
    if cfg!(feature = "alloc-meter") && cfg!(feature = "metrics") {
        return Err("allocation and work diagnostics must be separate".into());
    }
    if cfg!(feature = "carrier-contraction") {
        return Err("equation runner rejects carrier-contraction".into());
    }
    if matches!(config.backend, Backend::Conditional) && cfg!(feature = "arena-cow") {
        return Err("conditional must use isolated feature build".into());
    }
    if cfg!(feature = "metrics") != chr_compiled::COLLECT_METRICS
        || cfg!(feature = "metrics") != chr_compiled::COLLECT_KERNEL_METRICS
    {
        return Err("compiled and conditional metrics differ".into());
    }
    if config.queries == 0 || config.queries > 1000 || config.depth > 4096 {
        return Err("queries must be 1..1000 and work depths at most 4096".into());
    }
    if cfg!(feature = "metrics") != chr_observe::COLLECT_METRICS {
        return Err("observer and conditional metrics differ".into());
    }
    let harness = Instant::now();
    let source = fixtures::rules(config.placement);
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
            Backend::Specialized => {
                let prepared = chr_compiled::PreparedRuleset::new(rules, None)
                    .map(|p| p.specialize_inferred());
                prepared.map(Prepared::Specialized)
            }
        };
        (prepared, source_clone_ns)
    });
    let prepared = prepared?;
    for i in 0..config.queries {
        let depth = config.depth + i % 2;
        let (query, query_phase) = measure(|| fixtures::query(depth, config.clash));
        let (engine, setup) = measure(|| prepared.start(query));
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
        if config.clash && !collected.answers.is_empty() {
            return Err("clashing source published answer".into());
        }
        if collected.exhausted {
            let expected = if config.clash { 0 } else { 16 };
            if collected.answers.len() != expected
                || seen != if config.clash { 0 } else { u16::MAX }
            {
                return Err("source exhausted with incorrect answer multiset".into());
            }
            if let Some(failed) = collected.failure_leaves
                && (failed != if config.clash { 16 } else { 0 } || collected.splits != Some(15))
            {
                return Err("explicit source lineage counts violated".into());
            }
        }
        let validation_ns = validation.elapsed().as_nanos();
        let answers = collected.answers.len();
        let (_, engine_drop) = measure(|| drop(engine));
        let (_, outputs_drop) = measure(|| drop(collected.answers));
        let cutoff = !collected.exhausted;
        samples.push(Sample {
            depth,
            query: query_phase,
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
            failure_leaves: collected.failure_leaves,
            splits: collected.splits,
            work: collected.work,
        });
        if cutoff {
            break;
        }
    }
    let (_, prepared_drop) = measure(|| drop(prepared));
    let (_, final_reading) = measure(|| ());
    std::hint::black_box(&source);
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
            "{{\"depth\":{},\"query\":{},\"setup\":{},\"execution_observation\":{},\"first_answer_ns\":{},\"first_answer_request_ns\":{},\"validation_ns\":{},\"engine_drop\":{},\"outputs_drop\":{},\"ticks\":{},\"answers\":{},\"exhausted\":{},\"cutoff\":{},\"failure_leaves\":{},\"splits\":{},\"work\":{}}}",
            s.depth,s.query.json(),s.setup.json(),s.execution_observation.json(),optional(s.first_answer_ns),optional(s.first_answer_ns.map(|ns| s.query.ns + s.setup.ns + ns)),s.validation_ns,s.engine_drop.json(),s.outputs_drop.json(),s.ticks,s.answers,s.exhausted,s.cutoff,optional(s.failure_leaves.map(|x| x as u128)),optional(s.splits.map(|x| x as u128)),s.work.json()
        )).collect::<Vec<_>>().join(",");
        let measured_ns = self.prepare.ns
            + self.prepared_drop.ns
            + self
                .samples
                .iter()
                .map(|s| {
                    s.query.ns
                        + s.setup.ns
                        + s.execution_observation.ns
                        + s.engine_drop.ns
                        + s.outputs_drop.ns
                })
                .sum::<u128>();
        format!(
            concat!(
                "{{\"schema_version\":1,\"carrier_contraction\":{},\"arena_cow\":{},\"placement\":\"{}\",\"outcome\":\"{}\",\"backend\":\"{}\",\"depth\":{},\"queries\":{},\"metrics\":{},\"compiled_metrics\":{},\"kernel_metrics\":{},\"observer_metrics\":{},\"allocator_meter\":{},",
                "\"tick_limit\":{},\"completed\":{},\"complete\":{},\"harness_setup_ns\":{},\"baseline\":{},\"prepare\":{},\"source_clone_ns\":{},\"samples\":[{}],\"prepared_drop\":{},\"final\":{},\"measured_ns\":{},\"validation_ns\":{}}}"
            ),
            cfg!(feature = "carrier-contraction"),
            cfg!(feature = "arena-cow"),
            match self.config.placement {
                fixtures::Placement::BeforeGate => "before",
                fixtures::Placement::AfterGate => "after",
                fixtures::Placement::NaiveHoist => unreachable!(),
            },
            if self.config.clash {
                "clash"
            } else {
                "success"
            },
            self.config.backend.name(),
            self.config.depth,
            self.config.queries,
            cfg!(feature = "metrics"),
            chr_compiled::COLLECT_METRICS,
            chr_compiled::COLLECT_KERNEL_METRICS,
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
    if args.len() != 6 {
        return Err(
            "expected: conditional|specialized DEPTH QUERIES before|after success|clash".into(),
        );
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
        depth: parse(2)?,
        queries: parse(3)?,
        placement: match args[4].as_str() {
            "before" => fixtures::Placement::BeforeGate,
            "after" => fixtures::Placement::AfterGate,
            _ => return Err("invalid placement".into()),
        },
        clash: match args[5].as_str() {
            "success" => false,
            "clash" => true,
            _ => return Err("invalid outcome".into()),
        },
    })?;
    println!("{}", report.json());
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_lifecycles_success_clash_and_changed_queries() {
        if cfg!(feature = "carrier-contraction") {
            assert!(
                run(Config {
                    backend: Backend::Specialized,
                    depth: 0,
                    queries: 1,
                    placement: fixtures::Placement::BeforeGate,
                    clash: false
                })
                .is_err()
            );
            return;
        }
        for backend in [Backend::Conditional, Backend::Specialized] {
            if matches!(backend, Backend::Conditional) && cfg!(feature = "arena-cow") {
                continue;
            }
            for placement in [
                fixtures::Placement::BeforeGate,
                fixtures::Placement::AfterGate,
            ] {
                for clash in [false, true] {
                    let report = run(Config {
                        backend,
                        depth: 1,
                        queries: 2,
                        placement,
                        clash,
                    })
                    .unwrap();
                    assert_eq!(report.samples.len(), 2);
                    for (i, s) in report.samples.iter().enumerate() {
                        assert_eq!(s.depth, 1 + i);
                        if cfg!(feature = "metrics") && matches!(backend, Backend::Conditional) {
                            assert_eq!(
                                s.work.conditional_equality_jobs,
                                Some(if matches!(placement, fixtures::Placement::BeforeGate) {
                                    9
                                } else {
                                    24
                                })
                            );
                            assert!(s.work.conditional_equality_ticks.unwrap() > 0);
                        }
                        assert!(s.exhausted && !s.cutoff);
                        assert_eq!(s.answers, if clash { 0 } else { 16 });
                        assert_eq!(s.first_answer_ns.is_some(), !clash);
                        if matches!(backend, Backend::Specialized) {
                            assert_eq!(s.failure_leaves, Some(if clash { 16 } else { 0 }));
                            assert_eq!(s.splits, Some(15));
                        } else {
                            assert_eq!(s.failure_leaves, None);
                        }
                    }
                }
            }
        }
    }
}
