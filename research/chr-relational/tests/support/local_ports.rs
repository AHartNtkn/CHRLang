//! Local handle rewriting: no parent forest or relational match tuples.
use chr_syntax::{Answer, Constraint, Goal, Query, Rule, Term, Var, c, t, v};
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::Arc;
#[derive(Clone)]
pub struct Plan {
    pattern: Term,
    output: Var,
    body: Vec<Operation>,
}
#[derive(Clone)]
enum Operation {
    Fail,
    Equation(Term, Term),
    Post(Constraint),
}
impl Plan {
    pub fn compile(rule: &Rule) -> Result<Arc<Self>, &'static str> {
        if !rule.kept.is_empty() || !rule.guards.is_empty() || rule.removed.len() != 2 {
            return Err("requires one take and one token");
        }
        let (head, token) = (&rule.removed[0], &rule.removed[1]);
        if head.name != "take"
            || head.args.len() != 2
            || token.name != "token"
            || !token.args.is_empty()
        {
            return Err("unsupported heads");
        }
        let Term::Var(output) = head.args[1] else {
            return Err("output must be a variable");
        };
        fn contains(t: &Term, needle: Var) -> bool {
            match t {
                Term::Var(v) => *v == needle,
                Term::App(_, xs) => xs.iter().any(|t| contains(t, needle)),
            }
        }
        if contains(&head.args[0], output) {
            return Err("output also occurs in the input pattern");
        }
        fn lower(g: &Goal, ops: &mut Vec<Operation>) -> Result<(), &'static str> {
            match g {
                Goal::True => (),
                Goal::Fail => ops.push(Operation::Fail),
                Goal::Unify(a, b) => ops.push(Operation::Equation(a.clone(), b.clone())),
                Goal::Constraint(c) => ops.push(Operation::Post(c.clone())),
                Goal::And(gs) => {
                    for g in gs {
                        lower(g, ops)?;
                    }
                }
                Goal::Or(..) => return Err("body alternatives require a search owner"),
            }
            Ok(())
        }
        let mut body = vec![];
        lower(&rule.body, &mut body)?;
        Ok(Arc::new(Self {
            pattern: head.args[0].clone(),
            output,
            body,
        }))
    }
    pub fn start(self: &Arc<Self>, query: &Query, mode: DependencyMode) -> Run {
        let mut run = Run::with_dependencies(mode);
        let mut variables = BTreeMap::new();
        for c in &query.constraints {
            run.emit(self, c, &mut variables);
        }
        run.query_outputs = query
            .outputs
            .iter()
            .map(|(name, var)| {
                (
                    name.clone(),
                    run.instantiate(&Term::Var(*var), &mut variables),
                )
            })
            .collect();
        run
    }
}
#[derive(Clone)]
struct Descriptor {
    name: String,
    children: Vec<usize>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DependencyMode {
    #[default]
    Endpoint,
    Filtered,
    Indexed,
}
#[derive(Clone, Default, Debug)]
pub struct DependencyWork {
    pub notifications: usize,
    pub pair_operations: usize,
    pub membership_edits: usize,
    pub subscription_edits: usize,
    pub incident_visits: usize,
    pub incident_edits: usize,
    pub moved_relations: usize,
    pub moved_subscribers: usize,
    pub moved_endpoint_subscriptions: usize,
    pub pattern_nodes: Cell<usize>,
    pub equality_nodes: Cell<usize>,
}
#[derive(Clone, Default)]
struct Node {
    handles: Vec<usize>,
    descriptor: Option<Descriptor>,
    descriptor_watchers: BTreeSet<usize>,
    equality_watchers: BTreeSet<usize>,
    partners: BTreeSet<usize>,
}
#[derive(Default)]
struct Dependencies {
    descriptor: BTreeSet<usize>,
    equality: Option<(usize, usize)>,
}
#[derive(Clone)]
struct Take {
    input: usize,
    output: usize,
    live: bool,
    plan: Arc<Plan>,
    watched: Vec<usize>,
    matched: Option<BTreeMap<Var, usize>>,
    awaiting: Option<(usize, usize)>,
}
#[derive(Clone)]
struct BodyCursor {
    plan: Arc<Plan>,
    env: BTreeMap<Var, usize>,
    next: usize,
}
#[derive(Clone)]
struct Fact {
    name: String,
    args: Vec<usize>,
}
#[derive(Clone, Default)]
pub struct Run {
    mode: DependencyMode,
    body: Option<BodyCursor>,
    facts: Vec<Fact>,
    query_outputs: Vec<(String, usize)>,
    pairs: BTreeMap<(usize, usize), BTreeSet<usize>>,
    pub dependency_work: DependencyWork,
    nodes: Vec<Node>,
    targets: Vec<usize>,
    equations: VecDeque<(usize, usize)>,
    takes: Vec<Take>,
    ready: BTreeSet<usize>,
    tokens: VecDeque<usize>,
    next_token: usize,
    failed: bool,
    // This source gate is diagnostic, never a primary timing build.
    pub repaired_handles: usize,
    pub visited_requests: usize,
    pub consumed_tokens: Vec<usize>,
}
impl Run {
    pub fn with_dependencies(mode: DependencyMode) -> Self {
        Self {
            mode,
            ..Self::default()
        }
    }
    pub fn work_json(&self) -> String {
        let w = &self.dependency_work;
        format!(
            "{{\"inspections\":{},\"notifications\":{},\"pair_operations\":{},\"membership_edits\":{},\"subscription_edits\":{},\"incident_visits\":{},\"incident_edits\":{},\"moved_relations\":{},\"moved_subscribers\":{},\"moved_endpoint_subscriptions\":{},\"pattern_nodes\":{},\"equality_nodes\":{},\"repaired_handles\":{}}}",
            self.visited_requests,
            w.notifications,
            w.pair_operations,
            w.membership_edits,
            w.subscription_edits,
            w.incident_visits,
            w.incident_edits,
            w.moved_relations,
            w.moved_subscribers,
            w.moved_endpoint_subscriptions,
            w.pattern_nodes.get(),
            w.equality_nodes.get(),
            self.repaired_handles
        )
    }
    pub fn subscriptions(&self) -> usize {
        self.nodes
            .iter()
            .map(|n| n.descriptor_watchers.len() + n.equality_watchers.len() + n.partners.len())
            .sum::<usize>()
            + self.pairs.values().map(BTreeSet::len).sum::<usize>()
    }
    pub fn value(&mut self) -> usize {
        let handle = self.targets.len();
        self.targets.push(self.nodes.len());
        self.nodes.push(Node {
            handles: vec![handle],
            ..Node::default()
        });
        handle
    }
    pub fn describe(&mut self, value: usize, name: &str, children: Vec<usize>) {
        let fresh = self.value();
        self.nodes[self.targets[fresh]].descriptor = Some(Descriptor {
            name: name.into(),
            children,
        });
        self.equate(value, fresh);
    }
    pub fn equate(&mut self, left: usize, right: usize) {
        self.equations.push_back((left, right));
    }
    pub fn post(&mut self, plan: &Arc<Plan>, input: usize, output: usize) {
        let id = self.takes.len();
        self.takes.push(Take {
            input,
            output,
            live: true,
            plan: plan.clone(),
            watched: vec![],
            matched: None,
            awaiting: None,
        });
        self.inspect(id);
    }
    pub fn token(&mut self) {
        self.tokens.push_back(self.next_token);
        self.next_token += 1;
    }
    fn equal(&self, a: usize, b: usize, deps: &mut Dependencies) -> bool {
        self.dependency_work
            .equality_nodes
            .set(self.dependency_work.equality_nodes.get() + 1);
        let (a, b) = (self.targets[a], self.targets[b]);
        if a == b {
            return true;
        }
        match (&self.nodes[a].descriptor, &self.nodes[b].descriptor) {
            (Some(x), Some(y)) if x.name == y.name && x.children.len() == y.children.len() => x
                .children
                .iter()
                .zip(&y.children)
                .all(|(&x, &y)| self.equal(x, y, deps)),
            (None, _) | (_, None) => {
                deps.equality = Some((a, b));
                false
            }
            _ => false,
        }
    }
    fn matches(
        &self,
        pattern: &Term,
        handle: usize,
        env: &mut BTreeMap<Var, usize>,
        deps: &mut Dependencies,
    ) -> bool {
        self.dependency_work
            .pattern_nodes
            .set(self.dependency_work.pattern_nodes.get() + 1);
        match pattern {
            Term::Var(v) => match env.get(v) {
                Some(&prior) => self.equal(prior, handle, deps),
                None => {
                    env.insert(*v, handle);
                    true
                }
            },
            Term::App(name, args) => {
                let node = self.targets[handle];
                let Some(d) = self.nodes[node].descriptor.as_ref() else {
                    deps.descriptor.insert(node);
                    return false;
                };
                {
                    d.name == *name
                        && d.children.len() == args.len()
                        && args
                            .iter()
                            .zip(&d.children)
                            .all(|(p, &h)| self.matches(p, h, env, deps))
                }
            }
        }
    }
    fn pair_key(a: usize, b: usize) -> (usize, usize) {
        (a.min(b), a.max(b))
    }
    fn insert_pair(&mut self, a: usize, b: usize, ids: BTreeSet<usize>) {
        assert_ne!(a, b);
        self.dependency_work.pair_operations += 1;
        self.dependency_work.membership_edits += ids.len();
        self.pairs
            .entry(Self::pair_key(a, b))
            .or_default()
            .extend(ids);
        self.nodes[a].partners.insert(b);
        self.nodes[b].partners.insert(a);
        self.dependency_work.incident_edits += 2;
    }
    fn remove_subscription(&mut self, a: usize, b: usize, id: usize) {
        self.dependency_work.pair_operations += 1;
        let key = Self::pair_key(a, b);
        if let Some(ids) = self.pairs.get_mut(&key) {
            self.dependency_work.membership_edits += 1;
            ids.remove(&id);
            if ids.is_empty() {
                self.dependency_work.pair_operations += 1;
                self.pairs.remove(&key);
                self.nodes[a].partners.remove(&b);
                self.nodes[b].partners.remove(&a);
                self.dependency_work.incident_edits += 2;
            }
        }
    }
    fn unsubscribe(&mut self, id: usize) {
        if let Some((a, b)) = self.takes[id].awaiting.take()
            && self.mode == DependencyMode::Indexed
        {
            self.remove_subscription(self.targets[a], self.targets[b], id);
        }
        for handle in std::mem::take(&mut self.takes[id].watched) {
            let node = &mut self.nodes[self.targets[handle]];
            self.dependency_work.subscription_edits += 2;
            node.descriptor_watchers.remove(&id);
            node.equality_watchers.remove(&id);
        }
    }
    fn inspect(&mut self, id: usize) {
        self.unsubscribe(id);
        self.ready.remove(&id);
        if !self.takes[id].live {
            return;
        }
        self.visited_requests += 1;
        let request = &self.takes[id];
        let mut env = BTreeMap::new();
        let mut deps = Dependencies::default();
        let matched = self
            .matches(&request.plan.pattern, request.input, &mut env, &mut deps)
            .then_some(env);
        let is_ready = matched.is_some();
        self.takes[id].matched = matched;
        for &node in &deps.descriptor {
            self.dependency_work.subscription_edits += 1;
            self.nodes[node].descriptor_watchers.insert(id);
        }
        if let Some((a, b)) = deps.equality {
            self.takes[id].awaiting = Some((self.nodes[a].handles[0], self.nodes[b].handles[0]));
            if self.mode == DependencyMode::Indexed {
                self.insert_pair(a, b, BTreeSet::from([id]));
            } else {
                self.dependency_work.subscription_edits += 2;
                self.nodes[a].equality_watchers.insert(id);
                self.nodes[b].equality_watchers.insert(id);
                self.takes[id]
                    .watched
                    .extend([self.nodes[a].handles[0], self.nodes[b].handles[0]]);
            }
        }
        for node in deps.descriptor {
            self.takes[id].watched.push(self.nodes[node].handles[0]);
        }
        if is_ready {
            self.ready.insert(id);
        }
    }
    fn relocate_pairs(
        &mut self,
        old: usize,
        new: usize,
        partners: BTreeSet<usize>,
        gained_descriptor: bool,
        wake: &mut BTreeSet<usize>,
    ) {
        let mut changed = BTreeSet::new();
        for other in partners {
            self.dependency_work.incident_visits += 1;
            self.dependency_work.moved_relations += 1;
            self.dependency_work.pair_operations += 1;
            let ids = self
                .pairs
                .remove(&Self::pair_key(old, other))
                .expect("incident relation missing");
            self.dependency_work.moved_subscribers += ids.len();
            self.nodes[other].partners.remove(&old);
            self.dependency_work.incident_edits += 1;
            if other == new {
                self.dependency_work.notifications += ids.len();
                wake.extend(ids);
            } else {
                self.insert_pair(new, other, ids);
                changed.insert(other);
            }
        }
        // A newly described winner can enable its pre-existing relations too.
        // Otherwise only relocated relations acquired a different endpoint.
        if gained_descriptor {
            changed = self.nodes[new].partners.clone();
        }
        for other in changed {
            self.dependency_work.incident_visits += 1;
            if self.nodes[new].descriptor.is_some() && self.nodes[other].descriptor.is_some() {
                self.dependency_work.pair_operations += 1;
                let ids = &self.pairs[&Self::pair_key(new, other)];
                self.dependency_work.notifications += ids.len();
                wake.extend(ids);
            }
        }
    }
    pub fn assert_dependency_integrity(&self) {
        for (node, n) in self.nodes.iter().enumerate() {
            for &other in &n.partners {
                assert!(self.nodes[other].partners.contains(&node));
                assert!(self.pairs.contains_key(&Self::pair_key(node, other)));
            }
            for &id in &n.descriptor_watchers {
                let r = &self.takes[id];
                assert!(r.live && n.descriptor.is_none());
                assert!(r.watched.iter().any(|&h| self.targets[h] == node));
            }
            for &id in &n.equality_watchers {
                let r = &self.takes[id];
                assert!(r.live);
                let (a, b) = r.awaiting.expect("endpoint watcher without equality");
                assert!(self.targets[a] == node || self.targets[b] == node);
            }
        }
        for (&(a, b), ids) in &self.pairs {
            assert!(a < b && !ids.is_empty());
            assert!(self.nodes[a].descriptor.is_none() || self.nodes[b].descriptor.is_none());
            assert!(self.nodes[a].partners.contains(&b) && self.nodes[b].partners.contains(&a));
            for &id in ids {
                let r = &self.takes[id];
                assert!(r.live);
                let (x, y) = r.awaiting.expect("indexed request without equality");
                assert_eq!(Self::pair_key(self.targets[x], self.targets[y]), (a, b));
            }
        }
        for (id, r) in self.takes.iter().enumerate() {
            if let Some((a, b)) = r.awaiting {
                assert!(r.live);
                let (a, b) = (self.targets[a], self.targets[b]);
                if self.mode == DependencyMode::Indexed {
                    assert!(
                        self.pairs
                            .get(&Self::pair_key(a, b))
                            .is_some_and(|ids| ids.contains(&id))
                    );
                } else {
                    assert!(
                        self.nodes[a].equality_watchers.contains(&id)
                            && self.nodes[b].equality_watchers.contains(&id)
                    );
                }
            }
        }
    }
    fn reaches(&self, start: usize, target: usize) -> bool {
        let mut pending = vec![start];
        let mut seen = BTreeSet::new();
        while let Some(node) = pending.pop() {
            if node == target {
                return true;
            }
            if !seen.insert(node) {
                continue;
            }
            if let Some(d) = &self.nodes[node].descriptor {
                pending.extend(d.children.iter().map(|&h| self.targets[h]));
            }
        }
        false
    }
    fn merge(&mut self, left: usize, right: usize) {
        let (mut a, mut b) = (self.targets[left], self.targets[right]);
        if a == b {
            return;
        }
        if self.reaches(a, b) || self.reaches(b, a) {
            self.failed = true;
            return;
        }
        // Smaller handle sets move into larger ones; every moved handle has a
        // directly rewritten endpoint. No future lookup follows a parent link.
        if self.nodes[a].handles.len() < self.nodes[b].handles.len() {
            std::mem::swap(&mut a, &mut b);
        }
        if let (Some(x), Some(y)) = (&self.nodes[a].descriptor, &self.nodes[b].descriptor)
            && (x.name != y.name || x.children.len() != y.children.len())
        {
            self.failed = true;
            return;
        }
        let gained_descriptor =
            self.nodes[a].descriptor.is_none() && self.nodes[b].descriptor.is_some();
        let loser = std::mem::take(&mut self.nodes[b]);
        for &handle in &loser.handles {
            self.targets[handle] = a;
            self.repaired_handles += 1;
        }
        self.nodes[a].handles.extend(loser.handles);
        self.dependency_work.notifications +=
            self.nodes[a].equality_watchers.len() + loser.equality_watchers.len();
        let mut wake = self.nodes[a].equality_watchers.clone();
        wake.extend(&loser.equality_watchers);
        // Only watchers whose previously unknown class gains a descriptor
        // have new constructor information. Unknown/unknown aliases do not.
        match (&self.nodes[a].descriptor, &loser.descriptor) {
            (None, Some(_)) => {
                self.dependency_work.notifications += self.nodes[a].descriptor_watchers.len();
                wake.extend(&self.nodes[a].descriptor_watchers);
            }
            (Some(_), None) => {
                self.dependency_work.notifications += loser.descriptor_watchers.len();
                wake.extend(&loser.descriptor_watchers);
            }
            _ => (),
        }
        self.dependency_work.moved_endpoint_subscriptions +=
            loser.descriptor_watchers.len() + loser.equality_watchers.len();
        self.nodes[a]
            .descriptor_watchers
            .extend(loser.descriptor_watchers);
        self.nodes[a]
            .equality_watchers
            .extend(loser.equality_watchers);
        match (&self.nodes[a].descriptor, loser.descriptor) {
            (Some(x), Some(y)) => {
                self.equations
                    .extend(x.children.iter().copied().zip(y.children));
            }
            (None, Some(y)) => self.nodes[a].descriptor = Some(y),
            _ => (),
        }
        if self.mode == DependencyMode::Indexed {
            self.relocate_pairs(b, a, loser.partners, gained_descriptor, &mut wake);
        }
        for id in wake {
            if self.mode == DependencyMode::Filtered
                && let Some((x, y)) = self.takes[id].awaiting
            {
                let (x, y) = (self.targets[x], self.targets[y]);
                if x != y
                    && (self.nodes[x].descriptor.is_none() || self.nodes[y].descriptor.is_none())
                {
                    continue;
                }
            }
            self.inspect(id);
        }
    }
    fn instantiate(&mut self, term: &Term, env: &mut BTreeMap<Var, usize>) -> usize {
        match term {
            Term::Var(v) => *env.entry(*v).or_insert_with(|| self.value()),
            Term::App(name, args) => {
                let children = args.iter().map(|t| self.instantiate(t, env)).collect();
                let handle = self.value();
                self.nodes[self.targets[handle]].descriptor = Some(Descriptor {
                    name: name.clone(),
                    children,
                });
                handle
            }
        }
    }
    fn emit(&mut self, plan: &Arc<Plan>, c: &Constraint, env: &mut BTreeMap<Var, usize>) {
        let args = c
            .args
            .iter()
            .map(|t| self.instantiate(t, env))
            .collect::<Vec<_>>();
        match (c.name.as_str(), args.as_slice()) {
            ("take", [input, output]) => self.post(plan, *input, *output),
            ("token", []) => self.token(),
            _ => self.facts.push(Fact {
                name: c.name.clone(),
                args,
            }),
        }
    }
    pub fn body_pending(&self) -> bool {
        self.body.is_some()
    }
    pub fn observe(&self) -> Option<Answer> {
        let handles = self
            .query_outputs
            .iter()
            .map(|(_, h)| *h)
            .collect::<Vec<_>>();
        let mut answer = self.answer(&handles)?;
        for ((name, _), (label, _)) in answer.outputs.iter_mut().zip(&self.query_outputs) {
            *name = label.clone();
        }
        Some(answer)
    }
    pub fn settle(&mut self) {
        for _ in 0..200_000 {
            if self.failed {
                self.body = None;
                return;
            }
            if let Some((a, b)) = self.equations.pop_front() {
                self.merge(a, b);
                continue;
            }
            if let Some(mut cursor) = self.body.take() {
                if cursor.next == cursor.plan.body.len() {
                    continue;
                }
                let plan = Arc::clone(&cursor.plan);
                match &plan.body[cursor.next] {
                    Operation::Fail => {
                        self.failed = true;
                        continue;
                    }
                    Operation::Equation(a, b) => {
                        let a = self.instantiate(a, &mut cursor.env);
                        let b = self.instantiate(b, &mut cursor.env);
                        self.equate(a, b);
                    }
                    Operation::Post(c) => self.emit(&plan, c, &mut cursor.env),
                }
                cursor.next += 1;
                self.body = Some(cursor);
                continue;
            }
            if self.tokens.is_empty() {
                return;
            }
            let Some(id) = self.ready.pop_first() else {
                return;
            };
            if !self.takes[id].live {
                continue;
            }
            let output = self.takes[id].output;
            let plan = Arc::clone(&self.takes[id].plan);
            let mut env = self.takes[id]
                .matched
                .take()
                .expect("ready request has bindings");
            env.insert(plan.output, output);
            self.takes[id].live = false;
            self.unsubscribe(id);
            self.consumed_tokens.push(self.tokens.pop_front().unwrap());
            self.body = Some(BodyCursor { plan, env, next: 0 });
        }
        panic!("local rewrite service cutoff");
    }
    fn term(&self, handle: usize, fuel: usize) -> Term {
        assert!(fuel > 0, "cycle escaped local consistency");
        let node = self.targets[handle];
        match &self.nodes[node].descriptor {
            Some(d) => t(
                &d.name,
                d.children
                    .iter()
                    .map(|&h| self.term(h, fuel - 1))
                    .collect::<Vec<_>>(),
            ),
            None => v(node as u64),
        }
    }
    pub fn answer(&self, outputs: &[usize]) -> Option<Answer> {
        if self.failed {
            return None;
        }
        assert!(self.equations.is_empty());
        assert!(self.body.is_none());
        assert!(self.tokens.is_empty() || self.ready.is_empty());
        let term = |h| self.term(h, self.nodes.len() + 1);
        Some(Answer {
            outputs: outputs
                .iter()
                .enumerate()
                .map(|(i, &h)| (format!("v{i}"), term(h)))
                .collect(),
            residual: self
                .takes
                .iter()
                .filter(|r| r.live)
                .map(|r| c("take", [term(r.input), term(r.output)]))
                .chain(self.tokens.iter().map(|_| c("token", [])))
                .chain(
                    self.facts
                        .iter()
                        .map(|f| c(&f.name, f.args.iter().map(|&h| term(h)).collect::<Vec<_>>())),
                )
                .collect(),
        })
    }
}
