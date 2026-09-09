//! Engine-independent, owned language data. This crate contains no evaluator.

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var(pub u64);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Term {
    Var(Var),
    App(String, Vec<Term>),
}

pub fn v(id: u64) -> Term {
    Term::Var(Var(id))
}
pub fn t(name: &str, args: impl Into<Vec<Term>>) -> Term {
    Term::App(name.into(), args.into())
}
pub fn atom(name: &str) -> Term {
    t(name, [])
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Constraint {
    pub name: String,
    pub args: Vec<Term>,
}
pub fn c(name: &str, args: impl Into<Vec<Term>>) -> Constraint {
    Constraint {
        name: name.into(),
        args: args.into(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Goal {
    Constraint(Constraint),
    Unify(Term, Term),
    And(Vec<Goal>),
    Or(Box<Goal>, Box<Goal>),
    True,
    Fail,
}
impl From<Constraint> for Goal {
    fn from(value: Constraint) -> Self {
        Self::Constraint(value)
    }
}
pub fn eq(left: Term, right: Term) -> Goal {
    Goal::Unify(left, right)
}
pub fn and(goals: impl Into<Vec<Goal>>) -> Goal {
    Goal::And(goals.into())
}
pub fn or(left: Goal, right: Goal) -> Goal {
    Goal::Or(Box::new(left), Box::new(right))
}

/// Pure entailment in the finite-tree equality theory; never binds variables.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Guard {
    Equal(Term, Term),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    pub name: String,
    pub kept: Vec<Constraint>,
    pub removed: Vec<Constraint>,
    pub guards: Vec<Guard>,
    pub body: Goal,
}
impl Rule {
    pub fn simplify(name: &str, heads: impl Into<Vec<Constraint>>, body: Goal) -> Self {
        Self {
            name: name.into(),
            kept: vec![],
            removed: heads.into(),
            guards: vec![],
            body,
        }
    }
    pub fn propagate(name: &str, heads: impl Into<Vec<Constraint>>, body: Goal) -> Self {
        Self {
            name: name.into(),
            kept: heads.into(),
            removed: vec![],
            guards: vec![],
            body,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Query {
    pub constraints: Vec<Constraint>,
    pub outputs: Vec<(String, Var)>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Answer {
    pub outputs: Vec<(String, Term)>,
    pub residual: Vec<Constraint>,
}
