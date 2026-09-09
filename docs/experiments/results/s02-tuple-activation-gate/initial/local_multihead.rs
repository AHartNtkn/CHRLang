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
        let mut e = Execution {
            program: self.clone(),
            graph: Run::with_dependencies(DependencyMode::Indexed),
            live: BTreeMap::new(),
            next_occ: 0,
            history: BTreeSet::new(),
            outputs: vec![],
            firings: 0,
            cache: selective.then(Cache::default),
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
#[derive(Default)]
struct Cache {
    tuples: BTreeMap<Tuple, Vec<usize>>,
    ready: BTreeMap<Tuple, Env>,
    watchers: BTreeMap<usize, BTreeSet<Tuple>>,
    incident: BTreeMap<usize, BTreeSet<Tuple>>,
}
#[derive(Default, Debug)]
pub struct Work {
    pub head_attempts: Cell<usize>,
    pub combinations: Cell<usize>,
    pub inspections: usize,
    pub registrations: usize,
    pub notifications: usize,
    pub changed_handles: usize,
    pub peak_tuples: usize,
}
pub struct Execution<const METRICS: bool = true> {
    program: Arc<Program>,
    graph: Run<false>,
    live: BTreeMap<usize, Fact>,
    next_occ: usize,
    history: BTreeSet<(usize, Vec<usize>)>,
    outputs: Vec<(String, usize)>,
    pub firings: usize,
    cache: Option<Cache>,
    new_occ: Vec<usize>,
    changed: BTreeSet<usize>,
    pub work: Work,
}
impl<const METRICS: bool> Execution<METRICS> {
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
            return (!self.history.contains(&(rule, ids.clone()))).then(|| (ids.clone(), env));
        }
        for (&id, fact) in &self.live {
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
        required: Option<usize>,
        out: &mut Vec<Tuple>,
    ) {
        if heads.is_empty() {
            if METRICS {
                self.work.combinations.set(self.work.combinations.get() + 1);
            }
            if required.is_none_or(|id| ids.contains(&id)) {
                out.push((rule, ids.clone()));
            }
            return;
        }
        for (&id, f) in &self.live {
            if ids.contains(&id) || heads[0].name != f.name || heads[0].args.len() != f.args.len() {
                continue;
            }
            ids.push(id);
            self.combinations(rule, &heads[1..], ids, required, out);
            ids.pop();
        }
    }
    fn register(&mut self, required: Option<usize>) {
        let mut keys = vec![];
        for (i, r) in self.program.rules.iter().enumerate() {
            self.combinations(
                i,
                &r.kept.iter().chain(&r.removed).collect::<Vec<_>>(),
                &mut vec![],
                required,
                &mut keys,
            );
        }
        for key in keys {
            let cache = self.cache.as_mut().unwrap();
            if cache.tuples.contains_key(&key) || self.history.contains(&key) {
                continue;
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
    }
    fn unsubscribe_tuple(&mut self, key: &Tuple) {
        let cache = self.cache.as_mut().unwrap();
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
        let mut env = Env::new();
        let mut deps = Dependencies::default();
        let success = r
            .kept
            .iter()
            .chain(&r.removed)
            .zip(&key.1)
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
            cache.ready.insert(key.clone(), env);
            return;
        }
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
            self.inspect_tuple(&key);
        }
        for id in std::mem::take(&mut self.new_occ) {
            self.register(Some(id));
        }
        let Some((key, mut env)) = self.cache.as_mut().unwrap().ready.pop_first() else {
            return true;
        };
        self.history.insert(key.clone());
        let program = self.program.clone();
        let r = &program.rules[key.0];
        let mut invalid = BTreeSet::from([key.clone()]);
        for id in &key.1[r.kept.len()..] {
            invalid.extend(self.cache.as_ref().unwrap().incident[id].iter().cloned());
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
        for key in cache.ready.keys() {
            assert!(cache.tuples.contains_key(key));
            assert!(cache.tuples[key].is_empty());
        }
    }
}
