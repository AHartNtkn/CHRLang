//! Experimental suspended applications for a checked equation-producing source fragment.
//! Context-indexed results preserve application identity. This is not yet a
//! general multihead CHR executor or an implementation of local pull-tab rewrites.
use chr_syntax::{Answer, Constraint, Goal, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;
type Id = usize;
type Context = BTreeMap<usize, bool>;
type Env = BTreeMap<Var, Id>;
#[derive(Clone)]
enum Plan {
    Value(Term),
    Call(String, Vec<Term>),
    Choice(Box<Plan>, Box<Plan>),
    Producers(Vec<Action>, Box<Plan>),
    Fail,
}
#[derive(Clone)]
enum Action {
    Call(Var, String, Vec<Term>),
    Post(Constraint),
}
struct Resource {
    constraint: String,
    args: Vec<Id>,
    birth: Context,
    consumed: Vec<Context>,
}
struct Clause {
    name: String,
    inputs: Vec<Term>,
    body: Plan,
    partners: Vec<(Constraint, bool)>,
    reusable_static_match: bool,
}
/// Experimental result validity policies; neither selects a language architecture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reuse {
    CurrentContext,
    StaticBirth,
}
pub struct Prepared {
    clauses: Rc<Vec<Clause>>,
    resource_signatures: BTreeSet<(String, usize)>,
}
#[derive(Clone)]
enum Node {
    Unknown(u64),
    App(String, Vec<Id>),
    Alias(Id),
    Call(String, Vec<Id>, Id, usize),
    Choice(usize, Id, Id),
    Fail,
}
struct Cell {
    node: Node,
    results: Vec<(Context, Id)>,
}
pub struct Run {
    clauses: Rc<Vec<Clause>>,
    nodes: Vec<Cell>,
    tasks: VecDeque<(Context, usize, usize)>,
    obligations: Vec<(Context, Id)>,
    outputs: Vec<(String, Id)>,
    fresh: u64,
    births: Vec<Context>,
    resources: Vec<Resource>,
}
pub enum Event {
    Progress,
    Answer(Answer),
    Exhausted,
}
enum Signal {
    Progress,
    Split(usize),
    Fail,
}
fn variables(t: &Term, vars: &mut Vec<Var>) {
    match t {
        Term::Var(v) => vars.push(*v),
        Term::App(_, xs) => {
            for t in xs {
                variables(t, vars);
            }
        }
    }
}
fn cycle(v: Var, edges: &BTreeMap<Var, Vec<Var>>, path: &mut BTreeSet<Var>) -> bool {
    if !path.insert(v) {
        return true;
    }
    let yes = edges
        .get(&v)
        .is_some_and(|ds| ds.iter().any(|d| cycle(*d, edges, path)));
    path.remove(&v);
    yes
}
fn plan(
    g: &Goal,
    output: Var,
    inputs: &BTreeSet<Var>,
    resources: &BTreeSet<(String, usize)>,
) -> Result<Plan, String> {
    let valid_value = |t: &Term| {
        let mut vs = vec![];
        variables(t, &mut vs);
        !vs.contains(&output)
    };
    match g {
        Goal::Unify(Term::Var(v), t) if *v == output && valid_value(t) => {
            Ok(Plan::Value(t.clone()))
        }
        Goal::Constraint(c)
            if c.args.last() == Some(&Term::Var(output))
                && c.args[..c.args.len() - 1].iter().all(valid_value) =>
        {
            Ok(Plan::Call(
                c.name.clone(),
                c.args[..c.args.len() - 1].to_vec(),
            ))
        }
        Goal::Or(a, b) => Ok(Plan::Choice(
            Box::new(plan(a, output, inputs, resources)?),
            Box::new(plan(b, output, inputs, resources)?),
        )),
        Goal::Fail => Ok(Plan::Fail),
        Goal::And(goals) if !goals.is_empty() => {
            let mut bound = inputs.clone();
            bound.insert(output);
            let mut producers = vec![];
            for g in &goals[..goals.len() - 1] {
                let Goal::Constraint(c) = g else {
                    return Err("body prefix must contain fresh-output calls".into());
                };
                if resources.contains(&(c.name.clone(), c.args.len())) {
                    let mut used = vec![];
                    for t in &c.args {
                        variables(t, &mut used)
                    }
                    if !used.is_empty() {
                        return Err("passive body posts must be ground in this certificate".into());
                    }
                    producers.push(Action::Post(c.clone()));
                    continue;
                }
                let Some(Term::Var(v)) = c.args.last() else {
                    return Err("producer needs variable output".into());
                };
                if !bound.insert(*v) {
                    return Err("producer output is not fresh or has multiple writers".into());
                }
                let mut used = vec![];
                for t in &c.args[..c.args.len() - 1] {
                    variables(t, &mut used);
                }
                if used.contains(v) || used.contains(&output) {
                    return Err("cyclic producer/output dependence".into());
                }
                producers.push(Action::Call(
                    *v,
                    c.name.clone(),
                    c.args[..c.args.len() - 1].to_vec(),
                ));
            }
            let mut edges = BTreeMap::new();
            for action in &producers {
                let Action::Call(out, _, args) = action else {
                    continue;
                };
                let mut deps = vec![];
                for t in args {
                    variables(t, &mut deps)
                }
                edges.insert(*out, deps);
            }
            for out in edges.keys() {
                if cycle(*out, &edges, &mut BTreeSet::new()) {
                    return Err("cyclic body producer dependencies".into());
                }
            }
            Ok(Plan::Producers(
                producers,
                Box::new(plan(goals.last().unwrap(), output, &bound, resources)?),
            ))
        }
        _ => Err("unsupported body or output assignment".into()),
    }
}
fn calls(p: &Plan, out: &mut Vec<(String, usize)>) {
    match p {
        Plan::Call(n, args) => out.push((n.clone(), args.len())),
        Plan::Choice(a, b) => {
            calls(a, out);
            calls(b, out)
        }
        Plan::Producers(ps, p) => {
            out.extend(ps.iter().filter_map(|action| match action {
                Action::Call(_, n, a) => Some((n.clone(), a.len())),
                Action::Post(_) => None,
            }));
            calls(p, out)
        }
        _ => {}
    }
}
impl Prepared {
    pub fn new(rules: Vec<Rule>) -> Result<Self, String> {
        Self::with_reuse(rules, Reuse::StaticBirth)
    }
    pub fn with_reuse(rules: Vec<Rule>, reuse: Reuse) -> Result<Self, String> {
        let mut resource_signatures = BTreeSet::new();
        for rule in &rules {
            if rule.removed.is_empty() || !rule.guards.is_empty() {
                return Err("requires a removed primary head without guards".into());
            }
            for c in rule.kept.iter().chain(rule.removed.iter().skip(1)) {
                resource_signatures.insert((c.name.clone(), c.args.len()));
            }
        }
        for rule in &rules {
            let c = &rule.removed[0];
            if resource_signatures.contains(&(c.name.clone(), c.args.len())) {
                return Err("call and passive resource signatures must be disjoint".into());
            }
        }
        let mut clauses = vec![];
        for rule in rules {
            let head = &rule.removed[0];
            let Some(Term::Var(output)) = head.args.last() else {
                return Err("head needs distinct variable output".into());
            };
            let inputs = head.args[..head.args.len() - 1].to_vec();
            let mut vs = vec![];
            for t in &inputs {
                variables(t, &mut vs);
            }
            let mut set: BTreeSet<_> = vs.iter().copied().collect();
            if set.len() != vs.len() || set.contains(output) {
                return Err("input patterns must be linear and exclude output".into());
            }
            let partners: Vec<_> = rule
                .kept
                .iter()
                .cloned()
                .map(|c| (c, false))
                .chain(rule.removed.iter().skip(1).cloned().map(|c| (c, true)))
                .collect();
            for (c, _) in &partners {
                let mut vars = vec![];
                for t in &c.args {
                    variables(t, &mut vars)
                }
                if vars.contains(output) {
                    return Err("primary output cannot occur in a resource head".into());
                }
                set.extend(vars);
            }
            clauses.push(Clause {
                name: head.name.clone(),
                inputs,
                body: plan(&rule.body, *output, &set, &resource_signatures)?,
                partners,
                reusable_static_match: false,
            });
        }
        // Linear input patterns overlap iff no constructor position separates them.
        // Reordering a producer ahead of an enabled overlapping rule is not certified.
        fn overlap(a: &Term, b: &Term) -> bool {
            match (a, b) {
                (Term::Var(_), _) | (_, Term::Var(_)) => true,
                (Term::App(a, xs), Term::App(b, ys)) => {
                    a == b && xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| overlap(x, y))
                }
            }
        }
        for (i, a) in clauses.iter().enumerate() {
            for b in &clauses[i + 1..] {
                if a.name == b.name
                    && a.inputs.len() == b.inputs.len()
                    && a.inputs.iter().zip(&b.inputs).all(|(x, y)| overlap(x, y))
                {
                    return Err("overlapping clauses require source competition scheduling".into());
                }
            }
        }
        let signatures: BTreeSet<_> = clauses
            .iter()
            .map(|c| (c.name.clone(), c.inputs.len()))
            .collect();
        for c in &clauses {
            let mut used = vec![];
            calls(&c.body, &mut used);
            if used.iter().any(|s| !signatures.contains(s)) {
                return Err("body call has no checked definition".into());
            }
        }
        fn pure_plan(p: &Plan, allowed: &BTreeSet<(String, usize)>) -> bool {
            match p {
                Plan::Value(_) | Plan::Fail => true,
                Plan::Call(n, args) => allowed.contains(&(n.clone(), args.len())),
                Plan::Choice(_, _) => false,
                Plan::Producers(actions, p) => {
                    actions.iter().all(|a| match a {
                        Action::Call(_, n, args) => allowed.contains(&(n.clone(), args.len())),
                        Action::Post(_) => false,
                    }) && pure_plan(p, allowed)
                }
            }
        }
        let mut pure = if reuse == Reuse::StaticBirth {
            signatures.clone()
        } else {
            BTreeSet::new()
        };
        if reuse == Reuse::StaticBirth {
            loop {
                let invalid = clauses
                    .iter()
                    .filter(|c| !c.partners.is_empty() || !pure_plan(&c.body, &pure))
                    .map(|c| (c.name.clone(), c.inputs.len()))
                    .collect::<Vec<_>>();
                let mut changed = false;
                for signature in invalid {
                    changed |= pure.remove(&signature)
                }
                if !changed {
                    break;
                }
            }
        }
        for clause in &mut clauses {
            clause.reusable_static_match =
                pure.contains(&(clause.name.clone(), clause.inputs.len()));
        }
        Ok(Self {
            clauses: Rc::new(clauses),
            resource_signatures,
        })
    }
    pub fn start(&self, query: Query) -> Result<Run, String> {
        let mut run = Run {
            clauses: self.clauses.clone(),
            nodes: vec![],
            tasks: VecDeque::from([(Context::new(), 0, 0)]),
            obligations: vec![],
            outputs: vec![],
            fresh: 0,
            births: vec![],
            resources: vec![],
        };
        let mut env = Env::new();
        let mut writers = BTreeSet::new();
        let mut edges = BTreeMap::new();
        for c in &query.constraints {
            if self
                .resource_signatures
                .contains(&(c.name.clone(), c.args.len()))
            {
                continue;
            }
            let Some(Term::Var(out)) = c.args.last() else {
                return Err("query requires variable call outputs".into());
            };
            if !writers.insert(*out) {
                return Err("query output has multiple writers".into());
            }
            if !self
                .clauses
                .iter()
                .any(|cl| cl.name == c.name && cl.inputs.len() + 1 == c.args.len())
            {
                return Err("query contains an unchecked predicate".into());
            }
            let mut deps = vec![];
            for t in &c.args[..c.args.len() - 1] {
                variables(t, &mut deps)
            }
            edges.insert(*out, deps);
        }
        for v in &writers {
            if cycle(*v, &edges, &mut BTreeSet::new()) {
                return Err("cyclic query dependencies".into());
            }
        }
        for c in &query.constraints {
            if self
                .resource_signatures
                .contains(&(c.name.clone(), c.args.len()))
            {
                let mut vars = vec![];
                for t in &c.args {
                    variables(t, &mut vars)
                }
                if vars.iter().any(|v| writers.contains(v)) {
                    return Err(
                        "passive query resources cannot depend on call outputs in this certificate"
                            .into(),
                    );
                }
            }
        }
        for c in query.constraints {
            if self
                .resource_signatures
                .contains(&(c.name.clone(), c.args.len()))
            {
                let args = c.args.iter().map(|t| run.term(t, &mut env)).collect();
                run.resources.push(Resource {
                    constraint: c.name,
                    args,
                    birth: Context::new(),
                    consumed: vec![],
                });
                continue;
            }
            let Term::Var(out) = c.args.last().unwrap() else {
                unreachable!()
            };
            let args = c.args[..c.args.len() - 1]
                .iter()
                .map(|t| run.term(t, &mut env))
                .collect();
            run.producer(c.name, args, *out, &mut env, &Context::new());
        }
        for (name, var) in query.outputs {
            let id = run.term(&Term::Var(var), &mut env);
            run.outputs.push((name, id));
        }
        Ok(run)
    }
}
impl Run {
    fn push(&mut self, node: Node) -> Id {
        let id = self.nodes.len();
        self.nodes.push(Cell {
            node,
            results: vec![],
        });
        id
    }
    fn bind(&mut self, v: Var, id: Id, env: &mut Env) {
        if let Some(old) = env.insert(v, id) {
            self.nodes[old].node = Node::Alias(id)
        }
    }
    fn call(&mut self, name: String, args: Vec<Id>, output: Id, ctx: &Context) -> Id {
        let call = self.push(Node::Call(name, args, output, self.obligations.len()));
        self.obligations.push((ctx.clone(), call));
        call
    }
    fn producer(
        &mut self,
        name: String,
        args: Vec<Id>,
        out: Var,
        env: &mut Env,
        ctx: &Context,
    ) -> Id {
        let placeholder = self.term(&Term::Var(out), env);
        let Node::Unknown(logical) = self.nodes[placeholder].node else {
            unreachable!("checked unique output writer")
        };
        // The logical output and the reference redirected to its producer are
        // separate nodes. A residual call can expose the former without a cycle.
        let output = self.push(Node::Unknown(logical));
        let call = self.call(name, args, output, ctx);
        self.bind(out, call, env);
        call
    }
    fn term(&mut self, t: &Term, env: &mut Env) -> Id {
        match t {
            Term::Var(v) => {
                if let Some(id) = env.get(v) {
                    *id
                } else {
                    let fresh = self.fresh;
                    self.fresh += 1;
                    let id = self.push(Node::Unknown(fresh));
                    env.insert(*v, id);
                    id
                }
            }
            Term::App(n, args) => {
                let children = args.iter().map(|t| self.term(t, env)).collect();
                self.push(Node::App(n.clone(), children))
            }
        }
    }
    fn expand(&mut self, p: &Plan, env: &mut Env, ctx: &Context, output: Id) -> Id {
        match p {
            Plan::Value(t) => self.term(t, env),
            Plan::Call(n, args) => {
                let args = args.iter().map(|t| self.term(t, env)).collect();
                self.call(n.clone(), args, output, ctx)
            }
            Plan::Fail => self.push(Node::Fail),
            Plan::Choice(a, b) => {
                let label = self.births.len();
                self.births.push(ctx.clone());
                let mut left = ctx.clone();
                left.insert(label, false);
                let mut right = ctx.clone();
                right.insert(label, true);
                let a = self.expand(a, &mut env.clone(), &left, output);
                let b = self.expand(b, &mut env.clone(), &right, output);
                self.push(Node::Choice(label, a, b))
            }
            Plan::Producers(ps, p) => {
                for action in ps {
                    match action {
                        Action::Call(v, n, args) => {
                            let args = args.iter().map(|t| self.term(t, env)).collect();
                            self.producer(n.clone(), args, *v, env, ctx);
                        }
                        Action::Post(c) => {
                            let args = c.args.iter().map(|t| self.term(t, env)).collect();
                            self.resources.push(Resource {
                                constraint: c.name.clone(),
                                args,
                                birth: ctx.clone(),
                                consumed: vec![],
                            });
                        }
                    }
                }
                self.expand(p, env, ctx, output)
            }
        }
    }
    fn matches(&mut self, p: &Term, id: Id, ctx: &Context, env: &mut Env) -> Result<bool, Signal> {
        match p {
            Term::Var(v) => {
                if let Some(old) = env.get(v).copied() {
                    return Ok(self.normal(old, ctx)? == self.normal(id, ctx)?);
                }
                env.insert(*v, id);
                Ok(true)
            }
            Term::App(n, ps) => {
                let actual = self.force(id, ctx)?;
                match self.nodes[actual].node.clone() {
                    Node::Unknown(_) => Ok(false),
                    Node::App(m, args) if *n == m && ps.len() == args.len() => {
                        for (p, id) in ps.iter().zip(args) {
                            if !self.matches(p, id, ctx, env)? {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    _ => Ok(false),
                }
            }
        }
    }
    fn live(&self, id: usize, ctx: &Context) -> bool {
        let r = &self.resources[id];
        r.birth.iter().all(|(k, v)| ctx.get(k) == Some(v))
            && !r
                .consumed
                .iter()
                .any(|c| c.iter().all(|(k, v)| ctx.get(k) == Some(v)))
    }
    fn partners(
        &mut self,
        heads: &[(Constraint, bool)],
        selected: Vec<usize>,
        env: Env,
        ctx: &Context,
    ) -> Result<Option<(Vec<usize>, Env)>, Signal> {
        if selected.len() == heads.len() {
            return Ok(Some((selected, env)));
        }
        let head = &heads[selected.len()].0;
        for id in 0..self.resources.len() {
            if selected.contains(&id) || !self.live(id, ctx) {
                continue;
            }
            let row = &self.resources[id];
            if row.constraint != head.name || row.args.len() != head.args.len() {
                continue;
            }
            let args = row.args.clone();
            let mut next = env.clone();
            let mut matched = true;
            for (p, arg) in head.args.iter().zip(args) {
                if !self.matches(p, arg, ctx, &mut next)? {
                    matched = false;
                    break;
                }
            }
            if matched {
                let mut ids = selected.clone();
                ids.push(id);
                if let Some(found) = self.partners(heads, ids, next, ctx)? {
                    return Ok(Some(found));
                }
            }
        }
        Ok(None)
    }
    // This certificate reads no context-sensitive node. Captured variable
    // arguments may still contain choices: their pointers stay opaque in the result.
    fn static_match(&self, pattern: &Term, id: Id) -> bool {
        match (pattern, &self.nodes[id].node) {
            (Term::Var(_), _) => true,
            (Term::App(n, ps), Node::App(m, args)) => {
                n == m
                    && ps.len() == args.len()
                    && ps.iter().zip(args).all(|(p, id)| self.static_match(p, *id))
            }
            _ => false,
        }
    }
    fn force(&mut self, id: Id, ctx: &Context) -> Result<Id, Signal> {
        match self.nodes[id].node.clone() {
            Node::Unknown(_) | Node::App(_, _) => Ok(id),
            Node::Alias(next) => self.force(next, ctx),
            Node::Fail => Err(Signal::Fail),
            Node::Choice(label, a, b) => match ctx.get(&label) {
                Some(false) => self.force(a, ctx),
                Some(true) => self.force(b, ctx),
                None => Err(Signal::Split(label)),
            },
            Node::Call(name, args, output, origin) => {
                if let Some((_, next)) = self.nodes[id]
                    .results
                    .iter()
                    .rev()
                    .find(|(support, _)| support.iter().all(|(k, v)| ctx.get(k) == Some(v)))
                {
                    return self.force(*next, ctx);
                }
                let clauses = self.clauses.clone();
                for clause in clauses
                    .iter()
                    .filter(|c| c.name == name && c.inputs.len() == args.len())
                {
                    let mut env = Env::new();
                    let mut matched = true;
                    for (pattern, arg) in clause.inputs.iter().zip(&args) {
                        if !self.matches(pattern, *arg, ctx, &mut env)? {
                            matched = false;
                            break;
                        }
                    }
                    if matched {
                        if !clause.partners.is_empty()
                            && let Some((label, _)) =
                                self.births.iter().enumerate().find(|(label, birth)| {
                                    !ctx.contains_key(label)
                                        && birth.iter().all(|(k, v)| ctx.get(k) == Some(v))
                                })
                        {
                            return Err(Signal::Split(label));
                        }
                        let Some((ids, mut env)) =
                            self.partners(&clause.partners, vec![], env, ctx)?
                        else {
                            continue;
                        };
                        // No evaluation intervenes between completed matching and claims.
                        for (id, (_, removed)) in ids.into_iter().zip(&clause.partners) {
                            if *removed {
                                self.resources[id].consumed.push(ctx.clone());
                            }
                        }
                        let support = if clause.reusable_static_match
                            && clause
                                .inputs
                                .iter()
                                .zip(&args)
                                .all(|(p, id)| self.static_match(p, *id))
                        {
                            self.obligations[origin].0.clone()
                        } else {
                            ctx.clone()
                        };
                        let value = self.expand(&clause.body, &mut env, &support, output);
                        self.nodes[id].results.push((support, value));
                        return Err(Signal::Progress);
                    }
                }
                Ok(output)
            }
        }
    }
    fn normal(&mut self, id: Id, ctx: &Context) -> Result<Term, Signal> {
        let id = self.force(id, ctx)?;
        match self.nodes[id].node.clone() {
            Node::Unknown(v) => Ok(Term::Var(Var(v))),
            Node::App(n, args) => Ok(Term::App(
                n,
                args.into_iter()
                    .map(|a| self.normal(a, ctx))
                    .collect::<Result<_, _>>()?,
            )),
            _ => unreachable!(),
        }
    }
    fn answer(&mut self, ctx: &Context) -> Result<Answer, Signal> {
        for index in 0..self.obligations.len() {
            let (support, id) = &self.obligations[index];
            let id = *id;
            if support.iter().all(|(k, v)| ctx.get(k) == Some(v)) {
                self.force(id, ctx)?;
            }
        }
        let mut outputs = vec![];
        for (name, id) in self.outputs.clone() {
            outputs.push((name, self.normal(id, ctx)?));
        }
        let mut residual = vec![];
        for index in 0..self.obligations.len() {
            let (support, id) = &self.obligations[index];
            let id = *id;
            if !support.iter().all(|(k, v)| ctx.get(k) == Some(v)) {
                continue;
            }
            if self.nodes[id]
                .results
                .iter()
                .any(|(support, _)| support.iter().all(|(k, v)| ctx.get(k) == Some(v)))
            {
                continue;
            }
            let Node::Call(name, args, output, _) = self.nodes[id].node.clone() else {
                unreachable!("obligations are source calls")
            };
            let mut args = args
                .into_iter()
                .map(|id| self.normal(id, ctx))
                .collect::<Result<Vec<_>, _>>()?;
            args.push(self.normal(output, ctx)?);
            residual.push(Constraint { name, args });
        }
        for id in 0..self.resources.len() {
            if !self.live(id, ctx) {
                continue;
            }
            let name = self.resources[id].constraint.clone();
            let ids = self.resources[id].args.clone();
            let args = ids
                .into_iter()
                .map(|id| self.normal(id, ctx))
                .collect::<Result<Vec<_>, _>>()?;
            residual.push(Constraint { name, args });
        }
        Ok(Answer { outputs, residual })
    }
    /// Post-execution diagnostic of retained source expansions; no hot-path counters.
    pub fn retained_application_results(&self) -> BTreeMap<String, usize> {
        let mut counts = BTreeMap::new();
        for cell in &self.nodes {
            if let Node::Call(name, _, _, _) = &cell.node {
                *counts.entry(name.clone()).or_default() += cell.results.len();
            }
        }
        counts
    }
    /// Retained constructor nodes, excluding observer-created owned answer terms.
    pub fn retained_constructors(&self) -> BTreeMap<String, usize> {
        let mut counts = BTreeMap::new();
        for cell in &self.nodes {
            if let Node::App(name, _) = &cell.node {
                *counts.entry(name.clone()).or_default() += 1;
            }
        }
        counts
    }
    pub fn tick(&mut self) -> Event {
        let Some((ctx, mut cursor, mut round_end)) = self.tasks.pop_front() else {
            return Event::Exhausted;
        };
        // Service an active obligation independently of the observation demand.
        // Child obligations are eligible only after their enclosing choice is selected.
        let mut serviced = Ok(0);
        if cursor >= round_end {
            cursor = 0;
            round_end = self.obligations.len();
        }
        // Freeze the endpoint: freshly appended tail calls cannot keep a round
        // from returning to an older failure or unresolved occurrence.
        while cursor < round_end {
            let index = cursor;
            cursor += 1;
            let (support, id) = &self.obligations[index];
            let id = *id;
            if support.iter().all(|(k, v)| ctx.get(k) == Some(v)) {
                serviced = self.force(id, &ctx);
                break;
            }
        }
        let result = match serviced {
            Err(Signal::Progress | Signal::Split(_) | Signal::Fail) => {
                serviced.map(|_| unreachable!())
            }
            Ok(_) => self.answer(&ctx),
        };
        match result {
            Ok(a) => Event::Answer(a),
            Err(Signal::Progress) => {
                self.tasks.push_back((ctx, cursor, round_end));
                Event::Progress
            }
            Err(Signal::Split(label)) => {
                for value in [false, true] {
                    let mut child = ctx.clone();
                    child.insert(label, value);
                    self.tasks.push_back((child, cursor, round_end));
                }
                Event::Progress
            }
            Err(Signal::Fail) => Event::Progress,
        }
    }
}
