//! E16 regional lifecycle harness. Fixed numeric snapshots use only nonallocating
//! iterators/slices; no hooks, source traces, or diagnostic vectors are retained.
//! Primary cold wall time includes diagnostic gaps: workers can progress there.
//! The phase sum excluding those gaps is descriptive, not parallel elapsed time.
use chr_cases::Case;
use chr_factors::{Search as Baseline, parallel_regions as parallel};
use chr_syntax::{Answer, Term};
use std::time::Instant;

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

const OWNER_NAMES: [&str; 11] = [
    "certificate_predicates",
    "certificate_edges",
    "certificate_terms",
    "owner_steps",
    "owner_source_turns",
    "product_jobs_created",
    "products",
    "duplicates",
    "renamed_nodes",
    "peak_product_jobs",
    "empty_refutations",
];
const SOURCE_NAMES: [&str; 19] = [
    "steps",
    "applications",
    "introductions",
    "equations",
    "pairs",
    "dereferences",
    "occurs_visits",
    "head_candidates",
    "splits",
    "failed",
    "completed",
    "duplicates",
    "sum_region_peak_frontier",
    "term_nodes",
    "term_requests",
    "pending_allocations",
    "map_visits",
    "map_allocations",
    "snapshot_copies",
];
const OBSERVATION_NAMES: [&str; 4] = [
    "term_pairs",
    "occurrence_scans",
    "occurrence_candidates",
    "backtracks",
];
const TRANSPORT_NAMES: [&str; 13] = [
    "issued",
    "received",
    "accepted",
    "max_outstanding",
    "owner_buffered_peak",
    "outstanding",
    "buffered",
    "unaccepted_at_shutdown",
    "actual_source_steps",
    "accepted_source_steps",
    "cancelled_requests",
    "unserved_quantum_slots",
    "prefetch_visits",
];
const STATE_NAMES: [&str; 3] = ["factor_count", "cached_answers", "pending_jobs"];

struct Snapshot {
    owner: [u64; 11],
    state: [u64; 3],
    raw: Option<u128>,
    accepted: [u64; 19],
    actual: [u64; 19],
    accepted_observation: [u64; 4],
    actual_observation: [u64; 4],
    global_observation: [u64; 4],
    transport: [u64; 13],
}

fn owner(s: &chr_factors::Stats) -> [u64; 11] {
    [
        s.certificate_predicates as u64,
        s.certificate_edges,
        s.certificate_terms,
        s.steps,
        s.source_steps,
        s.product_jobs,
        s.products,
        s.duplicates,
        s.renamed_nodes,
        s.max_jobs as u64,
        s.empty_refutations,
    ]
}
fn baseline_source(s: &chr_persistent::Stats) -> [u64; 19] {
    [
        s.steps,
        s.applications,
        s.introductions,
        s.equations,
        s.pairs,
        s.dereferences,
        s.occurs_visits,
        s.head_candidates,
        s.splits,
        s.failed,
        s.completed,
        s.duplicates,
        s.max_frontier as u64,
        s.term_nodes as u64,
        s.term_requests,
        s.pending_allocations,
        s.storage.visits,
        s.storage.allocations,
        s.storage.snapshot_copies,
    ]
}
fn regional_source(s: &parallel::SourceStats) -> [u64; 19] {
    [
        s.steps,
        s.applications,
        s.introductions,
        s.equations,
        s.pairs,
        s.dereferences,
        s.occurs_visits,
        s.head_candidates,
        s.splits,
        s.failed,
        s.completed,
        s.duplicates,
        s.max_frontier as u64,
        s.term_nodes as u64,
        s.term_requests,
        s.pending_allocations,
        s.storage_visits,
        s.storage_allocations,
        s.snapshot_copies,
    ]
}
fn observation(s: &chr_observe::Stats) -> [u64; 4] {
    [
        s.term_pairs,
        s.occurrence_scans,
        s.occurrence_candidates,
        s.backtracks,
    ]
}
fn regional_observation(s: &parallel::ObservationStats) -> [u64; 4] {
    [
        s.term_pairs,
        s.occurrence_scans,
        s.occurrence_candidates,
        s.backtracks,
    ]
}
fn sum<const N: usize>(rows: impl Iterator<Item = [u64; N]>) -> [u64; N] {
    let mut total = [0; N];
    for row in rows {
        for (total, value) in total.iter_mut().zip(row) {
            *total += value;
        }
    }
    total
}

trait Engine {
    fn advance(&mut self) -> Result<(Vec<Answer>, bool), String>;
    fn shutdown(&mut self) -> Result<(), String>;
    fn snapshot(&self) -> Snapshot;
}
impl Engine for Baseline {
    fn advance(&mut self) -> Result<(Vec<Answer>, bool), String> {
        let batch = self.advance(1);
        Ok((batch.answers, batch.exhausted))
    }
    fn shutdown(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn snapshot(&self) -> Snapshot {
        let source = sum(self.source_stats().map(baseline_source));
        let regional = observation(&self.regional_observation_stats());
        Snapshot {
            owner: owner(self.stats()),
            state: [
                self.factor_count() as u64,
                self.cached_answers() as u64,
                self.pending_jobs() as u64,
            ],
            raw: self.raw_count(),
            accepted: source,
            actual: source,
            accepted_observation: regional,
            actual_observation: regional,
            global_observation: observation(self.observation_stats()),
            transport: [0; 13],
        }
    }
}
impl Engine for parallel::Search {
    fn advance(&mut self) -> Result<(Vec<Answer>, bool), String> {
        let batch = self.advance(1)?;
        Ok((batch.answers, batch.exhausted))
    }
    fn shutdown(&mut self) -> Result<(), String> {
        self.shutdown()
    }
    fn snapshot(&self) -> Snapshot {
        let t = self.transport_stats();
        Snapshot {
            owner: owner(self.stats()),
            state: [
                self.factor_count() as u64,
                self.cached_answers() as u64,
                self.pending_jobs() as u64,
            ],
            raw: self.raw_count(),
            accepted: sum(self.source_stats().map(regional_source)),
            actual: sum(self.actual_source_stats().iter().map(regional_source)),
            accepted_observation: regional_observation(&self.regional_observation_stats()),
            actual_observation: sum(self
                .actual_regional_observation_stats()
                .iter()
                .map(regional_observation)),
            global_observation: observation(self.observation_stats()),
            transport: [
                t.issued,
                t.received,
                t.accepted,
                t.max_outstanding as u64,
                t.owner_buffered_peak as u64,
                t.outstanding as u64,
                t.buffered as u64,
                t.unaccepted_at_shutdown as u64,
                t.actual_source_steps,
                t.accepted_source_steps,
                t.cancelled_requests,
                t.unserved_quantum_slots,
                t.prefetch_visits,
            ],
        }
    }
}

struct Interval {
    before: Reading,
    after: Reading,
}
const MEMORY_NAMES: [&str; 5] = [
    "allocation_calls",
    "requested_bytes",
    "baseline_live",
    "peak_live",
    "final_live",
];
impl Interval {
    fn values(&self) -> [u64; 5] {
        [
            self.after.calls - self.before.calls,
            self.after.requested - self.before.requested,
            self.before.live,
            self.after.peak,
            self.after.live,
        ]
    }
}
const TIME_NAMES: [&str; 11] = [
    "construct_ns",
    "first_answer_ns",
    "search_ns",
    "wall_stop_ns",
    "shutdown_ns",
    "wall_joined_ns",
    "search_drop_ns",
    "wall_through_drop_including_diagnostics_ns",
    "diagnostic_gap_ns",
    "operational_ns",
    "output_drop_ns",
];
struct ResultRow {
    times: [u64; 11],
    has_first: bool,
    exhausted: bool,
    answers: usize,
    at_stop: Snapshot,
    after_shutdown: Snapshot,
    through_join: Interval,
    search_drop: Interval,
    output_drop: Interval,
}
fn nanos(from: Instant, to: Instant) -> u64 {
    u64::try_from(to.duration_since(from).as_nanos()).expect("measurement duration fits u64")
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
fn validate(
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

fn session<M: Metering, E: Engine>(
    case: &Case,
    create: impl FnOnce() -> Result<E, String>,
) -> Result<ResultRow, String> {
    let mut answers = Vec::new();
    let before = M::start();
    let t0 = Instant::now();
    let mut engine = create()?;
    let t1 = Instant::now();
    let mut first = None;
    let mut exhausted = false;
    let mut stopped = false;
    for _ in 0..case.budget {
        let (produced, done) = engine.advance()?;
        answers.extend(produced);
        if first.is_none() && !answers.is_empty() {
            first = Some(Instant::now());
        }
        exhausted = done;
        if exhausted || case.answer_limit.is_some_and(|n| answers.len() >= n) {
            stopped = true;
            break;
        }
    }
    let stop = Instant::now();
    let at_stop = engine.snapshot();
    let shutdown_start = Instant::now();
    engine.shutdown()?;
    let joined = Instant::now();
    // First meter read after worker construction; shutdown has joined workers.
    let after = M::read();
    let after_shutdown = engine.snapshot();
    let drop_before = M::start();
    let drop_start = Instant::now();
    drop(engine);
    let dropped = Instant::now();
    let drop_after = M::read();

    if !stopped {
        return Err(format!(
            "{} reached owner-turn budget without a registered stop",
            case.id
        ));
    }
    validate(case, &answers, exhausted, at_stop.raw)?;
    if at_stop.owner != after_shutdown.owner
        || at_stop.raw != after_shutdown.raw
        || at_stop.accepted != after_shutdown.accepted
        || at_stop.accepted_observation != after_shutdown.accepted_observation
        || at_stop.global_observation != after_shutdown.global_observation
    {
        return Err("regional shutdown changed logical source/product observation".into());
    }
    let answer_count = answers.len();
    // Validation temporaries have gone out of scope. Output release is a
    // separate phase, not part of an uninterrupted wall-time execution claim.
    let output_before = M::start();
    let output_start = Instant::now();
    drop(answers);
    let output_end = Instant::now();
    let output_after = M::read();
    let construct = nanos(t0, t1);
    let search = nanos(t1, stop);
    let shutdown = nanos(shutdown_start, joined);
    let search_drop = nanos(drop_start, dropped);
    let operational = construct + search + shutdown + search_drop;
    let wall = nanos(t0, dropped);
    Ok(ResultRow {
        times: [
            construct,
            first.map_or(0, |instant| nanos(t0, instant)),
            search,
            nanos(t0, stop),
            shutdown,
            nanos(t0, joined),
            search_drop,
            wall,
            wall - operational,
            operational,
            nanos(output_start, output_end),
        ],
        has_first: first.is_some(),
        exhausted,
        answers: answer_count,
        at_stop,
        after_shutdown,
        through_join: Interval { before, after },
        search_drop: Interval {
            before: drop_before,
            after: drop_after,
        },
        output_drop: Interval {
            before: output_before,
            after: output_after,
        },
    })
}

fn snapshot_header(prefix: &str) {
    for name in OWNER_NAMES.into_iter().chain(STATE_NAMES) {
        print!("\t{prefix}_{name}");
    }
    print!("\t{prefix}_raw_available\t{prefix}_raw");
    for kind in ["accepted_source", "actual_source"] {
        for name in SOURCE_NAMES {
            print!("\t{prefix}_{kind}_{name}");
        }
    }
    for kind in [
        "accepted_regional_observer",
        "actual_regional_observer",
        "global_observer",
    ] {
        for name in OBSERVATION_NAMES {
            print!("\t{prefix}_{kind}_{name}");
        }
    }
    for name in TRANSPORT_NAMES {
        print!("\t{prefix}_transport_{name}");
    }
}
fn snapshot_values(snapshot: Snapshot) {
    for value in snapshot.owner.into_iter().chain(snapshot.state) {
        print!("\t{value}");
    }
    print!(
        "\t{}\t{}",
        snapshot.raw.is_some(),
        snapshot.raw.unwrap_or(0)
    );
    for value in snapshot
        .accepted
        .into_iter()
        .chain(snapshot.actual)
        .chain(snapshot.accepted_observation)
        .chain(snapshot.actual_observation)
        .chain(snapshot.global_observation)
        .chain(snapshot.transport)
    {
        print!("\t{value}");
    }
}
fn print_row<M: Metering>(case: &str, mode: &str, quantum: usize, limit: usize, row: ResultRow) {
    print!(
        "case\tmode\tquantum\tlimit\tmetered\ttransport_enabled\tpass\texhausted\tanswers\thas_first"
    );
    for name in TIME_NAMES {
        print!("\t{name}");
    }
    for prefix in ["stop", "joined"] {
        snapshot_header(prefix);
    }
    for prefix in ["through_join", "search_drop", "output_drop"] {
        for name in MEMORY_NAMES {
            print!("\t{prefix}_{name}");
        }
    }
    println!();
    print!(
        "{case}\t{mode}\t{quantum}\t{limit}\t{}\t{}\ttrue\t{}\t{}\t{}",
        M::ENABLED,
        mode != "Baseline",
        row.exhausted,
        row.answers,
        row.has_first
    );
    for value in row.times {
        print!("\t{value}");
    }
    snapshot_values(row.at_stop);
    snapshot_values(row.after_shutdown);
    for interval in [row.through_join, row.search_drop, row.output_drop] {
        for value in interval.values() {
            print!("\t{value}");
        }
    }
    println!();
}

fn run<M: Metering>() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--list"] {
        for id in crate::region_cost_cases::ids() {
            println!("{id}");
        }
        return Ok(());
    }
    if args.len() != 4 {
        return Err("usage: <case> <Baseline|Inline|Threads1|Threads2> <quantum> <limit>".into());
    }
    let quantum = args[2].parse::<usize>().map_err(|_| "invalid quantum")?;
    let limit = args[3]
        .parse::<usize>()
        .map_err(|_| "invalid reservation limit")?;
    let case = crate::region_cost_cases::case(&args[0]);
    let row = match args[1].as_str() {
        "Baseline" => {
            if quantum != 1 || limit != 0 {
                return Err("Baseline requires Q1/K0".into());
            }
            session::<M, _>(&case, || {
                Baseline::new(
                    case.rules.clone(),
                    case.query.clone(),
                    chr_factors::Mode::Factored,
                )
            })?
        }
        "Inline" | "Threads1" | "Threads2" => {
            if ![1, 8, 64].contains(&quantum) || ![1, 4].contains(&limit) {
                return Err("registered quantum/limit is Q1/8/64 with K1/4".into());
            }
            let mode = match args[1].as_str() {
                "Inline" => parallel::Mode::Inline,
                "Threads1" => parallel::Mode::Threads(1),
                _ => parallel::Mode::Threads(2),
            };
            session::<M, _>(&case, || {
                parallel::Search::new(case.rules.clone(), case.query.clone(), mode, quantum, limit)
            })?
        }
        _ => return Err("unrecognized regional mode".into()),
    };
    print_row::<M>(&case.id, &args[1], quantum, limit, row);
    Ok(())
}
pub fn main<M: Metering>() {
    if let Err(error) = run::<M>() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
