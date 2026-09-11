//! Finite-support conditional CHR experiment; independent of both scalar engines.
use chr_syntax::{Answer, Query, Rule};
use std::collections::{BTreeSet, VecDeque};
mod equality;
mod kernel;
type Support = BTreeSet<u64>;
#[derive(Clone, Copy, Debug)]
pub enum Grouping {
    Singleton,
    Ready,
}
#[derive(Default, Debug)]
pub struct Stats {
    pub quanta: u64,
    pub projected_steps: u64,
    pub expansions: u64,
    pub projected_applications: u64,
    pub introductions: u64,
    pub projected_introductions: u64,
    pub splits: u64,
    pub failed: u64,
    pub completed: u64,
    pub duplicates: u64,
    pub support_reads: u64,
    pub support_writes: u64,
    pub occurrence_scans: u64,
    pub binding_scans: u64,
    pub match_candidates: u64,
    pub term_visits: u64,
    pub term_copies: u64,
    pub unification_pairs: u64,
    pub occurs_visits: u64,
    pub previewed: u64,
    pub queue_scans: u64,
    pub work_nodes: u64,
    pub queue_refs: u64,
    pub max_frontier: usize,
}
pub struct Batch {
    pub answers: Vec<Answer>,
    pub exhausted: bool,
}
/// Read-only experimental projection. Runtime identities are not source terms or rule predicates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Projection {
    pub ticket: u64,
    pub pending: Vec<(u64, chr_syntax::Goal)>,
    pub store: Vec<(u64, chr_syntax::Constraint)>,
    pub bindings: Vec<(chr_syntax::Var, chr_syntax::Term)>,
    pub history: Vec<(usize, Vec<u64>)>,
    pub born_variables: Vec<chr_syntax::Var>,
    pub born_occurrences: Vec<u64>,
}
#[derive(Debug)]
pub struct Retained {
    pub occurrences: usize,
    pub variables: usize,
    pub occurrence_births: usize,
    pub binding_edges: usize,
    pub tokens: usize,
    pub support_memberships: usize,
}
pub struct Search {
    last_served: Vec<u64>,
    kernel: kernel::Kernel,
    frames: VecDeque<kernel::Frame>,
    grouping: Grouping,
    next_ticket: u64,
    seen: chr_observe::AnswerSet,
    stats: Stats,
}
impl Search {
    pub fn new(rules: Vec<Rule>, query: Query, grouping: Grouping) -> Result<Self, String> {
        let mut names = BTreeSet::new();
        for r in &rules {
            if r.kept.is_empty() && r.removed.is_empty() {
                return Err("empty rule heads".into());
            }
            if !names.insert(&r.name) {
                return Err("duplicate rule name".into());
            }
        }
        let mut names = BTreeSet::new();
        for (n, _) in &query.outputs {
            if !names.insert(n) {
                return Err("duplicate output name".into());
            }
        }
        let mut stats = Stats {
            max_frontier: 1,
            ..Stats::default()
        };
        let (kernel, frame) = kernel::Kernel::new(rules, query, &mut stats);
        Ok(Self {
            last_served: vec![],
            kernel,
            frames: VecDeque::from([frame]),
            grouping,
            next_ticket: 1,
            seen: chr_observe::AnswerSet::default(),
            stats,
        })
    }
    pub fn advance(&mut self, budget: usize) -> Batch {
        use kernel::{Kind, Operation};
        let mut answers = vec![];
        for _ in 0..budget {
            let Some(first) = self.frames.front() else {
                break;
            };
            let first_ticket = first.ticket;
            let operation = self.kernel.preview(
                first_ticket,
                first.pending.front().cloned(),
                &mut self.stats,
            );
            let mut served = Support::from([first_ticket]);
            let groupable = matches!(&operation, Operation::Apply { .. })
                || matches!(&operation,Operation::Pending(w) if matches!(w.kind,Kind::Insert(_)));
            if matches!(self.grouping, Grouping::Ready) && groupable {
                let candidates = self
                    .frames
                    .iter()
                    .skip(1)
                    .take(255)
                    .map(|f| (f.ticket, f.pending.front().cloned()))
                    .collect::<Vec<_>>();
                for (ticket, job) in candidates {
                    self.stats.queue_scans += 1;
                    let other = self.kernel.preview(ticket, job, &mut self.stats);
                    if compatible(&operation, &other, &mut self.stats) {
                        served.insert(ticket);
                    }
                }
            }
            let mut selected = vec![];
            if served.len() == 1 {
                self.stats.queue_scans += 1;
                selected.push(self.frames.pop_front().unwrap());
            } else {
                let mut remaining = VecDeque::new();
                while let Some(f) = self.frames.pop_front() {
                    self.stats.queue_scans += 1;
                    if served.contains(&f.ticket) {
                        selected.push(f);
                    } else {
                        remaining.push_back(f);
                    }
                }
                self.frames = remaining;
            }
            self.last_served = served.iter().copied().collect();
            self.stats.quanta += 1;
            self.stats.projected_steps += selected.len() as u64;
            match operation {
                Operation::Apply { rule, ids, scope } => {
                    let body = self
                        .kernel
                        .apply(rule, ids, scope, &served, &mut self.stats);
                    for mut f in selected {
                        f.pending.push_back(body.clone());
                        self.stats.queue_refs += 1;
                        self.frames.push_back(f);
                    }
                }
                Operation::Pending(job) => {
                    for f in &mut selected {
                        assert_eq!(f.pending.pop_front().unwrap().id, job.id);
                    }
                    match &job.kind {
                        Kind::Insert(c) => {
                            self.kernel.insert(c, &served, &mut self.stats);
                            self.frames.extend(selected);
                        }
                        Kind::Equal(a, b) => {
                            for f in selected {
                                if self.kernel.equate(a, b, f.ticket, &mut self.stats) {
                                    self.frames.push_back(f);
                                } else {
                                    self.stats.failed += 1;
                                }
                            }
                        }
                        Kind::And(work) => {
                            for mut f in selected {
                                for child in work.iter().rev() {
                                    f.pending.push_front(child.clone());
                                    self.stats.queue_refs += 1;
                                }
                                self.frames.push_back(f);
                            }
                        }
                        Kind::Or(a, b) => {
                            for mut f in selected {
                                let left = self.next_ticket;
                                let right = left + 1;
                                self.next_ticket += 2;
                                self.kernel.fork(f.ticket, left, right, &mut self.stats);
                                self.stats.queue_refs += f.pending.len() as u64 + 2;
                                let mut sibling = kernel::Frame {
                                    ticket: right,
                                    pending: f.pending.clone(),
                                };
                                f.ticket = left;
                                f.pending.push_front(a.clone());
                                sibling.pending.push_front(b.clone());
                                self.frames.push_back(f);
                                self.frames.push_back(sibling);
                                self.stats.splits += 1;
                            }
                        }
                        Kind::True => self.frames.extend(selected),
                        Kind::Fail => self.stats.failed += selected.len() as u64,
                    }
                }
                Operation::Done => {
                    for f in selected {
                        let answer = self.kernel.answer(f.ticket, &mut self.stats);
                        self.stats.completed += 1;
                        if self.seen.insert(answer.clone()) {
                            answers.push(answer);
                        } else {
                            self.stats.duplicates += 1;
                        }
                    }
                }
            }
            self.stats.max_frontier = self.stats.max_frontier.max(self.frames.len());
        }
        Batch {
            answers,
            exhausted: self.frames.is_empty(),
        }
    }
    pub fn checkpoint(&self) -> Vec<Projection> {
        self.frames.iter().map(|f| self.kernel.project(f)).collect()
    }
    pub fn last_served(&self) -> &[u64] {
        &self.last_served
    }
    pub fn retained(&self) -> Retained {
        self.kernel.retained()
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn observation_stats(&self) -> &chr_observe::Stats {
        &self.seen.stats
    }
    pub fn validate(&self) -> Result<(), String> {
        let ids = self.frames.iter().map(|f| f.ticket).collect::<Support>();
        if ids.len() != self.frames.len() {
            return Err("duplicate frontier ticket".into());
        }
        self.kernel.validate(&self.frames)
    }
    pub fn pending_alternatives(&self) -> usize {
        self.frames.len()
    }
}
fn compatible(a: &kernel::Operation, b: &kernel::Operation, stats: &mut Stats) -> bool {
    use kernel::{Kind, Operation};
    fn term(a: &chr_syntax::Term, b: &chr_syntax::Term, stats: &mut Stats) -> bool {
        use chr_syntax::Term;
        stats.term_visits += 1;
        match (a, b) {
            (Term::Var(a), Term::Var(b)) => a == b,
            (Term::App(a, x), Term::App(b, y)) => {
                a == b && x.len() == y.len() && x.iter().zip(y).all(|(a, b)| term(a, b, stats))
            }
            _ => false,
        }
    }
    match (a, b) {
        (Operation::Pending(a), Operation::Pending(b)) => {
            a.id == b.id && matches!(a.kind, Kind::Insert(_))
        }
        (
            Operation::Apply {
                rule: a,
                ids: x,
                scope: s,
            },
            Operation::Apply {
                rule: b,
                ids: y,
                scope: t,
            },
        ) => {
            a == b
                && x == y
                && s.len() == t.len()
                && s.iter()
                    .zip(t)
                    .all(|((a, x), (b, y))| a == b && term(x, y, stats))
        }
        _ => false,
    }
}
