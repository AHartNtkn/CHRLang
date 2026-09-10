//! Complete source execution over shared syntax and context-local equality/claims.
//! Scheduling follows the relational control; this is an experimental competitor.
#[cfg(test)]
#[allow(dead_code)]
#[path = "../examples/support/deduction_source.rs"]
mod cost_source;
use crate::{
    Match, Occurrence, Value,
    contextual::{MatchCursor, Store},
};
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
        self.start_mode(query, false, false, false, false)
    }
    pub fn start_relevant_deductions(self: &Arc<Self>, query: &Query) -> Engine {
        let mut engine = self.start(query);
        let state = engine.frontier.front_mut().expect("initial source state");
        state.store = state.store.clone().with_relevant_deductions();
        engine
    }
    pub fn start_validated_deductions(self: &Arc<Self>, query: &Query) -> Engine {
        let mut engine = self.start(query);
        let state = engine.frontier.front_mut().expect("initial source state");
        state.store = state.store.clone().with_validated_deductions();
        engine
    }
    pub fn start_persistent_validated_deductions(self: &Arc<Self>, query: &Query) -> Engine {
        let mut engine = self.start_validated_deductions(query);
        let state = engine.frontier.front_mut().expect("initial source state");
        state.store = state.store.clone().with_persistent_equality();
        engine
    }
    pub fn start_persistent_relevant_deductions(self: &Arc<Self>, query: &Query) -> Engine {
        let mut engine = self.start_relevant_deductions(query);
        let state = engine.frontier.front_mut().expect("initial source state");
        state.store = state.store.clone().with_persistent_equality();
        engine
    }
    pub fn start_shared_deductions(self: &Arc<Self>, query: &Query) -> Engine {
        self.start_mode(query, true, false, false, false)
    }
    pub fn start_persistent_equality(self: &Arc<Self>, query: &Query, shared: bool) -> Engine {
        self.start_mode(query, shared, true, false, false)
    }
    pub fn start_demand(self: &Arc<Self>, query: &Query) -> Engine {
        self.start_mode(query, false, false, true, false)
    }
    pub fn start_resumable(self: &Arc<Self>, query: &Query) -> Engine {
        self.start_mode(query, false, false, true, true)
    }
    fn start_mode(
        self: &Arc<Self>,
        query: &Query,
        shared: bool,
        persistent: bool,
        demand: bool,
        resumable: bool,
    ) -> Engine {
        let mut state = State {
            demand,
            resumable,
            cursors: if resumable {
                vec![None; self.rules.len()]
            } else {
                vec![]
            },
            candidates: vec![None; self.rules.len()],
            ..State::default()
        };
        if persistent {
            state.store = state.store.with_persistent_equality();
        }
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
            discovery_stats: DiscoveryStats::default(),
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
    demand: bool,
    resumable: bool,
    cursors: Vec<Option<MatchCursor>>,
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
    fn application(&mut self, p: &Prepared, stats: &mut DiscoveryStats) -> bool {
        for (ri, rule) in p.rules.iter().enumerate() {
            if !self.demand && self.candidates[ri].is_none() {
                self.candidates[ri] = Some(self.store.matches(&rule.kept, &rule.removed).into());
            }
            loop {
                let candidate = if self.resumable {
                    let cursor = self.cursors[ri]
                        .get_or_insert_with(|| MatchCursor::new(&rule.kept, &rule.removed));
                    cursor.next(&self.store, &rule.kept, &rule.removed)
                } else if self.demand {
                    self.store
                        .find_match(&rule.kept, &rule.removed, |candidate| {
                            if cfg!(feature = "local-work") {
                                stats.offered += 1;
                            }
                            let ids = candidate
                                .kept
                                .iter()
                                .chain(&candidate.removed)
                                .copied()
                                .collect::<Vec<_>>();
                            !(rule.removed.is_empty() && self.history.contains(&(ri, ids)))
                                && rule.guards.iter().all(|Guard::Equal(a, b)| {
                                    self.guard(a, &candidate.bindings)
                                        == self.guard(b, &candidate.bindings)
                                })
                        })
                } else {
                    self.candidates[ri].as_mut().unwrap().pop_front()
                };
                let Some(candidate) = candidate else {
                    break;
                };
                if cfg!(feature = "local-work") && (self.resumable || !self.demand) {
                    stats.offered += 1;
                }
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
#[derive(Default, Debug)]
pub struct DiscoveryStats {
    /// Complete candidates offered to history/guard checking; zero without local-work.
    pub offered: u64,
}
pub struct Engine {
    prepared: Arc<Prepared>,
    frontier: VecDeque<State>,
    discovery_stats: DiscoveryStats,
}
impl Engine {
    #[cfg(feature = "deduction-work")]
    pub fn relevant_deduction_hits(&self) -> usize {
        self.frontier
            .front()
            .map_or(0, |s| s.store.relevant_deduction_hits())
    }
    pub fn discovery_stats(&self) -> &DiscoveryStats {
        &self.discovery_stats
    }
    pub fn advance(&mut self) -> Step {
        let Some(mut state) = self.frontier.pop_front() else {
            return Step::Exhausted;
        };
        if state.store.step() {
            // Distinct-root work can change canonical keys, partial constructor
            // descriptions or guards. Same-root work still advances the queue.
            #[cfg(feature = "precise-invalidation")]
            let invalidate = state.store.last_step_changed();
            #[cfg(not(feature = "precise-invalidation"))]
            let invalidate = true;
            if invalidate {
                state.candidates.fill(None);
                state.cursors.fill(None);
            }
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
                            if state.resumable {
                                state.cursors[*ri] = None;
                            }
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
        } else if !state.application(&self.prepared, &mut self.discovery_stats)
            && state.store.pending() == 0
        {
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

#[cfg(test)]
mod deduction_cost_source_tests {
    use super::cost_source as source;
    use super::*;
    #[test]
    fn measured_sources_exercise_reuse_and_adverse_keys() {
        for family in ["single", "shared", "distinct", "changed"] {
            for resource in [false, true] {
                for depth in [0, 8] {
                    let schema = source::Schema::new(family, resource);
                    let p = Prepared::new(&schema.rules()).unwrap();
                    let mut e = p.start_shared_deductions(&schema.query(depth, false));
                    let observer = e.frontier[0].store.clone();
                    let mut count = 0;
                    for _ in 0..10000 {
                        match e.advance() {
                            Step::Answer(_) => count += 1,
                            Step::Exhausted => break,
                            Step::Progress => (),
                        }
                    }
                    assert_eq!(count, schema.branches);
                    let expected = match family {
                        "single" | "shared" => depth + 1,
                        "distinct" => 4 * (depth + 1),
                        "changed" => 4 * (depth + 2),
                        _ => unreachable!(),
                    };
                    assert_eq!(
                        observer.retained_deductions(),
                        expected,
                        "{family} {depth} {resource}"
                    );
                }
            }
        }
    }
}
