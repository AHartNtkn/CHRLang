//! Causal source choices. Enumeration is observation work, never matching.
use crate::support::{Arena, NodeView, Support};

#[derive(Default)]
pub struct Births {
    guards: Vec<Support>,
}
impl Births {
    pub fn new() -> Self {
        Self::default()
    }
    /// Own all Boolean variables in this query arena. A guard can depend only
    /// on earlier births. Equal source arms must still call this method.
    pub fn create(&mut self, arena: &mut Arena, active: Support) -> Support {
        assert_eq!(
            arena.variable_count(),
            self.guards.len(),
            "birth ledger owns source variables"
        );
        assert!(
            arena
                .max_variable(active)
                .is_none_or(|v| v < self.guards.len())
        );
        let (_, choice) = arena.fresh_variable();
        self.guards.push(active);
        choice
    }
    /// Freeze the current birth prefix. The runtime must establish completion
    /// on its observation support before using this cursor to publish answers.
    /// Prefix freezing alone is not a completion certificate.
    pub fn histories(&self) -> Histories {
        Histories {
            limit: self.guards.len(),
            path: vec![],
            frames: vec![],
            phase: Phase::Descend,
            output: vec![],
        }
    }
}
pub enum HistoryEvent {
    Progress,
    /// Inactive births have canonical value false and contribute no branching.
    History(Vec<bool>),
    Exhausted,
}
struct Frame {
    active: bool,
    second: bool,
}
enum Phase {
    Descend,
    Evaluate(Support),
    Copy(usize),
    Backtrack,
    Done,
}
pub struct Histories {
    limit: usize,
    path: Vec<bool>,
    frames: Vec<Frame>,
    phase: Phase,
    output: Vec<bool>,
}
impl Histories {
    /// One decision node, traversal frame or output bit per call. Vector/hash
    /// allocation costs are not a hard real-time guarantee.
    pub fn tick(&mut self, births: &Births, arena: &Arena) -> HistoryEvent {
        assert!(births.guards.len() >= self.limit);
        match self.phase {
            Phase::Done => return HistoryEvent::Exhausted,
            Phase::Descend => {
                self.phase = if self.path.len() == self.limit {
                    Phase::Copy(0)
                } else {
                    Phase::Evaluate(births.guards[self.path.len()])
                };
            }
            Phase::Evaluate(s) => match arena.inspect(s) {
                NodeView::Branch {
                    variable,
                    low,
                    high,
                } => {
                    assert!(variable < self.path.len(), "birth guard is not causal");
                    self.phase = Phase::Evaluate(if self.path[variable] { high } else { low });
                }
                NodeView::False | NodeView::True => {
                    let active = matches!(arena.inspect(s), NodeView::True);
                    self.path.push(false);
                    self.frames.push(Frame {
                        active,
                        second: false,
                    });
                    self.phase = Phase::Descend;
                }
            },
            Phase::Copy(index) => {
                if index < self.path.len() {
                    self.output.push(self.path[index]);
                    self.phase = Phase::Copy(index + 1);
                } else {
                    self.phase = Phase::Backtrack;
                    return HistoryEvent::History(std::mem::take(&mut self.output));
                }
            }
            Phase::Backtrack => {
                if let Some(frame) = self.frames.last_mut() {
                    if frame.active && !frame.second {
                        frame.second = true;
                        *self.path.last_mut().unwrap() = true;
                        self.phase = Phase::Descend;
                    } else {
                        self.frames.pop();
                        self.path.pop();
                    }
                } else {
                    self.phase = Phase::Done;
                }
            }
        }
        HistoryEvent::Progress
    }
}
