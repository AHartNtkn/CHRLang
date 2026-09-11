//! Complete source execution over relational ownership; choice copies a state.
use crate::{HeadPlan, Match, Occurrence, Value, store::Store};
use chr_syntax::{Answer, Constraint, Goal, Guard, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::Arc;

pub struct Prepared {
    reads: Option<crate::store::MatcherReads>,
    rules: Vec<Rule>,
    plans: Vec<HeadPlan>,
    arrivals: BTreeMap<(String, usize), Vec<usize>>,
}
impl Prepared {
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }
    pub fn new(rules: &[Rule]) -> Result<Arc<Self>, String> {
        Self::prepare(rules, false)
    }
    pub fn new_ready(rules: &[Rule]) -> Result<Arc<Self>, String> {
        Self::prepare(rules, true)
    }
    fn prepare(rules: &[Rule], ready: bool) -> Result<Arc<Self>, String> {
        if rules
            .iter()
            .any(|r| r.kept.is_empty() && r.removed.is_empty())
        {
            return Err("rules require at least one head".into());
        }
        let mut arrivals: BTreeMap<(String, usize), Vec<usize>> = BTreeMap::new();
        for (ri, rule) in rules.iter().enumerate() {
            for head in rule.kept.iter().chain(&rule.removed) {
                let readers = arrivals
                    .entry((head.name.clone(), head.args.len()))
                    .or_default();
                if !readers.contains(&ri) {
                    readers.push(ri);
                }
            }
        }
        Ok(Arc::new(Self {
            reads: ready.then(|| crate::store::MatcherReads::new(rules)),
            arrivals,
            rules: rules.to_vec(),
            plans: rules
                .iter()
                .map(|r| HeadPlan::compile(&r.kept, &r.removed))
                .collect(),
        }))
    }
    pub fn start(self: &Arc<Self>, query: &Query) -> Engine {
        let mut state = State {
            candidates: vec![None; self.rules.len()],
            ..State::default()
        };
        let mut env = BTreeMap::new();
        for c in &query.constraints {
            let args = c
                .args
                .iter()
                .map(|t| state.value(t, &mut env))
                .collect::<Vec<_>>();
            state.store.post(&c.name, &args);
        }
        state.outputs = query
            .outputs
            .iter()
            .map(|(n, v)| (n.clone(), state.value(&Term::Var(*v), &mut env)))
            .collect();
        Engine {
            prepared: self.clone(),
            frontier: VecDeque::from([state]),
        }
    }
}
#[derive(Clone)]
enum Effect {
    Post(String, Vec<Value>),
    Equal(Value, Value),
    And(Vec<Effect>),
    Or(Box<Effect>, Box<Effect>),
    True,
    Fail,
}
#[derive(Default, Clone)]
struct State {
    store: Store,
    pending: Vec<Effect>,
    history: BTreeSet<(usize, Vec<Occurrence>)>,
    outputs: Vec<(String, Value)>,
    candidates: Vec<Option<VecDeque<Match>>>,
}
#[derive(PartialEq, Eq)]
enum GuardTerm {
    Unknown(Value),
    Local(Var),
    App(String, Vec<GuardTerm>),
}
impl State {
    fn value(&mut self, t: &Term, env: &mut BTreeMap<Var, Value>) -> Value {
        match t {
            Term::Var(v) => *env.entry(*v).or_insert_with(|| self.store.unknown()),
            Term::App(n, xs) => {
                let children = xs.iter().map(|t| self.value(t, env)).collect::<Vec<_>>();
                self.store.constructor(n, &children)
            }
        }
    }
    fn effect(&mut self, g: &Goal, env: &mut BTreeMap<Var, Value>) -> Effect {
        match g {
            Goal::True => Effect::True,
            Goal::Fail => Effect::Fail,
            Goal::Constraint(c) => Effect::Post(
                c.name.clone(),
                c.args.iter().map(|t| self.value(t, env)).collect(),
            ),
            Goal::Unify(a, b) => Effect::Equal(self.value(a, env), self.value(b, env)),
            Goal::And(gs) => Effect::And(gs.iter().map(|g| self.effect(g, env)).collect()),
            Goal::Or(a, b) => {
                Effect::Or(Box::new(self.effect(a, env)), Box::new(self.effect(b, env)))
            }
        }
    }
    fn known(&self, v: Value) -> GuardTerm {
        let v = self.store.root(v);
        match self.store.view.descriptors(v).first() {
            None => GuardTerm::Unknown(v),
            Some((n, xs)) => GuardTerm::App(n.clone(), xs.iter().map(|v| self.known(*v)).collect()),
        }
    }
    fn guard(&self, t: &Term, env: &BTreeMap<Var, Value>) -> GuardTerm {
        match t {
            Term::Var(v) => env.get(v).map_or(GuardTerm::Local(*v), |v| self.known(*v)),
            Term::App(n, xs) => {
                GuardTerm::App(n.clone(), xs.iter().map(|t| self.guard(t, env)).collect())
            }
        }
    }
    fn application(&mut self, p: &Prepared) -> bool {
        for (ri, (rule, plan)) in p.rules.iter().zip(&p.plans).enumerate() {
            if self.candidates[ri].is_none() {
                self.candidates[ri] = Some(self.store.matches(plan).matches.into());
            }
            while let Some(candidate) = self.candidates[ri].as_mut().unwrap().pop_front() {
                let ids = candidate
                    .kept
                    .iter()
                    .chain(&candidate.removed)
                    .copied()
                    .collect::<Vec<_>>();
                if rule.removed.is_empty() && self.history.contains(&(ri, ids.clone())) {
                    continue;
                }
                if !rule.guards.iter().all(|Guard::Equal(a, b)| {
                    self.guard(a, &candidate.bindings) == self.guard(b, &candidate.bindings)
                }) {
                    continue;
                }
                if !self.store.consume(&candidate) {
                    continue;
                }
                if rule.removed.is_empty() {
                    self.history.insert((ri, ids));
                }
                let mut env = candidate.bindings;
                let effect = self.effect(&rule.body, &mut env);
                self.pending.push(effect);
                return true;
            }
        }
        false
    }
    fn answer(&self) -> Answer {
        let values = self.outputs.iter().map(|(_, v)| *v).collect::<Vec<_>>();
        let terms = self.store.export(&values).expect("settled publication");
        let mut residual = Vec::new();
        for (key, index) in self.store.view.locations.values() {
            let crate::Relation::Source(name, _) = key else {
                unreachable!()
            };
            let row = &self.store.view.tables[key].rows[*index];
            residual.push(Constraint {
                name: name.clone(),
                args: self.store.export(&row.values).unwrap(),
            });
        }
        Answer {
            outputs: self
                .outputs
                .iter()
                .map(|(n, _)| n.clone())
                .zip(terms)
                .collect(),
            residual,
        }
    }
}
pub enum Step {
    Progress,
    Answer(Answer),
    Exhausted,
}
pub struct Engine {
    prepared: Arc<Prepared>,
    frontier: VecDeque<State>,
}
impl Engine {
    /// Settle matcher-visible equality without batching quiescent output work.
    pub fn advance_selective(&mut self) -> Step {
        self.advance_scheduled::<false>(true, usize::MAX).0
    }
    /// Existing full-settlement experimental control: finish equality before
    /// every source advance, including pending body effects.
    pub fn advance_settled(&mut self) -> Step {
        if let Some(state) = self.frontier.front_mut() {
            while state.store.step() {
                state.candidates.fill(None);
            }
        }
        self.advance()
    }
    /// Preserve fixed rule/tuple priority while allowing unrelated equality to
    /// overlap source effects. The nonzero deduction budget bounds each call;
    /// pending states rotate through the frontier when the budget is exhausted.
    /// Read metadata is owned by `Prepared::new_ready`, so it cannot describe
    /// a different program from the one being executed.
    pub fn advance_ready(&mut self, budget: usize) -> Step {
        assert!(budget > 0, "deduction budget must be nonzero");
        self.advance_scheduled::<true>(true, budget).0
    }
    pub fn advance(&mut self) -> Step {
        self.advance_scheduled::<false>(false, 1).0
    }
    fn advance_scheduled<const DRAIN: bool>(
        &mut self,
        ready: bool,
        budget: usize,
    ) -> (Step, usize) {
        let reads = ready.then(|| {
            self.prepared
                .reads
                .as_ref()
                .expect("use Prepared::new_ready for readiness scheduling")
        });
        let mut used = 0;
        let Some(mut state) = self.frontier.pop_front() else {
            return (Step::Exhausted, used);
        };
        if state.pending.is_empty()
            && let Some(reads) = reads
        {
            while used < budget && state.store.step_for_matching(reads) {
                used += 1;
                state.candidates.fill(None);
            }
            if state.store.failed() {
                return (Step::Progress, used);
            }
            if used == budget {
                self.frontier.push_back(state);
                return (Step::Progress, used);
            }
        }
        if state.store.step() {
            used += 1;
            // Equality changes both canonical keys and positive guard entailment.
            // Conservatively invalidate even when this queued deduction is redundant.
            state.candidates.fill(None);
        }
        if state.store.failed() {
            return (Step::Progress, used);
        }
        if let Some(effect) = state.pending.pop() {
            match effect {
                Effect::Post(n, xs) => {
                    if let Some(readers) = self.prepared.arrivals.get(&(n.clone(), xs.len())) {
                        for ri in readers {
                            state.candidates[*ri] = None;
                        }
                    }
                    state.store.post(&n, &xs);
                }
                Effect::Equal(a, b) => state.store.equate(a, b),
                Effect::And(gs) => state.pending.extend(gs.into_iter().rev()),
                Effect::Or(a, b) => {
                    let mut other = state.clone();
                    other.pending.push(*b);
                    state.pending.push(*a);
                    self.frontier.push_back(state);
                    self.frontier.push_back(other);
                    return (Step::Progress, used);
                }
                Effect::True => (),
                Effect::Fail => return (Step::Progress, used),
            }
        } else if !state.application(&self.prepared) {
            // Matcher-visible equality is settled. With no source effect or
            // enabled application, remaining equality cannot enable a rule.
            if DRAIN && reads.is_some() {
                while used < budget && state.store.step() {
                    used += 1;
                }
            }
            if state.store.failed() {
                return (Step::Progress, used);
            }
            if state.store.pending() == 0 {
                return (Step::Answer(state.answer()), used);
            }
            // A subsequent ready call may reconsider matching after a bounded yield.
            if reads.is_some() {
                state.candidates.fill(None);
            }
        }
        self.frontier.push_back(state);
        (Step::Progress, used)
    }
}

#[cfg(test)]
#[path = "interleaving_tests.rs"]
mod interleaving_tests;
