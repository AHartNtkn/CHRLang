use chr_factors::parallel_regions::{Mode, Search};
use chr_factors::{Mode as BaselineMode, Search as Baseline};
use chr_syntax::{Answer, Rule, atom, c, eq, or, v};

fn rules() -> Vec<Rule> {
    ["p", "q"]
        .into_iter()
        .map(|name| {
            Rule::simplify(
                name,
                [c(name, [v(0)])],
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
            )
        })
        .collect()
}
fn same_sequence(a: &[Answer], b: &[Answer]) {
    assert_eq!(a.len(), b.len());
    assert!(
        a.iter()
            .zip(b)
            .all(|(a, b)| chr_observe::equivalent(a, b, &mut Default::default()))
    );
}
fn shutdown_checked(search: &mut Search) {
    let steps = search.stats().steps;
    search.shutdown().unwrap();
    let transport = search.transport_stats();
    assert_eq!(transport.issued, transport.received);
    assert_eq!(
        transport.issued - transport.accepted,
        transport.unaccepted_at_shutdown as u64
    );
    assert_eq!(transport.outstanding, 0);
    assert_eq!(transport.buffered, 0);
    assert_eq!(
        transport.accepted_source_steps,
        search.source_stats().map(|s| s.steps).sum::<u64>()
    );
    assert_eq!(
        transport.actual_source_steps,
        search
            .actual_source_stats()
            .iter()
            .map(|s| s.steps)
            .sum::<u64>()
    );
    assert!(transport.actual_source_steps >= transport.accepted_source_steps);
    assert_eq!(search.stats().steps, steps);
}
#[test]
fn one_step_regions_preserve_baseline_product_turns() {
    let query = chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]);
    for mode in [Mode::Inline, Mode::Threads(1), Mode::Threads(2)] {
        let mut baseline = Baseline::new(rules(), query.clone(), BaselineMode::Factored).unwrap();
        let mut candidate = Search::new(rules(), query.clone(), mode, 1, 4).unwrap();
        assert_eq!(candidate.factor_count(), 2);
        for _ in 0..100 {
            let a = baseline.advance(1);
            let b = candidate.advance(1).unwrap();
            same_sequence(&a.answers, &b.answers);
            assert_eq!(a.exhausted, b.exhausted);
            assert_eq!(baseline.stats().steps, candidate.stats().steps);
            assert_eq!(baseline.stats().products, candidate.stats().products);
            assert_eq!(
                baseline.stats().product_jobs,
                candidate.stats().product_jobs
            );
            assert_eq!(
                baseline.source_applications(),
                candidate.source_applications()
            );
            if a.exhausted {
                break;
            }
        }
        assert_eq!(candidate.raw_count(), Some(4));
        let steps = candidate.stats().steps;
        shutdown_checked(&mut candidate);
        assert_eq!(candidate.stats().steps, steps);
        assert!(candidate.advance(1).is_err());
    }
}
#[test]
fn coarse_workers_match_their_inline_product_schedule() {
    let query = chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]);
    for mode in [Mode::Threads(1), Mode::Threads(2)] {
        let mut inline = Search::new(rules(), query.clone(), Mode::Inline, 8, 4).unwrap();
        let mut workers = Search::new(rules(), query.clone(), mode, 8, 4).unwrap();
        for _ in 0..100 {
            let a = inline.advance(1).unwrap();
            let b = workers.advance(1).unwrap();
            same_sequence(&a.answers, &b.answers);
            assert_eq!(a.exhausted, b.exhausted);
            if a.exhausted {
                break;
            }
        }
        assert_eq!(workers.raw_count(), Some(4));
        shutdown_checked(&mut inline);
        shutdown_checked(&mut workers);
    }
}

#[test]
fn regional_freshness_and_output_aliases_survive_workers() {
    let query = chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[3, 3]);
    let expected = chr_cases::answer(vec![v(2), v(2)], vec![c("p", [v(0)]), c("q", [v(1)])]);
    for q in [1, 8] {
        for mode in [Mode::Inline, Mode::Threads(1), Mode::Threads(2)] {
            let mut search = Search::new(vec![], query.clone(), mode, q, 4).unwrap();
            assert_eq!(search.factor_count(), 3);
            let result = search.advance(100).unwrap();
            assert!(result.exhausted);
            assert_eq!(result.answers.len(), 1);
            assert!(chr_observe::equivalent(
                &result.answers[0],
                &expected,
                &mut Default::default()
            ));
            shutdown_checked(&mut search);
        }
    }
}

#[test]
fn certificate_keeps_shared_variables_and_multiheads_together() {
    for q in [1, 8] {
        for mode in [Mode::Inline, Mode::Threads(1), Mode::Threads(2)] {
            let query = chr_cases::query(vec![c("p", [v(0)]), c("q", [v(0)])], &[0]);
            let mut search = Search::new(rules(), query, mode, q, 4).unwrap();
            assert_eq!(search.factor_count(), 1);
            let result = search.advance(100).unwrap();
            assert!(result.exhausted);
            assert_eq!(result.answers.len(), 2);
            shutdown_checked(&mut search);
            let mut linked = rules();
            linked.push(Rule::simplify(
                "joint",
                [c("p", [v(0)]), c("q", [v(1)])],
                chr_syntax::Goal::True,
            ));
            let query = chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]);
            let mut search = Search::new(linked, query, mode, q, 4).unwrap();
            assert_eq!(search.factor_count(), 1);
            shutdown_checked(&mut search);
        }
    }
}

#[test]
fn empty_region_refutes_a_loop_but_unfinished_is_not_empty() {
    let rules = vec![
        Rule::simplify("loop", [c("loop", [v(0)])], c("loop", [v(0)]).into()),
        Rule::simplify("bad", [c("bad", [])], chr_syntax::Goal::Fail),
    ];
    for q in [1, 8] {
        for mode in [Mode::Inline, Mode::Threads(1), Mode::Threads(2)] {
            let query = chr_cases::query(vec![c("loop", [v(0)]), c("bad", [])], &[]);
            let mut search = Search::new(rules.clone(), query, mode, q, 4).unwrap();
            let result = search.advance(100).unwrap();
            assert!(result.exhausted);
            assert!(result.answers.is_empty());
            assert_eq!(search.raw_count(), Some(0));
            shutdown_checked(&mut search);
            let query = chr_cases::query(vec![c("loop", [v(0)])], &[]);
            let mut search = Search::new(rules.clone(), query, mode, q, 4).unwrap();
            let result = search.advance(10).unwrap();
            assert!(!result.exhausted);
            assert!(result.answers.is_empty());
            assert_eq!(search.raw_count(), None);
            shutdown_checked(&mut search);
        }
    }
}

#[test]
fn duplicate_regional_answers_keep_raw_product_multiplicity() {
    let rules = ["p", "q"]
        .into_iter()
        .map(|name| {
            Rule::simplify(
                name,
                [c(name, [v(0)])],
                or(eq(v(0), atom("a")), eq(v(0), atom("a"))),
            )
        })
        .collect::<Vec<_>>();
    for q in [1, 8] {
        for mode in [Mode::Inline, Mode::Threads(1), Mode::Threads(2)] {
            let query = chr_cases::query(vec![c("p", [v(0)]), c("q", [v(1)])], &[0, 1]);
            let mut search = Search::new(rules.clone(), query, mode, q, 4).unwrap();
            let result = search.advance(100).unwrap();
            assert!(result.exhausted);
            assert_eq!(result.answers.len(), 1);
            assert_eq!(search.raw_count(), Some(4));
            shutdown_checked(&mut search);
        }
    }
}

#[test]
fn infinite_product_stream_matches_same_quantum_prefix() {
    use chr_syntax::{and, t};
    let rules = vec![
        Rule::simplify(
            "nums",
            [c("nums", [v(0)])],
            or(
                eq(v(0), atom("z")),
                and([eq(v(0), t("s", [v(1)])), c("nums", [v(1)]).into()]),
            ),
        ),
        Rule::simplify(
            "pick",
            [c("pick", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
    ];
    let query = chr_cases::query(vec![c("nums", [v(0)]), c("pick", [v(1)])], &[0, 1]);
    for q in [1, 8] {
        let mut inline = Search::new(rules.clone(), query.clone(), Mode::Inline, q, 4).unwrap();
        let expected = inline.advance(500).unwrap();
        assert!(!expected.exhausted);
        for n in 0..4 {
            for letter in ["a", "b"] {
                let numeral = (0..n).fold(atom("z"), |x, _| t("s", [x]));
                let answer = chr_cases::answer(vec![numeral, atom(letter)], vec![]);
                assert!(expected.answers.iter().any(|a| chr_observe::equivalent(
                    a,
                    &answer,
                    &mut Default::default()
                )));
            }
        }
        shutdown_checked(&mut inline);
        for mode in [Mode::Threads(1), Mode::Threads(2)] {
            let mut search = Search::new(rules.clone(), query.clone(), mode, q, 4).unwrap();
            let result = search.advance(500).unwrap();
            assert!(!result.exhausted);
            same_sequence(&result.answers, &expected.answers);
            assert_eq!(search.raw_count(), None);
            let steps = search.stats().steps;
            shutdown_checked(&mut search);
            assert_eq!(search.stats().steps, steps);
        }
    }
}
