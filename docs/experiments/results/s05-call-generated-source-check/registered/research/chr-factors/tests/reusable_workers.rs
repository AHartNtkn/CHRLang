#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../experiments/reusable_workers.rs"]
mod workers;
use chr_syntax::{Query, Rule, atom, c, eq, or, v};
use workers::Pool;
fn rules() -> Vec<Rule> {
    vec![Rule::simplify(
        "choose",
        [c("p", [v(0)])],
        or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
    )]
}
fn query(name: &str) -> Query {
    chr_cases::query(vec![c("p", [v(0)]), c(name, [])], &[0])
}
#[test]
fn changed_queries_and_region_counts_reuse_workers_without_answer_history() {
    for count in [1, 2, 4] {
        let mut p = Pool::new(rules(), count, 4).unwrap();
        for n in [3, 1, 0, 4, 2] {
            let qs = (0..n)
                .map(|i| query(&format!("tag{n}_{i}")))
                .collect::<Vec<_>>();
            let id = p.begin(qs.clone()).unwrap();
            assert!(p.begin(vec![]).is_err());
            for (i, q) in qs.iter().enumerate() {
                let mut answers = Vec::new();
                loop {
                    p.submit(&id, i, 1).unwrap();
                    let b = p.receive(&id).unwrap();
                    assert_eq!(b.region, i);
                    assert!(!b.cancelled);
                    answers.extend(b.answers);
                    if b.exhausted {
                        assert_eq!(b.raw_completions, 2);
                        break;
                    }
                }
                oracle::same_raw(answers, oracle::run(&rules(), q, 10_000));
            }
            p.end(&id).unwrap();
            assert_eq!(p.live_queries(), 0);
            assert!(p.submit(&id, 0, 1).is_err());
        }
        p.shutdown().unwrap();
        assert!(p.begin(vec![]).is_err());
    }
}
#[test]
fn pending_cancellation_end_ack_and_identity_isolation() {
    let mut a = Pool::new(rules(), 2, 4).unwrap();
    let mut b = Pool::new(rules(), 1, 1).unwrap();
    let id = a.begin(vec![query("old"), query("old2")]).unwrap();
    let foreign = b.begin(vec![query("foreign")]).unwrap();
    assert!(a.submit(&foreign, 0, 1).is_err());
    for i in 0..4 {
        a.submit(&id, i % 2, 100).unwrap();
    }
    assert!(a.submit(&id, 0, 1).is_err());
    a.end(&id).unwrap();
    assert_eq!(a.live_queries(), 0);
    let fresh = a.begin(vec![query("fresh")]).unwrap();
    assert!(a.submit(&id, 0, 1).is_err());
    a.submit(&fresh, 0, 100).unwrap();
    let batch = a.receive(&fresh).unwrap();
    assert_eq!(batch.raw_completions, 2);
    oracle::same_raw(batch.answers, oracle::run(&rules(), &query("fresh"), 1000));
    a.end(&fresh).unwrap();
    b.end(&foreign).unwrap();
}
#[test]
fn duplicates_keep_raw_multiplicity_but_do_not_leak_seen_sets() {
    let rs = vec![Rule::simplify(
        "dup",
        [c("p", [v(0)])],
        or(eq(v(0), atom("a")), eq(v(0), atom("a"))),
    )];
    let mut p = Pool::new(rs, 1, 1).unwrap();
    for _ in 0..3 {
        let id = p.begin(vec![query("same")]).unwrap();
        p.submit(&id, 0, 100).unwrap();
        let b = p.receive(&id).unwrap();
        assert!(b.exhausted);
        assert_eq!(b.raw_completions, 2);
        assert_eq!(b.answers.len(), 1);
        p.end(&id).unwrap();
    }
}
#[test]
fn worker_failure_is_an_error_and_closes_cleanly() {
    let hook = std::sync::Arc::new(|_, _, _| panic!("injected worker failure"));
    let mut p = Pool::with_hook(rules(), 2, 2, hook).unwrap();
    let id = p.begin(vec![query("x"), query("y")]).unwrap();
    p.submit(&id, 0, 100).unwrap();
    assert!(p.receive(&id).is_err());
    assert!(p.begin(vec![]).is_err());
    assert!(p.shutdown().is_err());
}

#[test]
fn workers_overlap_and_cancel_before_source_service_then_start_fresh() {
    use std::sync::{Arc, Condvar, Mutex, mpsc};
    use std::time::Duration;
    let latch = Arc::new((Mutex::new(false), Condvar::new()));
    let wait = latch.clone();
    let (tx, rx) = mpsc::channel();
    let hook = Arc::new(move |worker, _, _| {
        tx.send(worker).unwrap();
        let (lock, wake) = &*wait;
        let released = lock.lock().unwrap();
        let (released, timeout) = wake
            .wait_timeout_while(released, Duration::from_secs(5), |r| !*r)
            .unwrap();
        assert!(*released && !timeout.timed_out(), "service gate timed out");
    });
    let mut p = Pool::with_hook(rules(), 2, 2, hook).unwrap();
    let id = p.begin(vec![query("old0"), query("old1")]).unwrap();
    p.submit(&id, 0, 100).unwrap();
    p.submit(&id, 1, 100).unwrap();
    let a = rx.recv_timeout(Duration::from_secs(5));
    let b = rx.recv_timeout(Duration::from_secs(5));
    p.cancel(&id).unwrap();
    {
        let (lock, wake) = &*latch;
        *lock.lock().unwrap() = true;
        wake.notify_all();
    }
    assert_ne!(a.unwrap(), b.unwrap());
    assert!(p.submit(&id, 0, 1).is_err());
    for _ in 0..2 {
        let result = p.receive(&id).unwrap();
        assert!(result.cancelled);
        assert!(!result.exhausted);
        assert_eq!(result.raw_completions, 0);
        assert!(result.answers.is_empty());
    }
    p.end(&id).unwrap();
    let fresh = p.begin(vec![query("fresh")]).unwrap();
    p.submit(&fresh, 0, 100).unwrap();
    let result = p.receive(&fresh).unwrap();
    assert!(!result.cancelled);
    oracle::same_raw(result.answers, oracle::run(&rules(), &query("fresh"), 1000));
    p.end(&fresh).unwrap();
}
#[test]
fn invalid_input_does_not_poison_an_idle_pool() {
    let mut p = Pool::new(rules(), 2, 1).unwrap();
    let mut bad = query("bad");
    bad.outputs.push(bad.outputs[0].clone());
    assert!(p.begin(vec![bad]).is_err());
    let id = p.begin(vec![query("valid")]).unwrap();
    p.submit(&id, 0, 100).unwrap();
    assert_eq!(p.receive(&id).unwrap().answers.len(), 2);
    p.end(&id).unwrap();
    let mut bad_rules = rules();
    bad_rules.extend(rules());
    assert!(Pool::new(bad_rules, 2, 1).is_err());
}

#[test]
fn logical_refutation_does_not_poison_the_next_query() {
    let mut pool = Pool::new(rules(), 2, 2).unwrap();
    let q = chr_cases::query(vec![c("p", [atom("c")])], &[]);
    assert!(oracle::run(&rules(), &q, 1000).is_empty());
    let id = pool.begin(vec![q]).unwrap();
    pool.submit(&id, 0, 100).unwrap();
    let b = pool.receive(&id).unwrap();
    assert!(b.exhausted && !b.cancelled);
    assert_eq!(b.raw_completions, 0);
    assert!(b.answers.is_empty());
    pool.end(&id).unwrap();
    assert_eq!(pool.live_queries(), 0);
    let id = pool.begin(vec![query("after_failure")]).unwrap();
    pool.submit(&id, 0, 100).unwrap();
    let b = pool.receive(&id).unwrap();
    oracle::same_raw(
        b.answers,
        oracle::run(&rules(), &query("after_failure"), 1000),
    );
    pool.end(&id).unwrap();
}

#[test]
fn shutdown_drains_a_full_request_window_without_logical_admission() {
    let mut p = Pool::new(rules(), 2, 4).unwrap();
    let id = p.begin(vec![query("x"), query("y")]).unwrap();
    for i in 0..4 {
        p.submit(&id, i % 2, 100).unwrap();
    }
    p.shutdown().unwrap();
    assert_eq!(p.live_queries(), 0);
    assert!(p.receive(&id).is_err());
}
