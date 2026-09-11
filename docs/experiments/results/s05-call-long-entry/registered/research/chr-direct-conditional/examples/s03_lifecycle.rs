//! Registered S03 isolated-process lifecycle pilot. No timings before the freeze.
use chr_syntax::{Answer, Query, Rule, Term, Var, atom, c, eq, or, t, v};
use std::time::Instant;
#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../tests/runtime_support/mod.rs"]
mod oracle;
#[derive(Clone, Copy)]
struct Measurement {
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn measure<T>(f: impl FnOnce() -> T) -> (T, Measurement) {
    #[cfg(feature = "alloc-meter")]
    let memory = meter::begin();
    let start = Instant::now();
    let value = f();
    let ns = start.elapsed().as_nanos();
    #[cfg(feature = "alloc-meter")]
    let memory = meter::end(memory);
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
        let memory = self.memory.json();
        #[cfg(not(feature = "alloc-meter"))]
        let memory = "null";
        format!("{{\"ns\":{},\"memory\":{}}}", self.ns, memory)
    }
}
fn alphabet(family: &str) -> Option<chr_direct_choice::words::Alphabet> {
    use chr_direct_choice::words::Alphabet;
    match family {
        "binary" => Some(Alphabet::Binary),
        "nested" => Some(Alphabet::Nested),
        "duplicate" => Some(Alphabet::Duplicate),
        _ => None,
    }
}
fn depths(family: &str) -> [usize; 4] {
    match family {
        "binary" => [0, 2, 4, 2],
        "nested" => [0, 1, 3, 1],
        "duplicate" => [0, 2, 5, 2],
        "plain" | "shared" | "discriminate" => [0, 1, 8, 32],
        _ => panic!("unknown family"),
    }
}
fn rules(family: &str) -> Vec<Rule> {
    if let Some(a) = alphabet(family) {
        return chr_direct_choice::words::source(a, false);
    }
    let mut rules = vec![Rule::simplify(
        "choose",
        [c("choose", [v(0)])],
        or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
    )];
    if family == "discriminate" {
        for bits in 0..16 {
            let pack = t(
                "pack",
                (0..4)
                    .map(|i| atom(if bits & (1 << i) == 0 { "a" } else { "b" }))
                    .collect::<Vec<_>>(),
            );
            rules.push(Rule::simplify(
                &format!("gate{bits}"),
                [c("gate", [v(0), pack.clone()])],
                c("work", [v(0), pack]).into(),
            ));
        }
    }
    rules.push(Rule::simplify(
        "step",
        [c("work", [t("s", [v(0)]), v(1)])],
        c("work", [v(0), v(1)]).into(),
    ));
    rules.push(Rule::simplify(
        "base",
        [c("work", [atom("z"), v(0)])],
        c("result", [v(0)]).into(),
    ));
    rules
}
fn query(family: &str, n: usize) -> Query {
    let depth = (0..n).fold(atom("z"), |x, _| t("s", [x]));
    if alphabet(family).is_some() {
        return Query {
            constraints: vec![
                c("token", [v(200)]),
                c("build", [depth, v(100)]),
                c("task", [v(100), v(100)]),
            ],
            outputs: vec![],
        };
    }
    let mut constraints = Vec::new();
    if family != "plain" {
        constraints.extend((0..4).map(|i| c("choose", [v(i)])));
    }
    constraints.push(c(
        if family == "discriminate" {
            "gate"
        } else {
            "work"
        },
        [depth, t("pack", (0..4).map(v).collect::<Vec<_>>())],
    ));
    Query {
        constraints,
        outputs: (0..4).map(|i| (format!("o{i}"), Var(i))).collect(),
    }
}
fn expected(family: &str, n: usize) -> Vec<Answer> {
    if alphabet(family).is_some() {
        // Independent mathematical product, deliberately unlike the compiled
        // mixed-radix iterator and the source evaluator.
        let letters = match family {
            "binary" => vec!["a", "b"],
            "nested" => vec!["a", "b", "c"],
            "duplicate" => vec!["a", "a"],
            _ => unreachable!(),
        };
        let mut words = vec![atom("nil")];
        for _ in 0..n {
            words = letters
                .iter()
                .flat_map(|letter| {
                    words
                        .iter()
                        .map(move |tail| t("cons", [atom(letter), tail.clone()]))
                })
                .collect();
        }
        return words
            .into_iter()
            .map(|word| Answer {
                outputs: vec![],
                residual: vec![
                    c("seen", [v(900)]),
                    c(
                        "out",
                        [
                            word.clone(),
                            word.clone(),
                            word.clone(),
                            word,
                            v(900),
                            v(900),
                        ],
                    ),
                ],
            })
            .collect();
    }
    let fields: Vec<Vec<Term>> = if family == "plain" {
        vec![(900..904).map(v).collect()]
    } else {
        (0..16)
            .map(|bits| {
                (0..4)
                    .map(|i| atom(if bits & (1 << i) == 0 { "a" } else { "b" }))
                    .collect()
            })
            .collect()
    };
    fields
        .into_iter()
        .map(|fields| Answer {
            outputs: fields
                .iter()
                .enumerate()
                .map(|(i, t)| (format!("o{i}"), t.clone()))
                .collect(),
            residual: vec![c("result", [t("pack", fields)])],
        })
        .collect()
}
enum Prepared {
    Graph(chr_direct_choice::engine::PreparedRuleset),
    Conditional(chr_direct_conditional::engine::PreparedRuleset),
    Compiled(
        chr_compiled::PreparedRuleset,
        chr_compiled::Policy,
        chr_compiled::Access,
    ),
    Words(chr_direct_choice::words::Prepared),
}
// Stack wrapper avoids imposing another allocation on any engine.
#[allow(clippy::large_enum_variant)]
enum Running {
    Graph(chr_direct_choice::engine::Engine),
    Conditional(chr_direct_conditional::engine::Engine),
    Compiled(chr_compiled::SearchEngine),
    Words(chr_direct_choice::words::Words),
}
impl Prepared {
    fn new(mode: &str, rules: &[Rule]) -> Self {
        match mode {
            "graph" => Self::Graph(
                chr_direct_choice::engine::PreparedRuleset::new(rules.to_vec()).unwrap(),
            ),
            "conditional" => Self::Conditional(
                chr_direct_conditional::engine::PreparedRuleset::new(rules.to_vec()).unwrap(),
            ),
            "words" => Self::Words(chr_direct_choice::words::Prepared::new(rules).unwrap()),
            "global-scan" | "global-index" | "active-index" => Self::Compiled(
                chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap(),
                if mode == "active-index" {
                    chr_compiled::Policy::Active
                } else {
                    chr_compiled::Policy::Global
                },
                if mode == "global-scan" {
                    chr_compiled::Access::Scan
                } else {
                    chr_compiled::Access::Indexed
                },
            ),
            _ => panic!("unknown mode"),
        }
    }
    fn start(&self, query: Query) -> Running {
        match self {
            Self::Graph(p) => Running::Graph(p.start(query).unwrap()),
            Self::Conditional(p) => Running::Conditional(p.start(query).unwrap()),
            Self::Compiled(p, policy, access) => {
                Running::Compiled(p.start_search(query, *policy, *access).unwrap())
            }
            Self::Words(p) => Running::Words(p.start(query).unwrap()),
        }
    }
}
impl Running {
    fn collect(&mut self) -> (Vec<Answer>, u128) {
        let start = Instant::now();
        let mut first = None;
        let mut answers = Vec::new();
        for _ in 0..2_000_000 {
            let mut exhausted = false;
            let answer = match self {
                Self::Graph(e) => match e.tick() {
                    chr_direct_choice::engine::Event::Progress => None,
                    chr_direct_choice::engine::Event::Answer(a) => Some(a),
                    chr_direct_choice::engine::Event::Exhausted => {
                        exhausted = true;
                        None
                    }
                },
                Self::Conditional(e) => match e.tick() {
                    chr_direct_conditional::engine::Event::Progress => None,
                    chr_direct_conditional::engine::Event::Answer(a) => Some(a),
                    chr_direct_conditional::engine::Event::Exhausted => {
                        exhausted = true;
                        None
                    }
                },
                Self::Compiled(e) => match e.tick() {
                    chr_compiled::SearchEvent::Complete(mut branch) => {
                        Some(branch.engine.observe().unwrap())
                    }
                    chr_compiled::SearchEvent::Exhausted => {
                        exhausted = true;
                        None
                    }
                    _ => None,
                },
                Self::Words(e) => {
                    let next = e.next();
                    exhausted = next.is_none();
                    next
                }
            };
            if let Some(answer) = answer {
                if first.is_none() {
                    first = Some(start.elapsed().as_nanos());
                }
                answers.push(answer);
                assert!(answers.len() <= 10_000, "answer bound");
            }
            if exhausted {
                return (answers, first.expect("registered sources have answers"));
            }
        }
        panic!("service cutoff is not exhaustion")
    }
}
fn modes(family: &str) -> Vec<&'static str> {
    if alphabet(family).is_some() {
        vec![
            "global-scan",
            "global-index",
            "active-index",
            "conditional",
            "graph",
            "words",
        ]
    } else {
        vec!["global-scan", "global-index", "conditional", "graph"]
    }
}
fn cell(mode: &str, family: &str) -> String {
    assert!(modes(family).contains(&mode), "unregistered cell");
    let source = rules(family);
    let inputs: Vec<_> = depths(family)
        .into_iter()
        .map(|n| (n, query(family, n), expected(family, n)))
        .collect();
    let warm = Prepared::new(mode, &source);
    for (_, q, expected) in &inputs {
        let mut e = warm.start(q.clone());
        let (actual, _) = e.collect();
        oracle::same_raw(actual, expected.clone());
    }
    drop(warm);
    let (prepared, preparation) = measure(|| Prepared::new(mode, &source));
    let mut rows = Vec::with_capacity(16);
    for cycle in 0..4 {
        for (depth, q, expected) in &inputs {
            let (mut engine, setup) = measure(|| prepared.start(q.clone()));
            let ((answers, first), execution) = measure(|| engine.collect());
            let (_, engine_drop) = measure(|| drop(engine));
            // Validation copies are excluded. Keep original consumer-owned answers
            // alive until their separately measured disposal.
            oracle::same_raw(answers.clone(), expected.clone());
            let raw = answers.len();
            let (_, answer_drop) = measure(|| drop(answers));
            rows.push((
                cycle,
                *depth,
                raw,
                first,
                setup,
                execution,
                engine_drop,
                answer_drop,
            ));
        }
    }
    let (_, prepared_drop) = measure(|| drop(prepared));
    let total = preparation.ns
        + prepared_drop.ns
        + rows
            .iter()
            .map(|r| r.4.ns + r.5.ns + r.6.ns + r.7.ns)
            .sum::<u128>();
    let queries=rows.into_iter().map(|(cycle,depth,raw,first,setup,execution,engine_drop,answer_drop)|format!("{{\"cycle\":{cycle},\"depth\":{depth},\"raw\":{raw},\"first_ns\":{first},\"setup\":{},\"execution_observation\":{},\"engine_drop\":{},\"answer_drop\":{}}}",setup.json(),execution.json(),engine_drop.json(),answer_drop.json())).collect::<Vec<_>>().join(",");
    format!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"metered\":{},\"total_ns\":{total},\"preparation\":{},\"prepared_drop\":{},\"queries\":[{queries}]}}",
        cfg!(feature = "alloc-meter"),
        preparation.json(),
        prepared_drop.json()
    )
}
fn main() {
    assert!(
        std::hint::black_box(
            !cfg!(feature = "metrics")
                && !chr_compiled::COLLECT_METRICS
                && !chr_compiled::COLLECT_KERNEL_METRICS
        ),
        "pilot requires counters off"
    );
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("validate") => {
            for family in [
                "binary",
                "nested",
                "duplicate",
                "plain",
                "shared",
                "discriminate",
            ] {
                let source = rules(family);
                for depth in depths(family) {
                    oracle::same_raw(
                        oracle::run(&source, &query(family, depth), 200_000),
                        expected(family, depth),
                    );
                }
                for mode in modes(family) {
                    std::hint::black_box(cell(mode, family));
                }
            }
            println!(
                "validated all 30 cells and independent source expectations; no comparative timings emitted"
            );
        }
        Some("calibrate") => {
            let mut samples: Vec<_> = (0..10001)
                .map(|_| measure(|| std::hint::black_box(())).1.ns)
                .collect();
            samples.sort();
            println!(
                "{{\"samples\":10001,\"median_ns\":{},\"p99_ns\":{}}}",
                samples[5000], samples[9900]
            );
        }
        #[cfg(feature = "alloc-meter")]
        Some("meter-check") => {
            meter::self_check().unwrap();
            println!("meter checked");
        }
        Some(mode) if args.len() == 2 => println!("{}", cell(mode, &args[1])),
        _ => panic!("expected validate, calibrate, meter-check, or MODE FAMILY"),
    }
}
