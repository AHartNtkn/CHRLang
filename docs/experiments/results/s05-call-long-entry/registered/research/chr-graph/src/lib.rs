//! Closed relation-region experiment. Reference execution is not imported.
use chr_syntax::{Answer, Constraint, Goal, Guard, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;
mod terms;
use terms::{Bindings, Scope};
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    General,
    Local,
    Cached,
}
#[derive(Default, Debug)]
pub struct Stats {
    pub steps: u64,
    pub applications: u64,
    pub expansions: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub candidates: u64,
    pub rules_examined: u64,
    pub occurrence_scans: u64,
    pub binding_reads: u64,
    pub term_visits: u64,
    pub unification_pairs: u64,
    pub occurs_visits: u64,
    pub fresh_variables: u64,
    pub work_nodes: u64,
    pub snapshot_entries: u64,
    pub introductions: u64,
    pub splits: u64,
    pub failed: u64,
    pub completed: u64,
    pub duplicates: u64,
    pub max_frontier: usize,
}
fn disjoint(a: &Term, b: &Term) -> bool {
    match (a, b) {
        (Term::App(a, x), Term::App(b, y)) => {
            a != b || x.len() != y.len() || x.iter().zip(y).any(|(x, y)| disjoint(x, y))
        }
        _ => false,
    }
}
/// Conservative whole-program certificate. Linking another rule requires recertification.
pub fn eligible(rules: &[Rule]) -> Result<(), String> {
    let mut names = BTreeSet::new();
    for (i, r) in rules.iter().enumerate() {
        if !names.insert(&r.name) {
            return Err(format!("duplicate rule name {}", r.name));
        }
        if !r.kept.is_empty() || r.removed.len() != 1 {
            return Err(format!("{} is not single-head simplification", r.name));
        }
        let h = &r.removed[0];
        for other in &rules[..i] {
            let g = &other.removed[0];
            if h.name == g.name
                && h.args.len() == g.args.len()
                && !h.args.iter().zip(&g.args).any(|(a, b)| disjoint(a, b))
            {
                return Err(format!("potential overlap: {} and {}", other.name, r.name));
            }
        }
    }
    Ok(())
}
#[derive(Clone)]
struct Call {
    id: u64,
    constraint: Constraint,
}
enum Work {
    Call(Rc<Call>),
    Equal(Term, Term),
    And(Vec<Rc<Work>>),
    Or(Rc<Work>, Rc<Work>),
    True,
    Fail,
}
type Pred = (String, usize);
#[derive(Clone, Default)]
struct Context {
    pending: VecDeque<Rc<Work>>,
    roots: BTreeMap<Pred, BTreeMap<u64, Rc<Call>>>,
    bindings: Bindings,
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    occurrence: u64,
    rule: usize,
    witness: Scope,
}
struct Application {
    key: Key,
    predicate: Pred,
}
pub struct Batch {
    pub answers: Vec<Answer>,
    pub exhausted: bool,
}
pub struct Search {
    rules: Vec<Rule>,
    mode: Mode,
    frontier: VecDeque<Context>,
    cache: BTreeMap<Key, Rc<Work>>,
    outputs: Vec<(String, Term)>,
    next_var: u64,
    next_call: u64,
    stats: Stats,
    seen: chr_observe::AnswerSet,
}
fn instantiate(term: &Term, scope: &mut Scope, next: &mut u64, stats: &mut Stats) -> Term {
    stats.term_visits += 1;
    match term {
        Term::Var(v) => scope
            .entry(*v)
            .or_insert_with(|| {
                let v = Var(*next);
                *next += 1;
                stats.fresh_variables += 1;
                Term::Var(v)
            })
            .clone(),
        Term::App(n, args) => Term::App(
            n.clone(),
            args.iter()
                .map(|a| instantiate(a, scope, next, stats))
                .collect(),
        ),
    }
}
fn compile(
    goal: &Goal,
    scope: &mut Scope,
    next: &mut u64,
    call: &mut u64,
    stats: &mut Stats,
) -> Rc<Work> {
    stats.work_nodes += 1;
    Rc::new(match goal {
        Goal::Constraint(c) => {
            let id = *call;
            *call += 1;
            Work::Call(Rc::new(Call {
                id,
                constraint: Constraint {
                    name: c.name.clone(),
                    args: c
                        .args
                        .iter()
                        .map(|a| instantiate(a, scope, next, stats))
                        .collect(),
                },
            }))
        }
        Goal::Unify(a, b) => Work::Equal(
            instantiate(a, scope, next, stats),
            instantiate(b, scope, next, stats),
        ),
        Goal::And(gs) => Work::And(
            gs.iter()
                .map(|g| compile(g, scope, next, call, stats))
                .collect(),
        ),
        Goal::Or(a, b) => Work::Or(
            compile(a, scope, next, call, stats),
            compile(b, scope, next, call, stats),
        ),
        Goal::True => Work::True,
        Goal::Fail => Work::Fail,
    })
}
fn matches(
    pattern: &Term,
    value: &Term,
    scope: &mut Scope,
    bindings: &Bindings,
    stats: &mut Stats,
) -> bool {
    stats.term_visits += 1;
    match pattern {
        Term::Var(v) => match scope.get(v) {
            Some(old) => terms::equal(old, value, bindings, stats),
            None => {
                scope.insert(*v, value.clone());
                true
            }
        },
        Term::App(n, args) => match terms::deref(value, bindings, stats) {
            Term::App(m, values) => {
                n == m
                    && args.len() == values.len()
                    && args
                        .iter()
                        .zip(values)
                        .all(|(p, v)| matches(p, v, scope, bindings, stats))
            }
            Term::Var(_) => false,
        },
    }
}
impl Search {
    pub fn new(rules: Vec<Rule>, query: Query, mode: Mode) -> Result<Self, String> {
        eligible(&rules)?;
        let mut names = BTreeSet::new();
        for (n, _) in &query.outputs {
            if !names.insert(n) {
                return Err("duplicate output name".into());
            }
        }
        let mut stats = Stats {
            max_frontier: 1,
            ..Default::default()
        };
        let mut scope = Scope::new();
        let mut next_var = 0;
        let mut next_call = 0;
        let pending = query
            .constraints
            .iter()
            .map(|c| {
                compile(
                    &Goal::Constraint(c.clone()),
                    &mut scope,
                    &mut next_var,
                    &mut next_call,
                    &mut stats,
                )
            })
            .collect();
        let outputs = query
            .outputs
            .into_iter()
            .map(|(n, v)| {
                (
                    n,
                    instantiate(&Term::Var(v), &mut scope, &mut next_var, &mut stats),
                )
            })
            .collect();
        Ok(Self {
            rules,
            mode,
            frontier: VecDeque::from([Context {
                pending,
                ..Default::default()
            }]),
            cache: BTreeMap::new(),
            outputs,
            next_var,
            next_call,
            stats,
            seen: Default::default(),
        })
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn observation_stats(&self) -> &chr_observe::Stats {
        &self.seen.stats
    }
    pub fn pending_alternatives(&self) -> usize {
        self.frontier.len()
    }
    pub fn retained_cache(&self) -> usize {
        self.cache.len()
    }
    pub fn retained_context_entries(&self) -> (usize, usize, usize) {
        self.frontier.iter().fold((0, 0, 0), |(r, b, p), c| {
            (
                r + c.roots.values().map(|g| g.len()).sum::<usize>(),
                b + c.bindings.len(),
                p + c.pending.len(),
            )
        })
    }
    fn select(&mut self, context: &Context) -> Option<Application> {
        for (idx, r) in self.rules.iter().enumerate() {
            self.stats.rules_examined += 1;
            let h = &r.removed[0];
            let pred = (h.name.clone(), h.args.len());
            // Same root representation in every mode. General deliberately scans all predicates.
            let groups: Vec<_> = match self.mode {
                Mode::General => context.roots.values().collect(),
                _ => context.roots.get(&pred).into_iter().collect(),
            };
            for roots in groups {
                for node in roots.values() {
                    self.stats.occurrence_scans += 1;
                    if node.constraint.name != h.name || node.constraint.args.len() != h.args.len()
                    {
                        continue;
                    }
                    self.stats.candidates += 1;
                    let mut scope = Scope::new();
                    if !h
                        .args
                        .iter()
                        .zip(&node.constraint.args)
                        .all(|(p, v)| matches(p, v, &mut scope, &context.bindings, &mut self.stats))
                    {
                        continue;
                    }
                    let mut next = self.next_var;
                    let mut guard_scope = scope.clone();
                    let guard_ok = r.guards.iter().all(|g| match g {
                        Guard::Equal(a, b) => {
                            let a = instantiate(a, &mut guard_scope, &mut next, &mut self.stats);
                            let b = instantiate(b, &mut guard_scope, &mut next, &mut self.stats);
                            terms::equal(&a, &b, &context.bindings, &mut self.stats)
                        }
                    });
                    if guard_ok {
                        return Some(Application {
                            key: Key {
                                occurrence: node.id,
                                rule: idx,
                                witness: scope,
                            },
                            predicate: pred,
                        });
                    }
                }
            }
        }
        None
    }
    fn answer(&mut self, context: &Context) -> Answer {
        Answer {
            outputs: self
                .outputs
                .iter()
                .map(|(n, t)| {
                    (
                        n.clone(),
                        terms::resolve(t, &context.bindings, &mut self.stats),
                    )
                })
                .collect(),
            residual: context
                .roots
                .values()
                .flat_map(|g| g.values())
                .map(|c| Constraint {
                    name: c.constraint.name.clone(),
                    args: c
                        .constraint
                        .args
                        .iter()
                        .map(|a| terms::resolve(a, &context.bindings, &mut self.stats))
                        .collect(),
                })
                .collect(),
        }
    }
    pub fn advance(&mut self, budget: usize) -> Batch {
        let mut answers = vec![];
        for _ in 0..budget {
            let Some(mut context) = self.frontier.pop_front() else {
                break;
            };
            self.stats.steps += 1;
            if let Some(work) = context.pending.pop_front() {
                match work.as_ref() {
                    Work::Call(c) => {
                        self.stats.introductions += 1;
                        context
                            .roots
                            .entry((c.constraint.name.clone(), c.constraint.args.len()))
                            .or_default()
                            .insert(c.id, c.clone());
                    }
                    Work::Equal(a, b) => {
                        if !terms::unify(a, b, &mut context.bindings, &mut self.stats) {
                            self.stats.failed += 1;
                            continue;
                        }
                    }
                    Work::And(gs) => {
                        for g in gs.iter().rev() {
                            context.pending.push_front(g.clone());
                        }
                    }
                    Work::Or(a, b) => {
                        self.stats.splits += 1;
                        self.stats.snapshot_entries += (context.pending.len()
                            + context.bindings.len()
                            + context.roots.values().map(|g| g.len()).sum::<usize>())
                            as u64;
                        let mut right = context.clone();
                        context.pending.push_front(a.clone());
                        right.pending.push_front(b.clone());
                        self.frontier.push_back(context);
                        self.frontier.push_back(right);
                        self.stats.max_frontier = self.stats.max_frontier.max(self.frontier.len());
                        continue;
                    }
                    Work::Fail => {
                        self.stats.failed += 1;
                        continue;
                    }
                    Work::True => {}
                }
            } else if let Some(application) = self.select(&context) {
                self.stats.applications += 1;
                let body = if matches!(self.mode, Mode::Cached) {
                    if let Some(body) = self.cache.get(&application.key) {
                        self.stats.cache_hits += 1;
                        body.clone()
                    } else {
                        self.stats.cache_misses += 1;
                        self.stats.expansions += 1;
                        let mut scope = application.key.witness.clone();
                        let body = compile(
                            &self.rules[application.key.rule].body,
                            &mut scope,
                            &mut self.next_var,
                            &mut self.next_call,
                            &mut self.stats,
                        );
                        self.cache.insert(application.key.clone(), body.clone());
                        body
                    }
                } else {
                    self.stats.expansions += 1;
                    let mut scope = application.key.witness.clone();
                    compile(
                        &self.rules[application.key.rule].body,
                        &mut scope,
                        &mut self.next_var,
                        &mut self.next_call,
                        &mut self.stats,
                    )
                };
                let group = context.roots.get_mut(&application.predicate).unwrap();
                group.remove(&application.key.occurrence);
                if group.is_empty() {
                    context.roots.remove(&application.predicate);
                }
                context.pending.push_back(body);
            } else {
                self.stats.completed += 1;
                let answer = self.answer(&context);
                if self.seen.insert(answer.clone()) {
                    answers.push(answer);
                } else {
                    self.stats.duplicates += 1;
                }
                continue;
            }
            self.frontier.push_back(context);
            self.stats.max_frontier = self.stats.max_frontier.max(self.frontier.len());
        }
        Batch {
            answers,
            exhausted: self.frontier.is_empty(),
        }
    }
}
