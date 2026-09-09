use chr_reuse::continuations::{Mode, Search};
use chr_syntax::{Goal, Query, Rule, and, c, eq, or, t, v};

#[test]
fn different_histories_reuse_future_work_without_merging_raw_answers() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(
                and([Goal::True, c("resume", [v(0), v(91)]).into()]),
                c("resume", [v(0), v(92)]).into(),
            ),
        ),
        Rule::simplify("trash", [c("trash", [v(0)])], Goal::True),
        Rule::simplify(
            "resume",
            [c("resume", [v(0), v(1)])],
            eq(v(0), t("pair", [v(1), v(1)])),
        ),
    ];
    let query = Query {
        constraints: vec![c("start", [v(7)])],
        outputs: vec![("result".into(), chr_syntax::Var(7))],
    };
    let mut reference = chr_reference::Search::new(rules.clone(), query.clone()).unwrap();
    let expected = reference.advance(1000);
    assert!(expected.exhausted);
    assert_eq!(reference.stats().completed_branches, 2);
    let mut executed = vec![];
    for mode in [Mode::Direct, Mode::ExactIds, Mode::Alpha, Mode::AlphaLive] {
        let mut run = Search::new(rules.clone(), query.clone(), mode).unwrap();
        let batch = run.advance(1000);
        assert!(batch.exhausted);
        assert_eq!(batch.raw_answers.len(), 2);
        for answer in &batch.raw_answers {
            assert!(chr_observe::equivalent(
                answer,
                &expected.answers[0],
                &mut Default::default()
            ));
        }
        assert_eq!(batch.answers.len(), 1);
        executed.push(run.stats().executed);
        if matches!(mode, Mode::Alpha) {
            assert!(run.stats().hits > 0);
        }
    }
    assert!(executed[2] < executed[1], "alpha {executed:?}");
    assert!(executed[3] < executed[1]);
    println!("renaming executed Direct/Exact/Alpha/AlphaLive: {executed:?}");
}

#[test]
fn consumed_history_remains_distinct_without_relevance_projection() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(
                and([
                    c("trash", [v(90)]).into(),
                    c("resume", [v(0), v(91)]).into(),
                ]),
                c("resume", [v(0), v(92)]).into(),
            ),
        ),
        Rule::simplify("trash", [c("trash", [v(0)])], Goal::True),
        Rule::simplify(
            "resume",
            [c("resume", [v(0), v(1)])],
            eq(v(0), t("pair", [v(1), v(1)])),
        ),
    ];
    let query = Query {
        constraints: vec![c("start", [v(7)])],
        outputs: vec![("result".into(), chr_syntax::Var(7))],
    };
    let mut reference = chr_reference::Search::new(rules.clone(), query.clone()).unwrap();
    let expected = reference.advance(1000);
    assert!(expected.exhausted);
    assert_eq!(reference.stats().completed_branches, 2);
    let mut executed = vec![];
    for mode in [Mode::Direct, Mode::ExactIds, Mode::Alpha, Mode::AlphaLive] {
        let mut run = Search::new(rules.clone(), query.clone(), mode).unwrap();
        let batch = run.advance(1000);
        assert!(batch.exhausted);
        assert_eq!(batch.raw_answers.len(), 2);
        for answer in &batch.raw_answers {
            assert!(chr_observe::equivalent(
                answer,
                &expected.answers[0],
                &mut Default::default()
            ));
        }
        assert_eq!(batch.answers.len(), 1);
        executed.push(run.stats().executed);
        if matches!(mode, Mode::Alpha) {
            assert_eq!(run.stats().hits, 0);
        }
    }
    assert_eq!(executed[2], executed[1]);
    assert!(executed[3] < executed[2]);
    println!("history projection executed Direct/Exact/Alpha/AlphaLive: {executed:?}");
}

#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;

#[test]
fn aliases_resources_history_and_fresh_replay_match_independent_raw_semantics() {
    use chr_syntax::atom;
    for offset in [0, 100, 9000] {
        for reverse in [false, true] {
            let variants = vec![
                c("finish", [v(0), v(1), v(1)]).into(),
                c("finish", [v(0), v(1), v(2)]).into(),
                and([c("token", []).into(), c("claim", [v(0)]).into()]),
                c("claim", [v(0)]).into(),
                and([c("stay", [v(1)]).into(), c("stay", [v(1)]).into()]),
                c("stay", [v(1)]).into(),
                and([c("signal", [v(0)]).into(), Goal::True]),
                and([eq(v(0), atom("bad")), c("signal", [v(0)]).into()]),
            ];
            for a in &variants {
                for b in &variants {
                    let (a, b) = if reverse { (b, a) } else { (a, b) };
                    let rules = vec![
                        Rule::simplify("start", [c("start", [v(0)])], or(a.clone(), b.clone())),
                        Rule::simplify(
                            "finish",
                            [c("finish", [v(0), v(1), v(2)])],
                            eq(v(0), t("triple", [v(1), v(2), v(99)])),
                        ),
                        Rule::simplify(
                            "claim",
                            [c("claim", [v(0)]), c("token", [])],
                            eq(v(0), atom("claimed")),
                        ),
                        Rule::propagate("signal", [c("signal", [v(0)])], eq(v(0), atom("good"))),
                    ];
                    let query = Query {
                        constraints: vec![c("start", [v(offset)])],
                        outputs: vec![("result".into(), chr_syntax::Var(offset))],
                    };
                    let expected = oracle::run(&rules, &query, 1000);
                    for mode in [Mode::Direct, Mode::ExactIds, Mode::Alpha, Mode::AlphaLive] {
                        let mut search = Search::new(rules.clone(), query.clone(), mode).unwrap();
                        let actual = search.advance(1000);
                        assert!(actual.exhausted);
                        assert_eq!(search.stats().completed as usize, expected.len());
                        oracle::same_raw(actual.raw_answers, expected.clone());
                    }
                }
            }
        }
    }
}
