#[path = "../../chr-persistent/examples/support/allocator.rs"]
mod allocator;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
use chr_factors::{Mode, Search};
#[path = "support/factor_cases.rs"]
mod cases;
use cases::{input, names};
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--list"] {
        for n in names() {
            println!("{n}");
        }
        return;
    }
    assert_eq!(args.len(), 2, "factors_probe CASE Scalar|Factored");
    let mode = match args[1].as_str() {
        "Scalar" => Mode::Scalar,
        "Factored" => Mode::Factored,
        _ => panic!("unknown mode"),
    };
    let case = input(&args[0]);
    assert!(case.exhausted);
    let start = allocator::start();
    let clock = std::time::Instant::now();
    let mut s = Search::new(case.rules.clone(), case.query.clone(), mode).unwrap();
    let b = s.advance(case.budget * 8);
    let elapsed = clock.elapsed().as_micros();
    let end = allocator::read();
    let pass = b.exhausted
        && b.answers.len() == case.expected.len()
        && s.raw_count() == Some(case.raw_answers as u128)
        && case.expected.iter().all(|e| {
            b.answers
                .iter()
                .any(|a| chr_observe::equivalent(a, e, &mut Default::default()))
        });
    let stats = s.stats();
    let sum = |f: fn(&chr_persistent::Stats) -> u64| s.source_stats().map(f).sum::<u64>();
    let observer = s.observation_stats();
    let regional = s.regional_observation_stats();
    println!(
        "case\tmode\tpass\tfactors\tcertificate_predicates\tcertificate_edges\tcertificate_terms\tsteps\tsource_steps\tapplications\tequations\tpairs\thead_candidates\tmap_visits\tmap_allocations\tterm_requests\tpending_allocations\tregional_raw\traw_product\tanswers\tproduct_jobs\tproducts\trenamed_nodes\tmax_jobs\tcache_answers\tpending_jobs\tempty_refutations\tobserver_pairs\tobserver_scans\tregional_observer_pairs\tregional_observer_scans\tregional_observer_backtracks\tallocation_calls\trequested_bytes\tbaseline_live\tpeak_live\tfinal_live\telapsed_us"
    );
    let mut row = vec![case.id, format!("{mode:?}"), pass.to_string()];
    row.extend(
        [
            s.factor_count() as u64,
            stats.certificate_predicates as u64,
            stats.certificate_edges,
            stats.certificate_terms,
            stats.steps,
            stats.source_steps,
            s.source_applications(),
            sum(|s| s.equations),
            sum(|s| s.pairs),
            sum(|s| s.head_candidates),
            sum(|s| s.storage.visits),
            sum(|s| s.storage.allocations),
            sum(|s| s.term_requests),
            sum(|s| s.pending_allocations),
            sum(|s| s.completed),
            b.answers.len() as u64,
            stats.product_jobs,
            stats.products,
            stats.renamed_nodes,
            stats.max_jobs as u64,
            s.cached_answers() as u64,
            s.pending_jobs() as u64,
            stats.empty_refutations,
            observer.term_pairs,
            observer.occurrence_scans,
            regional.term_pairs,
            regional.occurrence_scans,
            regional.backtracks,
            end.calls - start.calls,
            end.requested - start.requested,
            start.live,
            end.peak,
            end.live,
            elapsed as u64,
        ]
        .map(|x| x.to_string()),
    );
    row.insert(
        18,
        s.raw_count()
            .map(|n| n.to_string())
            .unwrap_or_else(|| "unavailable".into()),
    );
    println!("{}", row.join("\t"));
    assert!(pass);
}
