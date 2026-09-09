//! E16 semantic matrix. No timing evidence is collected by this probe.
use chr_cases::Case;
use chr_reuse::{equation_search as scalar, parallel_equations as parallel};
use chr_syntax::Answer;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq, Eq)]
struct Trace {
    steps: u64,
    introductions: u64,
    applications: u64,
    equations: u64,
    splits: u64,
    failed: u64,
    completed: u64,
}
impl Trace {
    fn read(s: &chr_persistent::Stats) -> Self {
        Self {
            steps: s.steps,
            introductions: s.introductions,
            applications: s.applications,
            equations: s.equations,
            splits: s.splits,
            failed: s.failed,
            completed: s.completed,
        }
    }
}

struct Observation {
    answers: Vec<Answer>,
    trace: Vec<Trace>,
    exhausted: bool,
}

fn equivalent(a: &Answer, b: &Answer) -> bool {
    chr_observe::equivalent(a, b, &mut Default::default())
}
fn same_set(a: &[Answer], b: &[Answer]) {
    assert_eq!(a.len(), b.len());
    assert!(a.iter().all(|a| b.iter().any(|b| equivalent(a, b))));
    assert!(b.iter().all(|b| a.iter().any(|a| equivalent(a, b))));
}
fn stop(case: &Case, o: &Observation) -> bool {
    o.exhausted || case.answer_limit.is_some_and(|n| o.answers.len() >= n)
}
fn validate(
    case: &Case,
    o: &Observation,
    reference: &(Vec<Answer>, u64),
    owned: Option<&Observation>,
) {
    assert_eq!(o.exhausted, case.exhausted, "{} exhaustion", case.id);
    assert_eq!(
        o.trace.last().unwrap().completed,
        case.raw_answers,
        "{} raw",
        case.id
    );
    same_set(&o.answers, &case.expected);
    same_set(&o.answers, &reference.0);
    assert_eq!(
        o.trace.last().unwrap().failed,
        reference.1,
        "{} reference failures",
        case.id
    );
    if let Some(owned) = owned {
        assert_eq!(o.trace, owned.trace, "{} committed trace", case.id);
        assert_eq!(o.answers.len(), owned.answers.len());
        assert!(
            o.answers
                .iter()
                .zip(&owned.answers)
                .all(|(a, b)| equivalent(a, b)),
            "{} FIFO answer sequence",
            case.id
        );
    }
}

fn reference(case: &Case) -> (Vec<Answer>, u64) {
    let mut search = chr_reference::Search::new(case.rules.clone(), case.query.clone()).unwrap();
    let mut answers = Vec::new();
    let mut exhausted = false;
    for _ in 0..case.budget {
        let batch = search.advance(1);
        answers.extend(batch.answers);
        exhausted = batch.exhausted;
        if exhausted || case.answer_limit.is_some_and(|n| answers.len() >= n) {
            break;
        }
    }
    assert_eq!(exhausted, case.exhausted);
    assert_eq!(search.stats().completed_branches, case.raw_answers);
    same_set(&answers, &case.expected);
    (answers, search.stats().failed_branches)
}

fn scalar(
    case: &Case,
    mode: scalar::Mode,
    reference: &(Vec<Answer>, u64),
    owned: Option<&Observation>,
) -> Observation {
    let mut search = scalar::Search::new(case.rules.clone(), case.query.clone(), mode).unwrap();
    let mut o = Observation {
        answers: Vec::new(),
        trace: Vec::new(),
        exhausted: false,
    };
    for _ in 0..case.budget {
        let batch = search.advance(1);
        o.answers.extend(batch.answers);
        o.exhausted = batch.exhausted;
        o.trace.push(Trace::read(search.source_stats()));
        if stop(case, &o) {
            break;
        }
    }
    validate(case, &o, reference, owned);
    let c = search.stats();
    let p = search.operation_stats();
    println!(
        "{}\t{mode:?}\t0\tpass\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t0\t0\t0\t0\t0\t{}\t{}\t{}\t{}\t[]",
        case.id,
        c.steps,
        c.completed,
        c.failed,
        o.answers.len(),
        o.exhausted,
        c.projected_equations,
        c.projected_equations,
        p.calls,
        p.pairs,
        p.resolve_nodes,
        p.occurs_nodes
    );
    o
}

fn parallel(
    case: &Case,
    mode: parallel::Mode,
    limit: usize,
    reference: &(Vec<Answer>, u64),
    owned: &Observation,
) {
    // This diagnostic hook records actual worker assignment/completion boundaries.
    // It is not enabled in a timing comparison.
    let completion = Arc::new(Mutex::new(Vec::new()));
    let mut search = match mode {
        parallel::Mode::Inline => {
            parallel::Search::new(case.rules.clone(), case.query.clone(), mode, limit, 8).unwrap()
        }
        parallel::Mode::Threads(_) => {
            let log = Arc::clone(&completion);
            let hook = Arc::new(move |worker, request, phase| {
                if matches!(phase, parallel::WorkerPhase::AfterSolve) {
                    log.lock().unwrap().push((worker, request));
                }
            });
            parallel::Search::new_with_worker_hook(
                case.rules.clone(),
                case.query.clone(),
                mode,
                limit,
                8,
                hook,
            )
            .unwrap()
        }
    };
    let mut o = Observation {
        answers: Vec::new(),
        trace: Vec::new(),
        exhausted: false,
    };
    for _ in 0..case.budget {
        let batch = search.advance(1).unwrap();
        o.answers.extend(batch.answers);
        o.exhausted = batch.exhausted;
        o.trace.push(Trace::read(search.source_stats()));
        if stop(case, &o) {
            break;
        }
    }
    let before = Trace::read(search.source_stats());
    search.shutdown().unwrap();
    assert_eq!(
        before,
        Trace::read(search.source_stats()),
        "shutdown committed source work"
    );
    validate(case, &o, reference, Some(owned));
    let c = search.stats();
    let p = search.operation_stats();
    assert_eq!(c.issued, c.received);
    assert_eq!(c.issued, p.calls);
    assert_eq!(c.outstanding, 0);
    assert_eq!(c.buffered, 0);
    assert!(c.max_outstanding <= limit && c.max_buffered <= limit);
    assert_eq!(
        c.issued - c.committed_equations,
        c.uncommitted_at_shutdown as u64
    );
    let completion = completion.lock().unwrap();
    if matches!(mode, parallel::Mode::Threads(_)) {
        assert_eq!(completion.len() as u64, p.calls);
        let mut ids = completion.iter().map(|(_, id)| *id).collect::<Vec<_>>();
        ids.sort();
        assert_eq!(ids, (0..c.issued).collect::<Vec<_>>());
    }
    println!(
        "{}\t{mode:?}\t{limit}\tpass\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{completion:?}",
        case.id,
        c.steps,
        c.completed,
        c.failed,
        o.answers.len(),
        o.exhausted,
        c.issued,
        c.committed_equations,
        c.lookahead_visits,
        c.projection_dereferences,
        c.max_outstanding,
        c.max_buffered,
        c.uncommitted_at_shutdown,
        p.calls,
        p.pairs,
        p.resolve_nodes,
        p.occurs_nodes
    );
}

fn main() {
    let cases = chr_cases::registry();
    assert_eq!(cases.len(), 64);
    println!(
        "case\tmode\tlimit\tstatus\tsteps\traw\tfailed\tanswers\texhausted\tissued\tcommitted_equations\tlookahead_visits\tprojection_dereferences\tmax_outstanding\towner_buffered_peak\tuncommitted_at_shutdown\tcalls\tpairs\tresolve_nodes\toccurs_nodes\tworker_completion"
    );
    for case in cases {
        let reference = reference(&case);
        let owned = scalar(&case, scalar::Mode::Owned, &reference, None);
        scalar(&case, scalar::Mode::Shared, &reference, Some(&owned));
        for mode in [
            parallel::Mode::Inline,
            parallel::Mode::Threads(1),
            parallel::Mode::Threads(2),
        ] {
            for limit in [1, 4] {
                parallel(&case, mode, limit, &reference, &owned);
            }
        }
    }
}
