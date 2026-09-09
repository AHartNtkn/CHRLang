//! Certified delayed-split experiment; storage/equality services are shared with the E01 control.
const COLLECT_KERNEL_METRICS: bool = true;
#[path = "../../chr-persistent/src/map.rs"]
mod map;
mod state;
#[path = "../../chr-persistent/src/terms.rs"]
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
    pub certificate_checks: u64,
    pub certificate_terms: u64,
    pub barrier_nodes: u64,
    pub barrier_rejections: u64,
    pub lifting_checks: u64,
    pub lifted_applications: u64,
    pub forced_splits: u64,
    pub max_delay: u64,
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
pub struct Batch {
    pub answers: Vec<Answer>,
    pub exhausted: bool,
}
pub struct Search {
    quota: usize,
    safe_rules: Vec<bool>,
    rules: Vec<Rule>,
    arena: terms::Arena,
    frontier: std::collections::VecDeque<state::State>,
    seen: chr_observe::AnswerSet,
    mode: Snapshot,
    stats: Stats,
}
impl Search {
    pub fn new(
        rules: Vec<Rule>,
        query: Query,
        mode: Snapshot,
        quota: usize,
    ) -> Result<Self, String> {
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
        let safe_rules = state::certify(&rules, &mut stats);
        Ok(Self {
            quota,
            safe_rules,
            rules,
            arena,
            frontier: std::collections::VecDeque::from([initial]),
            seen: chr_observe::AnswerSet::default(),
            mode,
            stats,
        })
    }
    pub fn advance(&mut self, budget: usize) -> Batch {
        let mut answers = vec![];
        for _ in 0..budget {
            let Some(mut branch) = self.frontier.pop_front() else {
                break;
            };
            self.stats.steps += 1;
            match branch.step(
                &self.rules,
                &self.safe_rules,
                self.quota,
                &mut self.arena,
                self.mode,
                &mut self.stats,
            ) {
                state::Event::Continue => self.frontier.push_back(branch),
                state::Event::Split(sibling) => {
                    self.stats.splits += 1;
                    self.frontier.push_back(branch);
                    self.frontier.push_back(*sibling);
                }
                state::Event::Failed => self.stats.failed += 1,
                state::Event::Answer(answer) => {
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
    pub fn observation_stats(&self) -> &chr_observe::Stats {
        &self.seen.stats
    }
    pub fn pending_alternatives(&self) -> usize {
        self.frontier.len()
    }
}
