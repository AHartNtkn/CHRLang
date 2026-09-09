//! Experimental checked resource access and ground-query declarations.
//!
//! These properties do not promise counting eligibility, resource sufficiency,
//! termination, or ground results. Source checks apply to the complete rule set.
use chr_syntax::{Constraint, Goal, Query, Rule, Term};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Predicate {
    pub name: String,
    pub arity: usize,
}
impl Predicate {
    fn matches(&self, c: &Constraint) -> bool {
        self.name == c.name && self.arity == c.args.len()
    }
}

/// `access_rules` is the explicit region allowed to read, consume or produce the
/// resource. Indices refer to this complete rule set, not separately linked units.
#[derive(Clone, Debug)]
pub struct Declaration {
    pub resource: Predicate,
    pub access_rules: Vec<usize>,
    pub entry: Predicate,
    pub ground_argument: usize,
}

pub struct CheckedSource<'a> {
    rules: &'a [Rule],
    declaration: Declaration,
}
fn posts(goal: &Goal, predicate: &Predicate) -> bool {
    match goal {
        Goal::Constraint(c) => predicate.matches(c),
        Goal::And(xs) => xs.iter().any(|g| posts(g, predicate)),
        Goal::Or(a, b) => posts(a, predicate) || posts(b, predicate),
        _ => false,
    }
}
fn ground(term: &Term) -> bool {
    match term {
        Term::Var(_) => false,
        Term::App(_, xs) => xs.iter().all(ground),
    }
}
impl<'a> CheckedSource<'a> {
    pub fn check(rules: &'a [Rule], declaration: Declaration) -> Result<Self, &'static str> {
        if declaration.ground_argument >= declaration.entry.arity {
            return Err("ground argument outside entry arity");
        }
        if declaration.access_rules.is_empty()
            || declaration.access_rules.iter().any(|i| *i >= rules.len())
        {
            return Err("resource access region is empty or outside rule set");
        }
        let mut resource_present = false;
        let mut entry_present = false;
        for (i, rule) in rules.iter().enumerate() {
            let resource_access = rule
                .kept
                .iter()
                .chain(&rule.removed)
                .any(|c| declaration.resource.matches(c))
                || posts(&rule.body, &declaration.resource);
            resource_present |= resource_access;
            entry_present |= rule
                .kept
                .iter()
                .chain(&rule.removed)
                .any(|c| declaration.entry.matches(c));
            if resource_access && !declaration.access_rules.contains(&i) {
                return Err("resource accessed outside declared rule region");
            }
        }
        if !resource_present {
            return Err("declared resource absent from rule set");
        }
        if !entry_present {
            return Err("declared entry absent from rule heads");
        }
        Ok(Self { rules, declaration })
    }
    pub fn rules(&self) -> &'a [Rule] {
        self.rules
    }

    /// Checks every matching initial entry. No entry satisfies this universal
    /// property vacuously; multiple entries are allowed. This is a submission
    /// boundary, not a promise about entries later produced by rule bodies.
    /// The result is not a transferable certificate for a subsequently mutated
    /// query: callers must check the actual query they submit.
    pub fn check_query(&self, query: &Query) -> Result<(), &'static str> {
        check_query(&self.declaration, query)
    }
}

fn check_query(declaration: &Declaration, query: &Query) -> Result<(), &'static str> {
    for c in &query.constraints {
        if declaration.entry.matches(c) && !ground(&c.args[declaration.ground_argument]) {
            return Err("entry argument is not ground at query submission");
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
pub enum Admission {
    Optional,
    Required,
}

/// Experimental admission boundary using the existing prepared executor.
/// Required admission requires a declaration; it does not add stronger facts.
/// The checked source and actual submitted queries cannot change behind it.
pub struct PreparedContract {
    prepared: crate::PreparedRuleset,
    declaration: Option<Declaration>,
}
impl PreparedContract {
    pub fn new(
        rules: Vec<Rule>,
        declaration: Option<Declaration>,
        admission: Admission,
    ) -> Result<Self, String> {
        if let Some(d) = &declaration {
            CheckedSource::check(&rules, d.clone())?;
        } else if matches!(admission, Admission::Required) {
            return Err("resource and ground-entry declaration required".into());
        }
        let prepared = crate::PreparedRuleset::new(rules, None)?;
        Ok(Self {
            prepared,
            declaration,
        })
    }
    pub fn start(
        &self,
        query: Query,
        policy: crate::Policy,
        access: crate::Access,
    ) -> Result<crate::SearchEngine, String> {
        if let Some(d) = &self.declaration {
            check_query(d, &query)?;
        }
        self.prepared.start_search(query, policy, access)
    }
}
