//! Source-derived finite bag joins; no interpreter transitions or reference imports.
use chr_syntax::{Answer, Constraint, Goal, Query, Rule, Term, Var};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
type Signature = (String, usize);
fn signature(c: &Constraint) -> Signature {
    (c.name.clone(), c.args.len())
}
fn ground(t: &Term) -> bool {
    match t {
        Term::Var(_) => false,
        Term::App(_, xs) => xs.iter().all(ground),
    }
}
fn variables(t: &Term, out: &mut BTreeSet<Var>) {
    match t {
        Term::Var(v) => {
            out.insert(*v);
        }
        Term::App(_, xs) => {
            for t in xs {
                variables(t, out);
            }
        }
    }
}
struct Table {
    rows: Vec<Vec<Term>>,
    columns: Vec<BTreeMap<Term, Vec<usize>>>,
}
pub struct Prepared {
    entry: Signature,
    parameters: Vec<Var>,
    calls: Vec<Constraint>,
    tables: BTreeMap<Signature, Table>,
}
fn heads(rule: &Rule) -> Result<(&Constraint, Vec<Var>), String> {
    if !rule.kept.is_empty() || rule.removed.len() != 1 || !rule.guards.is_empty() {
        return Err("one consuming head and no guards required".into());
    }
    let h = &rule.removed[0];
    let mut vars = Vec::new();
    for t in &h.args {
        let Term::Var(v) = t else {
            return Err("head arguments must be distinct variables".into());
        };
        if vars.contains(v) {
            return Err("repeated head variable".into());
        }
        vars.push(*v);
    }
    Ok((h, vars))
}
fn calls(g: &Goal, out: &mut Vec<Constraint>) -> Result<(), String> {
    match g {
        Goal::True => Ok(()),
        Goal::Constraint(c) => {
            out.push(c.clone());
            Ok(())
        }
        Goal::And(gs) => {
            for g in gs {
                calls(g, out)?;
            }
            Ok(())
        }
        _ => Err("request body must contain only table calls".into()),
    }
}
fn rows(g: &Goal, vars: &[Var], out: &mut Vec<Vec<Term>>) -> Result<(), String> {
    match g {
        Goal::Or(a, b) => {
            rows(a, vars, out)?;
            rows(b, vars, out)
        }
        Goal::Fail => Ok(()),
        _ => {
            fn assignments(g: &Goal, values: &mut BTreeMap<Var, Term>) -> Result<(), String> {
                match g {
                    Goal::True => Ok(()),
                    Goal::And(gs) => {
                        for g in gs {
                            assignments(g, values)?;
                        }
                        Ok(())
                    }
                    Goal::Unify(Term::Var(v), t) | Goal::Unify(t, Term::Var(v)) if ground(t) => {
                        if values.insert(*v, t.clone()).is_some() {
                            return Err("row assigns a variable twice".into());
                        }
                        Ok(())
                    }
                    _ => Err("row requires ground assignments".into()),
                }
            }
            let mut values = BTreeMap::new();
            assignments(g, &mut values)?;
            if values.len() != vars.len() || vars.iter().any(|v| !values.contains_key(v)) {
                return Err("row must assign every head variable and no locals".into());
            }
            out.push(vars.iter().map(|v| values[v].clone()).collect());
            Ok(())
        }
    }
}
impl Prepared {
    pub fn new(rules: &[Rule], entry: (&str, usize)) -> Result<Arc<Self>, String> {
        let entry = (entry.0.to_string(), entry.1);
        let mut parameters = None;
        let mut body = Vec::new();
        let mut tables = BTreeMap::new();
        let mut defined = BTreeSet::new();
        for rule in rules {
            let (head, vars) = heads(rule)?;
            let key = signature(head);
            if !defined.insert(key.clone()) {
                return Err("competing predicate definitions".into());
            }
            if key == entry {
                parameters = Some(vars);
                calls(&rule.body, &mut body)?;
            } else {
                let mut data = Vec::new();
                rows(&rule.body, &vars, &mut data)?;
                let mut columns = vec![BTreeMap::<Term, Vec<usize>>::new(); vars.len()];
                for (i, row) in data.iter().enumerate() {
                    for (col, t) in columns.iter_mut().zip(row) {
                        col.entry(t.clone()).or_default().push(i);
                    }
                }
                tables.insert(
                    key,
                    Table {
                        rows: data,
                        columns,
                    },
                );
            }
        }
        let parameters = parameters.ok_or("missing request rule")?;
        if body.iter().any(|c| !tables.contains_key(&signature(c))) {
            return Err("request calls a non-table predicate".into());
        }
        Ok(Arc::new(Self {
            entry,
            parameters,
            calls: body,
            tables,
        }))
    }
    pub fn start(self: &Arc<Self>, query: &Query) -> Result<Engine, String> {
        let mut vars = BTreeSet::new();
        for c in &query.constraints {
            for t in &c.args {
                variables(t, &mut vars);
            }
        }
        for (_, v) in &query.outputs {
            vars.insert(*v);
        }
        let mut next = vars.last().map_or(Ok(0), |v| {
            v.0.checked_add(1).ok_or("variable identity exhausted")
        })?;
        fn instantiate(
            t: &Term,
            env: &mut BTreeMap<Var, Term>,
            next: &mut u64,
        ) -> Result<Term, String> {
            Ok(match t {
                Term::Var(v) => {
                    if !env.contains_key(v) {
                        let fresh = Var(*next);
                        *next = next.checked_add(1).ok_or("variable identity exhausted")?;
                        env.insert(*v, Term::Var(fresh));
                    }
                    env[v].clone()
                }
                Term::App(n, xs) => Term::App(
                    n.clone(),
                    xs.iter()
                        .map(|t| instantiate(t, env, next))
                        .collect::<Result<_, _>>()?,
                ),
            })
        }
        let mut pending = Vec::new();
        let mut residual = Vec::new();
        for c in &query.constraints {
            if signature(c) == self.entry {
                let mut env = self
                    .parameters
                    .iter()
                    .copied()
                    .zip(c.args.iter().cloned())
                    .collect();
                for call in &self.calls {
                    pending.push(Constraint {
                        name: call.name.clone(),
                        args: call
                            .args
                            .iter()
                            .map(|t| instantiate(t, &mut env, &mut next))
                            .collect::<Result<_, _>>()?,
                    });
                }
            } else if self.tables.contains_key(&signature(c)) {
                pending.push(c.clone());
            } else {
                residual.push(c.clone());
            }
        }
        let state = State {
            pending,
            bindings: BTreeMap::new(),
        };
        Ok(Engine {
            prepared: self.clone(),
            outputs: query.outputs.clone(),
            residual,
            stack: vec![Frame::State(state)],
        })
    }
}
fn resolve(t: &Term, env: &BTreeMap<Var, Term>) -> Term {
    match t {
        Term::Var(v) => env.get(v).cloned().unwrap_or_else(|| t.clone()),
        Term::App(n, xs) => Term::App(n.clone(), xs.iter().map(|t| resolve(t, env)).collect()),
    }
}
fn bind(pattern: &Term, ground: &Term, env: &mut BTreeMap<Var, Term>) -> bool {
    match pattern {
        Term::Var(v) => match env.get(v) {
            Some(old) => old == ground,
            None => {
                env.insert(*v, ground.clone());
                true
            }
        },
        Term::App(n, xs) => match ground {
            Term::App(m, ys) => {
                n == m && xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| bind(x, y, env))
            }
            _ => unreachable!(),
        },
    }
}
#[derive(Clone)]
struct State {
    pending: Vec<Constraint>,
    bindings: BTreeMap<Var, Term>,
}
enum Frame {
    State(State),
    Rows {
        state: State,
        call: Constraint,
        rows: Vec<usize>,
        next: usize,
    },
}
pub enum Step {
    Progress,
    Answer(Answer),
    Exhausted,
}
pub struct Engine {
    prepared: Arc<Prepared>,
    outputs: Vec<(String, Var)>,
    residual: Vec<Constraint>,
    stack: Vec<Frame>,
}
impl Table {
    fn candidates(&self, c: &Constraint, env: &BTreeMap<Var, Term>) -> Vec<usize> {
        let mut best: Option<&Vec<usize>> = None;
        for (t, col) in c.args.iter().zip(&self.columns) {
            let t = resolve(t, env);
            if ground(&t) {
                let Some(bucket) = col.get(&t) else {
                    return Vec::new();
                };
                if best.is_none_or(|old| bucket.len() < old.len()) {
                    best = Some(bucket);
                }
            }
        }
        best.cloned()
            .unwrap_or_else(|| (0..self.rows.len()).collect())
    }
}
impl Engine {
    /// One finite join-selection or row attempt; no CHR rule-step encoding.
    pub fn advance(&mut self) -> Step {
        let Some(frame) = self.stack.pop() else {
            return Step::Exhausted;
        };
        match frame {
            Frame::State(mut state) => {
                if state.pending.is_empty() {
                    return Step::Answer(Answer {
                        outputs: self
                            .outputs
                            .iter()
                            .map(|(n, v)| (n.clone(), resolve(&Term::Var(*v), &state.bindings)))
                            .collect(),
                        residual: self
                            .residual
                            .iter()
                            .map(|c| Constraint {
                                name: c.name.clone(),
                                args: c.args.iter().map(|t| resolve(t, &state.bindings)).collect(),
                            })
                            .collect(),
                    });
                }
                let (position, rows) = state
                    .pending
                    .iter()
                    .enumerate()
                    .map(|(i, c)| {
                        (
                            i,
                            self.prepared.tables[&signature(c)].candidates(c, &state.bindings),
                        )
                    })
                    .min_by_key(|(_, r)| r.len())
                    .unwrap();
                if !rows.is_empty() {
                    let call = state.pending.remove(position);
                    self.stack.push(Frame::Rows {
                        state,
                        call,
                        rows,
                        next: 0,
                    });
                }
            }
            Frame::Rows {
                state,
                call,
                rows,
                next,
            } => {
                let row = &self.prepared.tables[&signature(&call)].rows[rows[next]];
                let mut chosen = state.clone();
                let compatible = call
                    .args
                    .iter()
                    .zip(row)
                    .all(|(p, t)| bind(p, t, &mut chosen.bindings));
                if next + 1 < rows.len() {
                    self.stack.push(Frame::Rows {
                        state,
                        call,
                        rows,
                        next: next + 1,
                    });
                }
                if compatible {
                    self.stack.push(Frame::State(chosen));
                }
            }
        }
        Step::Progress
    }
}
