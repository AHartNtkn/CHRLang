//! Ordered source-head selection over the local equality graph.
use super::*;
pub struct Program {
    rules: Vec<Rule>,
}
impl Program {
    pub fn compile(rules: &[Rule]) -> Result<Arc<Self>, &'static str> {
        fn body(g: &Goal) -> bool {
            match g {
                Goal::Or(..) => false,
                Goal::And(xs) => xs.iter().all(body),
                _ => true,
            }
        }
        if rules.iter().any(|r| {
            r.kept.is_empty() && r.removed.is_empty() || !r.guards.is_empty() || !body(&r.body)
        }) {
            return Err("requires nonempty heads, no guards and deterministic bodies");
        }
        Ok(Arc::new(Self {
            rules: rules.to_vec(),
        }))
    }
    pub fn start(self: &Arc<Self>, q: &Query) -> Execution {
        self.start_mode::<true>(q, false)
    }
    pub fn start_mode<const METRICS: bool>(
        self: &Arc<Self>,
        q: &Query,
        selective: bool,
    ) -> Execution<METRICS> {
        self.start_impl(q, selective, false, false)
    }
    pub fn start_partial<const METRICS: bool>(self: &Arc<Self>, q: &Query) -> Execution<METRICS> {
        self.start_impl(q, true, true, false)
    }
    pub fn start_intermediate<const METRICS: bool>(
        self: &Arc<Self>,
        q: &Query,
    ) -> Execution<METRICS> {
        self.start_impl(q, true, true, true)
    }
    fn start_impl<const METRICS: bool>(
        self: &Arc<Self>,
        q: &Query,
        selective: bool,
        partial: bool,
        intermediate: bool,
    ) -> Execution<METRICS> {
        let mut e = Execution {
            program: self.clone(),
            graph: Run::with_dependencies(DependencyMode::Indexed),
            live: BTreeMap::new(),
            next_occ: 0,
            history: BTreeSet::new(),
            outputs: vec![],
            firings: 0,
            cache: selective.then(Cache::default),
            partial,
            intermediate,
            new_occ: vec![],
            changed: BTreeSet::new(),
            work: Work::default(),
        };
        let mut env = BTreeMap::new();
        for c in &q.constraints {
            e.post(c, &mut env);
        }
        for (name, v) in &q.outputs {
            let h = e.graph.instantiate(&Term::Var(*v), &mut env);
            e.outputs.push((name.clone(), h));
        }
        if selective {
            e.register(None);
        }
        e.new_occ.clear();
        e
    }
}
type Tuple = (usize, Vec<usize>);
type Env = BTreeMap<Var, usize>;
#[derive(Clone)]
struct Waiting {
    descriptors: Vec<usize>,
    equality: Option<(usize, usize)>,
}
#[derive(Clone, Default)]
struct Cache {
    tuples: BTreeMap<Tuple, Vec<usize>>,
    ready: BTreeMap<Tuple, Env>,
    prefixes: BTreeMap<Tuple, Env>,
    conditions: BTreeMap<Tuple, Waiting>,
    watchers: BTreeMap<usize, BTreeSet<Tuple>>,
    incident: BTreeMap<usize, BTreeSet<Tuple>>,
}
#[derive(Clone, Default, Debug)]
pub struct Work {
    pub head_attempts: Cell<usize>,
    pub intermediate_visits: Cell<usize>,
    pub fact_visits: Cell<usize>,
    pub combinations: Cell<usize>,
    pub inspections: usize,
    pub registrations: usize,
    pub notifications: usize,
    pub wakeups: usize,
    pub changed_handles: usize,
    pub peak_tuples: usize,
}
#[derive(Clone)]
pub struct Execution<const METRICS: bool = true> {
    program: Arc<Program>,
    graph: Run<METRICS>,
    live: BTreeMap<usize, Fact>,
    next_occ: usize,
    history: BTreeSet<(usize, Vec<usize>)>,
    outputs: Vec<(String, usize)>,
    pub firings: usize,
    cache: Option<Cache>,
    partial: bool,
    intermediate: bool,
    new_occ: Vec<usize>,
    changed: BTreeSet<usize>,
    pub work: Work,
}
impl<const METRICS: bool> Execution<METRICS> {
    pub fn matching_work(&self) -> (usize, usize) {
        (
            self.graph.dependency_work.pattern_nodes.get(),
            self.graph.dependency_work.equality_nodes.get(),
        )
    }
    fn post(&mut self, c: &Constraint, env: &mut BTreeMap<Var, usize>) {
        let args = c
            .args
            .iter()
            .map(|t| self.graph.instantiate(t, env))
            .collect();
        self.live.insert(
            self.next_occ,
            Fact {
                name: c.name.clone(),
                args,
            },
        );
        if self.cache.is_some() {
            self.new_occ.push(self.next_occ);
        }
        self.next_occ += 1;
    }
    fn select(
        &self,
        rule: usize,
        heads: &[&Constraint],
        ids: &mut Vec<usize>,
        env: BTreeMap<Var, usize>,
    ) -> Option<(Vec<usize>, BTreeMap<Var, usize>)> {
        if heads.is_empty() {
            let enabled = !self.history.contains(&(rule, ids.clone()))
                && self.program.rules[rule]
                    .guards
                    .iter()
                    .all(|guard| match guard {
                        chr_syntax::Guard::Equal(a, b) => {
                            search::guard_equal(&self.graph, a, b, &env)
                        }
                    });
            return enabled.then(|| (ids.clone(), env));
        }
        for (&id, fact) in &self.live {
            if METRICS {
                self.work.fact_visits.set(self.work.fact_visits.get() + 1);
            }
            let head = heads[0];
            if ids.contains(&id) || head.name != fact.name || head.args.len() != fact.args.len() {
                continue;
            }
            if METRICS {
                self.work
                    .head_attempts
                    .set(self.work.head_attempts.get() + 1);
            }
            let mut next = env.clone();
            let mut deps = Dependencies::default();
            if !head
                .args
                .iter()
                .zip(&fact.args)
                .all(|(p, &h)| self.graph.matches(p, h, &mut next, &mut deps))
            {
                continue;
            }
            ids.push(id);
            if let Some(hit) = self.select(rule, &heads[1..], ids, next) {
                return Some(hit);
            }
            ids.pop();
        }
        None
    }
    fn body(&mut self, g: &Goal, env: &mut BTreeMap<Var, usize>) {
        if self.graph.failed {
            return;
        }
        match g {
            Goal::True => (),
            Goal::Fail => self.graph.failed = true,
            Goal::Constraint(c) => self.post(c, env),
            Goal::Unify(a, b) => {
                let a = self.graph.instantiate(a, env);
                let b = self.graph.instantiate(b, env);
                self.graph.equate(a, b);
                if self.cache.is_some() {
                    while !self.graph.failed {
                        let Some((a, b)) = self.graph.equations.pop_front() else {
                            break;
                        };
                        let (a_node, b_node) = (self.graph.targets[a], self.graph.targets[b]);
                        if a_node != b_node {
                            for &h in self.graph.nodes[a_node]
                                .handles
                                .iter()
                                .chain(&self.graph.nodes[b_node].handles)
                            {
                                if METRICS {
                                    self.work.changed_handles += 1;
                                }
                                self.changed.insert(h);
                            }
                        }
                        self.graph.merge(a, b);
                    }
                } else {
                    self.graph.settle();
                }
            }
            Goal::And(gs) => {
                for g in gs {
                    self.body(g, env);
                }
            }
            Goal::Or(..) => unreachable!("checked deterministic source"),
        }
    }
    /// One complete source firing, or quiescence/failure. Bodies remain atomic.
    pub fn advance(&mut self) -> bool {
        if self.graph.failed {
            return true;
        }
        if self.cache.is_some() {
            return self.advance_selective();
        }
        let program = self.program.clone();
        for (i, r) in program.rules.iter().enumerate() {
            let heads = r.kept.iter().chain(&r.removed).collect::<Vec<_>>();
            if let Some((ids, mut env)) = self.select(i, &heads, &mut vec![], BTreeMap::new()) {
                self.history.insert((i, ids.clone()));
                for id in &ids[r.kept.len()..] {
                    self.live.remove(id);
                }
                if METRICS {
                    self.firings += 1;
                }
                self.body(&r.body, &mut env);
                return false;
            }
        }
        true
    }
    pub fn answer(&self) -> Option<Answer> {
        if self.graph.failed {
            return None;
        }
        let term = |h| self.graph.term(h, self.graph.nodes.len() + 1);
        Some(Answer {
            outputs: self
                .outputs
                .iter()
                .map(|(n, h)| (n.clone(), term(*h)))
                .collect(),
            residual: self
                .live
                .values()
                .map(|f| c(&f.name, f.args.iter().map(|h| term(*h)).collect::<Vec<_>>()))
                .collect(),
        })
    }
}

impl<const METRICS: bool> Execution<METRICS> {
    fn combinations(
        &self,
        rule: usize,
        heads: &[&Constraint],
        ids: &mut Vec<usize>,
        anchor: Option<(usize, usize)>,
        out: &mut Vec<Tuple>,
    ) {
        if heads.is_empty() {
            if METRICS {
                self.work.combinations.set(self.work.combinations.get() + 1);
            }
            out.push((rule, ids.clone()));
            return;
        }
        use std::ops::Bound::{Included, Unbounded};
        let bounds = match anchor {
            Some((position, id)) if position == ids.len() => (Included(id), Included(id)),
            _ => (Unbounded, Unbounded),
        };
        for (&id, f) in self.live.range(bounds) {
            if METRICS {
                self.work.fact_visits.set(self.work.fact_visits.get() + 1);
            }
            if ids.contains(&id) || heads[0].name != f.name || heads[0].args.len() != f.args.len() {
                continue;
            }
            ids.push(id);
            self.combinations(rule, &heads[1..], ids, anchor, out);
            ids.pop();
        }
    }
    fn register(&mut self, required: Option<usize>) {
        if self.partial {
            self.register_prefixes(required);
            return;
        }
        let mut keys = vec![];
        for (i, r) in self.program.rules.iter().enumerate() {
            let heads = r.kept.iter().chain(&r.removed).collect::<Vec<_>>();
            if let Some(id) = required {
                let fact = &self.live[&id];
                for (position, head) in heads.iter().enumerate() {
                    if head.name == fact.name && head.args.len() == fact.args.len() {
                        self.combinations(i, &heads, &mut vec![], Some((position, id)), &mut keys);
                    }
                }
            } else {
                self.combinations(i, &heads, &mut vec![], None, &mut keys);
            }
        }
        for key in keys {
            self.add_entry(key);
        }
    }
    fn add_entry(&mut self, key: Tuple) {
        let cache = self.cache.as_mut().unwrap();
        if cache.tuples.contains_key(&key) || self.history.contains(&key) {
            return;
        }
        for id in &key.1 {
            cache.incident.entry(*id).or_default().insert(key.clone());
        }
        cache.tuples.insert(key.clone(), vec![]);
        if METRICS {
            self.work.registrations += 1;
            self.work.peak_tuples = self.work.peak_tuples.max(cache.tuples.len());
        }
        self.inspect_tuple(&key);
    }
    fn unsubscribe_tuple(&mut self, key: &Tuple) {
        let cache = self.cache.as_mut().unwrap();
        cache.conditions.remove(key);
        let handles = std::mem::take(cache.tuples.get_mut(key).unwrap());
        for h in handles {
            let xs = cache.watchers.get_mut(&h).unwrap();
            xs.remove(key);
            if xs.is_empty() {
                cache.watchers.remove(&h);
            }
        }
    }
    fn inspect_tuple(&mut self, key: &Tuple) {
        self.unsubscribe_tuple(key);
        if METRICS {
            self.work.inspections += 1;
        }
        let r = &self.program.rules[key.0];
        let (skip, mut env) = if self.partial && key.1.len() > 1 {
            let parent = (key.0, key.1[..key.1.len() - 1].to_vec());
            (
                key.1.len() - 1,
                self.cache.as_ref().unwrap().prefixes[&parent].clone(),
            )
        } else {
            (0, Env::new())
        };
        let mut deps = Dependencies::default();
        let success = r
            .kept
            .iter()
            .chain(&r.removed)
            .zip(&key.1)
            .skip(skip)
            .all(|(head, id)| {
                if METRICS {
                    self.work
                        .head_attempts
                        .set(self.work.head_attempts.get() + 1);
                }
                head.args
                    .iter()
                    .zip(&self.live[id].args)
                    .all(|(p, &h)| self.graph.matches(p, h, &mut env, &mut deps))
            });
        let cache = self.cache.as_mut().unwrap();
        if success {
            if self.partial && key.1.len() < r.kept.len() + r.removed.len() {
                cache.prefixes.insert(key.clone(), env);
                self.extend_prefix(key, None);
            } else {
                cache.ready.insert(key.clone(), env);
            }
            return;
        }
        let descriptor_handles = deps
            .descriptor
            .iter()
            .map(|&n| self.graph.nodes[n].handles[0])
            .collect();
        let pair = deps.equality.map(|(a, b)| {
            (
                self.graph.nodes[a].handles[0],
                self.graph.nodes[b].handles[0],
            )
        });
        cache.conditions.insert(
            key.clone(),
            Waiting {
                descriptors: descriptor_handles,
                equality: pair,
            },
        );
        let mut nodes = deps.descriptor;
        if let Some((a, b)) = deps.equality {
            nodes.insert(a);
            nodes.insert(b);
        }
        let handles = nodes
            .into_iter()
            .map(|n| self.graph.nodes[n].handles[0])
            .collect::<BTreeSet<_>>();
        for &h in &handles {
            cache.watchers.entry(h).or_default().insert(key.clone());
        }
        *cache.tuples.get_mut(key).unwrap() = handles.into_iter().collect();
    }
    fn remove_tuple(&mut self, key: &Tuple) {
        self.unsubscribe_tuple(key);
        let cache = self.cache.as_mut().unwrap();
        cache.ready.remove(key);
        cache.prefixes.remove(key);
        cache.tuples.remove(key);
        for id in &key.1 {
            let xs = cache.incident.get_mut(id).unwrap();
            xs.remove(key);
            if xs.is_empty() {
                cache.incident.remove(id);
            }
        }
    }
    fn advance_selective(&mut self) -> bool {
        let changed = std::mem::take(&mut self.changed);
        let mut wake = BTreeSet::new();
        for h in changed {
            if let Some(xs) = self.cache.as_ref().unwrap().watchers.get(&h) {
                if METRICS {
                    self.work.notifications += xs.len();
                }
                wake.extend(xs.iter().cloned());
            }
        }
        for key in wake {
            let waiting = &self.cache.as_ref().unwrap().conditions[&key];
            let known = |h: usize| self.graph.nodes[self.graph.targets[h]].descriptor.is_some();
            let possible = waiting.descriptors.iter().any(|&h| known(h))
                || waiting.equality.is_some_and(|(a, b)| {
                    self.graph.targets[a] == self.graph.targets[b] || known(a) && known(b)
                });
            if possible {
                if METRICS {
                    self.work.wakeups += 1;
                }
                self.inspect_tuple(&key);
            }
        }
        for id in std::mem::take(&mut self.new_occ) {
            self.register(Some(id));
        }
        let selected = if self.intermediate {
            let cache = self.cache.as_ref().unwrap();
            let mut best = cache
                .ready
                .first_key_value()
                .map(|(k, e)| (k.clone(), e.clone()));
            for (key, env) in &cache.prefixes {
                if METRICS {
                    self.work
                        .intermediate_visits
                        .set(self.work.intermediate_visits.get() + 1);
                }
                if best.as_ref().is_some_and(|(old, _)| key > old) {
                    break;
                }
                let rule = &self.program.rules[key.0];
                if key.1.len() + 1 != rule.kept.len() + rule.removed.len() {
                    continue;
                }
                let heads = rule
                    .kept
                    .iter()
                    .chain(&rule.removed)
                    .skip(key.1.len())
                    .collect::<Vec<_>>();
                if let Some((ids, env)) =
                    self.select(key.0, &heads, &mut key.1.clone(), env.clone())
                {
                    let full = (key.0, ids);
                    if best.as_ref().is_none_or(|(old, _)| full < *old) {
                        best = Some((full, env));
                    }
                }
            }
            best
        } else {
            self.cache.as_mut().unwrap().ready.pop_first()
        };
        let Some((key, mut env)) = selected else {
            return true;
        };
        self.history.insert(key.clone());
        let program = self.program.clone();
        let r = &program.rules[key.0];
        let mut invalid = BTreeSet::new();
        if self.cache.as_ref().unwrap().tuples.contains_key(&key) {
            invalid.insert(key.clone());
        }
        for id in &key.1[r.kept.len()..] {
            invalid.extend(
                self.cache
                    .as_ref()
                    .unwrap()
                    .incident
                    .get(id)
                    .into_iter()
                    .flatten()
                    .cloned(),
            );
            self.live.remove(id);
        }
        for key in invalid {
            self.remove_tuple(&key);
        }
        if METRICS {
            self.firings += 1;
        }
        self.body(&r.body, &mut env);
        false
    }
}

impl<const METRICS: bool> Execution<METRICS> {
    pub fn assert_cache_integrity(&self) {
        let Some(cache) = &self.cache else {
            return;
        };
        for (key, handles) in &cache.tuples {
            if self.intermediate {
                let r = &self.program.rules[key.0];
                let heads = r.kept.len() + r.removed.len();
                assert!(heads == 1 || key.1.len() < heads, "terminal tuple retained");
            }
            assert!(!self.history.contains(key));
            for id in &key.1 {
                assert!(self.live.contains_key(id));
                assert!(cache.incident[id].contains(key));
            }
            for h in handles {
                assert!(cache.watchers[h].contains(key));
            }
        }
        for (id, keys) in &cache.incident {
            for key in keys {
                assert!(cache.tuples.contains_key(key));
                assert!(key.1.contains(id));
            }
        }
        for (h, keys) in &cache.watchers {
            for key in keys {
                assert!(cache.tuples[key].contains(h));
            }
        }
        for key in cache.prefixes.keys() {
            assert!(self.partial);
            assert!(cache.tuples.contains_key(key));
            assert!(cache.tuples[key].is_empty());
            assert!(!cache.ready.contains_key(key));
            let r = &self.program.rules[key.0];
            assert!(key.1.len() < r.kept.len() + r.removed.len());
        }
        if self.partial {
            for key in cache.tuples.keys() {
                if key.1.len() > 1 {
                    assert!(
                        cache
                            .prefixes
                            .contains_key(&(key.0, key.1[..key.1.len() - 1].to_vec()))
                    );
                }
            }
        }
        for key in cache.ready.keys() {
            assert!(cache.tuples.contains_key(key));
            assert!(cache.tuples[key].is_empty());
        }
    }
}

impl<const METRICS: bool> Execution<METRICS> {
    fn register_prefixes(&mut self, required: Option<usize>) {
        // Existing successful prefixes can accept this new partner. A prefix
        // built below recursively considers all currently live partners itself.
        if let Some(id) = required {
            let prefixes = self
                .cache
                .as_ref()
                .unwrap()
                .prefixes
                .keys()
                .cloned()
                .collect::<Vec<_>>();
            for key in prefixes {
                self.extend_prefix(&key, Some(id));
            }
        }
        let mut keys = vec![];
        for (rule, r) in self.program.rules.iter().enumerate() {
            let head = r.kept.iter().chain(&r.removed).next().unwrap();
            use std::ops::Bound::{Included, Unbounded};
            let bounds = required.map_or((Unbounded, Unbounded), |id| (Included(id), Included(id)));
            for (&id, fact) in self.live.range(bounds) {
                if METRICS {
                    self.work.fact_visits.set(self.work.fact_visits.get() + 1);
                }
                if head.name == fact.name && head.args.len() == fact.args.len() {
                    if METRICS {
                        self.work.combinations.set(self.work.combinations.get() + 1);
                    }
                    keys.push((rule, vec![id]));
                }
            }
        }
        for key in keys {
            self.add_entry(key);
        }
    }
    fn extend_prefix(&mut self, key: &Tuple, required: Option<usize>) {
        let r = &self.program.rules[key.0];
        if self.intermediate && key.1.len() + 1 == r.kept.len() + r.removed.len() {
            return;
        }
        let head = r.kept.iter().chain(&r.removed).nth(key.1.len()).unwrap();
        let mut keys = vec![];
        use std::ops::Bound::{Included, Unbounded};
        let bounds = required.map_or((Unbounded, Unbounded), |id| (Included(id), Included(id)));
        for (&id, fact) in self.live.range(bounds) {
            if METRICS {
                self.work.fact_visits.set(self.work.fact_visits.get() + 1);
            }
            if !key.1.contains(&id) && head.name == fact.name && head.args.len() == fact.args.len()
            {
                if METRICS {
                    self.work.combinations.set(self.work.combinations.get() + 1);
                }
                let mut ids = key.1.clone();
                ids.push(id);
                keys.push((key.0, ids));
            }
        }
        for key in keys {
            self.add_entry(key);
        }
    }
}

#[path = "local_search.rs"]
pub mod search;

#[cfg(test)]
mod diagnostic_tests {
    #[test]
    fn nested_matching_diagnostics_follow_execution_mode() {
        use super::*;
        let rules = [Rule::simplify(
            "match",
            [chr_syntax::c("x", [chr_syntax::t("f", [chr_syntax::v(0)])])],
            Goal::True,
        )];
        let q = Query {
            constraints: vec![chr_syntax::c(
                "x",
                [chr_syntax::t("f", [chr_syntax::atom("a")])],
            )],
            outputs: vec![],
        };
        let p = Program::compile(&rules).unwrap();
        let mut counted = p.start_mode::<true>(&q, false);
        let mut plain = p.start_mode::<false>(&q, false);
        for _ in 0..10 {
            if counted.advance() {
                break;
            }
        }
        for _ in 0..10 {
            if plain.advance() {
                break;
            }
        }
        assert_eq!(counted.firings, 1);
        assert!(plain.answer().unwrap().residual.is_empty());
        assert!(counted.graph.dependency_work.pattern_nodes.get() >= 2);
        assert_eq!(plain.graph.dependency_work.pattern_nodes.get(), 0);
        assert!(chr_observe::equivalent(
            &counted.answer().unwrap(),
            &plain.answer().unwrap(),
            &mut Default::default()
        ));
    }
}
