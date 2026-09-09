#[path = "support/allocator.rs"]
mod allocator;
#[path = "support/failure_cases.rs"]
mod cases;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
use chr_reuse::failure_search::{Mode, Search};
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--list"] {
        for c in cases::cases() {
            println!("{}", c.id);
        }
        return;
    }
    assert_eq!(args.len(), 2);
    let mode = match args[1].as_str() {
        "Direct" => Mode::Direct,
        "Learn" => Mode::Learn,
        "Native" => Mode::Native,
        _ => panic!("unknown mode"),
    };
    let case = cases::cases()
        .into_iter()
        .find(|c| c.id == args[0])
        .expect("registered case");
    let before = allocator::start();
    let start = std::time::Instant::now();
    let mut search = Search::new(case.rules.clone(), case.query.clone(), mode).unwrap();
    let mut answers = vec![];
    let mut exhausted = false;
    for _ in 0..case.budget {
        let b = search.advance(1);
        answers.extend(b.answers);
        exhausted = b.exhausted;
        if exhausted || case.answer_limit.is_some_and(|n| answers.len() >= n) {
            break;
        }
    }
    let elapsed = start.elapsed().as_micros();
    let after = allocator::read();
    let pass = exhausted == case.exhausted
        && answers.len() == case.expected.len()
        && search.stats().completed == case.raw_answers
        && case.expected.iter().all(|e| {
            answers
                .iter()
                .any(|a| chr_observe::equivalent(a, e, &mut chr_observe::Stats::default()))
        });
    let s = search.stats();
    let c = search.source_stats();
    let p = search.proof_stats();
    println!(
        "case\tmode\tpass\tlogical_steps\texecuted\tprojected_equations\thead_reads\tproof_checks\tcheck_nodes\tdiscovery_nodes\tlearned\thits\traw\tfailed\tanswers\tmax_frontier\tapplications\tpairs\toccurs\tallocation_calls\trequested_bytes\tbaseline_live\tpeak_live\tfinal_live\telapsed_us"
    );
    println!(
        "{}\t{mode:?}\t{pass}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{elapsed}",
        case.id,
        s.logical_steps,
        s.executed,
        s.projected_equations,
        s.head_reads,
        p.checks,
        p.check_nodes,
        p.discovery_nodes,
        p.learned,
        p.hits,
        s.completed,
        s.failed,
        answers.len(),
        s.max_frontier,
        c.applications,
        c.pairs,
        c.occurs_visits,
        after.calls - before.calls,
        after.requested - before.requested,
        before.live,
        after.peak,
        after.live
    );
    assert!(pass, "{} {mode:?}", case.id);
}
