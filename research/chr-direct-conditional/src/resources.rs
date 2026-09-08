//! Serial, version-checked source resource commits. Preparation services support
//! work incrementally. Commit copies only the selected rule frame/tuple, never a
//! support graph. Pending bodies are durable obligations, not completion claims.
//!
//! The caller selects a source rule and ordered occurrence tuple; serial successful
//! commits establish that source application order. A requested scope restricts
//! internally proved eligibility and grants no authority by itself. One prepare
//! job returns one disjoint matching environment. Scheduling further environments,
//! executing bodies, fresh births and final publication belong to the runtime.
//!
//! Initial equality obligations must be serviced before source preparation. After
//! application commit, its busy support remains excluded until the runtime explicitly
//! acknowledges all body and equality effects. Dropping a preparation is harmless;
//! dropping an unacknowledged committed body is query cancellation. Global snapshot
//! invalidation is conservative and does not establish support-local fairness.
//!
//! Each tick services one Boolean step, matching step, guard field, dependency or
//! selected head. Tuple/frame copies are bounded by prepared rule size. Hash/tree
//! lookup, allocation and destruction costs are not hard real-time bounds.
use crate::engine::PreparedRuleset;
use crate::equality::{Change, DemandJob, DemandStatus, Store, Term};
use crate::matching::{MatchJob, MatchStatus, Matched, Node, Pattern, Plan};
use crate::support::{Arena, Job, Operation, Status, Support};
use chr_syntax::{Goal, Guard, Rule, Term as Source, Var};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT: AtomicU64 = AtomicU64::new(1);
#[derive(Debug)]
pub struct Occurrence {
    pub id: u64,
    pub predicate: String,
    pub args: Vec<Term>,
    pub live: Support,
}
#[derive(Debug)]
pub struct BodyObligation {
    pub id: u64,
    pub rule: usize,
    pub support: Support,
    pub slots: Vec<Option<Term>>,
}
pub(crate) struct Prepared {
    pub(crate) source: Rule,
    variables: Vec<Var>,
    heads: Plan,
    guards: Plan,
}
pub struct Resources {
    identity: u64,
    equality: u64,
    version: u64,
    pub(crate) rules: Arc<Vec<Prepared>>,
    occurrences: Vec<Occurrence>,
    history: BTreeMap<(usize, Vec<u64>), Support>,
    pending: BTreeMap<u64, BodyObligation>,
    busy: Support,
    next_body: u64,
}
fn vars(t: &Source, s: &mut BTreeSet<Var>) {
    match t {
        Source::Var(v) => {
            s.insert(*v);
        }
        Source::App(_, args) => {
            for a in args {
                vars(a, s)
            }
        }
    }
}
fn goal_vars(g: &Goal, s: &mut BTreeSet<Var>) {
    match g {
        Goal::Constraint(c) => {
            for t in &c.args {
                vars(t, s)
            }
        }
        Goal::Unify(a, b) => {
            vars(a, s);
            vars(b, s)
        }
        Goal::And(gs) => {
            for g in gs {
                goal_vars(g, s)
            }
        }
        Goal::Or(a, b) => {
            goal_vars(a, s);
            goal_vars(b, s)
        }
        _ => (),
    }
}
fn pattern(t: &Source, vs: &[Var]) -> Pattern {
    match t {
        Source::Var(v) => Pattern::Slot(vs.binary_search(v).unwrap()),
        Source::App(name, args) => Pattern::Constructor {
            name: name.clone(),
            args: args.iter().map(|a| pattern(a, vs)).collect(),
        },
    }
}
pub(crate) fn compile(rules: Vec<Rule>) -> Result<Vec<Prepared>, String> {
    let mut prepared = vec![];
    for rule in rules {
        if rule.kept.is_empty() && rule.removed.is_empty() {
            return Err("empty source head".into());
        }
        let mut vs = BTreeSet::new();
        for h in rule.kept.iter().chain(&rule.removed) {
            for a in &h.args {
                vars(a, &mut vs)
            }
        }
        for Guard::Equal(a, b) in &rule.guards {
            vars(a, &mut vs);
            vars(b, &mut vs)
        }
        goal_vars(&rule.body, &mut vs);
        let variables: Vec<_> = vs.into_iter().collect();
        let heads = Plan::new(
            rule.kept
                .iter()
                .chain(&rule.removed)
                .flat_map(|h| h.args.iter().map(|a| pattern(a, &variables)))
                .collect(),
            variables.len(),
        )?;
        let guards = Plan::new(
            rule.guards
                .iter()
                .flat_map(|Guard::Equal(a, b)| [pattern(a, &variables), pattern(b, &variables)])
                .collect(),
            variables.len(),
        )?;
        prepared.push(Prepared {
            source: rule,
            variables,
            heads,
            guards,
        });
    }
    Ok(prepared)
}
impl Resources {
    /// Allocate query-owned mutable resources while retaining the compiled owner.
    /// Preparation and validation happen only in PreparedRuleset::new.
    pub fn new(prepared: &PreparedRuleset, store: &Store) -> Self {
        Self {
            identity: NEXT
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
                .expect("resource identity overflow"),
            equality: store.identity(),
            version: 0,
            rules: Arc::clone(&prepared.rules),
            occurrences: vec![],
            history: BTreeMap::new(),
            pending: BTreeMap::new(),
            busy: Support::FALSE,
            next_body: 0,
        }
    }
    pub fn insert(
        &mut self,
        predicate: impl Into<String>,
        args: Vec<Term>,
        live: Support,
        store: &Store,
    ) -> Result<u64, String> {
        if store.identity() != self.equality {
            return Err("foreign equality store".into());
        }
        for &t in &args {
            store.inspect(t);
        }
        let id = self.occurrences.len() as u64;
        self.occurrences.push(Occurrence {
            id,
            predicate: predicate.into(),
            args,
            live,
        });
        self.bump();
        Ok(id)
    }
    fn bump(&mut self) {
        self.version = self
            .version
            .checked_add(1)
            .expect("resource version overflow")
    }
    pub fn occurrences(&self) -> &[Occurrence] {
        &self.occurrences
    }
    pub fn occurrence(&self, id: u64) -> Option<&Occurrence> {
        self.occurrences.get(usize::try_from(id).ok()?)
    }
    pub fn history(&self) -> &BTreeMap<(usize, Vec<u64>), Support> {
        &self.history
    }
    pub fn pending_bodies(&self) -> &BTreeMap<u64, BodyObligation> {
        &self.pending
    }
    pub fn body(&self, rule: usize) -> &Goal {
        &self.rules[rule].source.body
    }
    pub fn variables(&self, rule: usize) -> &[Var] {
        &self.rules[rule].variables
    }
    pub fn prepare(
        &self,
        rule: usize,
        ids: Vec<u64>,
        scope: Support,
        store: &Store,
    ) -> Result<ApplicationJob, String> {
        if store.identity() != self.equality {
            return Err("foreign equality store".into());
        }
        let p = self.rules.get(rule).ok_or("unknown rule")?;
        let hs: Vec<_> = p.source.kept.iter().chain(&p.source.removed).collect();
        if hs.len() != ids.len() {
            return Err("head tuple arity".into());
        }
        let mut seen = BTreeSet::new();
        let mut valid = true;
        for (h, id) in hs.iter().zip(&ids) {
            let o = self.occurrence(*id).ok_or("unknown occurrence")?;
            valid &= seen.insert(*id) && h.name == o.predicate && h.args.len() == o.args.len();
        }
        Ok(ApplicationJob {
            owner: self.identity,
            version: self.version,
            equality: store.identity(),
            eq_version: store.version(),
            rule,
            ids,
            region: if valid { scope } else { Support::FALSE },
            phase: 0,
            index: 0,
            wait: None,
            matcher: None,
            guard: None,
            matched: None,
            token: None,
            dependencies: vec![],
            dependency: 0,
        })
    }
    pub fn commit(&mut self, token: Token, store: &Store) -> Result<u64, CommitError> {
        if token.owner != self.identity
            || token.version != self.version
            || token.equality != store.identity()
            || token.eq_version != store.version()
        {
            return Err(CommitError::Stale);
        }
        let id = self.next_body;
        self.next_body = id.checked_add(1).expect("body id overflow");
        for (i, s) in token.lives {
            self.occurrences[i as usize].live = s
        }
        if let Some(s) = token.history {
            self.history.insert((token.rule, token.ids), s);
        }
        self.busy = token.busy;
        self.pending.insert(
            id,
            BodyObligation {
                id,
                rule: token.rule,
                support: token.support,
                slots: token.slots,
            },
        );
        self.bump();
        Ok(id)
    }
    pub fn begin_acknowledge_body(&self, id: u64) -> Result<BodyAckJob, String> {
        let body = self.pending.get(&id).ok_or("unknown body")?;
        Ok(BodyAckJob {
            owner: self.identity,
            version: self.version,
            id,
            operation: Operation::Difference(self.busy, body.support),
            job: None,
        })
    }
    /// Caller asserts every body effect and ensuing equality obligation has finished.
    pub fn acknowledge_body(&mut self, t: AckToken) -> Result<(), CommitError> {
        if t.owner != self.identity
            || t.version != self.version
            || !self.pending.contains_key(&t.id)
        {
            return Err(CommitError::Stale);
        }
        self.pending.remove(&t.id);
        self.busy = t.busy;
        self.bump();
        Ok(())
    }
}
#[derive(Debug)]
pub enum CommitError {
    Stale,
}
pub struct Token {
    owner: u64,
    version: u64,
    equality: u64,
    eq_version: u64,
    rule: usize,
    ids: Vec<u64>,
    support: Support,
    slots: Vec<Option<Term>>,
    lives: Vec<(u64, Support)>,
    history: Option<Support>,
    busy: Support,
}
pub struct AckToken {
    owner: u64,
    version: u64,
    id: u64,
    busy: Support,
}
pub struct BodyAckJob {
    owner: u64,
    version: u64,
    id: u64,
    operation: Operation,
    job: Option<Job>,
}
pub enum AckStatus {
    Pending,
    Ready(AckToken),
    Stale,
}
impl BodyAckJob {
    pub fn tick(&mut self, r: &Resources, a: &mut Arena) -> AckStatus {
        if r.identity != self.owner || r.version != self.version {
            return AckStatus::Stale;
        }
        let j = self.job.get_or_insert_with(|| a.job(self.operation));
        match j.tick(a) {
            Status::Pending => AckStatus::Pending,
            Status::Complete(busy) => AckStatus::Ready(AckToken {
                owner: self.owner,
                version: self.version,
                id: self.id,
                busy,
            }),
        }
    }
}
pub enum ApplicationStatus {
    /// This job already emitted its single eligible frame; no quiescence claim.
    Finished,
    Pending,
    Ready(Token),
    Ineligible,
    Stale,
}
pub struct ApplicationJob {
    owner: u64,
    version: u64,
    equality: u64,
    eq_version: u64,
    rule: usize,
    ids: Vec<u64>,
    region: Support,
    phase: u8,
    index: usize,
    wait: Option<Job>,
    matcher: Option<MatchJob>,
    guard: Option<GuardJob>,
    matched: Option<Matched>,
    token: Option<Token>,
    dependencies: Vec<Change>,
    dependency: usize,
}
impl ApplicationJob {
    pub fn dependencies(&self) -> &[Change] {
        &self.dependencies
    }
    pub fn tick(&mut self, r: &Resources, s: &Store, a: &mut Arena) -> ApplicationStatus {
        if self.phase == 12 {
            return ApplicationStatus::Finished;
        }
        if self.owner != r.identity
            || self.version != r.version
            || self.equality != s.identity()
            || self.eq_version != s.version()
        {
            return ApplicationStatus::Stale;
        }
        let p = &r.rules[self.rule];
        if let Some(j) = &mut self.wait {
            match j.tick(a) {
                Status::Pending => return ApplicationStatus::Pending,
                Status::Complete(v) => {
                    self.wait = None;
                    match self.phase {
                        0..=3 | 9 => self.region = v,
                        6 => self
                            .token
                            .as_mut()
                            .unwrap()
                            .lives
                            .push((self.ids[self.index - 1], v)),
                        10 => self.token.as_mut().unwrap().history = Some(v),
                        11 => self.token.as_mut().unwrap().busy = v,
                        _ => unreachable!(),
                    }
                }
            }
            return ApplicationStatus::Pending;
        }
        match self.phase {
            0 => {
                self.wait = Some(a.job(Operation::Difference(self.region, s.failed())));
                self.phase = 1
            }
            1 => {
                self.wait = Some(a.job(Operation::Difference(self.region, r.busy)));
                self.phase = 2
            }
            2 => {
                if self.index < self.ids.len() {
                    let live = r.occurrence(self.ids[self.index]).unwrap().live;
                    self.index += 1;
                    self.wait = Some(a.job(Operation::And(self.region, live)))
                } else {
                    self.phase = 3
                }
            }
            3 => {
                self.phase = 4;
                if p.source.removed.is_empty() {
                    self.wait = Some(
                        a.job(Operation::Difference(
                            self.region,
                            *r.history
                                .get(&(self.rule, self.ids.clone()))
                                .unwrap_or(&Support::FALSE),
                        )),
                    );
                    self.phase = 9;
                }
            }
            9 => self.phase = 4,
            4 => {
                if self.region == Support::FALSE {
                    return ApplicationStatus::Ineligible;
                }
                if self.matcher.is_none() {
                    let terms = self
                        .ids
                        .iter()
                        .flat_map(|id| r.occurrence(*id).unwrap().args.iter().copied())
                        .collect();
                    self.matcher = Some(p.heads.start(s, self.region, terms).unwrap());
                }
                let m = self.matcher.as_mut().unwrap();
                if self.dependency < m.dependencies().len() {
                    self.dependencies.push(m.dependencies()[self.dependency]);
                    self.dependency += 1;
                    return ApplicationStatus::Pending;
                }
                match m.tick(s, a) {
                    MatchStatus::Pending => (),
                    MatchStatus::Stale => return ApplicationStatus::Stale,
                    MatchStatus::Done => return ApplicationStatus::Ineligible,
                    MatchStatus::Match(frame) => {
                        self.guard = Some(GuardJob::new(p.guards.clone(), &frame));
                        self.matched = Some(frame);
                        self.phase = 5
                    }
                }
            }
            5 => {
                let g = self.guard.as_mut().unwrap();
                if let Some(d) = g.dependencies.pop() {
                    self.dependencies.push(d);
                    return ApplicationStatus::Pending;
                }
                match g.tick(s, a) {
                    GuardStatus::Pending => (),
                    GuardStatus::Done(region) => {
                        if region == Support::FALSE {
                            self.phase = 4
                        } else {
                            let frame = self.matched.take().unwrap();
                            self.token = Some(Token {
                                owner: self.owner,
                                version: self.version,
                                equality: self.equality,
                                eq_version: self.eq_version,
                                rule: self.rule,
                                ids: self.ids.clone(),
                                support: region,
                                slots: frame.slots,
                                lives: vec![],
                                history: None,
                                busy: Support::FALSE,
                            });
                            self.index = p.source.kept.len();
                            self.phase = 6
                        }
                    }
                }
            }
            6 => {
                if self.index < self.ids.len() {
                    let id = self.ids[self.index];
                    self.index += 1;
                    self.wait = Some(a.job(Operation::Difference(
                        r.occurrence(id).unwrap().live,
                        self.token.as_ref().unwrap().support,
                    )))
                } else {
                    self.phase = 7
                }
            }
            7 => {
                self.phase = 8;
                if p.source.removed.is_empty() {
                    self.wait = Some(
                        a.job(Operation::Or(
                            *r.history
                                .get(&(self.rule, self.ids.clone()))
                                .unwrap_or(&Support::FALSE),
                            self.token.as_ref().unwrap().support,
                        )),
                    );
                    self.phase = 10
                }
            }
            10 => self.phase = 8,
            8 => {
                self.wait =
                    Some(a.job(Operation::Or(r.busy, self.token.as_ref().unwrap().support)));
                self.phase = 11
            }
            11 => {
                self.phase = 12;
                return ApplicationStatus::Ready(self.token.take().unwrap());
            }
            _ => return ApplicationStatus::Finished,
        }
        ApplicationStatus::Pending
    }
}
enum GuardTask {
    Pair(usize, usize),
    Children(usize, usize, usize),
}
enum DependencySource {
    Demand(Box<DemandJob>),
    Match(Box<MatchJob>),
}
enum GuardWait {
    Collect {
        source: DependencySource,
        index: usize,
    },
    Demand(DemandJob),
    Match {
        job: MatchJob,
        union: Support,
    },
    Union {
        job: Job,
        matcher: MatchJob,
    },
}
struct GuardJob {
    plan: Plan,
    slots: Vec<Option<Term>>,
    region: Support,
    next: usize,
    tasks: Vec<GuardTask>,
    wait: Option<GuardWait>,
    dependencies: Vec<Change>,
}
enum GuardStatus {
    Pending,
    Done(Support),
}
impl GuardJob {
    fn new(plan: Plan, frame: &Matched) -> Self {
        Self {
            plan,
            slots: frame.slots.clone(),
            region: frame.support,
            next: 0,
            tasks: vec![],
            wait: None,
            dependencies: vec![],
        }
    }
    fn tick(&mut self, s: &Store, a: &mut Arena) -> GuardStatus {
        if let Some(w) = self.wait.take() {
            match w {
                GuardWait::Collect { source, index } => {
                    let deps = match &source {
                        DependencySource::Demand(j) => j.dependencies(),
                        DependencySource::Match(j) => j.dependencies(),
                    };
                    if let Some(d) = deps.get(index) {
                        self.dependencies.push(*d);
                        self.wait = Some(GuardWait::Collect {
                            source,
                            index: index + 1,
                        });
                    }
                }
                GuardWait::Demand(mut j) => match j.tick(s, a) {
                    DemandStatus::Pending => self.wait = Some(GuardWait::Demand(j)),
                    DemandStatus::Complete { entailed } => {
                        self.region = entailed;
                        self.wait = Some(GuardWait::Collect {
                            source: DependencySource::Demand(Box::new(j)),
                            index: 0,
                        });
                    }
                    DemandStatus::Stale => unreachable!(),
                },
                GuardWait::Match { mut job, union } => match job.tick(s, a) {
                    MatchStatus::Pending => self.wait = Some(GuardWait::Match { job, union }),
                    MatchStatus::Match(m) => {
                        self.wait = Some(GuardWait::Union {
                            job: a.job(Operation::Or(union, m.support)),
                            matcher: job,
                        })
                    }
                    MatchStatus::Done => {
                        self.region = union;
                        self.wait = Some(GuardWait::Collect {
                            source: DependencySource::Match(Box::new(job)),
                            index: 0,
                        });
                    }
                    MatchStatus::Stale => unreachable!(),
                },
                GuardWait::Union { mut job, matcher } => match job.tick(a) {
                    Status::Pending => self.wait = Some(GuardWait::Union { job, matcher }),
                    Status::Complete(union) => {
                        self.wait = Some(GuardWait::Match {
                            job: matcher,
                            union,
                        })
                    }
                },
            }
            return GuardStatus::Pending;
        }
        if self.region == Support::FALSE {
            return GuardStatus::Done(self.region);
        }
        let Some(task) = self.tasks.pop() else {
            if self.next == self.plan.roots().len() {
                return GuardStatus::Done(self.region);
            }
            self.tasks.push(GuardTask::Pair(
                self.plan.roots()[self.next],
                self.plan.roots()[self.next + 1],
            ));
            self.next += 2;
            return GuardStatus::Pending;
        };
        match task {
            GuardTask::Children(x, y, i) => {
                if let (Node::Constructor { args: xs, .. }, Node::Constructor { args: ys, .. }) =
                    (self.plan.node(x), self.plan.node(y))
                    && i < xs.len()
                {
                    self.tasks.push(GuardTask::Children(x, y, i + 1));
                    self.tasks.push(GuardTask::Pair(xs[i], ys[i]));
                }
            }
            GuardTask::Pair(x, y) => match (self.plan.node(x), self.plan.node(y)) {
                (Node::Slot(i), Node::Slot(j)) => {
                    if i != j {
                        match (self.slots[*i], self.slots[*j]) {
                            (Some(x), Some(y)) => {
                                self.wait = Some(GuardWait::Demand(s.entails(self.region, x, y)))
                            }
                            _ => self.region = Support::FALSE,
                        }
                    }
                }
                (
                    Node::Constructor { name: n, args: xs },
                    Node::Constructor { name: m, args: ys },
                ) => {
                    if n == m && xs.len() == ys.len() {
                        self.tasks.push(GuardTask::Children(x, y, 0))
                    } else {
                        self.region = Support::FALSE
                    }
                }
                (Node::Slot(i), _) | (_, Node::Slot(i)) => {
                    if let Some(t) = self.slots[*i] {
                        let root = if matches!(self.plan.node(x), Node::Slot(_)) {
                            y
                        } else {
                            x
                        };
                        self.wait = Some(GuardWait::Match {
                            job: self
                                .plan
                                .start_bound(root, s, self.region, t, self.slots.clone()),
                            union: Support::FALSE,
                        })
                    } else {
                        self.region = Support::FALSE
                    }
                }
            },
        }
        GuardStatus::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn emitted_frame_finishes_the_job_without_certifying_remaining_eligibility() {
        let store = Store::new();
        let mut arena = Arena::new();
        let rule = Rule::propagate("visit", [chr_syntax::c("p", [])], Goal::True);
        let mut resources = Resources::new(&PreparedRuleset::new(vec![rule]).unwrap(), &store);
        let id = resources
            .insert("p", vec![], Support::TRUE, &store)
            .unwrap();
        let mut job = resources
            .prepare(0, vec![id], Support::TRUE, &store)
            .unwrap();
        let token = loop {
            match job.tick(&resources, &store, &mut arena) {
                ApplicationStatus::Pending => (),
                ApplicationStatus::Ready(token) => break token,
                _ => panic!("expected a frame"),
            }
        };
        assert!(matches!(
            job.tick(&resources, &store, &mut arena),
            ApplicationStatus::Finished
        ));
        resources.commit(token, &store).unwrap();
        assert!(matches!(
            job.tick(&resources, &store, &mut arena),
            ApplicationStatus::Finished
        ));
    }
}
