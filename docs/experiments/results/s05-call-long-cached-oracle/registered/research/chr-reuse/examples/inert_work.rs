#[cfg(test)]
mod tests {
    #[test]
    fn deep_calls_preserve_independent_answers_and_caller_identity() {
        for family in 0..6 {
            for distinct in [false, true] {
                super::case(family, 32, distinct, false);
            }
        }
    }
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 5 {
        case_selected(
            args[1].parse().unwrap(),
            args[2].parse().unwrap(),
            args[3].parse().unwrap(),
            true,
            Some(&args[4]),
        );
        return;
    }
    assert_eq!(args.len(), 1);
    for family in 0..6 {
        for depth in [0, 4, 32, 128] {
            for distinct in [false, true] {
                case(family, depth, distinct, true);
            }
        }
    }
}
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "support/inert_source.rs"]
mod source;
use chr_reuse::continuations::{Batch, Mode, Prepared as Whole, Stats};
use chr_reuse::residuals::Prepared as Separated;
use chr_syntax::Answer;
fn same_order(a: &[Answer], b: &[Answer]) {
    assert_eq!(a.len(), b.len());
    for (a, b) in a.iter().zip(b) {
        assert!(chr_observe::equivalent(a, b, &mut Default::default()));
    }
}
fn collect<E>(engine: &mut E, advance: &impl Fn(&mut E) -> Batch) -> Vec<Answer> {
    let mut answers = vec![];
    for _ in 0..200_000 {
        let batch = advance(engine);
        answers.extend(batch.answers);
        if batch.exhausted {
            return answers;
        }
    }
    panic!("service bound");
}
fn counts(stats: &Stats) -> [u64; 8] {
    [
        stats.logical_steps,
        stats.executed,
        stats.hits,
        stats.key_requests,
        stats.states as u64,
        stats.completed,
        stats.failed,
        stats.max_frontier as u64,
    ]
}
struct Config<'a> {
    family: usize,
    depth: usize,
    distinct: bool,
    print: bool,
    expected: &'a [Vec<Answer>],
    selected: Option<&'a str>,
}
fn run<P, E>(
    cfg: &Config<'_>,
    mode: &str,
    prepared: P,
    start: impl Fn(&P, chr_syntax::Query) -> E,
    advance: impl Fn(&mut E) -> Batch,
    stats: impl Fn(&E) -> [u64; 8],
    #[cfg(feature = "stage-alloc")] profile: impl Fn(&E) -> chr_reuse::allocation_profile::Profile,
) {
    if cfg.selected.is_some_and(|selected| selected != mode) {
        return;
    }
    let mut held = vec![];
    for (i, offset) in [7, 1007].into_iter().enumerate() {
        let (_, query) = source::source(cfg.family, cfg.depth, cfg.distinct, offset);
        let mut cancelled = start(&prepared, query.clone());
        drop(advance(&mut cancelled));
        drop(cancelled);
        #[cfg(feature = "stage-alloc")]
        let allocation_start = chr_compiled::experiment::meter::begin();
        let mut engine = start(&prepared, query);
        let answers = collect(&mut engine, &advance);
        let counters = stats(&engine);
        #[cfg(feature = "stage-alloc")]
        let (allocation, stages) = {
            let stages = profile(&engine);
            let allocation = chr_compiled::experiment::meter::end(allocation_start);
            assert!(stages.bytes.iter().sum::<usize>() <= allocation.requested_bytes);
            (allocation, stages)
        };
        drop(engine);
        same_order(&answers, &cfg.expected[i]);
        assert_eq!(answers.len(), if cfg.family == 5 { 1 } else { 2 });
        if !chr_reuse::continuations::COLLECT_METRICS {
            assert_eq!(counters, [0; 8]);
        }
        held.push(answers);
        if cfg.print {
            #[cfg(feature = "stage-alloc")]
            println!(
                "{{\"event\":\"allocation\",\"family\":{},\"depth\":{},\"distinct\":{},\"offset\":{offset},\"mode\":\"{mode}\",\"requested\":{},\"stage_calls\":{:?},\"stage_bytes\":{:?},\"stage_allocations\":{:?}}}",
                cfg.family,
                cfg.depth,
                cfg.distinct,
                allocation.requested_bytes,
                stages.calls,
                stages.bytes,
                stages.allocations
            );
            println!(
                "{{\"family\":{},\"depth\":{},\"distinct\":{},\"offset\":{offset},\"mode\":\"{mode}\",\"counters\":{counters:?}}}",
                cfg.family, cfg.depth, cfg.distinct
            );
        }
    }
    drop(prepared);
    for (actual, expected) in held.iter().zip(cfg.expected) {
        same_order(actual, expected);
    }
}
fn case(family: usize, depth: usize, distinct: bool, print: bool) {
    case_selected(family, depth, distinct, print, None);
}
fn case_selected(family: usize, depth: usize, distinct: bool, print: bool, selected: Option<&str>) {
    assert!(
        selected.is_none_or(|s| ["direct", "whole", "compact", "separate", "memo"].contains(&s))
    );
    let rules = source::source(family, depth, distinct, 7).0;
    let direct = Whole::new(rules.clone(), Mode::Direct).unwrap();
    let mut expected = vec![];
    for offset in [7, 1007] {
        let (_, query) = source::source(family, depth, distinct, offset);
        let want = oracle::run(&rules, &query, 200_000);
        let mut engine = direct.start(query).unwrap();
        let answers = collect(&mut engine, &|e| e.advance(1));
        oracle::same_raw(answers.clone(), want);
        expected.push(answers);
    }
    drop(direct);
    let cfg = Config {
        family,
        depth,
        distinct,
        print,
        expected: &expected,
        selected,
    };
    for (name, mode) in [
        ("direct", Mode::Direct),
        ("whole", Mode::AlphaLive),
        ("compact", Mode::CompactLive),
    ] {
        run(
            &cfg,
            name,
            Whole::new(rules.clone(), mode).unwrap(),
            |p, q| p.start(q).unwrap(),
            |e| e.advance(1),
            |e| counts(e.stats()),
            #[cfg(feature = "stage-alloc")]
            |e| e.allocation_profile().clone(),
        );
    }
    for (name, memo) in [("separate", false), ("memo", true)] {
        run(
            &cfg,
            name,
            Separated::new(rules.clone(), memo).unwrap(),
            |p, q| p.start(q).unwrap(),
            |e| e.advance(1),
            |e| counts(e.stats()),
            #[cfg(feature = "stage-alloc")]
            |e| e.allocation_profile().clone(),
        );
    }
}
