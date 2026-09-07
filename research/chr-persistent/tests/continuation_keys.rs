use chr_persistent::continuations::{Cursor, Machine, Step};
use chr_syntax::{Goal, Query, Rule, Var, atom, c, or, v};
fn query(constraints: Vec<chr_syntax::Constraint>, outputs: &[u64]) -> Query {
    Query {
        constraints,
        outputs: outputs.iter().map(|&n| (format!("v{n}"), Var(n))).collect(),
    }
}
fn next(machine: &mut Machine, cursor: Cursor) -> Cursor {
    match machine.step(cursor) {
        Step::Continue(c) => c,
        _ => panic!("expected continuation"),
    }
}
#[test]
fn history_changes_the_next_enabled_event_with_identical_visible_store() {
    let (mut machine, root) = Machine::new(
        vec![Rule::propagate("once", [c("p", [])], Goal::True)],
        query(vec![c("p", [])], &[]),
    )
    .unwrap();
    let before = next(&mut machine, root);
    let body = next(&mut machine, before.clone());
    let after = next(&mut machine, body);
    assert_ne!(machine.key(&before), machine.key(&after));
    assert!(matches!(machine.step(before), Step::Continue(_)));
    assert!(matches!(machine.step(after), Step::Answer(_)));
}
#[test]
fn pending_failure_is_not_a_successful_continuation() {
    let (mut machine, root) = Machine::new(
        vec![Rule::simplify(
            "split",
            [c("p", [])],
            or(Goal::True, Goal::Fail),
        )],
        query(vec![c("p", [])], &[]),
    )
    .unwrap();
    let posted = next(&mut machine, root);
    let body = next(&mut machine, posted);
    let Step::Split(a, b) = machine.step(body) else {
        panic!("expected split")
    };
    assert_ne!(machine.key(&a), machine.key(&b));
    assert!(matches!(machine.step(a), Step::Continue(_)));
    assert!(matches!(machine.step(b), Step::Failed));
}
#[test]
fn keys_preserve_aliases_resource_multiplicity_and_occurrence_order() {
    let queries = [
        query(vec![c("p", [v(0), v(0)])], &[0, 1]),
        query(vec![c("p", [v(0), v(1)])], &[0, 1]),
        query(vec![c("p", [atom("a")])], &[]),
        query(vec![c("p", [atom("a")]), c("p", [atom("a")])], &[]),
        query(vec![c("p", [atom("a")]), c("p", [atom("b")])], &[]),
        query(vec![c("p", [atom("b")]), c("p", [atom("a")])], &[]),
    ];
    let keys = queries
        .into_iter()
        .map(|q| {
            let count = q.constraints.len();
            let (mut machine, mut cursor) = Machine::new(vec![], q).unwrap();
            for _ in 0..count {
                cursor = next(&mut machine, cursor);
            }
            machine.key(&cursor)
        })
        .collect::<Vec<_>>();
    assert_ne!(keys[0], keys[1]);
    assert_ne!(keys[2], keys[3]);
    assert_ne!(keys[4], keys[5]);
}
