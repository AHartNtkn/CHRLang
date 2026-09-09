use chr_reuse::continuations::{Mode, Prepared, Stats};
use chr_syntax::{Query, Rule, Var, c, eq, or, v};
#[test]
fn preparation_reuses_rules_but_not_query_answers_or_identities() {
    let rules = vec![Rule::simplify(
        "duplicate",
        [c("p", [v(0), v(1)])],
        or(eq(v(0), v(1)), eq(v(0), v(1))),
    )];
    for mode in [Mode::Direct, Mode::ExactIds, Mode::Alpha, Mode::AlphaLive] {
        let prepared = Prepared::new(rules.clone(), mode).unwrap();
        for n in [9, 90, 900] {
            let query = Query {
                constraints: vec![c("p", [v(n), v(n + 1)])],
                outputs: vec![("a".into(), Var(n)), ("b".into(), Var(n + 1))],
            };
            let mut search = prepared.start(query).unwrap();
            let batch = search.advance(100);
            assert!(batch.exhausted);
            assert_eq!(batch.answers.len(), 2);
            for a in batch.answers {
                assert_eq!(a.outputs[0].1, a.outputs[1].1);
                assert!(a.residual.is_empty());
            }
            if !chr_reuse::continuations::COLLECT_METRICS {
                assert_eq!(search.stats(), &Stats::default());
                assert_eq!(search.source_stats().steps, 0);
            }
        }
    }
}
