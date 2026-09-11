//! Resumable compiled source service for certified regional products.
use super::workers::Batch;
use chr_compiled::{
    Access, Policy, PreparedRuleset,
    search::{SearchEngine, SearchEvent},
};
use chr_syntax::Query;
use std::sync::atomic::{AtomicBool, Ordering};
pub struct Search {
    engine: SearchEngine,
    seen: chr_observe::AnswerSet,
    raw: u128,
}
impl Search {
    pub fn new(prepared: &PreparedRuleset, query: Query) -> Result<Self, String> {
        Ok(Self {
            engine: prepared.start_search(query, Policy::Global, Access::Scan)?,
            seen: Default::default(),
            raw: 0,
        })
    }
    pub fn service(
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
            match self.engine.tick() {
                SearchEvent::Complete(mut branch) => {
                    self.raw = self
                        .raw
                        .checked_add(1)
                        .expect("raw regional count overflow");
                    let answer = branch
                        .engine
                        .observe()
                        .expect("completed branch observation");
                    if self.seen.insert(answer.clone()) {
                        answers.push(answer);
                    }
                }
                SearchEvent::Exhausted => break,
                SearchEvent::Progress | SearchEvent::Split { .. } | SearchEvent::Failed(_) => {}
            }
        }
        Batch {
            request,
            region,
            answers,
            raw_completions: self.raw,
            exhausted: self.engine.pending_branches() == 0,
            cancelled,
        }
    }
}
