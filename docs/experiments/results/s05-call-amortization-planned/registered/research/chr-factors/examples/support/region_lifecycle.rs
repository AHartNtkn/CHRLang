//! Contiguous regional construction/service/shutdown/disposal, followed by
//! outside-interval validation and separately measured output disposal.
use chr_cases::Case;
use chr_factors::parallel_regions as parallel;
use chr_syntax::{Answer, Term};
use std::time::Instant;
#[path = "region_cost_cases.rs"]
pub mod cases;
#[derive(Clone, Copy, Default)]
pub struct Reading {
    pub calls: u64,
    pub requested: u64,
    pub live: u64,
    pub peak: u64,
}
pub trait Metering {
    const ENABLED: bool;
    fn start() -> Reading;
    fn read() -> Reading;
}

fn equivalent(a: &Answer, b: &Answer) -> bool {
    chr_observe::equivalent(a, b, &mut Default::default())
}
fn natural(mut term: &Term) -> bool {
    loop {
        match term {
            Term::App(name, args) if name == "z" && args.is_empty() => return true,
            Term::App(name, args) if name == "s" && args.len() == 1 => term = &args[0],
            _ => return false,
        }
    }
}
fn validate_complete(
    case: &Case,
    answers: &[Answer],
    exhausted: bool,
    raw: Option<u128>,
) -> Result<(), String> {
    if case.id == "stream-prefix" {
        if exhausted || raw.is_some() || answers.len() != 8 || case.query.outputs.len() != 2 {
            return Err(
                "stream prefix requires eight answers without exhaustion or a complete raw count"
                    .into(),
            );
        }
        for (index, answer) in answers.iter().enumerate() {
            if !answer.residual.is_empty()
                || answer.outputs.len() != 2
                || !answer
                    .outputs
                    .iter()
                    .zip(&case.query.outputs)
                    .all(|((name, _), (expected, _))| name == expected)
                || !natural(&answer.outputs[0].1)
                || !matches!(&answer.outputs[1].1, Term::App(name, args) if (name == "a" || name == "b") && args.is_empty())
                || answers[..index]
                    .iter()
                    .any(|previous| equivalent(previous, answer))
            {
                return Err("stream prefix contains a duplicate or unsound output/residual".into());
            }
        }
        return Ok(());
    }
    if exhausted != case.exhausted
        || raw != Some(case.raw_answers as u128)
        || answers.len() != case.expected.len()
        || !case
            .expected
            .iter()
            .all(|expected| answers.iter().any(|answer| equivalent(answer, expected)))
        || !answers.iter().all(|answer| {
            case.expected
                .iter()
                .any(|expected| equivalent(answer, expected))
        })
    {
        return Err(format!(
            "{} full output/residual, raw multiplicity, or exhaustion mismatch",
            case.id
        ));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Inline,
    Threads1,
    Threads2,
    Specialized,
}
impl Mode {
    fn name(self) -> &'static str {
        match self {
            Self::Inline => "Inline",
            Self::Threads1 => "Threads1",
            Self::Threads2 => "Threads2",
            Self::Specialized => "Specialized",
        }
    }
}
#[derive(Default)]
struct Work {
    applications: u64,
    actual_applications: u64,
    accepted_source_steps: u64,
    actual_source_steps: u64,
    products: u64,
    source_turns: u64,
    candidates: u64,
    specialized_applications: u64,
    cursor_steps: u64,
    history_checks: u64,
    transport: parallel::TransportStats,
}
impl Work {
    fn add(&mut self, stats: &chr_compiled::Stats) {
        if chr_factors::COLLECT_METRICS {
            self.applications += stats.applications;
            self.actual_applications += stats.applications;
            self.accepted_source_steps += stats.source_steps;
            self.actual_source_steps += stats.source_steps;
            self.candidates += stats.candidate_visits + stats.specialized_candidates;
            self.specialized_applications += stats.specialized_applications;
            self.cursor_steps += stats.cursor_steps;
            self.history_checks += stats.history_checks;
        }
    }
    fn json(&self) -> String {
        if !chr_factors::COLLECT_METRICS {
            return "null".into();
        }
        let t = &self.transport;
        format!(
            concat!(
                "{{\"applications\":{},\"actual_applications\":{},\"accepted_source_steps\":{},\"actual_source_steps\":{},\"products\":{},\"source_turns\":{},\"candidates\":{},\"specialized_applications\":{},\"cursor_steps\":{},\"history_checks\":{},",
                "\"transport\":{{\"issued\":{},\"received\":{},\"accepted\":{},\"max_outstanding\":{},\"owner_buffered_peak\":{},\"outstanding\":{},\"buffered\":{},\"unaccepted_at_shutdown\":{},\"actual_source_steps\":{},\"accepted_source_steps\":{},\"cancelled_requests\":{},\"unserved_quantum_slots\":{},\"prefetch_visits\":{}}}}}"
            ),
            self.applications,
            self.actual_applications,
            self.accepted_source_steps,
            self.actual_source_steps,
            self.products,
            self.source_turns,
            self.candidates,
            self.specialized_applications,
            self.cursor_steps,
            self.history_checks,
            t.issued,
            t.received,
            t.accepted,
            t.max_outstanding,
            t.owner_buffered_peak,
            t.outstanding,
            t.buffered,
            t.unaccepted_at_shutdown,
            t.actual_source_steps,
            t.accepted_source_steps,
            t.cancelled_requests,
            t.unserved_quantum_slots,
            t.prefetch_visits
        )
    }
}
#[allow(clippy::large_enum_variant)]
enum Engine {
    Regional(parallel::Search),
    Specialized {
        search: chr_compiled::SearchEngine,
        _prepared: chr_compiled::PreparedRuleset,
        seen: chr_observe::AnswerSet,
        raw: u128,
        exhausted: bool,
        work: Work,
    },
}
impl Engine {
    fn new(case: &Case, mode: Mode, quantum: usize, limit: usize) -> Result<Self, String> {
        if mode == Mode::Specialized {
            if quantum != 0
                || limit != 0
                || matches!(case.id.as_str(), "stream-prefix" | "refute-loop")
            {
                return Err("Specialized requires a finite case and Q0/K0".into());
            }
            let prepared =
                chr_compiled::PreparedRuleset::new(case.rules.clone(), None)?.specialize_inferred();
            let search = prepared.start_search(
                case.query.clone(),
                chr_compiled::Policy::Global,
                chr_compiled::Access::Indexed,
            )?;
            return Ok(Self::Specialized {
                search,
                _prepared: prepared,
                seen: Default::default(),
                raw: 0,
                exhausted: false,
                work: Work::default(),
            });
        }
        if ![1, 8, 64].contains(&quantum) || limit != 4 {
            return Err("regional modes require Q1/8/64 and K4".into());
        }
        let mode = match mode {
            Mode::Inline => parallel::Mode::Inline,
            Mode::Threads1 => parallel::Mode::Threads(1),
            Mode::Threads2 => parallel::Mode::Threads(2),
            Mode::Specialized => unreachable!(),
        };
        parallel::Search::new(case.rules.clone(), case.query.clone(), mode, quantum, limit)
            .map(Self::Regional)
    }
    fn advance(&mut self, answers: &mut Vec<Answer>) -> Result<bool, String> {
        match self {
            Self::Regional(search) => {
                let batch = search.advance(1)?;
                answers.extend(batch.answers);
                Ok(batch.exhausted)
            }
            Self::Specialized {
                search,
                seen,
                raw,
                exhausted,
                work,
                ..
            } => {
                match search.tick() {
                    chr_compiled::SearchEvent::Complete(mut branch) => {
                        *raw += 1;
                        let answer = branch.engine.observe().expect("complete branch answer");
                        work.add(branch.engine.stats());
                        if seen.insert(answer.clone()) {
                            answers.push(answer);
                        }
                    }
                    chr_compiled::SearchEvent::Split {
                        work: Some(segment),
                        ..
                    } => work.add(&segment),
                    chr_compiled::SearchEvent::Failed(branch) => work.add(branch.engine.stats()),
                    chr_compiled::SearchEvent::Exhausted => *exhausted = true,
                    chr_compiled::SearchEvent::Split { work: None, .. }
                    | chr_compiled::SearchEvent::Progress => (),
                }
                Ok(*exhausted)
            }
        }
    }
    fn shutdown(&mut self) -> Result<(), String> {
        match self {
            Self::Regional(s) => s.shutdown(),
            _ => Ok(()),
        }
    }
    fn raw(&self) -> Option<u128> {
        match self {
            Self::Regional(s) => s.raw_count(),
            Self::Specialized { raw, exhausted, .. } => exhausted.then_some(*raw),
        }
    }
    // Called only in work builds and only after every worker has joined.
    fn work(&self) -> Work {
        match self {
            Self::Regional(s) => Work {
                applications: s.source_applications(),
                actual_applications: s.actual_source_stats().iter().map(|s| s.applications).sum(),
                accepted_source_steps: s.source_stats().map(|s| s.steps).sum(),
                actual_source_steps: s.actual_source_stats().iter().map(|s| s.steps).sum(),
                products: s.stats().products,
                source_turns: s.stats().source_steps,
                candidates: s
                    .actual_source_stats()
                    .iter()
                    .map(|s| s.head_candidates)
                    .sum(),
                transport: *s.transport_stats(),
                ..Work::default()
            },
            Self::Specialized { work, .. } => Work {
                applications: work.applications,
                actual_applications: work.actual_applications,
                accepted_source_steps: work.accepted_source_steps,
                actual_source_steps: work.actual_source_steps,
                candidates: work.candidates,
                specialized_applications: work.specialized_applications,
                cursor_steps: work.cursor_steps,
                history_checks: work.history_checks,
                ..Work::default()
            },
        }
    }
}
struct Interval {
    before: Reading,
    after: Reading,
}
impl Interval {
    fn json(&self, enabled: bool) -> String {
        if !enabled {
            return "null".into();
        }
        format!(
            "{{\"allocation_calls\":{},\"requested_bytes\":{},\"baseline_live\":{},\"peak_live\":{},\"final_live\":{}}}",
            self.after.calls - self.before.calls,
            self.after.requested - self.before.requested,
            self.before.live,
            self.after.peak,
            self.after.live
        )
    }
}
pub struct Row {
    mode: Mode,
    quantum: usize,
    limit: usize,
    construct_ns: u128,
    first_answer_ns: Option<u128>,
    search_ns: u128,
    shutdown_ns: u128,
    drop_ns: u128,
    contiguous_ns: u128,
    validation_ns: u128,
    output_drop_ns: u128,
    exhausted: bool,
    raw: Option<u128>,
    answers: usize,
    service_calls: usize,
    budget: usize,
    stopped: bool,
    lifecycle: Interval,
    output_drop: Interval,
    work: Work,
}
impl Row {
    pub fn json<M: Metering>(&self, case: &str) -> String {
        let option = |v: Option<u128>| v.map_or_else(|| "null".into(), |v| v.to_string());
        format!(
            concat!(
                "{{\"schema_version\":1,\"case\":\"{}\",\"mode\":\"{}\",\"quantum\":{},\"limit\":{},\"metrics\":{},\"persistent_metrics\":{},\"kernel_metrics\":{},\"observer_metrics\":{},\"compiled_metrics\":{},\"metered\":{},\"status\":\"{}\",\"exhausted\":{},\"raw\":{},\"answers\":{},\"service_calls\":{},\"budget\":{},",
                "\"construct_ns\":{},\"first_answer_ns\":{},\"search_ns\":{},\"shutdown_ns\":{},\"drop_ns\":{},\"contiguous_ns\":{},\"validation_ns\":{},\"output_drop_ns\":{},\"total_ns\":{},\"retained_bytes\":{},\"memory\":{{\"lifecycle\":{},\"output_drop\":{}}},\"work\":{}}}"
            ),
            case,
            self.mode.name(),
            self.quantum,
            self.limit,
            chr_factors::COLLECT_METRICS,
            chr_persistent::COLLECT_METRICS,
            chr_persistent::COLLECT_KERNEL_METRICS,
            chr_observe::COLLECT_METRICS,
            chr_compiled::COLLECT_METRICS,
            M::ENABLED,
            if !self.stopped {
                "cutoff"
            } else if self.exhausted {
                "complete"
            } else {
                "prefix"
            },
            self.exhausted,
            option(self.raw),
            self.answers,
            self.service_calls,
            self.budget,
            self.construct_ns,
            option(self.first_answer_ns),
            self.search_ns,
            self.shutdown_ns,
            self.drop_ns,
            self.contiguous_ns,
            self.validation_ns,
            self.output_drop_ns,
            self.contiguous_ns + self.output_drop_ns,
            if M::ENABLED {
                (i128::from(self.output_drop.after.live) - i128::from(self.lifecycle.before.live))
                    .to_string()
            } else {
                "null".into()
            },
            self.lifecycle.json(M::ENABLED),
            self.output_drop.json(M::ENABLED),
            self.work.json()
        )
    }
}
fn validate_partial(case: &Case, answers: &[Answer]) -> Result<(), String> {
    for (i, answer) in answers.iter().enumerate() {
        if answers[..i].iter().any(|a| equivalent(a, answer)) {
            return Err("duplicate partial observation".into());
        }
        if case.id == "stream-prefix" {
            if !answer.residual.is_empty()
                || answer.outputs.len() != 2
                || !answer
                    .outputs
                    .iter()
                    .zip(&case.query.outputs)
                    .all(|((n, _), (e, _))| n == e)
                || !natural(&answer.outputs[0].1)
                || !matches!(&answer.outputs[1].1,Term::App(n,a) if (n=="a"||n=="b")&&a.is_empty())
            {
                return Err("unsound partial stream answer".into());
            }
        } else if !case.expected.iter().any(|a| equivalent(a, answer)) {
            return Err("unsound partial finite answer".into());
        }
    }
    Ok(())
}
pub fn session<M: Metering>(
    case: &Case,
    mode: Mode,
    quantum: usize,
    limit: usize,
) -> Result<Row, String> {
    let flags = [
        chr_persistent::COLLECT_METRICS,
        chr_persistent::COLLECT_KERNEL_METRICS,
        chr_observe::COLLECT_METRICS,
        chr_compiled::COLLECT_METRICS,
    ];
    if flags
        .iter()
        .any(|&flag| flag != chr_factors::COLLECT_METRICS)
    {
        return Err("resolved diagnostic features differ".into());
    }
    let mut answers = Vec::new();
    let before = M::start();
    let start = Instant::now();
    let mut engine = Engine::new(case, mode, quantum, limit)?;
    let constructed = Instant::now();
    let mut first = None;
    let mut exhausted = false;
    let mut stopped = false;
    let mut service_calls = 0;
    let budget = if mode == Mode::Specialized {
        20_000_000
    } else {
        case.budget
    };
    for _ in 0..budget {
        service_calls += 1;
        exhausted = engine.advance(&mut answers)?;
        if first.is_none() && !answers.is_empty() {
            first = Some(start.elapsed().as_nanos());
        }
        if exhausted || case.answer_limit.is_some_and(|n| answers.len() >= n) {
            stopped = true;
            break;
        }
    }
    let stop = Instant::now();
    engine.shutdown()?;
    let joined = Instant::now();
    let raw = engine.raw();
    let work = if chr_factors::COLLECT_METRICS {
        engine.work()
    } else {
        Work::default()
    };
    let drop_start = Instant::now();
    drop(engine);
    let dropped = Instant::now();
    let after = M::read();
    let validation = Instant::now();
    if stopped {
        validate_complete(case, &answers, exhausted, raw)?;
    } else {
        validate_partial(case, &answers)?;
    }
    let validation_ns = validation.elapsed().as_nanos();
    let count = answers.len();
    let output_before = M::start();
    let output_start = Instant::now();
    drop(answers);
    let output_drop_ns = output_start.elapsed().as_nanos();
    let output_after = M::read();
    // Validation temporaries are dead; fixtures remain borrowed across both reads.
    // Cold blocking-channel infrastructure may remain owned by the receiving
    // thread after every query/worker/output owner is dropped. Report the signed
    // difference, including unexpected values; attribution belongs to the audit.

    Ok(Row {
        mode,
        quantum,
        limit,
        construct_ns: constructed.duration_since(start).as_nanos(),
        first_answer_ns: first,
        search_ns: stop.duration_since(constructed).as_nanos(),
        shutdown_ns: joined.duration_since(stop).as_nanos(),
        drop_ns: dropped.duration_since(drop_start).as_nanos(),
        contiguous_ns: dropped.duration_since(start).as_nanos(),
        validation_ns,
        output_drop_ns,
        exhausted,
        raw,
        answers: count,
        service_calls,
        budget,
        stopped,
        lifecycle: Interval { before, after },
        output_drop: Interval {
            before: output_before,
            after: output_after,
        },
        work,
    })
}
pub fn main<M: Metering>() {
    if let Err(error) = run::<M>() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
fn run<M: Metering>() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--list"] {
        for id in cases::ids() {
            println!("{id}");
        }
        return Ok(());
    }
    if args.len() != 4 {
        return Err("expected CASE Inline|Threads1|Threads2|Specialized QUANTUM LIMIT".into());
    }
    if !cases::ids().contains(&args[0]) {
        return Err("unknown fixture".into());
    }
    let case = cases::case(&args[0]);
    let mode = match args[1].as_str() {
        "Inline" => Mode::Inline,
        "Threads1" => Mode::Threads1,
        "Threads2" => Mode::Threads2,
        "Specialized" => Mode::Specialized,
        _ => return Err("unknown mode".into()),
    };
    let quantum = args[2].parse().map_err(|_| "invalid quantum")?;
    let limit = args[3].parse().map_err(|_| "invalid limit")?;
    let row = session::<M>(&case, mode, quantum, limit)?;
    println!("{}", row.json::<M>(&case.id));
    Ok(())
}
