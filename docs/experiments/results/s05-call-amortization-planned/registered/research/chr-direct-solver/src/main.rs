use chr_reference::Search;
use chr_syntax::{Goal, Guard, Query, Rule, Term, Var, atom, c, eq, or, v};
use std::io::{self, BufRead};

fn term(s: &str) -> Term {
    match s.as_bytes()[0] {
        b'v' => v(s[1..].parse().unwrap()),
        b'a' => atom(s),
        _ => panic!("unsupported bridge term"),
    }
}

fn rules(order: usize) -> Vec<Rule> {
    let choose = Rule::simplify(
        "choose",
        vec![c("choose", [v(0)])],
        or(
            eq(v(0), atom("a0")),
            or(eq(v(0), atom("a1")), eq(v(0), atom("a2"))),
        ),
    );
    let given = Rule::simplify("given", vec![c("given", [v(0), v(1)])], eq(v(0), v(1)));
    let mut forbid = Rule::propagate(
        "forbid",
        vec![c("forbid", [v(0), v(1), v(2), v(3)])],
        Goal::Fail,
    );
    forbid.guards = vec![Guard::Equal(v(0), v(2)), Guard::Equal(v(1), v(3))];
    let all = [choose, given, forbid];
    let permutations = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    permutations[order]
        .iter()
        .map(|&i| all[i].clone())
        .collect()
}

fn ground(t: &Term) -> u8 {
    match t {
        Term::App(name, children) if children.is_empty() => match name.as_str() {
            "a0" => 0,
            "a1" => 1,
            "a2" => 2,
            _ => panic!("unknown output atom"),
        },
        _ => panic!("nonground or structured answer outside fragment"),
    }
}

fn run(query: Query, order: usize) {
    let mut search = Search::new(rules(order), query).unwrap();
    let batch = search.advance(1_000_000);
    let answers: Vec<String> = batch
        .answers
        .iter()
        .map(|answer| {
            let outputs: Vec<String> = answer
                .outputs
                .iter()
                .map(|(name, t)| format!("[{:?},{}]", name, ground(t)))
                .collect();
            let residual: Vec<String> = answer
                .residual
                .iter()
                .map(|constraint| {
                    assert_eq!(constraint.name, "forbid");
                    assert_eq!(constraint.args.len(), 4);
                    format!(
                        "{:?}",
                        constraint.args.iter().map(ground).collect::<Vec<_>>()
                    )
                })
                .collect();
            format!(
                "{{\"outputs\":[{}],\"residual\":[{}]}}",
                outputs.join(","),
                residual.join(",")
            )
        })
        .collect();
    println!(
        "{{\"raw\":{},\"splits\":{},\"exhausted\":{},\"answers\":[{}]}}",
        search.stats().completed_branches,
        search.stats().splits,
        batch.exhausted,
        answers.join(",")
    );
}

fn main() {
    let order: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut query = Query {
        constraints: vec![],
        outputs: vec![],
    };
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split_whitespace().collect();
        match fields[0] {
            "end" => {
                run(query, order);
                query = Query {
                    constraints: vec![],
                    outputs: vec![],
                };
            }
            "output" => {
                let Term::Var(Var(id)) = term(fields[2]) else {
                    panic!("variable output required")
                };
                query.outputs.push((fields[1].into(), Var(id)));
            }
            "choose" | "given" | "forbid" => query.constraints.push(c(
                fields[0],
                fields[1..].iter().map(|s| term(s)).collect::<Vec<_>>(),
            )),
            _ => panic!("unsupported bridge statement"),
        }
    }
    assert!(
        query.constraints.is_empty() && query.outputs.is_empty(),
        "unterminated query"
    );
}
