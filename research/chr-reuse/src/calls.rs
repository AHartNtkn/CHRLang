//! Experimental isolated-call table. The owner chooses a source-valid call
//! boundary; this table does not project arbitrary continuation state.
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
fn rename(term: &Term, map: &mut impl FnMut(Var) -> Var) -> Term {
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
    fn take(&mut self) -> Var {
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
    family: BTreeSet<(String, usize)>,
    memo: bool,
    cache: BTreeMap<Constraint, Vec<Vec<Term>>>,
    stats: Stats,
}
impl Table {
    pub fn new(rules: Vec<Rule>, memo: bool) -> Result<Self, String> {
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
        Ok(Self {
            prepared: PreparedMachine::new(rules)?,
            family,
            memo,
            cache: BTreeMap::new(),
            stats: Stats::default(),
        })
    }
    /// Expand at an already selected source call, or a separately justified
    /// commutation boundary. Arguments must be resolved in the caller's current
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
