//! Small source-level checker. No candidate matching, indexing or equality calls.
use chr_compiled::{Commit, View};
use chr_syntax::{Constraint, Goal, Guard, Rule, Term, Var};
use std::collections::BTreeMap;
fn matches(pattern: &Term, value: &Term, env: &mut BTreeMap<Var, Term>) -> bool {
    match pattern {
        Term::Var(v) => match env.get(v) {
            Some(old) => old == value,
            None => {
                env.insert(*v, value.clone());
                true
            }
        },
        Term::App(name, args) => match value {
            Term::App(n, xs) => {
                name == n
                    && args.len() == xs.len()
                    && args.iter().zip(xs).all(|(p, v)| matches(p, v, env))
            }
            Term::Var(_) => false,
        },
    }
}
fn term(t: &Term, env: &mut BTreeMap<Var, Term>, next: &mut u64) -> Term {
    match t {
        Term::Var(v) => env
            .entry(*v)
            .or_insert_with(|| {
                let t = Term::Var(Var(*next));
                *next += 1;
                t
            })
            .clone(),
        Term::App(n, args) => {
            Term::App(n.clone(), args.iter().map(|x| term(x, env, next)).collect())
        }
    }
}
fn goal(g: &Goal, env: &mut BTreeMap<Var, Term>, next: &mut u64) -> Goal {
    match g {
        Goal::Constraint(c) => Goal::Constraint(Constraint {
            name: c.name.clone(),
            args: c.args.iter().map(|x| term(x, env, next)).collect(),
        }),
        Goal::Unify(a, b) => Goal::Unify(term(a, env, next), term(b, env, next)),
        Goal::And(gs) => Goal::And(gs.iter().map(|g| goal(g, env, next)).collect()),
        Goal::True => Goal::True,
        Goal::Fail => Goal::Fail,
        Goal::Or(..) => panic!("outside checked fragment"),
    }
}
fn application(rules: &[Rule], view: &View, r: usize, ids: &[u64]) -> Option<(Goal, u64)> {
    let rule = &rules[r];
    let heads = rule.kept.iter().chain(&rule.removed).collect::<Vec<_>>();
    if ids.len() != heads.len() {
        return None;
    }
    if view.history.contains(&(r, ids.to_vec())) {
        return None;
    }
    let mut env = BTreeMap::new();
    for (h, (&id, head)) in ids.iter().zip(heads).enumerate() {
        if ids[..h].contains(&id) {
            return None;
        }
        let occ = &view.store.iter().find(|(i, _)| *i == id)?.1;
        if head.name != occ.name
            || head.args.len() != occ.args.len()
            || !head
                .args
                .iter()
                .zip(&occ.args)
                .all(|(p, v)| matches(p, v, &mut env))
        {
            return None;
        }
    }
    let mut next = view.next_var;
    for Guard::Equal(a, b) in &rule.guards {
        if term(a, &mut env, &mut next) != term(b, &mut env, &mut next) {
            return None;
        }
    }
    Some((goal(&rule.body, &mut env, &mut next), next))
}
pub fn check_commit(rules: &[Rule], commit: &Commit) {
    assert!(commit.before.pending.is_empty());
    let (body, next) = application(rules, &commit.before, commit.rule, &commit.ids)
        .expect("selected application must be enabled");
    assert_eq!(commit.after.pending, vec![body]);
    assert_eq!(commit.after.next_var, next);
    let removed = &commit.ids[rules[commit.rule].kept.len()..];
    assert_eq!(
        commit.after.store,
        commit
            .before
            .store
            .iter()
            .filter(|(id, _)| !removed.contains(id))
            .cloned()
            .collect::<Vec<_>>()
    );
    let mut history = commit.before.history.clone();
    history.push((commit.rule, commit.ids.clone()));
    history.sort();
    assert_eq!(commit.after.history, history);
    assert_eq!(commit.after.outputs, commit.before.outputs);
}
pub fn terminal(rules: &[Rule], view: &View) -> bool {
    if !view.pending.is_empty() {
        return false;
    }
    fn walk(rules: &[Rule], v: &View, r: usize, heads: usize, ids: &mut Vec<u64>) -> bool {
        if ids.len() == heads {
            return application(rules, v, r, ids).is_some();
        }
        for (id, _) in &v.store {
            if !ids.contains(id) {
                ids.push(*id);
                let found = walk(rules, v, r, heads, ids);
                ids.pop();
                if found {
                    return true;
                }
            }
        }
        false
    }
    !rules.iter().enumerate().any(|(r, rule)| {
        walk(
            rules,
            view,
            r,
            rule.kept.len() + rule.removed.len(),
            &mut vec![],
        )
    })
}
