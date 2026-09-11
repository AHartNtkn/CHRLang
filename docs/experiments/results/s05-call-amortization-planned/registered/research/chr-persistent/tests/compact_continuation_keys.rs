use chr_persistent::continuations::{Machine, Step};
use chr_syntax::{Query, Rule, Var, and, atom, c, eq, or, t, v};
use std::collections::VecDeque;
#[test]
fn compact_keys_expand_to_owned_keys_across_bindings_and_branches() {
    let rules = vec![Rule::simplify(
        "start",
        [c("start", [v(0), v(1)])],
        and([
            eq(v(0), t("f", [v(1)])),
            or(eq(v(1), atom("z")), eq(v(1), atom("a"))),
            c("keep", [v(0), t("f", [atom("z")])]).into(),
        ]),
    )];
    let q = Query {
        constraints: vec![c("start", [v(9), v(10)])],
        outputs: vec![("out".into(), Var(9))],
    };
    let (mut machine, cursor) = Machine::new(rules, q).unwrap();
    let mut queue = VecDeque::from([cursor]);
    let mut keys = vec![];
    while let Some(cursor) = queue.pop_front() {
        let owned = machine.key(&cursor);
        let compact = machine.compact_key(&cursor);
        assert_eq!(owned, machine.expand_compact_key(compact.clone()));
        assert_eq!(
            owned.clone().alpha(),
            machine.expand_compact_key(compact.clone().alpha())
        );
        assert_eq!(
            owned.clone().alpha_live_history(),
            machine.expand_compact_key(compact.clone().alpha_live_history())
        );
        keys.push((owned, compact));
        match machine.step(cursor) {
            Step::Continue(c) => queue.push_back(c),
            Step::Split(a, b) => {
                queue.push_back(a);
                queue.push_back(b);
            }
            Step::Answer(_) | Step::Failed => {}
        }
    }
    for (a, b) in &keys {
        for (c, d) in &keys {
            assert_eq!(a == c, b == d);
            assert_eq!(
                a.clone().alpha() == c.clone().alpha(),
                b.clone().alpha() == d.clone().alpha()
            );
        }
    }
}
#[test]
fn compact_key_identity_is_local_to_its_machine() {
    let q = Query {
        constraints: vec![c("p", [atom("a")])],
        outputs: vec![],
    };
    let (mut a, ca) = Machine::new(vec![], q.clone()).unwrap();
    let (mut b, cb) = Machine::new(vec![], q).unwrap();
    assert_eq!(a.key(&ca), b.key(&cb));
    assert_ne!(a.compact_key(&ca), b.compact_key(&cb));
}

#[test]
fn late_grounding_and_direct_ground_construction_have_identical_keys() {
    let rules = vec![Rule::simplify(
        "start",
        [c("start", [v(0)])],
        or(
            and([eq(v(0), t("f", [v(1)])), eq(v(1), atom("z"))]),
            and([eq(v(1), atom("z")), eq(v(0), t("f", [atom("z")]))]),
        ),
    )];
    let q = Query {
        constraints: vec![c("start", [v(9)])],
        outputs: vec![("out".into(), Var(9))],
    };
    let (mut m, c) = Machine::new(rules, q).unwrap();
    let mut queue = VecDeque::from([c]);
    let mut finals = vec![];
    while let Some(c) = queue.pop_front() {
        let a = m.key(&c);
        let b = m.compact_key(&c);
        match m.step(c) {
            Step::Continue(c) => queue.push_back(c),
            Step::Split(a, b) => {
                queue.push_back(a);
                queue.push_back(b);
            }
            Step::Answer(_) => finals.push((a, b)),
            Step::Failed => panic!("unexpected failure"),
        }
    }
    assert_eq!(finals.len(), 2);
    assert_eq!(finals[0].0, finals[1].0);
    assert_eq!(finals[0].1, finals[1].1);
}

#[test]
#[should_panic(expected = "key belongs to another machine")]
fn expansion_rejects_a_key_from_another_machine() {
    let q = Query {
        constraints: vec![c("p", [atom("a")])],
        outputs: vec![],
    };
    let (mut a, ca) = Machine::new(vec![], q.clone()).unwrap();
    let (mut b, _) = Machine::new(vec![], q).unwrap();
    let key = a.compact_key(&ca);
    b.expand_compact_key(key);
}
