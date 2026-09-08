//! Certified products over reusable workers and the same inline source service.
use super::workers::{self, Pool, QueryId, Search};
#[cfg(feature = "worker-lowering")]
#[path = "contracted_region.rs"]
mod contracted;
use chr_persistent::continuations::PreparedMachine;
use chr_syntax::{Answer, Constraint, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::{Arc, atomic::AtomicBool};
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Inline,
    Threads(usize),
    #[cfg(feature = "worker-lowering")]
    Contracted,
}
enum Backend {
    Inline(PreparedMachine),
    Threads(Pool),
    #[cfg(feature = "worker-lowering")]
    Contracted(chr_compiled::PreparedRuleset),
    Closed,
}
enum SerialSearches {
    Persistent(Vec<Search>),
    #[cfg(feature = "worker-lowering")]
    Contracted(Vec<contracted::Search>),
}
impl SerialSearches {
    fn service(
        &mut self,
        request: u64,
        region: usize,
        budget: usize,
        cancel: &AtomicBool,
    ) -> workers::Batch {
        match self {
            Self::Persistent(s) => s[region].service(request, region, budget, cancel),
            #[cfg(feature = "worker-lowering")]
            Self::Contracted(s) => s[region].service(request, region, budget, cancel),
        }
    }
    fn clear(&mut self) {
        match self {
            Self::Persistent(s) => s.clear(),
            #[cfg(feature = "worker-lowering")]
            Self::Contracted(s) => s.clear(),
        }
    }
}
pub struct Runtime {
    rules: Vec<Rule>,
    backend: Backend,
    quantum: usize,
    window: usize,
}
impl Runtime {
    pub fn new(
        rules: Vec<Rule>,
        mode: Mode,
        quantum: usize,
        window: usize,
    ) -> Result<Self, String> {
        if quantum == 0 || window == 0 {
            return Err("positive source quantum and window required".into());
        }
        let backend = match mode {
            #[cfg(feature = "worker-lowering")]
            Mode::Contracted => Backend::Contracted(
                chr_compiled::PreparedRuleset::new(rules.clone(), None)?
                    .specialize_inferred()
                    .contract_carriers_inferred()?,
            ),
            Mode::Inline => Backend::Inline(PreparedMachine::new(rules.clone())?),
            Mode::Threads(n) => Backend::Threads(Pool::new(rules.clone(), n, window)?),
        };
        Ok(Self {
            rules,
            backend,
            quantum,
            window,
        })
    }
    pub fn start(&mut self, q: Query) -> Result<Session<'_>, String> {
        if matches!(self.backend, Backend::Closed) {
            return Err("runtime closed".into());
        }
        let mut names = BTreeSet::new();
        if q.outputs.iter().any(|(n, _)| !names.insert(n.clone())) {
            return Err("duplicate output name".into());
        }
        let outputs = q.outputs.iter().map(|(n, _)| n.clone()).collect();
        let mut certificate = chr_factors::Stats::default();
        let queries = chr_factors::partition::regions(self.rules.clone(), q, &mut certificate)
            .into_iter()
            .map(|(_, q)| q)
            .collect::<Vec<_>>();
        let count = queries.len();
        let cancel = Arc::new(AtomicBool::new(false));
        #[cfg(test)]
        let live = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let (token, inline) = match &mut self.backend {
            Backend::Inline(p) => {
                let mut searches = Vec::new();
                for q in queries {
                    searches.push(Search::new(
                        p,
                        q,
                        #[cfg(test)]
                        live.clone(),
                    )?);
                }
                (None, SerialSearches::Persistent(searches))
            }
            #[cfg(feature = "worker-lowering")]
            Backend::Contracted(p) => {
                let searches = queries
                    .into_iter()
                    .map(|q| contracted::Search::new(p, q))
                    .collect::<Result<Vec<_>, _>>()?;
                (None, SerialSearches::Contracted(searches))
            }
            Backend::Threads(p) => (
                Some(p.begin(queries)?),
                SerialSearches::Persistent(Vec::new()),
            ),
            Backend::Closed => unreachable!(),
        };
        Ok(Session {
            backend: &mut self.backend,
            token,
            inline,
            cancel,
            quantum: self.quantum,
            window: self.window,
            factors: (0..count)
                .map(|_| Factor {
                    answers: Vec::new(),
                    done: false,
                    pending: false,
                    raw: 0,
                })
                .collect(),
            outputs,
            flights: VecDeque::new(),
            replies: BTreeMap::new(),
            jobs: VecDeque::new(),
            next_factor: 0,
            next_request: 0,
            prefer_source: true,
            seen: chr_observe::AnswerSet::default(),
            refuted: false,
            closed: false,
            certificate,
        })
    }
    pub fn shutdown(&mut self) -> Result<(), String> {
        match std::mem::replace(&mut self.backend, Backend::Closed) {
            Backend::Threads(mut p) => p.shutdown(),
            #[cfg(feature = "worker-lowering")]
            Backend::Contracted(_) => Ok(()),
            Backend::Inline(_) | Backend::Closed => Ok(()),
        }
    }
}
struct Factor {
    answers: Vec<Answer>,
    done: bool,
    pending: bool,
    raw: u128,
}
struct Job {
    fixed: usize,
    sizes: Vec<usize>,
    cursor: Vec<usize>,
}
impl Job {
    fn next(&mut self) -> bool {
        for i in (0..self.cursor.len()).rev() {
            if i == self.fixed {
                continue;
            }
            self.cursor[i] += 1;
            if self.cursor[i] < self.sizes[i] {
                return true;
            }
            self.cursor[i] = 0;
        }
        false
    }
}
pub struct Batch {
    pub answers: Vec<Answer>,
    pub exhausted: bool,
}
pub struct Session<'a> {
    backend: &'a mut Backend,
    token: Option<QueryId>,
    inline: SerialSearches,
    cancel: Arc<AtomicBool>,
    quantum: usize,
    window: usize,
    factors: Vec<Factor>,
    outputs: Vec<String>,
    flights: VecDeque<(u64, usize)>,
    replies: BTreeMap<u64, workers::Batch>,
    jobs: VecDeque<Job>,
    next_factor: usize,
    next_request: u64,
    prefer_source: bool,
    seen: chr_observe::AnswerSet,
    refuted: bool,
    closed: bool,
    pub certificate: chr_factors::Stats,
}
fn rename(term: &Term, scope: &mut BTreeMap<Var, Var>, next: &mut u64) -> Term {
    match term {
        Term::Var(v) => Term::Var(*scope.entry(*v).or_insert_with(|| {
            let v = Var(*next);
            *next = next.checked_add(1).expect("output variable overflow");
            v
        })),
        Term::App(n, xs) => Term::App(
            n.clone(),
            xs.iter().map(|t| rename(t, scope, next)).collect(),
        ),
    }
}
impl Session<'_> {
    pub fn factor_count(&self) -> usize {
        self.factors.len()
    }
    pub fn raw_count(&self) -> Option<u128> {
        if self.refuted {
            return Some(0);
        }
        if self.factors.iter().any(|f| !f.done) {
            return None;
        }
        Some(
            self.factors
                .iter()
                .try_fold(1u128, |n, f| n.checked_mul(f.raw))
                .expect("raw product count overflow"),
        )
    }
    pub fn exhausted(&self) -> bool {
        self.refuted
            || (self.factors.iter().all(|f| f.done)
                && self.jobs.is_empty()
                && self.flights.is_empty())
    }
    fn fill(&mut self) -> Result<(), String> {
        for _ in 0..self.factors.len() {
            if self.flights.len() == self.window {
                break;
            }
            let region = self.next_factor;
            self.next_factor = (region + 1) % self.factors.len();
            if self.factors[region].done || self.factors[region].pending {
                continue;
            }
            let request = match self.backend {
                Backend::Threads(p) => {
                    p.submit(self.token.as_ref().unwrap(), region, self.quantum)?
                }
                Backend::Closed => return Err("runtime closed".into()),
                _ => {
                    let request = self.next_request;
                    self.next_request = self
                        .next_request
                        .checked_add(1)
                        .ok_or("request identity overflow")?;
                    let batch = self
                        .inline
                        .service(request, region, self.quantum, &self.cancel);
                    self.replies.insert(request, batch);
                    request
                }
            };
            self.factors[region].pending = true;
            self.flights.push_back((request, region));
        }
        Ok(())
    }
    fn source(&mut self) -> Result<(), String> {
        self.fill()?;
        let Some(&(request, region)) = self.flights.front() else {
            return Ok(());
        };
        while !self.replies.contains_key(&request) {
            let Backend::Threads(p) = self.backend else {
                return Err("missing inline reply".into());
            };
            let batch = p.receive(self.token.as_ref().unwrap())?;
            if !self
                .flights
                .iter()
                .any(|&(id, r)| id == batch.request && r == batch.region)
                || self.replies.contains_key(&batch.request)
            {
                return Err("foreign or duplicate product-owner reply".into());
            }
            self.replies.insert(batch.request, batch);
        }
        let batch = self.replies.remove(&request).unwrap();
        self.flights.pop_front();
        if batch.cancelled {
            return Err("unexpected cancellation during query observation".into());
        }
        self.factors[region].pending = false;
        self.factors[region].done = batch.exhausted;
        self.factors[region].raw = batch.raw_completions;
        for answer in batch.answers {
            self.factors[region].answers.push(answer);
            let sizes = self
                .factors
                .iter()
                .map(|f| f.answers.len())
                .collect::<Vec<_>>();
            if sizes.iter().all(|n| *n > 0) {
                let mut cursor = vec![0; sizes.len()];
                cursor[region] = sizes[region] - 1;
                self.jobs.push_back(Job {
                    fixed: region,
                    sizes,
                    cursor,
                });
            }
        }
        if self.factors[region].done && self.factors[region].answers.is_empty() {
            self.refuted = true;
            self.jobs.clear();
            if let Backend::Threads(p) = self.backend {
                p.cancel(self.token.as_ref().unwrap())?;
            }
        }
        Ok(())
    }
    fn combine(&self, job: &Job) -> Answer {
        let mut outputs = BTreeMap::new();
        let mut residual = Vec::new();
        let mut next = 0;
        for (region, factor) in self.factors.iter().enumerate() {
            let answer = &factor.answers[job.cursor[region]];
            let mut scope = BTreeMap::new();
            for (name, term) in &answer.outputs {
                assert!(
                    outputs
                        .insert(name.clone(), rename(term, &mut scope, &mut next))
                        .is_none()
                );
            }
            for c in &answer.residual {
                residual.push(Constraint {
                    name: c.name.clone(),
                    args: c
                        .args
                        .iter()
                        .map(|t| rename(t, &mut scope, &mut next))
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
                        outputs.remove(name).expect("certified output owner"),
                    )
                })
                .collect(),
            residual,
        }
    }
    pub fn advance(&mut self, budget: usize) -> Result<Batch, String> {
        if self.closed {
            return Err("session closed".into());
        }
        let mut answers = Vec::new();
        for _ in 0..budget {
            if self.exhausted() {
                break;
            }
            if !self.jobs.is_empty() && (!self.prefer_source || self.factors.iter().all(|f| f.done))
            {
                let mut job = self.jobs.pop_front().unwrap();
                let answer = self.combine(&job);
                if job.next() {
                    self.jobs.push_back(job);
                }
                if self.seen.insert(answer.clone()) {
                    answers.push(answer);
                }
                self.prefer_source = true;
            } else {
                self.source()?;
                self.prefer_source = false;
            }
        }
        Ok(Batch {
            answers,
            exhausted: self.exhausted(),
        })
    }
    pub fn close(&mut self) -> Result<(), String> {
        if self.closed {
            return Ok(());
        }
        let result = if let Backend::Threads(p) = self.backend {
            p.end(self.token.as_ref().unwrap())
        } else {
            Ok(())
        };
        self.inline.clear();
        self.token = None;
        self.replies.clear();
        self.flights.clear();
        self.jobs.clear();
        self.closed = true;
        result
    }
}
impl Drop for Session<'_> {
    fn drop(&mut self) {
        let _ = self.close();
    }
}
