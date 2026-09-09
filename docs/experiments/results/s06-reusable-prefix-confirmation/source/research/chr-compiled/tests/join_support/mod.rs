//! One outstanding request joins stable tables, followed by source-owned cleanup.
use chr_syntax::{Answer, Query, Rule, Term, and, atom, c, t, v};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "join".into(),
            kept: vec![c("left", [v(0), v(1)]), c("right", [v(0), v(1)])],
            removed: vec![c("request", [v(0), v(2)])],
            guards: vec![],
            body: and(vec![
                c("receipt", [v(0), v(2), v(1), v(1)]).into(),
                c("ack", []).into(),
            ]),
        },
        Rule::simplify(
            "issue",
            [c(
                "drive",
                [t("cons", [t("item", [v(0), v(1)]), v(2)]), v(3)],
            )],
            and(vec![
                c("request", [v(0), v(1)]).into(),
                c("wait", [v(2), v(3)]).into(),
            ]),
        ),
        Rule::simplify(
            "acknowledge",
            [c("wait", [v(0), v(1)]), c("ack", [])],
            c("drive", [v(0), v(1)]).into(),
        ),
        Rule::simplify(
            "cleanup-start",
            [c("drive", [atom("nil"), v(0)])],
            c("clean", [v(0)]).into(),
        ),
        Rule::simplify(
            "cleanup-pair",
            [
                c("left", [v(0), v(1)]),
                c("right", [v(0), v(1)]),
                c("clean", [t("cons", [v(0), v(2)])]),
            ],
            c("clean", [v(2)]).into(),
        ),
        Rule::simplify(
            "complete",
            [c("clean", [atom("nil")])],
            c("done", []).into(),
        ),
    ]
}
pub fn key(i: usize) -> Term {
    atom(&format!("k{i}"))
}
pub fn round(i: usize) -> Term {
    atom(&format!("r{i}"))
}
pub fn query(n: usize, requests: usize) -> Query {
    assert!(n > 0);
    let mut constraints = vec![];
    for i in 0..n {
        constraints.extend([
            c("left", [key(i), v(i as u64)]),
            c("right", [key(i), v(i as u64)]),
        ]);
    }
    let keys = (0..n)
        .rev()
        .fold(atom("nil"), |tail, i| t("cons", [key(i), tail]));
    let rounds = (0..requests).rev().fold(atom("nil"), |tail, i| {
        t(
            "cons",
            [
                t("item", [key(if i % 2 == 0 { 0 } else { n - 1 }), round(i)]),
                tail,
            ],
        )
    });
    constraints.push(c("drive", [rounds, keys]));
    Query {
        constraints,
        outputs: vec![],
    }
}
/// Exact complete residual multiset; payload identity is stable within each key
/// and different across distinct keys. No source store or reference kernel used.
pub fn validate(answer: &Answer, n: usize, requests: usize) -> bool {
    if n == 0 || !answer.outputs.is_empty() || answer.residual.len() != requests + 1 {
        return false;
    }
    let mut seen = vec![false; requests];
    let mut payloads = [None, None];
    let mut done = false;
    for constraint in &answer.residual {
        if constraint.name == "done" && constraint.args.is_empty() && !done {
            done = true;
            continue;
        }
        let [
            Term::App(k, ka),
            Term::App(r, ra),
            Term::Var(a),
            Term::Var(b),
        ] = constraint.args.as_slice()
        else {
            return false;
        };
        if constraint.name != "receipt" || !ka.is_empty() || !ra.is_empty() || a != b {
            return false;
        }
        let Some(i) = r.strip_prefix('r').and_then(|s| s.parse::<usize>().ok()) else {
            return false;
        };
        if i >= requests
            || seen[i]
            || r != &format!("r{i}")
            || k != &format!("k{}", if i % 2 == 0 { 0 } else { n - 1 })
        {
            return false;
        }
        seen[i] = true;
        let slot = if n == 1 { 0 } else { i % 2 };
        if payloads[slot].is_some_and(|p| p != *a) {
            return false;
        }
        payloads[slot] = Some(*a);
    }
    done && seen.into_iter().all(|x| x)
        && !(n > 1 && payloads[0].is_some() && payloads[0] == payloads[1])
}
