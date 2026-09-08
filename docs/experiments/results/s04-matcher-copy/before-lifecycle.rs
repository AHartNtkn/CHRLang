#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use chr_restoration::{Mode, Prepared as Restored, Step};
use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, and, atom, c, eq, or, t, v};
use std::sync::Arc;
use std::time::Instant;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
const FAMILIES: [&str; 9] = [
    "linear", "small", "retained", "mutation", "work", "deep", "spine", "early", "late",
];
const MODES: [&str; 7] = [
    "copy",
    "trail",
    "replay",
    "checkpoint1",
    "checkpoint4",
    "checkpoint16",
    "indexed",
];
#[derive(Clone, Copy)]
struct Config {
    n: usize,
    depth: usize,
    edits: usize,
    work: usize,
}
fn config(f: &str) -> Config {
    let mut x = Config {
        n: 8,
        depth: 3,
        edits: 0,
        work: 2,
    };
    match f {
        "linear" => {
            x.n = 64;
            x.depth = 0;
            x.work = 16;
        }
        "small" => (),
        "retained" => x.n = 128,
        "mutation" => {
            x.n = 128;
            x.edits = 32;
        }
        "work" => x.work = 32,
        "deep" | "spine" => x.depth = 6,
        "early" | "late" => {
            x.n = 64;
            x.depth = 4;
            x.edits = 8;
            x.work = 16;
        }
        _ => panic!("unknown family"),
    }
    x
}
fn nat(n: usize) -> Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
fn rules(f: &str) -> Vec<Rule> {
    let x = config(f);
    let ids = (0..x.edits)
        .rev()
        .fold(atom("nil"), |xs, i| t("cons", [atom(&format!("i{i}")), xs]));
    let branch = |bit: &str, next: Term| Goal::from(c("edit", [ids.clone(), atom(bit), next]));
    let left = branch("a", t("next", [v(0), t("cons", [atom("a"), v(1)])]));
    let right = match f {
        "early" => Goal::Fail,
        "late" => branch("b", atom("reject")),
        _ => branch(
            "b",
            t(
                "next",
                [
                    if f == "spine" { atom("z") } else { v(0) },
                    t("cons", [atom("b"), v(1)]),
                ],
            ),
        ),
    };
    vec![
        Rule::simplify("choose", [c("go", [t("s", [v(0)]), v(1)])], or(left, right)),
        Rule::simplify(
            "done",
            [c("go", [atom("z"), v(0)])],
            c("result", [v(0)]).into(),
        ),
        Rule::simplify(
            "edit",
            [
                c("edit", [t("cons", [v(0), v(1)]), v(2), v(3)]),
                c("cell", [v(0), v(4)]),
            ],
            and(vec![
                eq(v(4), v(2)),
                c("cell", [v(0), v(5)]).into(),
                c("edit", [v(1), v(2), v(3)]).into(),
            ]),
        ),
        Rule::simplify(
            "edited",
            [c("edit", [atom("nil"), v(0), v(1)])],
            c("work", [nat(x.work), v(1)]).into(),
        ),
        Rule::simplify(
            "work",
            [c("work", [t("s", [v(0)]), v(1)])],
            c("work", [v(0), v(1)]).into(),
        ),
        Rule::simplify(
            "resume",
            [c("work", [atom("z"), t("next", [v(0), v(1)])])],
            c("go", [v(0), v(1)]).into(),
        ),
        Rule::simplify(
            "reject",
            [c("work", [atom("z"), atom("reject")])],
            Goal::Fail,
        ),
    ]
}
fn query(f: &str, seed: usize) -> Query {
    let x = config(f);
    let mut rows: Vec<_> = (0..x.n)
        .map(|i| {
            c(
                "cell",
                [
                    atom(&format!("i{i}")),
                    v(100 + seed as u64 * 1000 + i as u64),
                ],
            )
        })
        .collect();
    let path = atom(&format!("query{seed}"));
    rows.push(if f == "linear" {
        c("work", [nat(x.work), t("next", [nat(0), path])])
    } else {
        c("go", [nat(x.depth), path])
    });
    Query {
        constraints: rows,
        outputs: vec![("first".into(), Var(100 + seed as u64 * 1000))],
    }
}
enum Prepared {
    Rest(Arc<Restored>, Mode),
    Indexed(chr_compiled::PreparedRuleset),
}
#[allow(clippy::large_enum_variant)]
enum Running {
    Rest(chr_restoration::Engine),
    Indexed(chr_compiled::SearchEngine),
}
impl Prepared {
    fn new(mode: &str, rules: &[Rule]) -> Self {
        let mode = match mode {
            "indexed" => {
                return Self::Indexed(
                    chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap(),
                );
            }
            "copy" => Mode::Copy,
            "trail" => Mode::Trail,
            "replay" => Mode::Replay,
            "checkpoint1" => Mode::Checkpoint(1.try_into().unwrap()),
            "checkpoint4" => Mode::Checkpoint(4.try_into().unwrap()),
            "checkpoint16" => Mode::Checkpoint(16.try_into().unwrap()),
            _ => panic!("unknown mode"),
        };
        Self::Rest(Restored::new(rules).unwrap(), mode)
    }
    fn start(&self, q: &Query) -> Running {
        match self {
            Self::Rest(p, m) => Running::Rest(p.start(q, *m).unwrap()),
            Self::Indexed(p) => Running::Indexed(
                p.start_search(
                    q.clone(),
                    chr_compiled::Policy::Global,
                    chr_compiled::Access::Indexed,
                )
                .unwrap(),
            ),
        }
    }
}
fn execute(e: &mut Running, stop_after_first: bool) -> (Vec<Answer>, Option<u128>) {
    let start = Instant::now();
    let mut first = None;
    let mut answers = Vec::new();
    for _ in 0..100_000 {
        let answer = match e {
            Running::Rest(e) => match e.advance() {
                Step::Answer(a) => Some(a),
                Step::Progress => None,
                Step::Exhausted => return (answers, first),
            },
            Running::Indexed(e) => match e.tick() {
                chr_compiled::SearchEvent::Complete(mut b) => Some(b.engine.observe().unwrap()),
                chr_compiled::SearchEvent::Exhausted => return (answers, first),
                _ => None,
            },
        };
        if let Some(a) = answer {
            if first.is_none() {
                first = Some(start.elapsed().as_nanos());
            }
            answers.push(a);
            if stop_after_first {
                return (answers, first);
            }
        }
    }
    panic!("finite lifecycle source-service cutoff");
}
#[derive(Clone, Copy)]
struct Measurement {
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn measure<T>(f: impl FnOnce() -> T) -> (T, Measurement) {
    #[cfg(feature = "alloc-meter")]
    let m = meter::begin();
    let start = Instant::now();
    let value = f();
    let ns = start.elapsed().as_nanos();
    #[cfg(feature = "alloc-meter")]
    let memory = meter::end(m);
    (
        value,
        Measurement {
            ns,
            #[cfg(feature = "alloc-meter")]
            memory,
        },
    )
}
impl Measurement {
    fn json(self) -> String {
        #[cfg(feature = "alloc-meter")]
        let mem = self.memory.json();
        #[cfg(not(feature = "alloc-meter"))]
        let mem = "null";
        format!("{{\"ns\":{},\"memory\":{}}}", self.ns, mem)
    }
}
struct Row {
    setup: Measurement,
    execution: Measurement,
    disposal: Measurement,
    answers: Measurement,
    first: Option<u128>,
    count: usize,
}
fn gate(selected: Option<&str>, selected_mode: Option<&str>) {
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    for f in FAMILIES {
        if selected.is_some_and(|selected| selected != f) {
            continue;
        }
        let rules = rules(f);
        for seed in [0, 7] {
            let q = query(f, seed);
            let expected = oracle::run(&rules, &q, 100_000);
            let count = match f {
                "early" | "late" | "linear" => 1,
                "spine" => config(f).depth + 1,
                _ => 1 << config(f).depth,
            };
            assert_eq!(expected.len(), count);
            for mode in MODES {
                if selected_mode.is_some_and(|selected| selected != mode) {
                    continue;
                }
                let p = Prepared::new(mode, &rules);
                oracle::same_raw(execute(&mut p.start(&q), false).0, expected.clone());
            }
        }
    }
    println!(
        "selected source/query cases: complete executor comparisons and analytical answer counts pass"
    );
}
fn cell(mode: &str, f: &str, reuse: usize) {
    assert!(MODES.contains(&mode) && FAMILIES.contains(&f) && [1, 8].contains(&reuse));
    assert!(!cfg!(feature = "arena-cow") || mode == "indexed");
    let rules = rules(f);
    let queries: Vec<_> = (0..reuse).map(|i| query(f, i)).collect();
    let expected: Vec<_> = queries
        .iter()
        .map(|q| oracle::run(&rules, q, 100_000))
        .collect();
    let warm = Prepared::new(mode, &rules);
    for _ in 0..2 {
        oracle::same_raw(
            execute(&mut warm.start(&queries[0]), false).0,
            expected[0].clone(),
        );
    }
    drop(warm);
    let mut rows = Vec::with_capacity(reuse);
    let (p, preparation) = measure(|| Prepared::new(mode, &rules));
    for (q, expected) in queries.iter().zip(&expected) {
        let (mut e, setup) = measure(|| p.start(q));
        let ((answers, first), execution) = measure(|| execute(&mut e, false));
        oracle::same_raw(answers.clone(), expected.clone());
        let count = answers.len();
        let (_, disposal) = measure(|| drop(e));
        let (_, answers) = measure(|| drop(answers));
        rows.push(Row {
            setup,
            execution,
            disposal,
            answers,
            first,
            count,
        });
    }
    let (mut cancelled, cancel_setup) = measure(|| p.start(&queries[0]));
    let ((cancel_answers, _), cancel_execution) = measure(|| execute(&mut cancelled, true));
    assert_eq!(cancel_answers.len(), 1);
    assert!(expected[0].iter().any(|a| chr_observe::equivalent(
        a,
        &cancel_answers[0],
        &mut Default::default()
    )));
    let (_, cancel_disposal) = measure(|| drop(cancelled));
    let (_, cancel_answer_disposal) = measure(|| drop(cancel_answers));
    let (_, prepared_disposal) = measure(|| drop(p));
    #[cfg(feature = "alloc-meter")]
    assert_eq!(
        preparation.memory.live_start, prepared_disposal.memory.live_end,
        "owned lifecycle did not release allocations"
    );
    let rows=rows.into_iter().map(|r|format!("{{\"setup\":{},\"execution_observation\":{},\"engine_disposal\":{},\"answer_disposal\":{},\"first_ns\":{},\"count\":{}}}",r.setup.json(),r.execution.json(),r.disposal.json(),r.answers.json(),r.first.map_or("null".into(),|x|x.to_string()),r.count)).collect::<Vec<_>>().join(",");
    println!(
        "{{\"family\":\"{f}\",\"mode\":\"{mode}\",\"reuse\":{reuse},\"meter\":{},\"cow\":{},\"preparation\":{},\"prepared_disposal\":{},\"queries\":[{}],\"cancellation\":{{\"setup\":{},\"execution_observation\":{},\"engine_disposal\":{},\"answer_disposal\":{}}}}}",
        cfg!(feature = "alloc-meter"),
        cfg!(feature = "arena-cow"),
        preparation.json(),
        prepared_disposal.json(),
        rows,
        cancel_setup.json(),
        cancel_execution.json(),
        cancel_disposal.json(),
        cancel_answer_disposal.json()
    );
}
#[cfg(feature = "replay-diagnostic")]
fn work() {
    use chr_restoration::diagnostics;
    for f in FAMILIES {
        let source = rules(f);
        let q = query(f, 0);
        let expected = oracle::run(&source, &q, 100_000);
        let projection = diagnostics::project(&source, &q, 100_000);
        oracle::same_raw(projection.answers, expected.clone());
        for mode in MODES {
            if mode == "indexed" || (mode == "replay" && f == "mutation") {
                continue;
            }
            let p = Prepared::new(mode, &source);
            let mut e = p.start(&q);
            diagnostics::reset();
            let answers = execute(&mut e, false).0;
            let steps = diagnostics::steps();
            let env = diagnostics::environments();
            eprintln!(
                "{{\"family\":\"{f}\",\"mode\":\"{mode}\",\"environment_clones\":{},\"cloned_nodes\":{},\"rejected_clones\":{},\"rejected_nodes\":{}}}",
                env[0], env[1], env[2], env[3]
            );
            oracle::same_raw(answers, expected.clone());
            if mode == "copy" || mode == "trail" {
                assert_eq!(steps, projection.service_steps);
            }
            if mode == "replay" {
                assert_eq!(steps, projection.root_replay_steps);
            }
            println!(
                "{{\"family\":\"{f}\",\"mode\":\"{mode}\",\"actual_source_steps\":{steps},\"projected_root_steps\":{},\"service_steps\":{}}}",
                projection.root_replay_steps, projection.service_steps
            );
        }
    }
}
fn main() {
    assert!(!std::hint::black_box(chr_compiled::COLLECT_METRICS));
    let a: Vec<_> = std::env::args().collect();
    #[cfg(feature = "replay-diagnostic")]
    if a.get(1).map(String::as_str) == Some("work") {
        work();
        return;
    }
    assert!(
        !std::hint::black_box(cfg!(feature = "replay-diagnostic")),
        "diagnostic build cannot produce timing samples"
    );
    if a.get(1).map(String::as_str) == Some("gate") {
        gate(a.get(2).map(String::as_str), a.get(3).map(String::as_str));
    } else {
        assert_eq!(a.len(), 4);
        cell(&a[1], &a[2], a[3].parse().unwrap());
    }
}
