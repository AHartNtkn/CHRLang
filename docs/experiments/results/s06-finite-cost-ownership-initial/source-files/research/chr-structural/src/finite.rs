//! Acyclic constructor descriptions with finite-path equality.
//! Membership filters denote sets; multiplicity comes only from the source state.
use chr_syntax::Term;
pub const COLLECT_METRICS: bool = cfg!(feature = "metrics");
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Transition {
    pub symbol: String,
    pub children: Vec<usize>,
}
impl Transition {
    pub fn new(symbol: &str, children: impl IntoIterator<Item = usize>) -> Self {
        Self {
            symbol: symbol.into(),
            children: children.into_iter().collect(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Grammar {
    states: Vec<Vec<Transition>>,
    membership: Option<(Vec<Vec<Transition>>, Vec<usize>)>,
}
impl Grammar {
    pub fn new(states: Vec<Vec<Transition>>) -> Result<Self, String> {
        fn visit(i: usize, states: &[Vec<Transition>], marks: &mut [u8]) -> Result<(), String> {
            if i >= states.len() {
                return Err("grammar child outside states".into());
            }
            if marks[i] == 1 {
                return Err("cyclic grammar is outside the finite gate".into());
            }
            if marks[i] == 2 {
                return Ok(());
            }
            marks[i] = 1;
            for t in &states[i] {
                for &child in &t.children {
                    visit(child, states, marks)?;
                }
            }
            marks[i] = 2;
            Ok(())
        }
        let mut marks = vec![0; states.len()];
        for i in 0..states.len() {
            visit(i, &states, &mut marks)?;
        }
        Ok(Self {
            states,
            membership: None,
        })
    }
    /// Merge identical bottom-up membership descriptions and duplicate transitions.
    /// Original source alternatives stay intact for derivation counting.
    /// This is structural reduction, not complete language minimization.
    pub fn reduce_membership(mut self) -> Self {
        fn reduce(
            g: &Grammar,
            id: usize,
            mapping: &mut [usize],
            states: &mut Vec<Vec<Transition>>,
            intern: &mut BTreeMap<Vec<Transition>, usize>,
        ) -> usize {
            if mapping[id] != usize::MAX {
                return mapping[id];
            }
            let transitions = g.states[id]
                .iter()
                .map(|tr| Transition {
                    symbol: tr.symbol.clone(),
                    children: tr
                        .children
                        .iter()
                        .map(|&c| reduce(g, c, mapping, states, intern))
                        .collect(),
                })
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let new = if let Some(&id) = intern.get(&transitions) {
                id
            } else {
                let id = states.len();
                states.push(transitions.clone());
                intern.insert(transitions, id);
                id
            };
            mapping[id] = new;
            new
        }
        let mut mapping = vec![usize::MAX; self.states.len()];
        let mut states = vec![];
        let mut intern = BTreeMap::new();
        for id in 0..self.states.len() {
            reduce(&self, id, &mut mapping, &mut states, &mut intern);
        }
        self.membership = Some((states, mapping));
        self
    }
    fn membership_states(&self) -> &[Vec<Transition>] {
        self.membership
            .as_ref()
            .map_or(&self.states, |(states, _)| states)
    }
    fn membership_root(&self, original: usize) -> usize {
        self.membership
            .as_ref()
            .map_or(original, |(_, mapping)| mapping[original])
    }
    pub fn states(&self) -> &[Vec<Transition>] {
        &self.states
    }
    pub fn multiplicity(&self, state: usize, term: &Term) -> Result<u128, String> {
        // None means a positive count exceeds u128. Zero still annihilates it:
        // an impossible sibling makes the entire derivation contribute zero.
        fn count(
            g: &Grammar,
            state: usize,
            term: &Term,
            memo: &mut BTreeMap<(usize, usize), Option<u128>>,
        ) -> Option<u128> {
            let key = (state, term as *const Term as usize);
            if let Some(n) = memo.get(&key) {
                return *n;
            }
            let Term::App(symbol, args) = term else {
                unreachable!("solver emits ground trees");
            };
            let mut total = Some(0u128);
            for transition in &g.states[state] {
                if transition.symbol != *symbol || transition.children.len() != args.len() {
                    continue;
                }
                let mut product = Some(1u128);
                for (&child, arg) in transition.children.iter().zip(args) {
                    let n = count(g, child, arg, memo);
                    if n == Some(0) {
                        product = Some(0);
                        break;
                    }
                    product = product.zip(n).and_then(|(a, b)| a.checked_mul(b));
                }
                total = total.zip(product).and_then(|(a, b)| a.checked_add(b));
            }
            memo.insert(key, total);
            total
        }
        count(self, state, term, &mut BTreeMap::new())
            .ok_or_else(|| "source multiplicity overflow".into())
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub roots: Vec<usize>,
    pub equalities: Vec<(Vec<usize>, Vec<usize>)>,
}
#[derive(Debug)]
pub struct Answer {
    pub term: Term,
    pub multiplicity: u128,
}
#[derive(Debug)]
pub struct Batch {
    pub answers: Vec<Answer>,
    pub exhausted: bool,
}
#[derive(Default, Debug)]
pub struct Stats {
    pub requirements: u64,
    pub transition_trials: u64,
    pub duplicate_values: u64,
    pub emitted: u64,
    pub peak_frontier: usize,
}
#[derive(Clone, Default)]
struct Position {
    parent: usize,
    children: BTreeMap<usize, usize>,
    tag: Option<(String, usize)>,
    required: BTreeSet<usize>,
    done: BTreeSet<usize>,
}
#[derive(Clone)]
struct State {
    nodes: Vec<Position>,
}
impl State {
    fn fresh(&mut self) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Position {
            parent: id,
            ..Default::default()
        });
        id
    }
    fn root(&self, mut id: usize) -> usize {
        while self.nodes[id].parent != id {
            id = self.nodes[id].parent;
        }
        id
    }
    fn child(&mut self, id: usize, index: usize) -> usize {
        let id = self.root(id);
        if let Some(&child) = self.nodes[id].children.get(&index) {
            return self.root(child);
        }
        let child = self.fresh();
        self.nodes[id].children.insert(index, child);
        child
    }
    fn path(&mut self, path: &[usize]) -> usize {
        let mut id = 0;
        for &index in path {
            id = self.child(id, index);
        }
        self.root(id)
    }
    // Used before grammar requirements/constructors are installed. Equal subtrees
    // imply equality of corresponding demanded children, including earlier paths.
    fn merge(&mut self, a: usize, b: usize) {
        let mut pending = vec![(a, b)];
        while let Some((a, b)) = pending.pop() {
            let (a, b) = (self.root(a), self.root(b));
            if a == b {
                continue;
            }
            let (keep, lose) = (a.min(b), a.max(b));
            self.nodes[lose].parent = keep;
            for (index, child) in std::mem::take(&mut self.nodes[lose].children) {
                if let Some(&other) = self.nodes[keep].children.get(&index) {
                    pending.push((child, other));
                } else {
                    self.nodes[keep].children.insert(index, child);
                }
            }
        }
    }
    fn cyclic(&self) -> bool {
        fn visit(s: &State, id: usize, marks: &mut [u8]) -> bool {
            let id = s.root(id);
            if marks[id] == 1 {
                return true;
            }
            if marks[id] == 2 {
                return false;
            }
            marks[id] = 1;
            if s.nodes[id].children.values().any(|&c| visit(s, c, marks)) {
                return true;
            }
            marks[id] = 2;
            false
        }
        visit(self, 0, &mut vec![0; self.nodes.len()])
    }
    fn apply(&mut self, id: usize, transition: &Transition) -> bool {
        let arity = transition.children.len();
        let n = &mut self.nodes[id];
        if n.children.keys().any(|&i| i >= arity) {
            return false;
        }
        if let Some((name, old_arity)) = &n.tag {
            if name != &transition.symbol || *old_arity != arity {
                return false;
            }
        } else {
            n.tag = Some((transition.symbol.clone(), arity));
        }
        for (index, &state) in transition.children.iter().enumerate() {
            let child = self.child(id, index);
            self.nodes[child].required.insert(state);
        }
        true
    }
    fn materialize(&self, id: usize) -> Term {
        let node = &self.nodes[self.root(id)];
        let (name, arity) = node
            .tag
            .as_ref()
            .expect("all reachable constructors serviced");
        Term::App(
            name.clone(),
            (0..*arity)
                .map(|i| self.materialize(node.children[&i]))
                .collect(),
        )
    }
}
/// Query-owned search. Dropping it releases frontier and exact-observation history.
/// One service unit processes one grammar requirement or one complete value.
pub struct Search<'g> {
    grammar: &'g Grammar,
    source: usize,
    frontier: Vec<State>,
    seen: BTreeSet<Term>,
    stats: Stats,
    error: Option<String>,
}
impl<'g> Search<'g> {
    pub fn new(grammar: &'g Grammar, request: Request) -> Result<Self, String> {
        if request.roots.is_empty() || request.roots.iter().any(|&r| r >= grammar.states.len()) {
            return Err("request needs valid source and filter states".into());
        }
        let mut state = State { nodes: vec![] };
        state.fresh();
        for (a, b) in request.equalities {
            let a = state.path(&a);
            let b = state.path(&b);
            state.merge(a, b);
        }
        state.nodes[0]
            .required
            .extend(request.roots.iter().map(|&r| grammar.membership_root(r)));
        let frontier = if state.cyclic() { vec![] } else { vec![state] };
        Ok(Self {
            grammar,
            source: request.roots[0],
            frontier,
            seen: BTreeSet::new(),
            stats: Stats::default(),
            error: None,
        })
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn advance(&mut self, budget: usize) -> Result<Batch, String> {
        if let Some(e) = &self.error {
            return Err(e.clone());
        }
        let mut answers = vec![];
        for _ in 0..budget {
            let Some(mut state) = self.frontier.pop() else {
                break;
            };
            let grammar = self.grammar;
            let pending = state
                .nodes
                .iter()
                .enumerate()
                .filter(|(i, n)| *i == n.parent)
                .flat_map(|(id, n)| {
                    n.required
                        .difference(&n.done)
                        .map(move |&r| (grammar.membership_states()[r].len(), id, r))
                })
                .min();
            if let Some((_, id, required)) = pending {
                if COLLECT_METRICS {
                    self.stats.requirements += 1;
                }
                state.nodes[id].done.insert(required);
                for transition in self.grammar.membership_states()[required].iter().rev() {
                    if COLLECT_METRICS {
                        self.stats.transition_trials += 1;
                    }
                    let mut child = state.clone();
                    if child.apply(id, transition) {
                        self.frontier.push(child);
                    }
                }
                if COLLECT_METRICS {
                    self.stats.peak_frontier = self.stats.peak_frontier.max(self.frontier.len());
                }
            } else {
                let term = state.materialize(0);
                if self.seen.insert(term.clone()) {
                    let multiplicity = match self.grammar.multiplicity(self.source, &term) {
                        Ok(n) => n,
                        Err(e) => {
                            self.error = Some(e.clone());
                            return Err(e);
                        }
                    };
                    assert!(multiplicity > 0, "source membership preserved");
                    if COLLECT_METRICS {
                        self.stats.emitted += 1;
                    }
                    answers.push(Answer { term, multiplicity });
                } else {
                    if COLLECT_METRICS {
                        self.stats.duplicate_values += 1;
                    }
                }
            }
        }
        Ok(Batch {
            answers,
            exhausted: self.frontier.is_empty(),
        })
    }
}
