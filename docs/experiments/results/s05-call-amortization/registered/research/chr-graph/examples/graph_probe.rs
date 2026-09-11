// Measurement instrumentation only; no scalar execution algorithms are imported.
#[path = "../../chr-persistent/examples/support/allocator.rs"]
mod allocator;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
use chr_graph::{Mode, Search};
use chr_syntax::{Rule, atom, c, t, v};
fn names() -> Vec<String> {
    let mut names = chr_cases::registry()
        .into_iter()
        .filter(|c| chr_graph::eligible(&c.rules).is_ok())
        .map(|c| c.id)
        .collect::<Vec<_>>();
    for k in [0, 1, 2, 4, 8] {
        for w in [0, 1, 8, 64, 256] {
            for noise in [0, 16, 128] {
                names.push(format!("grid-carry-k{k}-w{w}-n{noise}"));
            }
        }
    }
    for k in [1, 2, 4] {
        for w in [1, 8, 64] {
            for noise in [0, 16] {
                names.push(format!("discriminate-k{k}-w{w}-n{noise}"));
            }
        }
    }
    names
}
fn input(id: &str) -> chr_cases::Case {
    if let Some(case) = chr_cases::registry().into_iter().find(|c| c.id == id) {
        return case;
    }
    for k in [0, 1, 2, 4, 8] {
        for w in [0, 1, 8, 64, 256] {
            for noise in [0, 16, 128] {
                if id == format!("grid-carry-k{k}-w{w}-n{noise}") {
                    let mut case = chr_cases::carry_case(k, w, noise);
                    case.id = id.into();
                    case.budget = 1_000_000;
                    return case;
                }
            }
        }
    }
    for k in [1, 2, 4] {
        for w in [1, 8, 64] {
            for noise in [0, 16] {
                if id == format!("discriminate-k{k}-w{w}-n{noise}") {
                    let mut case = chr_cases::carry_case(k, w, noise);
                    case.rules.truncate(1);
                    for bits in 0..1usize << k {
                        let tag = t(
                            "tag",
                            (0..k)
                                .map(|i| atom(if bits & (1 << i) == 0 { "a" } else { "b" }))
                                .collect::<Vec<_>>(),
                        );
                        case.rules.push(Rule::simplify(
                            &format!("carry-{bits}"),
                            [c("carry", [tag.clone(), t("s", [v(0)])])],
                            c("carry", [tag, v(0)]).into(),
                        ));
                    }
                    case.id = id.into();
                    case.budget = 1_000_000;
                    return case;
                }
            }
        }
    }
    panic!("unknown registered case: {id}")
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--list"] {
        for name in names() {
            println!("{name}");
        }
        return;
    }
    if args == ["--inventory"] {
        println!("case\teligible\treason");
        for c in chr_cases::registry() {
            match chr_graph::eligible(&c.rules) {
                Ok(()) => println!("{}\ttrue\tcertified", c.id),
                Err(e) => println!("{}\tfalse\t{}", c.id, e),
            }
        }
        return;
    }
    assert_eq!(args.len(), 2, "graph_probe CASE General|Local|Cached");
    let mode = match args[1].as_str() {
        "General" => Mode::General,
        "Local" => Mode::Local,
        "Cached" => Mode::Cached,
        _ => panic!("unknown mode"),
    };
    let case = input(&args[0]);
    let start = allocator::start();
    let clock = std::time::Instant::now();
    let mut search = Search::new(case.rules.clone(), case.query.clone(), mode).unwrap();
    let mut actual = vec![];
    let mut exhausted = false;
    for _ in 0..case.budget {
        let batch = search.advance(1);
        actual.extend(batch.answers);
        exhausted = batch.exhausted;
        if exhausted || case.answer_limit.is_some_and(|n| actual.len() >= n) {
            break;
        }
    }
    let elapsed = clock.elapsed().as_micros();
    let allocation = allocator::read();
    let pass = exhausted == case.exhausted
        && search.stats().completed == case.raw_answers
        && actual.len() == case.expected.len()
        && case.expected.iter().all(|e| {
            actual
                .iter()
                .any(|a| chr_observe::equivalent(a, e, &mut Default::default()))
        });
    let s = search.stats();
    let o = search.observation_stats();
    let retained = search.retained_context_entries();
    println!(
        "case\tmode\tpass\tsteps\tapplications\texpansions\tcache_hits\tcache_misses\tcache_retained\trules_examined\toccurrence_scans\tcandidates\tbinding_reads\tterm_visits\tunification_pairs\toccurs_visits\tfresh_variables\twork_nodes\tsnapshot_entries\tintroductions\tsplits\tfailed\traw\tanswers\tmax_frontier\tpending\tretained_roots\tretained_bindings\tretained_pending\tobserver_pairs\tobserver_scans\tallocation_calls\trequested_bytes\tbaseline_live\tpeak_live\tfinal_live\telapsed_us"
    );
    let mut row = vec![case.id, format!("{mode:?}"), pass.to_string()];
    row.extend(
        [
            s.steps,
            s.applications,
            s.expansions,
            s.cache_hits,
            s.cache_misses,
            search.retained_cache() as u64,
            s.rules_examined,
            s.occurrence_scans,
            s.candidates,
            s.binding_reads,
            s.term_visits,
            s.unification_pairs,
            s.occurs_visits,
            s.fresh_variables,
            s.work_nodes,
            s.snapshot_entries,
            s.introductions,
            s.splits,
            s.failed,
            s.completed,
            actual.len() as u64,
            s.max_frontier as u64,
            search.pending_alternatives() as u64,
            retained.0 as u64,
            retained.1 as u64,
            retained.2 as u64,
            o.term_pairs,
            o.occurrence_scans,
            allocation.calls - start.calls,
            allocation.requested - start.requested,
            start.live,
            allocation.peak,
            allocation.live,
            elapsed as u64,
        ]
        .map(|x| x.to_string()),
    );
    println!("{}", row.join("\t"));
    assert!(pass, "registered observation failed");
}
