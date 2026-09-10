#[path = "../examples/support/repeated_source.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_restoration::reunion::{AttemptPolicy, PreparedPhase};
use chr_restoration::{Mode, Prepared, Step};
use chr_syntax::{Answer, Goal, Query, Rule, atom, c, or};

fn policies() -> [AttemptPolicy; 5] {
    [
        AttemptPolicy::EveryBoundary,
        AttemptPolicy::FixedSkip(1),
        AttemptPolicy::FixedSkip(8),
        AttemptPolicy::FailedCheckBackoff { max_skip: 1 },
        AttemptPolicy::FailedCheckBackoff { max_skip: 8 },
    ]
}
fn collect(mut step: impl FnMut() -> Step) -> Vec<Answer> {
    let mut out = vec![];
    for _ in 0..200_000 {
        match step() {
            Step::Answer(a) => out.push(a),
            Step::Exhausted => return out,
            Step::Progress => (),
        }
    }
    panic!("finite source cutoff");
}
#[test]
fn adaptive_and_fixed_schedules_preserve_complete_repeated_source_answers() {
    let mut cases = 0;
    for family in ["plain", "history", "late"] {
        for rounds in [0, 1, 3] {
            for depth in [0, 4] {
                let (rules, local) = fixture::source(family, depth);
                let prepared = PreparedPhase::new(&rules, local).unwrap();
                for seed in [0, 1] {
                    let q = fixture::query(family, rounds, depth, seed);
                    let expected = oracle::run(&rules, &q, 200_000);
                    let copy = Prepared::new(&rules).unwrap();
                    let mut e = copy.start(&q, Mode::Copy).unwrap();
                    oracle::same_raw(collect(|| e.advance()), expected.clone());
                    let mut initial = prepared.start(&q).unwrap();
                    oracle::same_raw(collect(|| initial.advance().unwrap()), expected.clone());
                    for policy in policies() {
                        let mut e = prepared.start_repeated_with_policy(&q, policy).unwrap();
                        oracle::same_raw(collect(|| e.advance().unwrap()), expected.clone());
                        cases += 1;
                    }
                }
            }
        }
    }
    assert_eq!(cases, 180);
}
#[test]
fn finite_answer_progress_failure_and_cancelled_query_reuse() {
    let rules = vec![
        Rule::simplify(
            "choice",
            [c("job", [atom("left")])],
            or(
                c("done", [atom("left")]).into(),
                c("spin", [atom("left")]).into(),
            ),
        ),
        Rule::simplify(
            "spin",
            [c("spin", [atom("left")])],
            c("spin", [atom("left")]).into(),
        ),
        Rule::simplify("failure", [c("bad", [atom("left")])], Goal::Fail),
        Rule::simplify(
            "join",
            [c("done", [atom("left")]), c("done", [atom("right")])],
            c("answer", [atom("left")]).into(),
        ),
    ];
    let p = PreparedPhase::new(&rules, 3).unwrap();
    let q = Query {
        constraints: vec![c("job", [atom("left")]), c("done", [atom("right")])],
        outputs: vec![],
    };
    let expected = Answer {
        outputs: vec![],
        residual: vec![c("answer", [atom("left")])],
    };
    for policy in policies() {
        for stop in [0, 1, 5, 20] {
            let mut e = p.start_repeated_with_policy(&q, policy).unwrap();
            for _ in 0..stop {
                let _ = e.advance().unwrap();
            }
            drop(e);
            assert_eq!(std::sync::Arc::strong_count(&p), 1);
        }
        let mut e = p.start_repeated_with_policy(&q, policy).unwrap();
        let mut found = false;
        for _ in 0..20_000 {
            if let Step::Answer(a) = e.advance().unwrap() {
                oracle::same_raw(vec![a], vec![expected.clone()]);
                found = true;
                break;
            }
        }
        assert!(found, "finite answer starved");
        drop(e);
        let fail = Query {
            constraints: vec![c("bad", [atom("left")]), c("done", [atom("right")])],
            outputs: vec![],
        };
        let expected = oracle::run(&rules, &fail, 200_000);
        assert!(expected.is_empty());
        let mut e = p.start_repeated_with_policy(&fail, policy).unwrap();
        assert!(collect(|| e.advance().unwrap()).is_empty());
        drop(e);
        assert_eq!(std::sync::Arc::strong_count(&p), 1);
    }
    let weak = std::sync::Arc::downgrade(&p);
    drop(p);
    assert!(weak.upgrade().is_none());
}
#[cfg(feature = "replay-diagnostic")]
#[test]
fn backoff_avoids_failed_attempts_on_late_links() {
    for family in ["plain", "history", "late"] {
        for depth in [0, 4] {
            let (rules, local) = fixture::source(family, depth);
            let p = PreparedPhase::new(&rules, local).unwrap();
            let q = fixture::query(family, 3, depth, 0);
            let mut eager = p.start_repeated(&q).unwrap();
            let expected = collect(|| eager.advance().unwrap());
            println!(
                "{family} depth={depth} EveryBoundary: checks={} skipped=0 private={} coupled={} epochs={}",
                eager.work.boundary_checks,
                eager.work.private_steps,
                eager.work.coupled_steps,
                eager.epochs
            );
            for policy in policies().into_iter().skip(1) {
                let mut e = p.start_repeated_with_policy(&q, policy).unwrap();
                oracle::same_raw(collect(|| e.advance().unwrap()), expected.clone());
                println!(
                    "{family} depth={depth} {policy:?}: checks={} skipped={} private={} coupled={} epochs={}",
                    e.work.boundary_checks,
                    e.work.skipped_checks,
                    e.work.private_steps,
                    e.work.coupled_steps,
                    e.epochs
                );
                if family == "late" {
                    assert!(e.work.boundary_checks < eager.work.boundary_checks);
                    assert!(e.work.skipped_checks > 0);
                }
                assert!(
                    e.epochs > 1,
                    "later separation opportunities never recovered"
                );
            }
        }
    }
}
