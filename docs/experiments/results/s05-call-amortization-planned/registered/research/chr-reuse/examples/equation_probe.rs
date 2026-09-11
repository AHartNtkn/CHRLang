#[path = "support/allocator.rs"]
mod allocator;
#[path = "support/equation_cases.rs"]
mod cases;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
use chr_reuse::equation_search::{Mode, Search};
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
        "Shared" => Mode::Shared,
        "Owned" => Mode::Owned,
        "Memo" => Mode::Memo,
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
    let p = search.operation_stats();
    println!(
        "case\tmode\tpass\tsteps\traw\tfailed\tanswers\tmax_frontier\tprojected_equations\tcalls\tcomputed\thits\tkey_nodes\treplay_nodes\tcache_entries\towned_pairs\towned_resolve\towned_occurs\tshared_pairs\tshared_occurs\tterm_requests\tterm_nodes\tmap_visits\tallocation_calls\trequested_bytes\tbaseline_live\tpeak_live\tfinal_live\telapsed_us"
    );
    println!(
        "{}\t{mode:?}\t{pass}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{elapsed}",
        case.id,
        s.steps,
        s.completed,
        s.failed,
        answers.len(),
        s.max_frontier,
        s.projected_equations,
        p.calls,
        p.computed,
        p.hits,
        p.key_nodes,
        p.replay_nodes,
        p.cache_entries,
        p.pairs,
        p.resolve_nodes,
        p.occurs_nodes,
        c.pairs,
        c.occurs_visits,
        c.term_requests,
        c.term_nodes,
        c.storage.visits,
        after.calls - before.calls,
        after.requested - before.requested,
        before.live,
        after.peak,
        after.live
    );
    assert!(pass, "{} {mode:?}", case.id);
}
