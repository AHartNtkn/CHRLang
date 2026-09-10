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
    /// Freeze the birth prefix and enumerate only causal histories in `region`.
    /// The supplied support must mention only births already in this ledger.
    /// Later births may be appended without extending this cursor's prefix.
    /// The runtime must establish completion on this region before publication;
    /// restricting enumeration alone is not a completion certificate.
    pub fn histories(&self, region: Support, arena: &Arena) -> Histories {
        assert!(
            arena
                .max_variable(region)
                .is_none_or(|v| v < self.guards.len()),
            "history region mentions a variable outside the frozen birth prefix"
        );
        Histories {
            #[cfg(feature = "support-generic-histories")]
            restriction: None,
            limit: self.guards.len(),
            cofactor: region,
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
    before: Support,
}
enum Phase {
    Descend,
    Evaluate(Support),
    Restrict {
        before: Support,
        variable: usize,
        value: bool,
    },
    Copy(usize),
    Backtrack,
    Done,
}
pub struct Histories {
    #[cfg(feature = "support-generic-histories")]
    restriction: Option<Feasibility>,
    limit: usize,
    cofactor: Support,
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
                self.phase = if self.cofactor == Support::FALSE {
                    Phase::Backtrack
                } else if self.path.len() == self.limit {
                    debug_assert_eq!(self.cofactor, Support::TRUE);
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
                terminal @ (NodeView::False | NodeView::True) => {
                    let active = matches!(terminal, NodeView::True);
                    let variable = self.path.len();
                    self.path.push(false);
                    self.frames.push(Frame {
                        active,
                        second: false,
                        before: self.cofactor,
                    });
                    self.phase = Phase::Restrict {
                        before: self.cofactor,
                        variable,
                        value: false,
                    };
                }
            },
            Phase::Restrict {
                before,
                variable,
                value,
            } => {
                #[cfg(feature = "support-generic-histories")]
                {
                    // Check whether any diagram path agrees with the assigned
                    // chronological prefix. Future inactive births are checked
                    // when their own guards are reached, not assumed independent.
                    let job = self
                        .restriction
                        .get_or_insert_with(|| Feasibility::new(before));
                    if let Some(possible) = job.tick(arena, &self.path) {
                        self.cofactor = if !possible {
                            Support::FALSE
                        } else if self.path.len() == self.limit {
                            Support::TRUE
                        } else {
                            before
                        };
                        self.restriction = None;
                        self.phase = Phase::Descend;
                    }
                    let _ = (variable, value);
                }
                #[cfg(not(feature = "support-generic-histories"))]
                {
                    // Ordered supports need at most one node for this newly assigned
                    // variable: all earlier variables were restricted by ancestor frames.
                    self.cofactor = match arena.inspect(before) {
                        NodeView::Branch {
                            variable: next,
                            low,
                            high,
                        } => {
                            assert!(
                                next >= variable,
                                "support cofactor retained an assigned variable"
                            );
                            if next == variable {
                                if value { high } else { low }
                            } else {
                                before
                            }
                        }
                        NodeView::False | NodeView::True => before,
                    };
                    self.phase = Phase::Descend;
                }
            }
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
                        self.phase = Phase::Restrict {
                            before: frame.before,
                            variable: self.path.len() - 1,
                            value: true,
                        };
                    } else {
                        self.cofactor = frame.before;
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

/// Existential feasibility under a fixed assigned prefix. The visited set keeps
/// shared diagram paths from duplicating work; each tick examines at most one node.
#[cfg(feature = "support-generic-histories")]
struct Feasibility {
    pending: Vec<Support>,
    seen: std::collections::BTreeSet<Support>,
}
#[cfg(feature = "support-generic-histories")]
impl Feasibility {
    fn new(root: Support) -> Self {
        Self {
            pending: vec![root],
            seen: Default::default(),
        }
    }
    fn tick(&mut self, arena: &Arena, prefix: &[bool]) -> Option<bool> {
        let Some(node) = self.pending.pop() else {
            return Some(false);
        };
        if !self.seen.insert(node) {
            return None;
        }
        match arena.inspect(node) {
            NodeView::True => return Some(true),
            NodeView::False => (),
            NodeView::Branch {
                variable,
                low,
                high,
            } => {
                if let Some(value) = prefix.get(variable) {
                    self.pending.push(if *value { high } else { low });
                } else {
                    self.pending.push(high);
                    self.pending.push(low);
                }
            }
        }
        None
    }
}
