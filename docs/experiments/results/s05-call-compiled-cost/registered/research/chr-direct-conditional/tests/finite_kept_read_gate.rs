#[path = "../examples/support/finite_bridge.rs"]
mod bridge;
#[path = "../../chr-compiled/experiments/finite_phase.rs"]
#[allow(dead_code)]
mod finite_phase;
#[path = "../examples/support/finite_kept_read.rs"]
mod kept_read;
#[allow(dead_code)]
mod runtime_support;
use chr_syntax::{Answer, Goal, Query, Rule, Var, and, atom, c, eq, or, v};

fn producer(name: &str, repeated: usize) -> Rule {
    Rule {
        name: name.into(),
        kept: vec![c("ready", []); repeated],
        removed: vec![c(name, [v(0)])],
        guards: vec![],
        body: or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
    }
}
fn query(ready: usize) -> Query {
    let mut constraints = vec![c("pick", [v(100)])];
    constraints.extend(vec![c("ready", []); ready]);
    Query {
        constraints,
        outputs: vec![("out".into(), Var(100))],
    }
}
fn collect(rules: &[Rule], prefix: usize, q: &Query) -> Vec<Answer> {
    let phase = kept_read::Prepared::new(rules, prefix).unwrap();
    let bridge = bridge::Bridge::new(rules.to_vec());
    let mut answers = vec![];
    let mut machine = phase.start(q, Default::default()).unwrap();
    let mut report = None;
    for _ in 0..10000 {
        match machine.advance().unwrap() {
            finite_phase::Event::Progress => (),
            finite_phase::Event::Complete(r) => {
                report = Some(r);
                break;
            }
            finite_phase::Event::Exhausted => panic!("exhausted before admission report"),
        }
    }
    for solution in report.expect("bounded phase service").solutions {
        let (q, w) = bridge.transport(solution);
        let mut caller = bridge.start(q, w);
        let mut done = false;
        for _ in 0..10000 {
            match caller.step() {
                bridge::Event::Progress => (),
                bridge::Event::Answer(a) => answers.push(a),
                bridge::Event::Exhausted => {
                    done = true;
                    break;
                }
            }
        }
        assert!(done);
    }
    runtime_support::same_raw(answers.clone(), runtime_support::run(rules, q, 10000));
    answers
}
#[test]
fn kept_ready_read_is_admitted_with_complete_residuals() {
    let rules = vec![producer("pick", 1)];
    for count in [1, 2, 3] {
        assert_eq!(collect(&rules, 1, &query(count)).len(), 2);
    }
}
#[test]
fn repeated_witnesses_need_distinct_occurrences_but_rules_can_reuse_them() {
    let rules = vec![producer("pick", 2)];
    let p = kept_read::Prepared::new(&rules, 1).unwrap();
    assert!(p.start(&query(1), Default::default()).is_err());
    assert_eq!(collect(&rules, 1, &query(2)).len(), 2);
    let rules = vec![producer("pick", 1), producer("other", 1)];
    let mut q = query(1);
    q.constraints.push(c("other", [v(101)]));
    q.outputs.push(("other".into(), Var(101)));
    assert_eq!(collect(&rules, 2, &q).len(), 4);
}
#[test]
fn absent_and_late_readiness_are_unsupported_not_failed_answers() {
    let mut rules = vec![producer("pick", 1)];
    rules.push(Rule::simplify(
        "enable",
        [c("enable", [])],
        c("ready", []).into(),
    ));
    let p = kept_read::Prepared::new(&rules, 1).unwrap();
    let mut q = query(0);
    assert!(p.start(&q, Default::default()).is_err());
    q.constraints.push(c("enable", []));
    assert_eq!(runtime_support::run(&rules, &q, 10000).len(), 2);
    assert!(p.start(&q, Default::default()).is_err());
}
#[test]
fn nonzero_argument_reads_and_consuming_prefixes_are_not_admitted() {
    for arg in [v(0), atom("a")] {
        let mut rule = producer("pick", 1);
        rule.kept = vec![c("ready", [arg])];
        assert!(kept_read::Prepared::new(&[rule], 1).is_err());
    }
    let rules = vec![
        producer("pick", 1),
        Rule::simplify("consume", [c("ready", [])], Goal::True),
    ];
    assert!(kept_read::Prepared::new(&rules, 2).is_err());
}
#[test]
fn weighted_alternatives_aliases_and_failures_keep_raw_multiplicity() {
    for fail in [false, true] {
        let mut p = producer("pick", 1);
        p.body = or(
            eq(v(0), atom("a")),
            if fail {
                Goal::Fail
            } else {
                eq(v(0), atom("a"))
            },
        );
        let mut q = query(1);
        q.constraints.push(c("pick", [v(100)]));
        assert_eq!(collect(&[p], 1, &q).len(), if fail { 1 } else { 4 });
    }
}
#[test]
fn caller_recreates_private_work_using_original_kept_rules() {
    let rules = vec![
        producer("pick", 1),
        Rule::simplify("go", [c("go", [v(0)])], c("pick", [v(0)]).into()),
    ];
    let mut q = query(1);
    q.constraints[0] = c("go", [v(100)]);
    assert_eq!(collect(&rules, 1, &q).len(), 2);
}
#[test]
fn caller_consumption_and_new_private_work_do_not_bypass_readiness() {
    let rules = vec![
        producer("pick", 1),
        Rule::simplify(
            "go",
            [c("go", [v(0)]), c("ready", [])],
            c("pick", [v(0)]).into(),
        ),
    ];
    let mut q = query(1);
    q.constraints.push(c("go", [v(101)]));
    q.outputs.push(("later".into(), Var(101)));
    assert_eq!(collect(&rules, 1, &q).len(), 2);
}
#[test]
fn prefix_priority_precedes_caller_read_consumption() {
    let rules = vec![
        producer("pick", 1),
        Rule::simplify("take", [c("ready", [])], and(vec![c("taken", []).into()])),
    ];
    assert_eq!(collect(&rules, 1, &query(1)).len(), 2);
}
#[test]
fn query_rechecks_and_dropping_pending_admission_leave_prepared_rules_reusable() {
    let rules = vec![producer("pick", 1)];
    let p = kept_read::Prepared::new(&rules, 1).unwrap();
    for _ in 0..3 {
        let mut machine = p.start(&query(1), Default::default()).unwrap();
        assert!(matches!(
            machine.advance().unwrap(),
            finite_phase::Event::Progress
        ));
        drop(machine);
        assert!(p.start(&query(0), Default::default()).is_err());
        assert_eq!(
            p.solve(&query(1), Default::default())
                .unwrap()
                .solutions
                .len(),
            2
        );
    }
}
