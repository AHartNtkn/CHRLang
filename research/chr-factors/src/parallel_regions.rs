//! Certified permanent regions with fixed worker ownership and deterministic
//! owner-side product admission. A quantum contains atomic source steps, not
//! bounded wall time. This is a separate experimental scheduling control.
use crate::{Batch, Job, Stats, partition, rename};
use chr_syntax::{Answer, Constraint, Query, Rule};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread::{self, JoinHandle};

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Inline,
    Threads(usize),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SourceStats {
    pub steps: u64,
    pub applications: u64,
    pub introductions: u64,
    pub equations: u64,
    pub pairs: u64,
    pub dereferences: u64,
    pub occurs_visits: u64,
    pub head_candidates: u64,
    pub splits: u64,
    pub failed: u64,
    pub completed: u64,
    pub duplicates: u64,
    pub max_frontier: usize,
    pub term_nodes: usize,
    pub term_requests: u64,
    pub pending_allocations: u64,
    pub storage_visits: u64,
    pub storage_allocations: u64,
    pub snapshot_copies: u64,
}

impl SourceStats {
    fn read(s: &chr_persistent::Stats) -> Self {
        Self {
            steps: s.steps,
            applications: s.applications,
            introductions: s.introductions,
            equations: s.equations,
            pairs: s.pairs,
            dereferences: s.dereferences,
            occurs_visits: s.occurs_visits,
            head_candidates: s.head_candidates,
            splits: s.splits,
            failed: s.failed,
            completed: s.completed,
            duplicates: s.duplicates,
            max_frontier: s.max_frontier,
            term_nodes: s.term_nodes,
            term_requests: s.term_requests,
            pending_allocations: s.pending_allocations,
            storage_visits: s.storage.visits,
            storage_allocations: s.storage.allocations,
            snapshot_copies: s.storage.snapshot_copies,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ObservationStats {
    pub term_pairs: u64,
    pub occurrence_scans: u64,
    pub occurrence_candidates: u64,
    pub backtracks: u64,
}
impl ObservationStats {
    fn read(s: &chr_observe::Stats) -> Self {
        Self {
            term_pairs: s.term_pairs,
            occurrence_scans: s.occurrence_scans,
            occurrence_candidates: s.occurrence_candidates,
            backtracks: s.backtracks,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportStats {
    pub issued: u64,
    pub received: u64,
    pub accepted: u64,
    pub max_outstanding: usize,
    pub owner_buffered_peak: usize,
    pub outstanding: usize,
    pub buffered: usize,
    pub unaccepted_at_shutdown: usize,
    pub actual_source_steps: u64,
    pub accepted_source_steps: u64,
    pub cancelled_requests: u64,
    /// Unused slots of a canceled quantum, not a prediction of future work.
    pub unserved_quantum_slots: u64,
    pub prefetch_visits: u64,
}

struct Request {
    id: u64,
    region: usize,
    quantum: usize,
}
struct Response {
    id: u64,
    region: usize,
    answers: Vec<Answer>,
    exhausted: bool,
    source: SourceStats,
    observation: ObservationStats,
    steps: u64,
    cancelled: bool,
    unserved_quantum_slots: u64,
}
struct Slot {
    id: u64,
    response: Option<Response>,
}
struct Reservations {
    slots: Vec<Option<Slot>>,
    limit: usize,
    next: u64,
    stats: TransportStats,
    actual_source: Vec<SourceStats>,
    actual_observation: Vec<ObservationStats>,
}
impl Reservations {
    fn new(regions: usize, limit: usize) -> Self {
        Self {
            slots: (0..regions).map(|_| None).collect(),
            limit,
            next: 0,
            stats: TransportStats::default(),
            actual_source: vec![SourceStats::default(); regions],
            actual_observation: vec![ObservationStats::default(); regions],
        }
    }
    fn issue(&mut self, region: usize, quantum: usize) -> Result<Request, String> {
        if region >= self.slots.len()
            || self.slots[region].is_some()
            || self.stats.outstanding == self.limit
            || quantum == 0
        {
            return Err("regional reservation is unavailable".into());
        }
        let id = self.next;
        self.next = id.checked_add(1).ok_or("regional request ID overflow")?;
        self.slots[region] = Some(Slot { id, response: None });
        self.stats.issued += 1;
        self.stats.outstanding += 1;
        self.stats.max_outstanding = self.stats.max_outstanding.max(self.stats.outstanding);
        Ok(Request {
            id,
            region,
            quantum,
        })
    }
    fn receive(&mut self, response: Response) -> Result<(), String> {
        let region = response.region;
        let slot = self
            .slots
            .get_mut(region)
            .and_then(Option::as_mut)
            .ok_or("unknown regional reply")?;
        if slot.id != response.id {
            return Err("regional reply ID mismatch".into());
        }
        if slot.response.is_some() {
            return Err("duplicate regional reply".into());
        }
        self.stats.received += 1;
        self.stats.buffered += 1;
        self.stats.owner_buffered_peak = self.stats.owner_buffered_peak.max(self.stats.buffered);
        self.stats.actual_source_steps += response.steps;
        self.stats.cancelled_requests += u64::from(response.cancelled);
        self.stats.unserved_quantum_slots += response.unserved_quantum_slots;
        self.actual_source[region] = response.source;
        self.actual_observation[region] = response.observation;
        slot.response = Some(response);
        Ok(())
    }
    fn ready(&self, region: usize) -> bool {
        self.slots[region]
            .as_ref()
            .is_some_and(|slot| slot.response.is_some())
    }
    fn take(&mut self, region: usize) -> Result<Response, String> {
        if !self.ready(region) {
            return Err("selected regional reply is not ready".into());
        }
        if self.slots[region]
            .as_ref()
            .unwrap()
            .response
            .as_ref()
            .unwrap()
            .cancelled
        {
            return Err("canceled regional reply cannot enter logical observation".into());
        }
        let response = self.slots[region].take().unwrap().response.unwrap();
        self.stats.accepted += 1;
        self.stats.accepted_source_steps += response.steps;
        self.stats.outstanding -= 1;
        self.stats.buffered -= 1;
        Ok(response)
    }
    fn release(&mut self) {
        self.stats.unaccepted_at_shutdown = self.stats.outstanding;
        for slot in &mut self.slots {
            *slot = None;
        }
        self.stats.outstanding = 0;
        self.stats.buffered = 0;
    }
}

fn service(search: &mut chr_persistent::Search, request: Request, cancel: &AtomicBool) -> Response {
    service_with_step_hook(search, request, cancel, |_| {})
}

// One loop for real workers, inline service, and deterministic cancellation
// tests. The production instantiation has a statically dispatched no-op hook.
fn service_with_step_hook(
    search: &mut chr_persistent::Search,
    request: Request,
    cancel: &AtomicBool,
    mut after_step: impl FnMut(usize),
) -> Response {
    let before = search.stats().steps;
    let mut answers = Vec::new();
    let mut exhausted = false;
    let mut cancelled = false;
    let mut unserved_quantum_slots = 0;
    for step in 0..request.quantum {
        if cancel.load(Ordering::Acquire) {
            cancelled = true;
            unserved_quantum_slots = (request.quantum - step) as u64;
            break;
        }
        let batch = search.advance(1);
        after_step(step + 1);
        answers.extend(batch.answers);
        exhausted = batch.exhausted;
        if exhausted {
            break;
        }
    }
    Response {
        id: request.id,
        region: request.region,
        answers,
        exhausted,
        source: SourceStats::read(search.stats()),
        observation: ObservationStats::read(search.observation_stats()),
        steps: search.stats().steps - before,
        cancelled,
        unserved_quantum_slots,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerPhase {
    BeforeService,
    AfterService,
}
/// Zero-based worker index, region index, request ID and boundary around the
/// actual regional service. Hooks cannot replace the solver and require Threads.
pub type WorkerHook = Arc<dyn Fn(usize, usize, u64, WorkerPhase) + Send + Sync>;

type Initialization = Vec<(usize, SourceStats, ObservationStats)>;
// Numeric reply snapshots stay by value in the bounded channel. Boxing solely
// to shrink the rare initialization/error variants adds allocation per service.
#[expect(
    clippy::large_enum_variant,
    reason = "bounded by-value service snapshots avoid per-reply boxing"
)]
enum Message {
    Ready(Initialization),
    Served(Response),
    Failed(String),
}
struct Pool {
    inputs: Vec<mpsc::SyncSender<Request>>,
    output: mpsc::Receiver<Message>,
    workers: Vec<JoinHandle<()>>,
    sent: u64,
    received: u64,
}
impl Pool {
    fn new(
        regions: Vec<(Vec<Rule>, Query)>,
        count: usize,
        capacity: usize,
        cancel: Arc<AtomicBool>,
        hook: Option<WorkerHook>,
    ) -> Result<(Self, Initialization), String> {
        if count == 0 {
            return Err("worker count must be positive".into());
        }
        let mut assignments = (0..count).map(|_| Vec::new()).collect::<Vec<_>>();
        for (region, input) in regions.into_iter().enumerate() {
            assignments[region % count].push((region, input));
        }
        // Each worker request channel can hold K requests; the global accepted
        // request count is <= K. Thus submission never needs result progress,
        // even if workers are blocked on the shared ONE-slot reply channel.
        let (output, receiver) = mpsc::sync_channel(1);
        let mut pool = Self {
            inputs: Vec::new(),
            output: receiver,
            workers: Vec::new(),
            sent: 0,
            received: 0,
        };
        for (worker, inputs) in assignments.into_iter().enumerate() {
            let (sender, requests) = mpsc::sync_channel::<Request>(capacity);
            let worker_output = output.clone();
            let cancel = cancel.clone();
            let hook = hook.clone();
            let spawned = thread::Builder::new()
                .name(format!("chr-region-{worker}"))
                .spawn(move || {
                    let outcome = catch_unwind(AssertUnwindSafe(|| -> Result<(), String> {
                        // No Rc crosses the thread boundary: construction and
                        // all future source mutation happen inside this closure.
                        let mut searches = BTreeMap::new();
                        let mut initialized = Vec::new();
                        for (region, (rules, query)) in inputs {
                            let search = chr_persistent::Search::new(
                                rules,
                                query,
                                chr_persistent::Snapshot::Persistent,
                            )?;
                            initialized.push((
                                region,
                                SourceStats::read(search.stats()),
                                ObservationStats::read(search.observation_stats()),
                            ));
                            searches.insert(region, search);
                        }
                        worker_output
                            .send(Message::Ready(initialized))
                            .map_err(|_| "regional result receiver closed")?;
                        while let Ok(request) = requests.recv() {
                            let region = request.region;
                            let id = request.id;
                            let search = searches
                                .get_mut(&region)
                                .ok_or("region assigned to another worker")?;
                            if let Some(h) = &hook {
                                h(worker, region, id, WorkerPhase::BeforeService);
                            }
                            let response = service(search, request, &cancel);
                            if let Some(h) = &hook {
                                h(worker, region, id, WorkerPhase::AfterService);
                            }
                            worker_output
                                .send(Message::Served(response))
                                .map_err(|_| "regional result receiver closed")?;
                        }
                        Ok(())
                    }));
                    let error = match outcome {
                        Ok(Ok(())) => None,
                        Ok(Err(error)) => Some(format!("regional worker {worker}: {error}")),
                        Err(_) => Some(format!("regional worker {worker} panicked")),
                    };
                    if let Some(error) = error {
                        let _ = worker_output.send(Message::Failed(error));
                    }
                });
            match spawned {
                Ok(handle) => {
                    pool.inputs.push(sender);
                    pool.workers.push(handle);
                }
                Err(error) => {
                    drop(sender);
                    drop(output);
                    let _ = pool.finish(|_| Ok(()));
                    return Err(format!("cannot create regional worker: {error}"));
                }
            }
        }
        drop(output);
        let mut initial = Vec::new();
        for _ in 0..count {
            match pool.output.recv() {
                Ok(Message::Ready(rows)) => initial.extend(rows),
                other => {
                    let error = match other {
                        Ok(Message::Failed(error)) => error,
                        _ => "regional initialization did not acknowledge every worker".into(),
                    };
                    let _ = pool.finish(|_| Ok(()));
                    return Err(error);
                }
            }
        }
        Ok((pool, initial))
    }
    fn submit(&mut self, request: Request) -> Result<(), String> {
        self.inputs[request.region % self.inputs.len()]
            .send(request)
            .map_err(|_| "assigned regional worker disconnected")?;
        self.sent += 1;
        Ok(())
    }
    fn receive(&mut self) -> Result<Response, String> {
        match self
            .output
            .recv()
            .map_err(|_| "regional result channel closed with a pending request")?
        {
            Message::Served(response) => {
                self.received += 1;
                Ok(response)
            }
            Message::Failed(error) => Err(error),
            Message::Ready(_) => Err("unexpected regional initialization reply".into()),
        }
    }
    fn finish(
        &mut self,
        mut receive: impl FnMut(Response) -> Result<(), String>,
    ) -> Result<(), String> {
        self.inputs.clear();
        let mut failure = None;
        // Initialization and request responses share the bounded channel. Drain
        // both during cleanup before joining, including failed constructors.
        while let Ok(message) = self.output.recv() {
            match message {
                Message::Ready(_) => {}
                Message::Served(response) => {
                    self.received += 1;
                    if let Err(error) = receive(response) {
                        failure.get_or_insert(error);
                    }
                }
                Message::Failed(error) => {
                    failure.get_or_insert(error);
                }
            }
        }
        for worker in self.workers.drain(..) {
            if worker.join().is_err() {
                failure.get_or_insert("regional worker join failed".into());
            }
        }
        if self.sent != self.received {
            failure.get_or_insert(format!(
                "regional workers acknowledged {} of {} requests",
                self.received, self.sent
            ));
        }
        failure.map_or(Ok(()), Err)
    }
}

enum Backend {
    Inline(Vec<chr_persistent::Search>),
    Threads(Pool),
    Closed,
}
struct Factor {
    produced: usize,
    answers: Vec<Answer>,
    done: bool,
    source: SourceStats,
    observation: ObservationStats,
}

pub struct Search {
    factors: Vec<Factor>,
    jobs: VecDeque<Job>,
    next_factor: usize,
    prefer_source: bool,
    outputs: Vec<String>,
    seen: chr_observe::AnswerSet,
    stats: Stats,
    reservations: Reservations,
    quantum: usize,
    prepared: bool,
    refuted: bool,
    stopped: bool,
    failure: Option<String>,
    cancel: Arc<AtomicBool>,
    backend: Backend,
}
impl Search {
    pub fn new(
        rules: Vec<Rule>,
        query: Query,
        mode: Mode,
        quantum: usize,
        limit: usize,
    ) -> Result<Self, String> {
        Self::create(rules, query, mode, quantum, limit, None)
    }
    #[doc(hidden)]
    pub fn new_with_worker_hook(
        rules: Vec<Rule>,
        query: Query,
        mode: Mode,
        quantum: usize,
        limit: usize,
        hook: WorkerHook,
    ) -> Result<Self, String> {
        if matches!(mode, Mode::Inline) {
            return Err("regional worker hooks require Threads".into());
        }
        Self::create(rules, query, mode, quantum, limit, Some(hook))
    }
    fn create(
        rules: Vec<Rule>,
        query: Query,
        mode: Mode,
        quantum: usize,
        limit: usize,
        hook: Option<WorkerHook>,
    ) -> Result<Self, String> {
        if quantum == 0 || limit == 0 {
            return Err("regional quantum and reservation limit must be positive".into());
        }
        let mut names = BTreeSet::new();
        for rule in &rules {
            if !names.insert(&rule.name) {
                return Err("duplicate rule name".into());
            }
            if rule.kept.is_empty() && rule.removed.is_empty() {
                return Err("empty rule heads".into());
            }
        }
        let mut names = BTreeSet::new();
        for (name, _) in &query.outputs {
            if !names.insert(name) {
                return Err("duplicate output name".into());
            }
        }
        let outputs = query.outputs.iter().map(|(name, _)| name.clone()).collect();
        let mut stats = Stats::default();
        let regions = partition::regions(rules, query, &mut stats);
        let count = regions.len();
        let cancel = Arc::new(AtomicBool::new(false));
        let (backend, initial) = match mode {
            Mode::Inline => {
                let mut searches = Vec::new();
                let mut initial = Vec::new();
                for (region, (rules, query)) in regions.into_iter().enumerate() {
                    let search = chr_persistent::Search::new(
                        rules,
                        query,
                        chr_persistent::Snapshot::Persistent,
                    )?;
                    initial.push((
                        region,
                        SourceStats::read(search.stats()),
                        ObservationStats::read(search.observation_stats()),
                    ));
                    searches.push(search);
                }
                (Backend::Inline(searches), initial)
            }
            Mode::Threads(workers) => {
                let (pool, initial) = Pool::new(regions, workers, limit, cancel.clone(), hook)?;
                (Backend::Threads(pool), initial)
            }
        };
        let mut factors = (0..count)
            .map(|_| Factor {
                produced: 0,
                answers: Vec::new(),
                done: false,
                source: SourceStats::default(),
                observation: ObservationStats::default(),
            })
            .collect::<Vec<_>>();
        let mut reservations = Reservations::new(count, limit);
        for (region, source, observation) in initial {
            factors[region].source = source;
            factors[region].observation = observation;
            reservations.actual_source[region] = source;
            reservations.actual_observation[region] = observation;
        }
        Ok(Self {
            factors,
            jobs: VecDeque::new(),
            next_factor: 0,
            prefer_source: true,
            outputs,
            seen: Default::default(),
            stats,
            reservations,
            quantum,
            prepared: false,
            refuted: false,
            stopped: false,
            failure: None,
            cancel,
            backend,
        })
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn transport_stats(&self) -> &TransportStats {
        &self.reservations.stats
    }
    pub fn factor_count(&self) -> usize {
        self.factors.len()
    }
    pub fn source_applications(&self) -> u64 {
        self.factors.iter().map(|f| f.source.applications).sum()
    }
    pub fn source_stats(&self) -> Vec<&SourceStats> {
        self.factors.iter().map(|f| &f.source).collect()
    }
    pub fn actual_source_stats(&self) -> &[SourceStats] {
        &self.reservations.actual_source
    }
    pub fn actual_regional_observation_stats(&self) -> &[ObservationStats] {
        &self.reservations.actual_observation
    }
    pub fn regional_observation_stats(&self) -> ObservationStats {
        let mut total = ObservationStats::default();
        for factor in &self.factors {
            total.term_pairs += factor.observation.term_pairs;
            total.occurrence_scans += factor.observation.occurrence_scans;
            total.occurrence_candidates += factor.observation.occurrence_candidates;
            total.backtracks += factor.observation.backtracks;
        }
        total
    }
    pub fn observation_stats(&self) -> &chr_observe::Stats {
        &self.seen.stats
    }
    pub fn raw_count(&self) -> Option<u128> {
        if self.refuted {
            return Some(0);
        }
        if !self.factors.iter().all(|f| f.done) {
            return None;
        }
        self.factors
            .iter()
            .try_fold(1u128, |n, f| n.checked_mul(f.source.completed as u128))
    }
    pub fn exhausted(&self) -> bool {
        self.refuted || (self.jobs.is_empty() && self.factors.iter().all(|f| f.done))
    }
    pub fn cached_answers(&self) -> usize {
        self.factors.iter().map(|f| f.answers.len()).sum()
    }
    pub fn pending_jobs(&self) -> usize {
        self.jobs.len()
    }

    fn next_source(&self) -> Option<usize> {
        (0..self.factors.len())
            .map(|n| (self.next_factor + n) % self.factors.len())
            .find(|&i| !self.factors[i].done)
    }
    fn prepare_requests(&mut self) -> Result<Vec<Request>, String> {
        if self.prepared || self.exhausted() {
            return Ok(Vec::new());
        }
        self.prepared = true;
        let mut requests = Vec::new();
        let Some(first) = self.next_source() else {
            return Ok(requests);
        };
        for n in 0..self.factors.len() {
            if self.reservations.stats.outstanding == self.reservations.limit {
                break;
            }
            let region = (first + n) % self.factors.len();
            self.reservations.stats.prefetch_visits += 1;
            if !self.factors[region].done && self.reservations.slots[region].is_none() {
                requests.push(self.reservations.issue(region, self.quantum)?);
            }
        }
        Ok(requests)
    }
    fn dispatch(&mut self, request: Request) -> Result<(), String> {
        match &mut self.backend {
            Backend::Inline(regions) => {
                let response = service(&mut regions[request.region], request, &self.cancel);
                self.reservations.receive(response)
            }
            Backend::Threads(pool) => pool.submit(request),
            Backend::Closed => Err("regional services are closed".into()),
        }
    }
    fn combine(&mut self, job: &Job) -> Answer {
        let mut outputs = BTreeMap::new();
        let mut residual = Vec::new();
        let mut next = 0;
        for (i, factor) in self.factors.iter().enumerate() {
            let answer = &factor.answers[job.cursor[i]];
            let mut scope = BTreeMap::new();
            for (name, term) in &answer.outputs {
                outputs.insert(
                    name.clone(),
                    rename(term, &mut scope, &mut next, &mut self.stats),
                );
            }
            for constraint in &answer.residual {
                residual.push(Constraint {
                    name: constraint.name.clone(),
                    args: constraint
                        .args
                        .iter()
                        .map(|term| rename(term, &mut scope, &mut next, &mut self.stats))
                        .collect(),
                });
            }
        }
        Answer {
            outputs: self
                .outputs
                .iter()
                .map(|name| {
                    (
                        name.clone(),
                        outputs.remove(name).expect("one certified output owner"),
                    )
                })
                .collect(),
            residual,
        }
    }
    fn complete_turn(&mut self) -> Result<Batch, String> {
        if self.exhausted() {
            return Ok(Batch {
                answers: Vec::new(),
                exhausted: true,
            });
        }
        if !self.prepared {
            return Err("regional prefetch must precede owner turn".into());
        }
        let source = self
            .next_source()
            .filter(|_| self.prefer_source || self.jobs.is_empty());
        let mut answers = Vec::new();
        if let Some(region) = source {
            while !self.reservations.ready(region) {
                let Backend::Threads(pool) = &mut self.backend else {
                    return Err("selected region lacks a reply".into());
                };
                self.reservations.receive(pool.receive()?)?;
            }
            let response = self.reservations.take(region)?;
            self.stats.source_steps += 1;
            self.factors[region].done = response.exhausted;
            self.factors[region].source = response.source;
            self.factors[region].observation = response.observation;
            for answer in response.answers {
                self.factors[region].produced += 1;
                if self.factors.len() == 1 {
                    self.stats.products += 1;
                    answers.push(answer);
                    continue;
                }
                let index = self.factors[region].answers.len();
                self.factors[region].answers.push(answer);
                let sizes = self
                    .factors
                    .iter()
                    .map(|f| f.answers.len())
                    .collect::<Vec<_>>();
                if sizes.iter().all(|&size| size > 0) {
                    let mut cursor = vec![0; sizes.len()];
                    cursor[region] = index;
                    self.jobs.push_back(Job {
                        fixed: region,
                        sizes,
                        cursor,
                    });
                    self.stats.product_jobs += 1;
                    self.stats.max_jobs = self.stats.max_jobs.max(self.jobs.len());
                }
            }
            if self.factors[region].done && self.factors[region].produced == 0 {
                self.refuted = true;
                self.stats.empty_refutations += 1;
                self.jobs.clear();
            }
            self.next_factor = (region + 1) % self.factors.len();
            self.prefer_source = false;
        } else if let Some(mut job) = self.jobs.pop_front() {
            let answer = self.combine(&job);
            self.stats.products += 1;
            if self.seen.insert(answer.clone()) {
                answers.push(answer);
            } else {
                self.stats.duplicates += 1;
            }
            if job.next() {
                self.jobs.push_back(job);
            }
            self.prefer_source = true;
        }
        self.stats.steps += 1;
        self.prepared = false;
        if self.refuted {
            self.finish_services()?;
        }
        Ok(Batch {
            answers,
            exhausted: self.exhausted(),
        })
    }
    pub fn advance(&mut self, budget: usize) -> Result<Batch, String> {
        if self.stopped {
            return Err("regional search has been shut down".into());
        }
        if let Some(error) = &self.failure {
            return Err(error.clone());
        }
        let result = self.run(budget);
        if let Err(error) = &result {
            self.failure = Some(error.clone());
        }
        result
    }
    fn run(&mut self, budget: usize) -> Result<Batch, String> {
        let mut answers = Vec::new();
        for _ in 0..budget {
            if self.exhausted() {
                break;
            }
            for request in self.prepare_requests()? {
                self.dispatch(request)?;
            }
            let batch = self.complete_turn()?;
            answers.extend(batch.answers);
            if batch.exhausted {
                break;
            }
        }
        Ok(Batch {
            answers,
            exhausted: self.exhausted(),
        })
    }
    fn finish_services(&mut self) -> Result<(), String> {
        if matches!(self.backend, Backend::Closed) {
            return Ok(());
        }
        self.cancel.store(true, Ordering::Release);
        let backend = std::mem::replace(&mut self.backend, Backend::Closed);
        let result = match backend {
            Backend::Threads(mut pool) => {
                pool.finish(|response| self.reservations.receive(response))
            }
            Backend::Inline(_) | Backend::Closed => Ok(()),
        };
        self.reservations.release();
        result
    }
    /// Signal cancellation between source steps, drain acknowledgements before
    /// joining, and never accept a regional answer into logical products here.
    pub fn shutdown(&mut self) -> Result<(), String> {
        if self.stopped {
            return self.failure.clone().map_or(Ok(()), Err);
        }
        self.stopped = true;
        if let Err(error) = self.finish_services() {
            self.failure.get_or_insert(error);
        }
        self.failure.clone().map_or(Ok(()), Err)
    }
}
impl Drop for Search {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn looping_region() -> (Vec<Rule>, Query) {
        use chr_syntax::{c, v};
        (
            vec![Rule::simplify(
                "loop",
                [c("loop", [v(0)])],
                c("loop", [v(0)]).into(),
            )],
            chr_cases::query(vec![c("loop", [v(0)])], &[0]),
        )
    }

    #[test]
    fn canceled_actual_worker_acknowledges_without_source_or_logical_steps() {
        use std::sync::{Condvar, Mutex};
        use std::time::Duration;
        let latch = Arc::new((Mutex::new(false), Condvar::new()));
        let release = latch.clone();
        let (started, observed) = mpsc::sync_channel(1);
        let hook: WorkerHook = Arc::new(move |_, _, _, phase| {
            if phase == WorkerPhase::BeforeService {
                started.send(()).unwrap();
                let (lock, condition) = &*latch;
                let mut released = lock.lock().unwrap();
                while !*released {
                    released = condition.wait(released).unwrap();
                }
            }
        });
        let (rules, query) = looping_region();
        let mut search =
            Search::new_with_worker_hook(rules, query, Mode::Threads(1), 8, 1, hook).unwrap();
        let initial = search.factors[0].source;
        for request in search.prepare_requests().unwrap() {
            search.dispatch(request).unwrap();
        }
        let entered = observed.recv_timeout(Duration::from_secs(5)).is_ok();
        // Always release the latch before asserting the witness, including on
        // timeout. This tests ordering, not relative execution speed.
        search.cancel.store(true, Ordering::Release);
        let (lock, condition) = &*release;
        *lock.lock().unwrap() = true;
        condition.notify_all();
        let Backend::Threads(pool) = &mut search.backend else {
            unreachable!()
        };
        let response = pool.receive().unwrap();
        let properties = (
            response.steps,
            response.cancelled,
            response.unserved_quantum_slots,
            response.answers.len(),
            response.exhausted,
        );
        search.reservations.receive(response).unwrap();
        // Even a received cancellation acknowledgement cannot become a source
        // turn or empty-factor refutation.
        assert!(search.reservations.take(0).is_err());
        search.shutdown().unwrap();
        assert!(
            entered,
            "worker did not reach the controlled service boundary"
        );
        assert_eq!(properties, (0, true, 8, 0, false));
        let stats = search.transport_stats();
        assert_eq!((stats.issued, stats.received, stats.accepted), (1, 1, 0));
        assert_eq!(
            (stats.actual_source_steps, stats.accepted_source_steps),
            (0, 0)
        );
        assert_eq!(
            (stats.cancelled_requests, stats.unserved_quantum_slots),
            (1, 8)
        );
        assert_eq!(stats.unaccepted_at_shutdown, 1);
        assert_eq!(search.stats().steps, 0);
        assert_eq!(search.stats().empty_refutations, 0);
        assert_eq!(search.factors[0].source, initial);
        assert_eq!(search.raw_count(), None);
    }

    #[test]
    fn cancellation_between_real_source_steps_stops_the_rest_of_the_quantum() {
        let (rules, query) = looping_region();
        let mut control = chr_persistent::Search::new(
            rules.clone(),
            query.clone(),
            chr_persistent::Snapshot::Persistent,
        )
        .unwrap();
        let expected = control.advance(1);
        assert!(!expected.exhausted);
        let mut search =
            chr_persistent::Search::new(rules, query, chr_persistent::Snapshot::Persistent)
                .unwrap();
        let cancel = AtomicBool::new(false);
        let response = service_with_step_hook(
            &mut search,
            Request {
                id: 0,
                region: 0,
                quantum: 8,
            },
            &cancel,
            |_| cancel.store(true, Ordering::Release),
        );
        assert_eq!(response.steps, 1);
        assert!(response.cancelled);
        assert_eq!(response.unserved_quantum_slots, 7);
        assert_eq!(response.source, SourceStats::read(control.stats()));
        assert_eq!(
            response.observation,
            ObservationStats::read(control.observation_stats())
        );
        assert_eq!(response.answers, expected.answers);
        assert!(!response.exhausted);
    }

    fn reply(request: Request) -> Response {
        Response {
            id: request.id,
            region: request.region,
            answers: Vec::new(),
            exhausted: false,
            source: SourceStats::default(),
            observation: ObservationStats::default(),
            steps: 0,
            cancelled: false,
            unserved_quantum_slots: 0,
        }
    }

    #[test]
    fn receipt_keeps_credit_and_protocol_errors_do_not_accept() {
        let mut reservations = Reservations::new(2, 1);
        let request = reservations.issue(0, 1).unwrap();
        let id = request.id;
        reservations.receive(reply(request)).unwrap();
        assert_eq!(reservations.stats.received, 1);
        assert_eq!(reservations.stats.accepted, 0);
        assert!(reservations.issue(1, 1).is_err());
        assert!(
            reservations
                .receive(reply(Request {
                    id,
                    region: 0,
                    quantum: 1
                }))
                .is_err()
        );
        assert!(
            reservations
                .receive(reply(Request {
                    id: id + 100,
                    region: 0,
                    quantum: 1
                }))
                .is_err()
        );
        reservations.take(0).unwrap();
        assert_eq!(reservations.stats.accepted, 1);
        assert!(
            reservations
                .receive(reply(Request {
                    id,
                    region: 0,
                    quantum: 1
                }))
                .is_err()
        );
        assert!(reservations.issue(1, 1).is_ok());
    }

    #[test]
    fn reverse_receipt_preserves_logical_region_and_product_sequence() {
        use chr_syntax::{atom, c, eq, or, v};
        let rules = ["p", "q"]
            .into_iter()
            .map(|name| {
                Rule::simplify(
                    name,
                    [c(name, [v(0)])],
                    or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
                )
            })
            .collect::<Vec<_>>();
        let query = chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]);
        let mut baseline =
            crate::Search::new(rules.clone(), query.clone(), crate::Mode::Factored).unwrap();
        let mut candidate = Search::new(rules, query, Mode::Inline, 1, 4).unwrap();
        let mut reverse_witness = false;
        for _ in 0..100 {
            let requests = candidate.prepare_requests().unwrap();
            assert!(candidate.prepare_requests().unwrap().is_empty());
            let before = candidate.stats.steps;
            reverse_witness |= requests.len() > 1;
            for request in requests.into_iter().rev() {
                let Backend::Inline(regions) = &mut candidate.backend else {
                    unreachable!()
                };
                let response = service(&mut regions[request.region], request, &candidate.cancel);
                candidate.reservations.receive(response).unwrap();
                assert_eq!(candidate.stats.steps, before);
            }
            let actual = candidate.complete_turn().unwrap();
            let expected = baseline.advance(1);
            assert_eq!(actual.exhausted, expected.exhausted);
            assert_eq!(actual.answers.len(), expected.answers.len());
            for (a, b) in actual.answers.iter().zip(&expected.answers) {
                assert!(chr_observe::equivalent(a, b, &mut Default::default()));
            }
            assert_eq!(candidate.stats.products, baseline.stats().products);
            assert_eq!(candidate.stats.product_jobs, baseline.stats().product_jobs);
            assert_eq!(
                candidate.source_applications(),
                baseline.source_applications()
            );
            if actual.exhausted {
                break;
            }
        }
        assert!(reverse_witness);
        assert_eq!(candidate.raw_count(), Some(4));
        candidate.shutdown().unwrap();
    }
}
