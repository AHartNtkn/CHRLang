use chr_reference::Search;
use chr_syntax::{Goal, Query, Rule, and, c, or};
use std::io::{self, Read};
fn names(s: &str) -> Vec<chr_syntax::Constraint> {
    if s == "-" {
        return vec![];
    }
    s.split(',').map(|n| c(n, [])).collect()
}
fn main() {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text).unwrap();
    let mut lines = text.lines();
    let query = Query {
        constraints: names(lines.next().unwrap()),
        outputs: vec![],
    };
    let rules: Vec<_> = lines
        .enumerate()
        .map(|(i, line)| {
            let parts: Vec<_> = line.split(';').collect();
            assert_eq!(parts.len(), 3);
            let alternatives: Vec<_> = parts[2]
                .split('|')
                .map(|a| {
                    if a == "!" {
                        Goal::Fail
                    } else {
                        and(names(a).into_iter().map(Goal::from).collect::<Vec<_>>())
                    }
                })
                .collect();
            let body = alternatives
                .into_iter()
                .rev()
                .reduce(|r, l| or(l, r))
                .unwrap();
            Rule {
                name: format!("r{i}"),
                kept: names(parts[0]),
                removed: names(parts[1]),
                guards: vec![],
                body,
            }
        })
        .collect();
    assert!(rules.iter().all(|r| !r.removed.is_empty()));
    let mut search = Search::new(rules, query).unwrap();
    let batch = search.advance(100000);
    let mut values: Vec<_> = batch
        .answers
        .into_iter()
        .map(|a| {
            assert!(a.outputs.is_empty());
            let mut names: Vec<_> = a
                .residual
                .into_iter()
                .map(|c| {
                    assert!(c.args.is_empty());
                    c.name
                })
                .collect();
            names.sort();
            names.join(",")
        })
        .collect();
    values.sort();
    println!("{} {}", batch.exhausted, search.stats().completed_branches);
    for v in values {
        println!("{v}");
    }
}
