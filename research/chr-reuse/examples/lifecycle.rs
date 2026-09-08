#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use chr_reuse::stable::Policy as CachePolicy;
use chr_syntax::{Answer, Goal, Query, Rule, atom, c, eq, t, v};
use std::{collections::VecDeque, time::Instant};
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/equation_support/mod.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
const FAMILIES: [&str; 6] = [
    "before-success",
    "after-success",
    "before-clash",
    "after-clash",
    "trivial",
    "unique",
];
const MODES: [&str; 8] = [
    "ordinary",
    "direct",
    "exact",
    "dependencies",
    "indexed",
    "specialized",
    "conditional",
    "graph",
];
fn rules(f: &str) -> Vec<Rule> {
    let placement = if f.starts_with("before") {
        fixture::Placement::BeforeGate
    } else {
        fixture::Placement::AfterGate
    };
    let mut rules = fixture::rules(placement);
    if f == "unique" {
        for (key, rule) in rules.iter_mut().skip(2).enumerate() {
            let Goal::And(body) = &mut rule.body else {
                panic!("gate body")
            };
            body[0] = eq(
                t("tag", [fixture::tuple(key as u8), v(0)]),
                t("tag", [fixture::tuple(key as u8), v(1)]),
            );
        }
    }
    rules
}
fn query(f: &str, seed: usize) -> Query {
    let mut q = fixture::query(64, f.ends_with("clash"));
    if f == "trivial" {
        q.constraints[0].args[0] = atom("same");
        q.constraints[0].args[1] = atom("same");
    }
    q.constraints
        .push(c("query_tag", [atom(&format!("q{seed}"))]));
    q
}
fn expected(f: &str, seed: usize) -> Vec<Answer> {
    if f.ends_with("clash") {
        return vec![];
    }
    (0..16)
        .map(|key| Answer {
            outputs: vec![
                ("x".into(), v(10)),
                ("y".into(), v(if f == "trivial" { 11 } else { 10 })),
                ("z".into(), v(if f == "trivial" { 12 } else { 10 })),
                ("unused".into(), v(99)),
            ],
            residual: vec![
                c("query_tag", [atom(&format!("q{seed}"))]),
                c("witness", [fixture::tuple(key), v(10), v(10)]),
                c("done", [v(10), v(10)]),
            ],
        })
        .collect()
}
enum Prepared {
    Ordinary(chr_persistent::continuations::PreparedMachine),
    Cache(chr_reuse::stable_search::Prepared),
    Indexed(chr_compiled::PreparedRuleset),
    Conditional(chr_direct_conditional::engine::PreparedRuleset),
    Graph(chr_direct_choice::engine::PreparedRuleset),
}
#[allow(clippy::large_enum_variant)]
enum Running {
    Ordinary(
        chr_persistent::continuations::Machine,
        VecDeque<chr_persistent::continuations::Cursor>,
    ),
    Cache(chr_reuse::stable_search::Search),
    Indexed(chr_compiled::SearchEngine),
    Conditional(chr_direct_conditional::engine::Engine),
    Graph(chr_direct_choice::engine::Engine),
}
impl Prepared {
    fn new(mode: &str, rules: &[Rule]) -> Self {
        match mode {
            "ordinary" => Self::Ordinary(
                chr_persistent::continuations::PreparedMachine::new(rules.to_vec()).unwrap(),
            ),
            "direct" | "exact" | "dependencies" => Self::Cache(
                chr_reuse::stable_search::Prepared::new(
                    rules.to_vec(),
                    match mode {
                        "direct" => CachePolicy::Direct,
                        "exact" => CachePolicy::Exact,
                        _ => CachePolicy::Dependencies,
                    },
                    32,
                )
                .unwrap(),
            ),
            "indexed" => {
                Self::Indexed(chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap())
            }
            "specialized" => {
                let p = chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap();
                let eligible = p.region_eligibility();
                assert!(
                    rules
                        .iter()
                        .flat_map(|r| r.kept.iter().chain(&r.removed))
                        .all(|head| eligible
                            .iter()
                            .any(|e| e.predicate == (head.name.clone(), head.args.len())
                                && e.eligible))
                );
                Self::Indexed(p.specialize_inferred())
            }
            "conditional" => Self::Conditional(
                chr_direct_conditional::engine::PreparedRuleset::new(rules.to_vec()).unwrap(),
            ),
            "graph" => Self::Graph(
                chr_direct_choice::engine::PreparedRuleset::new(rules.to_vec()).unwrap(),
            ),
            _ => panic!("unknown mode"),
        }
    }
    fn start(&self, q: &Query) -> Running {
        match self {
            Self::Ordinary(p) => {
                let (m, c) = p.start(q.clone()).unwrap();
                Running::Ordinary(m, VecDeque::from([c]))
            }
            Self::Cache(p) => Running::Cache(p.start(q.clone()).unwrap()),
            Self::Indexed(p) => Running::Indexed(
                p.start_search(
                    q.clone(),
                    chr_compiled::Policy::Global,
                    chr_compiled::Access::Indexed,
                )
                .unwrap(),
            ),
            Self::Conditional(p) => Running::Conditional(p.start(q.clone()).unwrap()),
            Self::Graph(p) => Running::Graph(p.start(q.clone()).unwrap()),
        }
    }
}
fn execute(e: &mut Running, stop: bool) -> (Vec<Answer>, Option<u128>) {
    let start = Instant::now();
    let mut answers = vec![];
    let mut first = None;
    for _ in 0..2_000_000 {
        let mut exhausted = false;
        let answer = match e {
            Running::Ordinary(m, q) => {
                use chr_persistent::continuations::Step;
                if let Some(c) = q.pop_front() {
                    match m.step(c) {
                        Step::Continue(c) => {
                            q.push_back(c);
                            None
                        }
                        Step::Split(a, b) => {
                            q.push_back(a);
                            q.push_back(b);
                            None
                        }
                        Step::Failed => None,
                        Step::Answer(a) => Some(a),
                    }
                } else {
                    exhausted = true;
                    None
                }
            }
            Running::Cache(s) => {
                let mut b = s.advance(1);
                exhausted = b.exhausted;
                b.answers.pop()
            }
            Running::Indexed(e) => match e.tick() {
                chr_compiled::SearchEvent::Complete(mut b) => Some(b.engine.observe().unwrap()),
                chr_compiled::SearchEvent::Exhausted => {
                    exhausted = true;
                    None
                }
                _ => None,
            },
            Running::Conditional(e) => match e.tick() {
                chr_direct_conditional::engine::Event::Answer(a) => Some(a),
                chr_direct_conditional::engine::Event::Exhausted => {
                    exhausted = true;
                    None
                }
                _ => None,
            },
            Running::Graph(e) => match e.tick() {
                chr_direct_choice::engine::Event::Answer(a) => Some(a),
                chr_direct_choice::engine::Event::Exhausted => {
                    exhausted = true;
                    None
                }
                _ => None,
            },
        };
        if let Some(a) = answer {
            if first.is_none() {
                first = Some(start.elapsed().as_nanos());
            }
            answers.push(a);
            if stop {
                return (answers, first);
            }
        }
        if exhausted {
            return (answers, first);
        }
    }
    panic!("source-service cutoff")
}
fn gate(selected: Option<&str>, selected_mode: Option<&str>) {
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    for f in FAMILIES {
        if selected.is_some_and(|x| x != f) {
            continue;
        }
        let rs = rules(f);
        for seed in [0, 7] {
            let q = query(f, seed);
            let expected = expected(f, seed);
            oracle::same_raw(oracle::run(&rs, &q, 100_000), expected.clone());
            for mode in MODES {
                if selected_mode.is_some_and(|x| x != mode) {
                    continue;
                }
                let p = Prepared::new(mode, &rs);
                oracle::same_raw(execute(&mut p.start(&q), false).0, expected.clone());
            }
        }
    }
    println!("selected complete source cases pass independent and analytical answers");
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
fn cell(mode: &str, f: &str, reuse: usize) {
    assert!(MODES.contains(&mode) && FAMILIES.contains(&f) && [1, 8].contains(&reuse));
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
    assert_eq!(cancel_answers.len(), usize::from(!expected[0].is_empty()));
    assert!(
        cancel_answers.is_empty()
            || expected[0].iter().any(|a| chr_observe::equivalent(
                a,
                &cancel_answers[0],
                &mut Default::default()
            ))
    );
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
        "{{\"family\":\"{f}\",\"mode\":\"{mode}\",\"reuse\":{reuse},\"meter\":{},\"preparation\":{},\"prepared_disposal\":{},\"queries\":[{}],\"cancellation\":{{\"setup\":{},\"execution_observation\":{},\"engine_disposal\":{},\"answer_disposal\":{}}}}}",
        cfg!(feature = "alloc-meter"),
        preparation.json(),
        prepared_disposal.json(),
        rows,
        cancel_setup.json(),
        cancel_execution.json(),
        cancel_disposal.json(),
        cancel_answer_disposal.json()
    );
}
fn work() {
    assert!(std::hint::black_box(cfg!(feature = "metrics")));
    for f in FAMILIES {
        for mode in ["direct", "exact", "dependencies"] {
            let rs = rules(f);
            let p = Prepared::new(mode, &rs);
            let mut running = p.start(&query(f, 0));
            oracle::same_raw(execute(&mut running, false).0, expected(f, 0));
            let Running::Cache(s) = running else {
                unreachable!()
            };
            println!(
                "{{\"family\":\"{f}\",\"mode\":\"{mode}\",\"hits\":{},\"equations\":{},\"pairs\":{}}}",
                s.hits(),
                s.source_stats().equations,
                s.source_stats().pairs
            );
        }
    }
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    if a.get(1).map(String::as_str) == Some("work") {
        work();
        return;
    }
    assert!(
        !std::hint::black_box(chr_persistent::COLLECT_KERNEL_METRICS)
            && !std::hint::black_box(chr_compiled::COLLECT_METRICS)
    );
    if a.get(1).map(String::as_str) == Some("gate") {
        gate(a.get(2).map(String::as_str), a.get(3).map(String::as_str));
    } else {
        assert_eq!(a.len(), 4);
        cell(&a[1], &a[2], a[3].parse().unwrap());
    }
}
