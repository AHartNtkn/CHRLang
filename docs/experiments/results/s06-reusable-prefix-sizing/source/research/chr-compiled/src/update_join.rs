//! S01 exact-source lowering. No claim of a general join compiler.
use chr_syntax::{Answer, Constraint, Query, Rule, Term, Var, and, atom, c, eq, t, v};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Direct,
    Retained,
}
#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub constructed_pairs: u64,
    pub invalidated_pairs: u64,
    pub direct_pairs: u64,
    pub retained_visits: u64,
    pub emitted: u64,
    pub key_updates: u64,
}
macro_rules! count {
    ($s:expr,$field:ident) => {
        if crate::COLLECT_METRICS {
            $s.$field += 1;
        }
    };
}
#[derive(Clone)]
struct Row {
    key: Term,
    payload: Term,
}
#[derive(Clone)]
enum Op {
    Request(Term, Term),
    Replace(Term, Term, Term),
    Bind(Var, Term),
}
#[derive(Clone)]
struct Instruction {
    op: Op,
    remaining: Term,
}
/// Checking this exact source prevents silent specialization of a different program.
pub struct Prepared;
impl Prepared {
    pub fn new(source: &[Rule]) -> Result<Self, String> {
        if source != source_rules() {
            return Err("unsupported source: S01 exact six-rule contract required".into());
        }
        Ok(Self)
    }
    pub fn start(&self, query: Query, mode: Mode) -> Result<Engine, String> {
        Engine::new(query, mode)
    }
}
struct Request {
    round: Term,
    key: Term,
    previous: Option<(usize, usize)>,
}
pub struct Engine {
    mode: Mode,
    left: BTreeMap<usize, Row>,
    right: BTreeMap<usize, Row>,
    next_right: usize,
    left_keys: BTreeMap<Term, BTreeSet<usize>>,
    right_keys: BTreeMap<Term, BTreeSet<usize>>,
    pairs: BTreeMap<Term, BTreeSet<(usize, usize)>>,
    left_incidence: BTreeMap<usize, BTreeSet<(usize, usize)>>,
    right_incidence: BTreeMap<usize, BTreeSet<(usize, usize)>>,
    dependencies: BTreeMap<Var, BTreeSet<(bool, usize)>>,
    bindings: BTreeMap<Var, Term>,
    instructions: VecDeque<Instruction>,
    current: Option<Request>,
    receipts: Vec<Constraint>,
    outputs: Vec<(String, Var)>,
    stopped: Option<Term>,
    done: bool,
    stats: Stats,
}
fn key_ok(t: &Term) -> bool {
    matches!(t, Term::Var(_)) || matches!(t,Term::App(_,xs) if xs.is_empty())
}
fn resolve(term: &Term, bindings: &BTreeMap<Var, Term>) -> Term {
    match term {
        Term::Var(x) => bindings.get(x).cloned().unwrap_or_else(|| term.clone()),
        Term::App(n, xs) => t(
            n,
            xs.iter().map(|x| resolve(x, bindings)).collect::<Vec<_>>(),
        ),
    }
}
impl Engine {
    fn new(query: Query, mode: Mode) -> Result<Self, String> {
        let mut output_names = BTreeSet::new();
        if query
            .outputs
            .iter()
            .any(|(name, _)| !output_names.insert(name))
        {
            return Err("duplicate output name".into());
        }
        let mut left = BTreeMap::new();
        let mut right = BTreeMap::new();
        let mut script = None;
        for row in query.constraints {
            match (row.name.as_str(), row.args.as_slice()) {
                ("left", [key, payload]) if key_ok(key) => {
                    left.insert(
                        left.len(),
                        Row {
                            key: key.clone(),
                            payload: payload.clone(),
                        },
                    );
                }
                ("right", [key, payload]) if key_ok(key) => {
                    right.insert(
                        right.len(),
                        Row {
                            key: key.clone(),
                            payload: payload.clone(),
                        },
                    );
                }
                ("drive", [body]) if script.is_none() => {
                    script = Some(body.clone());
                }
                _ => return Err("unsupported query row or multiple drivers".into()),
            }
        }
        let mut tail = script.ok_or("one driver required")?;
        let mut instructions = VecDeque::new();
        let mut bound = BTreeSet::new();
        loop {
            if tail == atom("nil") {
                break;
            }
            let Term::App(ref name, ref xs) = tail else {
                return Err("closed instruction spine required".into());
            };
            if name != "cons" || xs.len() != 2 {
                return Err("closed instruction spine required".into());
            }
            let Term::App(op, args) = &xs[0] else {
                return Err("instruction constructor required".into());
            };
            let op = match (op.as_str(), args.as_slice()) {
                ("req", [key, round]) if key_ok(key) => Op::Request(key.clone(), round.clone()),
                ("replace", [key, old, new]) if key_ok(key) => {
                    Op::Replace(key.clone(), old.clone(), new.clone())
                }
                ("bind", [Term::Var(x), value])
                    if matches!(value,Term::App(_,xs) if xs.is_empty()) && bound.insert(*x) =>
                {
                    Op::Bind(*x, value.clone())
                }
                _ => return Err(
                    "unsupported instruction: bindings must be distinct variable-to-atom equations"
                        .into(),
                ),
            };
            instructions.push_back(Instruction {
                op,
                remaining: tail.clone(),
            });
            tail = xs[1].clone();
        }
        let next_right = right.len();
        let mut this = Self {
            mode,
            left,
            right,
            next_right,
            left_keys: BTreeMap::new(),
            right_keys: BTreeMap::new(),
            pairs: BTreeMap::new(),
            left_incidence: BTreeMap::new(),
            right_incidence: BTreeMap::new(),
            dependencies: BTreeMap::new(),
            bindings: BTreeMap::new(),
            instructions,
            current: None,
            receipts: vec![],
            outputs: query.outputs,
            stopped: None,
            done: false,
            stats: Stats::default(),
        };
        for side in [true, false] {
            let ids: Vec<_> = if side {
                this.left.keys()
            } else {
                this.right.keys()
            }
            .copied()
            .collect();
            for id in ids {
                let row = if side {
                    &this.left[&id]
                } else {
                    &this.right[&id]
                };
                if let Term::Var(x) = row.key {
                    this.dependencies.entry(x).or_default().insert((side, id));
                }
                this.attach(side, id);
            }
        }
        Ok(this)
    }
    fn attach(&mut self, side: bool, id: usize) {
        let key = resolve(
            if side {
                &self.left[&id].key
            } else {
                &self.right[&id].key
            },
            &self.bindings,
        );
        let (own, other) = if side {
            (&mut self.left_keys, &self.right_keys)
        } else {
            (&mut self.right_keys, &self.left_keys)
        };
        own.entry(key.clone()).or_default().insert(id);
        count!(self.stats, key_updates);
        if self.mode == Mode::Retained
            && let Some(partners) = other.get(&key)
        {
            for &partner in partners {
                let pair = if side { (id, partner) } else { (partner, id) };
                if self.pairs.entry(key.clone()).or_default().insert(pair) {
                    self.left_incidence.entry(pair.0).or_default().insert(pair);
                    self.right_incidence.entry(pair.1).or_default().insert(pair);
                    count!(self.stats, constructed_pairs);
                }
            }
        }
    }
    fn detach(&mut self, side: bool, id: usize) {
        let key = resolve(
            if side {
                &self.left[&id].key
            } else {
                &self.right[&id].key
            },
            &self.bindings,
        );
        let own = if side {
            &mut self.left_keys
        } else {
            &mut self.right_keys
        };
        if let Some(bucket) = own.get_mut(&key) {
            bucket.remove(&id);
            if bucket.is_empty() {
                own.remove(&key);
            }
        }
        count!(self.stats, key_updates);
        if self.mode == Mode::Retained {
            let incident = if side {
                self.left_incidence.remove(&id)
            } else {
                self.right_incidence.remove(&id)
            };
            if let Some(incident) = incident {
                for pair in incident {
                    let opposite = if side {
                        &mut self.right_incidence
                    } else {
                        &mut self.left_incidence
                    };
                    let other = if side { pair.1 } else { pair.0 };
                    if let Some(set) = opposite.get_mut(&other) {
                        set.remove(&pair);
                        if set.is_empty() {
                            opposite.remove(&other);
                        }
                    }
                    if let Some(set) = self.pairs.get_mut(&key) {
                        assert!(set.remove(&pair));
                        if set.is_empty() {
                            self.pairs.remove(&key);
                        }
                    }
                    count!(self.stats, invalidated_pairs);
                }
            }
        }
    }
    fn next_pair(&mut self, req: &Request) -> Option<(usize, usize)> {
        use std::ops::Bound::{Excluded, Unbounded};
        let result = if self.mode == Mode::Retained {
            let pairs = self.pairs.get(&req.key)?;
            if let Some(last) = req.previous {
                pairs.range((Excluded(last), Unbounded)).next().copied()
            } else {
                pairs.first().copied()
            }
        } else {
            let left = self.left_keys.get(&req.key)?;
            let right = self.right_keys.get(&req.key)?;
            match req.previous {
                None => Some((*left.first()?, *right.first()?)),
                Some((a, b)) => {
                    if let Some(&next) = right.range((Excluded(b), Unbounded)).next() {
                        Some((a, next))
                    } else {
                        Some((
                            *left.range((Excluded(a), Unbounded)).next()?,
                            *right.first()?,
                        ))
                    }
                }
            }
        };
        if result.is_some() {
            if self.mode == Mode::Retained {
                count!(self.stats, retained_visits);
            } else {
                count!(self.stats, direct_pairs);
            }
        }
        result
    }
    /// One receipt or one source-driver operation per step. Setup and row updates
    /// have size-dependent finite work; this is not a hard-latency guarantee.
    pub fn advance(&mut self, budget: usize) -> bool {
        for _ in 0..budget {
            if self.done {
                break;
            }
            if let Some(mut request) = self.current.take() {
                if let Some((a, b)) = self.next_pair(&request) {
                    self.receipts.push(c(
                        "receipt",
                        [
                            request.round.clone(),
                            self.left[&a].payload.clone(),
                            self.right[&b].payload.clone(),
                        ],
                    ));
                    request.previous = Some((a, b));
                    self.current = Some(request);
                    count!(self.stats, emitted);
                }
                continue;
            }
            let Some(instruction) = self.instructions.pop_front() else {
                self.done = true;
                break;
            };
            match instruction.op {
                Op::Request(key, round) => {
                    self.current = Some(Request {
                        key: resolve(&key, &self.bindings),
                        round,
                        previous: None,
                    });
                }
                Op::Replace(key, old, new) => {
                    let key = resolve(&key, &self.bindings);
                    let old = resolve(&old, &self.bindings);
                    let id = self.right_keys.get(&key).and_then(|ids| {
                        ids.iter()
                            .copied()
                            .find(|id| resolve(&self.right[id].payload, &self.bindings) == old)
                    });
                    let Some(id) = id else {
                        self.stopped = Some(instruction.remaining);
                        self.done = true;
                        break;
                    };
                    self.detach(false, id);
                    let row = self.right.remove(&id).unwrap();
                    if let Term::Var(x) = row.key
                        && let Some(deps) = self.dependencies.get_mut(&x)
                    {
                        deps.remove(&(false, id));
                    }
                    let id = self.next_right;
                    self.next_right += 1;
                    self.right.insert(
                        id,
                        Row {
                            key: key.clone(),
                            payload: new,
                        },
                    );
                    // A resolved key still needs its original variable dependency only
                    // when unbound; this fragment never rebinds an established atom.
                    if let Term::Var(x) = key {
                        self.dependencies.entry(x).or_default().insert((false, id));
                    }
                    self.attach(false, id);
                }
                Op::Bind(var, value) => {
                    let affected = self.dependencies.remove(&var).unwrap_or_default();
                    for &(side, id) in &affected {
                        self.detach(side, id);
                    }
                    self.bindings.insert(var, value);
                    for (side, id) in affected {
                        self.attach(side, id);
                    }
                }
            }
        }
        self.done
    }
    pub fn observe(&self) -> Result<Answer, String> {
        if !self.done {
            return Err("query has pending source work".into());
        }
        let mut residual: Vec<_> = self
            .left
            .values()
            .map(|r| c("left", [r.key.clone(), r.payload.clone()]))
            .chain(
                self.right
                    .values()
                    .map(|r| c("right", [r.key.clone(), r.payload.clone()])),
            )
            .chain(self.receipts.iter().cloned())
            .collect();
        residual.push(if let Some(tail) = &self.stopped {
            c("drive", [tail.clone()])
        } else {
            c("done", [])
        });
        for row in &mut residual {
            for arg in &mut row.args {
                *arg = resolve(arg, &self.bindings);
            }
        }
        Ok(Answer {
            outputs: self
                .outputs
                .iter()
                .map(|(name, var)| (name.clone(), resolve(&Term::Var(*var), &self.bindings)))
                .collect(),
            residual,
        })
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn retained_pairs(&self) -> usize {
        self.pairs.values().map(BTreeSet::len).sum()
    }
}
pub fn source_rules() -> Vec<Rule> {
    vec![
        Rule::propagate(
            "join",
            [
                c("left", [v(0), v(1)]),
                c("right", [v(0), v(2)]),
                c("request", [v(0), v(3)]),
            ],
            c("receipt", [v(3), v(1), v(2)]).into(),
        ),
        Rule::simplify(
            "ack",
            [c("request", [v(0), v(1)]), c("wait", [v(2)])],
            c("drive", [v(2)]).into(),
        ),
        Rule::simplify(
            "issue",
            [c("drive", [t("cons", [t("req", [v(0), v(1)]), v(2)])])],
            and([c("request", [v(0), v(1)]).into(), c("wait", [v(2)]).into()]),
        ),
        Rule::simplify(
            "replace",
            [
                c(
                    "drive",
                    [t("cons", [t("replace", [v(0), v(1), v(2)]), v(3)])],
                ),
                c("right", [v(0), v(1)]),
            ],
            and([c("right", [v(0), v(2)]).into(), c("drive", [v(3)]).into()]),
        ),
        Rule::simplify(
            "bind",
            [c("drive", [t("cons", [t("bind", [v(0), v(1)]), v(2)])])],
            and([eq(v(0), v(1)), c("drive", [v(2)]).into()]),
        ),
        Rule::simplify("done", [c("drive", [atom("nil")])], c("done", []).into()),
    ]
}
