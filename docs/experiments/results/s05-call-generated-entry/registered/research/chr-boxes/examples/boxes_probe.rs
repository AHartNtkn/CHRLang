#[path = "../../chr-persistent/examples/support/allocator.rs"]
mod allocator;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
use chr_boxes::{Search, Snapshot};
use chr_syntax::{Goal, Rule, and, atom, c, eq, or, v};
fn names() -> Vec<String> {
    let mut names = chr_cases::registry()
        .into_iter()
        .map(|c| c.id)
        .collect::<Vec<_>>();
    for k in [0, 1, 2, 4, 8] {
        for w in [0, 1, 8, 64, 256] {
            for n in [0, 16, 128] {
                names.push(format!("grid-carry-k{k}-w{w}-n{n}"));
            }
        }
    }
    for w in [0, 1, 8, 64, 256] {
        for n in [0, 16, 128] {
            names.push(format!("doomed-w{w}-n{n}"));
        }
    }
    names.push("refutation-loop".into());
    names
}
fn input(id: &str) -> chr_cases::Case {
    if let Some(case) = chr_cases::registry().into_iter().find(|c| c.id == id) {
        return case;
    }
    if id == "refutation-loop" {
        return chr_cases::Case {
            id: id.into(),
            rules: vec![
                Rule::simplify("choose", [c("choose", [])], or(Goal::Fail, Goal::Fail)),
                Rule::simplify("loop", [c("loop", [v(0)])], c("loop", [v(0)]).into()),
            ],
            query: chr_cases::query(vec![c("loop", [v(0)]), c("choose", [])], &[]),
            expected: vec![],
            exhausted: true,
            raw_answers: 0,
            budget: 2000,
            answer_limit: None,
        };
    }
    for k in [0, 1, 2, 4, 8] {
        for w in [0, 1, 8, 64, 256] {
            for n in [0, 16, 128] {
                if id == format!("grid-carry-k{k}-w{w}-n{n}") {
                    let mut case = chr_cases::carry_case(k, w, n);
                    case.id = id.into();
                    case.budget = 1_000_000;
                    return case;
                }
            }
        }
    }
    for w in [0, 1, 8, 64, 256] {
        for n in [0, 16, 128] {
            if id == format!("doomed-w{w}-n{n}") {
                let mut case = chr_cases::carry_case(1, w, n);
                case.rules[0].body = or(
                    and([eq(v(0), atom("a")), eq(v(0), atom("b"))]),
                    and([eq(v(0), atom("b")), eq(v(0), atom("a"))]),
                );
                case.id = id.into();
                case.budget = 1_000_000;
                case.expected.clear();
                case.raw_answers = 0;
                return case;
            }
        }
    }
    panic!("unknown case");
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--list"] {
        for n in names() {
            println!("{n}");
        }
        return;
    }
    assert_eq!(args.len(), 2, "boxes_probe CASE 0|1|8|64|256|Unlimited");
    let quota = if args[1] == "Unlimited" {
        usize::MAX
    } else {
        args[1].parse::<usize>().unwrap()
    };
    assert!([0, 1, 8, 64, 256, usize::MAX].contains(&quota));
    let mut case = input(&args[0]);
    if case.id == "refutation-loop" && quota == usize::MAX {
        case.exhausted = false;
    }
    let start = allocator::start();
    let clock = std::time::Instant::now();
    let mut s = Search::new(
        case.rules.clone(),
        case.query.clone(),
        Snapshot::Persistent,
        quota,
    )
    .unwrap();
    let mut answers = vec![];
    let mut exhausted = false;
    for _ in 0..case.budget {
        let b = s.advance(1);
        answers.extend(b.answers);
        exhausted = b.exhausted;
        if exhausted || case.answer_limit.is_some_and(|n| answers.len() >= n) {
            break;
        }
    }
    let elapsed = clock.elapsed().as_micros();
    let end = allocator::read();
    let stats = s.stats();
    let observer = s.observation_stats();
    let pass = exhausted == case.exhausted
        && stats.completed == case.raw_answers
        && answers.len() == case.expected.len()
        && case.expected.iter().all(|e| {
            answers
                .iter()
                .any(|a| chr_observe::equivalent(a, e, &mut Default::default()))
        });
    println!(
        "case\tmode\tpass\texhausted\tsteps\tapplications\tlifted_applications\tlifting_checks\tcertificate_checks\tcertificate_terms\tbarrier_nodes\tbarrier_rejections\tforced_splits\tmax_delay\tintroductions\tequations\tpairs\tdereferences\toccurs_visits\thead_candidates\tsplits\tfailed\traw\tanswers\tmax_frontier\tpending\tterm_nodes\tterm_requests\tpending_allocations\tmap_visits\tmap_allocations\tobserver_pairs\tobserver_scans\tallocation_calls\trequested_bytes\tbaseline_live\tpeak_live\tfinal_live\telapsed_us"
    );
    let mut row = vec![
        case.id,
        args[1].clone(),
        pass.to_string(),
        exhausted.to_string(),
    ];
    row.extend(
        [
            stats.steps,
            stats.applications,
            stats.lifted_applications,
            stats.lifting_checks,
            stats.certificate_checks,
            stats.certificate_terms,
            stats.barrier_nodes,
            stats.barrier_rejections,
            stats.forced_splits,
            stats.max_delay,
            stats.introductions,
            stats.equations,
            stats.pairs,
            stats.dereferences,
            stats.occurs_visits,
            stats.head_candidates,
            stats.splits,
            stats.failed,
            stats.completed,
            answers.len() as u64,
            stats.max_frontier as u64,
            s.pending_alternatives() as u64,
            stats.term_nodes as u64,
            stats.term_requests,
            stats.pending_allocations,
            stats.storage.visits,
            stats.storage.allocations,
            observer.term_pairs,
            observer.occurrence_scans,
            end.calls - start.calls,
            end.requested - start.requested,
            start.live,
            end.peak,
            end.live,
            elapsed as u64,
        ]
        .map(|v| v.to_string()),
    );
    println!("{}", row.join("\t"));
    assert!(pass);
}
