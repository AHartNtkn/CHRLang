#[allow(dead_code)]
mod runtime_support;
#[allow(dead_code)]
#[path = "../examples/support/post_choice_source.rs"]
mod source;
use chr_direct_conditional::engine::{Event, PreparedRuleset};

#[test]
fn immutable_heads_and_consumed_occurrences_do_not_need_candidate_entries() {
    for depth in [1, 4, 16] {
        let rules = source::rules("common", 0);
        let query = source::query("common", 0, depth, 0);
        let expected = source::expected("common", 0, depth, 0);
        runtime_support::same_raw(
            runtime_support::run(&rules, &query, 500_000),
            expected.clone(),
        );
        let mut engine = PreparedRuleset::new(rules).unwrap().start(query).unwrap();
        let mut answers = vec![];
        let mut exhausted = false;
        for _ in 0..500_000 {
            match engine.tick() {
                Event::Answer(a) => answers.push(a),
                Event::Exhausted => {
                    exhausted = true;
                    break;
                }
                Event::Progress => (),
            }
        }
        assert!(exhausted);
        runtime_support::same_raw(answers, expected);
        #[cfg(all(feature = "metrics", feature = "selective-discovery"))]
        assert_eq!(
            engine.stats().discovered_tuples,
            3 + (depth * (depth + 1) / 2) as u64
        );
        println!(
            "depth={depth} tuples={} ticks={}",
            engine.stats().discovered_tuples,
            engine.stats().ticks
        );
    }
}

#[test]
fn dense_unknown_head_patterns_keep_all_distinct_occurrence_pairs() {
    use chr_syntax::{Answer, Query, Rule, atom, c, v};
    let rules = vec![Rule::propagate(
        "pair",
        [c("p", [v(0)]), c("p", [v(1)])],
        c("seen", [v(0), v(1)]).into(),
    )];
    let constraints = (0..6)
        .map(|i| c("p", [atom(&format!("v{i}"))]))
        .collect::<Vec<_>>();
    let query = Query {
        constraints: constraints.clone(),
        outputs: vec![],
    };
    let mut residual = constraints;
    for i in 0..6 {
        for j in 0..6 {
            if i != j {
                residual.push(c("seen", [atom(&format!("v{i}")), atom(&format!("v{j}"))]));
            }
        }
    }
    let expected = vec![Answer {
        outputs: vec![],
        residual,
    }];
    runtime_support::same_raw(
        runtime_support::run(&rules, &query, 500_000),
        expected.clone(),
    );
    let mut engine = PreparedRuleset::new(rules).unwrap().start(query).unwrap();
    let mut answers = vec![];
    let mut exhausted = false;
    for _ in 0..500_000 {
        match engine.tick() {
            Event::Answer(a) => answers.push(a),
            Event::Exhausted => {
                exhausted = true;
                break;
            }
            Event::Progress => (),
        }
    }
    assert!(exhausted);
    runtime_support::same_raw(answers, expected);
    #[cfg(feature = "metrics")]
    assert_eq!(engine.stats().discovered_tuples, 30);
    println!(
        "dense tuples={} ticks={}",
        engine.stats().discovered_tuples,
        engine.stats().ticks
    );
}

#[test]
fn branching_sources_preserve_answers_while_exposing_discovery_work() {
    for family in source::FAMILIES {
        let rules = source::rules(family, 3);
        let query = source::query(family, 3, 16, 0);
        let expected = source::expected(family, 3, 16, 0);
        runtime_support::same_raw(
            runtime_support::run(&rules, &query, 500_000),
            expected.clone(),
        );
        let mut engine = PreparedRuleset::new(rules).unwrap().start(query).unwrap();
        let mut answers = vec![];
        let mut exhausted = false;
        for _ in 0..500_000 {
            match engine.tick() {
                Event::Answer(a) => answers.push(a),
                Event::Exhausted => {
                    exhausted = true;
                    break;
                }
                Event::Progress => (),
            }
        }
        assert!(exhausted);
        runtime_support::same_raw(answers, expected);
        println!(
            "family={family} choices=3 depth=16 tuples={} ticks={}",
            engine.stats().discovered_tuples,
            engine.stats().ticks
        );
    }
}
