//! Bounded equation lookahead with unchanged FIFO source commits.
//!
//! Cursors and their arena remain on one owner. Only owned, resolved equations
//! cross the service boundary. Atomic source/service operations are not a claim
//! of bounded-service fairness.
use crate::{EquationTable, Substitution};
use chr_persistent::continuations::{Cursor, Machine, Step};
use chr_syntax::{Answer, Query, Rule, Term};
use std::collections::{BTreeMap, VecDeque};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

#[derive(Default, Debug)]
pub struct Stats {
    pub steps: u64,
    pub completed: u64,
    pub failed: u64,
    pub duplicates: u64,
    pub max_frontier: usize,
    pub lookahead_visits: u64,
    pub projection_dereferences: u64,
    pub issued: u64,
    pub received: u64,
    pub committed_equations: u64,
    pub max_outstanding: usize,
    pub max_buffered: usize,
    pub outstanding: usize,
    pub buffered: usize,
    pub uncommitted_at_shutdown: usize,
}

pub struct Request {
    pub id: u64,
    pub left: Term,
    pub right: Term,
}

pub struct Batch {
    pub answers: Vec<Answer>,
    pub exhausted: bool,
}

struct Entry {
    cursor: Cursor,
    request: Option<u64>,
}

impl Entry {
    fn ready(cursor: Cursor) -> Self {
        Self {
            cursor,
            request: None,
        }
    }
}

pub struct Controller {
    machine: Machine,
    frontier: VecDeque<Entry>,
    // Outer None means no reply; Some(None) is a completed logical failure.
    replies: BTreeMap<u64, Option<Option<Substitution>>>,
    seen: chr_observe::AnswerSet,
    outstanding_limit: usize,
    lookahead: usize,
    next_request: u64,
    prepared: bool,
    stats: Stats,
}

impl Controller {
    pub fn new(
        rules: Vec<Rule>,
        query: Query,
        outstanding: usize,
        lookahead: usize,
    ) -> Result<Self, String> {
        if outstanding == 0 || lookahead == 0 {
            return Err("outstanding and lookahead bounds must be positive".into());
        }
        let (machine, cursor) = Machine::new(rules, query)?;
        Ok(Self {
            machine,
            frontier: VecDeque::from([Entry::ready(cursor)]),
            replies: BTreeMap::new(),
            seen: Default::default(),
            outstanding_limit: outstanding,
            lookahead,
            next_request: 0,
            prepared: false,
            stats: Stats {
                max_frontier: 1,
                ..Default::default()
            },
        })
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn source_stats(&self) -> &chr_persistent::Stats {
        self.machine.stats()
    }
    pub fn observation_stats(&self) -> &chr_observe::Stats {
        &self.seen.stats
    }

    /// Exactly one bounded pass per next committed transition, including passes
    /// that issue no requests. Receipt never resets this latch.
    pub fn prepare(&mut self) -> Result<Vec<Request>, String> {
        if self.prepared {
            return Ok(Vec::new());
        }
        self.prepared = true;
        let mut requests = Vec::new();
        for entry in self.frontier.iter_mut().take(self.lookahead) {
            if self.replies.len() == self.outstanding_limit {
                break;
            }
            self.stats.lookahead_visits += 1;
            if entry.request.is_some() || !self.machine.has_pending_equation(&entry.cursor) {
                continue;
            }
            let id = self.next_request;
            self.next_request = id.checked_add(1).ok_or("equation request ID overflow")?;
            let before = self.machine.stats().dereferences;
            let (left, right) = self
                .machine
                .pending_equation(&entry.cursor)
                .expect("equation checked on unchanged cursor");
            self.stats.projection_dereferences += self.machine.stats().dereferences - before;
            entry.request = Some(id);
            self.replies.insert(id, None);
            self.stats.issued += 1;
            requests.push(Request { id, left, right });
        }
        self.stats.outstanding = self.replies.len();
        self.stats.max_outstanding = self.stats.max_outstanding.max(self.replies.len());
        Ok(requests)
    }

    pub fn accept(&mut self, id: u64, solution: Option<Substitution>) -> Result<(), String> {
        let reply = self
            .replies
            .get_mut(&id)
            .ok_or_else(|| format!("unknown equation reply {id}"))?;
        if reply.is_some() {
            return Err(format!("duplicate equation reply {id}"));
        }
        *reply = Some(solution);
        self.stats.received += 1;
        self.stats.buffered += 1;
        self.stats.max_buffered = self.stats.max_buffered.max(self.stats.buffered);
        Ok(())
    }

    /// None means the front is parked, not exhausted. A successful call commits
    /// at most one source transition. Call prepare before each new attempt.
    pub fn commit(&mut self) -> Result<Option<Batch>, String> {
        let Some(front) = self.frontier.front() else {
            return Ok(Some(Batch {
                answers: Vec::new(),
                exhausted: true,
            }));
        };
        if !self.prepared {
            return Err("prepare must precede a commit attempt".into());
        }
        if front.request.is_some_and(|id| self.replies[&id].is_none()) {
            return Ok(None);
        }
        let entry = self.frontier.pop_front().expect("front checked");
        let event = if let Some(id) = entry.request {
            let solution = self
                .replies
                .remove(&id)
                .expect("reserved request")
                .expect("reply checked");
            self.stats.buffered -= 1;
            self.stats.outstanding = self.replies.len();
            self.stats.committed_equations += 1;
            self.machine.complete_equation(entry.cursor, solution)
        } else {
            // FIFO lookahead always visits the front first. Therefore a front
            // equation cannot execute via the shared-arena fallback.
            assert!(!self.machine.has_pending_equation(&entry.cursor));
            self.machine.step(entry.cursor)
        };
        self.prepared = false;
        self.stats.steps += 1;
        let mut answers = Vec::new();
        match event {
            Step::Continue(cursor) => self.frontier.push_back(Entry::ready(cursor)),
            Step::Split(left, right) => {
                self.frontier.push_back(Entry::ready(left));
                self.frontier.push_back(Entry::ready(right));
            }
            Step::Failed => self.stats.failed += 1,
            Step::Answer(answer) => {
                self.stats.completed += 1;
                if self.seen.insert(answer.clone()) {
                    answers.push(answer);
                } else {
                    self.stats.duplicates += 1;
                }
            }
        }
        self.stats.max_frontier = self.stats.max_frontier.max(self.frontier.len());
        Ok(Some(Batch {
            answers,
            exhausted: self.exhausted(),
        }))
    }

    fn exhausted(&self) -> bool {
        self.frontier.is_empty() && self.replies.is_empty()
    }

    fn release_uncommitted(&mut self) {
        self.stats.uncommitted_at_shutdown = self.replies.len();
        self.frontier.clear();
        self.replies.clear();
        self.stats.outstanding = 0;
        self.stats.buffered = 0;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Inline,
    Threads(usize),
}

/// Test synchronization boundaries around the actual direct solver.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerPhase {
    BeforeSolve,
    AfterSolve,
}

/// Arguments are zero-based worker index, request ID, and solver boundary.
/// A hook cannot replace the solver. It is unsupported for Inline mode.
pub type WorkerHook = Arc<dyn Fn(usize, u64, WorkerPhase) + Send + Sync>;

struct Response {
    id: u64,
    solution: Option<Substitution>,
    stats: crate::Stats,
}

fn solve(request: Request) -> Response {
    let mut table = EquationTable::new(crate::Mode::Direct);
    let solution = table.solve(&request.left, &request.right);
    // A direct table has no retained cache. Transfer its actual counters with
    // each reply, including replies completed after a caller's prefix stop.
    let s = table.stats();
    Response {
        id: request.id,
        solution,
        stats: crate::Stats {
            calls: s.calls,
            computed: s.computed,
            hits: s.hits,
            key_nodes: s.key_nodes,
            replay_nodes: s.replay_nodes,
            pairs: s.pairs,
            resolve_nodes: s.resolve_nodes,
            occurs_nodes: s.occurs_nodes,
            cache_entries: s.cache_entries,
        },
    }
}

fn add_stats(total: &mut crate::Stats, s: &crate::Stats) {
    total.calls += s.calls;
    total.computed += s.computed;
    total.hits += s.hits;
    total.key_nodes += s.key_nodes;
    total.replay_nodes += s.replay_nodes;
    total.pairs += s.pairs;
    total.resolve_nodes += s.resolve_nodes;
    total.occurs_nodes += s.occurs_nodes;
    total.cache_entries += s.cache_entries;
}

enum Message {
    Solved(Response),
    Failed(String),
}

struct Pool {
    requests: Option<mpsc::SyncSender<Request>>,
    replies: mpsc::Receiver<Message>,
    workers: Vec<JoinHandle<()>>,
    sent: u64,
    received: u64,
}

impl Pool {
    fn new(count: usize, capacity: usize, hook: Option<WorkerHook>) -> Result<Self, String> {
        if count == 0 {
            return Err("worker count must be positive".into());
        }
        let (requests, input) = mpsc::sync_channel::<Request>(capacity);
        let input = Arc::new(Mutex::new(input));
        // Registered gate transport: requests have the outstanding capacity,
        // while replies have one slot. Thus prefix cleanup must drain worker
        // output before joining even when several requests are still active.
        // Submitting a pass cannot deadlock on this backpressure: all accepted
        // but uncommitted requests together are bounded by request capacity, so
        // the entire new pass fits without requiring another result receive.
        let (output, replies) = mpsc::sync_channel(1);
        let mut pool = Self {
            requests: Some(requests),
            replies,
            workers: Vec::new(),
            sent: 0,
            received: 0,
        };
        for worker in 0..count {
            let input = input.clone();
            let worker_output = output.clone();
            let hook = hook.clone();
            let spawned = thread::Builder::new()
                .name(format!("chr-equation-{worker}"))
                .spawn(move || {
                    let outcome = catch_unwind(AssertUnwindSafe(|| -> Result<(), String> {
                        loop {
                            // The receiver mutex is released before either hook
                            // or solver runs. Cursors never enter this closure.
                            let request = {
                                let receiver =
                                    input.lock().map_err(|_| "request receiver poisoned")?;
                                receiver.recv()
                            };
                            let Ok(request) = request else {
                                return Ok(());
                            };
                            let id = request.id;
                            if let Some(h) = &hook {
                                h(worker, id, WorkerPhase::BeforeSolve);
                            }
                            let response = solve(request);
                            if let Some(h) = &hook {
                                h(worker, id, WorkerPhase::AfterSolve);
                            }
                            worker_output
                                .send(Message::Solved(response))
                                .map_err(|_| "result receiver closed")?;
                        }
                    }));
                    let failure = match outcome {
                        Ok(Ok(())) => None,
                        Ok(Err(error)) => Some(format!("equation worker {worker}: {error}")),
                        Err(_) => Some(format!("equation worker {worker} panicked")),
                    };
                    if let Some(error) = failure {
                        let _ = worker_output.send(Message::Failed(error));
                    }
                });
            match spawned {
                Ok(handle) => pool.workers.push(handle),
                Err(error) => {
                    // No requests have been issued. Drop the producer and the
                    // creator's result sender before waiting for existing workers.
                    pool.requests.take();
                    drop(output);
                    let _ = pool.finish(|_| Ok(()));
                    return Err(format!("cannot create equation worker: {error}"));
                }
            }
        }
        // Only workers retain result senders, so shutdown can drain to closure.
        drop(output);
        Ok(pool)
    }

    fn submit(&mut self, request: Request) -> Result<(), String> {
        self.requests
            .as_ref()
            .ok_or("worker pool is shut down")?
            .send(request)
            .map_err(|_| "all equation request receivers closed")?;
        self.sent += 1;
        Ok(())
    }

    fn receive(&mut self) -> Result<Response, String> {
        match self
            .replies
            .recv()
            .map_err(|_| "equation result channel closed with a pending request")?
        {
            Message::Solved(response) => {
                self.received += 1;
                Ok(response)
            }
            Message::Failed(error) => Err(error),
        }
    }

    fn finish(
        &mut self,
        mut accept: impl FnMut(Response) -> Result<(), String>,
    ) -> Result<(), String> {
        self.requests.take();
        let mut failure = None;
        // Drain before join: a worker can be sending into a bounded channel.
        // Worker failures have an explicit message even if other workers live.
        while let Ok(message) = self.replies.recv() {
            match message {
                Message::Solved(response) => {
                    self.received += 1;
                    if let Err(error) = accept(response) {
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
                failure.get_or_insert("equation worker join failed".into());
            }
        }
        if self.received != self.sent {
            failure.get_or_insert(format!(
                "equation workers returned {} of {} accepted requests",
                self.received, self.sent
            ));
        }
        failure.map_or(Ok(()), Err)
    }
}

enum Backend {
    Inline,
    Threads(Pool),
}

pub struct Search {
    controller: Controller,
    backend: Backend,
    operation_stats: crate::Stats,
    stopped: bool,
    failure: Option<String>,
}

impl Search {
    pub fn new(
        rules: Vec<Rule>,
        query: Query,
        mode: Mode,
        outstanding: usize,
        lookahead: usize,
    ) -> Result<Self, String> {
        Self::create(rules, query, mode, outstanding, lookahead, None)
    }

    #[doc(hidden)]
    pub fn new_with_worker_hook(
        rules: Vec<Rule>,
        query: Query,
        mode: Mode,
        outstanding: usize,
        lookahead: usize,
        hook: WorkerHook,
    ) -> Result<Self, String> {
        if matches!(mode, Mode::Inline) {
            return Err("worker hooks require Threads mode".into());
        }
        Self::create(rules, query, mode, outstanding, lookahead, Some(hook))
    }

    fn create(
        rules: Vec<Rule>,
        query: Query,
        mode: Mode,
        outstanding: usize,
        lookahead: usize,
        hook: Option<WorkerHook>,
    ) -> Result<Self, String> {
        let controller = Controller::new(rules, query, outstanding, lookahead)?;
        let backend = match mode {
            Mode::Inline => Backend::Inline,
            Mode::Threads(count) => Backend::Threads(Pool::new(count, outstanding, hook)?),
        };
        Ok(Self {
            controller,
            backend,
            operation_stats: Default::default(),
            stopped: false,
            failure: None,
        })
    }

    pub fn stats(&self) -> &Stats {
        self.controller.stats()
    }
    pub fn source_stats(&self) -> &chr_persistent::Stats {
        self.controller.source_stats()
    }
    pub fn observation_stats(&self) -> &chr_observe::Stats {
        self.controller.observation_stats()
    }
    /// Counts only responses received so far. After successful shutdown this
    /// includes every accepted request, including uncommitted prefix work.
    pub fn operation_stats(&self) -> &crate::Stats {
        &self.operation_stats
    }

    fn accept_response(&mut self, response: Response) -> Result<(), String> {
        add_stats(&mut self.operation_stats, &response.stats);
        self.controller.accept(response.id, response.solution)
    }

    pub fn advance(&mut self, budget: usize) -> Result<Batch, String> {
        if self.stopped {
            return Err("search has been shut down".into());
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
            for request in self.controller.prepare()? {
                match &mut self.backend {
                    Backend::Inline => self.accept_response(solve(request))?,
                    Backend::Threads(pool) => pool.submit(request)?,
                }
            }
            let batch = loop {
                if let Some(batch) = self.controller.commit()? {
                    break batch;
                }
                let response = match &mut self.backend {
                    Backend::Threads(pool) => pool.receive()?,
                    Backend::Inline => return Err("inline front lacks an issued reply".into()),
                };
                self.accept_response(response)?;
            };
            answers.extend(batch.answers);
            if batch.exhausted {
                break;
            }
        }
        Ok(Batch {
            answers,
            exhausted: self.controller.exhausted(),
        })
    }

    /// Finish accepted service work without any source commits, then release
    /// parked cursors/results. Later advance calls are errors. Idempotent.
    pub fn shutdown(&mut self) -> Result<(), String> {
        if self.stopped {
            return self.failure.clone().map_or(Ok(()), Err);
        }
        self.stopped = true;
        if let Backend::Threads(mut pool) = std::mem::replace(&mut self.backend, Backend::Inline)
            && let Err(error) = pool.finish(|response| self.accept_response(response))
        {
            self.failure.get_or_insert(error);
        }
        self.controller.release_uncommitted();
        self.failure.clone().map_or(Ok(()), Err)
    }
}

impl Drop for Search {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}
