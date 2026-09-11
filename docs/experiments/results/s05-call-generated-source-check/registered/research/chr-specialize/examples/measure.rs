#[path = "support/allocator.rs"]
mod allocator;
#[path = "support/cases.rs"]
mod cases;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
use chr_syntax::{Goal, Term};
fn term_size(t: &Term) -> usize {
    1 + match t {
        Term::Var(_) => 0,
        Term::App(_, args) => args.iter().map(term_size).sum(),
    }
}
fn code_size(g: &Goal) -> (usize, usize) {
    match g {
        Goal::Constraint(c) => (1, c.args.iter().map(term_size).sum()),
        Goal::Unify(a, b) => (1, term_size(a) + term_size(b)),
        Goal::And(gs) => gs
            .iter()
            .map(code_size)
            .fold((1, 0), |(a, b), (c, d)| (a + c, b + d)),
        Goal::Or(a, b) => {
            let a = code_size(a);
            let b = code_size(b);
            (1 + a.0 + b.0, a.1 + b.1)
        }
        _ => (1, 0),
    }
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--list"] {
        for c in cases::cases() {
            println!("{}", c.id);
        }
        return;
    }
    assert_eq!(args.len(), 2);
    let budget = args[1].parse::<usize>().unwrap();
    assert!([0, 1, 2, 4, 8, 16].contains(&budget));
    let case = cases::cases()
        .into_iter()
        .find(|c| c.id == args[0])
        .expect("registered case");
    let before = allocator::start();
    let time = std::time::Instant::now();
    // The unspecialized control pays no compiler or extra prepared-program clone.
    let prepared = if budget == 0 {
        None
    } else {
        Some(chr_specialize::specialize(&case.rules, &case.query, budget).unwrap())
    };
    let compile_us = time.elapsed().as_micros();
    let compiled = allocator::read();
    let rules = prepared
        .as_ref()
        .map_or(case.rules.as_slice(), |p| p.rules.as_slice());
    let query = prepared.as_ref().map_or(&case.query, |p| &p.query);
    let empty = chr_specialize::Stats::default();
    let stats = prepared.as_ref().map_or(&empty, |p| &p.stats);
    let code = rules
        .iter()
        .map(|r| {
            let (g, t) = code_size(&r.body);
            (
                g,
                t + r
                    .kept
                    .iter()
                    .chain(&r.removed)
                    .flat_map(|c| &c.args)
                    .map(term_size)
                    .sum::<usize>(),
            )
        })
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
    let mut samples = [[0u64; 17]; 3];
    for sample in &mut samples {
        let start = allocator::start();
        let time = std::time::Instant::now();
        let mut search = chr_persistent::Search::new(
            rules.to_vec(),
            query.clone(),
            chr_persistent::Snapshot::Persistent,
        )
        .unwrap();
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
        let elapsed = time.elapsed().as_micros() as u64;
        let after = allocator::read();
        let s = search.stats();
        *sample = [
            s.steps,
            s.applications,
            s.equations,
            s.pairs,
            s.occurs_visits,
            s.head_candidates,
            s.completed,
            actual.len() as u64,
            s.term_requests,
            s.term_nodes as u64,
            after.calls - start.calls,
            after.requested - start.requested,
            start.live,
            after.peak,
            after.live,
            elapsed,
            s.failed,
        ];
        assert!(
            exhausted == case.exhausted
                && actual.len() == case.expected.len()
                && s.completed == case.raw_answers
                && case
                    .expected
                    .iter()
                    .all(|e| actual.iter().any(|a| chr_observe::equivalent(
                        a,
                        e,
                        &mut chr_observe::Stats::default()
                    ))),
            "{} budget {budget}",
            case.id
        );
    }
    println!(
        "case\tbudget\trepetition\tpass\texpansions\tcontradictions\tidentities\tvariants\tcode_rules\tcode_goals\tcode_terms\tcompile_requested\tcompile_baseline\tcompile_peak\tcompile_final\tcompile_us\tsteps\tapplications\tequations\tpairs\toccurs\thead_candidates\traw\tanswers\tterm_requests\tterm_nodes\tallocation_calls\truntime_requested\truntime_baseline\truntime_peak\truntime_final\truntime_us\tfailed"
    );
    for (i, s) in samples.iter().enumerate() {
        let values = s.iter().map(u64::to_string).collect::<Vec<_>>().join("\t");
        println!(
            "{}\t{budget}\t{i}\ttrue\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{compile_us}\t{values}",
            case.id,
            stats.expansions,
            stats.contradictions,
            stats.identities,
            stats.variants,
            rules.len(),
            code.0,
            code.1,
            compiled.requested - before.requested,
            before.live,
            compiled.peak,
            compiled.live
        );
    }
}
