#[path = "../examples/support/region_cost_cases.rs"]
mod cases;
use chr_factors::parallel_regions::{Mode, Search};
use chr_syntax::{Answer, Term};
fn same(a: &Answer, b: &Answer) -> bool {
    chr_observe::equivalent(a, b, &mut Default::default())
}
fn check(case: &chr_cases::Case, answers: &[Answer], exhausted: bool, raw: Option<u128>) {
    assert_eq!(exhausted, case.exhausted);
    if case.id == "stream-prefix" {
        assert_eq!(raw, None);
        assert_eq!(answers.len(), 8);
        for (i, answer) in answers.iter().enumerate() {
            assert!(answer.residual.is_empty());
            assert_eq!(answer.outputs.len(), 2);
            assert_eq!(answer.outputs[0].0, case.query.outputs[0].0);
            assert_eq!(answer.outputs[1].0, case.query.outputs[1].0);
            let mut n = &answer.outputs[0].1;
            while let Term::App(name, args) = n {
                if name != "s" || args.len() != 1 {
                    break;
                }
                n = &args[0];
            }
            assert_eq!(*n, chr_syntax::atom("z"));
            assert!([chr_syntax::atom("a"), chr_syntax::atom("b")].contains(&answer.outputs[1].1));
            assert!(!answers[..i].iter().any(|a| same(a, answer)));
        }
    } else {
        assert_eq!(raw, Some(case.raw_answers as u128));
        assert_eq!(answers.len(), case.expected.len());
        assert!(
            case.expected
                .iter()
                .all(|e| answers.iter().any(|a| same(a, e)))
        );
    }
}
fn run(case: &chr_cases::Case, mode: Mode, q: usize, k: usize) -> Vec<Answer> {
    let mut search = Search::new(case.rules.clone(), case.query.clone(), mode, q, k).unwrap();
    let mut answers = vec![];
    for _ in 0..case.budget {
        let batch = search.advance(1).unwrap();
        answers.extend(batch.answers);
        if batch.exhausted || case.answer_limit.is_some_and(|n| answers.len() >= n) {
            check(case, &answers, batch.exhausted, search.raw_count());
            let accepted = search.source_stats().map(|s| s.steps).sum::<u64>();
            search.shutdown().unwrap();
            let stats = search.transport_stats();
            assert_eq!(stats.issued, stats.received);
            assert_eq!(
                stats.issued - stats.accepted,
                stats.unaccepted_at_shutdown as u64
            );
            assert_eq!(stats.outstanding, 0);
            assert_eq!(stats.buffered, 0);
            assert_eq!(stats.accepted_source_steps, accepted);
            assert!(stats.actual_source_steps >= accepted);
            assert_eq!(
                stats.actual_source_steps,
                search
                    .actual_source_stats()
                    .iter()
                    .map(|s| s.steps)
                    .sum::<u64>()
            );
            return answers;
        }
    }
    panic!("{} did not reach its registered endpoint", case.id);
}
#[test]
fn regional_cost_workloads_preserve_finite_contracts_and_scheduled_prefixes() {
    for id in cases::ids() {
        let case = cases::case(&id);
        let mut baseline = chr_factors::Search::new(
            case.rules.clone(),
            case.query.clone(),
            chr_factors::Mode::Factored,
        )
        .unwrap();
        let mut answers = vec![];
        for _ in 0..case.budget {
            let batch = baseline.advance(1);
            answers.extend(batch.answers);
            if batch.exhausted || case.answer_limit.is_some_and(|n| answers.len() >= n) {
                break;
            }
        }
        check(&case, &answers, baseline.exhausted(), baseline.raw_count());
        for q in [1, 8, 64] {
            if q == 64 && id != "one-work" && id != "two-work" {
                continue;
            }
            let inline = run(&case, Mode::Inline, q, 4);
            if q == 1 {
                assert_eq!(answers.len(), inline.len());
                assert!(answers.iter().zip(&inline).all(|(a, b)| same(a, b)));
            }
            for mode in [Mode::Threads(1), Mode::Threads(2)] {
                let workers = run(&case, mode, q, 4);
                assert_eq!(inline.len(), workers.len());
                assert!(inline.iter().zip(&workers).all(|(a, b)| same(a, b)));
            }
            if id == "two-work" && q == 8 {
                for mode in [Mode::Inline, Mode::Threads(1), Mode::Threads(2)] {
                    let limited = run(&case, mode, q, 1);
                    assert_eq!(inline.len(), limited.len());
                    assert!(inline.iter().zip(&limited).all(|(a, b)| same(a, b)));
                }
            }
        }
    }
}
