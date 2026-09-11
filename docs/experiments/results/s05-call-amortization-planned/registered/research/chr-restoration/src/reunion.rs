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
fn combine_at(left: &State, right: &State, fresh: u64, old_occ: usize) -> Result<State, String> {
    assert!(left.pending.is_empty() && right.pending.is_empty());
    let mut merged = left.clone();
    let offset = left.next_var - fresh;
    merged.next_var = left
        .next_var
        .checked_add(right.next_var - fresh)
        .ok_or("variable identity exhausted")?;
    merged.next_occ = left
        .next_occ
        .checked_add(right.next_occ - old_occ)
        .ok_or("occurrence identity exhausted")?;
    for (v, t) in &right.bindings {
        let key = if v.0 >= fresh { Var(v.0 + offset) } else { *v };
        let value = if offset == 0 {
            t.clone()
        } else {
            Arc::new(transport(t, fresh, offset))
        };
        if let Some(prior) = merged.bindings.get(&key) {
            if **prior != *value {
                return Err("overlapping component binding owners".into());
            }
        } else {
            merged.bindings.insert(key, value);
        }
    }
    let relocate = |id: usize| {
        if id < old_occ {
            id
        } else {
            id + left.next_occ - old_occ
        }
    };
    for (id, c) in &right.live {
        merged.live.insert(
            relocate(*id),
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
            .insert((*rule, ids.iter().map(|id| relocate(*id)).collect()));
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
            old_occ: 0,
            stop_at_body: false,
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
    old_occ: usize,
    stop_at_body: bool,
    #[cfg(feature = "replay-diagnostic")]
    pub local_steps: usize,
    #[cfg(feature = "replay-diagnostic")]
    pub reunion_steps: usize,
    #[cfg(feature = "replay-diagnostic")]
    products: usize,
    error: Option<String>,
}
enum PhaseEvent {
    Progress,
    Answer(State),
    Boundary(State),
    Exhausted,
}
impl ReunionEngine {
    pub fn advance(&mut self) -> Result<Step, String> {
        match self.advance_phase()? {
            PhaseEvent::Progress => Ok(Step::Progress),
            PhaseEvent::Answer(s) => Ok(Step::Answer(s.answer(&self.outputs))),
            PhaseEvent::Exhausted => Ok(Step::Exhausted),
            PhaseEvent::Boundary(_) => {
                unreachable!("initial-phase engine does not stop at a body boundary")
            }
        }
    }
    fn advance_phase(&mut self) -> Result<PhaseEvent, String> {
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
                        return Ok(PhaseEvent::Progress);
                    }
                }
                1 => {
                    if let Some(mut cursor) = self.cursors.pop_front() {
                        let mut state = State {
                            next_var: self.fresh,
                            next_occ: self.old_occ,
                            ..State::default()
                        };
                        for (component, index) in cursor.indices.iter().enumerate() {
                            match combine_at(
                                &state,
                                &self.saved[component][*index],
                                self.fresh,
                                self.old_occ,
                            ) {
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
                        return Ok(PhaseEvent::Progress);
                    }
                }
                2 => {
                    if let Some(mut state) = self.resumed.pop_front() {
                        #[cfg(feature = "replay-diagnostic")]
                        {
                            self.reunion_steps += 1;
                        }
                        match state.step(&self.prepared.rules, None, &mut Recorder::new(false)) {
                            Event::Progress => {
                                if self.stop_at_body && state.pending.is_empty() {
                                    return Ok(PhaseEvent::Boundary(state));
                                }
                                self.resumed.push_back(state);
                            }
                            Event::Failed => (),
                            Event::Answer => return Ok(PhaseEvent::Answer(state)),
                            Event::Fork(a, b) => {
                                let mut other = state.clone();
                                state.pending.push(a);
                                other.pending.push(b);
                                self.resumed.push_back(state);
                                self.resumed.push_back(other);
                            }
                        }
                        return Ok(PhaseEvent::Progress);
                    }
                }
                _ => (),
            }
        }
        Ok(PhaseEvent::Exhausted)
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
        let merged = combine_at(&left, &right, 100, 0).unwrap();
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

/// A new epoch can share inherited read-only bindings, but not a writable root.
fn partition_live(state: &State) -> Option<BTreeMap<String, Vec<usize>>> {
    if !state.pending.is_empty() {
        return None;
    }
    let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut roots = BTreeMap::new();
    for (id, c) in &state.live {
        let key = state.resolve(c.args.first()?);
        let Term::App(name, args) = key else {
            return None;
        };
        if !args.is_empty() {
            return None;
        }
        let mut names = BTreeSet::new();
        for term in &c.args {
            vars(&state.resolve(term), &mut names);
        }
        for root in names {
            if roots
                .insert(root, name.clone())
                .is_some_and(|prior| prior != name)
            {
                return None;
            }
        }
        groups.entry(name).or_default().push(*id);
    }
    (groups.len() >= 2).then_some(groups)
}
impl PreparedPhase {
    fn epoch(self: &Arc<Self>, state: &State) -> Option<ReunionEngine> {
        let groups = partition_live(state)?;
        let components = groups.len();
        let mut local = VecDeque::new();
        for (component, ids) in groups.into_values().enumerate() {
            let mut private = state.clone();
            private.live.retain(|id, _| ids.binary_search(id).is_ok());
            local.push_back((component, private));
        }
        Some(ReunionEngine {
            prepared: self.clone(),
            outputs: vec![],
            fresh: state.next_var,
            old_occ: state.next_occ,
            stop_at_body: true,
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
    pub fn start_repeated(
        self: &Arc<Self>,
        query: &Query,
    ) -> Result<RepeatedEngine<Eager>, String> {
        self.start_scheduled(query, Eager)
    }
    pub fn start_repeated_with_policy(
        self: &Arc<Self>,
        query: &Query,
        policy: AttemptPolicy,
    ) -> Result<RepeatedEngine<Scheduled>, String> {
        self.start_scheduled(
            query,
            Scheduled {
                policy,
                skip_remaining: 0,
                next_failure_skip: 1,
            },
        )
    }
    fn start_scheduled<S: AttemptSchedule>(
        self: &Arc<Self>,
        query: &Query,
        schedule: S,
    ) -> Result<RepeatedEngine<S>, String> {
        let mut engine = RepeatedEngine {
            prepared: self.clone(),
            outputs: query.outputs.clone(),
            jobs: VecDeque::new(),
            error: None,
            schedule,
            #[cfg(feature = "replay-diagnostic")]
            epochs: 0,
            #[cfg(feature = "replay-diagnostic")]
            coupled_boundaries: 0,
            #[cfg(feature = "replay-diagnostic")]
            work: RepeatedWork::default(),
        };
        engine.admit(State::new(query)?);
        Ok(engine)
    }
}
enum Job {
    Phase(Box<ReunionEngine>),
    Coupled(State),
}
/// Query-wide attempt scheduling. Skipped checks execute the source ordinarily.
#[derive(Clone, Copy, Debug)]
pub enum AttemptPolicy {
    EveryBoundary,
    FixedSkip(usize),
    FailedCheckBackoff { max_skip: usize },
}
/// Static schedule choice keeps eager execution free of adaptive state.
pub trait AttemptSchedule {
    fn skip(&mut self) -> bool;
    fn finished(&mut self, independent: bool);
}
pub struct Eager;
impl AttemptSchedule for Eager {
    fn skip(&mut self) -> bool {
        false
    }
    fn finished(&mut self, _: bool) {}
}
pub struct Scheduled {
    policy: AttemptPolicy,
    skip_remaining: usize,
    next_failure_skip: usize,
}
impl AttemptSchedule for Scheduled {
    fn skip(&mut self) -> bool {
        if self.skip_remaining == 0 {
            false
        } else {
            self.skip_remaining -= 1;
            true
        }
    }
    fn finished(&mut self, independent: bool) {
        match self.policy {
            AttemptPolicy::EveryBoundary => (),
            AttemptPolicy::FixedSkip(skip) => self.skip_remaining = skip,
            AttemptPolicy::FailedCheckBackoff { max_skip } => {
                if independent {
                    self.next_failure_skip = 1;
                } else {
                    self.skip_remaining = self.next_failure_skip.min(max_skip);
                    self.next_failure_skip = self.next_failure_skip.saturating_mul(2).min(max_skip);
                }
            }
        }
    }
}
/// FIFO service of branch-specific epochs and states not currently independent.
#[cfg(feature = "replay-diagnostic")]
#[derive(Default, Debug)]
pub struct RepeatedWork {
    pub private_steps: usize,
    pub coupled_steps: usize,
    pub boundary_checks: usize,
    pub skipped_checks: usize,
    pub inherited_bindings: usize,
    pub inherited_history: usize,
}
pub struct RepeatedEngine<S> {
    prepared: Arc<PreparedPhase>,
    outputs: Vec<(String, Var)>,
    jobs: VecDeque<Job>,
    error: Option<String>,
    schedule: S,
    #[cfg(feature = "replay-diagnostic")]
    pub epochs: usize,
    #[cfg(feature = "replay-diagnostic")]
    pub coupled_boundaries: usize,
    #[cfg(feature = "replay-diagnostic")]
    pub work: RepeatedWork,
}
impl<S: AttemptSchedule> RepeatedEngine<S> {
    fn admit(&mut self, state: State) {
        if self.schedule.skip() {
            #[cfg(feature = "replay-diagnostic")]
            {
                self.work.skipped_checks += 1;
            }
            self.jobs.push_back(Job::Coupled(state));
            return;
        }
        #[cfg(feature = "replay-diagnostic")]
        {
            self.work.boundary_checks += 1;
        }
        if let Some(epoch) = self.prepared.epoch(&state) {
            self.schedule.finished(true);
            #[cfg(feature = "replay-diagnostic")]
            {
                self.epochs += 1;
                self.work.inherited_bindings += state.bindings.len() * epoch.saved.len();
                self.work.inherited_history += state.history.len() * epoch.saved.len();
            }
            self.jobs.push_back(Job::Phase(Box::new(epoch)));
        } else {
            self.schedule.finished(false);
            #[cfg(feature = "replay-diagnostic")]
            {
                self.coupled_boundaries += 1;
            }
            self.jobs.push_back(Job::Coupled(state));
        }
    }
    pub fn advance(&mut self) -> Result<Step, String> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        let result = self.step();
        if let Err(error) = &result {
            self.error = Some(error.clone());
        }
        result
    }
    fn step(&mut self) -> Result<Step, String> {
        let Some(job) = self.jobs.pop_front() else {
            return Ok(Step::Exhausted);
        };
        match job {
            Job::Phase(mut phase) => {
                #[cfg(feature = "replay-diagnostic")]
                let before = (phase.local_steps, phase.reunion_steps);
                let event = phase.advance_phase()?;
                #[cfg(feature = "replay-diagnostic")]
                {
                    self.work.private_steps += phase.local_steps - before.0;
                    self.work.coupled_steps += phase.reunion_steps - before.1;
                }
                if !matches!(event, PhaseEvent::Exhausted) {
                    self.jobs.push_back(Job::Phase(phase));
                }
                match event {
                    PhaseEvent::Boundary(s) => self.admit(s),
                    PhaseEvent::Answer(s) => return Ok(Step::Answer(s.answer(&self.outputs))),
                    _ => (),
                }
            }
            Job::Coupled(mut state) => {
                #[cfg(feature = "replay-diagnostic")]
                {
                    self.work.coupled_steps += 1;
                }
                match state.step(&self.prepared.rules, None, &mut Recorder::new(false)) {
                    Event::Progress => {
                        if state.pending.is_empty() {
                            self.admit(state);
                        } else {
                            self.jobs.push_back(Job::Coupled(state));
                        }
                    }
                    Event::Failed => (),
                    Event::Answer => return Ok(Step::Answer(state.answer(&self.outputs))),
                    Event::Fork(a, b) => {
                        let mut other = state.clone();
                        state.pending.push(a);
                        other.pending.push(b);
                        self.jobs.push_back(Job::Coupled(state));
                        self.jobs.push_back(Job::Coupled(other));
                    }
                }
            }
        }
        Ok(Step::Progress)
    }
}

#[cfg(test)]
mod repeated_tests {
    use super::*;
    use chr_syntax::{atom, c, t, v};
    #[test]
    fn resolved_nested_roots_and_pending_bodies_control_partition() {
        let mut state = State::new(&Query {
            constraints: vec![
                c("x", [atom("left"), t("box", [v(1)])]),
                c("x", [atom("right"), v(2)]),
            ],
            outputs: vec![],
        })
        .unwrap();
        assert!(partition_live(&state).is_some());
        state.bindings.insert(Var(2), Arc::new(t("box", [v(1)])));
        assert!(partition_live(&state).is_none());
        state.bindings.insert(Var(1), Arc::new(atom("ground")));
        assert!(partition_live(&state).is_some());
        state.pending.push(Arc::new(Goal::True));
        assert!(partition_live(&state).is_none());
    }
    #[test]
    fn reunion_preserves_inherited_identity_and_relocates_only_new_ranges() {
        let inherited = BTreeMap::from([(Var(1), Arc::new(atom("ground")))]);
        let history = BTreeSet::from([(9, vec![2, 5])]);
        let left = State {
            bindings: inherited.clone(),
            history: history.clone(),
            live: BTreeMap::from([
                (2, Arc::new(c("old", [atom("left")]))),
                (10, Arc::new(c("new", [atom("left"), v(100)]))),
            ]),
            next_occ: 11,
            next_var: 101,
            ..State::default()
        };
        let mut right = State {
            bindings: inherited,
            history,
            live: BTreeMap::from([
                (5, Arc::new(c("old", [atom("right")]))),
                (10, Arc::new(c("new", [atom("right"), v(100)]))),
            ]),
            next_occ: 11,
            next_var: 101,
            ..State::default()
        };
        right.history.insert((10, vec![5, 10]));
        let merged = combine_at(&left, &right, 100, 10).unwrap();
        assert_eq!(
            merged.live.keys().copied().collect::<Vec<_>>(),
            [2, 5, 10, 11]
        );
        assert_eq!(merged.live[&10].args[1], v(100));
        assert_eq!(merged.live[&11].args[1], v(101));
        assert_eq!(
            merged.history,
            BTreeSet::from([(9, vec![2, 5]), (10, vec![5, 11])])
        );
        assert_eq!(merged.bindings.len(), 1);
        right.bindings.insert(Var(1), Arc::new(atom("conflict")));
        assert!(combine_at(&left, &right, 100, 10).is_err());
    }
    #[test]
    fn repeated_cancellation_releases_queued_phase_ownership() {
        let rules = vec![
            Rule::simplify("unused", [c("unused", [atom("left")])], Goal::True),
            Rule::simplify(
                "join",
                [c("x", [atom("left")]), c("x", [atom("right")])],
                Goal::True,
            ),
        ];
        let p = PreparedPhase::new(&rules, 1).unwrap();
        let weak = Arc::downgrade(&p);
        let q = Query {
            constraints: vec![c("x", [atom("left")]), c("x", [atom("right")])],
            outputs: vec![],
        };
        let mut engine = p.start_repeated(&q).unwrap();
        for _ in 0..4 {
            engine.advance().unwrap();
        }
        let saved = engine
            .jobs
            .iter()
            .filter_map(|job| match job {
                Job::Phase(phase) => Some(
                    phase
                        .saved
                        .iter()
                        .flatten()
                        .map(Arc::downgrade)
                        .collect::<Vec<_>>(),
                ),
                _ => None,
            })
            .flatten()
            .collect::<Vec<_>>();
        assert!(!saved.is_empty());
        drop(p);
        assert!(weak.upgrade().is_some());
        drop(engine);
        assert!(weak.upgrade().is_none());
        assert!(saved.iter().all(|s| s.upgrade().is_none()));
    }
}
