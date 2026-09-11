//! S00: independently predicted distinctions in the public reference contract.
use chr_reference::Search;
use chr_syntax::{Answer, Goal, Query, Rule, and, c, or};

fn run(name: &str, rules: Vec<Rule>, mut expected: Vec<Answer>, counts: (u64, u64, u64)) {
    let query = Query {
        constraints: vec![c("start", [])],
        outputs: vec![],
    };
    let mut search = Search::new(rules, query).unwrap();
    let mut batch = search.advance(200);
    assert!(batch.exhausted, "{name}: unfinished");
    batch.answers.sort();
    expected.sort();
    assert_eq!(batch.answers, expected, "{name}: full observations");
    let stats = search.stats();
    assert_eq!(
        (
            stats.completed_branches,
            stats.duplicate_answers,
            stats.failed_branches
        ),
        counts,
        "{name}: accounting"
    );
    println!(
        "{name}\tunique={}\tcompleted={}\tduplicates={}\tfailed={}\texhausted=true",
        batch.answers.len(),
        counts.0,
        counts.1,
        counts.2
    );
}
fn rule(body: Goal) -> Rule {
    Rule::simplify("start", [c("start", [])], body)
}
fn answer(residual: Vec<chr_syntax::Constraint>) -> Answer {
    Answer {
        outputs: vec![],
        residual,
    }
}
fn main() {
    run(
        "equal-arms",
        vec![rule(or(Goal::True, Goal::True))],
        vec![answer(vec![])],
        (2, 1, 0),
    );
    run(
        "failure-before-choice",
        vec![rule(and([Goal::Fail, or(Goal::True, Goal::True)]))],
        vec![],
        (0, 0, 1),
    );
    run(
        "failure-after-choice",
        vec![rule(and([or(Goal::True, Goal::True), Goal::Fail]))],
        vec![],
        (0, 0, 2),
    );
    let a = Rule::simplify("a", [c("start", [])], c("a", []).into());
    let b = Rule::simplify("b", [c("start", [])], c("b", []).into());
    run(
        "committed-a",
        vec![a.clone(), b.clone()],
        vec![answer(vec![c("a", [])])],
        (1, 0, 0),
    );
    run(
        "committed-b",
        vec![b, a],
        vec![answer(vec![c("b", [])])],
        (1, 0, 0),
    );
    run(
        "residual-multiplicity",
        vec![rule(or(
            c("p", []).into(),
            and([c("p", []).into(), c("p", []).into()]),
        ))],
        vec![
            answer(vec![c("p", [])]),
            answer(vec![c("p", []), c("p", [])]),
        ],
        (2, 0, 0),
    );
}
