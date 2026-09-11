use chr_reuse::{
    calls::trace::caller::Caller,
    continuations::{Mode, Search},
};
use chr_syntax::{Goal, Query, Rule, Var, atom, c, eq, t, v};
#[path = "../examples/support/call_trace_source.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use fixture::source;
fn check(caller: &mut Caller, rs: &[Rule], q: &Query, cutoff: usize) -> usize {
    let mut run = caller.start(q.clone()).unwrap();
    let mut direct = Search::new(rs.to_vec(), q.clone(), Mode::Direct).unwrap();
    let mut answers = vec![];
    for step in 0..cutoff {
        let actual = caller.advance(&mut run, 1).unwrap();
        let expected = direct.advance(1);
        assert_eq!(
            actual.exhausted, expected.exhausted,
            "exhaustion step {step}"
        );
        assert_eq!(
            actual.answers.len(),
            expected.answers.len(),
            "delivery step {step}"
        );
        for (a, b) in actual.answers.iter().zip(&expected.answers) {
            assert!(
                chr_observe::equivalent(a, b, &mut Default::default()),
                "answer at step {step}: {a:?} / {b:?}"
            );
        }
        answers.extend(actual.answers);
        if actual.exhausted {
            scalar::same_raw(answers, scalar::run(rs, q, 100_000));
            return step + 1;
        }
    }
    assert!(cutoff < 100_000, "source bound");
    cutoff
}
#[test]
fn complete_callers_preserve_delivery_resources_history_and_restarts() {
    let mut cases = 0;
    let mut events = 0;
    for a in [0, 1, 4, 8] {
        for b in [0, 1, 4, 8] {
            for reverse in [false, true] {
                for mode in 0..4 {
                    for offset in [10, 1000] {
                        let (rs, count, q) = source(a, b, reverse, mode, offset);
                        let mut caller = Caller::new(rs.clone(), count).unwrap();
                        for cutoff in [0, 1, 5] {
                            check(&mut caller, &rs, &q, cutoff);
                            events += check(&mut caller, &rs, &q, 100_000);
                        }
                        if cfg!(feature = "metrics") {
                            assert!(caller.stats().hits > 0);
                        }
                        cases += 1;
                    }
                }
            }
        }
    }
    assert_eq!(cases, 256);
    eprintln!("callers={cases} complete_service_events={events}");
}
#[test]
fn a_suspended_call_resumes_after_caller_input() {
    let rs = vec![
        Rule::simplify("go", [c("go", [v(0)])], c("blocked", [v(0)]).into()),
        Rule::simplify("ready", [c("blocked", [atom("a")])], Goal::True),
        Rule::simplify("supply", [c("input", [v(0)])], eq(v(0), atom("a"))),
    ];
    let q = Query {
        constraints: vec![c("go", [v(10)]), c("input", [v(10)])],
        outputs: vec![("out".into(), Var(10))],
    };
    let mut caller = Caller::new(rs.clone(), 2).unwrap();
    for _ in 0..2 {
        check(&mut caller, &rs, &q, 100_000);
    }
    if cfg!(feature = "metrics") {
        assert!(caller.stats().hits > 0);
    }
}

#[test]
fn callers_enforce_table_ownership_and_private_head_ownership() {
    let (mut rs, n, q) = source(1, 4, false, 0, 10);
    let a = Caller::new(rs.clone(), n).unwrap();
    let mut b = Caller::new(rs.clone(), n).unwrap();
    assert!(b.advance(&mut a.start(q).unwrap(), 1).is_err());
    rs.push(Rule::simplify(
        "external_reader",
        [c("wait", [v(0), v(1), v(2)]), c("token", [])],
        Goal::True,
    ));
    assert!(Caller::new(rs, n).is_err());
}

#[test]
fn suspended_fresh_alias_is_shared_with_the_caller_input() {
    let rs = vec![
        Rule::simplify(
            "go",
            [c("go", [v(0)])],
            Goal::And(vec![
                eq(v(0), t("box", [v(1)])),
                c("blocked", [v(1)]).into(),
            ]),
        ),
        Rule::simplify("ready", [c("blocked", [atom("a")])], Goal::True),
        Rule::simplify(
            "supply",
            [c("input", [t("box", [v(0)])])],
            eq(v(0), atom("a")),
        ),
    ];
    let mut caller = Caller::new(rs.clone(), 2).unwrap();
    for id in [10, 1000] {
        let q = Query {
            constraints: vec![c("go", [v(id)]), c("input", [v(id)])],
            outputs: vec![("out".into(), Var(id))],
        };
        check(&mut caller, &rs, &q, 100_000);
    }
}

#[test]
fn changed_query_retention_tracks_completed_and_unfinished_traces() {
    for mode in [0, 1] {
        let (rs, count) = fixture::program(false, mode);
        let mut caller = Caller::new(rs, count).unwrap();
        for q in 0..4 {
            let mut run = caller
                .start(fixture::input(4 + 2 * q, 5 + 2 * q, (1000 * q + 7) as u64))
                .unwrap();
            for _ in 0..100_000 {
                let batch = caller.advance(&mut run, 1).unwrap();
                if batch.exhausted || (mode == 0 && !batch.answers.is_empty()) {
                    break;
                }
            }
            drop(run);
            eprintln!(
                "mode={mode} query={q} unfinished={} nodes={}",
                caller.unfinished_calls(),
                caller.retained_nodes()
            );
            assert_eq!(caller.retained_nodes(), 2 * (q + 1));
            assert_eq!(caller.unfinished_calls(), if mode == 0 { q + 1 } else { 0 });
        }
    }
}
