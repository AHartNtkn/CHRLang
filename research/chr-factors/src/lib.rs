//! Permanent independent regions with incremental fair product observation.
use chr_syntax::{Answer, Constraint, Goal, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
mod partition;
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Scalar,
    Factored,
}
#[derive(Default, Debug)]
pub struct Stats {
    pub certificate_predicates: usize,
    pub certificate_edges: u64,
    pub certificate_terms: u64,
    pub steps: u64,
    pub source_steps: u64,
    pub product_jobs: u64,
    pub products: u64,
    pub duplicates: u64,
    pub renamed_nodes: u64,
    pub max_jobs: usize,
    pub empty_refutations: u64,
}
struct Factor {
    produced: usize,
    search: chr_persistent::Search,
    answers: Vec<Answer>,
    done: bool,
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
pub struct Search {
    factors: Vec<Factor>,
    jobs: VecDeque<Job>,
    next_factor: usize,
    prefer_source: bool,
    outputs: Vec<String>,
    seen: chr_observe::AnswerSet,
    stats: Stats,
    refuted: bool,
}
fn rename(t: &Term, scope: &mut BTreeMap<Var, Var>, next: &mut u64, stats: &mut Stats) -> Term {
    stats.renamed_nodes += 1;
    match t {
        Term::Var(v) => Term::Var(*scope.entry(*v).or_insert_with(|| {
            let v = Var(*next);
            *next += 1;
            v
        })),
        Term::App(n, args) => Term::App(
            n.clone(),
            args.iter().map(|a| rename(a, scope, next, stats)).collect(),
        ),
    }
}
impl Search {
    pub fn new(rules: Vec<Rule>, query: Query, mode: Mode) -> Result<Self, String> {
        let mut names = BTreeSet::new();
        for r in &rules {
            if !names.insert(&r.name) {
                return Err("duplicate rule name".into());
            }
            if r.kept.is_empty() && r.removed.is_empty() {
                return Err("empty rule heads".into());
            }
        }
        let mut names = BTreeSet::new();
        for (n, _) in &query.outputs {
            if !names.insert(n) {
                return Err("duplicate output name".into());
            }
        }
        let outputs = query.outputs.iter().map(|(n, _)| n.clone()).collect();
        let mut stats = Stats::default();
        let regions = match mode {
            Mode::Scalar => vec![(rules, query)],
            Mode::Factored => partition::regions(rules, query, &mut stats),
        };
        let factors = regions
            .into_iter()
            .map(|(r, q)| {
                Ok(Factor {
                    produced: 0,
                    search: chr_persistent::Search::new(
                        r,
                        q,
                        chr_persistent::Snapshot::Persistent,
                    )?,
                    answers: vec![],
                    done: false,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Self {
            factors,
            jobs: VecDeque::new(),
            next_factor: 0,
            prefer_source: true,
            outputs,
            seen: Default::default(),
            stats,
            refuted: false,
        })
    }
    pub fn factor_count(&self) -> usize {
        self.factors.len()
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn source_applications(&self) -> u64 {
        self.factors
            .iter()
            .map(|f| f.search.stats().applications)
            .sum()
    }
    pub fn source_stats(&self) -> Vec<&chr_persistent::Stats> {
        self.factors.iter().map(|f| f.search.stats()).collect()
    }
    pub fn regional_observation_stats(&self) -> chr_observe::Stats {
        let mut total = chr_observe::Stats::default();
        for f in &self.factors {
            let s = f.search.observation_stats();
            total.term_pairs += s.term_pairs;
            total.occurrence_scans += s.occurrence_scans;
            total.occurrence_candidates += s.occurrence_candidates;
            total.backtracks += s.backtracks;
        }
        total
    }
    pub fn observation_stats(&self) -> &chr_observe::Stats {
        &self.seen.stats
    }
    pub fn cached_answers(&self) -> usize {
        self.factors.iter().map(|f| f.answers.len()).sum()
    }
    pub fn pending_jobs(&self) -> usize {
        self.jobs.len()
    }
    pub fn raw_count(&self) -> Option<u128> {
        if self.refuted {
            return Some(0);
        }
        if !self.factors.iter().all(|f| f.done) {
            return None;
        }
        self.factors.iter().try_fold(1u128, |n, f| {
            n.checked_mul(f.search.stats().completed as u128)
        })
    }
    pub fn exhausted(&self) -> bool {
        self.refuted || (self.jobs.is_empty() && self.factors.iter().all(|f| f.done))
    }
    fn combine(&mut self, job: &Job) -> Answer {
        let mut outputs = BTreeMap::new();
        let mut residual = vec![];
        let mut next = 0;
        for (i, f) in self.factors.iter().enumerate() {
            let a = &f.answers[job.cursor[i]];
            let mut scope = BTreeMap::new();
            for (n, t) in &a.outputs {
                outputs.insert(n.clone(), rename(t, &mut scope, &mut next, &mut self.stats));
            }
            for c in &a.residual {
                residual.push(Constraint {
                    name: c.name.clone(),
                    args: c
                        .args
                        .iter()
                        .map(|t| rename(t, &mut scope, &mut next, &mut self.stats))
                        .collect(),
                });
            }
        }
        Answer {
            outputs: self
                .outputs
                .iter()
                .map(|n| {
                    (
                        n.clone(),
                        outputs.remove(n).expect("output has one certified owner"),
                    )
                })
                .collect(),
            residual,
        }
    }
    pub fn advance(&mut self, budget: usize) -> Batch {
        let mut answers = vec![];
        for _ in 0..budget {
            if self.exhausted() {
                break;
            }
            self.stats.steps += 1;
            let source = (0..self.factors.len())
                .map(|n| (self.next_factor + n) % self.factors.len())
                .find(|i| !self.factors[*i].done);
            if let Some(i) = source.filter(|_| self.prefer_source || self.jobs.is_empty()) {
                self.stats.source_steps += 1;
                let b = self.factors[i].search.advance(1);
                self.factors[i].done = b.exhausted;
                for answer in b.answers {
                    self.factors[i].produced += 1;
                    if self.factors.len() == 1 {
                        // The identity product needs no cache, renaming or second deduplication.
                        self.stats.products += 1;
                        answers.push(answer);
                        continue;
                    }

                    let index = self.factors[i].answers.len();
                    self.factors[i].answers.push(answer);
                    let sizes = self
                        .factors
                        .iter()
                        .map(|f| f.answers.len())
                        .collect::<Vec<_>>();
                    if sizes.iter().all(|s| *s > 0) {
                        let mut cursor = vec![0; sizes.len()];
                        cursor[i] = index;
                        self.jobs.push_back(Job {
                            fixed: i,
                            sizes,
                            cursor,
                        });
                        self.stats.product_jobs += 1;
                        self.stats.max_jobs = self.stats.max_jobs.max(self.jobs.len());
                    }
                }
                if self.factors[i].done && self.factors[i].produced == 0 {
                    self.refuted = true;
                    self.stats.empty_refutations += 1;
                    self.jobs.clear();
                }
                self.next_factor = (i + 1) % self.factors.len();
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
        }
        Batch {
            answers,
            exhausted: self.exhausted(),
        }
    }
}
