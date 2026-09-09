use chr_reuse::parallel_equations::{Mode, Search, WorkerPhase};
use chr_syntax::{Rule, atom, c, eq, or, v};
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::time::Duration;

fn rules() -> Vec<Rule> {
    vec![Rule::simplify(
        "choose",
        [c("start", [v(0)])],
        or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
    )]
}

#[test]
fn two_workers_enter_the_real_solver_before_either_is_released() {
    let latch = Arc::new((Mutex::new(false), Condvar::new()));
    let worker_latch = Arc::clone(&latch);
    let (started_tx, started_rx) = mpsc::channel();
    let hook = Arc::new(move |worker, request, phase| {
        if matches!(phase, WorkerPhase::BeforeSolve) {
            started_tx.send((worker, request)).unwrap();
            let (lock, wake) = &*worker_latch;
            let mut released = lock.lock().unwrap();
            while !*released {
                released = wake.wait(released).unwrap();
            }
        }
    });
    let owner = std::thread::spawn(move || {
        let mut search = Search::new_with_worker_hook(
            rules(),
            chr_cases::query(vec![c("start", [v(0)])], &[0]),
            Mode::Threads(2),
            4,
            8,
            hook,
        )
        .unwrap();
        let batch = search.advance(100).unwrap();
        search.shutdown().unwrap();
        (batch, search.operation_stats().calls)
    });
    // A deadline detects a broken protocol, not a throughput difference. Always
    // release the latch so even an accidentally serialized pool can shut down.
    let first = started_rx.recv_timeout(Duration::from_secs(10));
    let second = started_rx.recv_timeout(Duration::from_secs(10));
    let (lock, wake) = &*latch;
    *lock.lock().unwrap() = true;
    wake.notify_all();
    let (batch, calls) = owner.join().unwrap();
    let (w1, r1) = first.expect("first worker did not enter service");
    let (w2, r2) = second.expect("second worker did not overlap service");
    assert_ne!(w1, w2);
    assert_ne!(r1, r2);
    assert!(batch.exhausted);
    assert_eq!(calls, 2);
    assert_eq!(batch.answers.len(), 2);
    let outputs = batch
        .answers
        .iter()
        .map(|a| a.outputs[0].1.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(outputs, BTreeSet::from([atom("a"), atom("b")]));
}

#[test]
fn prefix_shutdown_finishes_accepted_work_without_committing_it() {
    let rules = vec![Rule::simplify(
        "choose",
        [c("start", [v(0)])],
        or(
            eq(v(0), atom("a")),
            or(eq(v(0), atom("b")), eq(v(0), atom("c"))),
        ),
    )];
    for mode in [Mode::Inline, Mode::Threads(1), Mode::Threads(2)] {
        let rules = rules.clone();
        let (done_tx, done_rx) = mpsc::channel();
        let owner = std::thread::spawn(move || {
            let mut search = Search::new(
                rules,
                chr_cases::query(vec![c("start", [v(0)])], &[0]),
                mode,
                4,
                8,
            )
            .unwrap();
            let mut answers = Vec::new();
            for _ in 0..100 {
                let batch = search.advance(1).unwrap();
                assert!(!batch.exhausted);
                answers.extend(batch.answers);
                if !answers.is_empty() {
                    break;
                }
            }
            assert_eq!(answers.len(), 1);
            assert_eq!(answers[0].outputs[0].1, atom("a"));
            let before = (
                search.source_stats().steps,
                search.source_stats().completed,
                search.stats().committed_equations,
            );
            let issued = search.stats().issued;
            assert_eq!(
                issued - before.2,
                2,
                "two replies must exceed the one-slot reply channel"
            );
            search.shutdown().unwrap();
            assert_eq!(
                before,
                (
                    search.source_stats().steps,
                    search.source_stats().completed,
                    search.stats().committed_equations
                )
            );
            assert_eq!(search.operation_stats().calls, issued);
            assert!(search.advance(1).is_err());
            done_tx.send(()).unwrap();
        });
        done_rx
            .recv_timeout(Duration::from_secs(10))
            .expect("prefix shutdown failed to drain bounded replies");
        owner.join().unwrap();
    }
}

#[test]
fn worker_panic_is_an_execution_error_with_other_workers_alive() {
    let (done_tx, done_rx) = mpsc::channel();
    let owner = std::thread::spawn(move || {
        let fired = AtomicBool::new(false);
        let hook = Arc::new(move |_, _, phase| {
            if matches!(phase, WorkerPhase::BeforeSolve) && !fired.swap(true, Ordering::SeqCst) {
                panic!("injected worker failure");
            }
        });
        let mut search = Search::new_with_worker_hook(
            rules(),
            chr_cases::query(vec![c("start", [v(0)])], &[0]),
            Mode::Threads(2),
            4,
            8,
            hook,
        )
        .unwrap();
        let failed = search.advance(100).is_err();
        let logical_failures = search.source_stats().failed;
        let _shutdown_result = search.shutdown();
        done_tx.send((failed, logical_failures)).unwrap();
    });
    let (failed, logical_failures) = done_rx
        .recv_timeout(Duration::from_secs(10))
        .expect("lost worker reply left owner waiting indefinitely");
    owner.join().unwrap();
    assert!(failed);
    assert_eq!(logical_failures, 0);
}
