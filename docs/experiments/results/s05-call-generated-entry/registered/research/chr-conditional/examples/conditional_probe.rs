// Shared measurement instrumentation only; engine implementations remain independent.
#[path = "../../chr-persistent/examples/support/allocator.rs"]
mod allocator;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
use chr_conditional::{Grouping, Search};
use chr_syntax::{Rule, atom, c, t, v};
fn names() -> Vec<String> {
    let mut names = chr_cases::registry()
        .into_iter()
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
    assert_eq!(
        args.len(),
        2,
        "usage: conditional_probe CASE Singleton|Ready, or --list"
    );
    let grouping = match args[1].as_str() {
        "Singleton" => Grouping::Singleton,
        "Ready" => Grouping::Ready,
        _ => panic!("unknown grouping mode"),
    };
    let case = input(&args[0]);
    let before = allocator::start();
    let started = std::time::Instant::now();
    let mut search = Search::new(case.rules.clone(), case.query.clone(), grouping).unwrap();
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
    search.validate().unwrap();
    let s = search.stats();
    let r = search.retained();
    let o = search.observation_stats();
    println!(
        "case\tmode\tpass\tquanta\tprojected_steps\texpansions\tprojected_applications\tintroductions\tprojected_introductions\tsplits\tfailed\traw\tanswers\tsupport_reads\tsupport_writes\toccurrence_scans\tbinding_scans\tmatch_candidates\tterm_visits\tterm_copies\tunification_pairs\toccurs_visits\tpreviewed\tqueue_scans\tqueue_refs\twork_nodes\tmax_frontier\tpending\tretained_occurrences\tvariables\toccurrence_births\tbinding_edges\ttokens\tsupport_memberships\tobserver_pairs\tobserver_scans\tallocation_calls\trequested_bytes\tbaseline_live\tpeak_live\tfinal_live\telapsed_us"
    );
    println!(
        "{}\t{grouping:?}\t{pass}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{elapsed}",
        case.id,
        s.quanta,
        s.projected_steps,
        s.expansions,
        s.projected_applications,
        s.introductions,
        s.projected_introductions,
        s.splits,
        s.failed,
        s.completed,
        actual.len(),
        s.support_reads,
        s.support_writes,
        s.occurrence_scans,
        s.binding_scans,
        s.match_candidates,
        s.term_visits,
        s.term_copies,
        s.unification_pairs,
        s.occurs_visits,
        s.previewed,
        s.queue_scans,
        s.queue_refs,
        s.work_nodes,
        s.max_frontier,
        search.pending_alternatives(),
        r.occurrences,
        r.variables,
        r.occurrence_births,
        r.binding_edges,
        r.tokens,
        r.support_memberships,
        o.term_pairs,
        o.occurrence_scans,
        after.calls - before.calls,
        after.requested - before.requested,
        before.live,
        after.peak,
        after.live
    );
    assert!(pass, "{} {grouping:?}", case.id);
}
