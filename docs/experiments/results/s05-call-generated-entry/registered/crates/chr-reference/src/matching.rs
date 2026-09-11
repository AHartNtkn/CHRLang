//! Exhaustive, nonbinding head matching in a frozen branch.
use crate::{
    Stats,
    branch::{Branch, Token},
    instantiate::{self, Bindings},
};
use chr_syntax::{Constraint, Goal, Guard, Rule, Term};

pub(crate) struct Application {
    pub token: Token,
    pub removed: Vec<u64>,
    pub body: Goal,
    pub next_var: u64,
}

pub(crate) fn find(branch: &Branch, rules: &[Rule], stats: &mut Stats) -> Option<Application> {
    for (rule_index, rule) in rules.iter().enumerate() {
        let heads = rule.kept.iter().chain(&rule.removed).collect::<Vec<_>>();
        let matcher = Matcher {
            branch,
            rule,
            rule_index,
            heads,
        };
        if let Some(result) = matcher.search(&mut vec![], &Bindings::new(), stats) {
            return Some(result);
        }
    }
    None
}

struct Matcher<'a> {
    branch: &'a Branch,
    rule: &'a Rule,
    rule_index: usize,
    heads: Vec<&'a Constraint>,
}
impl Matcher<'_> {
    fn search(
        &self,
        selected: &mut Vec<usize>,
        bindings: &Bindings,
        stats: &mut Stats,
    ) -> Option<Application> {
        if selected.len() == self.heads.len() {
            let ids = selected
                .iter()
                .map(|i| self.branch.store[*i].id)
                .collect::<Vec<_>>();
            let token = (self.rule_index, ids.clone());
            if self.branch.history.contains(&token) {
                return None;
            }
            let mut bindings = bindings.clone();
            let mut next_var = self.branch.next_var;
            for Guard::Equal(a, b) in &self.rule.guards {
                let a = instantiate::term(a, &mut bindings, &mut next_var);
                let b = instantiate::term(b, &mut bindings, &mut next_var);
                if self.branch.substitution.resolve(&a) != self.branch.substitution.resolve(&b) {
                    return None;
                }
            }
            let body = instantiate::goal(&self.rule.body, &mut bindings, &mut next_var);
            return Some(Application {
                token,
                removed: ids[self.rule.kept.len()..].to_vec(),
                body,
                next_var,
            });
        }
        let head = self.heads[selected.len()];
        for (index, occurrence) in self.branch.store.iter().enumerate() {
            if selected.contains(&index) {
                continue;
            }
            let value = &occurrence.constraint;
            if head.name != value.name || head.args.len() != value.args.len() {
                continue;
            }
            stats.head_candidates += 1;
            let mut extended = bindings.clone();
            if head
                .args
                .iter()
                .zip(&value.args)
                .all(|(p, t)| match_term(p, &self.branch.substitution.resolve(t), &mut extended))
            {
                selected.push(index);
                let result = self.search(selected, &extended, stats);
                selected.pop();
                if result.is_some() {
                    return result;
                }
            }
        }
        None
    }
}

fn match_term(pattern: &Term, value: &Term, bindings: &mut Bindings) -> bool {
    match pattern {
        Term::Var(var) => match bindings.get(var) {
            Some(previous) => previous == value,
            None => {
                bindings.insert(*var, value.clone());
                true
            }
        },
        Term::App(f, xs) => match value {
            Term::App(g, ys) if f == g && xs.len() == ys.len() => {
                xs.iter().zip(ys).all(|(x, y)| match_term(x, y, bindings))
            }
            _ => false,
        },
    }
}
