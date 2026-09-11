//! Multiplicity and transport admission remain operational with diagnostics off.
use chr_factors::parallel_regions::{Mode as RegionalMode, Search as RegionalSearch};
use chr_syntax::{Answer, Query, Rule, Var, atom, c, eq, or, v};

fn fixture() -> (Vec<Rule>, Query, Answer) {
    let rules = ["p", "q"]
        .into_iter()
        .map(|name| {
            Rule::simplify(
                name,
                [c(name, [v(0)])],
                or(eq(v(0), atom("a")), eq(v(0), atom("a"))),
            )
        })
        .collect();
    let query = Query {
        constraints: vec![c("p", [v(0)]), c("q", [v(1)])],
        outputs: vec![("left".into(), Var(0)), ("right".into(), Var(1))],
    };
    let answer = Answer {
        outputs: vec![("left".into(), atom("a")), ("right".into(), atom("a"))],
        residual: vec![],
    };
    (rules, query, answer)
}
fn zero<T: std::fmt::Debug + Default>(stats: &T) {
    assert_eq!(format!("{stats:?}"), format!("{:?}", T::default()));
}
#[test]
fn duplicate_completions_and_regional_credit_do_not_depend_on_diagnostics() {
    if !chr_factors::COLLECT_METRICS {
        // These assertions apply to standalone package builds: Cargo feature
        // unification with another workspace consumer may enable diagnostics.
        assert_eq!(
            std::hint::black_box((
                chr_persistent::COLLECT_METRICS,
                chr_persistent::COLLECT_KERNEL_METRICS,
                chr_observe::COLLECT_METRICS,
            )),
            (false, false, false)
        );
    }
    for mode in [
        RegionalMode::Inline,
        RegionalMode::Threads(1),
        RegionalMode::Threads(2),
    ] {
        let (rules, query, expected) = fixture();
        // One reservation forces repeated release/reissue even with two owners.
        let mut engine = RegionalSearch::new(rules, query, mode, 1, 1).unwrap();
        assert_eq!(engine.factor_count(), 2);
        assert_eq!(engine.raw_count(), None);
        let mut answers = Vec::new();
        for _ in 0..1000 {
            let batch = engine.advance(1).unwrap();
            answers.extend(batch.answers);
            if batch.exhausted {
                break;
            }
        }
        assert!(engine.exhausted());
        assert_eq!(answers, vec![expected]);
        assert_eq!(engine.raw_count(), Some(4));
        engine.shutdown().unwrap();
        assert!(engine.advance(1).is_err());
        if !chr_factors::COLLECT_METRICS {
            zero(engine.stats());
            zero(engine.transport_stats());
            zero(engine.observation_stats());
            for source in engine.source_stats() {
                zero(source);
            }
            zero(&engine.regional_observation_stats());
            assert!(engine.actual_source_stats().is_empty());
            assert!(engine.actual_regional_observation_stats().is_empty());
        }
    }
    for mode in [chr_factors::Mode::Scalar, chr_factors::Mode::Factored] {
        let (rules, query, expected) = fixture();
        let mut engine = chr_factors::Search::new(rules, query, mode).unwrap();
        let batch = engine.advance(1000);
        assert!(batch.exhausted);
        assert_eq!(batch.answers, vec![expected]);
        assert_eq!(engine.raw_count(), Some(4));
        if !chr_factors::COLLECT_METRICS {
            zero(engine.stats());
            zero(engine.observation_stats());
            for source in engine.source_stats() {
                zero(source);
            }
            zero(&engine.regional_observation_stats());
        }
    }
}
