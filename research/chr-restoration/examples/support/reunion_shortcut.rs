//! Exact source-family control; not a general CHR lowering pass.
use chr_syntax::{Constraint, Query, Rule, Term, Var, atom};
use std::collections::BTreeMap;
pub struct Checked {
    rules: Vec<Rule>,
    keys: Vec<String>,
    late: bool,
    payload: bool,
}
fn natural(mut t: &Term) -> bool {
    loop {
        match t {
            Term::App(n, xs) if n == "z" && xs.is_empty() => return true,
            Term::App(n, xs) if n == "s" && xs.len() == 1 => t = &xs[0],
            _ => return false,
        }
    }
}
fn claim(t: &Term, owner: usize, seen: &mut BTreeMap<Var, usize>) -> Result<(), String> {
    match t {
        Term::Var(v) => {
            if seen.insert(*v, owner).is_some_and(|prior| prior != owner) {
                return Err("cross-owner variable".into());
            }
        }
        Term::App(_, xs) => {
            for t in xs {
                claim(t, owner, seen)?;
            }
        }
    }
    Ok(())
}
impl Checked {
    pub fn new(family: &str, owners: usize, rules: &[Rule]) -> Result<Self, String> {
        if !["plain", "equal", "late", "payload"].contains(&family) || owners < 2 {
            return Err("unsupported source family".into());
        }
        if super::fixture::source(family, owners).0 != rules {
            return Err("source differs from checked pure countdown family".into());
        }
        Ok(Self {
            rules: rules.to_vec(),
            keys: (0..owners).map(|i| format!("owner{i}")).collect(),
            late: family == "late",
            payload: family == "payload",
        })
    }
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }
    pub fn lower(&self, query: &Query) -> Result<Query, String> {
        let mut jobs = vec![None; self.keys.len()];
        let mut waits = vec![None; self.keys.len()];
        let mut claims = BTreeMap::new();
        for c in &query.constraints {
            let Some(Term::App(name, xs)) = c.args.first() else {
                return Err("unknown owner".into());
            };
            if !xs.is_empty() {
                return Err("non-atomic owner".into());
            }
            let owner = self
                .keys
                .iter()
                .position(|k| k == name)
                .ok_or("unknown owner")?;
            for t in &c.args {
                claim(t, owner, &mut claims)?;
            }
            match (c.name.as_str(), c.args.len()) {
                ("job", 3) => {
                    if !natural(&c.args[1]) {
                        return Err("counter is not a ground natural".into());
                    }
                    if jobs[owner].replace(&c.args[2]).is_some() {
                        return Err("multiple active jobs for owner".into());
                    }
                }
                ("wait", 2) if self.late => {
                    if waits[owner].replace(&c.args[1]).is_some() {
                        return Err("multiple wait occurrences".into());
                    }
                }
                ("payload", 3) if self.payload => (),
                _ => return Err("extra active or unsupported occurrence".into()),
            }
        }
        for (job, wait) in jobs.iter().zip(&waits) {
            let job = job.ok_or("missing private job")?;
            if self.late && *wait != Some(job) {
                return Err("wait must observe its owner's private output".into());
            }
        }
        Ok(Query {
            constraints: query
                .constraints
                .iter()
                .map(|c| Constraint {
                    name: c.name.clone(),
                    args: c
                        .args
                        .iter()
                        .enumerate()
                        .map(|(i, t)| {
                            if c.name == "job" && i == 1 {
                                atom("z")
                            } else {
                                t.clone()
                            }
                        })
                        .collect(),
                })
                .collect(),
            outputs: query.outputs.clone(),
        })
    }
}
