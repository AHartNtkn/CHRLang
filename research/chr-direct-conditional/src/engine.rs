//! Direct conditional source execution. No evaluator is run per birth history.
//!
//! A round fixes its candidate set. Each selected application removes its support
//! from the round before any body effect; all resulting invalidations go to the
//! next round. The no-application remainder is therefore a stable completion
//! certificate. A finite sibling is certified even when another support rewrites
//! forever. Serial body/equality ownership prevents global version restart loops.
//! Observation alone enumerates histories, using frozen finite cursor bounds.
use crate::births::{Births, Histories, HistoryEvent};
use crate::equality::{Change, FailureJob, Store, Term, TermView, UnifyJob, UnifyStatus};
use crate::resources::{
    AckStatus, ApplicationJob, ApplicationStatus, BodyAckJob, Resources, Token,
};
use crate::support::{Arena, Job, NodeView, Operation, Status, Support};
use chr_syntax::{Answer, Constraint, Goal, Query, Rule, Term as Source, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::Arc;
const METRICS: bool = cfg!(feature = "metrics");
#[derive(Default, Debug)]
pub struct Stats {
    pub ticks: u64,
    pub applications: u64,
    pub births: u64,
    pub answers: u64,
    pub discovered_tuples: u64,
    pub invalidations: u64,
}
#[derive(Clone, Debug)]
pub enum Trace {
    Application {
        rule: usize,
        ids: Vec<u64>,
        support: Support,
    },
    Birth {
        support: Support,
        choice: Support,
    },
}
pub enum Event {
    Progress,
    Answer(Answer),
    Exhausted,
}
/// Immutable source and lowered matching plans, prepared once and shared by all queries.
/// Query engines retain this allocation after the public prepared handle is dropped.
#[derive(Clone)]
pub struct PreparedRuleset {
    pub(crate) rules: Arc<Vec<crate::resources::Prepared>>,
}
impl PreparedRuleset {
    pub fn new(rules: Vec<Rule>) -> Result<Self, String> {
        Ok(Self {
            rules: Arc::new(crate::resources::compile(rules)?),
        })
    }
    /// Input preparation lowers a finite owned query. Subsequent tick work is
    /// incremental; copies of source syntax are bounded by prepared rule size.
    pub fn start(&self, query: Query) -> Result<Engine, String> {
        let mut store = Store::new();
        let mut vars = BTreeMap::new();
        let mut initial = vec![];
        for c in query.constraints {
            let args = c
                .args
                .iter()
                .map(|t| lower(t, &mut vars, &mut store))
                .collect();
            initial.push((c.name, args));
        }
        let outputs = query
            .outputs
            .into_iter()
            .map(|(name, v)| {
                (
                    name,
                    *vars.entry(v).or_insert_with(|| store.fresh_variable()),
                )
            })
            .collect();
        let resources = Resources::new(self, &store);
        let mut e = Engine {
            store,
            resources,
            arena: Arena::new(),
            births: Births::new(),
            outputs,
            index: BTreeMap::new(),
            known: BTreeSet::new(),
            dependencies: BTreeMap::new(),
            pending: BTreeMap::new(),
            round: BTreeMap::new(),
            remaining: Support::FALSE,
            completed: Support::FALSE,
            phase: Phase::Begin,
            active: None,
            body: None,
            discoveries: VecDeque::new(),
            change_cursor: 0,
            notification: None,
            observations: VecDeque::new(),
            observation_turn: false,
            stats: Stats::default(),
            trace: None,
        };
        for (name, args) in initial {
            e.post(name, args, Support::TRUE);
        }
        Ok(e)
    }
}
fn lower(t: &Source, vars: &mut BTreeMap<Var, Term>, store: &mut Store) -> Term {
    match t {
        Source::Var(v) => *vars.entry(*v).or_insert_with(|| store.fresh_variable()),
        Source::App(name, args) => {
            let fields = args.iter().map(|t| lower(t, vars, store)).collect();
            store.constructor(name.clone(), fields)
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    rule: usize,
    ids: Vec<u64>,
}
type Dirty = BTreeMap<Key, Vec<Support>>;
struct Discovery {
    occurrence: u64,
    rule: usize,
    anchor: usize,
    digits: Option<Vec<usize>>,
}
struct Notification {
    change: Change,
    after: Option<Key>,
}
enum Phase {
    Begin,
    Active(Job),
    Alive(Job),
    Select,
    Certify(Job),
    CertifyNew(Job),
    Record {
        job: Job,
        region: Support,
    },
    Freeze {
        region: Support,
        histories: Histories,
        occurrences: usize,
        variables: usize,
        counts: Vec<usize>,
    },
    Done,
}
struct Active {
    key: Key,
    scopes: Vec<Support>,
    scope: Support,
    stage: ActiveStage,
    dependency: usize,
}
enum ActiveStage {
    Union(Option<Job>),
    Scope(Job),
    Defer {
        job: Job,
        clipped: Support,
    },
    Prepare,
    Match(ApplicationJob),
    Drain {
        job: ApplicationJob,
        token: Option<Token>,
    },
    Remove {
        job: Job,
        body: u64,
    },
    Body,
}
struct Body {
    id: u64,
    vars: BTreeMap<Var, Term>,
    stack: Vec<(Goal, Support)>,
    state: BodyState,
}
enum BodyState {
    Next,
    Scope {
        job: Job,
        goal: Goal,
    },
    Execute {
        goal: Goal,
        scope: Support,
    },
    Unify(Box<UnifyJob>),
    Fail(FailureJob),
    OrLeft {
        job: Job,
        left: Goal,
        right: Goal,
        scope: Support,
        choice: Support,
    },
    OrRight {
        job: Job,
        left: Goal,
        right: Goal,
        left_scope: Support,
    },
    Ack(BodyAckJob),
}
pub struct Engine {
    store: Store,
    resources: Resources,
    arena: Arena,
    births: Births,
    outputs: Vec<(String, Term)>,
    index: BTreeMap<(String, usize), Vec<u64>>,
    known: BTreeSet<Key>,
    dependencies: BTreeMap<usize, BTreeSet<Key>>,
    pending: Dirty,
    round: Dirty,
    remaining: Support,
    completed: Support,
    phase: Phase,
    active: Option<Active>,
    body: Option<Body>,
    discoveries: VecDeque<Discovery>,
    change_cursor: usize,
    notification: Option<Notification>,
    observations: VecDeque<Observation>,
    observation_turn: bool,
    stats: Stats,
    trace: Option<Vec<Trace>>,
}
impl Engine {
    pub fn enable_trace(&mut self) {
        self.trace.get_or_insert_with(Vec::new);
    }
    pub fn trace(&self) -> &[Trace] {
        self.trace.as_deref().unwrap_or(&[])
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn store(&self) -> &Store {
        &self.store
    }
    pub fn resources(&self) -> &Resources {
        &self.resources
    }
    pub fn supports(&self) -> &Arena {
        &self.arena
    }
    pub fn tick(&mut self) -> Event {
        if METRICS {
            self.stats.ticks += 1;
        }
        self.observation_turn = !self.observation_turn;
        if !self.observations.is_empty()
            && (self.observation_turn || matches!(self.phase, Phase::Done))
        {
            let mut observation = self.observations.pop_front().unwrap();
            let event = observation.tick(
                &self.births,
                &self.arena,
                &self.store,
                &self.resources,
                &self.outputs,
            );
            match event {
                ObservationEvent::Answer(a) => {
                    self.observations.push_back(observation);
                    if METRICS {
                        self.stats.answers += 1;
                    }
                    return Event::Answer(a);
                }
                ObservationEvent::Progress => self.observations.push_back(observation),
                ObservationEvent::Done => (),
            }
        } else if !matches!(self.phase, Phase::Done) {
            self.execute_tick();
        }
        if matches!(self.phase, Phase::Done) && self.observations.is_empty() {
            Event::Exhausted
        } else {
            Event::Progress
        }
    }
    fn post(&mut self, name: String, args: Vec<Term>, scope: Support) {
        let key = (name.clone(), args.len());
        let id = self
            .resources
            .insert(name, args, scope, &self.store)
            .expect("owned store");
        self.index.entry(key).or_default().push(id);
        self.discoveries.push_back(Discovery {
            occurrence: id,
            rule: 0,
            anchor: 0,
            digits: None,
        });
    }
    fn discovery_tick(&mut self) {
        let mut d = self.discoveries.pop_front().unwrap();
        if d.rule >= self.resources.rules.len() {
            return;
        }
        let rule = &self.resources.rules[d.rule].source;
        let heads: Vec<_> = rule.kept.iter().chain(&rule.removed).collect();
        if d.anchor >= heads.len() {
            d.rule += 1;
            d.anchor = 0;
            d.digits = None;
            self.discoveries.push_front(d);
            return;
        }
        let o = self.resources.occurrence(d.occurrence).unwrap();
        let h = heads[d.anchor];
        if h.name != o.predicate || h.args.len() != o.args.len() {
            d.anchor += 1;
            self.discoveries.push_front(d);
            return;
        }
        let buckets: Vec<_> = heads
            .iter()
            .map(|h| {
                self.index
                    .get(&(h.name.clone(), h.args.len()))
                    .map(Vec::as_slice)
                    .unwrap_or(&[])
            })
            .collect();
        if buckets.iter().any(|b| b.is_empty()) {
            d.anchor += 1;
            d.digits = None;
            self.discoveries.push_front(d);
            return;
        }
        let digits = d.digits.get_or_insert_with(|| vec![0; heads.len()]);
        let ids: Vec<_> = digits
            .iter()
            .enumerate()
            .map(|(i, &j)| {
                if i == d.anchor {
                    d.occurrence
                } else {
                    buckets[i][j]
                }
            })
            .collect();
        if ids.iter().copied().collect::<BTreeSet<_>>().len() == ids.len() {
            let key = Key { rule: d.rule, ids };
            if self.known.insert(key.clone()) {
                self.pending.entry(key).or_default().push(o.live);
                if METRICS {
                    self.stats.discovered_tuples += 1;
                }
            }
        }
        let mut carry = true;
        for i in (0..digits.len()).rev() {
            if i == d.anchor {
                continue;
            }
            digits[i] += 1;
            if digits[i] < buckets[i].len() {
                carry = false;
                break;
            }
            digits[i] = 0;
        }
        if carry {
            d.anchor += 1;
            d.digits = None;
        }
        self.discoveries.push_front(d);
    }
    fn notification_tick(&mut self) {
        let mut n = self.notification.take().unwrap();
        let key = self
            .dependencies
            .get(&n.change.variable)
            .and_then(|keys| match &n.after {
                Some(k) => keys.range((Excluded(k), Unbounded)).next(),
                None => keys.first(),
            })
            .cloned();
        if let Some(key) = key {
            self.pending
                .entry(key.clone())
                .or_default()
                .push(n.change.support);
            n.after = Some(key);
            self.notification = Some(n);
            if METRICS {
                self.stats.invalidations += 1;
            }
        }
    }
    fn execute_tick(&mut self) {
        if !self.discoveries.is_empty() {
            self.discovery_tick();
            return;
        }
        if self.notification.is_some() {
            self.notification_tick();
            return;
        }
        if let Some(&change) = self.store.changes().get(self.change_cursor) {
            self.change_cursor += 1;
            self.notification = Some(Notification {
                change,
                after: None,
            });
            return;
        }
        if self.body.is_some() {
            self.body_tick();
            return;
        }
        if self.active.is_some() {
            self.active_tick();
            return;
        }
        let phase = std::mem::replace(&mut self.phase, Phase::Select);
        self.phase = match phase {
            Phase::Begin => Phase::Active(
                self.arena
                    .job(Operation::Difference(Support::TRUE, self.completed)),
            ),
            Phase::Active(mut job) => match job.tick(&mut self.arena) {
                Status::Pending => Phase::Active(job),
                Status::Complete(s) => Phase::Alive(
                    self.arena
                        .job(Operation::Difference(s, self.store.failed())),
                ),
            },
            Phase::Alive(mut job) => match job.tick(&mut self.arena) {
                Status::Pending => Phase::Alive(job),
                Status::Complete(s) => {
                    self.remaining = s;
                    self.round = std::mem::take(&mut self.pending);
                    if s == Support::FALSE {
                        Phase::Done
                    } else {
                        Phase::Select
                    }
                }
            },
            Phase::Select => {
                if let Some((key, scopes)) = self.round.pop_first() {
                    self.active = Some(Active {
                        key,
                        scopes,
                        scope: Support::FALSE,
                        stage: ActiveStage::Union(None),
                        dependency: 0,
                    });
                    Phase::Select
                } else {
                    Phase::Certify(
                        self.arena
                            .job(Operation::Difference(self.remaining, self.store.failed())),
                    )
                }
            }
            Phase::Certify(mut job) => match job.tick(&mut self.arena) {
                Status::Pending => Phase::Certify(job),
                Status::Complete(s) => {
                    Phase::CertifyNew(self.arena.job(Operation::Difference(s, self.completed)))
                }
            },
            Phase::CertifyNew(mut job) => match job.tick(&mut self.arena) {
                Status::Pending => Phase::CertifyNew(job),
                Status::Complete(s) => {
                    if s == Support::FALSE {
                        Phase::Begin
                    } else {
                        Phase::Record {
                            job: self.arena.job(Operation::Or(self.completed, s)),
                            region: s,
                        }
                    }
                }
            },
            Phase::Record { mut job, region } => match job.tick(&mut self.arena) {
                Status::Pending => Phase::Record { job, region },
                Status::Complete(s) => {
                    self.completed = s;
                    Phase::Freeze {
                        region,
                        histories: self.births.histories(),
                        occurrences: self.resources.occurrences().len(),
                        variables: self.store.variable_count(),
                        counts: vec![],
                    }
                }
            },
            Phase::Freeze {
                region,
                histories,
                occurrences,
                variables,
                mut counts,
            } => {
                if counts.len() < variables {
                    counts.push(self.store.bindings(counts.len()).len());
                    Phase::Freeze {
                        region,
                        histories,
                        occurrences,
                        variables,
                        counts,
                    }
                } else {
                    self.observations.push_back(Observation::new(
                        region,
                        histories,
                        occurrences,
                        counts,
                    ));
                    Phase::Begin
                }
            }
            Phase::Done => Phase::Done,
        };
    }
    fn active_tick(&mut self) {
        let mut active = self.active.take().unwrap();
        let stage = std::mem::replace(&mut active.stage, ActiveStage::Prepare);
        active.stage = match stage {
            ActiveStage::Union(mut wait) => {
                if let Some(mut job) = wait.take() {
                    match job.tick(&mut self.arena) {
                        Status::Pending => ActiveStage::Union(Some(job)),
                        Status::Complete(s) => {
                            active.scope = s;
                            ActiveStage::Union(None)
                        }
                    }
                } else if let Some(s) = active.scopes.pop() {
                    ActiveStage::Union(Some(self.arena.job(Operation::Or(active.scope, s))))
                } else {
                    ActiveStage::Scope(self.arena.job(Operation::And(active.scope, self.remaining)))
                }
            }
            ActiveStage::Scope(mut job) => match job.tick(&mut self.arena) {
                Status::Pending => ActiveStage::Scope(job),
                Status::Complete(clipped) => ActiveStage::Defer {
                    job: self.arena.job(Operation::Difference(active.scope, clipped)),
                    clipped,
                },
            },
            ActiveStage::Defer { mut job, clipped } => match job.tick(&mut self.arena) {
                Status::Pending => ActiveStage::Defer { job, clipped },
                Status::Complete(deferred) => {
                    if deferred != Support::FALSE {
                        self.pending
                            .entry(active.key.clone())
                            .or_default()
                            .push(deferred);
                    }
                    active.scope = clipped;
                    ActiveStage::Prepare
                }
            },
            ActiveStage::Prepare => {
                if active.scope == Support::FALSE {
                    return;
                }
                active.dependency = 0;
                ActiveStage::Match(
                    self.resources
                        .prepare(
                            active.key.rule,
                            active.key.ids.clone(),
                            active.scope,
                            &self.store,
                        )
                        .expect("discovered tuple"),
                )
            }
            ActiveStage::Match(mut job) => {
                match job.tick(&self.resources, &self.store, &mut self.arena) {
                    ApplicationStatus::Pending => ActiveStage::Match(job),
                    ApplicationStatus::Ready(token) => ActiveStage::Drain {
                        job,
                        token: Some(token),
                    },
                    ApplicationStatus::Ineligible => ActiveStage::Drain { job, token: None },
                    ApplicationStatus::Stale => panic!("serial application snapshot changed"),
                    ApplicationStatus::Finished => {
                        unreachable!("one-frame attempt must be reprepared")
                    }
                }
            }
            ActiveStage::Drain { job, token } => {
                if let Some(d) = job.dependencies().get(active.dependency) {
                    self.dependencies
                        .entry(d.variable)
                        .or_default()
                        .insert(active.key.clone());
                    active.dependency += 1;
                    ActiveStage::Drain { job, token }
                } else if let Some(token) = token {
                    let id = self
                        .resources
                        .commit(token, &self.store)
                        .expect("serial snapshot");
                    let body = self.resources.pending_bodies().get(&id).unwrap();
                    if METRICS {
                        self.stats.applications += 1;
                    }
                    if let Some(trace) = &mut self.trace {
                        trace.push(Trace::Application {
                            rule: active.key.rule,
                            ids: active.key.ids.clone(),
                            support: body.support,
                        });
                    }
                    ActiveStage::Remove {
                        job: self
                            .arena
                            .job(Operation::Difference(self.remaining, body.support)),
                        body: id,
                    }
                } else {
                    return;
                }
            }
            ActiveStage::Remove { mut job, body } => match job.tick(&mut self.arena) {
                Status::Pending => ActiveStage::Remove { job, body },
                Status::Complete(s) => {
                    self.remaining = s;
                    let obligation = self.resources.pending_bodies().get(&body).unwrap();
                    let mut vars = BTreeMap::new();
                    for (&var, slot) in self
                        .resources
                        .variables(obligation.rule)
                        .iter()
                        .zip(&obligation.slots)
                    {
                        vars.insert(var, slot.unwrap_or_else(|| self.store.fresh_variable()));
                    }
                    self.body = Some(Body {
                        id: body,
                        vars,
                        stack: vec![(
                            self.resources.body(obligation.rule).clone(),
                            obligation.support,
                        )],
                        state: BodyState::Next,
                    });
                    ActiveStage::Body
                }
            },
            ActiveStage::Body => {
                ActiveStage::Scope(self.arena.job(Operation::And(active.scope, self.remaining)))
            }
        };
        self.active = Some(active);
    }
    fn body_tick(&mut self) {
        let mut body = self.body.take().unwrap();
        let state = std::mem::replace(&mut body.state, BodyState::Next);
        body.state = match state {
            BodyState::Next => {
                if let Some((goal, scope)) = body.stack.pop() {
                    BodyState::Scope {
                        job: self
                            .arena
                            .job(Operation::Difference(scope, self.store.failed())),
                        goal,
                    }
                } else {
                    BodyState::Ack(
                        self.resources
                            .begin_acknowledge_body(body.id)
                            .expect("pending body"),
                    )
                }
            }
            BodyState::Scope { mut job, goal } => match job.tick(&mut self.arena) {
                Status::Pending => BodyState::Scope { job, goal },
                Status::Complete(scope) => {
                    if scope == Support::FALSE {
                        BodyState::Next
                    } else {
                        BodyState::Execute { goal, scope }
                    }
                }
            },
            BodyState::Execute { goal, scope } => match goal {
                Goal::True => BodyState::Next,
                Goal::Fail => BodyState::Fail(self.store.fail(scope)),
                Goal::And(goals) => {
                    body.stack
                        .extend(goals.into_iter().rev().map(|g| (g, scope)));
                    BodyState::Next
                }
                Goal::Constraint(c) => {
                    let args = c
                        .args
                        .iter()
                        .map(|t| lower(t, &mut body.vars, &mut self.store))
                        .collect();
                    self.post(c.name, args, scope);
                    BodyState::Next
                }
                Goal::Unify(a, b) => {
                    let left = lower(&a, &mut body.vars, &mut self.store);
                    let right = lower(&b, &mut body.vars, &mut self.store);
                    BodyState::Unify(Box::new(self.store.unify(scope, left, right)))
                }
                Goal::Or(left, right) => {
                    let choice = self.births.create(&mut self.arena, scope);
                    if METRICS {
                        self.stats.births += 1;
                    }
                    if let Some(trace) = &mut self.trace {
                        trace.push(Trace::Birth {
                            support: scope,
                            choice,
                        });
                    }
                    BodyState::OrLeft {
                        job: self.arena.job(Operation::Difference(scope, choice)),
                        left: *left,
                        right: *right,
                        scope,
                        choice,
                    }
                }
            },
            BodyState::Unify(mut job) => match job.tick(&mut self.store, &mut self.arena) {
                UnifyStatus::Pending => BodyState::Unify(job),
                UnifyStatus::Complete { .. } => BodyState::Next,
                UnifyStatus::Stale => panic!("serial equality writer changed"),
            },
            BodyState::Fail(mut job) => match job.tick(&mut self.store, &mut self.arena) {
                UnifyStatus::Pending => BodyState::Fail(job),
                UnifyStatus::Complete { .. } => BodyState::Next,
                UnifyStatus::Stale => panic!("serial failure writer changed"),
            },
            BodyState::OrLeft {
                mut job,
                left,
                right,
                scope,
                choice,
            } => match job.tick(&mut self.arena) {
                Status::Pending => BodyState::OrLeft {
                    job,
                    left,
                    right,
                    scope,
                    choice,
                },
                Status::Complete(left_scope) => BodyState::OrRight {
                    job: self.arena.job(Operation::And(scope, choice)),
                    left,
                    right,
                    left_scope,
                },
            },
            BodyState::OrRight {
                mut job,
                left,
                right,
                left_scope,
            } => match job.tick(&mut self.arena) {
                Status::Pending => BodyState::OrRight {
                    job,
                    left,
                    right,
                    left_scope,
                },
                Status::Complete(right_scope) => {
                    body.stack.push((right, right_scope));
                    body.stack.push((left, left_scope));
                    BodyState::Next
                }
            },
            BodyState::Ack(mut job) => match job.tick(&self.resources, &mut self.arena) {
                AckStatus::Pending => BodyState::Ack(job),
                AckStatus::Ready(token) => {
                    self.resources
                        .acknowledge_body(token)
                        .expect("serial body acknowledgment");
                    return;
                }
                AckStatus::Stale => panic!("serial body ledger changed"),
            },
        };
        self.body = Some(body);
    }
}

struct Observation {
    region: Support,
    histories: Histories,
    occurrences: usize,
    bindings: Vec<usize>,
    state: ObserveState,
}
enum ObserveState {
    History,
    Filter { world: Vec<bool>, cursor: Support },
    Export(Export),
}
enum ObservationEvent {
    Progress,
    Answer(Answer),
    Done,
}
impl Observation {
    fn new(
        region: Support,
        histories: Histories,
        occurrences: usize,
        bindings: Vec<usize>,
    ) -> Self {
        Self {
            region,
            histories,
            occurrences,
            bindings,
            state: ObserveState::History,
        }
    }
    fn tick(
        &mut self,
        births: &Births,
        arena: &Arena,
        store: &Store,
        resources: &Resources,
        outputs: &[(String, Term)],
    ) -> ObservationEvent {
        let state = std::mem::replace(&mut self.state, ObserveState::History);
        self.state = match state {
            ObserveState::History => match self.histories.tick(births, arena) {
                HistoryEvent::Progress => ObserveState::History,
                HistoryEvent::Exhausted => return ObservationEvent::Done,
                HistoryEvent::History(world) => ObserveState::Filter {
                    world,
                    cursor: self.region,
                },
            },
            ObserveState::Filter { world, cursor } => match evaluate_node(arena, cursor, &world) {
                Decision::Next(cursor) => ObserveState::Filter { world, cursor },
                Decision::Done(false) => ObserveState::History,
                Decision::Done(true) => ObserveState::Export(Export::new(world)),
            },
            ObserveState::Export(mut export) => {
                if let Some(answer) = export.tick(
                    arena,
                    store,
                    resources,
                    outputs,
                    self.occurrences,
                    &self.bindings,
                ) {
                    return ObservationEvent::Answer(answer);
                } else {
                    ObserveState::Export(export)
                }
            }
        };
        ObservationEvent::Progress
    }
}
enum Decision {
    Next(Support),
    Done(bool),
}
fn evaluate_node(arena: &Arena, s: Support, world: &[bool]) -> Decision {
    match arena.inspect(s) {
        NodeView::False => Decision::Done(false),
        NodeView::True => Decision::Done(true),
        NodeView::Branch {
            variable,
            low,
            high,
        } => Decision::Next(if world.get(variable).copied().unwrap_or(false) {
            high
        } else {
            low
        }),
    }
}
struct Export {
    world: Vec<bool>,
    answer: Answer,
    free: BTreeMap<usize, Var>,
    output: usize,
    occurrence: usize,
    args: Vec<Source>,
    arg: usize,
    state: ExportState,
}
enum ExportState {
    Output,
    OutputTerm(TermExport),
    Occurrence,
    Live(Support),
    Args,
    ArgTerm(TermExport),
    Finish,
}
impl Export {
    fn new(world: Vec<bool>) -> Self {
        Self {
            world,
            answer: Answer {
                outputs: vec![],
                residual: vec![],
            },
            free: BTreeMap::new(),
            output: 0,
            occurrence: 0,
            args: vec![],
            arg: 0,
            state: ExportState::Output,
        }
    }
    fn tick(
        &mut self,
        arena: &Arena,
        store: &Store,
        resources: &Resources,
        outputs: &[(String, Term)],
        occurrences: usize,
        bindings: &[usize],
    ) -> Option<Answer> {
        let state = std::mem::replace(&mut self.state, ExportState::Finish);
        self.state = match state {
            ExportState::Output => {
                if let Some((_, term)) = outputs.get(self.output) {
                    ExportState::OutputTerm(TermExport::new(*term))
                } else {
                    ExportState::Occurrence
                }
            }
            ExportState::OutputTerm(mut job) => {
                if let Some(term) = job.tick(arena, store, &self.world, bindings, &mut self.free) {
                    self.answer
                        .outputs
                        .push((outputs[self.output].0.clone(), term));
                    self.output += 1;
                    ExportState::Output
                } else {
                    ExportState::OutputTerm(job)
                }
            }
            ExportState::Occurrence => {
                if self.occurrence < occurrences {
                    ExportState::Live(resources.occurrences()[self.occurrence].live)
                } else {
                    ExportState::Finish
                }
            }
            ExportState::Live(cursor) => match evaluate_node(arena, cursor, &self.world) {
                Decision::Next(s) => ExportState::Live(s),
                Decision::Done(false) => {
                    self.occurrence += 1;
                    ExportState::Occurrence
                }
                Decision::Done(true) => {
                    self.arg = 0;
                    self.args.clear();
                    ExportState::Args
                }
            },
            ExportState::Args => {
                let o = &resources.occurrences()[self.occurrence];
                if self.arg < o.args.len() {
                    ExportState::ArgTerm(TermExport::new(o.args[self.arg]))
                } else {
                    self.answer.residual.push(Constraint {
                        name: o.predicate.clone(),
                        args: std::mem::take(&mut self.args),
                    });
                    self.occurrence += 1;
                    ExportState::Occurrence
                }
            }
            ExportState::ArgTerm(mut job) => {
                if let Some(term) = job.tick(arena, store, &self.world, bindings, &mut self.free) {
                    self.args.push(term);
                    self.arg += 1;
                    ExportState::Args
                } else {
                    ExportState::ArgTerm(job)
                }
            }
            ExportState::Finish => {
                return Some(std::mem::replace(
                    &mut self.answer,
                    Answer {
                        outputs: vec![],
                        residual: vec![],
                    },
                ));
            }
        };
        None
    }
}
struct TermExport {
    tasks: Vec<TermTask>,
    values: Vec<Vec<Source>>,
}
enum TermTask {
    Visit(Term),
    Binding {
        variable: usize,
        index: usize,
    },
    Evaluate {
        variable: usize,
        index: usize,
        term: Term,
        cursor: Support,
    },
    Fields {
        term: Term,
        index: usize,
    },
    Finish {
        name: String,
    },
}
impl TermExport {
    fn new(term: Term) -> Self {
        Self {
            tasks: vec![TermTask::Visit(term)],
            values: vec![vec![]],
        }
    }
    fn tick(
        &mut self,
        arena: &Arena,
        store: &Store,
        world: &[bool],
        bindings: &[usize],
        free: &mut BTreeMap<usize, Var>,
    ) -> Option<Source> {
        let Some(task) = self.tasks.pop() else {
            return self.values.last_mut().unwrap().pop();
        };
        match task {
            TermTask::Visit(term) => match store.inspect(term) {
                TermView::Variable(variable) => {
                    self.tasks.push(TermTask::Binding { variable, index: 0 })
                }
                TermView::Constructor { name, .. } => {
                    self.values.push(vec![]);
                    self.tasks.push(TermTask::Finish { name: name.into() });
                    self.tasks.push(TermTask::Fields { term, index: 0 });
                }
            },
            TermTask::Binding { variable, index } => {
                if index < bindings[variable] {
                    let b = store.bindings(variable)[index];
                    self.tasks.push(TermTask::Evaluate {
                        variable,
                        index,
                        term: b.term,
                        cursor: b.support,
                    });
                } else {
                    let next = Var(free.len() as u64);
                    self.values
                        .last_mut()
                        .unwrap()
                        .push(Source::Var(*free.entry(variable).or_insert(next)));
                }
            }
            TermTask::Evaluate {
                variable,
                index,
                term,
                cursor,
            } => match evaluate_node(arena, cursor, world) {
                Decision::Next(cursor) => self.tasks.push(TermTask::Evaluate {
                    variable,
                    index,
                    term,
                    cursor,
                }),
                Decision::Done(true) => self.tasks.push(TermTask::Visit(term)),
                Decision::Done(false) => self.tasks.push(TermTask::Binding {
                    variable,
                    index: index + 1,
                }),
            },
            TermTask::Fields { term, index } => {
                if let TermView::Constructor { args, .. } = store.inspect(term)
                    && index < args.len()
                {
                    self.tasks.push(TermTask::Fields {
                        term,
                        index: index + 1,
                    });
                    self.tasks.push(TermTask::Visit(args[index]));
                }
            }
            TermTask::Finish { name } => {
                let args = self.values.pop().unwrap();
                self.values
                    .last_mut()
                    .unwrap()
                    .push(Source::App(name, args));
            }
        }
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use chr_syntax::{Goal, Query, Rule, c, or};
    #[test]
    fn starts_share_compiled_rules_and_own_independent_query_state() {
        let prepared =
            PreparedRuleset::new(vec![Rule::simplify("erase", [c("p", [])], Goal::True)]).unwrap();
        let input = Query {
            constraints: vec![c("p", [])],
            outputs: vec![],
        };
        let mut first = prepared.start(input.clone()).unwrap();
        let mut second = prepared.start(input).unwrap();
        assert!(Arc::ptr_eq(&prepared.rules, &first.resources.rules));
        assert!(Arc::ptr_eq(&first.resources.rules, &second.resources.rules));
        drop(prepared);
        for engine in [&mut first, &mut second] {
            let mut count = 0;
            for _ in 0..1000 {
                match engine.tick() {
                    Event::Answer(answer) => {
                        assert!(answer.residual.is_empty());
                        count += 1;
                    }
                    Event::Exhausted => break,
                    Event::Progress => (),
                }
            }
            assert_eq!(count, 1);
        }
    }
    #[test]
    fn finite_sibling_is_published_beside_symbolic_rewrite_loop() {
        let prepared = PreparedRuleset::new(vec![
            Rule::simplify(
                "choose",
                [c("start", [])],
                or(Goal::from(c("loop", [])), Goal::from(c("done", []))),
            ),
            Rule::simplify("again", [c("loop", [])], Goal::from(c("loop", []))),
        ])
        .unwrap();
        let mut engine = prepared
            .start(Query {
                constraints: vec![c("start", [])],
                outputs: vec![],
            })
            .unwrap();
        for _ in 0..10000 {
            if let Event::Answer(answer) = engine.tick() {
                assert_eq!(answer.residual, vec![c("done", [])]);
                return;
            }
        }
        panic!("finite sibling was starved");
    }
}
