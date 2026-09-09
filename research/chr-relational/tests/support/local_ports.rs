//! Local handle rewriting: no parent forest or relational match tuples.
use chr_syntax::{c, t, v, Answer, Term};
use std::collections::{BTreeSet, VecDeque};
#[derive(Clone)]
struct Descriptor {
    name: String,
    children: Vec<usize>,
}
#[derive(Clone, Default)]
struct Node {
    handles: Vec<usize>,
    descriptor: Option<Descriptor>,
    watchers: Vec<usize>,
}
#[derive(Clone)]
struct Take {
    input: usize,
    output: usize,
    live: bool,
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
    pub fn take(&mut self, input: usize, output: usize) {
        let id = self.takes.len();
        self.takes.push(Take {
            input,
            output,
            live: true,
        });
        let node = self.targets[input];
        self.nodes[node].watchers.push(id);
        // Register this occurrence once; earlier watchers are already ready.
        self.visited_requests += 1;
        if self.nodes[node]
            .descriptor
            .as_ref()
            .is_some_and(|d| d.name == "f" && d.children.len() == 1)
        {
            self.ready.insert(id);
        }
    }
    pub fn token(&mut self) {
        self.tokens.push_back(self.next_token);
        self.next_token += 1;
    }
    fn wake(&mut self, node: usize) {
        if self.nodes[node]
            .descriptor
            .as_ref()
            .is_some_and(|d| d.name == "f" && d.children.len() == 1)
        {
            for &id in &self.nodes[node].watchers {
                self.visited_requests += 1;
                if self.takes[id].live {
                    self.ready.insert(id);
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
            let request = &mut self.takes[id];
            let d = self.nodes[self.targets[request.input]]
                .descriptor
                .as_ref()
                .unwrap();
            assert_eq!(d.name, "f");
            assert_eq!(d.children.len(), 1);
            request.live = false;
            self.consumed_tokens.push(self.tokens.pop_front().unwrap());
            self.equations.push_back((request.output, d.children[0]));
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
