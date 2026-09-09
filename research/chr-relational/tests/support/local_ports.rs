//! Local handle rewriting: no parent forest or relational match tuples.
use chr_syntax::{c, t, v, Answer, Goal, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
#[derive(Clone)]
pub struct Plan {
    pattern: Term,
    result: Var,
}
impl Plan {
    pub fn compile(rule: &Rule) -> Result<Self, &'static str> {
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
        let Goal::Unify(Term::Var(left), Term::Var(result)) = &rule.body else {
            return Err("body must equate output to a captured variable");
        };
        fn contains(t: &Term, needle: Var) -> bool {
            match t {
                Term::Var(v) => *v == needle,
                Term::App(_, xs) => xs.iter().any(|t| contains(t, needle)),
            }
        }
        if *left != output || contains(&head.args[0], output) || !contains(&head.args[0], *result) {
            return Err("invalid capture/output ownership");
        }
        Ok(Self {
            pattern: head.args[0].clone(),
            result: *result,
        })
    }
}
#[derive(Clone)]
struct Descriptor {
    name: String,
    children: Vec<usize>,
}
#[derive(Clone, Default)]
struct Node {
    handles: Vec<usize>,
    descriptor: Option<Descriptor>,
    watchers: BTreeSet<usize>,
}
#[derive(Clone)]
struct Take {
    input: usize,
    output: usize,
    live: bool,
    plan: Plan,
    watched: Vec<usize>,
    matched: Option<usize>,
}
#[derive(Clone, Default)]
pub struct Run {
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
    pub fn post(&mut self, plan: &Plan, input: usize, output: usize) {
        let id = self.takes.len();
        self.takes.push(Take {
            input,
            output,
            live: true,
            plan: plan.clone(),
            watched: vec![],
            matched: None,
        });
        self.inspect(id);
    }
    pub fn token(&mut self) {
        self.tokens.push_back(self.next_token);
        self.next_token += 1;
    }
    fn equal(&self, a: usize, b: usize, deps: &mut BTreeSet<usize>) -> bool {
        let (a, b) = (self.targets[a], self.targets[b]);
        deps.insert(a);
        deps.insert(b);
        if a == b {
            return true;
        }
        match (&self.nodes[a].descriptor, &self.nodes[b].descriptor) {
            (Some(x), Some(y)) if x.name == y.name && x.children.len() == y.children.len() => x
                .children
                .iter()
                .zip(&y.children)
                .all(|(&x, &y)| self.equal(x, y, deps)),
            _ => false,
        }
    }
    fn matches(
        &self,
        pattern: &Term,
        handle: usize,
        env: &mut BTreeMap<Var, usize>,
        deps: &mut BTreeSet<usize>,
    ) -> bool {
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
                deps.insert(node);
                self.nodes[node].descriptor.as_ref().is_some_and(|d| {
                    d.name == *name
                        && d.children.len() == args.len()
                        && args
                            .iter()
                            .zip(&d.children)
                            .all(|(p, &h)| self.matches(p, h, env, deps))
                })
            }
        }
    }
    fn unsubscribe(&mut self, id: usize) {
        for handle in std::mem::take(&mut self.takes[id].watched) {
            self.nodes[self.targets[handle]].watchers.remove(&id);
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
        let mut deps = BTreeSet::new();
        let matched = self
            .matches(&request.plan.pattern, request.input, &mut env, &mut deps)
            .then(|| env[&request.plan.result]);
        self.takes[id].matched = matched;
        for node in deps {
            self.nodes[node].watchers.insert(id);
            // A registered handle survives a merge even when its old node retires.
            self.takes[id].watched.push(self.nodes[node].handles[0]);
        }
        if matched.is_some() {
            self.ready.insert(id);
        }
    }
    fn wake(&mut self, node: usize) {
        let watchers = self.nodes[node]
            .watchers
            .iter()
            .copied()
            .collect::<Vec<_>>();
        for id in watchers {
            self.inspect(id);
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
        let loser = std::mem::take(&mut self.nodes[b]);
        for &handle in &loser.handles {
            self.targets[handle] = a;
            self.repaired_handles += 1;
        }
        self.nodes[a].handles.extend(loser.handles);
        self.nodes[a].watchers.extend(loser.watchers);
        match (&self.nodes[a].descriptor, loser.descriptor) {
            (Some(x), Some(y)) => {
                if x.name != y.name || x.children.len() != y.children.len() {
                    self.failed = true;
                    return;
                }
                self.equations
                    .extend(x.children.iter().copied().zip(y.children));
            }
            (None, Some(y)) => self.nodes[a].descriptor = Some(y),
            _ => (),
        }
        self.wake(a);
    }
    pub fn settle(&mut self) {
        for _ in 0..200_000 {
            if self.failed {
                return;
            }
            if let Some((a, b)) = self.equations.pop_front() {
                self.merge(a, b);
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
            let captured = self.takes[id].matched.expect("ready request has a capture");
            self.takes[id].live = false;
            self.unsubscribe(id);
            self.consumed_tokens.push(self.tokens.pop_front().unwrap());
            self.equations.push_back((output, captured));
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
                .collect(),
        })
    }
}
