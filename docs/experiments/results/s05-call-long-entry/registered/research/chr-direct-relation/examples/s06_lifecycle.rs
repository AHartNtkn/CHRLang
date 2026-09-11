use chr_syntax::{Answer, Query, Rule, Term, Var, and, atom, c, eq, or, v};
use std::time::Instant;
#[cfg(feature = "alloc-meter")]
// Shared meter has optional fork instrumentation unused by this package.
#[allow(unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
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
const MODES: [&str; 3] = ["direct", "global-scan", "global-index"];
const FAMILIES: [&str; 6] = [
    "sparse",
    "dense",
    "duplicate",
    "disconnected",
    "contradiction",
    "tiny",
];
fn table(name: &str, rows: &[Vec<Term>]) -> Rule {
    let body = rows
        .iter()
        .map(|row| {
            and(row
                .iter()
                .enumerate()
                .map(|(i, t)| eq(v(i as u64), t.clone()))
                .collect::<Vec<_>>())
        })
        .reduce(or)
        .unwrap();
    Rule::simplify(
        name,
        [c(
            name,
            (0..rows[0].len()).map(|i| v(i as u64)).collect::<Vec<_>>(),
        )],
        body,
    )
}
fn data(f: &str) -> (Vec<Rule>, Vec<Vec<Term>>, Vec<Vec<Term>>) {
    let n = if f == "tiny" {
        1
    } else if f == "sparse" {
        32
    } else {
        4
    };
    let mut left = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if f == "dense" || i == j {
                left.push(vec![atom(&format!("k{i}")), atom(&format!("k{j}"))]);
            }
        }
    }
    let right = left.clone();
    if f == "duplicate" {
        left.extend(left.clone());
    }
    let rules = vec![
        Rule::simplify(
            "request",
            [c("request", [v(0), v(1), v(2), v(3)])],
            and(vec![
                c("ab", [v(0), v(1)]).into(),
                c("bc", [if f == "disconnected" { v(4) } else { v(1) }, v(2)]).into(),
            ]),
        ),
        table("ab", &left),
        table("bc", &right),
        table("pin", &[vec![atom("k0")]]),
    ];
    (rules, left, right)
}
fn input(f: &str, variant: usize, left: &[Vec<Term>], right: &[Vec<Term>]) -> (Query, Vec<Answer>) {
    let repeated = variant == 2;
    let mut constraints = vec![
        c(
            "request",
            [
                if f == "contradiction" {
                    atom("k99")
                } else {
                    v(10)
                },
                v(11),
                if repeated { v(10) } else { v(12) },
                v(13),
            ],
        ),
        c("carry", [v(13), v(13)]),
    ];
    if variant == 1 {
        constraints.push(c("pin", [v(10)]));
    }
    if variant == 3 {
        constraints.push(c("pin", [v(11)]));
    }
    let outputs = vec![
        ("x".into(), Var(10)),
        ("y".into(), Var(11)),
        ("z".into(), if repeated { Var(10) } else { Var(12) }),
        ("u".into(), Var(13)),
    ];
    let mut expected = Vec::new();
    for a in left {
        for b in right {
            if f == "contradiction"
                || (f != "disconnected" && a[1] != b[0])
                || (repeated && a[0] != b[1])
                || (variant == 1 && a[0] != atom("k0"))
                || (variant == 3 && a[1] != atom("k0"))
            {
                continue;
            }
            expected.push(Answer {
                outputs: vec![
                    ("x".into(), a[0].clone()),
                    ("y".into(), a[1].clone()),
                    ("z".into(), b[1].clone()),
                    ("u".into(), v(13)),
                ],
                residual: vec![c("carry", [v(13), v(13)])],
            });
        }
    }
    (
        Query {
            constraints,
            outputs,
        },
        expected,
    )
}
enum Prepared {
    Direct(std::sync::Arc<chr_direct_relation::Prepared>),
    Dedicated(chr_compiled::PreparedRuleset, chr_compiled::Access),
}
#[allow(clippy::large_enum_variant)]
enum Running {
    Direct(chr_direct_relation::Engine),
    Dedicated(chr_compiled::SearchEngine),
}
impl Prepared {
    fn new(mode: &str, rules: &[Rule]) -> Self {
        if mode == "direct" {
            Self::Direct(chr_direct_relation::Prepared::new(rules, ("request", 4)).unwrap())
        } else {
            Self::Dedicated(
                chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap(),
                if mode == "global-scan" {
                    chr_compiled::Access::Scan
                } else {
                    chr_compiled::Access::Indexed
                },
            )
        }
    }
    fn start(&self, q: Query) -> Running {
        match self {
            Self::Direct(p) => Running::Direct(p.start(&q).unwrap()),
            Self::Dedicated(p, a) => {
                Running::Dedicated(p.start_search(q, chr_compiled::Policy::Global, *a).unwrap())
            }
        }
    }
}
impl Running {
    fn collect(&mut self) -> (Vec<Answer>, Option<u128>) {
        let start = Instant::now();
        let mut first = None;
        let mut answers = Vec::new();
        for _ in 0..2_000_000 {
            let mut exhausted = false;
            let answer = match self {
                Self::Direct(e) => match e.advance() {
                    chr_direct_relation::Step::Answer(a) => Some(a),
                    chr_direct_relation::Step::Exhausted => {
                        exhausted = true;
                        None
                    }
                    _ => None,
                },
                Self::Dedicated(e) => match e.tick() {
                    chr_compiled::SearchEvent::Complete(mut b) => Some(b.engine.observe().unwrap()),
                    chr_compiled::SearchEvent::Exhausted => {
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
                assert!(answers.len() <= 10000);
            }
            if exhausted {
                return (answers, first);
            }
        }
        panic!("service cutoff")
    }
}
fn gate() {
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    for f in FAMILIES {
        let (rules, left, right) = data(f);
        for variant in 0..4 {
            let (q, expected) = input(f, variant, &left, &right);
            oracle::same_raw(oracle::run(&rules, &q, 2_000_000), expected.clone());
            for mode in MODES {
                let p = Prepared::new(mode, &rules);
                oracle::same_raw(p.start(q.clone()).collect().0, expected.clone());
            }
        }
    }
    println!("24 scalar and 72 engine configurations passed");
}
fn cell(mode: &str, f: &str, reuse: usize) {
    assert!(MODES.contains(&mode) && FAMILIES.contains(&f) && [1, 16].contains(&reuse));
    let (rules, left, right) = data(f);
    let inputs = (0..4)
        .map(|i| input(f, i, &left, &right))
        .collect::<Vec<_>>();
    let warm = Prepared::new(mode, &rules);
    for (q, expected) in &inputs {
        oracle::same_raw(warm.start(q.clone()).collect().0, expected.clone());
    }
    drop(warm);
    let (prepared, preparation) = measure(|| Prepared::new(mode, &rules));
    let mut rows = Vec::with_capacity(reuse);
    for i in 0..reuse {
        let (q, expected) = &inputs[i % 4];
        let q = q.clone();
        let (mut engine, setup) = measure(|| prepared.start(q));
        let ((answers, first), execution) = measure(|| engine.collect());
        let count = answers.len();
        let (_, disposal) = measure(|| drop(engine));
        oracle::same_raw(answers.clone(), expected.clone());
        let (_, answer_disposal) = measure(|| drop(answers));
        rows.push(format!("{{\"variant\":{},\"count\":{count},\"setup\":{},\"execution_observation\":{},\"engine_disposal\":{},\"answer_disposal\":{},\"first_ns\":{}}}",i%4,setup.json(),execution.json(),disposal.json(),answer_disposal.json(),first.map_or("null".into(),|n|n.to_string())));
    }
    let (_, disposal) = measure(|| drop(prepared));
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{f}\",\"reuse\":{reuse},\"meter\":{},\"preparation\":{},\"prepared_disposal\":{},\"queries\":[{}]}}",
        cfg!(feature = "alloc-meter"),
        preparation.json(),
        disposal.json(),
        rows.join(",")
    );
}
fn main() {
    const {
        assert!(
            !chr_compiled::COLLECT_METRICS,
            "counter-free build required"
        );
    }
    let args = std::env::args().collect::<Vec<_>>();
    if args.get(1).map(String::as_str) == Some("gate") {
        gate();
    } else {
        assert_eq!(args.len(), 4);
        cell(&args[1], &args[2], args[3].parse().unwrap());
    }
}
