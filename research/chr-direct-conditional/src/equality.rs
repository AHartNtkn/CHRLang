//! Supported finite-tree equality. Jobs own unfinished equation obligations.
//!
//! Dropping an unfinished writer cancels the query unless its original equation
//! is restarted. Already committed bindings, failures, and change notifications
//! remain in Store. Global version validation conservatively rejects interleaved
//! mutation; it is not a support-local runtime progress protocol.
use crate::support::{Arena, Job, Operation, Status, Support};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT_STORE: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Term(usize);
pub enum TermView<'a> {
    Variable(usize),
    Constructor { name: &'a str, args: &'a [Term] },
}
enum Node {
    Variable(usize),
    Constructor { name: String, args: Vec<Term> },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Binding {
    pub support: Support,
    pub term: Term,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Change {
    pub variable: usize,
    pub support: Support,
}
pub struct Store {
    identity: u64,
    nodes: Vec<Node>,
    constructors: BTreeMap<(String, Vec<Term>), Term>,
    bindings: Vec<Vec<Binding>>,
    failed: Support,
    changes: Vec<Change>,
    version: u64,
}
impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
impl Store {
    pub fn new() -> Self {
        Self {
            identity: NEXT_STORE
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
                .expect("equality store identity capacity exhausted"),
            nodes: vec![],
            constructors: BTreeMap::new(),
            bindings: vec![],
            failed: Support::FALSE,
            changes: vec![],
            version: 0,
        }
    }
    pub fn fresh_variable(&mut self) -> Term {
        let term = Term(self.nodes.len());
        let variable = self.bindings.len();
        self.bindings.push(vec![]);
        self.nodes.push(Node::Variable(variable));
        term
    }
    pub fn constructor(&mut self, name: impl Into<String>, args: Vec<Term>) -> Term {
        for arg in &args {
            self.inspect(*arg);
        }
        let key = (name.into(), args);
        if let Some(term) = self.constructors.get(&key) {
            return *term;
        }
        let term = Term(self.nodes.len());
        self.nodes.push(Node::Constructor {
            name: key.0.clone(),
            args: key.1.clone(),
        });
        self.constructors.insert(key, term);
        term
    }
    pub fn inspect(&self, term: Term) -> TermView<'_> {
        match self.nodes.get(term.0).expect("term handle outside store") {
            Node::Variable(v) => TermView::Variable(*v),
            Node::Constructor { name, args } => TermView::Constructor { name, args },
        }
    }
    pub fn bindings(&self, variable: usize) -> &[Binding] {
        &self.bindings[variable]
    }
    pub fn failed(&self) -> Support {
        self.failed
    }
    pub fn changes(&self) -> &[Change] {
        &self.changes
    }
    pub fn identity(&self) -> u64 {
        self.identity
    }
    pub fn version(&self) -> u64 {
        self.version
    }
    fn changed(&mut self) {
        self.version = self
            .version
            .checked_add(1)
            .expect("equality version capacity exhausted");
    }
    fn bind(&mut self, variable: usize, support: Support, term: Term) {
        if support == Support::FALSE {
            return;
        }
        self.bindings[variable].push(Binding { support, term });
        self.changes.push(Change { variable, support });
        self.changed();
    }
    pub fn unify(&self, region: Support, left: Term, right: Term) -> UnifyJob {
        self.inspect(left);
        self.inspect(right);
        UnifyJob {
            request: (region, left, right),
            identity: self.identity,
            version: self.version,
            walker: Walker::new(region, left, right, false),
            effect: None,
            result: None,
        }
    }
    pub fn entails(&self, region: Support, left: Term, right: Term) -> DemandJob {
        self.inspect(left);
        self.inspect(right);
        DemandJob {
            request: (region, left, right),
            identity: self.identity,
            version: self.version,
            walker: Walker::new(region, left, right, true),
            possible: region,
            wait: None,
            result: None,
            initialized: false,
        }
    }
}
#[derive(Clone, Copy)]
struct Pair {
    region: Support,
    left: Term,
    right: Term,
}
#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}
#[derive(Clone, Copy)]
struct Resolve {
    pair: Pair,
    side: Side,
    index: usize,
}
enum WalkTask {
    Pair(Pair),
    Resolve(Resolve),
    Children { pair: Pair, index: usize },
}
enum WalkAction {
    Filtered(Pair),
    Overlap {
        resolve: Resolve,
        binding: Binding,
    },
    Split {
        resolve: Resolve,
        binding: Binding,
        overlap: Support,
    },
}
struct WalkWait {
    job: Job,
    action: WalkAction,
}
enum WalkEvent {
    Pending,
    Ready(Pair),
    Done,
}
struct Walker {
    tasks: Vec<WalkTask>,
    wait: Option<WalkWait>,
    track: bool,
    region: Support,
    touched: BTreeSet<usize>,
    dependencies: Vec<Change>,
}
impl Walker {
    fn new(region: Support, left: Term, right: Term, track: bool) -> Self {
        Self {
            tasks: vec![WalkTask::Pair(Pair {
                region,
                left,
                right,
            })],
            wait: None,
            track,
            region,
            touched: BTreeSet::new(),
            dependencies: vec![],
        }
    }
    fn touch(&mut self, variable: usize) {
        if self.track && self.touched.insert(variable) {
            self.dependencies.push(Change {
                variable,
                support: self.region,
            });
        }
    }
    fn children(&mut self, pair: Pair) {
        self.tasks.push(WalkTask::Children { pair, index: 0 });
    }
    fn tick(
        &mut self,
        store: &Store,
        arena: &mut Arena,
        allowed: Support,
        exclude: bool,
    ) -> WalkEvent {
        if let Some(mut wait) = self.wait.take() {
            let result = match wait.job.tick(arena) {
                Status::Pending => {
                    self.wait = Some(wait);
                    return WalkEvent::Pending;
                }
                Status::Complete(result) => result,
            };
            match wait.action {
                WalkAction::Filtered(mut pair) => {
                    pair.region = result;
                    if result != Support::FALSE {
                        if pair.left == pair.right {
                            if let TermView::Variable(v) = store.inspect(pair.left) {
                                self.touch(v);
                            }
                            return WalkEvent::Ready(pair);
                        }
                        self.tasks.push(WalkTask::Resolve(Resolve {
                            pair,
                            side: Side::Left,
                            index: 0,
                        }));
                    }
                }
                WalkAction::Overlap { resolve, binding } => {
                    self.wait = Some(WalkWait {
                        job: arena.job(Operation::Difference(resolve.pair.region, binding.support)),
                        action: WalkAction::Split {
                            resolve,
                            binding,
                            overlap: result,
                        },
                    });
                }
                WalkAction::Split {
                    mut resolve,
                    binding,
                    overlap,
                } => {
                    let mut pair = resolve.pair;
                    if result != Support::FALSE {
                        resolve.pair.region = result;
                        resolve.index += 1;
                        self.tasks.push(WalkTask::Resolve(resolve));
                    }
                    if overlap != Support::FALSE {
                        pair.region = overlap;
                        match resolve.side {
                            Side::Left => pair.left = binding.term,
                            Side::Right => pair.right = binding.term,
                        }
                        self.tasks.push(WalkTask::Pair(pair));
                    }
                }
            }
            return WalkEvent::Pending;
        }
        let Some(task) = self.tasks.pop() else {
            return WalkEvent::Done;
        };
        match task {
            WalkTask::Pair(pair) => {
                let operation = if exclude {
                    Operation::Difference(pair.region, allowed)
                } else {
                    Operation::And(pair.region, allowed)
                };
                self.wait = Some(WalkWait {
                    job: arena.job(operation),
                    action: WalkAction::Filtered(pair),
                });
            }
            WalkTask::Resolve(resolve) => {
                let term = match resolve.side {
                    Side::Left => resolve.pair.left,
                    Side::Right => resolve.pair.right,
                };
                if let TermView::Variable(v) = store.inspect(term) {
                    self.touch(v);
                    if let Some(binding) = store.bindings(v).get(resolve.index).copied() {
                        self.wait = Some(WalkWait {
                            job: arena.job(Operation::And(resolve.pair.region, binding.support)),
                            action: WalkAction::Overlap { resolve, binding },
                        });
                        return WalkEvent::Pending;
                    }
                }
                match resolve.side {
                    Side::Left => self.tasks.push(WalkTask::Resolve(Resolve {
                        pair: resolve.pair,
                        side: Side::Right,
                        index: 0,
                    })),
                    Side::Right => return WalkEvent::Ready(resolve.pair),
                }
            }
            WalkTask::Children { pair, index } => {
                let (
                    TermView::Constructor { args: left, .. },
                    TermView::Constructor { args: right, .. },
                ) = (store.inspect(pair.left), store.inspect(pair.right))
                else {
                    unreachable!("constructor child continuation");
                };
                if index < left.len() {
                    self.tasks.push(WalkTask::Children {
                        pair,
                        index: index + 1,
                    });
                    self.tasks.push(WalkTask::Pair(Pair {
                        region: pair.region,
                        left: left[index],
                        right: right[index],
                    }));
                }
            }
        }
        WalkEvent::Pending
    }
}

#[derive(Clone, Copy)]
struct Visit {
    region: Support,
    term: Term,
}
enum OccTask {
    Visit(Visit),
    Children {
        visit: Visit,
        index: usize,
    },
    Bindings {
        visit: Visit,
        variable: usize,
        index: usize,
    },
}
enum OccAction {
    Delta(Visit),
    Seen(Visit),
    Cycle,
    Binding { term: Term },
}
struct OccWait {
    job: Job,
    action: OccAction,
}
struct Occurs {
    variable: usize,
    tasks: Vec<OccTask>,
    wait: Option<OccWait>,
    seen: BTreeMap<Term, Support>,
    cycles: Support,
}
impl Occurs {
    fn new(variable: usize, region: Support, term: Term) -> Self {
        Self {
            variable,
            tasks: vec![OccTask::Visit(Visit { region, term })],
            wait: None,
            seen: BTreeMap::new(),
            cycles: Support::FALSE,
        }
    }
    fn tick(&mut self, store: &Store, arena: &mut Arena) -> Option<Support> {
        if let Some(mut wait) = self.wait.take() {
            let result = match wait.job.tick(arena) {
                Status::Pending => {
                    self.wait = Some(wait);
                    return None;
                }
                Status::Complete(result) => result,
            };
            match wait.action {
                OccAction::Delta(mut visit) => {
                    visit.region = result;
                    if result != Support::FALSE {
                        let prior = self
                            .seen
                            .get(&visit.term)
                            .copied()
                            .unwrap_or(Support::FALSE);
                        self.wait = Some(OccWait {
                            job: arena.job(Operation::Or(prior, result)),
                            action: OccAction::Seen(visit),
                        });
                    }
                }
                OccAction::Seen(visit) => {
                    self.seen.insert(visit.term, result);
                    match store.inspect(visit.term) {
                        TermView::Variable(v) if v == self.variable => {
                            self.wait = Some(OccWait {
                                job: arena.job(Operation::Or(self.cycles, visit.region)),
                                action: OccAction::Cycle,
                            })
                        }
                        TermView::Variable(v) => self.tasks.push(OccTask::Bindings {
                            visit,
                            variable: v,
                            index: 0,
                        }),
                        TermView::Constructor { .. } => {
                            self.tasks.push(OccTask::Children { visit, index: 0 })
                        }
                    }
                }
                OccAction::Cycle => self.cycles = result,
                OccAction::Binding { term } => {
                    if result != Support::FALSE {
                        self.tasks.push(OccTask::Visit(Visit {
                            term,
                            region: result,
                        }));
                    }
                }
            }
            return None;
        }
        let Some(task) = self.tasks.pop() else {
            return Some(self.cycles);
        };
        match task {
            OccTask::Visit(visit) => {
                let prior = self
                    .seen
                    .get(&visit.term)
                    .copied()
                    .unwrap_or(Support::FALSE);
                self.wait = Some(OccWait {
                    job: arena.job(Operation::Difference(visit.region, prior)),
                    action: OccAction::Delta(visit),
                });
            }
            OccTask::Children { visit, index } => {
                let TermView::Constructor { args, .. } = store.inspect(visit.term) else {
                    unreachable!()
                };
                if let Some(child) = args.get(index) {
                    self.tasks.push(OccTask::Children {
                        visit,
                        index: index + 1,
                    });
                    self.tasks.push(OccTask::Visit(Visit {
                        region: visit.region,
                        term: *child,
                    }));
                }
            }
            OccTask::Bindings {
                visit,
                variable,
                index,
            } => {
                if let Some(binding) = store.bindings(variable).get(index) {
                    self.tasks.push(OccTask::Bindings {
                        visit,
                        variable,
                        index: index + 1,
                    });
                    self.wait = Some(OccWait {
                        job: arena.job(Operation::And(visit.region, binding.support)),
                        action: OccAction::Binding { term: binding.term },
                    });
                }
            }
        }
        None
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnifyStatus {
    Pending,
    Complete { failed: Support },
    Stale,
}
enum Effect {
    Occurs {
        job: Occurs,
        variable: usize,
        term: Term,
        region: Support,
    },
    Failure {
        job: Job,
        then: Option<(usize, Term, Support, Support)>,
    },
    Safe {
        job: Job,
        variable: usize,
        term: Term,
    },
}
pub struct UnifyJob {
    identity: u64,
    request: (Support, Term, Term),
    version: u64,
    walker: Walker,
    effect: Option<Effect>,
    result: Option<Support>,
}
impl UnifyJob {
    pub fn request(&self) -> (Support, Term, Term) {
        self.request
    }
    pub fn tick(&mut self, store: &mut Store, arena: &mut Arena) -> UnifyStatus {
        if let Some(failed) = self.result {
            return UnifyStatus::Complete { failed };
        }
        if self.version != store.version || self.identity != store.identity {
            return UnifyStatus::Stale;
        }
        if let Some(effect) = self.effect.take() {
            match effect {
                Effect::Occurs {
                    mut job,
                    variable,
                    term,
                    region,
                } => {
                    if let Some(cycles) = job.tick(store, arena) {
                        self.effect = Some(Effect::Failure {
                            job: arena.job(Operation::Or(store.failed, cycles)),
                            then: Some((variable, term, region, cycles)),
                        });
                    } else {
                        self.effect = Some(Effect::Occurs {
                            job,
                            variable,
                            term,
                            region,
                        });
                    }
                }
                Effect::Failure { mut job, then } => match job.tick(arena) {
                    Status::Pending => self.effect = Some(Effect::Failure { job, then }),
                    Status::Complete(failed) => {
                        if store.failed != failed {
                            store.failed = failed;
                            store.changed();
                            self.version = store.version;
                        }
                        if let Some((variable, term, region, cycles)) = then {
                            self.effect = Some(Effect::Safe {
                                job: arena.job(Operation::Difference(region, cycles)),
                                variable,
                                term,
                            });
                        }
                    }
                },
                Effect::Safe {
                    mut job,
                    variable,
                    term,
                } => match job.tick(arena) {
                    Status::Pending => {
                        self.effect = Some(Effect::Safe {
                            job,
                            variable,
                            term,
                        })
                    }
                    Status::Complete(support) => {
                        store.bind(variable, support, term);
                        self.version = store.version;
                    }
                },
            }
            return UnifyStatus::Pending;
        }
        match self.walker.tick(store, arena, store.failed, true) {
            WalkEvent::Pending => (),
            WalkEvent::Done => {
                self.result = Some(store.failed);
                return UnifyStatus::Complete {
                    failed: store.failed,
                };
            }
            WalkEvent::Ready(pair) => {
                if pair.left == pair.right {
                    return UnifyStatus::Pending;
                }
                match (store.inspect(pair.left), store.inspect(pair.right)) {
                    (TermView::Variable(a), TermView::Variable(b)) => {
                        let (variable, term) = if a > b {
                            (a, pair.right)
                        } else {
                            (b, pair.left)
                        };
                        store.bind(variable, pair.region, term);
                        self.version = store.version;
                    }
                    (TermView::Variable(variable), _) => {
                        self.effect = Some(Effect::Occurs {
                            job: Occurs::new(variable, pair.region, pair.right),
                            variable,
                            term: pair.right,
                            region: pair.region,
                        })
                    }
                    (_, TermView::Variable(variable)) => {
                        self.effect = Some(Effect::Occurs {
                            job: Occurs::new(variable, pair.region, pair.left),
                            variable,
                            term: pair.left,
                            region: pair.region,
                        })
                    }
                    (
                        TermView::Constructor { name: a, args: x },
                        TermView::Constructor { name: b, args: y },
                    ) => {
                        if a != b || x.len() != y.len() {
                            self.effect = Some(Effect::Failure {
                                job: arena.job(Operation::Or(store.failed, pair.region)),
                                then: None,
                            });
                        } else {
                            self.walker.children(pair);
                        }
                    }
                }
            }
        }
        UnifyStatus::Pending
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DemandStatus {
    Pending,
    Complete { entailed: Support },
    Stale,
}
pub struct DemandJob {
    identity: u64,
    request: (Support, Term, Term),
    version: u64,
    walker: Walker,
    possible: Support,
    wait: Option<Job>,
    result: Option<Support>,
    initialized: bool,
}
impl DemandJob {
    pub fn request(&self) -> (Support, Term, Term) {
        self.request
    }
    pub fn dependencies(&self) -> &[Change] {
        &self.walker.dependencies
    }
    pub fn tick(&mut self, store: &Store, arena: &mut Arena) -> DemandStatus {
        if let Some(entailed) = self.result {
            return DemandStatus::Complete { entailed };
        }
        if self.version != store.version || self.identity != store.identity {
            return DemandStatus::Stale;
        }
        if !self.initialized {
            self.initialized = true;
            self.wait = Some(arena.job(Operation::Difference(self.possible, store.failed)));
            return DemandStatus::Pending;
        }
        if let Some(mut job) = self.wait.take() {
            match job.tick(arena) {
                Status::Pending => self.wait = Some(job),
                Status::Complete(result) => self.possible = result,
            }
            return DemandStatus::Pending;
        }
        match self.walker.tick(store, arena, self.possible, false) {
            WalkEvent::Pending => (),
            WalkEvent::Done => {
                self.result = Some(self.possible);
                return DemandStatus::Complete {
                    entailed: self.possible,
                };
            }
            WalkEvent::Ready(pair) => {
                if pair.left == pair.right {
                    return DemandStatus::Pending;
                }
                match (store.inspect(pair.left), store.inspect(pair.right)) {
                    (
                        TermView::Constructor { name: a, args: x },
                        TermView::Constructor { name: b, args: y },
                    ) if a == b && x.len() == y.len() => self.walker.children(pair),
                    _ => {
                        self.wait =
                            Some(arena.job(Operation::Difference(self.possible, pair.region)))
                    }
                }
            }
        }
        DemandStatus::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deep_occurs_check_and_demand_are_resumable_and_nonbinding() {
        let mut arena = Arena::new();
        let mut store = Store::new();
        let x = store.fresh_variable();
        let atom = store.constructor("a", vec![]);
        let mut nested_x = x;
        let mut nested_a = atom;
        for _ in 0..64 {
            nested_x = store.constructor("f", vec![nested_x]);
            nested_a = store.constructor("f", vec![nested_a]);
        }
        let mut demand = store.entails(Support::TRUE, nested_x, nested_a);
        let mut ticks = 0;
        loop {
            ticks += 1;
            assert!(ticks < 10_000);
            match demand.tick(&store, &mut arena) {
                DemandStatus::Pending => (),
                DemandStatus::Complete { entailed } => {
                    assert_eq!(entailed, Support::FALSE);
                    break;
                }
                DemandStatus::Stale => panic!("read-only serial demand became stale"),
            }
        }
        assert!(ticks > 64);
        assert!(store.bindings(0).is_empty());
        assert_eq!(
            demand.dependencies(),
            &[Change {
                variable: 0,
                support: Support::TRUE
            }]
        );
        let mut equality = store.unify(Support::TRUE, x, nested_x);
        let mut ticks = 0;
        loop {
            ticks += 1;
            assert!(ticks < 10_000);
            match equality.tick(&mut store, &mut arena) {
                UnifyStatus::Pending => (),
                UnifyStatus::Complete { failed } => {
                    assert_eq!(failed, Support::TRUE);
                    break;
                }
                UnifyStatus::Stale => panic!("serial occurs check became stale"),
            }
        }
        assert!(ticks > 64);
        assert!(store.bindings(0).is_empty());
        assert!(store.changes().is_empty());
    }
}
