//! Experimental suspended applications for a checked equation-producing source fragment.
//! Context-indexed results preserve application identity. This is not yet a
//! general multihead CHR executor. Optional local pull-tabs lift directly demanded choices.
#[cfg(feature = "candidate-profile")]
pub mod candidate_profile;
mod templates;
use chr_syntax::{Answer, Constraint, Goal, Query, Rule, Term, Var};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;
/// This crate owns the optional traversal counters; cost runners check this value.
pub const COLLECT_WORK_DIAGNOSTICS: bool = cfg!(feature = "work-diagnostics");
type Id = usize;
type Context = BTreeMap<usize, bool>;
type Env = BTreeMap<Var, Id>;
#[derive(Clone)]
enum Plan<T = Term> {
    Value(T),
    Call(String, Vec<T>),
    Choice(Box<Plan<T>>, Box<Plan<T>>),
    Producers(Vec<Action<T>>, Box<Plan<T>>),
    BindOutput(Var, Box<Plan<T>>),
    Fail,
}
#[derive(Clone)]
enum Action<T = Term> {
    Call(Var, String, Vec<T>),
    Post(String, Vec<T>),
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
    MatchDependencies,
}
pub struct Prepared {
    miss_reuse: bool,
    derivation_templates: bool,
    reuse: Reuse,
    pull_tabs: bool,
    clauses: Rc<Vec<Clause>>,
    call_signatures: BTreeSet<(String, usize)>,
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
// Follow completed result edges iteratively; source dependency evaluation remains
// in force_body, under the same active-call and recursive-probe guards.
enum Forced {
    Value(Id),
    Follow(Id),
}
struct Cell {
    node: Node,
    results: Vec<(Context, Id)>,
}
/// Retained ownership, obtained by scanning after execution; not allocation bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetainedGraph {
    pub nodes: usize,
    pub calls: usize,
    pub choices: usize,
    pub births: usize,
    pub results: usize,
    pub obligations: usize,
}
/// Non-mutating inventory. Incompatible supports cannot match any extension of
/// a pending task; this does not prove their referenced nodes are unreachable.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FutureSupports {
    pub pending_tasks: usize,
    pub results: usize,
    pub dead_results: usize,
    pub obligations: usize,
    pub dead_obligations: usize,
    pub births: usize,
    pub dead_births: usize,
    pub consumption_claims: usize,
    pub dead_consumption_claims: usize,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ReclaimedSupports {
    pub results: usize,
    pub consumption_claims: usize,
}
/// Diagnostic entries into distinct operations; not a combined cost model.
#[cfg(feature = "work-diagnostics")]
#[derive(Debug, Default, Clone, Copy)]
pub struct Work {
    pub resource_posts: usize,
    pub output_binders: usize,
    pub resource_candidates: usize,
    pub force_entries: usize,
    pub validation_passes: usize,
    pub validation_entries: usize,
    pub match_entries: usize,
    pub lift_walk_entries: usize,
    pub dependency_entries: usize,
    pub template_hits: usize,
    pub miss_lookups: usize,
    pub miss_hits: usize,
    pub miss_inserts: usize,
}
pub struct Run {
    miss_reuse: bool,
    misses: BTreeMap<Id, Id>,
    recursive_probes: Vec<bool>,
    derivation_templates: bool,
    templates: BTreeMap<(String, Vec<Term>), Rc<templates::Template>>,
    reuse: Reuse,
    #[cfg(feature = "work-diagnostics")]
    work: Work,
    pull_tabs: bool,
    clauses: Rc<Vec<Clause>>,
    nodes: Vec<Cell>,
    tasks: VecDeque<(Context, usize, usize)>,
    obligations: Vec<(Context, Id)>,
    outputs: Vec<(String, Id)>,
    fresh: u64,
    births: Vec<Context>,
    resources: Vec<Resource>,
    forcing: Vec<Id>,
    normalizing: Vec<Id>,
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
                    producers.push(Action::Post(c.name.clone(), c.args.clone()));
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
        Plan::BindOutput(_, p) => calls(p, out),
        Plan::Producers(ps, p) => {
            out.extend(ps.iter().filter_map(|action| match action {
                Action::Call(_, n, a) => Some((n.clone(), a.len())),
                Action::Post(_, _) => None,
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
            let mut body = plan(&rule.body, *output, &set, &resource_signatures)?;
            fn posts_output(p: &Plan, output: Var) -> bool {
                match p {
                    Plan::Producers(actions, last) => {
                        actions.iter().any(|a| {
                            let Action::Post(_, args) = a else {
                                return false;
                            };
                            let mut vars = vec![];
                            for t in args {
                                variables(t, &mut vars);
                            }
                            vars.contains(&output)
                        }) || posts_output(last, output)
                    }
                    Plan::Choice(a, b) => posts_output(a, output) || posts_output(b, output),
                    Plan::BindOutput(_, p) => posts_output(p, output),
                    _ => false,
                }
            }
            if posts_output(&body, *output) {
                body = Plan::BindOutput(*output, Box::new(body));
            }
            clauses.push(Clause {
                name: head.name.clone(),
                inputs,
                body,
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
                Plan::BindOutput(_, p) => pure_plan(p, allowed),
                Plan::Call(n, args) => allowed.contains(&(n.clone(), args.len())),
                Plan::Choice(_, _) => false,
                Plan::Producers(actions, p) => {
                    actions.iter().all(|a| match a {
                        Action::Call(_, n, args) => allowed.contains(&(n.clone(), args.len())),
                        Action::Post(_, _) => false,
                    }) && pure_plan(p, allowed)
                }
            }
        }
        let mut pure = if reuse != Reuse::CurrentContext {
            signatures.clone()
        } else {
            BTreeSet::new()
        };
        if reuse != Reuse::CurrentContext {
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
            miss_reuse: false,
            derivation_templates: false,
            reuse,
            pull_tabs: false,
            clauses: Rc::new(clauses),
            call_signatures: signatures
                .into_iter()
                .map(|(name, inputs)| (name, inputs + 1))
                .collect(),
        })
    }
    /// Reuse residual source derivations; instantiate fresh identities per application.
    pub fn with_derivation_templates(mut self) -> Self {
        self.derivation_templates = true;
        self
    }
    /// Enable the experimental direct-argument local rewrite, independently of cache validity.
    pub fn with_pull_tabs(mut self) -> Self {
        self.pull_tabs = true;
        self
    }
    /// Reuse nonrecursive unsuccessful probes within one source service turn.
    /// A new turn discards all misses before any further source work.
    pub fn with_miss_reuse(mut self) -> Self {
        self.miss_reuse = true;
        self
    }
    pub fn start(&self, query: Query) -> Result<Run, String> {
        let mut run = Run {
            miss_reuse: self.miss_reuse,
            misses: BTreeMap::new(),
            recursive_probes: vec![],
            derivation_templates: self.derivation_templates,
            templates: BTreeMap::new(),
            reuse: self.reuse,
            #[cfg(feature = "work-diagnostics")]
            work: Work::default(),
            pull_tabs: self.pull_tabs,
            clauses: self.clauses.clone(),
            nodes: vec![],
            tasks: VecDeque::from([(Context::new(), 0, 0)]),
            obligations: vec![],
            outputs: vec![],
            fresh: 0,
            births: vec![],
            resources: vec![],
            forcing: vec![],
            normalizing: vec![],
        };
        let mut env = Env::new();
        let mut writers = BTreeSet::new();
        let mut edges = BTreeMap::new();
        for c in &query.constraints {
            if !self
                .call_signatures
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
        for c in query.constraints {
            if !self
                .call_signatures
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
        self.expand_plan(p, env, ctx, output, &mut Self::term)
    }
    fn expand_plan<T>(
        &mut self,
        p: &Plan<T>,
        env: &mut Env,
        ctx: &Context,
        output: Id,
        term: &mut impl FnMut(&mut Self, &T, &mut Env) -> Id,
    ) -> Id {
        match p {
            Plan::Value(t) => term(self, t, env),
            Plan::Call(n, args) => {
                let args = args.iter().map(|t| term(self, t, env)).collect();
                self.call(n.clone(), args, output, ctx)
            }
            Plan::Fail => self.push(Node::Fail),
            Plan::BindOutput(var, body) => {
                #[cfg(feature = "work-diagnostics")]
                {
                    self.work.output_binders += 1;
                }
                // Posts can precede the equation defining this call's output.
                // Keep one local reference, then link it to the contextual result.
                let slot = self.term(&Term::Var(*var), env);
                let value = self.expand_plan(body, env, ctx, output, term);
                self.nodes[slot].node = Node::Alias(value);
                value
            }
            Plan::Choice(a, b) => {
                let label = self.births.len();
                self.births.push(ctx.clone());
                let mut left = ctx.clone();
                left.insert(label, false);
                let mut right = ctx.clone();
                right.insert(label, true);
                let a = self.expand_plan(a, &mut env.clone(), &left, output, term);
                let b = self.expand_plan(b, &mut env.clone(), &right, output, term);
                self.push(Node::Choice(label, a, b))
            }
            Plan::Producers(ps, p) => {
                for action in ps {
                    match action {
                        Action::Call(v, n, args) => {
                            let args = args.iter().map(|t| term(self, t, env)).collect();
                            self.producer(n.clone(), args, *v, env, ctx);
                        }
                        Action::Post(name, args) => {
                            #[cfg(feature = "work-diagnostics")]
                            {
                                self.work.resource_posts += 1;
                            }
                            let args = args.iter().map(|t| term(self, t, env)).collect();
                            self.resources.push(Resource {
                                constraint: name.clone(),
                                args,
                                birth: ctx.clone(),
                                consumed: vec![],
                            });
                        }
                    }
                }
                self.expand_plan(p, env, ctx, output, term)
            }
        }
    }
    fn template_term(
        &mut self,
        t: &templates::Expr,
        env: &mut Env,
        ground: &mut BTreeMap<*const templates::ExprNode, Id>,
    ) -> Id {
        let key = Rc::as_ptr(t);
        if t.ground
            && let Some(id) = ground.get(&key)
        {
            return *id;
        }
        let id = match &t.kind {
            templates::ExprKind::Local(v) => self.term(&Term::Var(*v), env),
            templates::ExprKind::App(n, xs) => {
                let children = xs
                    .iter()
                    .map(|x| self.template_term(x, env, ground))
                    .collect();
                self.push(Node::App(n.clone(), children))
            }
        };
        // Only immutable ground values are independent of branch-local bindings.
        if t.ground {
            ground.insert(key, id);
        }
        id
    }
    fn expand_template(
        &mut self,
        p: &Plan<templates::Expr>,
        env: &mut Env,
        ctx: &Context,
        output: Id,
        ground: &mut BTreeMap<*const templates::ExprNode, Id>,
    ) -> Id {
        self.expand_plan(p, env, ctx, output, &mut |run, t, env| {
            run.template_term(t, env, ground)
        })
    }
    fn matches(
        &mut self,
        p: &Term,
        id: Id,
        ctx: &Context,
        env: &mut Cow<'_, Env>,
    ) -> Result<bool, Signal> {
        #[cfg(feature = "work-diagnostics")]
        {
            self.work.match_entries += 1;
        }
        match p {
            Term::Var(v) => {
                if let Some(old) = env.get(v).copied() {
                    return Ok(self.normal(old, ctx)? == self.normal(id, ctx)?);
                }
                let owned = if matches!(env, Cow::Borrowed(_)) {
                    #[cfg(feature = "candidate-profile")]
                    let _scope =
                        candidate_profile::Scope::new(candidate_profile::Phase::Environment);
                    env.to_mut()
                } else {
                    env.to_mut()
                };
                owned.insert(*v, id);
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
            #[cfg(feature = "work-diagnostics")]
            {
                self.work.resource_candidates += 1;
            }
            if selected.contains(&id) || !self.live(id, ctx) {
                continue;
            }
            let row = &self.resources[id];
            if row.constraint != head.name || row.args.len() != head.args.len() {
                continue;
            }
            // Existing resource argument vectors are immutable. Forcing may append
            // resources, so copy each ID before the recursive mutable call.
            let mut next = Cow::Borrowed(&env);
            let mut matched = true;
            for (index, p) in head.args.iter().enumerate() {
                let arg = self.resources[id].args[index];
                if !self.matches(p, arg, ctx, &mut next)? {
                    matched = false;
                    break;
                }
            }
            if matched {
                let ids = {
                    #[cfg(feature = "candidate-profile")]
                    let _scope = candidate_profile::Scope::new(candidate_profile::Phase::Selection);
                    let mut ids = selected.clone();
                    ids.push(id);
                    ids
                };
                let next = match next {
                    Cow::Owned(env) => env,
                    Cow::Borrowed(env) => {
                        #[cfg(feature = "candidate-profile")]
                        let _scope =
                            candidate_profile::Scope::new(candidate_profile::Phase::Environment);
                        env.clone()
                    }
                };
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
    fn force(&mut self, mut id: Id, ctx: &Context) -> Result<Id, Signal> {
        let forcing_depth = self.forcing.len();
        let probe_depth = self.recursive_probes.len();
        let result = (|| {
            loop {
                if let Node::Call(_, _, output, _) = self.nodes[id].node {
                    if self.forcing.contains(&id) {
                        if self.miss_reuse {
                            self.recursive_probes.fill(true);
                        }
                        // Only completed alias edges imply equality. A recursive matching
                        // dependency with no completed result merely exposes its unknown.
                        let mut path: Vec<Id> = Vec::new();
                        let mut at = id;
                        loop {
                            if let Some(start) = path.iter().position(|x| *x == at) {
                                let canonical = path[start..]
                                    .iter()
                                    .filter_map(|i| {
                                        if let Node::Call(_, _, out, _) = self.nodes[*i].node
                                            && let Node::Unknown(logical) = self.nodes[out].node
                                        {
                                            return Some((logical, out));
                                        }
                                        None
                                    })
                                    .min()
                                    .map(|(_, out)| out)
                                    .unwrap_or(output);
                                return Ok(canonical);
                            }
                            path.push(at);
                            match &self.nodes[at].node {
                                Node::Alias(next) => at = *next,
                                Node::Call(_, _, _, _) => {
                                    let Some((_, next)) =
                                        self.nodes[at].results.iter().rev().find(|(support, _)| {
                                            support.iter().all(|(k, v)| ctx.get(k) == Some(v))
                                        })
                                    else {
                                        return Ok(output);
                                    };
                                    at = *next;
                                }
                                Node::Choice(label, left, right) => {
                                    let Some(side) = ctx.get(label) else {
                                        return Ok(output);
                                    };
                                    at = if *side { *right } else { *left };
                                }
                                _ => return Ok(output),
                            }
                        }
                    }
                    if self.miss_reuse {
                        #[cfg(feature = "work-diagnostics")]
                        {
                            self.work.miss_lookups += 1;
                        }
                        if let Some(output) = self.misses.get(&id).copied() {
                            #[cfg(feature = "work-diagnostics")]
                            {
                                self.work.miss_hits += 1;
                            }
                            return Ok(output);
                        }
                    }
                    self.forcing.push(id);
                    if self.miss_reuse {
                        self.recursive_probes.push(false);
                    }
                }
                match self.force_body(id, ctx)? {
                    Forced::Value(value) => return Ok(value),
                    Forced::Follow(next) => id = next,
                }
            }
        })();
        self.forcing.truncate(forcing_depth);
        self.recursive_probes.truncate(probe_depth);
        result
    }
    fn force_body(&mut self, id: Id, ctx: &Context) -> Result<Forced, Signal> {
        #[cfg(feature = "work-diagnostics")]
        {
            self.work.force_entries += 1;
        }
        // Exposed values already have their result identity. Inspecting them
        // requires neither ownership of the name/children nor graph mutation.
        if matches!(&self.nodes[id].node, Node::Unknown(_) | Node::App(_, _)) {
            return Ok(Forced::Value(id));
        }
        match self.nodes[id].node.clone() {
            Node::Unknown(_) | Node::App(_, _) => unreachable!(),
            Node::Alias(next) => Ok(Forced::Follow(next)),
            Node::Fail => Err(Signal::Fail),
            Node::Choice(label, a, b) => match ctx.get(&label) {
                Some(false) => Ok(Forced::Follow(a)),
                Some(true) => Ok(Forced::Follow(b)),
                None => Err(Signal::Split(label)),
            },
            Node::Call(name, args, output, origin) => {
                if let Some((_, next)) = self.nodes[id]
                    .results
                    .iter()
                    .rev()
                    .find(|(support, _)| support.iter().all(|(k, v)| ctx.get(k) == Some(v)))
                {
                    return Ok(Forced::Follow(*next));
                }
                let clauses = self.clauses.clone();
                for clause in clauses
                    .iter()
                    .filter(|c| c.name == name && c.inputs.len() == args.len())
                {
                    let mut env = Cow::Owned(Env::new());
                    let mut matched = true;
                    for (index, (pattern, arg)) in clause.inputs.iter().zip(&args).enumerate() {
                        let matches = match self.matches(pattern, *arg, ctx, &mut env) {
                            Err(Signal::Split(label))
                                if self.pull_tabs && self.pull_argument(id, index, label, ctx) =>
                            {
                                // The graph rewrite is administrative: preserve the
                                // control's service point for competing source requests.
                                return Err(Signal::Split(label));
                            }
                            result => result?,
                        };
                        if !matches {
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
                            self.partners(&clause.partners, vec![], env.into_owned(), ctx)?
                        else {
                            continue;
                        };
                        // No evaluation intervenes between completed matching and claims.
                        for (id, (_, removed)) in ids.into_iter().zip(&clause.partners) {
                            if *removed {
                                self.resources[id].consumed.push(ctx.clone());
                            }
                        }
                        let support = if clause.reusable_static_match {
                            match self.reuse {
                                Reuse::StaticBirth
                                    if clause
                                        .inputs
                                        .iter()
                                        .zip(&args)
                                        .all(|(p, id)| self.static_match(p, *id)) =>
                                {
                                    self.obligations[origin].0.clone()
                                }
                                Reuse::MatchDependencies => {
                                    let mut support = self.obligations[origin].0.clone();
                                    if clause.inputs.iter().zip(&args).all(|(p, id)| {
                                        self.match_dependencies(p, *id, ctx, &mut support)
                                    }) {
                                        support
                                    } else {
                                        ctx.clone()
                                    }
                                }
                                _ => ctx.clone(),
                            }
                        } else {
                            ctx.clone()
                        };
                        let value = if let Some(template) = self.derivation_template(clause, &args)
                        {
                            self.expand_template(
                                &template.body,
                                &mut Env::new(),
                                &support,
                                output,
                                &mut BTreeMap::new(),
                            )
                        } else {
                            self.expand(&clause.body, &mut env, &support, output)
                        };
                        self.nodes[id].results.push((support, value));
                        return Err(Signal::Progress);
                    }
                }
                if self.miss_reuse && !self.recursive_probes.last().copied().unwrap_or(true) {
                    self.misses.insert(id, output);
                    #[cfg(feature = "work-diagnostics")]
                    {
                        self.work.miss_inserts += 1;
                    }
                }
                Ok(Forced::Value(output))
            }
        }
    }
    fn closed_key(&self, id: Id, fuel: &mut usize) -> Option<Term> {
        if *fuel == 0 {
            return None;
        }
        *fuel -= 1;
        let Node::App(n, args) = &self.nodes[id].node else {
            return None;
        };
        Some(Term::App(
            n.clone(),
            args.iter()
                .map(|id| self.closed_key(*id, fuel))
                .collect::<Option<_>>()?,
        ))
    }
    fn derivation_template(
        &mut self,
        clause: &Clause,
        args: &[Id],
    ) -> Option<Rc<templates::Template>> {
        if !self.derivation_templates || !clause.partners.is_empty() {
            return None;
        }
        let mut fuel = 4096;
        let args = args
            .iter()
            .map(|id| self.closed_key(*id, &mut fuel))
            .collect::<Option<Vec<_>>>()?;
        let key = (clause.name.clone(), args);
        if let Some(template) = self.templates.get(&key) {
            #[cfg(feature = "work-diagnostics")]
            {
                self.work.template_hits += 1;
            }
            return Some(template.clone());
        }
        if self.templates.len() == 64 {
            return None;
        }
        let template = Rc::new(templates::derive(clause, &key.1, &self.clauses)?);
        self.templates.insert(key, template.clone());
        Some(template)
    }
    /// Query-owned template count and source calls followed during their construction.
    pub fn retained_derivation_templates(&self) -> (usize, usize) {
        (
            self.templates.len(),
            self.templates.values().map(|t| t.followed_calls).sum(),
        )
    }
    // The successful head match is already established. Read its dependencies
    // without running producers or resolving any additional source work.
    fn match_dependencies(
        &mut self,
        pattern: &Term,
        mut id: Id,
        ctx: &Context,
        support: &mut Context,
    ) -> bool {
        let Term::App(name, patterns) = pattern else {
            return true;
        };
        loop {
            #[cfg(feature = "work-diagnostics")]
            {
                self.work.dependency_entries += 1;
            }
            match &self.nodes[id].node {
                Node::Alias(next) => id = *next,
                Node::Call(..) => {
                    let Some((validity, next)) =
                        self.nodes[id].results.iter().rev().find(|(validity, _)| {
                            validity.iter().all(|(k, v)| ctx.get(k) == Some(v))
                        })
                    else {
                        return false;
                    };
                    support.extend(validity.iter().map(|(k, v)| (*k, *v)));
                    id = *next;
                }
                Node::Choice(label, a, b) => {
                    let Some(side) = ctx.get(label) else {
                        return false;
                    };
                    support.insert(*label, *side);
                    support.extend(self.births[*label].iter().map(|(k, v)| (*k, *v)));
                    id = if *side { *b } else { *a };
                }
                Node::App(actual, args) if actual == name && args.len() == patterns.len() => break,
                _ => return false,
            }
        }
        for (index, p) in patterns.iter().enumerate() {
            let Node::App(_, args) = &self.nodes[id].node else {
                unreachable!()
            };
            let child = args[index];
            if !self.match_dependencies(p, child, ctx, support) {
                return false;
            }
        }
        true
    }
    // Follow only already available edges: nested demand and resource settlement
    // are not direct-argument redexes. Never force additional source work here.
    fn pull_argument(&mut self, call: Id, index: usize, label: usize, ctx: &Context) -> bool {
        let Node::Call(name, args, output, origin) = self.nodes[call].node.clone() else {
            unreachable!()
        };
        let dependencies = self.reuse == Reuse::MatchDependencies;
        let mut validity = if dependencies {
            self.obligations[origin].0.clone()
        } else {
            ctx.clone()
        };
        let mut at = args[index];
        let (left, right) = loop {
            #[cfg(feature = "work-diagnostics")]
            {
                self.work.lift_walk_entries += 1;
            }
            match &self.nodes[at].node {
                Node::Alias(next) => at = *next,
                Node::Call(..) => {
                    let Some((support, next)) =
                        self.nodes[at].results.iter().rev().find(|(support, _)| {
                            support.iter().all(|(k, v)| ctx.get(k) == Some(v))
                        })
                    else {
                        return false;
                    };
                    if dependencies {
                        validity.extend(support.iter().map(|(k, v)| (*k, *v)));
                    }
                    at = *next;
                }
                Node::Choice(found, a, b) => {
                    if dependencies {
                        validity.extend(self.births[*found].iter().map(|(k, v)| (*k, *v)));
                    }
                    if let Some(side) = ctx.get(found) {
                        if dependencies {
                            validity.insert(*found, *side);
                        }
                        at = if *side { *b } else { *a };
                    } else if *found == label {
                        break (*a, *b);
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        };
        let mut children = Vec::with_capacity(2);
        for (side, argument) in [(false, left), (true, right)] {
            let mut support = validity.clone();
            support.insert(label, side);
            let mut substituted = args.clone();
            substituted[index] = argument;
            children.push(self.call(name.clone(), substituted, output, &support));
        }
        let lifted = self.push(Node::Choice(label, children[0], children[1]));
        self.nodes[call].results.push((validity, lifted));
        true
    }
    fn normal(&mut self, id: Id, ctx: &Context) -> Result<Term, Signal> {
        let id = self.force(id, ctx)?;
        let (name, len) = match &self.nodes[id].node {
            Node::Unknown(v) => return Ok(Term::Var(Var(*v))),
            Node::App(name, children) => (name.clone(), children.len()),
            _ => unreachable!(),
        };
        if self.normalizing.contains(&id) {
            return Err(Signal::Fail);
        }
        self.normalizing.push(id);
        let result = (|| {
            let mut terms = Vec::with_capacity(len);
            for index in 0..len {
                let Node::App(_, children) = &self.nodes[id].node else {
                    unreachable!()
                };
                let child = children[index];
                terms.push(self.normal(child, ctx)?);
            }
            Ok(Term::App(name, terms))
        })();
        self.normalizing.pop();
        result
    }

    // Check completed equations independently of output demand. This traversal
    // does not force producers or select choices, so it cannot reorder claims.
    fn finite_result(&mut self, root: Id, ctx: &Context) -> Result<(), Signal> {
        fn visit(
            run: &Run,
            id: Id,
            ctx: &Context,
            path: &mut Vec<Id>,
            done: &mut BTreeSet<Id>,
            #[cfg(feature = "work-diagnostics")] entries: &mut usize,
        ) -> Result<(), Signal> {
            #[cfg(feature = "work-diagnostics")]
            {
                *entries += 1;
            }
            if let Some(start) = path.iter().position(|old| *old == id) {
                return if path[start..]
                    .iter()
                    .any(|i| matches!(run.nodes[*i].node, Node::App(..)))
                {
                    Err(Signal::Fail)
                } else {
                    Ok(())
                };
            }
            if done.contains(&id) {
                return Ok(());
            }
            path.push(id);
            match &run.nodes[id].node {
                Node::App(_, children) => {
                    for child in children {
                        visit(
                            run,
                            *child,
                            ctx,
                            path,
                            done,
                            #[cfg(feature = "work-diagnostics")]
                            entries,
                        )?;
                    }
                }
                Node::Alias(next) => visit(
                    run,
                    *next,
                    ctx,
                    path,
                    done,
                    #[cfg(feature = "work-diagnostics")]
                    entries,
                )?,
                Node::Call(..) => {
                    if let Some((_, next)) = run.nodes[id]
                        .results
                        .iter()
                        .rev()
                        .find(|(support, _)| support.iter().all(|(k, v)| ctx.get(k) == Some(v)))
                    {
                        visit(
                            run,
                            *next,
                            ctx,
                            path,
                            done,
                            #[cfg(feature = "work-diagnostics")]
                            entries,
                        )?;
                    }
                }
                Node::Choice(label, left, right) => {
                    if let Some(side) = ctx.get(label) {
                        visit(
                            run,
                            if *side { *right } else { *left },
                            ctx,
                            path,
                            done,
                            #[cfg(feature = "work-diagnostics")]
                            entries,
                        )?;
                    }
                }
                Node::Unknown(_) | Node::Fail => {}
            }
            path.pop();
            done.insert(id);
            Ok(())
        }
        #[cfg(feature = "work-diagnostics")]
        let mut entries = 0;
        let result = visit(
            self,
            root,
            ctx,
            &mut Vec::new(),
            &mut BTreeSet::new(),
            #[cfg(feature = "work-diagnostics")]
            &mut entries,
        );
        #[cfg(feature = "work-diagnostics")]
        {
            self.work.validation_passes += 1;
            self.work.validation_entries += entries;
        }
        result
    }

    fn answer(&mut self, ctx: &Context) -> Result<Answer, Signal> {
        for index in 0..self.obligations.len() {
            let (support, id) = &self.obligations[index];
            let id = *id;
            if support.iter().all(|(k, v)| ctx.get(k) == Some(v)) {
                self.finite_result(id, ctx)?;
                self.force(id, ctx)?;
            }
        }
        let mut outputs = Vec::with_capacity(self.outputs.len());
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
            let mut terms = Vec::with_capacity(args.len() + 1);
            for id in args {
                terms.push(self.normal(id, ctx)?);
            }
            terms.push(self.normal(output, ctx)?);
            let args = terms;
            residual.push(Constraint { name, args });
        }
        for id in 0..self.resources.len() {
            if !self.live(id, ctx) {
                continue;
            }
            let name = self.resources[id].constraint.clone();
            let ids = self.resources[id].args.clone();
            let mut args = Vec::with_capacity(ids.len());
            for id in ids {
                args.push(self.normal(id, ctx)?);
            }
            residual.push(Constraint { name, args });
        }
        Ok(Answer { outputs, residual })
    }
    pub fn retained_graph(&self) -> RetainedGraph {
        RetainedGraph {
            nodes: self.nodes.len(),
            calls: self
                .nodes
                .iter()
                .filter(|c| matches!(c.node, Node::Call(..)))
                .count(),
            choices: self
                .nodes
                .iter()
                .filter(|c| matches!(c.node, Node::Choice(..)))
                .count(),
            births: self.births.len(),
            results: self.nodes.iter().map(|c| c.results.len()).sum(),
            obligations: self.obligations.len(),
        }
    }
    pub fn future_supports(&self) -> FutureSupports {
        let dead = |support: &Context| {
            !self
                .tasks
                .iter()
                .any(|(task, _, _)| supports_compatible(support, task))
        };
        FutureSupports {
            pending_tasks: self.tasks.len(),
            results: self.nodes.iter().map(|c| c.results.len()).sum(),
            dead_results: self
                .nodes
                .iter()
                .flat_map(|c| &c.results)
                .filter(|(s, _)| dead(s))
                .count(),
            obligations: self.obligations.len(),
            dead_obligations: self.obligations.iter().filter(|(s, _)| dead(s)).count(),
            births: self.births.len(),
            dead_births: self.births.iter().filter(|s| dead(s)).count(),
            consumption_claims: self.resources.iter().map(|r| r.consumed.len()).sum(),
            dead_consumption_claims: self
                .resources
                .iter()
                .flat_map(|r| &r.consumed)
                .filter(|s| dead(s))
                .count(),
        }
    }
    /// Reclaim only metadata whose support conflicts with every pending task.
    /// Call between service steps. Node, label and obligation identities survive.
    pub fn reclaim_incompatible_supports(&mut self) -> ReclaimedSupports {
        let tasks = &self.tasks;
        let possible = |support: &Context| {
            tasks
                .iter()
                .any(|(task, _, _)| supports_compatible(support, task))
        };
        let mut removed = ReclaimedSupports::default();
        for cell in &mut self.nodes {
            let before = cell.results.len();
            cell.results.retain(|(support, _)| possible(support));
            if cell.results.len() != before {
                removed.results += before - cell.results.len();
                cell.results.shrink_to_fit();
            }
        }
        for resource in &mut self.resources {
            let before = resource.consumed.len();
            resource.consumed.retain(possible);
            if resource.consumed.len() != before {
                removed.consumption_claims += before - resource.consumed.len();
                resource.consumed.shrink_to_fit();
            }
        }
        removed
    }
    #[cfg(feature = "work-diagnostics")]
    pub fn work(&self) -> Work {
        self.work
    }
    /// Post-execution call-result edges, including administrative pull-tab edges.
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
        self.misses.clear();
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
                serviced = self
                    .finite_result(id, &ctx)
                    .and_then(|()| self.force(id, &ctx));
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

fn supports_compatible(support: &Context, task: &Context) -> bool {
    support
        .iter()
        .all(|(label, value)| task.get(label).is_none_or(|assigned| assigned == value))
}

#[cfg(test)]
mod pull_tab_tests {
    use super::*;
    use chr_syntax::{atom, c, eq, v};

    #[test]
    fn candidate_bindings_borrow_until_extension_and_never_leak() {
        use std::borrow::Cow;
        let mut run = Prepared::new(vec![])
            .unwrap()
            .start(Query {
                constraints: vec![],
                outputs: vec![],
            })
            .unwrap();
        let a = run.push(Node::App("a".into(), vec![]));
        let b = run.push(Node::App("b".into(), vec![]));
        let choice = run.push(Node::Choice(0, a, b));
        let base = Env::from([(Var(0), a)]);
        let ctx = Context::new();
        let mut trial = Cow::Borrowed(&base);
        assert!(matches!(run.matches(&v(0), b, &ctx, &mut trial), Ok(false)));
        assert!(matches!(trial, Cow::Borrowed(_)));
        assert!(matches!(run.matches(&v(1), a, &ctx, &mut trial), Ok(true)));
        assert!(matches!(trial, Cow::Owned(_)));
        assert!(matches!(run.matches(&v(1), b, &ctx, &mut trial), Ok(false)));
        drop(trial);
        assert_eq!(base.len(), 1);
        assert!(!base.contains_key(&Var(1)));
        let mut trial = Cow::Borrowed(&base);
        assert!(matches!(
            run.matches(&atom("a"), choice, &ctx, &mut trial),
            Err(Signal::Split(0))
        ));
        assert!(matches!(trial, Cow::Borrowed(_)));
    }

    #[test]
    fn support_compatibility_matches_exhaustive_total_extensions() {
        let partials = (0..27)
            .map(|mut code| {
                let mut c = Context::new();
                for label in 0..3 {
                    match code % 3 {
                        1 => {
                            c.insert(label, false);
                        }
                        2 => {
                            c.insert(label, true);
                        }
                        _ => (),
                    }
                    code /= 3;
                }
                c
            })
            .collect::<Vec<_>>();
        for support in &partials {
            for task in &partials {
                let possible = (0..8).any(|bits| {
                    support
                        .iter()
                        .chain(task)
                        .all(|(label, value)| ((bits >> label) & 1 != 0) == *value)
                });
                assert_eq!(supports_compatible(support, task), possible);
            }
        }
    }

    #[test]
    fn fresh_derivation_eliminates_recursive_call_expansion() {
        use chr_syntax::{or, t};
        let rules = vec![
            Rule::simplify(
                "base",
                [c("build", [atom("z"), v(0)])],
                or(eq(v(0), t("box", [v(99)])), eq(v(0), t("other", [v(99)]))),
            ),
            Rule::simplify(
                "step",
                [c("build", [t("s", [v(0)]), v(1)])],
                c("build", [v(0), v(1)]).into(),
            ),
        ];
        let depth = (0..8).fold(atom("z"), |x, _| t("s", [x]));
        let mut run = Prepared::new(rules)
            .unwrap()
            .with_derivation_templates()
            .start(Query {
                constraints: vec![
                    c("build", [depth.clone(), v(100)]),
                    c("build", [depth, v(101)]),
                ],
                outputs: vec![("x".into(), Var(100)), ("y".into(), Var(101))],
            })
            .unwrap();
        let mut answers = 0;
        for _ in 0..1000 {
            match run.tick() {
                Event::Answer(_) => answers += 1,
                Event::Exhausted => break,
                Event::Progress => {}
            }
        }
        assert_eq!(answers, 4);
        assert_eq!(run.retained_derivation_templates(), (1, 8));
        #[cfg(feature = "work-diagnostics")]
        assert_eq!(run.work().template_hits, 1);
        assert_eq!(run.retained_application_results().get("build"), Some(&2));
    }

    #[test]
    fn derivation_shares_growing_terms_without_exhausting_construction_budget() {
        use chr_syntax::t;
        let rules = vec![
            Rule::simplify("base", [c("grow", [atom("z"), v(0), v(1)])], eq(v(1), v(0))),
            Rule::simplify(
                "step",
                [c("grow", [t("s", [v(0)]), v(1), v(2)])],
                c("grow", [v(0), t("pair", [v(1), v(1)]), v(2)]).into(),
            ),
        ];
        let depth = (0..12).fold(atom("z"), |x, _| t("s", [x]));
        let mut run = Prepared::new(rules)
            .unwrap()
            .with_derivation_templates()
            .start(Query {
                constraints: vec![c("grow", [depth, atom("leaf"), v(100)])],
                outputs: vec![],
            })
            .unwrap();
        assert!(matches!(run.tick(), Event::Progress));
        let template = run.templates.values().next().unwrap();
        assert_eq!(template.followed_calls, 12);
        assert!(matches!(&template.body, Plan::Value(_)));
        assert!(
            run.nodes.len() < 100,
            "ground duplication must retain graph sharing"
        );
    }

    #[test]
    fn shared_derivation_size_limit_keeps_ordinary_execution() {
        use chr_syntax::t;
        let value = t("wide", (0..17000).map(|_| v(0)).collect::<Vec<_>>());
        let prepared = Prepared::new(vec![Rule::simplify(
            "wide",
            [c("wide", [v(0), v(1)])],
            eq(v(1), value.clone()),
        )])
        .unwrap()
        .with_derivation_templates();
        assert!(
            templates::derive(&prepared.clauses[0], &[atom("leaf")], &prepared.clauses).is_none()
        );
        let mut run = prepared
            .start(Query {
                constraints: vec![c("wide", [atom("leaf"), v(100)])],
                outputs: vec![("x".into(), Var(100))],
            })
            .unwrap();
        let mut answer = None;
        for _ in 0..100 {
            match run.tick() {
                Event::Answer(a) => answer = Some(a),
                Event::Exhausted => break,
                Event::Progress => {}
            }
        }
        assert_eq!(
            answer.unwrap().outputs,
            vec![("x".into(), t("wide", vec![atom("leaf"); 17000]))]
        );
        assert_eq!(run.retained_derivation_templates(), (0, 0));
    }

    #[test]
    fn shared_derivation_retains_call_at_fuel_bound() {
        use chr_syntax::t;
        let rules = vec![
            Rule::simplify(
                "base",
                [c("walk", [atom("z"), v(0)])],
                eq(v(0), atom("done")),
            ),
            Rule::simplify(
                "step",
                [c("walk", [t("s", [v(0)]), v(1)])],
                c("walk", [v(0), v(1)]).into(),
            ),
        ];
        let depth = (0..66).fold(atom("z"), |x, _| t("s", [x]));
        let mut run = Prepared::new(rules)
            .unwrap()
            .with_derivation_templates()
            .start(Query {
                constraints: vec![c("walk", [depth, v(100)])],
                outputs: vec![("x".into(), Var(100))],
            })
            .unwrap();
        assert!(matches!(run.tick(), Event::Progress));
        let template = run.templates.values().next().unwrap();
        assert_eq!(template.followed_calls, 64);
        assert!(matches!(&template.body, Plan::Call(n, _) if n == "walk"));
        let mut answer = None;
        for _ in 0..1000 {
            match run.tick() {
                Event::Answer(a) => answer = Some(a),
                Event::Exhausted => break,
                Event::Progress => {}
            }
        }
        assert_eq!(answer.unwrap().outputs, vec![("x".into(), atom("done"))]);
    }

    #[test]
    fn template_entry_limit_preserves_distinct_and_repeated_requests() {
        let rule = Rule::simplify("id", [c("id", [v(0), v(1)])], eq(v(1), v(0)));
        let keys = (0..65).chain(std::iter::once(0)).collect::<Vec<_>>();
        let mut run = Prepared::new(vec![rule])
            .unwrap()
            .with_derivation_templates()
            .start(Query {
                constraints: keys
                    .iter()
                    .enumerate()
                    .map(|(i, k)| c("id", [atom(&format!("k{k}")), v(100 + i as u64)]))
                    .collect(),
                outputs: keys
                    .iter()
                    .enumerate()
                    .map(|(i, _)| (format!("o{i}"), Var(100 + i as u64)))
                    .collect(),
            })
            .unwrap();
        let mut answered = false;
        let mut exhausted = false;
        for _ in 0..1000 {
            match run.tick() {
                Event::Answer(a) => {
                    assert!(!answered);
                    answered = true;
                    assert_eq!(
                        a.outputs,
                        keys.iter()
                            .enumerate()
                            .map(|(i, k)| (format!("o{i}"), atom(&format!("k{k}"))))
                            .collect::<Vec<_>>()
                    );
                    assert!(a.residual.is_empty());
                }
                Event::Exhausted => {
                    exhausted = true;
                    break;
                }
                Event::Progress => {}
            }
        }
        assert!(answered && exhausted);
        assert_eq!(run.retained_derivation_templates(), (64, 0));
        #[cfg(feature = "work-diagnostics")]
        assert_eq!(run.work().template_hits, 1);
    }

    #[test]
    fn pure_match_records_observed_choice_and_producer_support() {
        let rules = vec![Rule::simplify(
            "use",
            [c("use", [atom("a"), v(0)])],
            eq(v(0), atom("done")),
        )];
        let mut run = Prepared::with_reuse(rules, Reuse::MatchDependencies)
            .unwrap()
            .start(Query {
                constraints: vec![],
                outputs: vec![],
            })
            .unwrap();
        // Label 1 is a producer's conditional birth; label 2 is unrelated.
        // The matched choice is label 0 and is born under label 1.
        run.births = vec![Context::from([(1, true)]), Context::new(), Context::new()];
        let a = run.push(Node::App("a".into(), vec![]));
        let b = run.push(Node::App("b".into(), vec![]));
        let choice = run.push(Node::Choice(0, a, b));
        let producer_output = run.push(Node::Unknown(10));
        let producer = run.call(
            "producer".into(),
            vec![],
            producer_output,
            &Context::from([(1, true)]),
        );
        run.nodes[producer]
            .results
            .push((Context::from([(1, true)]), choice));
        let output = run.push(Node::Unknown(11));
        let call = run.call("use".into(), vec![producer], output, &Context::new());
        let ctx = Context::from([(0, false), (1, true), (2, false)]);
        assert!(matches!(run.force(call, &ctx), Err(Signal::Progress)));
        assert_eq!(
            run.nodes[call].results[0].0,
            Context::from([(0, false), (1, true)])
        );
        let other = Context::from([(0, false), (1, true), (2, true)]);
        assert!(run.force(call, &other).is_ok());
        assert_eq!(run.nodes[call].results.len(), 1);
        let opposite = Context::from([(0, true), (1, true), (2, true)]);
        assert!(matches!(run.force(call, &opposite), Ok(id) if id == output));
        let born_output = run.push(Node::Unknown(12));
        let born = run.call(
            "use".into(),
            vec![producer],
            born_output,
            &Context::from([(2, false)]),
        );
        assert!(matches!(run.force(born, &ctx), Err(Signal::Progress)));
        assert_eq!(run.nodes[born].results[0].0, ctx);
    }

    #[test]
    fn lifted_calls_do_not_inherit_unrelated_rewrite_choices() {
        let rules = vec![Rule::simplify(
            "use",
            [c("use", [atom("a"), v(0)])],
            eq(v(0), atom("done")),
        )];
        let mut run = Prepared::with_reuse(rules, Reuse::MatchDependencies)
            .unwrap()
            .with_pull_tabs()
            .start(Query {
                constraints: vec![],
                outputs: vec![],
            })
            .unwrap();
        run.births = vec![
            Context::new(),
            Context::new(),
            Context::new(),
            Context::from([(0, true)]),
            Context::new(),
        ];
        let a = run.push(Node::App("a".into(), vec![]));
        let b = run.push(Node::App("b".into(), vec![]));
        let choice = run.push(Node::Choice(3, a, b));
        let out = run.push(Node::Unknown(1));
        let producer = run.call("producer".into(), vec![], out, &Context::from([(1, false)]));
        run.nodes[producer]
            .results
            .push((Context::from([(1, false)]), choice));
        let call = run.call(
            "use".into(),
            vec![producer],
            out,
            &Context::from([(2, true)]),
        );
        assert!(matches!(
            run.force(
                call,
                &Context::from([(0, true), (1, false), (2, true), (4, false)])
            ),
            Err(Signal::Split(3))
        ));
        assert_eq!(
            run.nodes[call].results[0].0,
            Context::from([(0, true), (1, false), (2, true)])
        );
        let Node::Choice(_, left, right) = run.nodes[run.nodes[call].results[0].1].node else {
            panic!()
        };
        for (side, child) in [(false, left), (true, right)] {
            let Node::Call(_, _, _, origin) = run.nodes[child].node else {
                panic!()
            };
            assert_eq!(
                run.obligations[origin].0,
                Context::from([(0, true), (1, false), (2, true), (3, side)])
            );
        }
    }

    #[test]
    fn consuming_match_keeps_full_context_validity() {
        let rules = vec![Rule::simplify(
            "take",
            [c("take", [atom("a"), v(0)]), c("token", [])],
            eq(v(0), atom("done")),
        )];
        let mut run = Prepared::with_reuse(rules, Reuse::MatchDependencies)
            .unwrap()
            .start(Query {
                constraints: vec![c("token", [])],
                outputs: vec![],
            })
            .unwrap();
        run.births.push(Context::new());
        let arg = run.push(Node::App("a".into(), vec![]));
        let output = run.push(Node::Unknown(10));
        let call = run.call("take".into(), vec![arg], output, &Context::new());
        for side in [false, true] {
            let ctx = Context::from([(0, side)]);
            assert!(matches!(run.force(call, &ctx), Err(Signal::Progress)));
            assert_eq!(run.nodes[call].results.last().unwrap().0, ctx);
            assert_eq!(run.resources[0].consumed.last().unwrap(), &ctx);
        }
        assert_eq!(run.resources[0].consumed.len(), 2);
    }

    #[test]
    fn demanded_choice_becomes_choice_over_conditional_calls() {
        let rules = vec![Rule::simplify(
            "take",
            [c("take", [atom("a"), v(0)])],
            eq(v(0), atom("done")),
        )];
        let mut run = Prepared::new(rules)
            .unwrap()
            .with_pull_tabs()
            .start(Query {
                constraints: vec![],
                outputs: vec![],
            })
            .unwrap();
        let ctx = Context::new();
        let a = run.push(Node::App("a".into(), vec![]));
        let b = run.push(Node::App("b".into(), vec![]));
        run.births.push(ctx.clone());
        let choice = run.push(Node::Choice(0, a, b));
        let alias = run.push(Node::Alias(choice));
        let output = run.push(Node::Unknown(0));
        let call = run.call("take".into(), vec![alias], output, &ctx);
        assert!(
            matches!(run.force(call, &ctx), Err(Signal::Split(0))),
            "the rewrite must preserve the control split service point"
        );
        let lifted = run.nodes[call].results[0].1;
        let Node::Choice(label, left, right) = run.nodes[lifted].node else {
            panic!("expected a choice over copied calls")
        };
        assert_eq!(label, 0);
        assert_eq!(run.births.len(), 1);
        assert_eq!(
            run.retained_graph(),
            RetainedGraph {
                nodes: 9,
                calls: 3,
                choices: 2,
                births: 1,
                results: 1,
                obligations: 3,
            }
        );
        #[cfg(feature = "work-diagnostics")]
        {
            let work = run.work();
            assert_eq!(work.force_entries, 3);
            assert_eq!(work.match_entries, 1);
            assert_eq!(work.lift_walk_entries, 2);
        }
        for (side, child, arg) in [(false, left, a), (true, right, b)] {
            let Node::Call(ref name, ref args, out, origin) = run.nodes[child].node else {
                panic!("choice arm must contain a call")
            };
            assert_eq!(name, "take");
            assert_eq!(args, &[arg]);
            assert_eq!(out, output);
            assert_eq!(run.obligations[origin], (Context::from([(0, side)]), child));
        }
    }
}
