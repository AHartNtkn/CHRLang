use chr_reference::Search;
use chr_syntax::{Constraint, Goal, Query, Rule, Term, Var, and, atom, c, eq, v};
use std::io::{self, Read};
fn term(s: &str) -> Term {
    if let Some(n) = s.strip_prefix('v') { v(n.parse().unwrap()) } else { atom(s) }
}
fn constraint(s: &str) -> Constraint {
    let mut parts = s.split(':');
    c(parts.next().unwrap(), parts.map(term).collect::<Vec<_>>())
}
fn constraints(s: &str) -> Vec<Constraint> {
    if s == "-" { vec![] } else { s.split(',').map(constraint).collect() }
}
fn show(t: &Term) -> String {
    match t {
        Term::Var(Var(n)) => format!("v{n}"),
        Term::App(name, args) => { assert!(args.is_empty()); name.clone() }
    }
}
fn main() {
    let mut text = String::new(); io::stdin().read_to_string(&mut text).unwrap();
    let mut lines = text.lines();
    let constraints = constraints(lines.next().unwrap());
    let out = lines.next().unwrap();
    let outputs = if out == "-" { vec![] } else { out.split(',').enumerate().map(|(i,n)| (format!("o{i}"), Var(n.parse().unwrap()))).collect() };
    let rules = lines.enumerate().map(|(i,line)| {
        let parts: Vec<_> = line.split(';').collect(); assert_eq!(parts.len(),3);
        let goals: Vec<Goal> = if parts[2] == "-" { vec![] } else { parts[2].split(',').map(|s| {
            if let Some(rest) = s.strip_prefix("=:") { let ts: Vec<_> = rest.split(':').collect(); eq(term(ts[0]),term(ts[1])) } else { constraint(s).into() }
        }).collect() };
        Rule { name: format!("r{i}"), kept: self::constraints(parts[0]), removed: self::constraints(parts[1]), guards: vec![], body: and(goals) }
    }).collect();
    let mut search = Search::new(rules, Query { constraints, outputs }).unwrap();
    let batch = search.advance(20_000);
    println!("{} {}",batch.exhausted,search.stats().completed_branches);
    assert_eq!(batch.answers.len(),1);
    for answer in batch.answers {
        println!("{}",answer.outputs.iter().map(|(_,t)|show(t)).collect::<Vec<_>>().join(","));
        for x in answer.residual { println!("{}",std::iter::once(x.name).chain(x.args.iter().map(show)).collect::<Vec<_>>().join(":")); }
    }
}
