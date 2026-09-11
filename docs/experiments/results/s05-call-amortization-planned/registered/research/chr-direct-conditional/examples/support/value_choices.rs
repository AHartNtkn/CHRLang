//! Expose closed value-choice prefixes as ordinary fresh-output producers.
use chr_syntax::{Goal, Rule, Term, Var, c, v};
use std::collections::BTreeSet;
fn names(g: &Goal, n: &mut BTreeSet<String>) {
    match g {
        Goal::Constraint(c) => {
            n.insert(c.name.clone());
        }
        Goal::And(gs) => {
            for g in gs {
                names(g, n)
            }
        }
        Goal::Or(a, b) => {
            names(a, n);
            names(b, n);
        }
        _ => (),
    }
}
fn ground(t: &Term) -> bool {
    match t {
        Term::Var(_) => false,
        Term::App(_, xs) => xs.iter().all(ground),
    }
}
fn output(g: &Goal) -> Option<Var> {
    match g {
        Goal::Unify(Term::Var(x), t) if ground(t) => Some(*x),
        Goal::Or(a, b) => match (&**a, &**b) {
            (Goal::Fail, _) => output(b),
            (_, Goal::Fail) => output(a),
            _ => {
                let x = output(a)?;
                (output(b) == Some(x)).then_some(x)
            }
        },
        _ => None,
    }
}
fn rewrite(g: &mut Goal, names: &mut BTreeSet<String>, generated: &mut Vec<Rule>) {
    match g {
        Goal::And(gs) => {
            let end = gs.len().saturating_sub(1);
            for g in &mut gs[..end] {
                if let Some(out) = output(g) {
                    let mut i = generated.len();
                    let name = loop {
                        let name = format!("__value_producer_{i}");
                        if names.insert(name.clone()) {
                            break name;
                        }
                        i += 1;
                    };
                    generated.push(Rule::simplify(&name, [c(&name, [v(out.0)])], g.clone()));
                    *g = c(&name, [v(out.0)]).into();
                } else {
                    rewrite(g, names, generated);
                }
            }
            if let Some(last) = gs.last_mut() {
                rewrite(last, names, generated);
            }
        }
        Goal::Or(a, b) => {
            rewrite(a, names, generated);
            rewrite(b, names, generated);
        }
        _ => (),
    }
}
pub fn lower(rules: &[Rule]) -> Vec<Rule> {
    let mut names_set = BTreeSet::new();
    for r in rules {
        for c in r.kept.iter().chain(&r.removed) {
            names_set.insert(c.name.clone());
        }
        names(&r.body, &mut names_set);
    }
    let mut result = rules.to_vec();
    let mut generated = vec![];
    for r in &mut result {
        rewrite(&mut r.body, &mut names_set, &mut generated);
    }
    result.extend(generated);
    result
}
