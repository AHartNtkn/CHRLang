//! Shared source definitions for reunion allocation and control qualification.
use chr_syntax::{Goal, Query, Rule, Var, and, atom, c, eq, or, t, v};
fn nat(n: usize) -> chr_syntax::Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
pub fn source(f: &str, owners: usize) -> (Vec<Rule>, usize) {
    let mut rules = vec![];
    for i in 0..owners {
        let k = atom(&format!("owner{i}"));
        rules.push(Rule::simplify(
            &format!("choose{i}"),
            [c("job", [k.clone(), v(0), v(1)])],
            or(
                c("walk", [k.clone(), v(0), v(1), atom("a")]).into(),
                c("walk", [k.clone(), v(0), v(1), atom("b")]).into(),
            ),
        ));
        rules.push(Rule::simplify(
            &format!("walk{i}"),
            [c("walk", [k.clone(), t("s", [v(0)]), v(1), v(2)])],
            c("walk", [k.clone(), v(0), v(1), v(2)]).into(),
        ));
        let ready = c("ready", [k.clone(), v(0), v(1)]);
        let effect = if f == "late" {
            Goal::from(ready)
        } else {
            and(vec![eq(v(0), v(1)), ready.into()])
        };
        rules.push(Rule::simplify(
            &format!("ready{i}"),
            [c("walk", [k.clone(), atom("z"), v(0), v(1)])],
            effect,
        ));
        if f == "late" {
            rules.push(Rule::simplify(
                &format!("wake{i}"),
                [c("wait", [k.clone(), t("done", [v(0)])])],
                c("awake", [k, v(0)]).into(),
            ));
        }
    }
    let n = rules.len();
    let mut heads = vec![];
    let mut effects = vec![];
    for i in 0..owners as u64 {
        heads.push(c("ready", [atom(&format!("owner{i}")), v(i), v(100 + i)]));
        if f == "equal" {
            effects.push(eq(v(100), v(100 + i)));
        }
        if f == "late" {
            effects.push(eq(v(i), t("done", [v(100 + i)])));
        }
    }
    effects.push(
        c(
            "seen",
            std::iter::once(atom("owner0"))
                .chain((0..owners as u64).map(|i| v(100 + i)))
                .collect::<Vec<_>>(),
        )
        .into(),
    );
    rules.push(Rule::simplify("join", heads, and(effects)));
    (rules, n)
}
pub fn query(f: &str, owners: usize, depth: usize, seed: usize) -> Query {
    let mut constraints = vec![];
    let mut outputs = vec![];
    for i in 0..owners {
        let key = atom(&format!("owner{i}"));
        let id = 1000 + seed as u64 * 100 + i as u64;
        constraints.push(c("job", [key.clone(), nat(depth + seed % 2), v(id)]));
        if f == "late" {
            constraints.push(c("wait", [key.clone(), v(id)]));
        }
        if f == "payload" {
            for j in 0..8 {
                constraints.push(c(
                    "payload",
                    [key.clone(), nat(64), atom(&format!("tag{j}-{seed}"))],
                ));
            }
        }
        outputs.push((format!("out{i}"), Var(id)));
    }
    Query {
        constraints,
        outputs,
    }
}
