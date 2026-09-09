//! Isolated-process streaming lifecycle runner. Comparative runs require registration.
use chr_syntax::Answer;
use std::time::Instant;
#[path = "../tests/stream_support/mod.rs"]
mod fixtures;
#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
use fixtures::{Extent, Family, fixture, valid_answer};
const TICK_LIMIT: usize = 20_000_000;
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
enum Consumer {
    Drop,
    Retain,
}
impl Consumer {
    fn name(self) -> &'static str {
        match self {
            Self::Drop => "drop",
            Self::Retain => "retain",
        }
    }
}
#[derive(Clone, Copy)]
struct Config {
    backend: Backend,
    family: Family,
    extent: Extent,
    n: usize,
    consumer: Consumer,
}
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
        format!("{{\"ns\":{},\"memory\":{memory}}}", self.ns)
    }
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
enum Delivery {
    Answer(Answer),
    Exhausted,
    Cutoff,
}
impl Prepared {
    fn start(&self, q: chr_syntax::Query) -> Result<Running, String> {
        match self {
            Self::Conditional(p) => p.start(q).map(Running::Conditional),
            Self::Specialized(p) => p
                .start_search(
                    q,
                    chr_compiled::Policy::Global,
                    chr_compiled::Access::Indexed,
                )
                .map(Running::Specialized),
        }
    }
}
#[derive(Clone, Copy)]
struct Diagnostics {
    applications: Option<u64>,
    choices: u64,
    raw_completions: u64,
    service_steps: u64,
    pending_branches: Option<usize>,
    occurrence_records: Option<usize>,
    history_records: Option<usize>,
    body_records: Option<usize>,
    equality_variables: Option<usize>,
    support_nodes: Option<usize>,
    support_variables: Option<usize>,
}
impl Running {
    fn service(&mut self, total: &mut usize) -> Delivery {
        while *total < TICK_LIMIT {
            *total += 1;
            match self {
                Self::Conditional(e) => match e.tick() {
                    chr_direct_conditional::engine::Event::Progress => (),
                    chr_direct_conditional::engine::Event::Answer(a) => return Delivery::Answer(a),
                    chr_direct_conditional::engine::Event::Exhausted => return Delivery::Exhausted,
                },
                Self::Specialized(e) => match e.tick() {
                    chr_compiled::SearchEvent::Complete(mut branch) => {
                        let answer = branch.engine.observe().expect("successful complete branch");
                        drop(branch);
                        return Delivery::Answer(answer);
                    }
                    chr_compiled::SearchEvent::Exhausted => return Delivery::Exhausted,
                    chr_compiled::SearchEvent::Progress
                    | chr_compiled::SearchEvent::Split { .. }
                    | chr_compiled::SearchEvent::Failed(_) => (),
                },
            }
        }
        Delivery::Cutoff
    }
    /// Scalar reads only, performed after service/consumer intervals. No branch
    /// snapshots, trace retention, walking live stores, or formatting is involved.
    fn diagnostics(&self) -> Option<Diagnostics> {
        if !cfg!(feature = "metrics") {
            return None;
        }
        Some(match self {
            Self::Conditional(e) => {
                let s = e.stats();
                Diagnostics {
                    applications: Some(s.applications),
                    choices: s.births,
                    raw_completions: s.answers,
                    service_steps: s.ticks,
                    pending_branches: None,
                    occurrence_records: Some(e.resources().occurrences().len()),
                    history_records: Some(e.resources().history().len()),
                    body_records: Some(e.resources().pending_bodies().len()),
                    equality_variables: Some(e.store().variable_count()),
                    support_nodes: Some(e.supports().node_count()),
                    support_variables: Some(e.supports().variable_count()),
                }
            }
            Self::Specialized(e) => {
                let s = e.stats();
                Diagnostics {
                    applications: None,
                    choices: s.forks,
                    raw_completions: s.raw_completions,
                    service_steps: s.service_steps,
                    pending_branches: Some(e.pending_branches()),
                    occurrence_records: None,
                    history_records: None,
                    body_records: None,
                    equality_variables: None,
                    support_nodes: None,
                    support_variables: None,
                }
            }
        })
    }
}
struct Sample {
    answer: usize,
    ticks: usize,
    service: Measurement,
    validation_ns: u128,
    consumer: Measurement,
    diagnostics: Option<Diagnostics>,
}
struct Terminal {
    ticks: usize,
    service: Measurement,
    cutoff: bool,
}
struct Report {
    config: Config,
    harness_setup_ns: u128,
    baseline: Measurement,
    prepare: Measurement,
    source_clone_ns: u128,
    setup: Measurement,
    samples: Vec<Sample>,
    terminal_service: Option<Terminal>,
    outputs_drop: Measurement,
    engine_drop: Measurement,
    prepared_drop: Measurement,
    final_reading: Measurement,
    total_ticks: usize,
    exhausted: bool,
    cutoff: bool,
    complete: bool,
    final_diagnostics: Option<Diagnostics>,
}
fn run(config: Config) -> Result<Report, String> {
    if config.n > TICK_LIMIT {
        return Err("target exceeds total service tick bound".into());
    }
    if cfg!(feature = "metrics") != chr_compiled::COLLECT_METRICS {
        return Err("conditional and compiled metrics features differ".into());
    }
    let harness = Instant::now();
    let fixtures = fixture(config.family, config.extent);
    // These allocations stay fixed and alive across the baseline and final reading.
    let mut samples = Vec::new();
    samples
        .try_reserve_exact(config.n)
        .map_err(|e| format!("record capacity: {e}"))?;
    let mut outputs: Vec<Answer> = Vec::new();
    let harness_setup_ns = harness.elapsed().as_nanos();
    let (_, baseline) = measure(|| ());
    let ((prepared, source_clone_ns), prepare) = measure(|| {
        // Timer only: nested meter resets would invalidate preparation's peak.
        let start = Instant::now();
        let rules = fixtures.rules.clone();
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
    let (engine, setup) = measure(|| prepared.start(fixtures.query.clone()));
    let mut engine = engine?;
    let mut total_ticks = 0;
    let mut exhausted = false;
    let mut cutoff = false;
    let mut terminal_service = None;
    loop {
        if samples.len() == config.n && matches!(config.extent, Extent::Unbounded) {
            break;
        }
        let start_ticks = total_ticks;
        let (delivery, service) = measure(|| engine.service(&mut total_ticks));
        let ticks = total_ticks - start_ticks;
        match delivery {
            Delivery::Answer(answer) => {
                let validation = Instant::now();
                let valid = valid_answer(fixtures.family, &answer);
                let validation_ns = validation.elapsed().as_nanos();
                if !valid {
                    drop(answer);
                    return Err("complete answer violates closed-form source contract".into());
                }
                if samples.len() == config.n {
                    drop(answer);
                    return Err("finite source delivered more than its exact raw count".into());
                }
                let (_, consumer) = measure(|| match config.consumer {
                    Consumer::Drop => drop(answer),
                    Consumer::Retain => outputs.push(answer),
                });
                let diagnostics = engine.diagnostics();
                samples.push(Sample {
                    answer: samples.len() + 1,
                    ticks,
                    service,
                    validation_ns,
                    consumer,
                    diagnostics,
                });
            }
            Delivery::Exhausted => {
                exhausted = true;
                terminal_service = Some(Terminal {
                    ticks,
                    service,
                    cutoff: false,
                });
                if fixtures.finite_answers != Some(samples.len()) {
                    return Err("source exhausted before required raw prefix/count".into());
                }
                break;
            }
            Delivery::Cutoff => {
                cutoff = true;
                terminal_service = Some(Terminal {
                    ticks,
                    service,
                    cutoff: true,
                });
                break;
            }
        }
    }
    let final_diagnostics = engine.diagnostics();
    let (_, outputs_drop) = measure(|| drop(outputs));
    let (_, engine_drop) = measure(|| drop(engine));
    let (_, prepared_drop) = measure(|| drop(prepared));
    let (_, final_reading) = measure(|| ());
    // Pin original fixtures and record storage through the final meter boundary.
    std::hint::black_box(&fixtures);
    std::hint::black_box(&samples);
    let complete = !cutoff
        && samples.len() == config.n
        && (matches!(config.extent, Extent::Unbounded) || exhausted);
    Ok(Report {
        config,
        harness_setup_ns,
        baseline,
        prepare,
        source_clone_ns,
        setup,
        samples,
        terminal_service,
        outputs_drop,
        engine_drop,
        prepared_drop,
        final_reading,
        total_ticks,
        exhausted,
        cutoff,
        complete,
        final_diagnostics,
    })
}
fn optional<T: std::fmt::Display>(value: Option<T>) -> String {
    value.map_or_else(|| "null".into(), |v| v.to_string())
}
impl Diagnostics {
    fn json(self) -> String {
        format!(
            "{{\"applications\":{},\"choices\":{},\"raw_completions\":{},\"service_steps\":{},\"pending_branches\":{},\"occurrence_records\":{},\"history_records\":{},\"body_records\":{},\"equality_variables\":{},\"support_nodes\":{},\"support_variables\":{}}}",
            optional(self.applications),
            self.choices,
            self.raw_completions,
            self.service_steps,
            optional(self.pending_branches),
            optional(self.occurrence_records),
            optional(self.history_records),
            optional(self.body_records),
            optional(self.equality_variables),
            optional(self.support_nodes),
            optional(self.support_variables)
        )
    }
}
fn diagnostics_json(d: Option<Diagnostics>) -> String {
    d.map_or_else(|| "null".into(), Diagnostics::json)
}
impl Report {
    fn json(&self) -> String {
        let samples=self.samples.iter().map(|s|format!("{{\"answer\":{},\"ticks\":{},\"service\":{},\"validation_ns\":{},\"consumer\":{},\"diagnostics\":{}}}",s.answer,s.ticks,s.service.json(),s.validation_ns,s.consumer.json(),diagnostics_json(s.diagnostics))).collect::<Vec<_>>().join(",");
        let terminal = self.terminal_service.as_ref().map_or_else(
            || "null".into(),
            |t| {
                format!(
                    "{{\"ticks\":{},\"service\":{},\"outcome\":\"{}\"}}",
                    t.ticks,
                    t.service.json(),
                    if t.cutoff { "cutoff" } else { "exhausted" }
                )
            },
        );
        let measured_ns = self.prepare.ns
            + self.setup.ns
            + self
                .samples
                .iter()
                .map(|s| s.service.ns + s.consumer.ns)
                .sum::<u128>()
            + self.terminal_service.as_ref().map_or(0, |t| t.service.ns)
            + self.outputs_drop.ns
            + self.engine_drop.ns
            + self.prepared_drop.ns;
        let first = self.samples.first().map(|s| s.service.ns);
        format!(
            concat!(
                "{{\"schema_version\":1,\"backend\":\"{}\",\"family\":\"{}\",\"extent\":\"{}\",\"n\":{},\"consumer\":\"{}\",",
                "\"metrics\":{},\"compiled_metrics\":{},\"allocator_meter\":{},\"tick_limit\":{},\"total_ticks\":{},\"delivered\":{},\"exhausted\":{},\"cutoff\":{},\"complete\":{},",
                "\"harness_setup_ns\":{},\"baseline\":{},\"prepare\":{},\"source_clone_ns\":{},\"setup\":{},\"first_answer_service_ns\":{},\"first_answer_request_ns\":{},\"measured_ns\":{},\"validation_ns\":{},",
                "\"samples\":[{}],\"terminal_service\":{},\"disposal\":{{\"outputs\":{},\"engine\":{},\"prepared\":{}}},\"final\":{},\"final_diagnostics\":{}}}"
            ),
            self.config.backend.name(),
            self.config.family.name(),
            if matches!(self.config.extent, Extent::Unbounded) {
                "unbounded"
            } else {
                "finite"
            },
            self.config.n,
            self.config.consumer.name(),
            cfg!(feature = "metrics"),
            chr_compiled::COLLECT_METRICS,
            cfg!(feature = "alloc-meter"),
            TICK_LIMIT,
            self.total_ticks,
            self.samples.len(),
            self.exhausted,
            self.cutoff,
            self.complete,
            self.harness_setup_ns,
            self.baseline.json(),
            self.prepare.json(),
            self.source_clone_ns,
            self.setup.json(),
            optional(first),
            optional(first.map(|ns| self.prepare.ns + self.setup.ns + ns)),
            measured_ns,
            self.samples.iter().map(|s| s.validation_ns).sum::<u128>(),
            samples,
            terminal,
            self.outputs_drop.json(),
            self.engine_drop.json(),
            self.prepared_drop.json(),
            self.final_reading.json(),
            diagnostics_json(self.final_diagnostics)
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
        return Err("expected: backend family extent n consumer".into());
    }
    let backend = match args[1].as_str() {
        "conditional" => Backend::Conditional,
        "specialized" => Backend::Specialized,
        _ => return Err("unknown backend".into()),
    };
    let family = Family::parse(&args[2]).ok_or("unknown family")?;
    let n = args[4].parse::<usize>().map_err(|_| "invalid n")?;
    let extent = match args[3].as_str() {
        "unbounded" => Extent::Unbounded,
        "finite" => Extent::Finite(n),
        _ => return Err("unknown extent".into()),
    };
    let consumer = match args[5].as_str() {
        "drop" => Consumer::Drop,
        "retain" => Consumer::Retain,
        _ => return Err("unknown consumer".into()),
    };
    let report = run(Config {
        backend,
        family,
        extent,
        n,
        consumer,
    })?;
    println!("{}", report.json());
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_streams_observe_validate_consume_and_dispose_all_backends() {
        for backend in [Backend::Conditional, Backend::Specialized] {
            for family in [Family::Ground, Family::Aliases] {
                for consumer in [Consumer::Drop, Consumer::Retain] {
                    for extent in [Extent::Unbounded, Extent::Finite(2)] {
                        let report = run(Config {
                            backend,
                            family,
                            extent,
                            n: 2,
                            consumer,
                        })
                        .unwrap();
                        assert!(report.complete);
                        assert!(!report.cutoff);
                        assert_eq!(report.samples.len(), 2);
                        assert_eq!(report.exhausted, matches!(extent, Extent::Finite(_)));
                        assert_eq!(
                            report.total_ticks,
                            report.samples.iter().map(|s| s.ticks).sum::<usize>()
                                + report.terminal_service.as_ref().map_or(0, |t| t.ticks)
                        );
                        assert!(report.samples.iter().all(|s| s.ticks > 0));
                        assert_eq!(
                            report.terminal_service.is_some(),
                            matches!(extent, Extent::Finite(_))
                        );
                        assert_eq!(
                            report.samples.iter().all(|s| s.diagnostics.is_some()),
                            cfg!(feature = "metrics")
                        );
                        let json = report.json();
                        assert!(json.starts_with('{') && json.ends_with('}'));
                    }
                }
            }
        }
    }
    #[test]
    fn tick_limit_reports_incomplete_without_fabricating_an_answer() {
        let f = fixture(Family::Ground, Extent::Unbounded);
        let p = Prepared::Conditional(
            chr_direct_conditional::engine::PreparedRuleset::new(f.rules).unwrap(),
        );
        let mut e = p.start(f.query).unwrap();
        let mut ticks = TICK_LIMIT;
        assert!(matches!(e.service(&mut ticks), Delivery::Cutoff));
        assert_eq!(ticks, TICK_LIMIT);
    }
}
