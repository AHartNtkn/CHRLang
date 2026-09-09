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
        let mut e = Execution {
            program: self.clone(),
            graph: Run::with_dependencies(DependencyMode::Indexed),
            live: BTreeMap::new(),
            next_occ: 0,
            history: BTreeSet::new(),
            outputs: vec![],
            firings: 0,
        };
        let mut env = BTreeMap::new();
        for c in &q.constraints {
            e.post(c, &mut env);
        }
        for (name, v) in &q.outputs {
            let h = e.graph.instantiate(&Term::Var(*v), &mut env);
            e.outputs.push((name.clone(), h));
        }
        e
    }
}
pub struct Execution {
    program: Arc<Program>,
    graph: Run<false>,
    live: BTreeMap<usize, Fact>,
    next_occ: usize,
    history: BTreeSet<(usize, Vec<usize>)>,
    outputs: Vec<(String, usize)>,
    pub firings: usize,
}
impl Execution {
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
                self.graph.settle();
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
        let program = self.program.clone();
        for (i, r) in program.rules.iter().enumerate() {
            let heads = r.kept.iter().chain(&r.removed).collect::<Vec<_>>();
            if let Some((ids, mut env)) = self.select(i, &heads, &mut vec![], BTreeMap::new()) {
                self.history.insert((i, ids.clone()));
                for id in &ids[r.kept.len()..] {
                    self.live.remove(id);
                }
                self.firings += 1;
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
