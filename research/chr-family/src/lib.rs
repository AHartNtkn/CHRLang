//! Frozen first-order family service. Named choices are private runtime nodes, not source terms.
use chr_syntax::{Answer, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;
#[derive(Clone, Debug)]
pub struct Value(Rc<Node>);
#[derive(Debug)]
enum Node {
    Var(u64),
    App(String, Vec<Value>),
    Choice(u32, Value, Value),
}
impl Value {
    pub fn var(v: u64) -> Self {
        Self(Rc::new(Node::Var(v)))
    }
    pub fn app(name: &str, args: impl IntoIterator<Item = Value>) -> Self {
        Self(Rc::new(Node::App(name.into(), args.into_iter().collect())))
    }
    pub fn choice(label: u32, left: Self, right: Self) -> Self {
        Self(Rc::new(Node::Choice(label, left, right)))
    }
}
#[derive(Clone)]
pub struct Request {
    pub labels: u32,
    pub equations: Vec<(Value, Value)>,
    pub outputs: Vec<u64>,
}
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Eager,
    Named,
    Partitioned,
}
#[derive(Default, Debug)]
pub struct Stats {
    pub steps: u64,
    pub pairs: u64,
    pub root_reads: u64,
    pub absence_visits: u64,
    pub occurs_visits: u64,
    pub splits: u64,
    pub copied_entries: u64,
    pub completed_regions: u64,
    pub failed_regions: u64,
    pub projection_visits: u64,
    pub eager_nodes: u64,
    pub bindings: u64,
    pub raw_answers: u64,
    pub observer_pairs: u64,
    pub observer_scans: u64,
    pub observer_backtracks: u64,
    pub projection_cache_hits: u64,
    pub interning_lookups: u64,
    pub partition_groups: u64,
    pub max_frontier: usize,
}
#[derive(Clone, Default)]
struct Context {
    support: Option<Vec<u64>>,
    decisions: BTreeMap<u32, bool>,
    bindings: BTreeMap<u64, Value>,
    work: Vec<(Value, Value)>,
}
enum Root {
    Value(Value),
    Need(u32),
}
fn root(v: &Value, c: &Context, s: &mut Stats) -> Root {
    let mut v = v.clone();
    loop {
        s.root_reads += 1;
        match v.0.as_ref() {
            Node::Var(id) => {
                if let Some(t) = c.bindings.get(id) {
                    v = t.clone();
                } else {
                    return Root::Value(v);
                }
            }
            Node::Choice(d, l, r) => {
                if let Some(right) = c.decisions.get(d) {
                    v = if *right { r.clone() } else { l.clone() };
                } else {
                    return Root::Need(*d);
                }
            }
            Node::App(..) => return Root::Value(v),
        }
    }
}
fn absent(id: u64, v: &Value, c: &Context, path: &mut BTreeSet<usize>, s: &mut Stats) -> bool {
    s.absence_visits += 1;
    let key = Rc::as_ptr(&v.0) as usize;
    if !path.insert(key) {
        return false;
    }
    let result = match v.0.as_ref() {
        Node::Var(other) => {
            *other != id
                && c.bindings
                    .get(other)
                    .is_none_or(|t| absent(id, t, c, path, s))
        }
        Node::App(_, args) => args.iter().all(|t| absent(id, t, c, path, s)),
        Node::Choice(d, l, r) => match c.decisions.get(d) {
            Some(false) => absent(id, l, c, path, s),
            Some(true) => absent(id, r, c, path, s),
            None => absent(id, l, c, path, s) && absent(id, r, c, path, s),
        },
    };
    path.remove(&key);
    result
}
enum Occurs {
    Absent,
    Present,
    Need(u32),
}
fn occurs(id: u64, v: &Value, c: &Context, s: &mut Stats) -> Occurs {
    s.occurs_visits += 1;
    match root(v, c, s) {
        Root::Need(d) => Occurs::Need(d),
        Root::Value(v) => match v.0.as_ref() {
            Node::Var(other) => {
                if id == *other {
                    Occurs::Present
                } else {
                    Occurs::Absent
                }
            }
            Node::App(_, args) => {
                for t in args {
                    match occurs(id, t, c, s) {
                        Occurs::Absent => {}
                        other => return other,
                    }
                }
                Occurs::Absent
            }
            Node::Choice(..) => unreachable!(),
        },
    }
}
enum Step {
    Continue,
    Fail,
    Split(u32),
}
fn bind(id: u64, t: &Value, c: &mut Context, s: &mut Stats) -> Step {
    if !absent(id, t, c, &mut BTreeSet::new(), s) {
        match occurs(id, t, c, s) {
            Occurs::Present => return Step::Fail,
            Occurs::Need(d) => return Step::Split(d),
            Occurs::Absent => {}
        }
    }
    c.bindings.insert(id, t.clone());
    s.bindings += 1;
    Step::Continue
}
fn equation(a: &Value, b: &Value, c: &mut Context, s: &mut Stats) -> Step {
    s.pairs += 1;
    if Rc::ptr_eq(&a.0, &b.0) {
        return Step::Continue;
    }
    let original_a = a;
    let original_b = b;
    let a = root(a, c, s);
    let b = root(b, c, s);
    match (&a, &b) {
        (Root::Value(a), Root::Value(b)) => match (a.0.as_ref(), b.0.as_ref()) {
            (Node::Var(a), Node::Var(b)) if a == b => Step::Continue,
            (Node::Var(a), Node::Var(b)) => {
                let (child, parent) = if a > b { (*a, *b) } else { (*b, *a) };
                c.bindings.insert(child, Value::var(parent));
                s.bindings += 1;
                Step::Continue
            }
            (Node::Var(id), _) => bind(*id, b, c, s),
            (_, Node::Var(id)) => bind(*id, a, c, s),
            (Node::App(a, x), Node::App(b, y)) => {
                if a != b || x.len() != y.len() {
                    Step::Fail
                } else {
                    c.work.extend(x.iter().cloned().zip(y.iter().cloned()));
                    Step::Continue
                }
            }
            _ => unreachable!(),
        },
        (Root::Value(v), Root::Need(_)) if matches!(v.0.as_ref(), Node::Var(_)) => {
            let Node::Var(id) = v.0.as_ref() else {
                unreachable!()
            };
            bind(*id, original_b, c, s)
        }
        (Root::Need(_), Root::Value(v)) if matches!(v.0.as_ref(), Node::Var(_)) => {
            let Node::Var(id) = v.0.as_ref() else {
                unreachable!()
            };
            bind(*id, original_a, c, s)
        }
        (Root::Need(d), _) | (_, Root::Need(d)) => Step::Split(*d),
    }
}
fn project_value(v: &Value, c: &Context, s: &mut Stats) -> Value {
    s.projection_visits += 1;
    match root(v, c, s) {
        Root::Value(v) => match v.0.as_ref() {
            Node::Var(id) => {
                s.eager_nodes += 1;
                Value::var(*id)
            }
            Node::App(n, args) => {
                s.eager_nodes += 1;
                Value::app(n, args.iter().map(|a| project_value(a, c, s)))
            }
            _ => unreachable!(),
        },
        Root::Need(_) => unreachable!("full projection requires all active decisions"),
    }
}
fn source(v: &Value, c: &Context, s: &mut Stats) -> Term {
    s.projection_visits += 1;
    match root(v, c, s) {
        Root::Value(v) => match v.0.as_ref() {
            Node::Var(id) => Term::Var(Var(*id)),
            Node::App(n, args) => {
                Term::App(n.clone(), args.iter().map(|a| source(a, c, s)).collect())
            }
            _ => unreachable!(),
        },
        Root::Need(_) => unreachable!(),
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum InternKey {
    Var(u64),
    App(String, Vec<usize>),
}
#[derive(Default)]
struct Projector {
    memo: BTreeMap<(usize, u64), Value>,
    masks: BTreeMap<usize, u64>,
    intern: BTreeMap<InternKey, Value>,
    ids: BTreeMap<usize, usize>,
}
impl Projector {
    fn identify(&mut self, v: &Value) -> usize {
        let next = self.ids.len();
        *self.ids.entry(Rc::as_ptr(&v.0) as usize).or_insert(next)
    }

    fn mask(&mut self, v: &Value) -> u64 {
        let id = Rc::as_ptr(&v.0) as usize;
        if let Some(mask) = self.masks.get(&id) {
            return *mask;
        }
        let mask = match v.0.as_ref() {
            Node::Var(_) => 0,
            Node::App(_, args) => args.iter().fold(0, |m, a| m | self.mask(a)),
            Node::Choice(d, a, b) => (1 << d) | self.mask(a) | self.mask(b),
        };
        self.masks.insert(id, mask);
        mask
    }
    fn project(&mut self, v: &Value, bits: u64, s: &mut Stats) -> Value {
        s.projection_visits += 1;
        let key = (Rc::as_ptr(&v.0) as usize, bits & self.mask(v));
        if let Some(value) = self.memo.get(&key) {
            s.projection_cache_hits += 1;
            return value.clone();
        }
        let value = match v.0.as_ref() {
            Node::Choice(d, a, b) => {
                self.project(if bits & (1 << d) == 0 { a } else { b }, bits, s)
            }
            Node::Var(id) => {
                s.interning_lookups += 1;
                self.intern
                    .entry(InternKey::Var(*id))
                    .or_insert_with(|| {
                        s.eager_nodes += 1;
                        Value::var(*id)
                    })
                    .clone()
            }
            Node::App(n, args) => {
                let args = args
                    .iter()
                    .map(|a| self.project(a, bits, s))
                    .collect::<Vec<_>>();
                let ik = InternKey::App(n.clone(), args.iter().map(|a| self.identify(a)).collect());
                s.interning_lookups += 1;
                self.intern
                    .entry(ik)
                    .or_insert_with(|| {
                        s.eager_nodes += 1;
                        Value::app(n, args)
                    })
                    .clone()
            }
        };
        self.identify(&value);
        self.memo.insert(key, value.clone());
        value
    }
}
pub struct Solver {
    request: Request,
    frontier: VecDeque<Context>,
    solutions: Vec<Context>,
    stats: Stats,
}
impl Solver {
    pub fn new(request: Request, mode: Mode) -> Result<Self, String> {
        if request.labels > 16 {
            return Err("frozen probe supports at most 16 active labels".into());
        }
        let mut todo = request
            .equations
            .iter()
            .flat_map(|(a, b)| [a, b])
            .collect::<Vec<_>>();
        let mut seen = BTreeSet::new();
        while let Some(v) = todo.pop() {
            if !seen.insert(Rc::as_ptr(&v.0) as usize) {
                continue;
            }
            match v.0.as_ref() {
                Node::Choice(d, a, b) => {
                    if *d >= request.labels {
                        return Err("choice label outside frozen domain".into());
                    }
                    todo.extend([a, b]);
                }
                Node::App(_, args) => todo.extend(args),
                _ => {}
            }
        }
        let mut stats = Stats::default();
        let mut frontier = VecDeque::new();
        match mode {
            Mode::Partitioned => {
                let mut projector = Projector::default();
                let mut groups: BTreeMap<Vec<(usize, usize)>, Context> = BTreeMap::new();
                for bits in 0..1u64 << request.labels {
                    let work = request
                        .equations
                        .iter()
                        .rev()
                        .map(|(a, b)| {
                            (
                                projector.project(a, bits, &mut stats),
                                projector.project(b, bits, &mut stats),
                            )
                        })
                        .collect::<Vec<_>>();
                    let key = work
                        .iter()
                        .map(|(a, b)| (projector.identify(a), projector.identify(b)))
                        .collect();
                    let c = groups.entry(key).or_insert_with(|| Context {
                        work,
                        support: Some(vec![]),
                        ..Default::default()
                    });
                    c.support.as_mut().unwrap().push(bits);
                }
                stats.partition_groups = groups.len() as u64;
                frontier.extend(groups.into_values());
            }
            Mode::Named => frontier.push_back(Context {
                work: request.equations.iter().rev().cloned().collect(),
                ..Default::default()
            }),
            Mode::Eager => {
                for bits in 0..1u64 << request.labels {
                    let mut c = Context {
                        decisions: (0..request.labels)
                            .map(|d| (d, bits & (1 << d) != 0))
                            .collect(),
                        ..Default::default()
                    };
                    c.work = request
                        .equations
                        .iter()
                        .rev()
                        .map(|(a, b)| {
                            (
                                project_value(a, &c, &mut stats),
                                project_value(b, &c, &mut stats),
                            )
                        })
                        .collect();
                    frontier.push_back(c);
                }
            }
        }
        stats.max_frontier = frontier.len();
        Ok(Self {
            request,
            frontier,
            solutions: vec![],
            stats,
        })
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn exhausted(&self) -> bool {
        self.frontier.is_empty()
    }
    pub fn retained_regions(&self) -> usize {
        self.solutions.len()
    }
    pub fn advance(&mut self, budget: usize) {
        for _ in 0..budget {
            let Some(mut c) = self.frontier.pop_front() else {
                break;
            };
            self.stats.steps += 1;
            if let Some((a, b)) = c.work.pop() {
                let result = equation(&a, &b, &mut c, &mut self.stats);
                match result {
                    Step::Continue => self.frontier.push_back(c),
                    Step::Fail => self.stats.failed_regions += 1,
                    Step::Split(d) => {
                        assert!(!c.decisions.contains_key(&d));
                        c.work.push((a, b));
                        self.stats.splits += 1;
                        self.stats.copied_entries +=
                            (c.work.len() + c.bindings.len() + c.decisions.len()) as u64;
                        let mut right = c.clone();
                        c.decisions.insert(d, false);
                        right.decisions.insert(d, true);
                        self.frontier.push_back(c);
                        self.frontier.push_back(right);
                    }
                }
            } else {
                self.stats.completed_regions += 1;
                self.solutions.push(c);
            }
            self.stats.max_frontier = self.stats.max_frontier.max(self.frontier.len());
        }
    }
    /// Experimental assignment-level observation, available only after request completion.
    pub fn projected_answers(&mut self) -> Result<Vec<(u64, Answer)>, String> {
        if !self.exhausted() {
            return Err("request remains unfinished".into());
        }
        self.stats.raw_answers = 0;
        let mut answers = vec![];
        let mut assigned = BTreeSet::new();
        for region in &self.solutions {
            for bits in 0..1u64 << self.request.labels {
                if let Some(support) = &region.support {
                    if !support.contains(&bits) {
                        continue;
                    }
                } else if region
                    .decisions
                    .iter()
                    .any(|(d, b)| *b != (bits & (1 << d) != 0))
                {
                    continue;
                }
                if !assigned.insert(bits) {
                    return Err("overlapping solution regions".into());
                }
                let mut c = region.clone();
                c.decisions = (0..self.request.labels)
                    .map(|d| (d, bits & (1 << d) != 0))
                    .collect();
                let answer = Answer {
                    outputs: self
                        .request
                        .outputs
                        .iter()
                        .enumerate()
                        .map(|(i, id)| {
                            (
                                format!("out{i}"),
                                source(&Value::var(*id), &c, &mut self.stats),
                            )
                        })
                        .collect(),
                    residual: vec![],
                };
                self.stats.raw_answers += 1;
                answers.push((bits, answer));
            }
        }
        Ok(answers)
    }
    /// Complete finite projection followed by exact alpha-deduplication.
    pub fn observe(&mut self) -> Result<Vec<Answer>, String> {
        let mut seen = chr_observe::AnswerSet::default();
        for (_, answer) in self.projected_answers()? {
            seen.insert(answer);
        }
        self.stats.observer_pairs = seen.stats.term_pairs;
        self.stats.observer_scans = seen.stats.occurrence_scans;
        self.stats.observer_backtracks = seen.stats.backtracks;
        Ok(seen.into_answers())
    }
}
