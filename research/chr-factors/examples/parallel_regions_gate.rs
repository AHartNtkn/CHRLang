//! E16 regional semantic gate; elapsed time is not comparative evidence.
#[path = "support/factor_cases.rs"]
mod cases;
use chr_cases::Case;
use chr_factors::{Mode as BaselineMode, Search as Baseline, parallel_regions as parallel};
use chr_syntax::Answer;

#[derive(Debug, PartialEq, Eq)]
struct Trace {
    steps: u64,
    source_turns: u64,
    applications: u64,
    products: u64,
    jobs: u64,
    duplicates: u64,
    renamed: u64,
    refutations: u64,
    max_jobs: usize,
    answers: usize,
    exhausted: bool,
    raw: Option<u128>,
}
fn trace(
    s: &chr_factors::Stats,
    applications: u64,
    answers: usize,
    exhausted: bool,
    raw: Option<u128>,
) -> Trace {
    Trace {
        steps: s.steps,
        source_turns: s.source_steps,
        applications,
        products: s.products,
        jobs: s.product_jobs,
        duplicates: s.duplicates,
        renamed: s.renamed_nodes,
        refutations: s.empty_refutations,
        max_jobs: s.max_jobs,
        answers,
        exhausted,
        raw,
    }
}
struct Observation {
    answers: Vec<Answer>,
    trace: Vec<Trace>,
}
fn equivalent(a: &Answer, b: &Answer) -> bool {
    chr_observe::equivalent(a, b, &mut Default::default())
}
fn validate(case: &Case, result: &Observation, control: Option<&Observation>) {
    let last = result.trace.last().unwrap();
    assert!(last.exhausted, "{} exhausted", case.id);
    assert_eq!(last.raw, Some(case.raw_answers as u128), "{} raw", case.id);
    assert_eq!(
        result.answers.len(),
        case.expected.len(),
        "{} answer count",
        case.id
    );
    assert!(
        case.expected
            .iter()
            .all(|a| result.answers.iter().any(|b| equivalent(a, b)))
    );
    assert!(
        result
            .answers
            .iter()
            .all(|a| case.expected.iter().any(|b| equivalent(a, b)))
    );
    if let Some(control) = control {
        assert_eq!(result.trace, control.trace, "{} per-turn control", case.id);
        assert!(
            result
                .answers
                .iter()
                .zip(&control.answers)
                .all(|(a, b)| equivalent(a, b)),
            "{} ordered answers",
            case.id
        );
    }
}
fn reference(case: &Case) {
    let mut search = chr_reference::Search::new(case.rules.clone(), case.query.clone()).unwrap();
    let result = search.advance(case.budget);
    assert!(result.exhausted, "{} reference exhausted", case.id);
    assert_eq!(search.stats().completed_branches, case.raw_answers);
    assert_eq!(result.answers.len(), case.expected.len());
    assert!(
        case.expected
            .iter()
            .all(|e| result.answers.iter().any(|a| equivalent(a, e)))
    );
}
fn report(
    case: &Case,
    mode: &str,
    q: usize,
    k: usize,
    factors: usize,
    result: &Observation,
    transport: &parallel::TransportStats,
) {
    let s = result.trace.last().unwrap();
    print!(
        "{}\t{mode}\t{q}\t{k}\tpass\t{factors}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        case.id,
        result.answers.len(),
        s.raw.unwrap(),
        s.steps,
        s.source_turns,
        s.applications,
        s.products,
        s.jobs,
        s.duplicates,
        s.refutations
    );
    println!(
        "\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        transport.issued,
        transport.received,
        transport.accepted,
        transport.max_outstanding,
        transport.owner_buffered_peak,
        transport.outstanding,
        transport.buffered,
        transport.unaccepted_at_shutdown,
        transport.actual_source_steps,
        transport.accepted_source_steps,
        transport.cancelled_requests,
        transport.unserved_quantum_slots,
        transport.prefetch_visits
    );
}
fn baseline(case: &Case) -> Observation {
    let mut search = Baseline::new(
        case.rules.clone(),
        case.query.clone(),
        BaselineMode::Factored,
    )
    .unwrap();
    let mut result = Observation {
        answers: vec![],
        trace: vec![],
    };
    for _ in 0..case.budget * 8 {
        let batch = search.advance(1);
        result.trace.push(trace(
            search.stats(),
            search.source_applications(),
            batch.answers.len(),
            batch.exhausted,
            search.raw_count(),
        ));
        result.answers.extend(batch.answers);
        if batch.exhausted {
            break;
        }
    }
    validate(case, &result, None);
    report(
        case,
        "Baseline",
        1,
        0,
        search.factor_count(),
        &result,
        &parallel::TransportStats {
            actual_source_steps: search.source_stats().iter().map(|s| s.steps).sum(),
            accepted_source_steps: search.source_stats().iter().map(|s| s.steps).sum(),
            ..Default::default()
        },
    );
    result
}
fn candidate(case: &Case, mode: parallel::Mode, q: usize, control: &Observation) -> Observation {
    let mut search =
        parallel::Search::new(case.rules.clone(), case.query.clone(), mode, q, 4).unwrap();
    let mut result = Observation {
        answers: vec![],
        trace: vec![],
    };
    for _ in 0..case.budget * 8 {
        let batch = search.advance(1).unwrap();
        result.trace.push(trace(
            search.stats(),
            search.source_applications(),
            batch.answers.len(),
            batch.exhausted,
            search.raw_count(),
        ));
        result.answers.extend(batch.answers);
        if batch.exhausted {
            break;
        }
    }
    // Q8's inline control has its own explicit owner schedule.
    let comparison = if q == 8 && matches!(mode, parallel::Mode::Inline) {
        None
    } else {
        Some(control)
    };
    validate(case, &result, comparison);
    let before = trace(
        search.stats(),
        search.source_applications(),
        0,
        true,
        search.raw_count(),
    );
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
        search.source_stats().iter().map(|s| s.steps).sum::<u64>()
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
    assert_eq!(
        before,
        trace(
            search.stats(),
            search.source_applications(),
            0,
            true,
            search.raw_count()
        )
    );
    report(
        case,
        &format!("{mode:?}"),
        q,
        4,
        search.factor_count(),
        &result,
        search.transport_stats(),
    );
    result
}
fn main() {
    let names = cases::names();
    assert_eq!(names.len(), 103);
    let e00 = chr_cases::registry()
        .into_iter()
        .map(|c| c.id)
        .collect::<std::collections::BTreeSet<_>>();
    println!(
        "case\tmode\tquantum\tlimit\tstatus\tfactors\tanswers\traw_products\towner_turns\tsource_turns\tapplications\tproducts\tproduct_jobs\tduplicates\trefutations\tissued\treceived\taccepted\tmax_outstanding\towner_buffered_peak\toutstanding\tbuffered\tunaccepted_at_shutdown\tactual_source_steps\taccepted_source_steps\tcancelled_requests\tunserved_quantum_slots\tprefetch_visits"
    );
    for id in names {
        let case = cases::input(&id);
        assert!(case.exhausted);
        if e00.contains(&id) {
            reference(&case);
        }
        let original = baseline(&case);
        for mode in [
            parallel::Mode::Inline,
            parallel::Mode::Threads(1),
            parallel::Mode::Threads(2),
        ] {
            candidate(&case, mode, 1, &original);
        }
        let coarse = candidate(&case, parallel::Mode::Inline, 8, &original);
        for mode in [parallel::Mode::Threads(1), parallel::Mode::Threads(2)] {
            candidate(&case, mode, 8, &coarse);
        }
    }
}
