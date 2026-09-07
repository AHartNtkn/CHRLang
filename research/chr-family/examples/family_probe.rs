#[path = "../../chr-persistent/examples/support/allocator.rs"]
mod allocator;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
use chr_family::{Mode, Request, Solver, Value};
use chr_syntax::{Answer, Term};
fn names() -> Vec<String> {
    let mut names = vec![];
    for family in ["opaque", "prefix", "correlated", "discriminate", "cycle"] {
        for k in [0, 1, 2, 4, 8] {
            if family == "cycle" && k == 0 {
                continue;
            }
            for w in [0, 1, 8, 64, 256] {
                names.push(format!("{family}-k{k}-w{w}"));
            }
        }
    }
    names
}
fn wrap(mut t: Value, w: usize) -> Value {
    for _ in 0..w {
        t = Value::app("s", [t]);
    }
    t
}
fn source_wrap(mut t: Term, w: usize) -> Term {
    for _ in 0..w {
        t = chr_syntax::t("s", [t]);
    }
    t
}
fn family_tag(k: u32) -> Value {
    Value::app(
        "tag",
        (0..k).map(|d| Value::choice(d, Value::app("a", []), Value::app("b", []))),
    )
}
fn request(family: &str, k: u32, w: usize) -> Request {
    let (a, b, outputs) = match family {
        "opaque" => (Value::var(0), wrap(family_tag(k), w), vec![0]),
        "prefix" => (
            wrap(family_tag(k), w),
            wrap(Value::app("tag", (0..k).map(|i| Value::var(i as u64))), w),
            (0..k as u64).collect(),
        ),
        "correlated" => (wrap(family_tag(k), w), wrap(family_tag(k), w), vec![]),
        "discriminate" => (
            wrap(family_tag(k), w),
            wrap(Value::app("tag", (0..k).map(|_| Value::app("a", []))), w),
            vec![],
        ),
        "cycle" => (
            Value::var(0),
            Value::choice(
                0,
                wrap(Value::app("a", []), w),
                Value::app("s", [Value::var(0)]),
            ),
            vec![0],
        ),
        _ => panic!("unknown family"),
    };
    Request {
        labels: k,
        equations: vec![(a, b)],
        outputs,
    }
}
fn expected(family: &str, k: u32, w: usize) -> (Vec<Answer>, u64) {
    let mut answers = vec![];
    let count = if matches!(family, "correlated" | "discriminate" | "cycle") {
        1
    } else {
        1 << k
    };
    for bits in 0..count {
        let values = match family {
            "opaque" => vec![source_wrap(
                chr_syntax::t(
                    "tag",
                    (0..k)
                        .map(|d| chr_syntax::atom(if bits & (1 << d) == 0 { "a" } else { "b" }))
                        .collect::<Vec<_>>(),
                ),
                w,
            )],
            "prefix" => (0..k)
                .map(|d| chr_syntax::atom(if bits & (1 << d) == 0 { "a" } else { "b" }))
                .collect(),
            "cycle" => vec![source_wrap(chr_syntax::atom("a"), w)],
            _ => vec![],
        };
        answers.push(Answer {
            outputs: values
                .into_iter()
                .enumerate()
                .map(|(i, t)| (format!("out{i}"), t))
                .collect(),
            residual: vec![],
        });
    }
    (
        answers,
        match family {
            "cycle" => 1 << (k - 1),
            "discriminate" => 1,
            _ => 1 << k,
        },
    )
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--list"] {
        for n in names() {
            println!("{n}");
        }
        return;
    }
    assert_eq!(args.len(), 2, "family_probe CASE Eager|Named");
    assert!(names().contains(&args[0]));
    let parts = args[0].split('-').collect::<Vec<_>>();
    let family = parts[0];
    let k = parts[1][1..].parse::<u32>().unwrap();
    let w = parts[2][1..].parse::<usize>().unwrap();
    let mode = match args[1].as_str() {
        "Eager" => Mode::Eager,
        "Named" => Mode::Named,
        _ => panic!("unknown mode"),
    };
    let (expected, raw) = expected(family, k, w);
    let start = allocator::start();
    let clock = std::time::Instant::now();
    let request = request(family, k, w);
    let constructed = allocator::read();
    let construct_us = clock.elapsed().as_micros();
    let clock = std::time::Instant::now();
    let mut s = Solver::new(request, mode).unwrap();
    s.advance(1_000_000);
    let solved = allocator::read();
    let solve_us = clock.elapsed().as_micros();
    let before_projection = s.stats().projection_visits;
    let before_root_reads = s.stats().root_reads;
    let clock = std::time::Instant::now();
    let answers = s.observe().unwrap();
    let end = allocator::read();
    let observe_us = clock.elapsed().as_micros();
    let stats = s.stats();
    let pass = answers.len() == expected.len()
        && stats.raw_answers == raw
        && expected.iter().all(|e| {
            answers
                .iter()
                .any(|a| chr_observe::equivalent(a, e, &mut Default::default()))
        });
    println!(
        "case\tmode\tpass\tsteps\tpairs\troot_reads\toutput_root_reads\tabsence_visits\toccurs_visits\tsplits\tcopied_entries\tcompleted_regions\tfailed_regions\teager_projection_visits\toutput_projection_visits\teager_nodes\tbindings\traw\tanswers\tmax_frontier\tretained_regions\tobserver_pairs\tobserver_scans\tobserver_backtracks\tallocation_calls\tconstruct_requested\tsolve_requested\tobserve_requested\trequested_bytes\tbaseline_live\tpeak_live\tfinal_live\tconstruct_us\tsolve_us\tobserve_us"
    );
    let mut row = vec![args[0].clone(), format!("{mode:?}"), pass.to_string()];
    row.extend(
        [
            stats.steps,
            stats.pairs,
            before_root_reads,
            stats.root_reads - before_root_reads,
            stats.absence_visits,
            stats.occurs_visits,
            stats.splits,
            stats.copied_entries,
            stats.completed_regions,
            stats.failed_regions,
            before_projection,
            stats.projection_visits - before_projection,
            stats.eager_nodes,
            stats.bindings,
            stats.raw_answers,
            answers.len() as u64,
            stats.max_frontier as u64,
            s.retained_regions() as u64,
            stats.observer_pairs,
            stats.observer_scans,
            stats.observer_backtracks,
            end.calls - start.calls,
            constructed.requested - start.requested,
            solved.requested - constructed.requested,
            end.requested - solved.requested,
            end.requested - start.requested,
            start.live,
            end.peak,
            end.live,
            construct_us as u64,
            solve_us as u64,
            observe_us as u64,
        ]
        .map(|x| x.to_string()),
    );
    println!("{}", row.join("\t"));
    assert!(pass);
}
