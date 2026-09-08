//! Experimental direct named-term graph and context-local occurrence ownership.
//! The engine module connects this kernel to source rules. IDs belong to their creating arena.
//! Observation requires a caller-established completion boundary; it does not run
//! pending source effects. Recursive traversal is bounded only by the input graph.
pub mod engine;
mod equality;
pub mod words;

use chr_syntax::{Term, Var};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label(usize);

/// A conjunction of selections; absent labels are undemanded, not false.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Context(BTreeMap<Label, bool>);
impl Context {
    pub fn all() -> Self {
        Self::default()
    }
    pub fn select(&self, label: Label, arm: bool) -> Option<Self> {
        if self.0.get(&label).is_some_and(|old| *old != arm) {
            return None;
        }
        let mut result = self.clone();
        result.0.insert(label, arm);
        Some(result)
    }
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let mut result = self.clone();
        for (&label, &arm) in &other.0 {
            result = result.select(label, arm)?;
        }
        Some(result)
    }
    /// Disjoint cubes for `self AND NOT other`. No full assignment enumeration.
    pub fn subtract(&self, other: &Self) -> Vec<Self> {
        if self.intersection(other).is_none() {
            return vec![self.clone()];
        }
        let mut inside = self.clone();
        let mut outside = Vec::new();
        for (&label, &arm) in &other.0 {
            if !inside.0.contains_key(&label) {
                outside.push(inside.select(label, !arm).unwrap());
                inside.0.insert(label, arm);
            }
        }
        outside
    }
    pub fn contains_assignment(&self, values: &[bool]) -> bool {
        self.0
            .iter()
            .all(|(label, arm)| values.get(label.0) == Some(arm))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeId(usize);
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum View {
    Unknown(u64),
    Constructor(String, Vec<NodeId>),
}
enum Node {
    Unknown(u64),
    App(String, Vec<NodeId>),
    Choice(Label, NodeId, NodeId),
}
#[derive(Default)]
pub struct Graph {
    nodes: Vec<Node>,
    births: Vec<Context>,
    bindings: BTreeMap<u64, Vec<(Context, NodeId)>>,
    failed: Vec<Context>,
}
impl Graph {
    /// A fresh dynamic event, even for equal arms. Its activation must include
    /// the ancestors of every selected conditional birth.
    pub fn birth(&mut self, active: Context) -> Label {
        for label in active.0.keys() {
            let parent = &self.births[label.0];
            assert!(
                parent
                    .0
                    .iter()
                    .all(|(key, arm)| active.0.get(key) == Some(arm)),
                "birth activation omits a causal ancestor"
            );
        }
        let label = Label(self.births.len());
        self.births.push(active);
        label
    }
    fn push(&mut self, node: Node) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(node);
        id
    }
    pub fn unknown(&mut self, id: u64) -> NodeId {
        self.push(Node::Unknown(id))
    }
    pub fn app(&mut self, name: &str, children: Vec<NodeId>) -> NodeId {
        assert!(children.iter().all(|id| id.0 < self.nodes.len()));
        self.push(Node::App(name.into(), children))
    }
    pub fn choice(&mut self, label: Label, left: NodeId, right: NodeId) -> NodeId {
        assert!(label.0 < self.births.len());
        assert!(left.0 < self.nodes.len() && right.0 < self.nodes.len());
        self.push(Node::Choice(label, left, right))
    }
    /// Demand the outer constructor only. Child nodes stay shared and opaque.
    /// A conditional node is meaningful only within its birth activation.
    pub fn expose(&self, node: NodeId, context: &Context) -> Vec<(Context, View)> {
        self.live(context)
            .into_iter()
            .flat_map(|context| {
                self.resolve(node, &context)
                    .into_iter()
                    .flat_map(|(context, node)| match &self.nodes[node.0] {
                        Node::Unknown(id) => vec![(context, View::Unknown(*id))],
                        Node::App(name, children) => {
                            vec![(context, View::Constructor(name.clone(), children.clone()))]
                        }
                        Node::Choice(label, left, right) => {
                            let Some(active) = context.intersection(&self.births[label.0]) else {
                                return Vec::new();
                            };
                            [(false, *left), (true, *right)]
                                .into_iter()
                                .flat_map(|(arm, child)| {
                                    active
                                        .select(*label, arm)
                                        .map(|next| self.expose(child, &next))
                                        .unwrap_or_default()
                                })
                                .collect()
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    /// Complete dynamic histories, omitting assignments to inactive births.
    /// Choices unrelated to the observed value still count as raw alternatives.
    pub fn histories(&self, context: &Context) -> Vec<Context> {
        assert!(context.0.keys().all(|label| label.0 < self.births.len()));
        let mut histories = self.live(context);
        for (index, active) in self.births.iter().enumerate() {
            let label = Label(index);
            histories = histories
                .into_iter()
                .flat_map(|history| {
                    if active
                        .0
                        .iter()
                        .all(|(key, arm)| history.0.get(key) == Some(arm))
                    {
                        [false, true]
                            .into_iter()
                            .filter_map(|arm| history.select(label, arm))
                            .collect()
                    } else if history.0.contains_key(&label) {
                        Vec::new()
                    } else {
                        vec![history]
                    }
                })
                .collect();
        }
        histories
    }
    fn terms(&self, roots: &[NodeId], context: &Context) -> Vec<(Context, Vec<Term>)> {
        let mut partial = vec![(context.clone(), Vec::new())];
        for root in roots {
            let mut next = Vec::new();
            for (region, values) in partial {
                for (region, view) in self.expose(*root, &region) {
                    let expanded = match view {
                        View::Unknown(id) => vec![(region, Term::Var(Var(id)))],
                        View::Constructor(name, children) => self
                            .terms(&children, &region)
                            .into_iter()
                            .map(|(r, args)| (r, Term::App(name.clone(), args)))
                            .collect(),
                    };
                    for (region, value) in expanded {
                        let mut extended = values.clone();
                        extended.push(value);
                        next.push((region, extended));
                    }
                }
            }
            partial = next;
        }
        partial
    }
    pub fn observe(&self, roots: &[NodeId], context: &Context) -> Vec<(Context, Vec<Term>)> {
        self.terms(roots, context)
            .into_iter()
            .flat_map(|(region, values)| {
                self.histories(&region)
                    .into_iter()
                    .map(move |history| (history, values.clone()))
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Occurrence(usize);
/// Only occurrence ownership; source values and pending obligations are external.
/// Each occurrence's live cubes are disjoint. No adjacent-cube merging is attempted.
#[derive(Default)]
pub struct Resources {
    live: Vec<Vec<Context>>,
}
impl Resources {
    pub fn post(&mut self, context: Context) -> Occurrence {
        let id = Occurrence(self.live.len());
        self.live.push(vec![context]);
        id
    }
    /// Atomically consume distinct heads wherever all are available.
    /// Failure in one region leaves every head unchanged in that region.
    pub fn consume(&mut self, ids: &[Occurrence], context: &Context) -> Vec<Context> {
        if ids.iter().copied().collect::<BTreeSet<_>>().len() != ids.len() {
            return Vec::new();
        }
        let mut eligible = vec![context.clone()];
        for id in ids {
            eligible = eligible
                .iter()
                .flat_map(|region| {
                    self.live[id.0]
                        .iter()
                        .filter_map(|live| region.intersection(live))
                })
                .collect();
        }
        for id in ids {
            for taken in &eligible {
                self.live[id.0] = self.live[id.0]
                    .iter()
                    .flat_map(|live| live.subtract(taken))
                    .collect();
            }
        }
        eligible
    }
    pub fn available(&self, id: Occurrence) -> &[Context] {
        &self.live[id.0]
    }
}
