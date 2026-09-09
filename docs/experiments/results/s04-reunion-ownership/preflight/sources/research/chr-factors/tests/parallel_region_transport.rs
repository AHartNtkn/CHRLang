use chr_factors::parallel_regions::{Mode, Search, WorkerPhase};
use chr_syntax::{Rule, atom, c, eq, or, v};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::time::Duration;

fn rules() -> Vec<Rule> {
    ["p", "q"]
        .into_iter()
        .map(|n| {
            Rule::simplify(
                n,
                [c(n, [v(0)])],
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
            )
        })
        .collect()
}
#[test]
fn real_regional_workers_overlap_service() {
    let latch = Arc::new((Mutex::new(false), Condvar::new()));
    let worker_latch = Arc::clone(&latch);
    let (started_tx, started_rx) = mpsc::channel();
    let hook = Arc::new(move |worker, region, request, phase| {
        if matches!(phase, WorkerPhase::BeforeService) {
            started_tx.send((worker, region, request)).unwrap();
            let (lock, wake) = &*worker_latch;
            let mut released = lock.lock().unwrap();
            while !*released {
                released = wake.wait(released).unwrap();
            }
        }
    });
    let owner = std::thread::spawn(move || {
        let query = chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]);
        let mut search =
            Search::new_with_worker_hook(rules(), query, Mode::Threads(2), 8, 4, hook).unwrap();
        let result = search.advance(100).unwrap();
        search.shutdown().unwrap();
        result
    });
    let first = started_rx.recv_timeout(Duration::from_secs(10));
    let second = started_rx.recv_timeout(Duration::from_secs(10));
    let (lock, wake) = &*latch;
    *lock.lock().unwrap() = true;
    wake.notify_all();
    let result = owner.join().unwrap();
    let (a, r1, id1) = first.expect("first regional service");
    let (b, r2, id2) = second.expect("second regional service overlaps first");
    assert_ne!(a, b);
    assert_ne!(r1, r2);
    assert_ne!(id1, id2);
    assert!(result.exhausted);
    assert_eq!(result.answers.len(), 4);
}
#[test]
fn worker_panic_is_not_logical_refutation() {
    let (done_tx, done_rx) = mpsc::channel();
    let owner = std::thread::spawn(move || {
        let fired = AtomicBool::new(false);
        let hook = Arc::new(move |_, _, _, phase| {
            if matches!(phase, WorkerPhase::BeforeService) && !fired.swap(true, Ordering::SeqCst) {
                panic!("injected regional worker failure");
            }
        });
        let query = chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]);
        let mut search =
            Search::new_with_worker_hook(rules(), query, Mode::Threads(2), 8, 4, hook).unwrap();
        assert!(search.advance(100).is_err());
        assert_eq!(search.stats().empty_refutations, 0);
        let _result = search.shutdown();
        done_tx.send(()).unwrap();
    });
    done_rx
        .recv_timeout(Duration::from_secs(10))
        .expect("worker failure must not leave owner waiting");
    owner.join().unwrap();
}
