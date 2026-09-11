#[path = "support/allocator.rs"]
mod allocator;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
use chr_persistent::{Search, Snapshot};
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--list"] {
        for case in chr_cases::registry() {
            println!("{}", case.id);
        }
        for k in [0, 1, 2, 4, 8] {
            for w in [0, 1, 8, 64, 256] {
                for noise in [0, 16, 128] {
                    println!("grid-carry-k{k}-w{w}-n{noise}");
                }
            }
        }
        return;
    }
    assert_eq!(
        args.len(),
        2,
        "usage: batch CASE Persistent|Copy, or batch --list"
    );
    let mode = match args[1].as_str() {
        "Persistent" => Snapshot::Persistent,
        "Copy" => Snapshot::Copy,
        _ => panic!("unknown snapshot mode"),
    };
    let mut selected = chr_cases::registry().into_iter().find(|c| c.id == args[0]);
    if selected.is_none() {
        for k in [0, 1, 2, 4, 8] {
            for w in [0, 1, 8, 64, 256] {
                for noise in [0, 16, 128] {
                    if args[0] == format!("grid-carry-k{k}-w{w}-n{noise}") {
                        let mut case = chr_cases::carry_case(k, w, noise);
                        case.id = args[0].clone();
                        case.budget = 1_000_000;
                        selected = Some(case);
                    }
                }
            }
        }
    }
    println!(
        "case\tmode\tpass\tsteps\tapplications\tintroductions\tequations\tpairs\tdereferences\toccurs\thead_candidates\tsplits\tfailed\traw\tanswers\tmax_frontier\tpending\tmap_visits\tmap_allocations\tsnapshot_copies\tpending_allocations\tterm_requests\tterm_nodes\tobserver_pairs\tobserver_scans\tallocation_calls\trequested_bytes\tbaseline_live\tpeak_live\tfinal_live\telapsed_us"
    );
    run_case(selected.expect("unknown registered case"), mode);
}
fn run_case(case: chr_cases::Case, mode: Snapshot) {
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
            actual
                .iter()
                .any(|got| chr_observe::equivalent(got, want, &mut chr_observe::Stats::default()))
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
