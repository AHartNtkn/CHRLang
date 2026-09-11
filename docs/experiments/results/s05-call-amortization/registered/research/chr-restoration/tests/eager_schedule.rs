#[path = "../examples/support/repeated_source.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_restoration::{
    Step,
    reunion::{AttemptPolicy, Eager, PreparedPhase, RepeatedEngine, Scheduled},
};
#[test]
fn stateless_eager_matches_scheduled_events_and_independent_answers() {
    let mut cases = 0;
    for family in ["plain", "history", "late"] {
        for rounds in [0, 1, 3] {
            for depth in [0, 4] {
                let (rules, local) = fixture::source(family, depth);
                let p = PreparedPhase::new(&rules, local).unwrap();
                for seed in [0, 1] {
                    let query = fixture::query(family, rounds, depth, seed);
                    let mut eager = p.start_repeated(&query).unwrap();
                    let mut scheduled = p
                        .start_repeated_with_policy(&query, AttemptPolicy::EveryBoundary)
                        .unwrap();
                    let mut answers = vec![];
                    let mut exhausted = false;
                    for _ in 0..200_000 {
                        match (eager.advance().unwrap(), scheduled.advance().unwrap()) {
                            (Step::Progress, Step::Progress) => (),
                            (Step::Answer(a), Step::Answer(b)) => {
                                assert_eq!(a, b);
                                answers.push(a);
                            }
                            (Step::Exhausted, Step::Exhausted) => {
                                exhausted = true;
                                break;
                            }
                            _ => panic!("eager service events differ"),
                        }
                    }
                    assert!(exhausted);
                    oracle::same_raw(answers, oracle::run(&rules, &query, 200_000));
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 36);
}
#[test]
fn eager_schedule_has_no_adaptive_storage() {
    assert_eq!(std::mem::size_of::<Eager>(), 0);
    assert!(
        std::mem::size_of::<RepeatedEngine<Eager>>()
            < std::mem::size_of::<RepeatedEngine<Scheduled>>()
    );
    println!(
        "eager={} scheduled={} schedule={}",
        std::mem::size_of::<RepeatedEngine<Eager>>(),
        std::mem::size_of::<RepeatedEngine<Scheduled>>(),
        std::mem::size_of::<Scheduled>()
    );
}
