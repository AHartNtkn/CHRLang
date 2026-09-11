use chr_persistent::continuations::{PreparedMachine, Step};
use chr_reuse::calls::{
    Fresh,
    trace::{Event, Table},
};
use chr_syntax::{Answer, Constraint, Goal, Query, Rule, Term, Var, atom, c, eq, or, t, v};
use std::collections::{BTreeSet, VecDeque};
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
#[allow(dead_code)]
mod scalar;
fn nat(n: usize) -> Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
fn rules(depth: usize, reverse: bool, fail: bool) -> Vec<Rule> {
    let left = Goal::Constraint(c("wait", [nat(depth), v(0), v(1)]));
    let right = if fail {
        Goal::Fail
    } else {
        Goal::And(vec![eq(v(0), atom("right")), eq(v(1), atom("right"))])
    };
    vec![
        Rule::simplify(
            "call",
            [c("call", [v(0), v(1)])],
            if reverse {
                or(right, left)
            } else {
                or(left, right)
            },
        ),
        Rule::simplify(
            "wait",
            [c("wait", [t("s", [v(0)]), v(1), v(2)])],
            Goal::Constraint(c("wait", [v(0), v(1), v(2)])),
        ),
        Rule::simplify(
            "done",
            [c("wait", [atom("z"), v(0), v(1)])],
            Goal::And(vec![
                eq(v(0), t("pair", [v(3), v(3)])),
                eq(v(1), atom("left")),
            ]),
        ),
    ]
}
fn query(call: Constraint) -> Query {
    let vars = call
        .args
        .iter()
        .filter_map(|t| if let Term::Var(v) = t { Some(*v) } else { None })
        .collect::<BTreeSet<_>>();
    Query {
        constraints: vec![call],
        outputs: vars.into_iter().map(|v| (format!("v{}", v.0), v)).collect(),
    }
}
fn answer(bindings: Vec<(Var, Term)>) -> Answer {
    Answer {
        outputs: bindings
            .into_iter()
            .map(|(v, t)| (format!("v{}", v.0), t))
            .collect(),
        residual: vec![],
    }
}
fn drive(table: &mut Table, rs: &[Rule], calls: &[Constraint], cutoff: usize) -> usize {
    let prepared = PreparedMachine::new(rs.to_vec()).unwrap();
    let qs = calls.iter().cloned().map(query).collect::<Vec<_>>();
    let mut all = Query {
        constraints: calls.to_vec(),
        outputs: vec![],
    };
    for q in &qs {
        all.outputs.extend(q.outputs.clone());
    }
    let mut fresh = Fresh::for_query(&all);
    let mut machines = vec![];
    let mut jobs = VecDeque::new();
    for (i, (call, q)) in calls.iter().zip(&qs).enumerate() {
        let (m, c) = prepared.start(q.clone()).unwrap();
        machines.push(m);
        jobs.push_back((i, table.start(call).unwrap(), c));
    }
    let mut observed = vec![vec![]; calls.len()];
    let mut steps = 0;
    while let Some((i, job, cursor)) = jobs.pop_front() {
        if steps == cutoff {
            return steps;
        }
        steps += 1;
        assert!(steps < 100_000);
        match (
            table.step(job, &mut fresh).unwrap(),
            machines[i].step(cursor),
        ) {
            (Event::Continue(j), Step::Continue(c)) => jobs.push_back((i, j, c)),
            (Event::Split(a, b), Step::Split(c, d)) => {
                jobs.push_back((i, a, c));
                jobs.push_back((i, b, d));
            }
            (Event::Failed, Step::Failed) => (),
            (Event::Answer(bs), Step::Answer(a)) => {
                let actual = answer(bs);
                scalar::same_raw(vec![actual.clone()], vec![a]);
                observed[i].push(actual);
            }
            _ => panic!("source service event differs"),
        }
    }
    for (q, actual) in qs.iter().zip(observed) {
        scalar::same_raw(actual, scalar::run(rs, q, 100_000));
    }
    steps
}
#[test]
fn compressed_calls_preserve_each_fifo_service_event_and_restart() {
    let mut cases = 0;
    let mut steps = 0;
    for depth in [0, 1, 4, 8] {
        for reverse in [false, true] {
            for fail in [false, true] {
                for distinct in [false, true] {
                    for offset in [10, 1000] {
                        let rs = rules(depth, reverse, fail);
                        let mut table = Table::new(rs.clone()).unwrap();
                        let calls = vec![
                            c("call", [v(offset), v(offset + 1)]),
                            c(
                                "call",
                                [
                                    v(offset + 10),
                                    if distinct {
                                        v(offset + 10)
                                    } else {
                                        v(offset + 11)
                                    },
                                ],
                            ),
                        ];
                        for cutoff in [0, 1, 5] {
                            drive(&mut table, &rs, &calls, cutoff);
                            steps += drive(&mut table, &rs, &calls, 100_000);
                        }
                        let before = table.stats().executed;
                        steps += drive(&mut table, &rs, &calls, 100_000);
                        assert_eq!(
                            table.stats().executed,
                            before,
                            "completed replay executes nothing"
                        );
                        cases += 1;
                    }
                }
            }
        }
    }
    assert_eq!(cases, 64);
    eprintln!("cases={cases} exact_service_events={steps}");
}
#[test]
fn linear_progress_is_one_node_and_unique_keys_execute() {
    let rs = rules(8, false, false);
    let mut table = Table::new(rs.clone()).unwrap();
    let call = c("wait", [nat(8), v(10), v(11)]);
    // drive's query helper sees precisely these output variables.
    let steps = drive(&mut table, &rs, std::slice::from_ref(&call), 100_000);
    assert_eq!(table.retained_nodes(), 1);
    assert_eq!(table.unfinished_calls(), 0);
    let before = table.stats().executed;
    drive(&mut table, &rs, &[call], 100_000);
    assert_eq!(table.stats().executed, before);
    drive(
        &mut table,
        &rs,
        &[c("wait", [nat(4), v(10), v(11)])],
        100_000,
    );
    assert_eq!(table.retained_nodes(), 2);
    if cfg!(feature = "metrics") {
        assert_eq!(table.stats().lookups, 3);
        assert_eq!(table.stats().hits, 1);
        assert!(table.stats().executed > before);
        eprintln!(
            "linear_steps={steps} first_call_nodes=1 stats={:?}",
            table.stats()
        );
    }
}
#[test]
fn foreign_jobs_and_unowned_families_are_rejected() {
    let rs = rules(0, false, false);
    let mut a = Table::new(rs.clone()).unwrap();
    let mut b = Table::new(rs).unwrap();
    let call = c("call", [v(10), v(11)]);
    let j = a.start(&call).unwrap();
    assert!(b.step(j, &mut Fresh::for_query(&query(call))).is_err());
    assert!(
        Table::new(vec![Rule::simplify(
            "external",
            [c("call", [v(0)])],
            Goal::Constraint(c("external", [v(0)]))
        )])
        .is_err()
    );
}

#[test]
fn replayed_existentials_are_fresh_across_callers_and_suspended_calls_stay_errors() {
    let rs = rules(0, false, false);
    let mut table = Table::new(rs).unwrap();
    let q = query(c("wait", [nat(0), v(100), v(101)]));
    let mut fresh = Fresh::for_query(&q);
    let mut returned = BTreeSet::new();
    for _ in 0..3 {
        let mut job = table.start(&q.constraints[0]).unwrap();
        loop {
            match table.step(job, &mut fresh).unwrap() {
                Event::Continue(next) => job = next,
                Event::Answer(bs) => {
                    let Term::App(name, args) = &bs[0].1 else {
                        panic!("pair")
                    };
                    assert_eq!(name, "pair");
                    assert_eq!(args[0], args[1]);
                    let Term::Var(v) = args[0] else {
                        panic!("fresh variable")
                    };
                    assert!(v.0 > 101);
                    assert!(returned.insert(v));
                    break;
                }
                _ => panic!("linear result"),
            }
        }
    }
    let mut table = Table::new(vec![Rule::simplify(
        "only_a",
        [c("p", [atom("a")])],
        Goal::True,
    )])
    .unwrap();
    for _ in 0..2 {
        let mut job = table.start(&c("p", [atom("b")])).unwrap();
        loop {
            match table.step(job, &mut fresh) {
                Ok(Event::Continue(next)) => job = next,
                Err(e) => {
                    assert!(e.contains("residual"));
                    break;
                }
                _ => panic!("suspension must be an error"),
            }
        }
    }
}
