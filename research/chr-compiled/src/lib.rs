//! Experimental indexed CHR execution with explicit source-disjunction search.
#[cfg(feature = "fork-diagnostics")]
use chr_persistent::kernel::{ForkObserver, NoopForkObserver, observed_clone};
use chr_persistent::{
    Stats as KernelStats,
    kernel::{Arena, Bindings, Term, deref},
};
use chr_syntax::{Answer, Constraint, Goal, Guard, Query, Rule, Term as Source, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::Arc;
/// Compile-time availability of execution diagnostics.
pub const COLLECT_METRICS: bool = cfg!(feature = "metrics");
pub mod carriers;
#[cfg(feature = "experiment")]
#[path = "../experiments/native.rs"]
pub mod experiment;
pub mod fixtures;
pub mod generate;
pub mod recursive;
pub mod regions;
pub mod search;
pub mod search_fixtures;
pub use search::{CompletedBranch, SearchEngine, SearchEvent, SearchStats};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Policy {
    Global,
    Active,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    Scan,
    Indexed,
}
#[derive(Clone, Copy, Debug)]
pub enum Execution {
    Generic,
    Generated,
}
#[derive(Default, Debug)]
pub struct Stats {
    pub kernel: KernelStats,
    pub predicate_dictionary_entries: usize,
    pub predicate_setup_lookups: u64,
    pub key_visits: u64,
    pub key_template_visits: u64,
    pub key_normalization_requests: u64,
    pub key_normalization_allocations: u64,
    pub index_repairs: u64,
    pub index_inserts: u64,
    pub index_removes: u64,
    pub index_lookups: u64,
    pub index_bucket_entries: u64,
    pub index_entries: usize,
    pub max_index_entries: usize,
    pub source_steps: u64,
    pub applications: u64,
    /// Validated nonempty suffixes handled by the carrier job.
    pub carrier_contractions: u64,
    /// Certified source applications; trace/audit mode expands their commits.
    pub carrier_steps: u64,
    /// Resumable control-spine inspections, including unsuccessful admission.
    pub carrier_checks: u64,
    pub candidate_visits: u64,
    pub structural_tests: u64,
    pub generic_ast_visits: u64,
    pub binding_slot_copies: u64,
    pub rule_dispatches: u64,
    pub cursor_steps: u64,
    pub specialized_rule_dispatches: u64,
    pub specialized_candidates: u64,
    pub specialized_applications: u64,
    /// Flat compiled head and guard operations, excluding the shared body kernel.
    pub specialized_head_instructions: u64,
    pub cursor_pool_entries: u64,
    pub max_cursor_pool_entries: usize,
    pub max_cursor_frames: usize,
    pub pool_visits: u64,
    pub history_checks: u64,
    pub activation_pushes: u64,
    pub activation_coalesced: u64,
    pub activation_pops: u64,
    pub stale_activations: u64,
    pub dependency_visits: u64,
    pub dependency_refreshes: u64,
    pub changed_variables: u64,
    pub observation_visits: u64,
    pub audit_snapshots: u64,
    pub audit_dereferences: u64,
    pub audit_storage_visits: u64,
    pub max_queue: usize,
    pub max_occurrences: usize,
    pub max_dependency_edges: usize,
    pub dependency_edges: usize,
}
#[derive(Clone, Debug)]
pub enum Work {
    Insert(usize, Vec<Term>),
    Equal(Term, Term),
    And(Vec<Work>),
    Or(Box<Work>, Box<Work>),
    True,
    Fail,
}
#[derive(Clone)]
pub struct Frame {
    pub slots: Vec<Option<Term>>,
    pub next: u64,
}
impl Frame {
    pub fn new(slots: usize, next: u64) -> Self {
        Self {
            slots: vec![None; slots],
            next,
        }
    }
}
pub struct Application {
    pub rule: usize,
    pub ids: Vec<u64>,
    pub body: Work,
    pub next: u64,
}
pub type RuleSelector = fn(&mut Core, &mut Cursor, Option<(usize, u64)>) -> Selection;
#[derive(Clone, Copy)]
pub struct Compiled {
    pub source: &'static str,
    pub selectors: &'static [RuleSelector],
}
#[derive(Clone)]
struct Occurrence {
    pred: usize,
    args: Vec<Term>,
}
#[derive(Clone)]
enum Template {
    Slot(usize),
    App(String, Vec<Template>),
}
#[derive(Clone)]
enum Body {
    Insert(usize, Vec<Template>),
    Equal(Template, Template),
    And(Vec<Body>),
    Or(Box<Body>, Box<Body>),
    True,
    Fail,
}
#[derive(Clone)]
struct Head {
    pred: usize,
    args: Vec<Template>,
}
#[derive(Clone)]
struct Prepared {
    carrier: Option<Arc<carriers::Plan>>,
    kept: usize,
    heads: Vec<Head>,
    slots: usize,
    guards: Vec<(Template, Template)>,
    body: Body,
    body_preds: Vec<usize>,
}
fn lower_term(t: &Source, slots: &BTreeMap<u64, usize>) -> Template {
    match t {
        Source::Var(Var(v)) => Template::Slot(slots[v]),
        Source::App(n, args) => Template::App(
            n.clone(),
            args.iter().map(|x| lower_term(x, slots)).collect(),
        ),
    }
}
fn lower_goal(
    g: &Goal,
    slots: &BTreeMap<u64, usize>,
    arena: &mut Arena,
    preds: &mut Vec<usize>,
) -> Body {
    match g {
        Goal::Constraint(c) => {
            let p = arena.predicate(&c.name, c.args.len());
            preds.push(p);
            Body::Insert(p, c.args.iter().map(|x| lower_term(x, slots)).collect())
        }
        Goal::Unify(a, b) => Body::Equal(lower_term(a, slots), lower_term(b, slots)),
        Goal::And(gs) => Body::And(
            gs.iter()
                .map(|g| lower_goal(g, slots, arena, preds))
                .collect(),
        ),
        Goal::True => Body::True,
        Goal::Fail => Body::Fail,
        Goal::Or(a, b) => Body::Or(
            Box::new(lower_goal(a, slots, arena, preds)),
            Box::new(lower_goal(b, slots, arena, preds)),
        ),
    }
}

pub enum Selection {
    Yield,
    Done,
    Found(Application),
}
pub enum Candidate {
    Value(u64),
    Yield,
    Done,
}
#[derive(Clone)]
struct PoolCursor {
    ids: Vec<u64>,
    next: usize,
}
/// DFS partner state. Source effects only run after a cursor returns Found.
#[derive(Clone)]
pub struct Cursor {
    pub anchor_pending: bool,
    pub depth: usize,
    pub ids: Vec<u64>,
    pub frames: Vec<Frame>,
    pools: Vec<Option<PoolCursor>>,
    pool_entries: usize,
}
impl Cursor {
    fn new(slots: usize, heads: usize, next: u64) -> Self {
        Self {
            anchor_pending: true,
            depth: 0,
            ids: vec![],
            frames: vec![Frame::new(slots, next)],
            pools: (0..heads).map(|_| None).collect(),
            pool_entries: 0,
        }
    }
    pub fn descend(&mut self, id: u64, frame: Frame) {
        self.ids.push(id);
        self.frames.push(frame);
        self.depth += 1;
    }
    pub fn backtrack(&mut self) {
        self.depth -= 1;
        self.ids.pop();
        self.frames.pop();
    }
}
#[derive(Clone)]
struct SearchCursor {
    anchor: Option<u64>,
    calls: Vec<(usize, Option<(usize, u64)>)>,
    call: usize,
    cursor: Option<Cursor>,
    direct: Option<regions::DirectCursor>,
}

/// Predicate, argument position, canonical ground term node.
type IndexKey = (usize, usize, usize);

/// Public only so generated Rust uses shared primitive operations, not an AST evaluator.
pub struct Core {
    rules: Arc<Vec<Prepared>>,
    regions: Option<Arc<Vec<Option<regions::Plan>>>>,
    arena: Arena,
    bindings: Bindings,
    store: BTreeMap<u64, Occurrence>,
    pools: BTreeMap<usize, BTreeSet<u64>>,
    dispatch: Arc<BTreeMap<usize, Vec<(usize, usize)>>>,
    history: BTreeSet<(usize, Vec<u64>)>,
    pending: Vec<Work>,
    outputs: Vec<(String, Term)>,
    pub next_var: u64,
    next_occ: u64,
    queue: VecDeque<u64>,
    queued: BTreeSet<u64>,
    dependencies: BTreeMap<u64, BTreeSet<u64>>,
    watchers: BTreeMap<u64, BTreeSet<u64>>,
    pub stats: Stats,
    policy: Policy,
    access: Access,
    index: BTreeMap<IndexKey, BTreeSet<u64>>,
    occurrence_keys: BTreeMap<u64, Vec<Option<IndexKey>>>,
}
fn vars_term(term: &Source, out: &mut BTreeSet<u64>) {
    match term {
        Source::Var(Var(v)) => {
            out.insert(*v);
        }
        Source::App(_, xs) => {
            for x in xs {
                vars_term(x, out)
            }
        }
    }
}
fn vars_goal(goal: &Goal, out: &mut BTreeSet<u64>) -> Result<(), String> {
    match goal {
        Goal::Constraint(c) => {
            for x in &c.args {
                vars_term(x, out)
            }
        }
        Goal::Unify(a, b) => {
            vars_term(a, out);
            vars_term(b, out)
        }
        Goal::And(gs) => {
            for g in gs {
                vars_goal(g, out)?
            }
        }
        Goal::Or(a, b) => {
            vars_goal(a, out)?;
            vars_goal(b, out)?;
        }
        Goal::True | Goal::Fail => (),
    }
    Ok(())
}
impl Core {
    fn segment_stats(&self) -> Stats {
        if !COLLECT_METRICS {
            return Stats::default();
        }
        Stats {
            predicate_dictionary_entries: self.arena.predicates().len(),
            index_entries: self.stats.index_entries,
            dependency_edges: self.stats.dependency_edges,
            max_index_entries: self.stats.index_entries,
            max_dependency_edges: self.stats.dependency_edges,
            max_queue: self.queue.len(),
            max_occurrences: self.store.len(),
            ..Stats::default()
        }
    }
    fn fork_clone(
        &self,
        #[cfg(feature = "fork-diagnostics")] observer: &mut impl ForkObserver,
    ) -> Self {
        macro_rules! copied {
            ($field:ident) => {{
                #[cfg(feature = "fork-diagnostics")]
                {
                    observed_clone(&self.$field, stringify!($field), observer)
                }
                #[cfg(not(feature = "fork-diagnostics"))]
                {
                    self.$field.clone()
                }
            }};
        }
        Self {
            rules: copied!(rules),
            regions: copied!(regions),
            #[cfg(feature = "fork-diagnostics")]
            arena: self.arena.clone_observed(observer),
            #[cfg(not(feature = "fork-diagnostics"))]
            arena: self.arena.clone(),
            bindings: copied!(bindings),
            store: copied!(store),
            pools: copied!(pools),
            dispatch: copied!(dispatch),
            history: copied!(history),
            pending: copied!(pending),
            outputs: copied!(outputs),
            queue: copied!(queue),
            queued: copied!(queued),
            dependencies: copied!(dependencies),
            watchers: copied!(watchers),
            index: copied!(index),
            occurrence_keys: copied!(occurrence_keys),
            next_var: self.next_var,
            next_occ: self.next_occ,
            policy: self.policy,
            access: self.access,
            stats: self.segment_stats(),
        }
    }
    pub fn predicate(&mut self, name: &str, arity: usize) -> usize {
        if COLLECT_METRICS {
            self.stats.predicate_setup_lookups += 1;
        }
        let pred = self.arena.predicate(name, arity);
        if COLLECT_METRICS {
            self.stats.predicate_dictionary_entries = self.arena.predicates().len();
        }
        pred
    }
    fn key_make(&mut self, name: &str, args: Vec<Term>) -> usize {
        let before = if COLLECT_METRICS {
            self.arena.node_count()
        } else {
            0
        };
        if COLLECT_METRICS {
            self.stats.key_normalization_requests += 1;
        }
        let Term::Node(id) = self.arena.make(name, args, &mut self.stats.kernel) else {
            unreachable!()
        };
        if COLLECT_METRICS {
            self.stats.key_normalization_allocations += (self.arena.node_count() - before) as u64;
        }
        id
    }
    fn ground_key(&mut self, value: Term) -> Option<usize> {
        if COLLECT_METRICS {
            self.stats.key_visits += 1;
        }
        match deref(value, &self.bindings, &mut self.stats.kernel) {
            Term::Var(_) => None,
            Term::Node(id) => {
                if self.arena.is_closed(id) {
                    return Some(id);
                }
                let node = self.arena.node(id).clone();
                let mut args = Vec::with_capacity(node.args.len());
                for t in node.args {
                    args.push(Term::Node(self.ground_key(t)?));
                }
                Some(self.key_make(&node.name, args))
            }
        }
    }
    fn template_key(&mut self, t: &Template, frame: &Frame) -> Option<usize> {
        if COLLECT_METRICS {
            self.stats.key_template_visits += 1;
        }
        match t {
            Template::Slot(slot) => self.ground_key(frame.slots[*slot]?),
            Template::App(name, ts) => {
                let mut args = Vec::with_capacity(ts.len());
                for t in ts {
                    args.push(Term::Node(self.template_key(t, frame)?));
                }
                Some(self.key_make(name, args))
            }
        }
    }
    fn best_key(&mut self, rule: usize, head: usize, frame: &Frame) -> Option<IndexKey> {
        if self.access != Access::Indexed {
            return None;
        }
        let program = self.rules.clone();
        let desc = &program[rule].heads[head];
        let pred = desc.pred;
        let mut best = None;
        for (arg, t) in desc.args.iter().enumerate() {
            if let Some(key) = self.template_key(t, frame) {
                if COLLECT_METRICS {
                    self.stats.index_lookups += 1;
                }
                let size = self.index.get(&(pred, arg, key)).map_or(0, BTreeSet::len);
                if best.is_none_or(|(_, n)| size < n) {
                    best = Some(((pred, arg, key), size));
                }
                if size == 0 {
                    break;
                }
            }
        }
        best.map(|(key, _)| key)
    }
    fn pool(&mut self, rule: usize, head: usize, frame: &Frame) -> Vec<u64> {
        let pred = self.rules[rule].heads[head].pred;
        if let Some(key) = self.best_key(rule, head, frame) {
            if COLLECT_METRICS {
                self.stats.index_lookups += 1;
            }
            let ids: Vec<_> = self
                .index
                .get(&key)
                .into_iter()
                .flatten()
                .copied()
                .collect();
            if COLLECT_METRICS {
                self.stats.index_bucket_entries += ids.len() as u64;
            }
            return ids;
        }
        let ids: Vec<_> = self
            .pools
            .get(&pred)
            .into_iter()
            .flatten()
            .copied()
            .collect();
        if COLLECT_METRICS {
            self.stats.pool_visits += ids.len() as u64;
        }
        ids
    }
    fn remove_keys(&mut self, id: u64) {
        if let Some(keys) = self.occurrence_keys.remove(&id) {
            // Reverse key records retain the predicate independently of the live store.
            for key in keys.into_iter().flatten() {
                let bucket = self.index.get_mut(&key).expect("recorded key bucket");
                assert!(bucket.remove(&id));
                if COLLECT_METRICS {
                    self.stats.index_removes += 1;
                }
                if COLLECT_METRICS {
                    self.stats.index_entries -= 1;
                }
                if bucket.is_empty() {
                    self.index.remove(&key);
                }
            }
        }
    }
    fn needs_dependencies(&self) -> bool {
        self.policy == Policy::Active || self.access == Access::Indexed
    }
    pub fn arguments(&mut self, id: u64) -> Option<Vec<Term>> {
        if COLLECT_METRICS {
            self.stats.candidate_visits += 1;
        }
        self.store.get(&id).map(|o| o.args.clone())
    }
    pub fn bind(&mut self, frame: &mut Frame, slot: usize, value: Term) -> bool {
        if COLLECT_METRICS {
            self.stats.structural_tests += 1;
        }
        let value = deref(value, &self.bindings, &mut self.stats.kernel);
        match frame.slots[slot] {
            Some(old) => self
                .arena
                .equal(old, value, &self.bindings, &mut self.stats.kernel),
            None => {
                frame.slots[slot] = Some(value);
                true
            }
        }
    }
    pub fn constructor(&mut self, value: Term, name: &str, arity: usize) -> Option<Vec<Term>> {
        if COLLECT_METRICS {
            self.stats.structural_tests += 1;
        }
        match deref(value, &self.bindings, &mut self.stats.kernel) {
            Term::Node(id) => {
                let n = self.arena.node(id);
                (n.name == name && n.args.len() == arity).then(|| n.args.clone())
            }
            Term::Var(_) => None,
        }
    }
    pub fn variable(&mut self, frame: &mut Frame, slot: usize) -> Term {
        *frame.slots[slot].get_or_insert_with(|| {
            let t = Term::Var(frame.next);
            frame.next += 1;
            t
        })
    }
    pub fn make(&mut self, name: &str, args: Vec<Term>) -> Term {
        self.arena.make(name, args, &mut self.stats.kernel)
    }
    pub fn equal(&mut self, a: Term, b: Term) -> bool {
        self.arena
            .equal(a, b, &self.bindings, &mut self.stats.kernel)
    }
    pub fn eligible(&mut self, rule: usize, ids: &[u64]) -> bool {
        if COLLECT_METRICS {
            self.stats.history_checks += 1;
        }
        !self.history.contains(&(rule, ids.to_vec()))
    }
    pub fn body_predicate(&self, rule: usize, index: usize) -> usize {
        self.rules[rule].body_preds[index]
    }
    pub fn copy_frame(&mut self, frame: &Frame) -> Frame {
        if COLLECT_METRICS {
            self.stats.binding_slot_copies += frame.slots.len() as u64;
        }
        frame.clone()
    }
    pub fn candidate(
        &mut self,
        rule: usize,
        head: usize,
        at: Option<(usize, u64)>,
        cursor: &mut Cursor,
    ) -> Candidate {
        if cursor.pools[head].is_none() {
            let ids = if at.is_some_and(|(h, _)| h == head) {
                vec![at.unwrap().1]
            } else {
                self.pool(rule, head, &cursor.frames[head])
            };
            if COLLECT_METRICS {
                cursor.pool_entries += ids.len();
            }
            if COLLECT_METRICS {
                self.stats.cursor_pool_entries += ids.len() as u64;
            }
            if COLLECT_METRICS {
                self.stats.max_cursor_pool_entries =
                    self.stats.max_cursor_pool_entries.max(cursor.pool_entries);
            }
            cursor.pools[head] = Some(PoolCursor { ids, next: 0 });
        }
        let pool = cursor.pools[head].as_mut().unwrap();
        if pool.next == pool.ids.len() {
            if COLLECT_METRICS {
                cursor.pool_entries -= pool.ids.len();
            }
            cursor.pools[head] = None;
            if head == 0 {
                return Candidate::Done;
            }
            cursor.backtrack();
            return Candidate::Yield;
        }
        let id = pool.ids[pool.next];
        pool.next += 1;
        if cursor.ids.contains(&id) {
            return Candidate::Yield;
        }
        Candidate::Value(id)
    }
    fn generic_pattern(&mut self, p: &Template, value: Term, f: &mut Frame) -> bool {
        if COLLECT_METRICS {
            self.stats.generic_ast_visits += 1;
        }
        match p {
            Template::Slot(slot) => self.bind(f, *slot, value),
            Template::App(n, ps) => match self.constructor(value, n, ps.len()) {
                Some(xs) => ps
                    .iter()
                    .zip(xs)
                    .all(|(p, x)| self.generic_pattern(p, x, f)),
                None => false,
            },
        }
    }
    fn instantiate(&mut self, t: &Template, f: &mut Frame) -> Term {
        if COLLECT_METRICS {
            self.stats.generic_ast_visits += 1;
        }
        match t {
            Template::Slot(slot) => self.variable(f, *slot),
            Template::App(n, xs) => {
                let args = xs.iter().map(|x| self.instantiate(x, f)).collect();
                self.make(n, args)
            }
        }
    }
    fn body(&mut self, g: &Body, f: &mut Frame) -> Work {
        if COLLECT_METRICS {
            self.stats.generic_ast_visits += 1;
        }
        match g {
            Body::Insert(pred, args) => {
                Work::Insert(*pred, args.iter().map(|t| self.instantiate(t, f)).collect())
            }
            Body::Equal(a, b) => Work::Equal(self.instantiate(a, f), self.instantiate(b, f)),
            Body::And(gs) => Work::And(gs.iter().map(|g| self.body(g, f)).collect()),
            Body::Or(a, b) => Work::Or(Box::new(self.body(a, f)), Box::new(self.body(b, f))),
            Body::True => Work::True,
            Body::Fail => Work::Fail,
        }
    }
    fn enqueue(&mut self, id: u64) {
        if self.policy == Policy::Global
            || !self
                .store
                .get(&id)
                .is_some_and(|occ| self.dispatch.contains_key(&occ.pred))
        {
            return;
        }
        if self.queued.insert(id) {
            self.queue.push_back(id);
            if COLLECT_METRICS {
                self.stats.activation_pushes += 1;
            }
            if COLLECT_METRICS {
                self.stats.max_queue = self.stats.max_queue.max(self.queue.len());
            }
        } else {
            if COLLECT_METRICS {
                self.stats.activation_coalesced += 1;
            }
        }
    }
    fn refresh(&mut self, id: u64) {
        // Prepared dispatch is immutable: a no-head predicate cannot become matchable.
        // Residual terms still resolve through the query's ordinary binding map.
        if self
            .store
            .get(&id)
            .is_some_and(|occ| !self.dispatch.contains_key(&occ.pred))
        {
            return;
        }
        if COLLECT_METRICS {
            self.stats.dependency_refreshes += 1;
        }
        if self.access == Access::Indexed {
            if COLLECT_METRICS {
                self.stats.index_repairs += 1;
            }
            self.remove_keys(id);
        }
        if let Some(old) = self.dependencies.remove(&id) {
            if COLLECT_METRICS {
                self.stats.dependency_edges -= old.len();
            }
            for var in old {
                if let Some(w) = self.watchers.get_mut(&var) {
                    w.remove(&id);
                    if w.is_empty() {
                        self.watchers.remove(&var);
                    }
                }
            }
        }
        let Some(occ) = self.store.get(&id) else {
            return;
        };
        let pred = occ.pred;
        let index_args = (self.access == Access::Indexed).then(|| occ.args.clone());
        let mut todo = occ.args.clone();
        let mut vars = BTreeSet::new();
        let mut nodes = BTreeSet::new();
        while let Some(t) = todo.pop() {
            if COLLECT_METRICS {
                self.stats.dependency_visits += 1;
            }
            match t {
                Term::Var(v) => {
                    if vars.insert(v)
                        && let Some(t) = self.bindings.get(&v, &mut self.stats.kernel.storage)
                    {
                        todo.push(t)
                    }
                }
                Term::Node(n) => {
                    if !self.arena.is_closed(n) && nodes.insert(n) {
                        todo.extend(&self.arena.node(n).args)
                    }
                }
            }
        }
        for v in &vars {
            self.watchers.entry(*v).or_default().insert(id);
        }
        if COLLECT_METRICS {
            self.stats.dependency_edges += vars.len();
        }
        self.dependencies.insert(id, vars);
        if COLLECT_METRICS {
            self.stats.max_dependency_edges = self
                .stats
                .max_dependency_edges
                .max(self.stats.dependency_edges);
        }
        if self.access == Access::Indexed {
            let mut keys = vec![];
            for (arg, t) in index_args.unwrap().into_iter().enumerate() {
                let key = self.ground_key(t).map(|k| (pred, arg, k));
                if let Some(key) = key {
                    assert!(self.index.entry(key).or_default().insert(id));
                    if COLLECT_METRICS {
                        self.stats.index_inserts += 1;
                    }
                    if COLLECT_METRICS {
                        self.stats.index_entries += 1;
                    }
                }
                keys.push(key);
            }
            self.occurrence_keys.insert(id, keys);
            if COLLECT_METRICS {
                self.stats.max_index_entries =
                    self.stats.max_index_entries.max(self.stats.index_entries);
            }
        }
    }
    fn insert(&mut self, pred: usize, args: Vec<Term>) {
        let id = self.next_occ;
        self.next_occ += 1;
        self.store.insert(id, Occurrence { pred, args });
        if self.dispatch.contains_key(&pred) {
            self.pools.entry(pred).or_default().insert(id);
            if self.needs_dependencies() {
                self.refresh(id);
            }
        }
        self.enqueue(id);
        if COLLECT_METRICS {
            self.stats.max_occurrences = self.stats.max_occurrences.max(self.store.len());
        }
    }
    fn remove(&mut self, id: u64) {
        if let Some(occ) = self.store.remove(&id)
            && self.dispatch.contains_key(&occ.pred)
        {
            self.pools.get_mut(&occ.pred).unwrap().remove(&id);
            if self.needs_dependencies() {
                self.refresh(id);
            }
        }
    }
    fn equation(&mut self, a: Term, b: Term) -> bool {
        if COLLECT_METRICS {
            self.stats.kernel.equations += 1;
        }
        if !self.needs_dependencies() {
            return self
                .arena
                .unify(a, b, &mut self.bindings, &mut self.stats.kernel);
        }
        let mut changed = Vec::new();
        if !self.arena.unify_record(
            a,
            b,
            &mut self.bindings,
            &mut self.stats.kernel,
            &mut changed,
        ) {
            return false;
        }
        let mut affected = BTreeSet::new();
        for v in changed {
            if COLLECT_METRICS {
                self.stats.changed_variables += 1;
            }
            if let Some(ids) = self.watchers.get(&v) {
                affected.extend(ids)
            }
        }
        for id in affected {
            self.refresh(id);
            self.enqueue(id);
        }
        true
    }
    fn search(&self, anchor: Option<u64>) -> SearchCursor {
        let calls = if let Some(id) = anchor {
            let pred = self.store[&id].pred;
            self.dispatch
                .get(&pred)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .map(|(r, h)| (r, Some((h, id))))
                .collect::<Vec<_>>()
        } else {
            (0..self.rules.len()).map(|r| (r, None)).collect()
        };
        SearchCursor {
            anchor,
            calls,
            call: 0,
            cursor: None,
            direct: None,
        }
    }
    fn search_tick(&mut self, code: Option<Compiled>, search: &mut SearchCursor) -> Selection {
        let Some(&(rule, at)) = search.calls.get(search.call) else {
            return Selection::Done;
        };
        if self
            .regions
            .as_ref()
            .is_some_and(|plans| plans[rule].is_some())
        {
            debug_assert!(at.is_none(), "Global policy has no anchor");
            if search.direct.is_none() {
                if COLLECT_METRICS {
                    self.stats.rule_dispatches += 1;
                    self.stats.specialized_rule_dispatches += 1;
                }
                search.direct = Some(regions::DirectCursor::new(self, rule));
            }
            return match search.direct.as_mut().unwrap().tick(self, rule) {
                Selection::Done => {
                    search.call += 1;
                    search.direct = None;
                    Selection::Yield
                }
                event => event,
            };
        }
        if search.cursor.is_none() {
            if COLLECT_METRICS {
                self.stats.rule_dispatches += 1;
            }
            let r = &self.rules[rule];
            search.cursor = Some(Cursor::new(r.slots, r.heads.len(), self.next_var));
        }
        if COLLECT_METRICS {
            self.stats.cursor_steps += 1;
        }
        let cursor = search.cursor.as_mut().unwrap();
        let event = match code {
            Some(c) => (c.selectors[rule])(self, cursor, at),
            None => generic_rule(self, rule, cursor, at),
        };
        if COLLECT_METRICS {
            self.stats.max_cursor_frames = self.stats.max_cursor_frames.max(cursor.frames.len());
        }
        match event {
            Selection::Done => {
                search.call += 1;
                search.cursor = None;
                Selection::Yield
            }
            other => other,
        }
    }
    fn export(&mut self) -> Answer {
        let s = &mut self.stats.kernel;
        Answer {
            outputs: self
                .outputs
                .iter()
                .map(|(n, t)| (n.clone(), self.arena.export(*t, &self.bindings, s)))
                .collect(),
            residual: self
                .store
                .values()
                .map(|o| Constraint {
                    name: self.arena.predicates()[o.pred].0.clone(),
                    args: o
                        .args
                        .iter()
                        .map(|t| self.arena.export(*t, &self.bindings, s))
                        .collect(),
                })
                .collect(),
        }
    }
    fn view(&mut self) -> View {
        let mut stats = KernelStats::default();
        fn source(
            work: &Work,
            arena: &Arena,
            bindings: &Bindings,
            stats: &mut KernelStats,
        ) -> Goal {
            match work {
                Work::Insert(p, args) => Goal::Constraint(Constraint {
                    name: arena.predicates()[*p].0.clone(),
                    args: args
                        .iter()
                        .map(|t| arena.export(*t, bindings, stats))
                        .collect(),
                }),
                Work::Equal(a, b) => Goal::Unify(
                    arena.export(*a, bindings, stats),
                    arena.export(*b, bindings, stats),
                ),
                Work::And(gs) => Goal::And(
                    gs.iter()
                        .map(|g| source(g, arena, bindings, stats))
                        .collect(),
                ),
                Work::Or(a, b) => Goal::Or(
                    Box::new(source(a, arena, bindings, stats)),
                    Box::new(source(b, arena, bindings, stats)),
                ),
                Work::True => Goal::True,
                Work::Fail => Goal::Fail,
            }
        }
        let result = View {
            store: self
                .store
                .iter()
                .map(|(id, o)| {
                    (
                        *id,
                        Constraint {
                            name: self.arena.predicates()[o.pred].0.clone(),
                            args: o
                                .args
                                .iter()
                                .map(|t| self.arena.export(*t, &self.bindings, &mut stats))
                                .collect(),
                        },
                    )
                })
                .collect(),
            outputs: self
                .outputs
                .iter()
                .map(|(n, t)| (n.clone(), self.arena.export(*t, &self.bindings, &mut stats)))
                .collect(),
            pending: self
                .pending
                .iter()
                .rev()
                .map(|w| source(w, &self.arena, &self.bindings, &mut stats))
                .collect(),
            history: self.history.iter().cloned().collect(),
            next_var: self.next_var,
        };
        if COLLECT_METRICS {
            self.stats.audit_snapshots += 1;
        }
        if COLLECT_METRICS {
            self.stats.audit_dereferences += stats.dereferences;
        }
        if COLLECT_METRICS {
            self.stats.audit_storage_visits += stats.storage.visits;
        }
        result
    }
}
fn generic_rule(
    core: &mut Core,
    rule: usize,
    cursor: &mut Cursor,
    at: Option<(usize, u64)>,
) -> Selection {
    let program = core.rules.clone();
    let prepared = &program[rule];
    if cursor.anchor_pending {
        cursor.anchor_pending = false;
        if let Some((head, id)) = at.filter(|(head, _)| *head > 0) {
            let Some(args) = core.arguments(id) else {
                return Selection::Done;
            };
            let mut frame = core.copy_frame(&cursor.frames[0]);
            if !prepared.heads[head]
                .args
                .iter()
                .zip(args)
                .all(|(p, value)| core.generic_pattern(p, value, &mut frame))
            {
                return Selection::Done;
            }
            cursor.frames[0] = frame;
            return Selection::Yield;
        }
    }
    let h = cursor.depth;
    if h == prepared.heads.len() {
        if !core.eligible(rule, &cursor.ids) {
            cursor.backtrack();
            return Selection::Yield;
        }
        let mut f = core.copy_frame(&cursor.frames[h]);
        for (a, b) in &prepared.guards {
            let a = core.instantiate(a, &mut f);
            let b = core.instantiate(b, &mut f);
            if !core.equal(a, b) {
                cursor.backtrack();
                return Selection::Yield;
            }
        }
        let body = core.body(&prepared.body, &mut f);
        return Selection::Found(Application {
            rule,
            ids: cursor.ids.clone(),
            body,
            next: f.next,
        });
    }
    let id = match core.candidate(rule, h, at, cursor) {
        Candidate::Value(id) => id,
        Candidate::Yield => return Selection::Yield,
        Candidate::Done => return Selection::Done,
    };
    let Some(args) = core.arguments(id) else {
        return Selection::Yield;
    };
    let mut f = core.copy_frame(&cursor.frames[h]);
    if prepared.heads[h]
        .args
        .iter()
        .zip(args)
        .all(|(p, value)| core.generic_pattern(p, value, &mut f))
    {
        cursor.descend(id, f)
    }
    Selection::Yield
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct View {
    pub store: Vec<(u64, Constraint)>,
    pub outputs: Vec<(String, Source)>,
    pub history: Vec<(usize, Vec<u64>)>,
    pub pending: Vec<Goal>,
    pub next_var: u64,
}
#[derive(Clone, Debug)]
pub struct Commit {
    pub rule: usize,
    pub ids: Vec<u64>,
    pub before: View,
    pub after: View,
}
/// Logical retained records, not allocator bytes. Measured by inspection on request.
#[derive(Debug)]
pub struct Retention {
    pub occurrences: usize,
    pub pending: usize,
    pub history: usize,
    pub queue: usize,
    pub dependency_edges: usize,
    pub term_nodes: usize,
    pub index_entries: usize,
    pub index_buckets: usize,
    pub index_reverse_records: usize,
    pub cursor_frames: usize,
    pub cursor_pool_entries: usize,
    pub trace_entries: usize,
    pub audit_entries: usize,
}
#[derive(Debug)]
pub struct Status {
    /// A source disjunction awaits explicit branch search; no answer is available.
    pub pending_split: bool,
    pub exhausted: bool,
    pub failed: bool,
}
pub struct Engine {
    carrier: Option<carriers::Job>,
    carrier_blocked: Option<u64>,
    core: Core,
    code: Option<Compiled>,
    done: bool,
    failed: bool,
    trace: Vec<(usize, Vec<u64>)>,
    trace_enabled: bool,
    audit_enabled: bool,
    audit: Vec<Commit>,
    search: Option<SearchCursor>,
}
#[derive(Debug)]
pub struct PreparationStats {
    pub rules: usize,
    pub heads: usize,
    pub slots: usize,
    pub predicates: usize,
}
#[derive(Clone)]
pub struct PreparedRuleset {
    rules: Arc<Vec<Prepared>>,
    regions: Option<Arc<Vec<Option<regions::Plan>>>>,
    dispatch: Arc<BTreeMap<usize, Vec<(usize, usize)>>>,
    predicates: Arc<Vec<(String, usize)>>,
    code: Option<Compiled>,
}
/// A posted query topology with no source applications or choices executed.
/// Source-variable identities belong to this template; each start owns its engine.
pub struct PreparedQuery {
    engine: Engine,
    scope: BTreeMap<u64, Term>,
}
impl PreparedQuery {
    pub fn start(&self, extra: Vec<Constraint>) -> Result<SearchEngine, String> {
        fn lower(
            core: &mut Core,
            term: &Source,
            scope: &BTreeMap<u64, Term>,
        ) -> Result<Term, String> {
            match term {
                Source::Var(Var(var)) => scope.get(var).copied().ok_or_else(|| {
                    format!("query addition refers to undeclared source variable {var}")
                }),
                Source::App(name, args) => {
                    let args = args
                        .iter()
                        .map(|term| lower(core, term, scope))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(core.arena.make(name, args, &mut core.stats.kernel))
                }
            }
        }
        let mut engine = self.engine.fork_clone(
            #[cfg(feature = "fork-diagnostics")]
            &mut NoopForkObserver,
        );
        for constraint in extra {
            let pred = engine
                .core
                .predicate(&constraint.name, constraint.args.len());
            let args = constraint
                .args
                .iter()
                .map(|term| lower(&mut engine.core, term, &self.scope))
                .collect::<Result<Vec<_>, _>>()?;
            engine.core.insert(pred, args);
        }
        Ok(engine.into_search())
    }
    pub fn retention(&self) -> Retention {
        self.engine.retention()
    }
    pub fn stats(&self) -> &Stats {
        self.engine.stats()
    }
}
impl PreparedRuleset {
    pub fn bundled(id: usize, execution: Execution) -> Result<Self, String> {
        let rules = fixtures::programs()
            .get(id)
            .cloned()
            .ok_or("unknown program")?;
        let code = match execution {
            Execution::Generic => None,
            Execution::Generated => Some(bundled(id)),
        };
        Self::new(rules, code)
    }
    pub fn new(rules: Vec<Rule>, code: Option<Compiled>) -> Result<Self, String> {
        if code.is_some_and(|c| c.source != format!("{rules:?}")) {
            return Err("generated program does not match supplied rules".into());
        }
        let mut arena = Arena::default();
        let mut prepared = vec![];
        let mut dispatch: BTreeMap<usize, Vec<(usize, usize)>> = BTreeMap::new();
        let mut rule_names = BTreeSet::new();
        for (r, rule) in rules.into_iter().enumerate() {
            if rule.kept.is_empty() && rule.removed.is_empty() {
                return Err("empty heads".into());
            }
            if !rule_names.insert(rule.name.clone()) {
                return Err("duplicate rule name".into());
            }
            let mut vars = BTreeSet::new();
            let mut preds = vec![];
            for (h, head) in rule.kept.iter().chain(&rule.removed).enumerate() {
                let pred = arena.predicate(&head.name, head.args.len());
                preds.push(pred);
                dispatch.entry(pred).or_default().push((r, h));
                for t in &head.args {
                    vars_term(t, &mut vars)
                }
            }
            for Guard::Equal(a, b) in &rule.guards {
                vars_term(a, &mut vars);
                vars_term(b, &mut vars)
            }
            vars_goal(&rule.body, &mut vars)?;
            let slots = vars
                .into_iter()
                .enumerate()
                .map(|(i, v)| (v, i))
                .collect::<BTreeMap<_, _>>();
            let heads = rule
                .kept
                .iter()
                .chain(&rule.removed)
                .zip(preds)
                .map(|(h, pred)| Head {
                    pred,
                    args: h.args.iter().map(|t| lower_term(t, &slots)).collect(),
                })
                .collect();
            let guards = rule
                .guards
                .iter()
                .map(|Guard::Equal(a, b)| (lower_term(a, &slots), lower_term(b, &slots)))
                .collect();
            let mut body_preds = vec![];
            let body = lower_goal(&rule.body, &slots, &mut arena, &mut body_preds);
            prepared.push(Prepared {
                carrier: None,
                kept: rule.kept.len(),
                heads,
                slots: slots.len(),
                guards,
                body,
                body_preds,
            });
        }
        Ok(Self {
            rules: Arc::new(prepared),
            regions: None,
            dispatch: Arc::new(dispatch),
            predicates: Arc::new(arena.into_predicates()),
            code,
        })
    }
    pub fn stats(&self) -> PreparationStats {
        PreparationStats {
            rules: self.rules.len(),
            heads: self.rules.iter().map(|r| r.heads.len()).sum(),
            slots: self.rules.iter().map(|r| r.slots).sum(),
            predicates: self.predicates.len(),
        }
    }
    pub fn start_search(
        &self,
        query: Query,
        policy: Policy,
        access: Access,
    ) -> Result<SearchEngine, String> {
        self.start(query, policy, access).map(Engine::into_search)
    }
    pub fn prepare_query(
        &self,
        query: Query,
        policy: Policy,
        access: Access,
    ) -> Result<PreparedQuery, String> {
        let (mut engine, scope) = self.start_with_scope(query, policy, access)?;
        while let Some(work) = engine.core.pending.pop() {
            let Work::Insert(pred, args) = work else {
                unreachable!("query setup contains only initial constraints");
            };
            engine.core.insert(pred, args);
        }
        Ok(PreparedQuery { engine, scope })
    }
    pub fn start(&self, query: Query, policy: Policy, access: Access) -> Result<Engine, String> {
        self.start_with_scope(query, policy, access)
            .map(|(engine, _)| engine)
    }
    fn start_with_scope(
        &self,
        query: Query,
        policy: Policy,
        access: Access,
    ) -> Result<(Engine, BTreeMap<u64, Term>), String> {
        if self.regions.is_some() && policy != Policy::Global {
            return Err("sealed specialization requires Global source policy".into());
        }
        let mut arena = Arena::default();
        for (name, arity) in self.predicates.iter() {
            arena.predicate(name, *arity);
        }
        let mut core = Core {
            rules: self.rules.clone(),
            regions: self.regions.clone(),
            arena,
            bindings: Bindings::default(),
            store: BTreeMap::new(),
            pools: BTreeMap::new(),
            dispatch: self.dispatch.clone(),
            history: BTreeSet::new(),
            pending: vec![],
            outputs: vec![],
            next_var: 0,
            next_occ: 0,
            queue: VecDeque::new(),
            queued: BTreeSet::new(),
            dependencies: BTreeMap::new(),
            watchers: BTreeMap::new(),
            stats: if COLLECT_METRICS {
                Stats {
                    predicate_dictionary_entries: self.predicates.len(),
                    predicate_setup_lookups: self.predicates.len() as u64,
                    ..Stats::default()
                }
            } else {
                Stats::default()
            },
            policy,
            access,
            index: BTreeMap::new(),
            occurrence_keys: BTreeMap::new(),
        };
        let mut scope = BTreeMap::new();
        let mut names = BTreeSet::new();
        for c in query.constraints {
            let pred = core.predicate(&c.name, c.args.len());
            let args = c
                .args
                .iter()
                .map(|t| {
                    core.arena.instantiate(
                        t,
                        &mut scope,
                        &mut core.next_var,
                        &mut core.stats.kernel,
                    )
                })
                .collect();
            core.pending.push(Work::Insert(pred, args));
        }
        core.pending.reverse();
        for (name, var) in query.outputs {
            if !names.insert(name.clone()) {
                return Err("duplicate output name".into());
            }
            let value = core.arena.instantiate(
                &Source::Var(var),
                &mut scope,
                &mut core.next_var,
                &mut core.stats.kernel,
            );
            core.outputs.push((name, value));
        }
        Ok((
            Engine {
                carrier: None,
                carrier_blocked: None,
                core,
                code: self.code,
                done: false,
                failed: false,
                trace: vec![],
                trace_enabled: false,
                audit_enabled: false,
                audit: vec![],
                search: None,
            },
            scope,
        ))
    }
}
impl Engine {
    fn pending_split(&self) -> bool {
        !self.done && matches!(self.core.pending.last(), Some(Work::Or(..)))
    }
    fn fork_clone(
        &self,
        #[cfg(feature = "fork-diagnostics")] observer: &mut impl ForkObserver,
    ) -> Self {
        macro_rules! copied {
            ($field:ident) => {{
                #[cfg(feature = "fork-diagnostics")]
                {
                    observed_clone(&self.$field, stringify!($field), observer)
                }
                #[cfg(not(feature = "fork-diagnostics"))]
                {
                    self.$field.clone()
                }
            }};
        }
        Self {
            carrier: self.carrier.clone(),
            carrier_blocked: self.carrier_blocked,
            core: self.core.fork_clone(
                #[cfg(feature = "fork-diagnostics")]
                observer,
            ),
            code: self.code,
            done: self.done,
            failed: self.failed,
            trace: copied!(trace),
            trace_enabled: self.trace_enabled,
            audit_enabled: self.audit_enabled,
            audit: copied!(audit),
            search: copied!(search),
        }
    }
    pub fn into_search(self) -> SearchEngine {
        SearchEngine::new(self)
    }
    pub fn step(&mut self) {
        if self.done || self.pending_split() {
            return;
        }
        if COLLECT_METRICS {
            self.core.stats.source_steps += 1;
        }
        if self.carrier_tick() {
            return;
        }
        if let Some(work) = self.core.pending.pop() {
            match work {
                Work::Insert(p, args) => {
                    let id = self.core.next_occ;
                    self.core.insert(p, args);
                    self.carrier_inserted(id);
                }
                Work::Equal(a, b) => {
                    self.carrier_blocked = None;
                    if !self.core.equation(a, b) {
                        self.done = true;
                        self.failed = true
                    }
                }
                Work::And(gs) => self.core.pending.extend(gs.into_iter().rev()),
                Work::Or(..) => unreachable!("pending disjunction stops before service"),
                Work::True => (),
                Work::Fail => {
                    self.done = true;
                    self.failed = true
                }
            }
            return;
        }
        if self.search.is_none() {
            let anchor = if self.core.policy == Policy::Active {
                let Some(id) = self.core.queue.pop_front() else {
                    self.done = true;
                    return;
                };
                self.core.queued.remove(&id);
                if COLLECT_METRICS {
                    self.core.stats.activation_pops += 1;
                }
                if !self.core.store.contains_key(&id) {
                    if COLLECT_METRICS {
                        self.core.stats.stale_activations += 1;
                    }
                    return;
                }
                Some(id)
            } else {
                None
            };
            self.search = Some(self.core.search(anchor));
        }
        let event = self
            .core
            .search_tick(self.code, self.search.as_mut().unwrap());
        match event {
            Selection::Yield => (),
            Selection::Done => {
                self.search = None;
                if self.core.policy == Policy::Global {
                    self.done = true
                }
            }
            Selection::Found(app) => {
                let anchor = self.search.take().unwrap().anchor;
                self.carrier_entry(&app);
                self.commit_application(app, anchor);
            }
        }
    }

    fn commit_application(&mut self, app: Application, anchor: Option<u64>) {
        let before = if self.audit_enabled {
            Some((self.core.view(), app.ids.clone()))
        } else {
            None
        };
        let audit_rule = app.rule;
        let kept = self.core.rules[app.rule].kept;
        for id in app.ids.iter().skip(kept) {
            self.core.remove(*id)
        }
        if self
            .core
            .regions
            .as_ref()
            .is_some_and(|plans| plans[app.rule].is_some())
        {
            if COLLECT_METRICS {
                self.core.stats.specialized_applications += 1;
            }
        } else {
            self.core.history.insert((app.rule, app.ids.clone()));
        }
        if self.trace_enabled {
            self.trace.push((app.rule, app.ids));
        }
        self.core.next_var = app.next;
        self.core.pending.push(app.body);
        if COLLECT_METRICS {
            self.core.stats.applications += 1;
        }
        if let Some((before, ids)) = before {
            self.audit.push(Commit {
                rule: audit_rule,
                ids,
                before,
                after: self.core.view(),
            })
        }
        if let Some(id) = anchor
            && self.core.store.contains_key(&id)
        {
            self.core.enqueue(id)
        }
    }

    /// Advance execution without exporting an answer.
    pub fn advance(&mut self, budget: usize) -> Status {
        for _ in 0..budget {
            if self.done || self.pending_split() {
                break;
            }
            self.step()
        }
        self.status()
    }
    pub fn status(&self) -> Status {
        Status {
            pending_split: self.pending_split(),
            exhausted: self.done,
            failed: self.failed,
        }
    }
    /// Export a completed successful state. Repeated calls produce the same observation.
    pub fn observe(&mut self) -> Option<Answer> {
        if self.done && !self.failed {
            if COLLECT_METRICS {
                self.core.stats.observation_visits += 1;
            }
            Some(self.core.export())
        } else {
            None
        }
    }
    pub fn enable_trace(&mut self) {
        self.trace_enabled = true
    }
    pub fn enable_audit(&mut self) {
        self.trace_enabled = true;
        self.audit_enabled = true
    }
    pub fn audit(&self) -> &[Commit] {
        &self.audit
    }
    pub fn view(&mut self) -> View {
        self.core.view()
    }
    pub fn retention(&self) -> Retention {
        let cursor = self.search.as_ref().and_then(|s| s.cursor.as_ref());
        Retention {
            occurrences: self.core.store.len(),
            pending: self.core.pending.len(),
            history: self.core.history.len(),
            queue: self.core.queue.len(),
            dependency_edges: self.core.dependencies.values().map(BTreeSet::len).sum(),
            term_nodes: self.core.arena.node_count(),
            index_entries: self.core.index.values().map(BTreeSet::len).sum(),
            index_buckets: self.core.index.len(),
            index_reverse_records: self.core.occurrence_keys.len(),
            cursor_frames: cursor.map_or(0, |c| c.frames.len()),
            cursor_pool_entries: cursor
                .map_or(0, |c| c.pools.iter().flatten().map(|p| p.ids.len()).sum()),
            trace_entries: self.trace.len(),
            audit_entries: self.audit.len(),
        }
    }
    pub fn stats(&self) -> &Stats {
        &self.core.stats
    }
    pub fn trace(&self) -> &[(usize, Vec<u64>)] {
        &self.trace
    }
}
include!(concat!(env!("OUT_DIR"), "/generated.rs"));
