//! Direct graph source experiment. Rule selection is declaration/occurrence order
//! independently within each context. One tick may do size-dependent finite work.
use crate::{Context, Graph, NodeId, Occurrence, Resources, View};
use chr_syntax::{Answer, Constraint, Goal, Guard, Query, Rule, Term, Var};
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

type Locals = BTreeMap<Var, NodeId>;
#[derive(Clone)]
struct Row {
    name: String,
    args: Vec<NodeId>,
}
struct Pending {
    goal: Goal,
    locals: Locals,
    region: Context,
}
struct Application {
    rule: usize,
    ids: Vec<Occurrence>,
    locals: Locals,
    region: Context,
}
pub struct PreparedRuleset {
    rules: Rc<Vec<Rule>>,
}
pub struct Engine {
    rules: Rc<Vec<Rule>>,
    graph: Graph,
    resources: Resources,
    rows: BTreeMap<Occurrence, Row>,
    history: BTreeMap<(usize, Vec<Occurrence>), Vec<Context>>,
    pending: VecDeque<Pending>,
    published: Vec<Context>,
    answers: VecDeque<Answer>,
    outputs: Vec<(String, NodeId)>,
    fresh: u64,
}
pub enum Event {
    Progress,
    Answer(Answer),
    Exhausted,
}
fn subtract(mut regions: Vec<Context>, excluded: &[Context]) -> Vec<Context> {
    for x in excluded {
        regions = regions.iter().flat_map(|r| r.subtract(x)).collect();
    }
    regions
}
impl PreparedRuleset {
    pub fn new(rules: Vec<Rule>) -> Result<Self, String> {
        if rules
            .iter()
            .any(|r| r.kept.is_empty() && r.removed.is_empty())
        {
            return Err("a rule must have at least one head".into());
        }
        Ok(Self {
            rules: Rc::new(rules),
        })
    }
    pub fn start(&self, query: Query) -> Result<Engine, String> {
        let mut engine = Engine {
            rules: self.rules.clone(),
            graph: Graph::default(),
            resources: Resources::default(),
            rows: BTreeMap::new(),
            history: BTreeMap::new(),
            pending: VecDeque::new(),
            published: Vec::new(),
            answers: VecDeque::new(),
            outputs: Vec::new(),
            fresh: 0,
        };
        let mut locals = Locals::new();
        for c in query.constraints {
            engine.post(&c, &mut locals, &Context::all());
        }
        for (name, var) in query.outputs {
            let node = engine.term(&Term::Var(var), &mut locals);
            engine.outputs.push((name, node));
        }
        Ok(engine)
    }
}
impl Engine {
    fn term(&mut self, t: &Term, locals: &mut Locals) -> NodeId {
        match t {
            Term::Var(var) => *locals.entry(*var).or_insert_with(|| {
                let id = self.fresh;
                self.fresh = self
                    .fresh
                    .checked_add(1)
                    .expect("logical identity exhausted");
                self.graph.unknown(id)
            }),
            Term::App(name, args) => {
                let args = args.iter().map(|t| self.term(t, locals)).collect();
                self.graph.app(name, args)
            }
        }
    }
    fn post(&mut self, c: &Constraint, locals: &mut Locals, region: &Context) {
        let args = c.args.iter().map(|t| self.term(t, locals)).collect();
        let id = self.resources.post(region.clone());
        self.rows.insert(
            id,
            Row {
                name: c.name.clone(),
                args,
            },
        );
    }
    fn active(&self, region: &Context) -> Vec<Context> {
        subtract(self.graph.live(region), &self.published)
    }
    fn pattern(
        &self,
        pattern: &Term,
        value: NodeId,
        locals: Locals,
        region: &Context,
    ) -> Vec<(Context, Locals)> {
        match pattern {
            Term::Var(var) => match locals.get(var) {
                Some(old) => self
                    .graph
                    .equal(*old, value, region)
                    .into_iter()
                    .map(|r| (r, locals.clone()))
                    .collect(),
                None => {
                    let mut next = locals;
                    next.insert(*var, value);
                    vec![(region.clone(), next)]
                }
            },
            Term::App(name, args) => {
                let mut result = Vec::new();
                for (r, view) in self.graph.expose(value, region) {
                    if let View::Constructor(actual, children) = view
                        && &actual == name
                        && children.len() == args.len()
                    {
                        let mut partial = vec![(r, locals.clone())];
                        for (p, v) in args.iter().zip(children) {
                            partial = partial
                                .into_iter()
                                .flat_map(|(r, env)| self.pattern(p, v, env, &r))
                                .collect();
                        }
                        result.extend(partial);
                    }
                }
                result
            }
        }
    }
    fn heads(
        &self,
        heads: &[Constraint],
        index: usize,
        ids: Vec<Occurrence>,
        locals: Locals,
        region: &Context,
    ) -> Vec<(Context, Vec<Occurrence>, Locals)> {
        if index == heads.len() {
            return vec![(region.clone(), ids, locals)];
        }
        let head = &heads[index];
        let mut result = Vec::new();
        for (id, row) in &self.rows {
            if ids.contains(id) || row.name != head.name || row.args.len() != head.args.len() {
                continue;
            }
            for live in self.resources.available(*id) {
                let Some(region) = region.intersection(live) else {
                    continue;
                };
                let mut partial = vec![(region, locals.clone())];
                for (p, v) in head.args.iter().zip(&row.args) {
                    partial = partial
                        .into_iter()
                        .flat_map(|(r, env)| self.pattern(p, *v, env, &r))
                        .collect();
                }
                for (r, env) in partial {
                    let mut next = ids.clone();
                    next.push(*id);
                    result.extend(self.heads(heads, index + 1, next, env, &r));
                }
            }
        }
        result
    }
    fn applications(&mut self) -> Vec<Application> {
        let mut uncovered = self.active(&Context::all());
        let mut result = Vec::new();
        // Preparation is shared; temporary clones keep graph allocation for guard
        // terms separate from the immutable source borrow.
        let rules = self.rules.clone();
        for (index, rule) in rules.iter().enumerate() {
            let heads: Vec<_> = rule.kept.iter().chain(&rule.removed).cloned().collect();
            let regions = uncovered.clone();
            for region in regions {
                for (region, ids, mut locals) in
                    self.heads(&heads, 0, Vec::new(), Locals::new(), &region)
                {
                    let mut eligible = subtract(
                        vec![region],
                        self.history
                            .get(&(index, ids.clone()))
                            .map(Vec::as_slice)
                            .unwrap_or(&[]),
                    );
                    for guard in &rule.guards {
                        let Guard::Equal(left, right) = guard;
                        let left = self.term(left, &mut locals);
                        let right = self.term(right, &mut locals);
                        eligible = eligible
                            .into_iter()
                            .flat_map(|r| self.graph.equal(left, right, &r))
                            .collect();
                    }
                    for r in eligible {
                        // An earlier rule/tuple may already own part of this region.
                        let available: Vec<_> = uncovered
                            .iter()
                            .filter_map(|u| u.intersection(&r))
                            .collect();
                        for r in available {
                            uncovered = subtract(uncovered, std::slice::from_ref(&r));
                            result.push(Application {
                                rule: index,
                                ids: ids.clone(),
                                locals: locals.clone(),
                                region: r,
                            });
                        }
                    }
                }
            }
            if uncovered.is_empty() {
                break;
            }
        }
        result
    }
    fn term_variables(&mut self, term: &Term, locals: &mut Locals) {
        match term {
            Term::Var(_) => {
                self.term(term, locals);
            }
            Term::App(_, args) => {
                for term in args {
                    self.term_variables(term, locals);
                }
            }
        }
    }
    fn body_variables(&mut self, goal: &Goal, locals: &mut Locals) {
        match goal {
            Goal::Constraint(c) => {
                for t in &c.args {
                    self.term_variables(t, locals);
                }
            }
            Goal::Unify(a, b) => {
                self.term_variables(a, locals);
                self.term_variables(b, locals);
            }
            Goal::And(gs) => {
                for g in gs {
                    self.body_variables(g, locals);
                }
            }
            Goal::Or(a, b) => {
                self.body_variables(a, locals);
                self.body_variables(b, locals);
            }
            Goal::True | Goal::Fail => {}
        }
    }
    fn fire(&mut self, mut app: Application) {
        let rule = self.rules[app.rule].clone();
        let removed = &app.ids[rule.kept.len()..];
        let granted = self.resources.consume(removed, &app.region);
        for id in removed {
            if self.resources.available(*id).is_empty() {
                self.rows.remove(id);
            }
        }
        for region in granted {
            if rule.removed.is_empty() {
                self.history
                    .entry((app.rule, app.ids.clone()))
                    .or_default()
                    .push(region.clone());
            }
            self.body_variables(&rule.body, &mut app.locals);
            self.pending.push_back(Pending {
                goal: rule.body.clone(),
                locals: app.locals.clone(),
                region,
            });
        }
    }
    /// Recognize only pure OR trees assigning the same source variable.
    fn assignment(goal: &Goal) -> Option<Var> {
        match goal {
            Goal::Unify(Term::Var(v), _) => Some(*v),
            Goal::Or(a, b) => {
                let a = Self::assignment(a)?;
                (Self::assignment(b)? == a).then_some(a)
            }
            _ => None,
        }
    }
    fn choice_value(&mut self, goal: &Goal, locals: &mut Locals, region: &Context) -> NodeId {
        match goal {
            Goal::Unify(_, value) => self.term(value, locals),
            Goal::Or(a, b) => {
                let label = self.graph.birth(region.clone());
                let left = self.choice_value(a, locals, &region.select(label, false).unwrap());
                let right = self.choice_value(b, locals, &region.select(label, true).unwrap());
                self.graph.choice(label, left, right)
            }
            _ => unreachable!("checked pure assignment tree"),
        }
    }
    fn execute(&mut self, job: Pending) {
        for region in self.active(&job.region) {
            let mut locals = job.locals.clone();
            match &job.goal {
                Goal::Constraint(c) => self.post(c, &mut locals, &region),
                Goal::Unify(a, b) => {
                    let a = self.term(a, &mut locals);
                    let b = self.term(b, &mut locals);
                    self.graph.unify(a, b, &region);
                }
                Goal::True => {}
                Goal::Fail => self.graph.fail(&region),
                Goal::And(goals) => {
                    for goal in goals.iter().rev() {
                        self.pending.push_front(Pending {
                            goal: goal.clone(),
                            locals: locals.clone(),
                            region: region.clone(),
                        });
                    }
                }
                Goal::Or(a, b) => {
                    if let Some(var) = Self::assignment(&job.goal) {
                        let target = self.term(&Term::Var(var), &mut locals);
                        let value = self.choice_value(&job.goal, &mut locals, &region);
                        self.graph.unify(target, value, &region);
                    } else {
                        let label = self.graph.birth(region.clone());
                        self.pending.push_front(Pending {
                            goal: *b.clone(),
                            locals: locals.clone(),
                            region: region.select(label, true).unwrap(),
                        });
                        self.pending.push_front(Pending {
                            goal: *a.clone(),
                            locals,
                            region: region.select(label, false).unwrap(),
                        });
                    }
                }
            }
        }
    }
    fn publish(&mut self, region: Context) {
        // Partition only on residual membership; values remain graph nodes until
        // the complete joint observation, which preserves aliases across roots.
        let mut cases = vec![(region.clone(), Vec::<Row>::new())];
        for (id, row) in &self.rows {
            let mut next = Vec::new();
            for (r, rows) in cases {
                for live in self.resources.available(*id) {
                    if let Some(inside) = r.intersection(live) {
                        let mut included = rows.clone();
                        included.push(row.clone());
                        next.push((inside, included));
                    }
                }
                for outside in subtract(vec![r], self.resources.available(*id)) {
                    next.push((outside, rows.clone()));
                }
            }
            cases = next;
        }
        for (r, rows) in cases {
            let roots: Vec<_> = self
                .outputs
                .iter()
                .map(|(_, n)| *n)
                .chain(rows.iter().flat_map(|r| r.args.iter().copied()))
                .collect();
            for (_, terms) in self.graph.observe(&roots, &r) {
                let mut values = terms.into_iter();
                let outputs = self
                    .outputs
                    .iter()
                    .map(|(name, _)| (name.clone(), values.next().unwrap()))
                    .collect();
                let residual = rows
                    .iter()
                    .map(|row| Constraint {
                        name: row.name.clone(),
                        args: (0..row.args.len())
                            .map(|_| values.next().unwrap())
                            .collect(),
                    })
                    .collect();
                self.answers.push_back(Answer { outputs, residual });
            }
        }
        self.published.push(region);
    }
    pub fn retained_constructors(&self) -> BTreeMap<String, usize> {
        self.graph.retained_constructors()
    }
    pub fn tick(&mut self) -> Event {
        if let Some(answer) = self.answers.pop_front() {
            return Event::Answer(answer);
        }
        if let Some(job) = self.pending.pop_front() {
            self.execute(job);
        }
        let applications = self.applications();
        for app in applications {
            self.fire(app);
        }
        let pending: Vec<_> = self.pending.iter().map(|p| p.region.clone()).collect();
        for region in subtract(self.active(&Context::all()), &pending) {
            self.publish(region);
        }
        if let Some(answer) = self.answers.pop_front() {
            Event::Answer(answer)
        } else if self.active(&Context::all()).is_empty() {
            Event::Exhausted
        } else {
            Event::Progress
        }
    }
}
