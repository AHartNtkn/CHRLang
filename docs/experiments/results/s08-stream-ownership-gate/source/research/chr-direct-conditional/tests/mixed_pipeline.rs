mod mixed_support;
use chr_syntax::Answer;
use mixed_support::{RAW_ANSWERS, answer_key, query, rules};

const LIMIT: usize = 2_000_000;
fn record(answer: &Answer, seen: &mut u16) {
    let key = answer_key(answer).expect("complete answer violates source contract");
    let bit = 1 << key;
    assert_eq!(*seen & bit, 0, "duplicate source choice");
    *seen |= bit;
}

#[test]
fn conditional_mixed_phases_preserve_all_choices_and_joint_aliases() {
    use chr_direct_conditional::engine::{Event, PreparedRuleset};
    let prepared = PreparedRuleset::new(rules()).unwrap();
    for (pre, post) in [(0, 0), (1, 0), (0, 1), (2, 3), (3, 2), (0, 0)] {
        let mut engine = prepared.start(query(pre, post)).unwrap();
        let mut seen = 0;
        let mut retained = Vec::new();
        let mut exhausted = false;
        for _ in 0..LIMIT {
            match engine.tick() {
                Event::Answer(answer) => {
                    record(&answer, &mut seen);
                    retained.push(answer);
                }
                Event::Exhausted => {
                    exhausted = true;
                    break;
                }
                Event::Progress => (),
            }
        }
        assert!(exhausted);
        assert_eq!(seen, u16::MAX);
        assert_eq!(retained.len(), RAW_ANSWERS);
        if cfg!(feature = "metrics") {
            eprintln!(
                "conditional pre={pre} post={post} applications={} predicted={}",
                engine.stats().applications,
                pre + 16 * post + 34
            );
        }
        drop(engine);
        assert!(retained.iter().all(|a| answer_key(a).is_some()));
    }
}

#[cfg(feature = "experiment")]
#[test]
fn specialized_mixed_phases_preserve_all_choices_and_joint_aliases() {
    use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
    let prepared = PreparedRuleset::new(rules(), None)
        .unwrap()
        .specialize_inferred();
    for (pre, post) in [(0, 0), (1, 0), (0, 1), (2, 3), (3, 2), (0, 0)] {
        let mut engine = prepared
            .start_search(query(pre, post), Policy::Global, Access::Indexed)
            .unwrap();
        let mut seen = 0;
        let mut retained = Vec::new();
        let mut exhausted = false;
        let mut applications = 0;
        for _ in 0..LIMIT {
            match engine.tick() {
                SearchEvent::Complete(mut branch) => {
                    applications += branch.engine.stats().applications;
                    let answer = branch.engine.observe().unwrap();
                    record(&answer, &mut seen);
                    retained.push(answer);
                }
                SearchEvent::Failed(_) => panic!("source has no failing branch"),
                SearchEvent::Exhausted => {
                    exhausted = true;
                    break;
                }
                SearchEvent::Split { work, .. } => {
                    if let Some(work) = work {
                        applications += work.applications;
                    }
                }
                SearchEvent::Progress => (),
            }
        }
        assert!(exhausted);
        assert_eq!(seen, u16::MAX);
        assert_eq!(retained.len(), RAW_ANSWERS);
        if cfg!(feature = "metrics") {
            eprintln!(
                "specialized pre={pre} post={post} applications={applications} predicted={}",
                1 + 16 * (pre + post + 3)
            );
        }
        drop(engine);
        assert!(retained.iter().all(|a| answer_key(a).is_some()));
    }
}

#[test]
fn checker_rejects_wrong_correlations_aliases_and_residual_multiplicity() {
    use chr_syntax::{atom, c, t, v};
    for key in 0..16 {
        let tuple = t(
            "tuple",
            (0..4)
                .map(|bit| atom(if key & (1 << bit) == 0 { "a" } else { "b" }))
                .collect::<Vec<_>>(),
        );
        let answer = Answer {
            outputs: vec![],
            residual: vec![c("done", [v(9), v(9)]), c("witness", [tuple, v(9), v(9)])],
        };
        assert_eq!(answer_key(&answer), Some(key));
        let mut bad = answer.clone();
        bad.residual[0].args[0] = v(10);
        assert_eq!(answer_key(&bad), None);
        let mut bad = answer.clone();
        bad.residual[1].args[0] = atom("tuple");
        assert_eq!(answer_key(&bad), None);
        let mut bad = answer.clone();
        bad.residual.push(answer.residual[0].clone());
        assert_eq!(answer_key(&bad), None);
        let mut bad = answer.clone();
        bad.residual[1] = bad.residual[0].clone();
        assert_eq!(answer_key(&bad), None);
    }
}
