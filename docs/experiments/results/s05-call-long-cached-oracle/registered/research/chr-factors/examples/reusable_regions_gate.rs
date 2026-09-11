//! Ordinary counter-free complete-product gate, without test-only worker hooks.
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../experiments/reusable_regions.rs"]
mod regions;
#[allow(dead_code)]
#[path = "../experiments/reusable_workers.rs"]
mod workers;
use chr_syntax::{Answer, Rule, atom, c, eq, or, v};
fn main() {
    assert!(!std::hint::black_box(chr_factors::COLLECT_METRICS));
    assert!(!std::hint::black_box(chr_persistent::COLLECT_METRICS));
    let rules = ["p", "q"]
        .into_iter()
        .map(|name| {
            Rule::simplify(
                name,
                [c(name, [v(0)])],
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
            )
        })
        .collect::<Vec<_>>();
    let mut checked = 0;
    for mode in [
        regions::Mode::Inline,
        regions::Mode::Threads(1),
        regions::Mode::Threads(2),
        regions::Mode::Threads(4),
        #[cfg(feature = "worker-lowering")]
        regions::Mode::Contracted,
    ] {
        for quantum in [1, 7] {
            let mut runtime = regions::Runtime::new(rules.clone(), mode, quantum, 4).unwrap();
            for q in [
                chr_cases::query(vec![], &[]),
                chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]),
                chr_cases::query(vec![c("p", [v(0)]), c("q", [v(0)])], &[0]),
                chr_cases::query(vec![c("left", [v(0)]), c("right", [v(1)])], &[0, 1]),
                chr_cases::query(vec![c("p", [atom("c")]), c("q", [v(1)])], &[1]),
            ] {
                let raw = oracle::run(&rules, &q, 100_000);
                let mut expected = Vec::<Answer>::new();
                for answer in &raw {
                    if !expected
                        .iter()
                        .any(|x| chr_observe::equivalent(answer, x, &mut Default::default()))
                    {
                        expected.push(answer.clone());
                    }
                }
                let mut session = runtime.start(q.clone()).unwrap();
                assert!(session.factor_count() > 0);
                let _ = &session.certificate;
                let mut actual = Vec::new();
                for _ in 0..100_000 {
                    let batch = session.advance(1).unwrap();
                    actual.extend(batch.answers);
                    if batch.exhausted {
                        break;
                    }
                }
                assert!(session.exhausted());
                assert_eq!(session.raw_count(), Some(raw.len() as u128));
                oracle::same_raw(actual, expected);
                session.close().unwrap();
                drop(session);
                let mut cancelled = runtime.start(q).unwrap();
                cancelled.advance(1).unwrap();
                drop(cancelled);
                checked += 1;
            }
            runtime.shutdown().unwrap();
        }
    }
    println!(
        "{checked} complete source queries, each followed by cancellation; configured inline/worker source modes"
    );
}
