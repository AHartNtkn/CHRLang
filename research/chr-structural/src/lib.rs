//! Closed structural operations over immutable finite DAGs; host residuals remain occurrences.
use chr_syntax::{Constraint, Goal, Rule, Term, Var, atom, c, v};
use std::collections::BTreeMap;
use std::rc::Rc;
#[derive(Clone, Debug)]
pub struct Datum(Rc<Node>);
#[derive(Debug)]
enum Node {
    Var(u64),
    App(String, Vec<Datum>),
}
impl Datum {
    pub fn var(id: u64) -> Self {
        Self(Rc::new(Node::Var(id)))
    }
    pub fn app(name: &str, args: impl IntoIterator<Item = Datum>) -> Self {
        Self(Rc::new(Node::App(name.into(), args.into_iter().collect())))
    }
    pub fn export(&self) -> Term {
        match self.0.as_ref() {
            Node::Var(id) => v(*id),
            Node::App(n, args) => Term::App(n.clone(), args.iter().map(Self::export).collect()),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Direct,
    Memo,
}
#[derive(Default, Debug)]
pub struct Stats {
    pub calls: u64,
    pub computed: u64,
    pub hits: u64,
    pub cache_entries: usize,
    pub residual_refs: u64,
    pub certificate_rules: u64,
    pub certificate_terms: u64,
}
pub fn no_c_rules() -> Vec<Rule> {
    vec![
        Rule::simplify("no-c-k", [c("no_c", [atom("k")])], Goal::True),
        Rule::simplify("no-c-s", [c("no_c", [atom("s")])], Goal::True),
        Rule::simplify(
            "no-c-a",
            [c("no_c", [chr_syntax::t("a", [v(0), v(1)])])],
            Goal::And(vec![c("no_c", [v(0)]).into(), c("no_c", [v(1)]).into()]),
        ),
        Rule::simplify(
            "no-c-c",
            [c("no_c", [chr_syntax::t("c", [v(0)])])],
            Goal::Fail,
        ),
    ]
}
fn normalized(rule: &Rule, stats: &mut Stats) -> Rule {
    fn term(t: &Term, vars: &mut BTreeMap<Var, Var>, stats: &mut Stats) -> Term {
        stats.certificate_terms += 1;
        let next = Var(vars.len() as u64);
        match t {
            Term::Var(v) => Term::Var(*vars.entry(*v).or_insert(next)),
            Term::App(n, args) => Term::App(
                n.clone(),
                args.iter().map(|t| term(t, vars, stats)).collect(),
            ),
        }
    }
    fn constraint(c: &Constraint, vars: &mut BTreeMap<Var, Var>, stats: &mut Stats) -> Constraint {
        Constraint {
            name: c.name.clone(),
            args: c.args.iter().map(|t| term(t, vars, stats)).collect(),
        }
    }
    fn goal(g: &Goal, vars: &mut BTreeMap<Var, Var>, stats: &mut Stats) -> Goal {
        match g {
            Goal::Constraint(c) => constraint(c, vars, stats).into(),
            Goal::Unify(a, b) => Goal::Unify(term(a, vars, stats), term(b, vars, stats)),
            Goal::And(gs) => Goal::And(gs.iter().map(|g| goal(g, vars, stats)).collect()),
            Goal::Or(a, b) => Goal::Or(
                Box::new(goal(a, vars, stats)),
                Box::new(goal(b, vars, stats)),
            ),
            Goal::True => Goal::True,
            Goal::Fail => Goal::Fail,
        }
    }
    let mut vars = BTreeMap::new();
    Rule {
        name: String::new(),
        kept: rule
            .kept
            .iter()
            .map(|c| constraint(c, &mut vars, stats))
            .collect(),
        removed: rule
            .removed
            .iter()
            .map(|c| constraint(c, &mut vars, stats))
            .collect(),
        guards: rule.guards.clone(),
        body: goal(&rule.body, &mut vars, stats),
    }
}
type Outcome = Option<Vec<Datum>>;
pub struct NoC {
    mode: Mode,
    cache: BTreeMap<usize, (Datum, Outcome)>,
    stats: Stats,
}
impl NoC {
    pub fn new(rules: &[Rule], mode: Mode) -> Result<Self, String> {
        let mut stats = Stats::default();
        let mut required = no_c_rules()
            .iter()
            .map(|r| normalized(r, &mut stats))
            .collect::<Vec<_>>();
        for r in rules {
            stats.certificate_rules += 1;
            if r.kept
                .iter()
                .chain(&r.removed)
                .any(|c| c.name == "no_c" && c.args.len() == 1)
            {
                let normalized = normalized(r, &mut stats);
                if let Some(i) = required.iter().position(|r| *r == normalized) {
                    required.swap_remove(i);
                } else {
                    return Err("no_c has an uncertified head interaction".into());
                }
            }
        }
        if !required.is_empty() {
            return Err("incomplete no_c operation theory".into());
        }
        Ok(Self {
            mode,
            cache: BTreeMap::new(),
            stats,
        })
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    fn one(&mut self, input: &Datum) -> Outcome {
        self.stats.calls += 1;
        let key = Rc::as_ptr(&input.0) as usize;
        if matches!(self.mode, Mode::Memo)
            && let Some((_, answer)) = self.cache.get(&key)
        {
            self.stats.hits += 1;
            if let Some(a) = answer {
                self.stats.residual_refs += a.len() as u64;
            }
            return answer.clone();
        }
        self.stats.computed += 1;
        let answer = match input.0.as_ref() {
            Node::App(n, args) if (n == "k" || n == "s") && args.is_empty() => Some(vec![]),
            Node::App(n, args) if n == "c" && args.len() == 1 => None,
            Node::App(n, args) if n == "a" && args.len() == 2 => match self.one(&args[0]) {
                None => None,
                Some(mut left) => match self.one(&args[1]) {
                    None => None,
                    Some(right) => {
                        self.stats.residual_refs += right.len() as u64;
                        left.extend(right);
                        Some(left)
                    }
                },
            },
            _ => {
                self.stats.residual_refs += 1;
                Some(vec![input.clone()])
            }
        };
        if matches!(self.mode, Mode::Memo) {
            if let Some(a) = &answer {
                self.stats.residual_refs += a.len() as u64;
            }
            // Retain the input key so its address cannot be reused while the memo entry exists.
            self.cache.insert(key, (input.clone(), answer.clone()));
            self.stats.cache_entries = self.cache.len();
        }
        answer
    }
    pub fn reduce(&mut self, inputs: &[Datum]) -> Outcome {
        let mut residual = vec![];
        for input in inputs {
            let result = self.one(input)?;
            self.stats.residual_refs += result.len() as u64;
            residual.extend(result);
        }
        Some(residual)
    }
}

pub mod regular;

pub mod space;

pub mod finite;
