//! Source-built payload with a fired post-choice constructor rule at every leaf.
//! The right-recursive choice chain and payload cleanup match the read-preserving
//! fixture. Each leaf adds one candidate application and retains its constructed
//! term in the complete answer; failing leaves construct before equality clashes.
// The embedding test/runner supplies the shared read fixture once.
use crate::fixture as base;
use chr_syntax::{Answer, Query, Rule, Term, and, atom, c, eq, or, t, v};
pub fn rules() -> Vec<Rule> {
    let mut rules = base::rules();
    rules[2].body = or(
        c("candidate", [t("s", [v(0)]), v(1), v(2), v(3)]).into(),
        c("choose", [v(0), v(1), v(2), v(3)]).into(),
    );
    rules[3].body = c("candidate", [atom("z"), v(1), v(2), v(3)]).into();
    rules.push(Rule::simplify(
        "construct-candidate",
        [c("candidate", [v(0), v(1), v(2), v(3)])],
        and(vec![
            c("stamp", [t("new", [v(0), v(2), v(2)])]).into(),
            eq(v(3), v(0)),
            c("clean", [v(1), v(2)]).into(),
        ]),
    ));
    rules
}
pub fn query(n: usize, a: usize, all_success: bool) -> Query {
    base::query(n, a, all_success)
}
/// Full answer oracle, modulo joint variable renaming. Returns the selected key
/// depth; callers must also check the complete raw key multiset for their query.
pub fn answer_key(answer: &Answer) -> Option<usize> {
    let [
        (x, Term::Var(a)),
        (alias, Term::Var(b)),
        (unused, Term::Var(u)),
    ] = answer.outputs.as_slice()
    else {
        return None;
    };
    if x != "x"
        || alias != "alias"
        || unused != "unused"
        || a != b
        || a == u
        || answer.residual.len() != 3
    {
        return None;
    }
    let (mut done, mut selected) = (0, None);
    for constraint in &answer.residual {
        match (constraint.name.as_str(), constraint.args.as_slice()) {
            ("done", [Term::Var(left), Term::Var(right)]) if left == a && right == a => done += 1,
            ("stamp", [Term::App(name, args)]) if name == "new" && selected.is_none() => {
                let [key, Term::Var(left), Term::Var(right)] = args.as_slice() else {
                    return None;
                };
                if left != a || right != a {
                    return None;
                }
                let (mut cursor, mut depth) = (key, 0);
                loop {
                    match cursor {
                        Term::App(name, args) if name == "z" && args.is_empty() => break,
                        Term::App(name, args) if name == "s" && args.len() == 1 => {
                            depth += 1;
                            cursor = &args[0]
                        }
                        _ => return None,
                    }
                }
                selected = Some(depth);
            }
            _ => return None,
        }
    }
    if done == 2 { selected } else { None }
}
