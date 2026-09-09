//! Shared E16 lifecycle harness. No hooks, per-step traces, or checks run in the
//! measured execution. Meter snapshots/reset occur only without live workers.
use chr_cases::Case;
use chr_reuse::{equation_search as scalar, parallel_equations as parallel};
use chr_syntax::Answer;
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

// Numeric snapshots are allocation-free and remain valid after engine drop.
const SOURCE_NAMES: [&str; 19] = [
    "source_steps",
    "applications",
    "introductions",
    "equations",
    "shared_pairs",
    "dereferences",
    "shared_occurs",
    "head_candidates",
    "splits",
    "source_failed",
    "source_completed",
    "source_duplicates",
    "source_max_frontier",
    "term_nodes",
    "term_requests",
    "pending_allocations",
    "map_visits",
    "map_allocations",
    "snapshot_copies",
];
const DRIVER_NAMES: [&str; 5] = [
    "driver_steps",
    "raw",
    "failed",
    "max_frontier",
    "scalar_projected_equations",
];
const BROKER_NAMES: [&str; 13] = [
    "lookahead_visits",
    "projection_dereferences",
    "issued",
    "received",
    "committed_equations",
    "max_outstanding",
    "owner_buffered_peak",
    "outstanding",
    "buffered",
    "uncommitted_at_shutdown",
    "broker_duplicates",
    "broker_completed",
    "broker_failed",
];
const OPERATION_NAMES: [&str; 9] = [
    "calls",
    "computed",
    "hits",
    "key_nodes",
    "replay_nodes",
    "owned_pairs",
    "owned_resolve",
    "owned_occurs",
    "cache_entries",
];

struct Snapshot {
    source: [u64; 19],
    driver: [u64; 5],
    broker: [u64; 13],
    operations: [u64; 9],
}

fn source(s: &chr_persistent::Stats) -> [u64; 19] {
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

fn operations(s: &chr_reuse::Stats) -> [u64; 9] {
    [
        s.calls,
        s.computed,
        s.hits,
        s.key_nodes,
        s.replay_nodes,
        s.pairs,
        s.resolve_nodes,
        s.occurs_nodes,
        s.cache_entries as u64,
    ]
}

trait Engine {
    fn advance(&mut self) -> Result<(Vec<Answer>, bool), String>;
    fn shutdown(&mut self) -> Result<(), String>;
    fn snapshot(&self) -> Snapshot;
}

impl Engine for scalar::Search {
    fn advance(&mut self) -> Result<(Vec<Answer>, bool), String> {
        let batch = self.advance(1);
        Ok((batch.answers, batch.exhausted))
    }
    fn shutdown(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn snapshot(&self) -> Snapshot {
        let s = self.stats();
        Snapshot {
            source: source(self.source_stats()),
            driver: [
                s.steps,
                s.completed,
                s.failed,
                s.max_frontier as u64,
                s.projected_equations,
            ],
            broker: [0; 13],
            operations: operations(self.operation_stats()),
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
        let s = self.stats();
        Snapshot {
            source: source(self.source_stats()),
            driver: [s.steps, s.completed, s.failed, s.max_frontier as u64, 0],
            broker: [
                s.lookahead_visits,
                s.projection_dereferences,
                s.issued,
                s.received,
                s.committed_equations,
                s.max_outstanding as u64,
                s.max_buffered as u64,
                s.outstanding as u64,
                s.buffered as u64,
                s.uncommitted_at_shutdown as u64,
                s.duplicates,
                s.completed,
                s.failed,
            ],
            operations: operations(self.operation_stats()),
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

const TIME_NAMES: [&str; 9] = [
    "construct_ns",
    "first_answer_ns",
    "search_ns",
    "stop_ns",
    "shutdown_ns",
    "joined_ns",
    "search_drop_ns",
    "search_dropped_ns",
    "output_drop_ns",
];
struct ResultRow {
    times: [u64; 9],
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

fn validate(case: &Case, answers: &[Answer], exhausted: bool, raw: u64) -> Result<(), String> {
    if exhausted != case.exhausted
        || raw != case.raw_answers
        || answers.len() != case.expected.len()
    {
        return Err(format!(
            "{} observation mismatch: exhausted={exhausted}, raw={raw}, answers={}",
            case.id,
            answers.len()
        ));
    }
    let equivalent =
        |a: &Answer, b: &Answer| chr_observe::equivalent(a, b, &mut Default::default());
    if !case
        .expected
        .iter()
        .all(|e| answers.iter().any(|a| equivalent(a, e)))
        || !answers
            .iter()
            .all(|a| case.expected.iter().any(|e| equivalent(a, e)))
    {
        return Err(format!("{} full output/residual answer mismatch", case.id));
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
    // The factory clones query/rules here, inside construction measurement.
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
    // This is the first meter read after starting workers. Reading/resetting at
    // construction return, first answer, or prefix stop would not be quiescent.
    let after = M::read();
    let after_shutdown = engine.snapshot();
    let drop_before = M::start();
    let search_drop_start = Instant::now();
    drop(engine);
    let search_dropped = Instant::now();
    let drop_after = M::read();

    // The external semantic check is outside execution and search release.
    // Its temporary allocations finish before the separate output-drop phase.
    if !stopped {
        return Err(format!(
            "{} exhausted source budget without a registered stop",
            case.id
        ));
    }
    validate(case, &answers, exhausted, at_stop.driver[1])?;
    if at_stop.source != after_shutdown.source || at_stop.driver != after_shutdown.driver {
        return Err("shutdown changed committed source work".into());
    }
    let answer_count = answers.len();
    let output_before = M::start();
    let output_start = Instant::now();
    drop(answers);
    let output_end = Instant::now();
    let output_after = M::read();
    Ok(ResultRow {
        times: [
            nanos(t0, t1),
            first.map_or(0, |t| nanos(t0, t)),
            nanos(t1, stop),
            nanos(t0, stop),
            nanos(shutdown_start, joined),
            nanos(t0, joined),
            nanos(search_drop_start, search_dropped),
            nanos(t0, search_dropped),
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

fn print_row<M: Metering>(case: &str, mode: &str, outstanding: usize, row: ResultRow) {
    print!(
        "case\tmode\toutstanding_limit\tlookahead\tmetered\tpass\texhausted\tanswers\thas_first"
    );
    for name in TIME_NAMES {
        print!("\t{name}");
    }
    for prefix in ["stop", "joined"] {
        for name in SOURCE_NAMES
            .into_iter()
            .chain(DRIVER_NAMES)
            .chain(BROKER_NAMES)
            .chain(OPERATION_NAMES)
        {
            print!("\t{prefix}_{name}");
        }
    }
    for prefix in ["through_join", "search_drop", "output_drop"] {
        for name in MEMORY_NAMES {
            print!("\t{prefix}_{name}");
        }
    }
    println!();
    print!(
        "{case}\t{mode}\t{outstanding}\t8\t{}\ttrue\t{}\t{}\t{}",
        M::ENABLED,
        row.exhausted,
        row.answers,
        row.has_first
    );
    for value in row.times {
        print!("\t{value}");
    }
    for snapshot in [row.at_stop, row.after_shutdown] {
        for value in snapshot
            .source
            .into_iter()
            .chain(snapshot.driver)
            .chain(snapshot.broker)
            .chain(snapshot.operations)
        {
            print!("\t{value}");
        }
    }
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
        for id in crate::parallel_cases::ids() {
            println!("{id}");
        }
        return Ok(());
    }
    if args.len() != 3 {
        return Err("usage: <case> <Shared|Owned|Inline|Threads1|Threads2> <outstanding>".into());
    }
    let outstanding = args[2]
        .parse::<usize>()
        .map_err(|_| "invalid outstanding limit")?;
    let case = crate::parallel_cases::case(&args[0]);
    let row = match args[1].as_str() {
        "Shared" | "Owned" => {
            let mode = if args[1] == "Shared" {
                scalar::Mode::Shared
            } else {
                scalar::Mode::Owned
            };
            session::<M, _>(&case, || {
                scalar::Search::new(case.rules.clone(), case.query.clone(), mode)
            })?
        }
        "Inline" | "Threads1" | "Threads2" => {
            let mode = match args[1].as_str() {
                "Inline" => parallel::Mode::Inline,
                "Threads1" => parallel::Mode::Threads(1),
                _ => parallel::Mode::Threads(2),
            };
            session::<M, _>(&case, || {
                parallel::Search::new(case.rules.clone(), case.query.clone(), mode, outstanding, 8)
            })?
        }
        _ => return Err("unrecognized mode".into()),
    };
    print_row::<M>(&case.id, &args[1], outstanding, row);
    Ok(())
}

pub fn main<M: Metering>() {
    if let Err(error) = run::<M>() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
