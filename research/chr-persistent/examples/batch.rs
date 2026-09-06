#[path = "support/allocator.rs"]
mod allocator;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
use chr_persistent::{Search, Snapshot};
fn main() {
    println!(
        "case\tmode\tpass\tsteps\tapplications\tintroductions\tequations\tpairs\tdereferences\toccurs\thead_candidates\tsplits\tfailed\traw\tanswers\tmax_frontier\tpending\tmap_visits\tmap_allocations\tsnapshot_copies\tpending_allocations\tterm_requests\tterm_nodes\tobserver_pairs\tobserver_scans\tallocation_calls\trequested_bytes\tbaseline_live\tpeak_live\tfinal_live\telapsed_us"
    );
    for case in chr_cases::registry() {
        run_case(case);
    }
    for k in [0, 1, 2, 4, 8] {
        for w in [0, 1, 8, 64, 256] {
            for noise in [0, 16, 128] {
                let mut case = chr_cases::carry_case(k, w, noise);
                case.id = format!("grid-{}", case.id);
                case.budget = 1_000_000;
                run_case(case);
            }
        }
    }
}
fn run_case(case: chr_cases::Case) {
    for mode in [Snapshot::Persistent, Snapshot::Copy] {
        let before = allocator::start();
        let started = std::time::Instant::now();
        let mut search = Search::new(case.rules.clone(), case.query.clone(), mode).unwrap();
        let mut actual = vec![];
        let mut exhausted = false;
        for _ in 0..case.budget {
            let b = search.advance(1);
            actual.extend(b.answers);
            exhausted = b.exhausted;
            if exhausted || case.answer_limit.is_some_and(|n| actual.len() >= n) {
                break;
            }
        }
        let elapsed = started.elapsed().as_micros();
        let after = allocator::read();
        let pass = actual.len() == case.expected.len()
            && exhausted == case.exhausted
            && search.stats().completed == case.raw_answers
            && case.expected.iter().all(|want| {
                actual.iter().any(|got| {
                    chr_observe::equivalent(got, want, &mut chr_observe::Stats::default())
                })
            });
        let s = search.stats();
        let o = search.observation_stats();
        println!(
            "{}\t{mode:?}\t{pass}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{elapsed}",
            case.id,
            s.steps,
            s.applications,
            s.introductions,
            s.equations,
            s.pairs,
            s.dereferences,
            s.occurs_visits,
            s.head_candidates,
            s.splits,
            s.failed,
            s.completed,
            actual.len(),
            s.max_frontier,
            search.pending_alternatives(),
            s.storage.visits,
            s.storage.allocations,
            s.storage.snapshot_copies,
            s.pending_allocations,
            s.term_requests,
            s.term_nodes,
            o.term_pairs,
            o.occurrence_scans,
            after.calls - before.calls,
            after.requested - before.requested,
            before.live,
            after.peak,
            after.live
        );
        assert!(pass, "{} {mode:?}", case.id);
    }
}
