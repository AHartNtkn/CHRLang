use chr_reference::Search;
use chr_syntax::{Constraint, Goal, Query, Rule, Term, Var, and, atom, c, eq, or, v};
use std::io::{self, Read};
fn term(s: &str) -> Term {
    if let Some(n) = s.strip_prefix('v') {
        v(n.parse().unwrap())
    } else {
        atom(s)
    }
}
fn constraint(s: &str) -> Constraint {
    let mut parts = s.split(':');
    c(parts.next().unwrap(), parts.map(term).collect::<Vec<_>>())
}
fn constraints(s: &str) -> Vec<Constraint> {
    if s == "-" {
        vec![]
    } else {
        s.split(',').map(constraint).collect()
    }
}
fn show(t: &Term) -> String {
    match t {
        Term::Var(Var(n)) => format!("v{n}"),
        Term::App(name, args) => {
            assert!(args.is_empty());
            name.clone()
        }
    }
}
fn split_top(s: &str, separator: char) -> Vec<&str> {
    let mut depth = 0;
    let mut start = 0;
    let mut parts = vec![];
    for (i, ch) in s.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if ch == separator && depth == 0 {
            parts.push(&s[start..i]);
            start = i + 1;
        }
    }
    assert_eq!(depth, 0);
    parts.push(&s[start..]);
    parts
}
fn goal(s: &str) -> Goal {
    if s == "-" {
        return and(Vec::<Goal>::new());
    }
    if s == "!" {
        return Goal::Fail;
    }
    let parts = split_top(s, ',');
    if parts.len() > 1 {
        return and(parts.into_iter().map(goal).collect::<Vec<_>>());
    }
    if s.starts_with('(') {
        assert!(s.ends_with(')'));
        let arms = split_top(&s[1..s.len() - 1], '|');
        assert_eq!(arms.len(), 2);
        return or(goal(arms[0]), goal(arms[1]));
    }
    if let Some(rest) = s.strip_prefix("=:") {
        let ts: Vec<_> = rest.split(':').collect();
        assert_eq!(ts.len(), 2);
        eq(term(ts[0]), term(ts[1]))
    } else {
        constraint(s).into()
    }
}
fn main() {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text).unwrap();
    let mut lines = text.lines();
    let constraints = constraints(lines.next().unwrap());
    let out = lines.next().unwrap();
    let outputs = if out == "-" {
        vec![]
    } else {
        out.split(',')
            .enumerate()
            .map(|(i, n)| (format!("o{i}"), Var(n.parse().unwrap())))
            .collect()
    };
    let rules = lines
        .enumerate()
        .map(|(i, line)| {
            let parts: Vec<_> = line.split(';').collect();
            assert_eq!(parts.len(), 3);
            Rule {
                name: format!("r{i}"),
                kept: self::constraints(parts[0]),
                removed: self::constraints(parts[1]),
                guards: vec![],
                body: goal(parts[2]),
            }
        })
        .collect();
    let mut search = Search::new(
        rules,
        Query {
            constraints,
            outputs,
        },
    )
    .unwrap();
    let batch = search.advance(20_000);
    println!("{} {}", batch.exhausted, search.stats().completed_branches);

    for answer in batch.answers {
        println!("ANSWER");
        println!(
            "{}",
            answer
                .outputs
                .iter()
                .map(|(_, t)| show(t))
                .collect::<Vec<_>>()
                .join(",")
        );
        for x in answer.residual {
            println!(
                "{}",
                std::iter::once(x.name)
                    .chain(x.args.iter().map(show))
                    .collect::<Vec<_>>()
                    .join(":")
            );
        }
    }
}
