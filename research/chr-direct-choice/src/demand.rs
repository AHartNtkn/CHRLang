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
    Producers(Vec<(Var, String, Vec<Term>)>, Box<Plan>),
    Fail,
}
struct Clause {
    name: String,
    inputs: Vec<Term>,
    body: Plan,
}
pub struct Prepared {
    clauses: Rc<Vec<Clause>>,
}
#[derive(Clone)]
enum Node {
    Unknown(u64),
    App(String, Vec<Id>),
    Alias(Id),
    Call(String, Vec<Id>, Id),
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
    labels: usize,
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
fn plan(g: &Goal, output: Var, inputs: &BTreeSet<Var>) -> Result<Plan, String> {
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
            Box::new(plan(a, output, inputs)?),
            Box::new(plan(b, output, inputs)?),
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
                producers.push((*v, c.name.clone(), c.args[..c.args.len() - 1].to_vec()));
            }
            let mut edges = BTreeMap::new();
            for (out, _, args) in &producers {
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
                Box::new(plan(goals.last().unwrap(), output, &bound)?),
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
            out.extend(ps.iter().map(|(_, n, a)| (n.clone(), a.len())));
            calls(p, out)
        }
        _ => {}
    }
}
impl Prepared {
    pub fn new(rules: Vec<Rule>) -> Result<Self, String> {
        let mut clauses = vec![];
        for rule in rules {
            if !rule.kept.is_empty() || rule.removed.len() != 1 || !rule.guards.is_empty() {
                return Err("requires single removed head without guards".into());
            }
            let head = &rule.removed[0];
            let Some(Term::Var(output)) = head.args.last() else {
                return Err("head needs distinct variable output".into());
            };
            let inputs = head.args[..head.args.len() - 1].to_vec();
            let mut vs = vec![];
            for t in &inputs {
                variables(t, &mut vs);
            }
            let set: BTreeSet<_> = vs.iter().copied().collect();
            if set.len() != vs.len() || set.contains(output) {
                return Err("input patterns must be linear and exclude output".into());
            }
            clauses.push(Clause {
                name: head.name.clone(),
                inputs,
                body: plan(&rule.body, *output, &set)?,
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
        Ok(Self {
            clauses: Rc::new(clauses),
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
            labels: 0,
        };
        let mut env = Env::new();
        let mut writers = BTreeSet::new();
        let mut edges = BTreeMap::new();
        for c in &query.constraints {
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
        for c in query.constraints {
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
        let call = self.push(Node::Call(name, args, output));
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
                let label = self.labels;
                self.labels += 1;
                let mut left = ctx.clone();
                left.insert(label, false);
                let mut right = ctx.clone();
                right.insert(label, true);
                let a = self.expand(a, &mut env.clone(), &left, output);
                let b = self.expand(b, &mut env.clone(), &right, output);
                self.push(Node::Choice(label, a, b))
            }
            Plan::Producers(ps, p) => {
                for (v, n, args) in ps {
                    let args = args.iter().map(|t| self.term(t, env)).collect();
                    self.producer(n.clone(), args, *v, env, ctx);
                }
                self.expand(p, env, ctx, output)
            }
        }
    }
    fn matches(&mut self, p: &Term, id: Id, ctx: &Context, env: &mut Env) -> Result<bool, Signal> {
        match p {
            Term::Var(v) => {
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
            Node::Call(name, args, output) => {
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
                        let value = self.expand(&clause.body, &mut env, ctx, output);
                        self.nodes[id].results.push((ctx.clone(), value));
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
        for (support, id) in self.obligations.clone() {
            if support.iter().all(|(k, v)| ctx.get(k) == Some(v)) {
                self.force(id, ctx)?;
            }
        }
        let mut outputs = vec![];
        for (name, id) in self.outputs.clone() {
            outputs.push((name, self.normal(id, ctx)?));
        }
        let mut residual = vec![];
        for (support, id) in self.obligations.clone() {
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
            let Node::Call(name, args, output) = self.nodes[id].node.clone() else {
                unreachable!("obligations are source calls")
            };
            let mut args = args
                .into_iter()
                .map(|id| self.normal(id, ctx))
                .collect::<Result<Vec<_>, _>>()?;
            args.push(self.normal(output, ctx)?);
            residual.push(Constraint { name, args });
        }
        Ok(Answer { outputs, residual })
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
            let (support, id) = self.obligations[index].clone();
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
