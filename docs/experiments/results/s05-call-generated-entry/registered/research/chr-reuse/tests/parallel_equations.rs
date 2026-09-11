//! E16 forced-order checks at the real source/equality boundary.
use chr_reuse::parallel_equations::{Controller, Request};
use chr_reuse::{EquationTable, Mode, Substitution};
use chr_syntax::{Answer, Goal, Query, Rule, and, atom, c, eq, or, v};
use std::collections::BTreeMap;

fn solve(request: &Request) -> Option<Substitution> {
    EquationTable::new(Mode::Direct).solve(&request.left, &request.right)
}

fn query() -> Query {
    chr_cases::query(vec![c("start", [v(0)])], &[0])
}

fn choosing(body: Goal) -> Vec<Rule> {
    vec![Rule::simplify("choose", [c("start", [v(0)])], body)]
}

fn binary() -> Vec<Rule> {
    choosing(or(eq(v(0), atom("a")), eq(v(0), atom("b"))))
}

fn same_answers(actual: &[Answer], expected: &[Answer]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert!(chr_observe::equivalent(
            actual,
            expected,
            &mut chr_observe::Stats::default()
        ));
    }
}

// Hold all replies until the source front blocks. Ordinary transitions before
// that point may expose additional already-pending equations in other cursors.
fn hold_until_blocked(controller: &mut Controller) -> Vec<Request> {
    let mut requests = Vec::new();
    for _ in 0..100 {
        requests.extend(controller.prepare().unwrap());
        assert!(controller.prepare().unwrap().is_empty());
        match controller.commit().unwrap() {
            None => return requests,
            Some(batch) => {
                assert!(batch.answers.is_empty());
                assert!(!batch.exhausted);
            }
        }
    }
    panic!("source never reached a blocked equation");
}

fn finish(controller: &mut Controller) -> Vec<Answer> {
    let mut answers = Vec::new();
    for _ in 0..10_000 {
        let requests = controller.prepare().unwrap();
        assert!(controller.prepare().unwrap().is_empty());
        for request in requests.into_iter().rev() {
            controller.accept(request.id, solve(&request)).unwrap();
        }
        let batch = controller.commit().unwrap().expect("all replies supplied");
        answers.extend(batch.answers);
        if batch.exhausted {
            return answers;
        }
    }
    panic!("finite test did not exhaust");
}

#[test]
fn reverse_receipt_does_not_bypass_fifo_commit_or_exhaust_parked_frontier() {
    let mut controller = Controller::new(binary(), query(), 4, 8).unwrap();
    let mut requests = hold_until_blocked(&mut controller);
    assert_eq!(requests.len(), 2);
    let second = requests.pop().unwrap();
    let first = requests.pop().unwrap();
    let before = controller.source_stats().steps;

    controller.accept(second.id, solve(&second)).unwrap();
    assert_eq!(controller.source_stats().steps, before);
    assert!(controller.prepare().unwrap().is_empty());
    assert!(controller.commit().unwrap().is_none());
    assert_eq!(controller.source_stats().steps, before);

    controller.accept(first.id, solve(&first)).unwrap();
    assert_eq!(controller.source_stats().steps, before);
    let batch = controller.commit().unwrap().unwrap();
    assert!(batch.answers.is_empty());
    assert!(!batch.exhausted);
    assert_eq!(controller.source_stats().steps, before + 1);

    let actual = finish(&mut controller);
    let mut owned = chr_reuse::equation_search::Search::new(
        binary(),
        query(),
        chr_reuse::equation_search::Mode::Owned,
    )
    .unwrap();
    let expected = owned.advance(100);
    assert!(expected.exhausted);
    same_answers(&actual, &expected.answers);
    assert_eq!(controller.source_stats().steps, owned.source_stats().steps);
    assert_eq!(controller.source_stats().completed, 2);
}

#[test]
fn buffered_reply_keeps_reservation_until_commit() {
    let rules = choosing(or(
        or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        or(eq(v(0), atom("c")), eq(v(0), atom("d"))),
    ));
    let mut controller = Controller::new(rules, query(), 2, 8).unwrap();
    let mut requests = hold_until_blocked(&mut controller);
    assert_eq!(requests.len(), 2);
    let second = requests.pop().unwrap();
    let first = requests.pop().unwrap();
    controller.accept(second.id, solve(&second)).unwrap();
    assert!(controller.prepare().unwrap().is_empty());
    assert!(controller.commit().unwrap().is_none());
    controller.accept(first.id, solve(&first)).unwrap();
    controller.commit().unwrap().unwrap();
    let new = controller.prepare().unwrap();
    assert_eq!(
        new.len(),
        1,
        "buffered second reply still occupies one slot"
    );
    for request in new {
        controller.accept(request.id, solve(&request)).unwrap();
    }
    controller.commit().unwrap().unwrap();
    assert_eq!(finish(&mut controller).len(), 4);
}

#[test]
fn repeated_prepare_is_inert_even_when_first_pass_issues_nothing() {
    let mut controller = Controller::new(binary(), query(), 4, 8).unwrap();
    assert!(controller.prepare().unwrap().is_empty());
    let before = controller.source_stats().steps;
    let visits = controller.stats().lookahead_visits;
    assert!(visits > 0);
    assert!(controller.prepare().unwrap().is_empty());
    assert!(controller.prepare().unwrap().is_empty());
    assert_eq!(controller.source_stats().steps, before);
    assert_eq!(controller.stats().lookahead_visits, visits);
    let batch = controller.commit().unwrap().unwrap();
    assert!(!batch.exhausted);
    assert_eq!(controller.source_stats().steps, before + 1);
    assert_eq!(finish(&mut controller).len(), 2);
}

#[test]
fn unknown_and_duplicate_replies_are_errors_without_source_effects() {
    let mut controller = Controller::new(binary(), query(), 1, 8).unwrap();
    let mut requests = hold_until_blocked(&mut controller);
    assert_eq!(requests.len(), 1);
    let request = requests.pop().unwrap();
    let before = controller.source_stats().steps;
    // A different currently unissued ID, avoiding any assumption about ID origin.
    let unknown = request.id.checked_add(10_000).unwrap();
    assert!(controller.accept(unknown, None).is_err());
    controller.accept(request.id, solve(&request)).unwrap();
    assert!(controller.accept(request.id, solve(&request)).is_err());
    assert_eq!(controller.source_stats().steps, before);
    controller.commit().unwrap().unwrap();
    assert!(controller.accept(request.id, solve(&request)).is_err());
    assert_eq!(finish(&mut controller).len(), 2);
}

#[test]
fn aliases_dependent_equations_failures_and_wakeup_match_owned_and_reference() {
    use chr_syntax::{Guard, t};
    let bodies = [
        // Different bindings of the same pre-split variable; the second equation
        // in each branch must observe that branch's first result.
        or(
            and([eq(v(0), atom("a")), eq(v(1), v(0))]),
            and([eq(v(0), atom("b")), eq(v(1), v(0))]),
        ),
        // First arm fails only on its second, dependent equation.
        or(
            and([eq(v(0), atom("a")), eq(v(0), atom("b"))]),
            and([eq(v(0), atom("a")), eq(v(1), v(0))]),
        ),
        // Occurs check rejects one branch, preserving the successful sibling.
        or(eq(v(0), t("f", [v(0)])), eq(v(0), atom("a"))),
        // Raw multiplicity must survive even though the answer is a duplicate.
        or(eq(v(0), atom("a")), eq(v(0), atom("a"))),
    ];
    for body in bodies {
        let rules = vec![
            Rule::simplify("choose", [c("start", [v(0), v(1)])], body),
            Rule::simplify(
                "constructor",
                [c("wait", [atom("a"), v(0)])],
                c("seen", [v(0)]).into(),
            ),
            Rule {
                name: "guard".into(),
                kept: vec![],
                removed: vec![c("guarded", [v(0), v(1)])],
                guards: vec![Guard::Equal(v(0), atom("a"))],
                body: c("guard_seen", [v(1)]).into(),
            },
        ];
        let query = chr_cases::query(
            vec![
                c("wait", [v(0), v(1)]),
                c("guarded", [v(0), v(1)]),
                c("alias", [v(0), v(1), v(1)]),
                c("start", [v(0), v(1)]),
            ],
            &[0, 1],
        );
        let mut reference = chr_reference::Search::new(rules.clone(), query.clone()).unwrap();
        let expected = reference.advance(1000);
        assert!(expected.exhausted);
        let mut owned = chr_reuse::equation_search::Search::new(
            rules.clone(),
            query.clone(),
            chr_reuse::equation_search::Mode::Owned,
        )
        .unwrap();
        let scalar = owned.advance(1000);
        assert!(scalar.exhausted);
        for outstanding in [1, 4] {
            let mut controller =
                Controller::new(rules.clone(), query.clone(), outstanding, 8).unwrap();
            let actual = finish(&mut controller);
            same_answers(&actual, &scalar.answers);
            // The copying reference is a semantic oracle only.
            same_answers(&actual, &expected.answers);
            assert_eq!(controller.source_stats().steps, owned.source_stats().steps);
            assert_eq!(
                controller.source_stats().failed,
                owned.source_stats().failed
            );
            assert_eq!(controller.source_stats().completed, owned.stats().completed);
        }
    }
}

#[test]
fn prefix_reply_cleanup_alone_does_not_commit_or_publish() {
    let rules = choosing(or(
        eq(v(0), atom("a")),
        and([eq(v(0), atom("b")), eq(v(0), atom("b"))]),
    ));
    let mut controller = Controller::new(rules.clone(), query(), 4, 8).unwrap();
    let mut pending = BTreeMap::new();
    let mut prefix = Vec::new();
    for _ in 0..100 {
        for request in controller.prepare().unwrap() {
            assert!(pending.insert(request.id, request).is_none());
        }
        let batch = match controller.commit().unwrap() {
            Some(batch) => batch,
            None => {
                let (_, request) = pending.pop_first().expect("blocked request");
                controller.accept(request.id, solve(&request)).unwrap();
                continue;
            }
        };
        prefix.extend(batch.answers);
        if !prefix.is_empty() {
            break;
        }
    }
    assert_eq!(prefix.len(), 1);
    assert!(
        !pending.is_empty(),
        "prefix must leave speculative work to drain"
    );
    let mut owned = chr_reuse::equation_search::Search::new(
        rules,
        query(),
        chr_reuse::equation_search::Mode::Owned,
    )
    .unwrap();
    let mut expected = Vec::new();
    while expected.is_empty() {
        expected.extend(owned.advance(1).answers);
    }
    same_answers(&prefix, &expected);
    assert_eq!(controller.source_stats().steps, owned.source_stats().steps);
    let before = controller.source_stats().steps;
    for (_, request) in pending {
        controller.accept(request.id, solve(&request)).unwrap();
        assert_eq!(controller.source_stats().steps, before);
        assert_eq!(controller.source_stats().completed, 1);
    }
    // Deliberately do not call commit after the caller stops its prefix.
    // The worker wrapper must enforce this boundary during drain/join.
    drop(controller);
}

// Additional actual-worker tests once that API is fixed:
// - Put a two-party barrier immediately around real solve calls in two worker
//   loops, with two eligible requests. This must complete and validates that a
//   receive mutex is not held during solving (queued requests alone do not).
// - Stop on the first answer with accepted requests still outstanding; drain
//   replies and join, preserving committed counts/prefix while accounting all
//   accepted service work. Exercise a bounded result channel to catch join-before-
//   drain deadlock; no source continuation may commit during cleanup.
// - Inject worker panic/channel loss while another worker remains alive. The
//   owner must return an execution error promptly rather than wait forever on
//   retained reply senders or reinterpret the event as unification failure.
