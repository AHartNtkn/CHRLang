//! Reusable worker-local prepared source and query lifetime (S09/T069).
use chr_persistent::continuations::{Cursor, Machine, PreparedMachine, Step};
use chr_syntax::{Answer, Query, Rule};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::panic::{AssertUnwindSafe, catch_unwind};
#[cfg(test)]
use std::sync::atomic::AtomicUsize;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::thread::{self, JoinHandle};
#[derive(Clone)]
pub struct QueryId {
    owner: Arc<()>,
    generation: u64,
}
pub struct Batch {
    pub request: u64,
    pub region: usize,
    pub answers: Vec<Answer>,
    pub raw_completions: u128,
    pub exhausted: bool,
    pub cancelled: bool,
}
pub(super) struct Search {
    #[cfg(test)]
    live: Arc<AtomicUsize>,
    machine: Machine,
    frontier: VecDeque<Cursor>,
    seen: chr_observe::AnswerSet,
    raw: u128,
}
#[cfg(test)]
impl Drop for Search {
    fn drop(&mut self) {
        self.live.fetch_sub(1, Ordering::SeqCst);
    }
}
impl Search {
    pub(super) fn new(
        prepared: &PreparedMachine,
        q: Query,
        #[cfg(test)] live: Arc<AtomicUsize>,
    ) -> Result<Self, String> {
        let (machine, cursor) = prepared.start(q)?;
        #[cfg(test)]
        live.fetch_add(1, Ordering::SeqCst);
        Ok(Self {
            #[cfg(test)]
            live,
            machine,
            frontier: VecDeque::from([cursor]),
            seen: chr_observe::AnswerSet::default(),
            raw: 0,
        })
    }
    pub(super) fn service(
        &mut self,
        request: u64,
        region: usize,
        budget: usize,
        cancel: &AtomicBool,
    ) -> Batch {
        let mut answers = Vec::new();
        let mut cancelled = false;
        for _ in 0..budget {
            if cancel.load(Ordering::Acquire) {
                cancelled = true;
                break;
            }
            let Some(cursor) = self.frontier.pop_front() else {
                break;
            };
            match self.machine.step(cursor) {
                Step::Continue(c) => self.frontier.push_back(c),
                Step::Split(a, b) => {
                    self.frontier.push_back(a);
                    self.frontier.push_back(b);
                }
                Step::Failed => {}
                Step::Answer(a) => {
                    self.raw = self.raw.checked_add(1).expect("raw count overflow");
                    if self.seen.insert(a.clone()) {
                        answers.push(a);
                    }
                }
            }
        }
        Batch {
            request,
            region,
            answers,
            raw_completions: self.raw,
            exhausted: self.frontier.is_empty(),
            cancelled,
        }
    }
}
enum Command {
    Begin(u64, Vec<(usize, Query)>, Arc<AtomicBool>),
    Service(u64, u64, usize, usize),
    End(u64),
}
enum Reply {
    Ready(usize),
    Begun(usize, u64),
    Served(u64, Batch),
    Ended(usize, u64),
    Failed(String),
}
#[cfg(test)]
pub type Hook = Arc<dyn Fn(usize, u64, u64) + Send + Sync>;
pub struct Pool {
    #[cfg(test)]
    live: Arc<AtomicUsize>,
    owner: Arc<()>,
    generation: u64,
    next_request: u64,
    active: Option<Arc<AtomicBool>>,
    regions: usize,
    capacity: usize,
    pending: BTreeMap<u64, usize>,
    inputs: Vec<mpsc::SyncSender<Command>>,
    output: mpsc::Receiver<Reply>,
    workers: Vec<JoinHandle<()>>,
    failure: Option<String>,
    closed: bool,
}
impl Pool {
    pub fn new(rules: Vec<Rule>, workers: usize, capacity: usize) -> Result<Self, String> {
        Self::create(
            rules,
            workers,
            capacity,
            #[cfg(test)]
            None,
        )
    }
    #[cfg(test)]
    pub fn with_hook(
        rules: Vec<Rule>,
        workers: usize,
        capacity: usize,
        hook: Hook,
    ) -> Result<Self, String> {
        Self::create(rules, workers, capacity, Some(hook))
    }
    fn create(
        rules: Vec<Rule>,
        count: usize,
        capacity: usize,
        #[cfg(test)] hook: Option<Hook>,
    ) -> Result<Self, String> {
        if count == 0 || capacity == 0 {
            return Err("positive worker count and capacity required".into());
        }
        let reply_capacity = capacity
            .checked_add(count)
            .ok_or("channel capacity overflow")?;
        let (send, output) = mpsc::sync_channel(reply_capacity);
        let mut pool = Self {
            #[cfg(test)]
            live: Arc::new(AtomicUsize::new(0)),
            owner: Arc::new(()),
            generation: 0,
            next_request: 0,
            active: None,
            regions: 0,
            capacity,
            pending: BTreeMap::new(),
            inputs: vec![],
            output,
            workers: vec![],
            failure: None,
            closed: false,
        };
        let rules = Arc::new(rules);
        for worker in 0..count {
            let (input, commands) = mpsc::sync_channel(capacity);
            let worker_send = send.clone();
            let rules = rules.clone();
            #[cfg(test)]
            let live = pool.live.clone();
            #[cfg(test)]
            let hook = hook.clone();
            let spawned = thread::Builder::new()
                .name(format!("chr-reusable-{worker}"))
                .spawn(move || {
                    let result = catch_unwind(AssertUnwindSafe(|| -> Result<(), String> {
                        let prepared = PreparedMachine::new((*rules).clone())?;
                        drop(rules);
                        let mut active: Option<(u64, Arc<AtomicBool>)> = None;
                        let mut searches = BTreeMap::new();
                        worker_send
                            .send(Reply::Ready(worker))
                            .map_err(|_| "reply channel closed")?;
                        while let Ok(command) = commands.recv() {
                            match command {
                                Command::Begin(epoch, queries, cancel) => {
                                    if active.is_some() {
                                        return Err("begin while query active".into());
                                    }
                                    for (region, q) in queries {
                                        searches.insert(
                                            region,
                                            Search::new(
                                                &prepared,
                                                q,
                                                #[cfg(test)]
                                                live.clone(),
                                            )?,
                                        );
                                    }
                                    active = Some((epoch, cancel));
                                    worker_send
                                        .send(Reply::Begun(worker, epoch))
                                        .map_err(|_| "reply channel closed")?;
                                }
                                Command::Service(epoch, request, region, budget) => {
                                    let (current, cancel) =
                                        active.as_ref().ok_or("service without query")?;
                                    if *current != epoch {
                                        return Err("stale service generation".into());
                                    }
                                    #[cfg(test)]
                                    if let Some(hook) = &hook {
                                        hook(worker, epoch, request);
                                    }
                                    let batch = searches
                                        .get_mut(&region)
                                        .ok_or("wrong region owner")?
                                        .service(request, region, budget, cancel);
                                    worker_send
                                        .send(Reply::Served(epoch, batch))
                                        .map_err(|_| "reply channel closed")?;
                                }
                                Command::End(epoch) => {
                                    if active.as_ref().map(|a| a.0) != Some(epoch) {
                                        return Err("stale end generation".into());
                                    }
                                    searches.clear();
                                    active = None;
                                    worker_send
                                        .send(Reply::Ended(worker, epoch))
                                        .map_err(|_| "reply channel closed")?;
                                }
                            }
                        }
                        Ok(())
                    }));
                    let error = match result {
                        Ok(Ok(())) => None,
                        Ok(Err(e)) => Some(e),
                        Err(_) => Some("worker panicked".into()),
                    };
                    if let Some(e) = error {
                        let _ = worker_send.send(Reply::Failed(format!("worker {worker}: {e}")));
                    }
                });
            match spawned {
                Ok(handle) => {
                    pool.inputs.push(input);
                    pool.workers.push(handle);
                }
                Err(e) => {
                    drop(input);
                    drop(send);
                    let _ = pool.shutdown();
                    return Err(e.to_string());
                }
            }
        }
        drop(send);
        if let Err(e) = pool.barrier(0, 0) {
            let _ = pool.shutdown();
            return Err(e);
        }
        Ok(pool)
    }
    fn healthy(&self) -> Result<(), String> {
        if self.closed {
            return Err("pool closed".into());
        }
        self.failure.clone().map_or(Ok(()), Err)
    }
    fn check(&self, id: &QueryId) -> Result<(), String> {
        self.healthy()?;
        if !Arc::ptr_eq(&id.owner, &self.owner)
            || id.generation != self.generation
            || self.active.is_none()
        {
            Err("foreign or stale query identity".into())
        } else {
            Ok(())
        }
    }
    fn fault<T>(&mut self, e: String) -> Result<T, String> {
        if let Some(c) = &self.active {
            c.store(true, Ordering::Release);
        }
        self.failure.get_or_insert(e.clone());
        Err(e)
    }
    fn barrier(&mut self, kind: u8, epoch: u64) -> Result<(), String> {
        let mut seen = BTreeSet::new();
        for _ in 0..self.workers.len() {
            let reply = self
                .output
                .recv()
                .map_err(|_| "worker channel closed".to_string());
            let worker = match reply {
                Ok(Reply::Ready(w)) if kind == 0 => w,
                Ok(Reply::Begun(w, e)) if kind == 1 && e == epoch => w,
                Ok(Reply::Ended(w, e)) if kind == 2 && e == epoch => w,
                Ok(Reply::Failed(e)) | Err(e) => return self.fault(e),
                _ => return self.fault("unexpected lifecycle acknowledgement".into()),
            };
            if worker >= self.workers.len() || !seen.insert(worker) {
                return self.fault("duplicate or foreign worker acknowledgement".into());
            }
        }
        Ok(())
    }
    pub fn begin(&mut self, queries: Vec<Query>) -> Result<QueryId, String> {
        self.healthy()?;
        if self.active.is_some() {
            return Err("end current query before begin".into());
        }
        for q in &queries {
            let mut names = BTreeSet::new();
            if q.outputs.iter().any(|(n, _)| !names.insert(n)) {
                return Err("duplicate output name".into());
            }
        }
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or("query generation overflow")?;
        let cancel = Arc::new(AtomicBool::new(false));
        self.active = Some(cancel.clone());
        self.regions = queries.len();
        let mut groups: Vec<Vec<(usize, Query)>> = (0..self.inputs.len()).map(|_| vec![]).collect();
        for (i, q) in queries.into_iter().enumerate() {
            let w = i % groups.len();
            groups[w].push((i, q));
        }
        for (input, queries) in self.inputs.iter().zip(groups) {
            if input
                .send(Command::Begin(self.generation, queries, cancel.clone()))
                .is_err()
            {
                return self.fault("begin channel closed".into());
            }
        }
        self.barrier(1, self.generation)?;
        Ok(QueryId {
            owner: self.owner.clone(),
            generation: self.generation,
        })
    }
    pub fn submit(&mut self, id: &QueryId, region: usize, budget: usize) -> Result<u64, String> {
        self.check(id)?;
        if self.active.as_ref().unwrap().load(Ordering::Acquire) {
            return Err("query cancelled".into());
        }
        if region >= self.regions || budget == 0 {
            return Err("valid region and positive quantum required".into());
        }
        if self.pending.len() == self.capacity {
            return Err("request window full".into());
        }
        let request = self.next_request;
        self.next_request = self
            .next_request
            .checked_add(1)
            .ok_or("request identity overflow")?;
        if self.inputs[region % self.inputs.len()]
            .send(Command::Service(id.generation, request, region, budget))
            .is_err()
        {
            return self.fault("service channel closed".into());
        }
        self.pending.insert(request, region);
        Ok(request)
    }
    fn accept(&mut self, epoch: u64, batch: Batch) -> Result<Batch, String> {
        if epoch != self.generation || self.pending.get(&batch.request) != Some(&batch.region) {
            return self.fault("stale or mismatched service reply".into());
        }
        self.pending.remove(&batch.request);
        Ok(batch)
    }
    pub fn receive(&mut self, id: &QueryId) -> Result<Batch, String> {
        self.check(id)?;
        if self.pending.is_empty() {
            return Err("no pending service".into());
        }
        match self.output.recv() {
            Ok(Reply::Served(epoch, batch)) => self.accept(epoch, batch),
            Ok(Reply::Failed(e)) => self.fault(e),
            _ => self.fault("unexpected service reply".into()),
        }
    }
    pub fn cancel(&mut self, id: &QueryId) -> Result<(), String> {
        self.check(id)?;
        self.active.as_ref().unwrap().store(true, Ordering::Release);
        Ok(())
    }
    pub fn end(&mut self, id: &QueryId) -> Result<(), String> {
        self.cancel(id)?;
        while !self.pending.is_empty() {
            self.receive(id)?;
        }
        for input in &self.inputs {
            if input.send(Command::End(id.generation)).is_err() {
                return self.fault("end channel closed".into());
            }
        }
        self.barrier(2, id.generation)?;
        self.active = None;
        self.regions = 0;
        Ok(())
    }
    #[cfg(test)]
    pub fn live_queries(&self) -> usize {
        self.live.load(Ordering::SeqCst)
    }
    pub fn shutdown(&mut self) -> Result<(), String> {
        if self.closed {
            return self.failure.clone().map_or(Ok(()), Err);
        }
        if let Some(c) = &self.active {
            c.store(true, Ordering::Release);
        }
        self.inputs.clear();
        while let Ok(reply) = self.output.recv() {
            match reply {
                Reply::Failed(e) => {
                    self.failure.get_or_insert(e);
                }
                Reply::Served(epoch, batch) => {
                    let _ = self.accept(epoch, batch);
                }
                Reply::Ready(_) | Reply::Begun(_, _) | Reply::Ended(_, _) => {}
            }
        }
        if !self.pending.is_empty() {
            self.failure
                .get_or_insert("workers exited without acknowledging submitted requests".into());
        }
        for w in self.workers.drain(..) {
            if w.join().is_err() {
                self.failure.get_or_insert("worker join failed".into());
            }
        }
        self.pending.clear();
        self.active = None;
        self.closed = true;
        self.failure.clone().map_or(Ok(()), Err)
    }
}
impl Drop for Pool {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

#[cfg(test)]
mod gate {
    use super::*;
    #[test]
    fn stale_reply_cannot_consume_a_current_reservation() {
        let mut p = Pool::new(vec![], 1, 1).unwrap();
        let old = p.begin(vec![]).unwrap();
        p.end(&old).unwrap();
        let fresh = p
            .begin(vec![Query {
                constraints: vec![],
                outputs: vec![],
            }])
            .unwrap();
        let request = p.submit(&fresh, 0, 1).unwrap();
        let fake = Batch {
            request,
            region: 0,
            answers: vec![],
            raw_completions: 0,
            exhausted: true,
            cancelled: false,
        };
        assert!(p.accept(old.generation, fake).is_err());
        assert!(p.pending.contains_key(&request));
        assert!(p.receive(&fresh).is_err());
        assert!(p.shutdown().is_err());
    }
}
