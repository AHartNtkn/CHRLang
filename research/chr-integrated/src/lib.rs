//! Experimental, single-interpretation CHR with interleaved equality deductions.
//!
//! Value identities and occurrence identities are separate. Constructor congruence
//! is repaired through child incidence; source indexes use canonical value classes.
//! Matching only reads established facts. Publication additionally requires an
//! complete worklist exhaustion; `audit` supplies an independent terminal check.
use chr_syntax::{Answer, Constraint, Goal, Guard, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::Arc;

type Value = usize;
type Signature = (String, usize);
type Key = (String, Vec<Value>);
type Column = (String, usize, usize, Value);

#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub steps: usize,
    pub equality_steps: usize,
    pub descriptor_repairs: usize,
    pub index_migrations: usize,
    pub activations: usize,
    pub candidate_visits: usize,
    pub indexed_lookups: usize,
    pub applications: usize,
    pub speculative_applications: usize,
}
macro_rules! count {
    ($self:ident, $field:ident) => {
        #[cfg(feature = "metrics")]
        {
            $self.stats.$field += 1;
        }
    };
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Progress,
    Complete,
    Failed,
}
#[derive(Clone, Debug)]
pub struct Commit {
    pub rule: usize,
    pub ids: Vec<u64>,
    pub pending_equalities: usize,
    pub pending_repairs: usize,
}
#[derive(Clone, Debug, Default)]
pub struct Audit {
    pub pending_equalities: usize,
    pub pending_repairs: usize,
    pub pending_source: usize,
    pub pending_bodies: usize,
    pub enabled_applications: usize,
    pub inconsistent: bool,
    pub indexes_valid: bool,
    pub live_occurrences: usize,
    pub retained_occurrences: usize,
    pub allocated_values: usize,
    pub value_classes: usize,
    pub constructor_descriptors: usize,
    pub propagation_tokens: usize,
    /// Recorded applications; populated only when diagnostic tracing is enabled.
    pub applications: usize,
    /// Recorded applications whose commit preceded queued equality deductions.
    pub applications_with_pending_equalities: usize,
    pub complete: bool,
}
#[derive(Clone, Debug)]
pub struct View {
    pub outputs: Vec<(String, Term)>,
    pub live: Vec<(u64, Constraint)>,
    pub history: Vec<(usize, Vec<u64>)>,
}
#[derive(Clone, Debug)]
pub struct PreparedRuleset {
    rules: Arc<Vec<Rule>>,
    heads: Arc<BTreeMap<Signature, Vec<(usize, usize)>>>,
}
impl PreparedRuleset {
    pub fn new(rules: &[Rule]) -> Result<Self, String> {
        fn ordinary(goal: &Goal) -> bool {
            match goal {
                Goal::Or(..) => false,
                Goal::And(gs) => gs.iter().all(ordinary),
                _ => true,
            }
        }
        let mut heads: BTreeMap<Signature, Vec<(usize, usize)>> = BTreeMap::new();
        for (ri, rule) in rules.iter().enumerate() {
            if rule.kept.is_empty() && rule.removed.is_empty() {
                return Err("rules require at least one head".into());
            }
            if !ordinary(&rule.body) {
                return Err("OR is outside the ordinary integrated experiment".into());
            }
            for (hi, head) in rule.kept.iter().chain(&rule.removed).enumerate() {
                heads
                    .entry((head.name.clone(), head.args.len()))
                    .or_default()
                    .push((ri, hi));
            }
        }
        Ok(Self {
            rules: Arc::new(rules.to_vec()),
            heads: Arc::new(heads),
        })
    }
    pub fn start(&self, query: &Query) -> Engine {
        let mut engine = Engine::new(self.clone());
        let mut vars = BTreeMap::new();
        for constraint in &query.constraints {
            engine.post(constraint, &mut vars);
        }
        for (name, var) in &query.outputs {
            let value = *vars.entry(*var).or_insert_with(|| engine.fresh());
            engine.outputs.push((name.clone(), value));
        }
        engine
    }
}
#[derive(Clone, Debug)]
struct Descriptor {
    name: String,
    children: Vec<Value>,
    owner: Value,
    key: Option<Key>,
}
#[derive(Clone, Debug)]
struct Node {
    parent: Value,
    rank: usize,
    descriptor: Option<usize>,
    parents: BTreeSet<usize>,
    uses: BTreeSet<(u64, usize)>,
}
#[derive(Clone, Debug)]
struct Occurrence {
    name: String,
    args: Vec<Value>,
    live: bool,
}
#[derive(Clone, Debug)]
struct Scan {
    rule: usize,
    anchor: usize,
    lists: Vec<Vec<u64>>,
    positions: Vec<usize>,
}
impl Scan {
    fn next(&mut self) -> Option<Vec<u64>> {
        if self.lists.is_empty() || self.lists.iter().any(Vec::is_empty) {
            return None;
        }
        let tuple = self
            .lists
            .iter()
            .zip(&self.positions)
            .map(|(xs, i)| xs[*i])
            .collect();
        let mut overflow = true;
        for i in (0..self.positions.len()).rev() {
            self.positions[i] += 1;
            if self.positions[i] < self.lists[i].len() {
                overflow = false;
                break;
            }
            self.positions[i] = 0;
        }
        if overflow {
            self.lists.clear();
        }
        Some(tuple)
    }
}
#[derive(Clone, Debug)]
pub struct Engine {
    prepared: PreparedRuleset,
    nodes: Vec<Node>,
    descriptors: Vec<Descriptor>,
    constructors: BTreeMap<Key, BTreeSet<usize>>,
    occurrences: Vec<Occurrence>,
    predicates: BTreeMap<Signature, BTreeSet<u64>>,
    columns: BTreeMap<Column, BTreeSet<u64>>,
    equations: VecDeque<(Value, Value)>,
    equation_set: BTreeSet<(Value, Value)>,
    repairs: VecDeque<usize>,
    repair_set: BTreeSet<usize>,
    activations: VecDeque<(u64, usize, usize)>,
    activation_set: BTreeSet<(u64, usize, usize)>,
    scan: Option<Scan>,
    history: BTreeSet<(usize, Vec<u64>)>,
    outputs: Vec<(String, Value)>,
    trace: Vec<Commit>,
    trace_enabled: bool,
    stats: Stats,
    source_turn: bool,
    repair_turn: bool,
    failed: bool,
    complete: bool,
}
impl Engine {
    fn new(prepared: PreparedRuleset) -> Self {
        Self {
            prepared,
            nodes: vec![],
            descriptors: vec![],
            constructors: BTreeMap::new(),
            occurrences: vec![],
            predicates: BTreeMap::new(),
            columns: BTreeMap::new(),
            equations: VecDeque::new(),
            equation_set: BTreeSet::new(),
            repairs: VecDeque::new(),
            repair_set: BTreeSet::new(),
            activations: VecDeque::new(),
            activation_set: BTreeSet::new(),
            scan: None,
            history: BTreeSet::new(),
            outputs: vec![],
            trace: vec![],
            trace_enabled: false,
            stats: Stats::default(),
            source_turn: true,
            repair_turn: true,
            failed: false,
            complete: false,
        }
    }
    fn root(&self, mut value: Value) -> Value {
        while self.nodes[value].parent != value {
            value = self.nodes[value].parent;
        }
        value
    }
    fn fresh(&mut self) -> Value {
        let id = self.nodes.len();
        self.nodes.push(Node {
            parent: id,
            rank: 0,
            descriptor: None,
            parents: BTreeSet::new(),
            uses: BTreeSet::new(),
        });
        id
    }
    fn lower(&mut self, term: &Term, vars: &mut BTreeMap<Var, Value>) -> Value {
        match term {
            Term::Var(var) => *vars.entry(*var).or_insert_with(|| self.fresh()),
            Term::App(name, children) => {
                let children: Vec<_> = children.iter().map(|t| self.lower(t, vars)).collect();
                let key = (
                    name.clone(),
                    children.iter().map(|v| self.root(*v)).collect::<Vec<_>>(),
                );
                if let Some(existing) = self.constructors.get(&key).and_then(|bucket| {
                    bucket.iter().find(|id| {
                        self.descriptors[**id]
                            .children
                            .iter()
                            .map(|v| self.root(*v))
                            .eq(key.1.iter().copied())
                    })
                }) {
                    return self.root(self.descriptors[*existing].owner);
                }
                let owner = self.fresh();
                let id = self.descriptors.len();
                self.descriptors.push(Descriptor {
                    name: name.clone(),
                    children: children.clone(),
                    owner,
                    key: Some(key.clone()),
                });
                self.nodes[owner].descriptor = Some(id);
                for child in children {
                    let root = self.root(child);
                    self.nodes[root].parents.insert(id);
                }
                self.constructors.entry(key).or_default().insert(id);
                owner
            }
        }
    }
    fn repair(&mut self, descriptor: usize) {
        if self.repair_set.insert(descriptor) {
            self.repairs.push_back(descriptor);
        }
    }
    fn equate(&mut self, left: Value, right: Value) {
        let (a, b) = (self.root(left), self.root(right));
        if a == b {
            return;
        }
        let pair = if a < b { (a, b) } else { (b, a) };
        if self.equation_set.insert(pair) {
            self.equations.push_back(pair);
        }
    }
    fn activate(&mut self, id: u64) {
        let occ = &self.occurrences[id as usize];
        if !occ.live {
            return;
        }
        let entries = self
            .prepared
            .heads
            .get(&(occ.name.clone(), occ.args.len()))
            .cloned()
            .unwrap_or_default();
        for (rule, head) in entries {
            let event = (id, rule, head);
            if self.activation_set.insert(event) {
                self.activations.push_back(event);
                count!(self, activations);
            }
        }
    }
    fn wake(&mut self, value: Value) {
        let mut todo = vec![value];
        let mut visited = BTreeSet::new();
        let mut occurrences = BTreeSet::new();
        while let Some(value) = todo.pop() {
            let root = self.root(value);
            if !visited.insert(root) {
                continue;
            }
            occurrences.extend(self.nodes[root].uses.iter().map(|(id, _)| *id));
            todo.extend(
                self.nodes[root]
                    .parents
                    .iter()
                    .map(|d| self.descriptors[*d].owner),
            );
        }
        for id in occurrences {
            self.activate(id);
        }
    }
    fn post(&mut self, constraint: &Constraint, vars: &mut BTreeMap<Var, Value>) {
        let args: Vec<_> = constraint
            .args
            .iter()
            .map(|term| self.lower(term, vars))
            .collect();
        let id = self.occurrences.len() as u64;
        self.occurrences.push(Occurrence {
            name: constraint.name.clone(),
            args: args.clone(),
            live: true,
        });
        self.predicates
            .entry((constraint.name.clone(), args.len()))
            .or_default()
            .insert(id);
        for (column, arg) in args.iter().enumerate() {
            let root = self.root(*arg);
            self.nodes[root].uses.insert((id, column));
            self.columns
                .entry((constraint.name.clone(), args.len(), column, root))
                .or_default()
                .insert(id);
        }
        self.activate(id);
    }
    fn body(&mut self, body: &Goal, vars: &mut BTreeMap<Var, Value>) {
        match body {
            Goal::Constraint(c) => self.post(c, vars),
            Goal::Unify(a, b) => {
                let a = self.lower(a, vars);
                let b = self.lower(b, vars);
                self.equate(a, b);
            }
            Goal::And(goals) => {
                for goal in goals {
                    self.body(goal, vars);
                }
            }
            Goal::True => (),
            Goal::Fail => self.failed = true,
            Goal::Or(..) => unreachable!("validated ordinary body"),
        }
    }
    fn retire(&mut self, id: u64) {
        let occ = self.occurrences[id as usize].clone();
        self.occurrences[id as usize].live = false;
        if let Some(bucket) = self.predicates.get_mut(&(occ.name.clone(), occ.args.len())) {
            bucket.remove(&id);
        }
        for (column, value) in occ.args.iter().enumerate() {
            let root = self.root(*value);
            self.nodes[root].uses.remove(&(id, column));
            if let Some(bucket) =
                self.columns
                    .get_mut(&(occ.name.clone(), occ.args.len(), column, root))
            {
                bucket.remove(&id);
            }
        }
    }
    fn equality_step(&mut self) {
        let (left, right) = self.equations.pop_front().unwrap();
        self.equation_set.remove(&(left, right));
        count!(self, equality_steps);
        let (mut a, mut b) = (self.root(left), self.root(right));
        if a == b {
            return;
        }
        // The maintained selected-descriptor graph is acyclic. Identifying two
        // vertices creates a positive cycle exactly when one reaches the other.
        // Unselected descriptors retain their child equality obligations; those
        // later deductions receive the same check before they can close a cycle.
        if self.reaches(a, b) || self.reaches(b, a) {
            self.failed = true;
            return;
        }
        if self.nodes[a].rank < self.nodes[b].rank {
            std::mem::swap(&mut a, &mut b);
        }
        let ad = self.nodes[a].descriptor;
        let bd = self.nodes[b].descriptor;
        if let (Some(x), Some(y)) = (ad, bd) {
            let (x, y) = (&self.descriptors[x], &self.descriptors[y]);
            if x.name != y.name || x.children.len() != y.children.len() {
                self.failed = true;
                return;
            }
            let pairs: Vec<_> = x
                .children
                .iter()
                .copied()
                .zip(y.children.iter().copied())
                .collect();
            for (x, y) in pairs {
                self.equate(x, y);
            }
        }
        let uses = std::mem::take(&mut self.nodes[b].uses);
        let parents = std::mem::take(&mut self.nodes[b].parents);
        self.nodes[b].parent = a;
        if self.nodes[a].rank == self.nodes[b].rank {
            self.nodes[a].rank += 1;
        }
        if ad.is_none() {
            self.nodes[a].descriptor = bd;
        }
        for (id, col) in &uses {
            let occ = &self.occurrences[*id as usize];
            let old = (occ.name.clone(), occ.args.len(), *col, b);
            if let Some(bucket) = self.columns.get_mut(&old) {
                bucket.remove(id);
            }
            self.columns
                .entry((occ.name.clone(), occ.args.len(), *col, a))
                .or_default()
                .insert(*id);
            count!(self, index_migrations);
        }
        self.nodes[a].uses.extend(uses);
        self.nodes[a].parents.extend(parents.iter().copied());
        for descriptor in parents {
            self.repair(descriptor);
        }
        self.wake(a);
    }
    fn repair_step(&mut self) {
        let id = self.repairs.pop_front().unwrap();
        self.repair_set.remove(&id);
        count!(self, descriptor_repairs);
        let descriptor = self.descriptors[id].clone();
        if let Some(old) = descriptor.key
            && let Some(bucket) = self.constructors.get_mut(&old)
        {
            bucket.remove(&id);
        }
        let key = (
            descriptor.name,
            descriptor.children.iter().map(|x| self.root(*x)).collect(),
        );
        let peers = self.constructors.get(&key).cloned().unwrap_or_default();
        self.constructors.entry(key.clone()).or_default().insert(id);
        self.descriptors[id].key = Some(key);
        for peer in peers {
            self.equate(descriptor.owner, self.descriptors[peer].owner);
        }
    }
    fn reaches(&self, start: Value, target: Value) -> bool {
        let mut pending = vec![start];
        let mut visited = BTreeSet::new();
        while let Some(value) = pending.pop() {
            let root = self.root(value);
            if root == target {
                return true;
            }
            if !visited.insert(root) {
                continue;
            }
            if let Some(id) = self.nodes[root].descriptor {
                pending.extend(self.descriptors[id].children.iter().copied());
            }
        }
        false
    }
    fn cyclic(&self) -> bool {
        fn visit(engine: &Engine, value: Value, colors: &mut [u8]) -> bool {
            let root = engine.root(value);
            if colors[root] == 1 {
                return true;
            }
            if colors[root] == 2 {
                return false;
            }
            colors[root] = 1;
            if let Some(d) = engine.nodes[root].descriptor {
                for child in &engine.descriptors[d].children {
                    if visit(engine, *child, colors) {
                        return true;
                    }
                }
            }
            colors[root] = 2;
            false
        }
        let mut colors = vec![0; self.nodes.len()];
        (0..self.nodes.len()).any(|value| visit(self, value, &mut colors))
    }
    fn equal_values(&self, a: Value, b: Value) -> bool {
        let (a, b) = (self.root(a), self.root(b));
        if a == b {
            return true;
        }
        match (self.nodes[a].descriptor, self.nodes[b].descriptor) {
            (Some(x), Some(y)) => {
                let (x, y) = (&self.descriptors[x], &self.descriptors[y]);
                x.name == y.name
                    && x.children.len() == y.children.len()
                    && x.children
                        .iter()
                        .zip(&y.children)
                        .all(|(x, y)| self.equal_values(*x, *y))
            }
            _ => false,
        }
    }
    fn match_term(&self, pattern: &Term, value: Value, env: &mut BTreeMap<Var, Value>) -> bool {
        match pattern {
            Term::Var(var) => match env.get(var) {
                Some(bound) => self.equal_values(*bound, value),
                None => {
                    env.insert(*var, value);
                    true
                }
            },
            Term::App(name, children) => {
                let Some(d) = self.nodes[self.root(value)].descriptor else {
                    return false;
                };
                let d = &self.descriptors[d];
                name == &d.name
                    && children.len() == d.children.len()
                    && children
                        .iter()
                        .zip(&d.children)
                        .all(|(p, v)| self.match_term(p, *v, env))
            }
        }
    }
    fn match_head(&self, head: &Constraint, id: u64, env: &mut BTreeMap<Var, Value>) -> bool {
        let occurrence = &self.occurrences[id as usize];
        occurrence.live
            && head.name == occurrence.name
            && head.args.len() == occurrence.args.len()
            && head
                .args
                .iter()
                .zip(&occurrence.args)
                .all(|(p, v)| self.match_term(p, *v, env))
    }
    // Read-only resolution uses only repaired keys whose current children still
    // equal the key. A stale descriptor must never establish constructor equality.
    fn resolve(&self, term: &Term, env: &BTreeMap<Var, Value>) -> Option<Value> {
        match term {
            Term::Var(v) => env.get(v).map(|v| self.root(*v)),
            Term::App(name, args) => {
                let children: Option<Vec<_>> = args.iter().map(|t| self.resolve(t, env)).collect();
                let key = (name.clone(), children?);
                self.constructors.get(&key)?.iter().find_map(|id| {
                    let d = &self.descriptors[*id];
                    (d.children
                        .iter()
                        .map(|v| self.root(*v))
                        .eq(key.1.iter().copied()))
                    .then(|| self.root(d.owner))
                })
            }
        }
    }
    fn guard_equal(&self, left: &Term, right: &Term, env: &BTreeMap<Var, Value>) -> bool {
        match (left, right) {
            (Term::Var(a), Term::Var(b)) => {
                a == b
                    || match (env.get(a), env.get(b)) {
                        (Some(a), Some(b)) => self.equal_values(*a, *b),
                        _ => false,
                    }
            }
            (Term::App(a, x), Term::App(b, y)) => {
                a == b
                    && x.len() == y.len()
                    && x.iter().zip(y).all(|(a, b)| self.guard_equal(a, b, env))
            }
            (Term::Var(var), term) | (term, Term::Var(var)) => {
                let Some(value) = env.get(var) else {
                    return false;
                };
                self.entails_term(*value, term, env)
            }
        }
    }
    fn entails_term(&self, value: Value, term: &Term, env: &BTreeMap<Var, Value>) -> bool {
        match term {
            Term::Var(v) => env.get(v).is_some_and(|v| self.equal_values(value, *v)),
            Term::App(name, args) => {
                let Some(id) = self.nodes[self.root(value)].descriptor else {
                    return false;
                };
                let d = &self.descriptors[id];
                name == &d.name
                    && args.len() == d.children.len()
                    && args
                        .iter()
                        .zip(&d.children)
                        .all(|(t, v)| self.entails_term(*v, t, env))
            }
        }
    }
    fn heads(&self, rule: usize) -> Vec<&Constraint> {
        let rule = &self.prepared.rules[rule];
        rule.kept.iter().chain(&rule.removed).collect()
    }
    fn eligible(&self, rule: usize, ids: &[u64], anchor: usize) -> Option<BTreeMap<Var, Value>> {
        if ids.iter().copied().collect::<BTreeSet<_>>().len() != ids.len() {
            return None;
        }
        let source = &self.prepared.rules[rule];
        if source.removed.is_empty() && self.history.contains(&(rule, ids.to_vec())) {
            return None;
        }
        let heads = self.heads(rule);
        let mut env = BTreeMap::new();
        if !self.match_head(heads[anchor], ids[anchor], &mut env) {
            return None;
        }
        for (i, head) in heads.iter().enumerate() {
            if i != anchor && !self.match_head(head, ids[i], &mut env) {
                return None;
            }
        }
        if !source.guards.iter().all(|g| match g {
            Guard::Equal(a, b) => self.guard_equal(a, b, &env),
        }) {
            return None;
        }
        Some(env)
    }
    fn build_scan(&mut self, id: u64, rule: usize, anchor: usize) -> Option<Scan> {
        let program = self.prepared.rules.clone();
        let source = &program[rule];
        let heads: Vec<_> = source.kept.iter().chain(&source.removed).collect();
        let mut env = BTreeMap::new();
        if !self.match_head(heads[anchor], id, &mut env) {
            return None;
        }
        let mut lists = Vec::new();
        for (hi, head) in heads.iter().enumerate() {
            if hi == anchor {
                lists.push(vec![id]);
                continue;
            }
            let mut best = self.predicates.get(&(head.name.clone(), head.args.len()));
            for (col, term) in head.args.iter().enumerate() {
                if let Some(value) = self.resolve(term, &env) {
                    count!(self, indexed_lookups);
                    let bucket = self.columns.get(&(
                        head.name.clone(),
                        head.args.len(),
                        col,
                        self.root(value),
                    ));
                    if bucket.map_or(0, BTreeSet::len) < best.map_or(0, BTreeSet::len) {
                        best = bucket;
                    }
                }
            }
            lists.push(
                best.into_iter()
                    .flat_map(|bucket| bucket.iter().copied())
                    .collect(),
            );
        }
        let positions = vec![0; lists.len()];
        Some(Scan {
            rule,
            anchor,
            lists,
            positions,
        })
    }
    fn source_step(&mut self) {
        if self.scan.is_none()
            && let Some((id, rule, head)) = self.activations.pop_front()
        {
            self.activation_set.remove(&(id, rule, head));
            self.scan = self.build_scan(id, rule, head);
        }
        let Some(mut scan) = self.scan.take() else {
            return;
        };
        let Some(ids) = scan.next() else {
            return;
        };
        count!(self, candidate_visits);
        if let Some(mut env) = self.eligible(scan.rule, &ids, scan.anchor) {
            let program = self.prepared.rules.clone();
            let rule = &program[scan.rule];
            if self.trace_enabled {
                self.trace.push(Commit {
                    rule: scan.rule,
                    ids: ids.clone(),
                    pending_equalities: self.equations.len(),
                    pending_repairs: self.repairs.len(),
                });
            }
            count!(self, applications);
            if !self.equations.is_empty() {
                count!(self, speculative_applications);
            }
            if rule.removed.is_empty() {
                self.history.insert((scan.rule, ids.clone()));
            }
            for id in &ids[rule.kept.len()..] {
                self.retire(*id);
            }
            self.body(&rule.body, &mut env);
        }
        if !scan.lists.is_empty() {
            self.scan = Some(scan);
        }
    }
    pub fn advance(&mut self) -> Step {
        if self.failed {
            return Step::Failed;
        }
        if self.complete {
            return Step::Complete;
        }
        count!(self, steps);
        let source = self.scan.is_some() || !self.activations.is_empty();
        let consistency = !self.equations.is_empty() || !self.repairs.is_empty();
        if !source && !consistency {
            self.complete = true;
            return Step::Complete;
        }
        if source && (self.source_turn || !consistency) {
            self.source_step();
            self.source_turn = false;
        } else {
            if !self.repairs.is_empty() && (self.repair_turn || self.equations.is_empty()) {
                self.repair_step();
                self.repair_turn = false;
            } else {
                self.equality_step();
                self.repair_turn = true;
            }
            self.source_turn = true;
        }
        if self.failed {
            Step::Failed
        } else {
            Step::Progress
        }
    }
    pub fn run(&mut self, budget: usize) -> Step {
        for _ in 0..budget {
            let step = self.advance();
            if step != Step::Progress {
                return step;
            }
        }
        if self.failed {
            Step::Failed
        } else if self.complete {
            Step::Complete
        } else {
            Step::Progress
        }
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    /// Enable diagnostic replay recording before the first advance.
    pub fn enable_trace(&mut self) {
        self.trace_enabled = true;
    }
    pub fn trace(&self) -> &[Commit] {
        &self.trace
    }
    pub fn rules(&self) -> &[Rule] {
        &self.prepared.rules
    }
    /// Expensive independent terminal inspection; intentionally outside cost paths.
    pub fn audit(&self) -> Audit {
        let mut inconsistent = self.failed || self.cyclic();
        if !inconsistent && self.equations.is_empty() && self.repairs.is_empty() {
            // Every descriptor, including nonselected descriptors of a merged class,
            // must agree with that class's selected constructor and children.
            for d in &self.descriptors {
                let Some(selected) = self.nodes[self.root(d.owner)].descriptor else {
                    inconsistent = true;
                    break;
                };
                let selected = &self.descriptors[selected];
                if d.name != selected.name
                    || d.children.len() != selected.children.len()
                    || !d
                        .children
                        .iter()
                        .zip(&selected.children)
                        .all(|(a, b)| self.equal_values(*a, *b))
                {
                    inconsistent = true;
                    break;
                }
            }
            // Constructor identity access must be congruence-complete at publication.
            let mut keys = BTreeMap::new();
            for d in &self.descriptors {
                let key = (
                    d.name.clone(),
                    d.children.iter().map(|v| self.root(*v)).collect::<Vec<_>>(),
                );
                if keys
                    .insert(key, self.root(d.owner))
                    .is_some_and(|old| old != self.root(d.owner))
                {
                    inconsistent = true;
                }
            }
        }
        let mut predicates: BTreeMap<Signature, BTreeSet<u64>> = BTreeMap::new();
        let mut columns: BTreeMap<Column, BTreeSet<u64>> = BTreeMap::new();
        let mut uses = vec![BTreeSet::new(); self.nodes.len()];
        for (id, occ) in self.occurrences.iter().enumerate().filter(|(_, o)| o.live) {
            predicates
                .entry((occ.name.clone(), occ.args.len()))
                .or_default()
                .insert(id as u64);
            for (col, arg) in occ.args.iter().enumerate() {
                columns
                    .entry((occ.name.clone(), occ.args.len(), col, self.root(*arg)))
                    .or_default()
                    .insert(id as u64);
                uses[self.root(*arg)].insert((id as u64, col));
            }
        }
        let nonempty_predicates: BTreeMap<_, _> = self
            .predicates
            .iter()
            .filter(|(_, v)| !v.is_empty())
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let nonempty_columns: BTreeMap<_, _> = self
            .columns
            .iter()
            .filter(|(_, v)| !v.is_empty())
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let indexes_valid = predicates == nonempty_predicates
            && columns == nonempty_columns
            && self
                .nodes
                .iter()
                .enumerate()
                .all(|(i, n)| n.uses == uses[i]);
        let mut enabled = 0;
        if !inconsistent {
            // Enumerate directly from occurrence storage, without access indexes or
            // activation queues, so the audit detects lost wakeups and stale keys.
            for ri in 0..self.prepared.rules.len() {
                let lists: Vec<Vec<u64>> = self
                    .heads(ri)
                    .iter()
                    .map(|head| {
                        self.occurrences
                            .iter()
                            .enumerate()
                            .filter(|(_, occ)| {
                                occ.live
                                    && occ.name == head.name
                                    && occ.args.len() == head.args.len()
                            })
                            .map(|(id, _)| id as u64)
                            .collect()
                    })
                    .collect();
                let mut scan = Scan {
                    rule: ri,
                    anchor: 0,
                    positions: vec![0; lists.len()],
                    lists,
                };
                while let Some(ids) = scan.next() {
                    if self.eligible(ri, &ids, 0).is_some() {
                        enabled += 1;
                    }
                }
            }
        }
        let pending_source = self.activations.len() + usize::from(self.scan.is_some());
        Audit {
            pending_equalities: self.equations.len(),
            pending_repairs: self.repairs.len(),
            pending_source,
            pending_bodies: 0,
            enabled_applications: enabled,
            inconsistent,
            indexes_valid,
            live_occurrences: self.occurrences.iter().filter(|o| o.live).count(),
            retained_occurrences: self.occurrences.len(),
            allocated_values: self.nodes.len(),
            value_classes: self
                .nodes
                .iter()
                .enumerate()
                .filter(|(i, n)| *i == n.parent)
                .count(),
            constructor_descriptors: self.descriptors.len(),
            propagation_tokens: self.history.len(),
            applications: self.trace.len(),
            applications_with_pending_equalities: self
                .trace
                .iter()
                .filter(|c| c.pending_equalities > 0)
                .count(),
            complete: !inconsistent
                && indexes_valid
                && enabled == 0
                && pending_source == 0
                && self.equations.is_empty()
                && self.repairs.is_empty(),
        }
    }
    fn export_value(&self, value: Value, free: &mut BTreeMap<Value, Var>) -> Term {
        let root = self.root(value);
        if let Some(d) = self.nodes[root].descriptor {
            let d = &self.descriptors[d];
            Term::App(
                d.name.clone(),
                d.children
                    .iter()
                    .map(|v| self.export_value(*v, free))
                    .collect(),
            )
        } else {
            let next = Var(free.len() as u64);
            Term::Var(*free.entry(root).or_insert(next))
        }
    }
    /// Diagnostic occurrence identities and history are materialized only on demand.
    pub fn view(&self) -> Option<View> {
        let answer = self.answer()?;
        let live = self
            .occurrences
            .iter()
            .enumerate()
            .filter(|(_, o)| o.live)
            .map(|(id, _)| id as u64)
            .zip(answer.residual)
            .collect();
        Some(View {
            outputs: answer.outputs,
            live,
            history: self.history.iter().cloned().collect(),
        })
    }
    /// Joint export preserves aliases between selected terms and every occurrence.
    /// An answer is available only after successful worklist completion.
    pub fn answer(&self) -> Option<Answer> {
        if !self.complete || self.failed {
            return None;
        }
        let mut free = BTreeMap::new();
        let outputs = self
            .outputs
            .iter()
            .map(|(name, v)| (name.clone(), self.export_value(*v, &mut free)))
            .collect();
        let residual = self
            .occurrences
            .iter()
            .filter(|o| o.live)
            .map(|o| Constraint {
                name: o.name.clone(),
                args: o
                    .args
                    .iter()
                    .map(|v| self.export_value(*v, &mut free))
                    .collect(),
            })
            .collect();
        Some(Answer { outputs, residual })
    }
}
