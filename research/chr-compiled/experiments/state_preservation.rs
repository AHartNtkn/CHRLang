//! Primary whole-service clock; separate inclusive event-class allocation diagnostics.
#[path = "../tests/state_preservation_support/mod.rs"]
mod fixture;
#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent, Stats};
use std::time::Instant;
const BUDGET: usize = 20_000_000;
const LABELS: [&str; 9] = [
    "split",
    "progress",
    "failed",
    "complete",
    "exhausted",
    "observation",
    "answer_retention",
    "terminal_disposal",
    "split_metadata_disposal",
];
#[derive(Clone, Copy, Default)]
struct Memory {
    start: usize,
    end: usize,
    peak: usize,
    calls: usize,
    bytes: usize,
    frees: usize,
    delta: i128,
}
impl Memory {
    #[cfg(feature = "alloc-meter")]
    fn from(r: meter::Reading) -> Self {
        Self {
            start: r.live_start,
            end: r.live_end,
            peak: r.peak_live,
            calls: r.allocation_calls,
            bytes: r.requested_bytes,
            frees: r.deallocation_calls,
            delta: r.live_end as i128 - r.live_start as i128,
        }
    }
    fn append(&mut self, other: Self, first: bool) {
        if first {
            self.start = other.start;
        }
        self.end = other.end;
        self.peak = self.peak.max(other.peak);
        self.calls += other.calls;
        self.bytes += other.bytes;
        self.frees += other.frees;
        self.delta += other.delta;
    }
    fn json(self) -> String {
        format!(
            "{{\"live_start\":{},\"live_end\":{},\"peak_live\":{},\"allocation_calls\":{},\"requested_bytes\":{},\"deallocation_calls\":{},\"net_live_delta\":{}}}",
            self.start, self.end, self.peak, self.calls, self.bytes, self.frees, self.delta
        )
    }
}
#[derive(Clone, Copy, Default)]
struct Interval {
    ns: u128,
    memory: Option<Memory>,
}
impl Interval {
    fn json(self) -> String {
        format!(
            "{{\"ns\":{},\"memory\":{}}}",
            self.ns,
            self.memory.map_or_else(|| "null".into(), Memory::json)
        )
    }
}
fn measure<T>(f: impl FnOnce() -> T) -> (T, Interval) {
    #[cfg(feature = "alloc-meter")]
    let start = meter::begin();
    let clock = Instant::now();
    let result = f();
    let ns = clock.elapsed().as_nanos();
    #[cfg(feature = "alloc-meter")]
    let memory = Some(Memory::from(meter::end(start)));
    #[cfg(not(feature = "alloc-meter"))]
    let memory = None;
    (result, Interval { ns, memory })
}
#[derive(Clone, Copy, Default)]
struct Category {
    count: usize,
    interval: Interval,
}
impl Category {
    fn add(&mut self, i: Interval) {
        self.interval.ns += i.ns;
        if let Some(m) = i.memory {
            self.interval
                .memory
                .get_or_insert_with(Memory::default)
                .append(m, self.count == 0);
        }
        self.count += 1;
    }
    // These visits are discontiguous; start/end are intentionally not attributed to this class.
    fn json(self) -> String {
        let memory=self.interval.memory.map_or_else(||"null".into(),|m|format!("{{\"peak_live\":{},\"allocation_calls\":{},\"requested_bytes\":{},\"deallocation_calls\":{},\"net_live_delta\":{}}}",m.peak,m.calls,m.bytes,m.frees,m.delta));
        format!(
            "{{\"count\":{},\"ns\":{},\"memory\":{}}}",
            self.count, self.interval.ns, memory
        )
    }
}
#[derive(Default)]
struct Work {
    applications: u64,
    source_steps: u64,
    candidates: u64,
    cursor_steps: u64,
    dependency_visits: u64,
    key_visits: u64,
    history_checks: u64,
    pairs: u64,
    occurs_visits: u64,
    term_requests: u64,
}
impl Work {
    fn add(&mut self, s: &Stats) {
        self.applications += s.applications;
        self.source_steps += s.source_steps;
        self.candidates += s.candidate_visits;
        self.cursor_steps += s.cursor_steps;
        self.dependency_visits += s.dependency_visits;
        self.key_visits += s.key_visits;
        self.history_checks += s.history_checks;
        self.pairs += s.kernel.pairs;
        self.occurs_visits += s.kernel.occurs_visits;
        self.term_requests += s.kernel.term_requests;
    }
    fn json(&self) -> String {
        format!(
            "{{\"applications\":{},\"source_steps\":{},\"candidate_visits\":{},\"cursor_steps\":{},\"dependency_visits\":{},\"key_visits\":{},\"history_checks\":{},\"pairs\":{},\"occurs_visits\":{},\"term_requests\":{}}}",
            self.applications,
            self.source_steps,
            self.candidates,
            self.cursor_steps,
            self.dependency_visits,
            self.key_visits,
            self.history_checks,
            self.pairs,
            self.occurs_visits,
            self.term_requests
        )
    }
}
#[derive(Default)]
struct Sample {
    n: usize,
    predicted_applications: usize,
    first_observation_ns: Option<u128>,
    maximum_frontier: usize,
    query: Interval,
    setup: Interval,
    service: Interval,
    engine_drop: Interval,
    validation: Interval,
    output_drop: Interval,
    ticks: usize,
    splits: usize,
    failed: usize,
    answers: usize,
    exhausted: bool,
    categories: [Category; 9],
    work: Work,
}
impl Sample {
    fn total_ns(&self) -> u128 {
        self.query.ns + self.setup.ns + self.service.ns + self.engine_drop.ns + self.output_drop.ns
    }
    fn record(&mut self, label: usize, i: Interval) {
        let first = self.categories.iter().all(|c| c.count == 0);
        self.categories[label].add(i);
        if let Some(m) = i.memory {
            self.service
                .memory
                .get_or_insert_with(Memory::default)
                .append(m, first);
        }
    }
    fn json(&self) -> String {
        let diagnostic = if cfg!(feature = "alloc-meter") {
            format!(
                "{{{}}}",
                LABELS
                    .iter()
                    .zip(&self.categories)
                    .map(|(name, c)| format!("{name:?}:{}", c.json()))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        } else {
            "null".into()
        };
        format!(
            "{{\"n\":{},\"predicted_applications\":{},\"first_observation_ns\":{},\"maximum_frontier\":{},\"total_ns\":{},\"query\":{},\"setup\":{},\"service\":{},\"engine_drop\":{},\"validation\":{},\"output_drop\":{},\"ticks\":{},\"splits\":{},\"failed\":{},\"answers\":{},\"exhausted\":{},\"diagnostic\":{},\"work\":{}}}",
            self.n,
            self.predicted_applications,
            self.first_observation_ns
                .map_or_else(|| "null".into(), |n| n.to_string()),
            if chr_compiled::COLLECT_METRICS || cfg!(feature = "alloc-meter") {
                self.maximum_frontier.to_string()
            } else {
                "null".into()
            },
            self.total_ns(),
            self.query.json(),
            self.setup.json(),
            self.service.json(),
            self.engine_drop.json(),
            self.validation.json(),
            self.output_drop.json(),
            self.ticks,
            self.splits,
            self.failed,
            self.answers,
            self.exhausted,
            diagnostic,
            if chr_compiled::COLLECT_METRICS {
                self.work.json()
            } else {
                "null".into()
            }
        )
    }
}
struct Report {
    n: usize,
    a: usize,
    all_success: bool,
    queries: usize,
    attempted: usize,
    budget: usize,
    baseline: Option<usize>,
    final_live: Option<usize>,
    prepare: Interval,
    prepared_drop: Interval,
    samples: Vec<Sample>,
}
impl Report {
    fn json(&self) -> String {
        format!(
            "{{\"schema_version\":1,\"n\":{},\"alternatives\":{},\"all_success\":{},\"queries\":{},\"attempted\":{},\"budget_per_query\":{},\"metrics\":{},\"kernel_metrics\":{},\"observer_metrics\":{},\"metered\":{},\"baseline_live\":{},\"final_live\":{},\"prepare\":{},\"prepared_drop\":{},\"samples\":[{}]}}",
            self.n,
            self.a,
            self.all_success,
            self.queries,
            self.attempted,
            self.budget,
            chr_compiled::COLLECT_METRICS,
            chr_persistent::COLLECT_KERNEL_METRICS,
            chr_observe::COLLECT_METRICS,
            cfg!(feature = "alloc-meter"),
            self.baseline
                .map_or_else(|| "null".into(), |x| x.to_string()),
            self.final_live
                .map_or_else(|| "null".into(), |x| x.to_string()),
            self.prepare.json(),
            self.prepared_drop.json(),
            self.samples
                .iter()
                .take(self.attempted)
                .map(Sample::json)
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}
fn run(
    n: usize,
    a: usize,
    all_success: bool,
    queries: usize,
    budget: usize,
) -> Result<Report, String> {
    if a == 0 || queries == 0 || n > 4096 || a > 4096 || queries > 64 {
        return Err("requires N<=4096, 1<=A<=4096, 1<=QUERIES<=64".into());
    }
    if chr_compiled::COLLECT_METRICS != chr_persistent::COLLECT_KERNEL_METRICS
        || chr_compiled::COLLECT_METRICS != chr_observe::COLLECT_METRICS
    {
        return Err("metric feature mismatch".into());
    }
    if cfg!(feature = "alloc-meter") && chr_compiled::COLLECT_METRICS {
        return Err("allocation and work diagnostics require separate configurations".into());
    }
    let rules = fixture::rules();
    let samples = std::iter::repeat_with(Sample::default)
        .take(queries)
        .collect();
    #[cfg(feature = "alloc-meter")]
    let baseline = Some(meter::end(meter::begin()).live_end);
    #[cfg(not(feature = "alloc-meter"))]
    let baseline = None;
    let (prepared, prepare) =
        measure(|| PreparedRuleset::new(rules.clone(), None).map(|p| p.specialize_inferred()));
    let prepared = prepared?;
    let mut report = Report {
        n,
        a,
        all_success,
        queries,
        attempted: 0,
        budget,
        baseline,
        final_live: None,
        prepare,
        prepared_drop: Interval::default(),
        samples,
    };
    for index in 0..queries {
        let s = &mut report.samples[index];
        s.n = n + index % 2;
        s.predicted_applications = s.n + 1 + a + (s.n + 1) * fixture::expected_raw(a, all_success);
        let (query, phase) = measure(|| fixture::query(s.n, a, all_success));
        s.query = phase;
        let (engine, phase) =
            measure(|| prepared.start_search(query, Policy::Global, Access::Indexed));
        let mut engine = engine?;
        s.setup = phase;
        let mut answers = Vec::new();
        let service_clock = Instant::now();
        if chr_compiled::COLLECT_METRICS || cfg!(feature = "alloc-meter") {
            s.maximum_frontier = engine.pending_branches();
        }
        for _ in 0..budget {
            let event = if cfg!(feature = "alloc-meter") {
                let (event, interval) = measure(|| engine.tick());
                let kind = match &event {
                    SearchEvent::Split { .. } => 0,
                    SearchEvent::Progress => 1,
                    SearchEvent::Failed(_) => 2,
                    SearchEvent::Complete(_) => 3,
                    SearchEvent::Exhausted => 4,
                };
                s.record(kind, interval);
                event
            } else {
                engine.tick()
            };
            s.ticks += 1;
            if chr_compiled::COLLECT_METRICS || cfg!(feature = "alloc-meter") {
                s.maximum_frontier = s.maximum_frontier.max(engine.pending_branches());
            }
            match event {
                SearchEvent::Split { lineage, work } => {
                    s.splits += 1;
                    if chr_compiled::COLLECT_METRICS
                        && let Some(work) = &work
                    {
                        s.work.add(work)
                    }
                    if cfg!(feature = "alloc-meter") {
                        let (_, i) = measure(|| drop((lineage, work)));
                        s.record(8, i);
                    } else {
                        drop((lineage, work));
                    }
                }
                SearchEvent::Failed(branch) => {
                    s.failed += 1;
                    if chr_compiled::COLLECT_METRICS {
                        s.work.add(branch.engine.stats());
                    }
                    if cfg!(feature = "alloc-meter") {
                        let (_, i) = measure(|| drop(branch));
                        s.record(7, i);
                    } else {
                        drop(branch);
                    }
                }
                SearchEvent::Complete(mut branch) => {
                    let answer = if cfg!(feature = "alloc-meter") {
                        let (a, i) = measure(|| branch.engine.observe());
                        s.record(5, i);
                        a
                    } else {
                        branch.engine.observe()
                    }
                    .ok_or("completed branch has no answer")?;
                    if s.first_observation_ns.is_none() {
                        s.first_observation_ns = Some(service_clock.elapsed().as_nanos());
                    }
                    if chr_compiled::COLLECT_METRICS {
                        s.work.add(branch.engine.stats());
                    }
                    if cfg!(feature = "alloc-meter") {
                        let (_, i) = measure(|| answers.push(answer));
                        s.record(6, i);
                        let (_, i) = measure(|| drop(branch));
                        s.record(7, i);
                    } else {
                        answers.push(answer);
                        drop(branch);
                    }
                }
                SearchEvent::Exhausted => {
                    s.exhausted = true;
                    break;
                }
                SearchEvent::Progress => (),
            }
        }
        s.service.ns = service_clock.elapsed().as_nanos();
        s.answers = answers.len();
        let (_, phase) = measure(|| drop(engine));
        s.engine_drop = phase;
        let start = Instant::now();
        let valid = answers.iter().all(fixture::answer_matches);
        s.validation.ns = start.elapsed().as_nanos();
        if !valid {
            return Err("complete answer mismatch".into());
        }
        if s.exhausted
            && (s.answers != fixture::expected_raw(a, all_success)
                || s.splits != a - 1
                || s.failed != if all_success { 0 } else { a - 1 })
        {
            return Err("terminal branch counts mismatch".into());
        }
        if s.exhausted
            && chr_compiled::COLLECT_METRICS
            && s.work.applications != s.predicted_applications as u64
        {
            return Err("source application prediction mismatch".into());
        }
        let (_, phase) = measure(|| drop(answers));
        s.output_drop = phase;
        report.attempted += 1;
        if !s.exhausted {
            break;
        }
    }
    let (_, phase) = measure(|| drop(prepared));
    report.prepared_drop = phase;
    #[cfg(feature = "alloc-meter")]
    {
        report.final_live = Some(meter::end(meter::begin()).live_end);
        if report.final_live != report.baseline {
            return Err("requested heap not restored".into());
        }
    }
    std::hint::black_box(&rules);
    Ok(report)
}
fn main() {
    if let Err(error) = entry() {
        eprintln!("{error}");
        std::process::exit(1)
    }
}
fn entry() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["meter-check"] {
        #[cfg(feature = "alloc-meter")]
        {
            meter::self_check()?;
            println!(
                "{{\"meter_check\":true,\"reading\":{}}}",
                meter::end(meter::begin()).json()
            );
            return Ok(());
        }
        #[cfg(not(feature = "alloc-meter"))]
        return Err("requires alloc-meter".into());
    }
    if args.len() != 4 {
        return Err("usage: N A all-success|mostly-fail QUERIES".into());
    }
    let n = args[0].parse().map_err(|_| "invalid N")?;
    let a = args[1].parse().map_err(|_| "invalid A")?;
    let all_success = match args[2].as_str() {
        "all-success" => true,
        "mostly-fail" => false,
        _ => return Err("invalid outcome mode".into()),
    };
    let queries = args[3].parse().map_err(|_| "invalid queries")?;
    let report = run(n, a, all_success, queries, BUDGET)?;
    println!("{}", report.json());
    Ok(())
}
#[cfg(all(test, not(feature = "alloc-meter")))]
mod tests {
    use super::*;
    #[test]
    fn small_complete_sessions_and_cutoff() {
        for all in [false, true] {
            let r = run(2, 3, all, 2, 100_000).unwrap();
            assert_eq!(r.attempted, 2);
            assert!(
                r.samples
                    .iter()
                    .all(|s| s.exhausted && s.answers == if all { 3 } else { 1 })
            );
        }
        let r = run(0, 1, false, 1, 100_000).unwrap();
        assert_eq!(r.samples[0].splits, 0);
        let r = run(2, 3, false, 2, 0).unwrap();
        assert_eq!(r.attempted, 1);
        assert!(!r.samples[0].exhausted);
        assert_eq!(r.samples[0].answers, 0);
    }
}
