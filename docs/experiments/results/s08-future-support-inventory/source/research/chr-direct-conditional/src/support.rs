//! Canonical ordered Boolean supports with resumable Boolean operations.
use std::collections::BTreeMap;

/// Handles are scoped to their originating arena. Both terminal handles are universal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Support(usize);
impl Support {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
    pub fn index(self) -> usize {
        self.0
    }
}
pub const FALSE: Support = Support::FALSE;
pub const TRUE: Support = Support::TRUE;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeView {
    False,
    True,
    Branch {
        variable: usize,
        low: Support,
        high: Support,
    },
}
struct Node {
    view: NodeView,
    max_variable: Option<usize>,
}
pub struct Arena {
    nodes: Vec<Node>,
    unique: BTreeMap<(usize, Support, Support), Support>,
    variables: usize,
}
impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}
impl Arena {
    pub fn new() -> Self {
        Self {
            nodes: vec![
                Node {
                    view: NodeView::False,
                    max_variable: None,
                },
                Node {
                    view: NodeView::True,
                    max_variable: None,
                },
            ],
            unique: BTreeMap::new(),
            variables: 0,
        }
    }
    pub fn fresh_variable(&mut self) -> (usize, Support) {
        let variable = self.variables;
        self.variables = self
            .variables
            .checked_add(1)
            .expect("support variable capacity exhausted");
        (variable, self.mk(variable, FALSE, TRUE))
    }
    /// Construct an ordered node. The variable must exist and each nonterminal
    /// child must test a strictly later variable. All handles belong to this arena.
    pub fn mk(&mut self, variable: usize, low: Support, high: Support) -> Support {
        assert!(
            variable < self.variables,
            "support variable has not been allocated"
        );
        for child in [low, high] {
            if let NodeView::Branch {
                variable: child_variable,
                ..
            } = self.inspect(child)
            {
                assert!(
                    child_variable > variable,
                    "support children violate variable ordering"
                );
            }
        }
        if low == high {
            return low;
        }
        let key = (variable, low, high);
        if let Some(handle) = self.unique.get(&key) {
            return *handle;
        }
        let max_variable = Some(variable)
            .max(self.max_variable(low))
            .max(self.max_variable(high));
        let result = Support(self.nodes.len());
        self.nodes.push(Node {
            view: NodeView::Branch {
                variable,
                low,
                high,
            },
            max_variable,
        });
        self.unique.insert(key, result);
        result
    }
    pub fn inspect(&self, handle: Support) -> NodeView {
        self.nodes
            .get(handle.0)
            .expect("support handle is outside arena bounds")
            .view
    }
    pub fn max_variable(&self, handle: Support) -> Option<usize> {
        self.nodes
            .get(handle.0)
            .expect("support handle is outside arena bounds")
            .max_variable
    }
    /// Includes the two terminal nodes. This is retained structure, not a work counter.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    pub fn variable_count(&self) -> usize {
        self.variables
    }
    /// Diagnostic evaluation. Production observers can use inspect for one-node service.
    pub fn eval(&self, mut handle: Support, assignment: &[bool]) -> bool {
        loop {
            match self.inspect(handle) {
                NodeView::False => return false,
                NodeView::True => return true,
                NodeView::Branch {
                    variable,
                    low,
                    high,
                } => {
                    handle = if *assignment
                        .get(variable)
                        .expect("support assignment omits a dependency")
                    {
                        high
                    } else {
                        low
                    };
                }
            }
        }
    }
    pub fn job(&self, operation: Operation) -> Job {
        let (kind, a, b) = match operation {
            Operation::Not(a) => (Kind::Not, a, FALSE),
            Operation::And(a, b) => (Kind::And, a, b),
            Operation::Or(a, b) => (Kind::Or, a, b),
            Operation::Difference(a, b) => (Kind::Difference, a, b),
        };
        self.inspect(a);
        self.inspect(b);
        let root = kind.key(a, b);
        Job {
            kind,
            root,
            frames: vec![Frame::Evaluate(root)],
            memo: BTreeMap::new(),
            result: None,
            work: Work::default(),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Not(Support),
    And(Support, Support),
    Or(Support, Support),
    Difference(Support, Support),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Pending,
    Complete(Support),
}
/// Diagnostic counters are zero when the metrics feature is disabled.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    pub ticks: u64,
    pub expanded: u64,
    pub memo_hits: u64,
    pub created_nodes: u64,
}
#[derive(Clone, Copy)]
enum Kind {
    Not,
    And,
    Or,
    Difference,
}
type Key = (Support, Support);
impl Kind {
    fn key(self, a: Support, b: Support) -> Key {
        if matches!(self, Self::And | Self::Or) && a > b {
            (b, a)
        } else {
            (a, b)
        }
    }
    fn simple(self, (a, b): Key) -> Option<Support> {
        match self {
            Self::Not if a == FALSE => Some(TRUE),
            Self::Not if a == TRUE => Some(FALSE),
            Self::And if a == FALSE || b == FALSE => Some(FALSE),
            Self::And if a == TRUE => Some(b),
            Self::And if b == TRUE || a == b => Some(a),
            Self::Or if a == TRUE || b == TRUE => Some(TRUE),
            Self::Or if a == FALSE => Some(b),
            Self::Or if b == FALSE || a == b => Some(a),
            Self::Difference if a == FALSE || b == TRUE || a == b => Some(FALSE),
            Self::Difference if b == FALSE => Some(a),
            _ => None,
        }
    }
}
enum Frame {
    Evaluate(Key),
    Join {
        key: Key,
        variable: usize,
        low: Key,
        high: Key,
    },
}
/// An apply job owns its continuation and memo table. Interleaved jobs share only
/// immutable nodes and the arena's canonical node table, never projected worlds.
pub struct Job {
    kind: Kind,
    root: Key,
    frames: Vec<Frame>,
    memo: BTreeMap<Key, Support>,
    result: Option<Support>,
    work: Work,
}
impl Job {
    pub fn work(&self) -> Work {
        self.work
    }
    /// Pop exactly one explicit frame, creating at most three frames and one node.
    /// Map lookups and allocation retain their usual size-dependent cost; this is
    /// an algorithmic service boundary, not a hard real-time latency guarantee.
    pub fn tick(&mut self, arena: &mut Arena) -> Status {
        if let Some(result) = self.result {
            return Status::Complete(result);
        }
        #[cfg(feature = "metrics")]
        {
            self.work.ticks += 1;
        }
        match self
            .frames
            .pop()
            .expect("unfinished support job has a continuation")
        {
            Frame::Evaluate(key) => {
                if self.memo.contains_key(&key) {
                    #[cfg(feature = "metrics")]
                    {
                        self.work.memo_hits += 1;
                    }
                } else if let Some(result) = self.kind.simple(key) {
                    self.memo.insert(key, result);
                } else {
                    fn variable(view: NodeView) -> Option<usize> {
                        match view {
                            NodeView::Branch { variable, .. } => Some(variable),
                            _ => None,
                        }
                    }
                    fn cofactors(
                        handle: Support,
                        view: NodeView,
                        top: usize,
                    ) -> (Support, Support) {
                        match view {
                            NodeView::Branch {
                                variable,
                                low,
                                high,
                            } if variable == top => (low, high),
                            _ => (handle, handle),
                        }
                    }
                    let (a, b) = (arena.inspect(key.0), arena.inspect(key.1));
                    let top = match (variable(a), variable(b)) {
                        (Some(a), Some(b)) => a.min(b),
                        (Some(a), None) => a,
                        (None, Some(b)) => b,
                        (None, None) => {
                            unreachable!("terminal truth table handled by simplification")
                        }
                    };
                    let (al, ah) = cofactors(key.0, a, top);
                    let (bl, bh) = cofactors(key.1, b, top);
                    let low = self.kind.key(al, bl);
                    let high = self.kind.key(ah, bh);
                    self.frames.push(Frame::Join {
                        key,
                        variable: top,
                        low,
                        high,
                    });
                    self.frames.push(Frame::Evaluate(high));
                    self.frames.push(Frame::Evaluate(low));
                    #[cfg(feature = "metrics")]
                    {
                        self.work.expanded += 1;
                    }
                }
            }
            Frame::Join {
                key,
                variable,
                low,
                high,
            } => {
                #[cfg(feature = "metrics")]
                let before = arena.node_count();
                let result = arena.mk(variable, self.memo[&low], self.memo[&high]);
                #[cfg(feature = "metrics")]
                {
                    self.work.created_nodes += (arena.node_count() - before) as u64;
                }
                self.memo.insert(key, result);
            }
        }
        if self.frames.is_empty() {
            let result = self.memo[&self.root];
            self.result = Some(result);
            Status::Complete(result)
        } else {
            Status::Pending
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn finish(arena: &mut Arena, operation: Operation) -> Support {
        let mut job = arena.job(operation);
        for _ in 0..10_000 {
            if let Status::Complete(result) = job.tick(arena) {
                return result;
            }
        }
        panic!("finite apply did not complete");
    }
    #[test]
    fn canonical_reduction_and_complement_have_boolean_meaning() {
        let mut arena = Arena::new();
        let (x_id, x) = arena.fresh_variable();
        let (y_id, y) = arena.fresh_variable();
        assert_eq!(arena.mk(x_id, FALSE, TRUE), x);
        assert_eq!(arena.mk(x_id, y, y), y);
        let nx = finish(&mut arena, Operation::Not(x));
        assert_eq!(finish(&mut arena, Operation::And(x, nx)), FALSE);
        assert_eq!(finish(&mut arena, Operation::Or(x, nx)), TRUE);
        let ny = finish(&mut arena, Operation::Not(y));
        assert_eq!(
            finish(&mut arena, Operation::Difference(x, y)),
            finish(&mut arena, Operation::And(x, ny))
        );
        assert_eq!(arena.max_variable(nx), Some(x_id));
        let both = finish(&mut arena, Operation::And(x, y));
        assert_eq!(arena.max_variable(both), Some(y_id));
        assert!(!arena.eval(both, &[true, false]));
        assert!(arena.eval(both, &[true, true]));
    }
    #[test]
    fn independent_jobs_interleave_and_finished_jobs_do_no_more_work() {
        let mut arena = Arena::new();
        let (_, x) = arena.fresh_variable();
        let (_, y) = arena.fresh_variable();
        let mut conjunction = arena.job(Operation::And(x, y));
        let mut disjunction = arena.job(Operation::Or(x, y));
        assert!(matches!(conjunction.tick(&mut arena), Status::Pending));
        assert!(matches!(disjunction.tick(&mut arena), Status::Pending));
        let mut and = None;
        let mut or = None;
        for _ in 0..100 {
            if let Status::Complete(value) = conjunction.tick(&mut arena) {
                and = Some(value);
            }
            if let Status::Complete(value) = disjunction.tick(&mut arena) {
                or = Some(value);
            }
            if and.is_some() && or.is_some() {
                break;
            }
        }
        let (and, or) = (and.unwrap(), or.unwrap());
        assert!(!arena.eval(and, &[false, true]));
        assert!(arena.eval(or, &[false, true]));
        let before = conjunction.work();
        assert_eq!(conjunction.tick(&mut arena), Status::Complete(and));
        assert_eq!(conjunction.work(), before);
    }
    #[test]
    fn long_negation_yields_with_at_most_one_node_allocation_per_tick() {
        let mut arena = Arena::new();
        for _ in 0..64 {
            arena.fresh_variable();
        }
        let input = (0..64)
            .rev()
            .fold(TRUE, |tail, variable| arena.mk(variable, FALSE, tail));
        let mut job = arena.job(Operation::Not(input));
        let mut ticks = 0;
        let result = loop {
            let before = arena.node_count();
            let status = job.tick(&mut arena);
            ticks += 1;
            assert!(arena.node_count() - before <= 1);
            assert!(ticks < 1000);
            if let Status::Complete(result) = status {
                break result;
            }
        };
        assert!(ticks > 64, "the nontrivial traversal must remain resumable");
        assert!(!arena.eval(result, &[true; 64]));
        assert!(arena.eval(result, &[false; 64]));
        assert_eq!(arena.max_variable(result), Some(63));
        if cfg!(feature = "metrics") {
            assert_eq!(job.work().ticks, ticks);
        } else {
            assert_eq!(job.work(), Work::default());
        }
    }
    #[test]
    #[should_panic(expected = "variable ordering")]
    fn construction_rejects_an_earlier_child_variable() {
        let mut arena = Arena::new();
        let (_, x) = arena.fresh_variable();
        let (y, _) = arena.fresh_variable();
        arena.mk(y, FALSE, x);
    }
    #[test]
    #[should_panic(expected = "outside arena bounds")]
    fn construction_rejects_out_of_bounds_handles() {
        let mut arena = Arena::new();
        let (x, _) = arena.fresh_variable();
        arena.mk(x, FALSE, Support(usize::MAX));
    }
}
