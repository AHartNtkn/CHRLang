#[path = "../examples/support/answer_wire.rs"]
mod answer_wire;
use answer_wire::{OwnedWire, Symbols};
use chr_syntax::{Answer, Goal, Query, Rule, Var, atom, c, v};
#[test]
fn fixed_wire_preserves_joint_unknowns_and_duplicate_occurrences() {
    let rules = vec![Rule::simplify("rule", [c("p", [atom("a")])], Goal::True)];
    let source = Symbols::source(&rules);
    let query = Query {
        constraints: vec![c("p", [v(7)])],
        outputs: vec![("x".into(), Var(7))],
    };
    let answer = Answer {
        outputs: vec![("x".into(), v(7)), ("a".into(), atom("a"))],
        residual: vec![c("p", [atom("a")]), c("p", [atom("a")])],
    };
    let wire = OwnedWire::new(source.query(&query), vec![answer]);
    drop((rules, source, query));
    assert_eq!(
        wire.bytes,
        vec![
            4, 1, 7, 0, 0, 0, 2, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 2,
            0, 0, 0, 0, 0, 0
        ]
    );
    assert_eq!(wire.symbols.predicates, vec!["p"]);
    assert_eq!(wire.symbols.atoms, vec!["a"]);
}
#[test]
fn query_symbols_append_without_renumbering_source_and_survive_owners() {
    let rules = vec![Rule::simplify("rule", [c("z", [atom("z")])], Goal::True)];
    let source = Symbols::source(&rules);
    let query = Query {
        constraints: vec![c("a", [atom("a")])],
        outputs: vec![],
    };
    let wire = OwnedWire::new(source.query(&query), vec![]);
    drop((rules, source, query));
    assert!(wire.bytes.is_empty());
    assert_eq!(wire.symbols.predicates, vec!["z", "a"]);
    assert_eq!(wire.symbols.atoms, vec!["z", "a"]);
}

#[test]
fn incremental_publication_exposes_each_complete_prefix() {
    let rules = vec![Rule::simplify("rule", [c("p", [atom("a")])], Goal::True)];
    let answer = Answer {
        outputs: vec![("x".into(), atom("a"))],
        residual: vec![],
    };
    let symbols = Symbols::source(&rules);
    let expected = OwnedWire::new(symbols.clone(), vec![answer.clone(), answer.clone()]);
    let mut stream = OwnedWire::new(symbols, vec![]);
    assert!(stream.bytes.is_empty());
    stream.push(answer.clone());
    assert_eq!(stream.bytes, vec![4, 2, 0, 0, 0, 0, 0, 0]);
    let first = stream.bytes.clone();
    stream.push(answer);
    assert_eq!(&stream.bytes[..first.len()], first);
    assert_eq!(stream.bytes, expected.bytes);
}
