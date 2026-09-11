//! Experimental isolated-call table. The owner chooses a source-valid call
//! boundary; this table does not project arbitrary continuation state.
pub mod trace;
pub mod body;
use chr_persistent::continuations::{PreparedMachine, Step};
use chr_syntax::{Constraint, Goal, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
const METRICS: bool = cfg!(feature = "metrics");
type Bindings = Vec<(Var, Term)>;
fn variables(term: &Term, found: &mut BTreeSet<Var>) {
    match term {
        Term::Var(v) => {
            found.insert(*v);
        }
        Term::App(_, xs) => {
            for x in xs {
                variables(x, found);
            }
        }
    }
}
pub(crate) fn rename(term: &Term, map: &mut impl FnMut(Var) -> Var) -> Term {
    match term {
        Term::Var(v) => Term::Var(map(*v)),
        Term::App(name, xs) => Term::App(name.clone(), xs.iter().map(|t| rename(t, map)).collect()),
    }
}
#[derive(Clone)]
pub struct Fresh {
    next: u64,
}
impl Fresh {
    pub fn for_query(query: &Query) -> Self {
        let mut vars = BTreeSet::new();
        for c in &query.constraints {
            for t in &c.args {
                variables(t, &mut vars);
            }
        }
        vars.extend(query.outputs.iter().map(|(_, v)| *v));
        Self {
            next: vars
                .last()
                .map_or(0, |v| v.0.checked_add(1).expect("variable id exhausted")),
        }
    }
    pub(crate) fn from_next(next: u64) -> Self {
        Self { next }
    }
    pub(crate) fn take(&mut self) -> Var {
        let v = Var(self.next);
        self.next = self.next.checked_add(1).expect("variable id exhausted");
        v
    }
}
#[derive(Default, Debug, Clone)]
pub struct Stats {
    pub computed: u64,
    pub hits: u64,
    pub executed: u64,
}
pub struct Table {
    prepared: PreparedMachine,
    rules: Vec<Rule>,
    family: BTreeSet<(String, usize)>,
    memo: bool,
    cache: BTreeMap<Constraint, Vec<Vec<Term>>>,
    stats: Stats,
}
pub(crate) fn canonical(call: &Constraint) -> (Constraint, Vec<Var>) {
    let mut vars = BTreeMap::new();
    let mut originals = vec![];
    let key = Constraint {
        name: call.name.clone(),
        args: call
            .args
            .iter()
            .map(|t| {
                rename(t, &mut |v| {
                    *vars.entry(v).or_insert_with(|| {
                        let slot = Var(originals.len() as u64);
                        originals.push(v);
                        slot
                    })
                })
            })
            .collect(),
    };
    (key, originals)
}
pub(crate) fn checked_family(rules: &[Rule]) -> Result<BTreeSet<(String, usize)>, String> {
    if rules.is_empty()
        || rules
            .iter()
            .any(|r| !r.kept.is_empty() || r.removed.len() != 1)
    {
        return Err("isolated call rules must simplify one private head".into());
    }
    let family = rules
        .iter()
        .map(|r| (r.removed[0].name.clone(), r.removed[0].args.len()))
        .collect::<BTreeSet<_>>();
    fn private(goal: &Goal, family: &BTreeSet<(String, usize)>) -> bool {
        match goal {
            Goal::Constraint(c) => family.contains(&(c.name.clone(), c.args.len())),
            Goal::And(xs) => xs.iter().all(|g| private(g, family)),
            Goal::Or(a, b) => private(a, family) && private(b, family),
            Goal::Unify(_, _) | Goal::True | Goal::Fail => true,
        }
    }
    if rules.iter().any(|r| !private(&r.body, &family)) {
        return Err("call body reaches outside its private family".into());
    }
    Ok(family)
}
impl Table {
    pub fn new(rules: Vec<Rule>, memo: bool) -> Result<Self, String> {
        let family = checked_family(&rules)?;
        Ok(Self {
            prepared: PreparedMachine::new(rules.clone())?,
            rules,
            family,
            memo,
            cache: BTreeMap::new(),
            stats: Stats::default(),
        })
    }
    /// Checked contraction at an initial query boundary. This does not inspect
    /// an arbitrary live cursor. A complete private rule-priority phase permits
    /// shared caller variables; every interface binding must be replayed.
    pub fn expand_query(
        &mut self,
        query: &Query,
        selected: usize,
        global_rules: &[Rule],
        bound: usize,
    ) -> Result<Vec<Bindings>, String> {
        let call = query
            .constraints
            .get(selected)
            .ok_or("missing selected call")?;
        if !self.family.contains(&(call.name.clone(), call.args.len())) {
            return Err("selected call is outside prepared family".into());
        }
        let touching = global_rules
            .iter()
            .filter(|r| {
                r.kept
                    .iter()
                    .chain(&r.removed)
                    .any(|c| self.family.contains(&(c.name.clone(), c.args.len())))
            })
            .collect::<Vec<_>>();
        if touching.len() != self.rules.len()
            || touching.iter().zip(&self.rules).any(|(a, b)| *a != b)
        {
            return Err(
                "private family ownership or rule order differs from caller program".into(),
            );
        }
        if global_rules.get(..self.rules.len()) != Some(self.rules.as_slice()) {
            return Err("private phase must be a rule-priority prefix".into());
        }
        if query
            .constraints
            .iter()
            .filter(|c| self.family.contains(&(c.name.clone(), c.args.len())))
            .count()
            != 1
        {
            return Err("private phase requires exactly one initial family occurrence".into());
        }
        let mut fresh = Fresh::for_query(query);
        self.expand(call, &mut fresh, bound)
    }
    /// Expand within an independently justified complete call-execution phase
    /// or commutation boundary. Selection of its first rule alone is insufficient.
    /// Arguments must be resolved in the caller's current
    /// environment. `fresh` must cover every live caller variable and remain
    /// coordinated with its allocator. The family check proves none of these
    /// caller-side preconditions; see the scheduling counterexample in the gate.
    pub fn expand(
        &mut self,
        call: &Constraint,
        fresh: &mut Fresh,
        bound: usize,
    ) -> Result<Vec<Bindings>, String> {
        if !self.family.contains(&(call.name.clone(), call.args.len())) {
            return Err("call is outside prepared family".into());
        }
        let (key, originals) = canonical(call);
        let cached = if self.memo {
            self.cache.get(&key).cloned()
        } else {
            None
        };
        let results = if let Some(results) = cached {
            if METRICS {
                self.stats.hits += 1;
            }
            results
        } else {
            if METRICS {
                self.stats.computed += 1;
            }
            let query = Query {
                constraints: vec![key.clone()],
                outputs: (0..originals.len())
                    .map(|i| (format!("arg{i}"), Var(i as u64)))
                    .collect(),
            };
            let (mut machine, cursor) = self.prepared.start(query)?;
            let mut pending = VecDeque::from([cursor]);
            let mut answers = vec![];
            for _ in 0..bound {
                let Some(cursor) = pending.pop_front() else {
                    break;
                };
                if METRICS {
                    self.stats.executed += 1;
                }
                match machine.step(cursor) {
                    Step::Continue(c) => pending.push_back(c),
                    Step::Split(a, b) => {
                        pending.push_back(a);
                        pending.push_back(b);
                    }
                    Step::Failed => (),
                    Step::Answer(answer) => {
                        if !answer.residual.is_empty() {
                            return Err("isolated call suspended with residual work".into());
                        }
                        answers.push(
                            answer
                                .outputs
                                .into_iter()
                                .map(|(_, t)| t)
                                .collect::<Vec<_>>(),
                        );
                    }
                }
            }
            if !pending.is_empty() {
                return Err("isolated call reached its service bound".into());
            }
            if self.memo {
                self.cache.insert(key, answers.clone());
            }
            answers
        };
        Ok(results
            .into_iter()
            .map(|result| {
                // Interface variables keep caller identities. Each alternative receives its
                // own fresh internal names, allocated beyond the whole caller's live syntax.
                let mut mapping = originals
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| (Var(i as u64), v))
                    .collect::<BTreeMap<_, _>>();
                originals
                    .iter()
                    .copied()
                    .zip(result.into_iter().map(|t| {
                        rename(&t, &mut |v| {
                            *mapping.entry(v).or_insert_with(|| fresh.take())
                        })
                    }))
                    .collect()
            })
            .collect())
    }
    pub fn stats(&self) -> Stats {
        self.stats.clone()
    }
}

/// Prepared complete-caller path for the checked initial private phase.
pub struct Caller {
    table: Table,
    program: PreparedMachine,
    rules: Vec<Rule>,
    owner: std::rc::Rc<()>,
}
pub struct CallerRun {
    owner: std::rc::Rc<()>,
    query: Option<Query>,
    machines: VecDeque<(
        chr_persistent::continuations::Machine,
        VecDeque<chr_persistent::continuations::Cursor>,
    )>,
    call_bound: usize,
    remaining: usize,
    error: Option<String>,
}
#[derive(Debug)]
pub enum CallerEvent {
    Progress,
    Answer(chr_syntax::Answer),
    Done,
}
impl Caller {
    pub fn new(rules: Vec<Rule>, private_rules: usize, memo: bool) -> Result<Self, String> {
        let family = rules
            .get(..private_rules)
            .ok_or("invalid private rule count")?
            .to_vec();
        Ok(Self {
            table: Table::new(family, memo)?,
            program: PreparedMachine::new(rules.clone())?,
            rules,
            owner: std::rc::Rc::new(()),
        })
    }
    pub fn start(&self, query: Query, call_bound: usize, caller_bound: usize) -> CallerRun {
        CallerRun {
            owner: self.owner.clone(),
            query: Some(query),
            machines: VecDeque::new(),
            call_bound,
            remaining: caller_bound,
            error: None,
        }
    }
    pub fn stats(&self) -> Stats {
        self.table.stats()
    }
    pub fn step(&mut self, run: &mut CallerRun) -> Result<CallerEvent, String> {
        assert!(
            std::rc::Rc::ptr_eq(&self.owner, &run.owner),
            "caller run belongs to a different prepared program"
        );
        if let Some(error) = &run.error {
            return Err(error.clone());
        }
        let result = self.step_inner(run);
        if let Err(error) = &result {
            run.error = Some(error.clone());
        }
        result
    }
    fn step_inner(&mut self, run: &mut CallerRun) -> Result<CallerEvent, String> {
        if let Some(mut query) = run.query.take() {
            let selected = query
                .constraints
                .iter()
                .position(|c| self.table.family.contains(&(c.name.clone(), c.args.len())))
                .ok_or("query has no private call")?;
            let answers = self
                .table
                .expand_query(&query, selected, &self.rules, run.call_bound)?;
            query.constraints.remove(selected);
            for bindings in answers {
                let (machine, cursor) = self.program.start_replaying(query.clone(), bindings)?;
                run.machines.push_back((machine, VecDeque::from([cursor])));
            }
            return Ok(CallerEvent::Progress);
        }
        while run
            .machines
            .front()
            .is_some_and(|(_, cursors)| cursors.is_empty())
        {
            run.machines.pop_front();
        }
        let Some((machine, cursors)) = run.machines.front_mut() else {
            return Ok(CallerEvent::Done);
        };
        if run.remaining == 0 {
            return Err("resumed caller reached its service bound".into());
        }
        run.remaining -= 1;
        let cursor = cursors.pop_front().unwrap();
        match machine.step(cursor) {
            Step::Continue(c) => cursors.push_back(c),
            Step::Split(a, b) => {
                cursors.push_back(a);
                cursors.push_back(b);
            }
            Step::Failed => (),
            Step::Answer(answer) => return Ok(CallerEvent::Answer(answer)),
        }
        Ok(CallerEvent::Progress)
    }
}
