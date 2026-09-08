//! Independent scalar controls with immutable terms and explicit state snapshot policies.
/// Availability of shared term/map diagnostics; legacy Search counters are separate.
pub const COLLECT_KERNEL_METRICS: bool = cfg!(feature = "kernel-metrics");
mod map;
mod state;
mod terms;
use chr_syntax::{Answer, Query, Rule};
pub use map::Storage;
#[derive(Clone, Copy, Debug)]
pub enum Snapshot {
    Persistent,
    Copy,
}
#[derive(Default, Debug)]
pub struct Stats {
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
    pub storage: Storage,
}
/// Contribution already included in historical eager `Stats` totals. Export
/// affects only dereferences and storage visits; all other source fields are unchanged.
#[derive(Default, Debug, Clone, Copy)]
pub struct EagerExportStats {
    pub answers: u64,
    pub dereferences: u64,
    pub storage_visits: u64,
}
pub struct Batch {
    pub answers: Vec<Answer>,
    pub exhausted: bool,
}
pub struct Search {
    rules: Vec<Rule>,
    arena: terms::Arena,
    frontier: std::collections::VecDeque<state::State>,
    seen: chr_observe::AnswerSet,
    mode: Snapshot,
    stats: Stats,
    eager_export: EagerExportStats,
}
impl Search {
    pub fn new(rules: Vec<Rule>, query: Query, mode: Snapshot) -> Result<Self, String> {
        let mut names = std::collections::BTreeSet::new();
        for r in &rules {
            if r.kept.is_empty() && r.removed.is_empty() {
                return Err("empty rule heads".into());
            }
            if !names.insert(&r.name) {
                return Err("duplicate rule name".into());
            }
        }
        let mut names = std::collections::BTreeSet::new();
        for (name, _) in &query.outputs {
            if !names.insert(name) {
                return Err("duplicate output name".into());
            }
        }
        let mut arena = terms::Arena::default();
        let mut stats = Stats {
            max_frontier: 1,
            ..Stats::default()
        };
        let initial = state::State::new(query, &mut arena, &mut stats);
        Ok(Self {
            rules,
            arena,
            frontier: std::collections::VecDeque::from([initial]),
            seen: chr_observe::AnswerSet::default(),
            mode,
            stats,
            eager_export: EagerExportStats::default(),
        })
    }
    pub fn advance(&mut self, budget: usize) -> Batch {
        let mut answers = vec![];
        for _ in 0..budget {
            let Some(mut branch) = self.frontier.pop_front() else {
                break;
            };
            self.stats.steps += 1;
            match branch.step(&self.rules, &mut self.arena, self.mode, &mut self.stats) {
                state::Event::Continue => self.frontier.push_back(branch),
                state::Event::Split(sibling) => {
                    self.stats.splits += 1;
                    self.frontier.push_back(branch);
                    self.frontier.push_back(*sibling);
                }
                state::Event::Failed => self.stats.failed += 1,
                state::Event::Complete => {
                    let answer =
                        branch.export_answer(&self.arena, &mut self.stats, &mut self.eager_export);
                    self.stats.completed += 1;
                    if self.seen.insert(answer.clone()) {
                        answers.push(answer);
                    } else {
                        self.stats.duplicates += 1;
                    }
                }
            }
            self.stats.max_frontier = self.stats.max_frontier.max(self.frontier.len());
        }
        Batch {
            answers,
            exhausted: self.frontier.is_empty(),
        }
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn eager_export_stats(&self) -> &EagerExportStats {
        &self.eager_export
    }
    pub fn observation_stats(&self) -> &chr_observe::Stats {
        &self.seen.stats
    }
    pub fn pending_alternatives(&self) -> usize {
        self.frontier.len()
    }
}
pub mod continuations;

pub mod observation;

/// Candidate-only primitives shared by the integrated compilation experiment.
/// This exposes no reference interpreter implementation.
pub mod kernel {
    use crate::Stats;
    pub use crate::terms::{Arena, Bindings, Scope, Term, deref};

    impl Arena {
        /// Report variables changed by a successful transaction. Failure leaves both
        /// the caller's bindings and change report unchanged.
        pub fn unify_record(
            &self,
            left: Term,
            right: Term,
            bindings: &mut Bindings,
            stats: &mut Stats,
            changed: &mut Vec<u64>,
        ) -> bool {
            let mut transaction = Vec::new();
            if !self.unify_impl::<true>(left, right, bindings, stats, &mut transaction) {
                return false;
            }
            changed.extend(transaction);
            true
        }
    }
}
