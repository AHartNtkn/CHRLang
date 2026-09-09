//! Complete source execution over shared syntax and context-local equality/claims.
//! Scheduling follows the relational control; this is an experimental competitor.
use crate::{Match, Occurrence, Value, contextual::Store};
use chr_syntax::{Answer, Goal, Guard, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::Arc;

pub struct Prepared {
    rules: Vec<Rule>,
    arrivals: BTreeMap<(String, usize), Vec<usize>>,
}
impl Prepared {
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }
    pub fn new(rules: &[Rule]) -> Result<Arc<Self>, String> {
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
            arrivals,
            rules: rules.to_vec(),
        }))
    }
    pub fn start(self: &Arc<Self>, query: &Query) -> Engine {
        self.start_mode(query, false)
    }
    pub fn start_shared_deductions(self: &Arc<Self>, query: &Query) -> Engine {
        self.start_mode(query, true)
    }
    fn start_mode(self: &Arc<Self>, query: &Query, shared: bool) -> Engine {
        let mut state = State {
            candidates: vec![None; self.rules.len()],
            ..State::default()
        };
        if shared {
            state.store = state.store.with_shared_deductions();
        }
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
        match self.store.descriptions(v).first() {
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
        for (ri, rule) in p.rules.iter().enumerate() {
            if self.candidates[ri].is_none() {
                self.candidates[ri] = Some(self.store.matches(&rule.kept, &rule.removed).into());
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
        let residual = self.store.residual().expect("settled publication");
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
    pub fn advance(&mut self) -> Step {
        let Some(mut state) = self.frontier.pop_front() else {
            return Step::Exhausted;
        };
        if state.store.step() {
            // Equality changes both canonical keys and positive guard entailment.
            // Conservatively invalidate even when this queued deduction is redundant.
            state.candidates.fill(None);
        }
        if state.store.failed() {
            return Step::Progress;
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
                    return Step::Progress;
                }
                Effect::True => (),
                Effect::Fail => return Step::Progress,
            }
        } else if !state.application(&self.prepared) && state.store.pending() == 0 {
            return Step::Answer(state.answer());
        }
        self.frontier.push_back(state);
        Step::Progress
    }
}

#[cfg(test)]
mod shared_deduction_tests {
    use super::*;
    use chr_syntax::{and, atom, c, eq, or, t, v};
    #[test]
    fn source_execution_reuses_post_fork_equality_states() {
        let rules = vec![
            Rule::simplify(
                "start",
                [c("start", [v(0), v(1)])],
                and([
                    or(c("left", []).into(), c("right", []).into()),
                    eq(v(0), v(1)),
                    c("take", []).into(),
                ]),
            ),
            Rule::simplify(
                "take",
                [c("take", []), c("token", [t("f", [atom("a")])])],
                c("done", []).into(),
            ),
        ];
        let q = Query {
            constraints: vec![
                c("start", [t("f", [v(10)]), t("f", [atom("a")])]),
                c("token", [t("f", [v(10)])]),
            ],
            outputs: vec![("x".into(), Var(10))],
        };
        let p = Prepared::new(&rules).unwrap();
        let mut control_trace = None;
        for sharing in [false, true] {
            let mut engine = if sharing {
                p.start_shared_deductions(&q)
            } else {
                p.start(&q)
            };
            let mut observed_reuse = false;
            let mut answers = 0;
            let mut tags = BTreeSet::new();
            let mut trace = vec![];
            for _ in 0..1000 {
                if engine.frontier.len() == 2 {
                    let a = &engine.frontier[0].store;
                    let b = &engine.frontier[1].store;
                    if a.retained_deductions() >= 2
                        && a.pending() == 0
                        && b.pending() == 0
                        && a.shares_equality(b)
                    {
                        observed_reuse = true;
                    }
                }
                match engine.advance() {
                    Step::Answer(a) => {
                        trace.push("answer");
                        assert_eq!(a.outputs, vec![("x".into(), atom("a"))]);
                        let tag = &a.residual[0].name;
                        assert!(tag == "left" || tag == "right");
                        assert_eq!(a.residual, vec![c(tag, []), c("done", [])]);
                        tags.insert(tag.clone());
                        answers += 1;
                    }
                    Step::Exhausted => {
                        trace.push("exhausted");
                        break;
                    }
                    Step::Progress => trace.push("progress"),
                }
            }
            assert_eq!(answers, 2);
            assert_eq!(
                tags,
                BTreeSet::from(["left".to_string(), "right".to_string()])
            );
            assert_eq!(observed_reuse, sharing);
            if let Some(control) = &control_trace {
                assert_eq!(&trace, control);
            } else {
                control_trace = Some(trace);
            }
        }
    }
}
