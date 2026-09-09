//! Checked private phases followed by state reunion; finite raw-answer contract.
use super::*;
pub struct Outcome {
    pub answers: Vec<Answer>,
    #[cfg(feature = "replay-diagnostic")]
    pub local_steps: usize,
    #[cfg(feature = "replay-diagnostic")]
    pub reunion_steps: usize,
    #[cfg(feature = "replay-diagnostic")]
    pub products: usize,
    pub components: usize,
}
fn owner(c: &Constraint) -> Result<&str, String> {
    match c.args.first() {
        Some(Term::App(n, xs)) if xs.is_empty() => Ok(n),
        _ => Err("ownership key must be a literal atom".into()),
    }
}
fn preserves(g: &Goal, key: &str) -> bool {
    match g {
        Goal::Constraint(c) => owner(c).is_ok_and(|k| k == key),
        Goal::And(gs) => gs.iter().all(|g| preserves(g, key)),
        Goal::Or(a, b) => preserves(a, key) && preserves(b, key),
        _ => true,
    }
}
fn transport(t: &Term, fresh: u64, offset: u64) -> Term {
    match t {
        Term::Var(v) => Term::Var(if v.0 >= fresh { Var(v.0 + offset) } else { *v }),
        Term::App(n, xs) => Term::App(
            n.clone(),
            xs.iter().map(|x| transport(x, fresh, offset)).collect(),
        ),
    }
}
fn combine(left: &State, right: &State, fresh: u64) -> Result<State, String> {
    assert!(left.pending.is_empty() && right.pending.is_empty());
    let mut merged = left.clone();
    let offset = left.next_var - fresh;
    merged.next_var = left
        .next_var
        .checked_add(right.next_var - fresh)
        .ok_or("variable identity exhausted")?;
    merged.next_occ = left
        .next_occ
        .checked_add(right.next_occ)
        .ok_or("occurrence identity exhausted")?;
    for (v, t) in &right.bindings {
        let key = if v.0 >= fresh { Var(v.0 + offset) } else { *v };
        if merged
            .bindings
            .insert(
                key,
                if offset == 0 {
                    t.clone()
                } else {
                    Arc::new(transport(t, fresh, offset))
                },
            )
            .is_some()
        {
            return Err("overlapping component binding owners".into());
        }
    }
    for (id, c) in &right.live {
        merged.live.insert(
            id + left.next_occ,
            if offset == 0 {
                c.clone()
            } else {
                Arc::new(Constraint {
                    name: c.name.clone(),
                    args: c.args.iter().map(|t| transport(t, fresh, offset)).collect(),
                })
            },
        );
    }
    for (rule, ids) in &right.history {
        merged
            .history
            .insert((*rule, ids.iter().map(|id| id + left.next_occ).collect()));
    }
    Ok(merged)
}
/// Rules and the checked private-prefix boundary, reusable across queries.
pub struct PreparedPhase {
    rules: Vec<Rule>,
    local_rules: usize,
}
impl PreparedPhase {
    pub fn new(rules: &[Rule], local_rules: usize) -> Result<Arc<Self>, String> {
        if local_rules == 0 || local_rules >= rules.len() {
            return Err("expected local prefix followed by joining rules".into());
        }
        for (i, r) in rules.iter().enumerate() {
            let heads = r.kept.iter().chain(&r.removed).collect::<Vec<_>>();
            let key = owner(heads.first().ok_or("empty-head rules unsupported")?)?;
            for head in &heads {
                let next = owner(head)?;
                if i < local_rules && next != key {
                    return Err("local rule crosses ownership boundary".into());
                }
            }
            if i < local_rules && !preserves(&r.body, key) {
                return Err("local rule emits foreign or unknown ownership".into());
            }
        }
        Ok(Arc::new(Self {
            rules: rules.to_vec(),
            local_rules,
        }))
    }
    /// Validate each query's variable ownership and create its private states.
    pub fn start(self: &Arc<Self>, query: &Query) -> Result<ReunionEngine, String> {
        let fresh = State::new(query)?.next_var;
        let mut groups: BTreeMap<String, Vec<Constraint>> = BTreeMap::new();
        let mut owners = BTreeMap::new();
        for c in &query.constraints {
            let key = owner(c)?;
            let mut names = BTreeSet::new();
            for t in &c.args {
                vars(t, &mut names);
            }
            for v in names {
                if owners.insert(v, key).is_some_and(|prior| prior != key) {
                    return Err("initial variables cross ownership boundary".into());
                }
            }
            groups.entry(key.to_owned()).or_default().push(c.clone());
        }
        if groups.len() < 2 {
            return Err("at least two component owners required".into());
        }
        let components = groups.len();
        let mut local = VecDeque::new();
        for (component, constraints) in groups.into_values().enumerate() {
            let mut initial = State::new(&Query {
                constraints,
                outputs: vec![],
            })?;
            initial.next_var = fresh;
            local.push_back((component, initial));
        }
        Ok(ReunionEngine {
            prepared: self.clone(),
            outputs: query.outputs.clone(),
            fresh,
            local,
            saved: vec![vec![]; components],
            cursors: VecDeque::new(),
            resumed: VecDeque::new(),
            lane: 0,
            #[cfg(feature = "replay-diagnostic")]
            local_steps: 0,
            #[cfg(feature = "replay-diagnostic")]
            reunion_steps: 0,
            #[cfg(feature = "replay-diagnostic")]
            products: 0,
            error: None,
        })
    }
}

struct ProductCursor {
    fixed: usize,
    limits: Vec<usize>,
    indices: Vec<usize>,
}
impl ProductCursor {
    fn next(&mut self) -> bool {
        for i in (0..self.indices.len()).rev() {
            if i == self.fixed {
                continue;
            }
            self.indices[i] += 1;
            if self.indices[i] < self.limits[i] {
                return true;
            }
            self.indices[i] = 0;
        }
        false
    }
}
/// Owns private alternatives, lazy product cursors and reunited source states.
/// Service alternates the three queues; it is not a constant wall-time quantum.
pub struct ReunionEngine {
    prepared: Arc<PreparedPhase>,
    outputs: Vec<(String, Var)>,
    fresh: u64,
    local: VecDeque<(usize, State)>,
    saved: Vec<Vec<Arc<State>>>,
    cursors: VecDeque<ProductCursor>,
    resumed: VecDeque<State>,
    lane: usize,
    #[cfg(feature = "replay-diagnostic")]
    local_steps: usize,
    #[cfg(feature = "replay-diagnostic")]
    reunion_steps: usize,
    #[cfg(feature = "replay-diagnostic")]
    products: usize,
    error: Option<String>,
}
impl ReunionEngine {
    pub fn advance(&mut self) -> Result<Step, String> {
        if let Some(e) = &self.error {
            return Err(e.clone());
        }
        for _ in 0..3 {
            let lane = self.lane;
            self.lane = (self.lane + 1) % 3;
            match lane {
                0 => {
                    if let Some((component, mut state)) = self.local.pop_front() {
                        #[cfg(feature = "replay-diagnostic")]
                        {
                            self.local_steps += 1;
                        }
                        match state.step(
                            &self.prepared.rules[..self.prepared.local_rules],
                            None,
                            &mut Recorder::new(false),
                        ) {
                            Event::Progress => self.local.push_back((component, state)),
                            Event::Failed => (),
                            Event::Fork(a, b) => {
                                let mut other = state.clone();
                                state.pending.push(a);
                                other.pending.push(b);
                                self.local.push_back((component, state));
                                self.local.push_back((component, other));
                            }
                            Event::Answer => {
                                self.saved[component].push(Arc::new(state));
                                let limits = self.saved.iter().map(Vec::len).collect::<Vec<_>>();
                                if limits.iter().all(|n| *n > 0) {
                                    let mut indices = vec![0; limits.len()];
                                    indices[component] = limits[component] - 1;
                                    self.cursors.push_back(ProductCursor {
                                        fixed: component,
                                        limits,
                                        indices,
                                    });
                                }
                            }
                        }
                        return Ok(Step::Progress);
                    }
                }
                1 => {
                    if let Some(mut cursor) = self.cursors.pop_front() {
                        let mut state = State {
                            next_var: self.fresh,
                            ..State::default()
                        };
                        for (component, index) in cursor.indices.iter().enumerate() {
                            match combine(&state, &self.saved[component][*index], self.fresh) {
                                Ok(next) => state = next,
                                Err(e) => {
                                    self.error = Some(e.clone());
                                    return Err(e);
                                }
                            }
                        }
                        #[cfg(feature = "replay-diagnostic")]
                        {
                            self.products += 1;
                        }
                        self.resumed.push_back(state);
                        if cursor.next() {
                            self.cursors.push_back(cursor);
                        }
                        return Ok(Step::Progress);
                    }
                }
                2 => {
                    if let Some(mut state) = self.resumed.pop_front() {
                        #[cfg(feature = "replay-diagnostic")]
                        {
                            self.reunion_steps += 1;
                        }
                        match state.step(&self.prepared.rules, None, &mut Recorder::new(false)) {
                            Event::Progress => self.resumed.push_back(state),
                            Event::Failed => (),
                            Event::Answer => return Ok(Step::Answer(state.answer(&self.outputs))),
                            Event::Fork(a, b) => {
                                let mut other = state.clone();
                                state.pending.push(a);
                                other.pending.push(b);
                                self.resumed.push_back(state);
                                self.resumed.push_back(other);
                            }
                        }
                        return Ok(Step::Progress);
                    }
                }
                _ => (),
            }
        }
        Ok(Step::Exhausted)
    }
}
impl PreparedPhase {
    /// Complete finite observations within a service budget or report unfinished work.
    pub fn run(self: &Arc<Self>, query: &Query, limit: usize) -> Result<Outcome, String> {
        let mut e = self.start(query)?;
        let mut answers = vec![];
        for _ in 0..limit {
            match e.advance()? {
                Step::Progress => (),
                Step::Answer(a) => answers.push(a),
                Step::Exhausted => {
                    return Ok(Outcome {
                        answers,
                        #[cfg(feature = "replay-diagnostic")]
                        local_steps: e.local_steps,
                        #[cfg(feature = "replay-diagnostic")]
                        reunion_steps: e.reunion_steps,
                        #[cfg(feature = "replay-diagnostic")]
                        products: e.products,
                        components: e.saved.len(),
                    });
                }
            }
        }
        Err("source/service budget exhausted".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chr_syntax::{atom, c};
    #[test]
    fn identity_transport_shares_values_but_relocates_occurrences_and_history() {
        let left = State {
            next_var: 100,
            next_occ: 7,
            ..State::default()
        };
        let value = Arc::new(Term::Var(Var(100)));
        let constraint = Arc::new(c("saved", [atom("right"), Term::Var(Var(100))]));
        let right = State {
            bindings: BTreeMap::from([(Var(1), value.clone())]),
            live: BTreeMap::from([(2, constraint.clone())]),
            history: BTreeSet::from([(0, vec![2])]),
            next_var: 101,
            next_occ: 3,
            ..State::default()
        };
        let merged = combine(&left, &right, 100).unwrap();
        assert!(Arc::ptr_eq(&merged.bindings[&Var(1)], &value));
        assert!(Arc::ptr_eq(&merged.live[&9], &constraint));
        assert!(merged.history.contains(&(0, vec![9])));
        assert_eq!(merged.next_var, 101);
        assert_eq!(merged.next_occ, 10);
    }
    #[test]
    fn cancellation_releases_saved_states_and_prepared_owner() {
        let rules = vec![
            Rule::simplify("unused", [c("unused", [atom("left")])], Goal::True),
            Rule::simplify(
                "join",
                [c("value", [atom("left")]), c("value", [atom("right")])],
                Goal::True,
            ),
        ];
        let q = Query {
            constraints: vec![c("value", [atom("left")]), c("value", [atom("right")])],
            outputs: vec![],
        };
        let mut engine = PreparedPhase::new(&rules, 1).unwrap().start(&q).unwrap();
        let prepared = Arc::downgrade(&engine.prepared);
        for _ in 0..10 {
            engine.advance().unwrap();
            if engine.saved.iter().all(|xs| !xs.is_empty()) {
                break;
            }
        }
        let saved = engine
            .saved
            .iter()
            .map(|xs| Arc::downgrade(&xs[0]))
            .collect::<Vec<_>>();
        assert!(!engine.cursors.is_empty());
        assert!(saved.iter().all(|w| w.upgrade().is_some()));
        drop(engine);
        assert!(prepared.upgrade().is_none());
        assert!(saved.iter().all(|w| w.upgrade().is_none()));
    }
}
