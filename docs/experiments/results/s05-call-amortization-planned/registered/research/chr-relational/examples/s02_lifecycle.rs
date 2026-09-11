use chr_syntax::{Answer, Query, Rule, atom, c, eq, t, v};
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
const MODES: [&str; 4] = ["relational", "integrated", "global-index", "global-scan"];
const FAMILIES: [&str; 5] = [
    "flat",
    "selective",
    "dense",
    "delayed-selective",
    "delayed-dense",
];
fn rules(family: &str) -> Vec<Rule> {
    let pattern = if family == "flat" {
        v(0)
    } else {
        t("f", [atom("a"), v(0)])
    };
    vec![
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
        Rule::simplify(
            "join",
            [c("open", [pattern]), c("ticket", [v(0)])],
            c("done", [v(0)]).into(),
        ),
    ]
}
fn input(family: &str, n: usize) -> (Query, Answer) {
    let mut constraints = Vec::new();
    let mut residual = Vec::new();
    for i in 0..n {
        let key = atom(&format!("key{i}"));
        let matched = family == "flat" || family.ends_with("dense") || i == 0;
        let value = if family == "flat" {
            key.clone()
        } else {
            t("f", [atom(if matched { "a" } else { "b" }), key.clone()])
        };
        if family.starts_with("delayed") {
            constraints.push(c("open", [v(100 + i as u64)]));
            constraints.push(c("bind", [v(100 + i as u64), value.clone()]));
        } else {
            constraints.push(c("open", [value.clone()]));
        }
        constraints.push(c("ticket", [key.clone()]));
        if matched {
            residual.push(c("done", [key]));
        } else {
            residual.push(c("open", [value]));
            residual.push(c("ticket", [key]));
        }
    }
    (
        Query {
            constraints,
            outputs: vec![],
        },
        Answer {
            outputs: vec![],
            residual,
        },
    )
}
enum Prepared {
    Relational(std::sync::Arc<chr_relational::execute::Prepared>),
    Integrated(chr_integrated::PreparedRuleset),
    Dedicated(chr_compiled::PreparedRuleset, chr_compiled::Access),
}
#[allow(clippy::large_enum_variant)]
enum Running {
    Relational(chr_relational::execute::Engine),
    Integrated(chr_integrated::Engine),
    Dedicated(chr_compiled::Engine),
}
impl Prepared {
    fn new(mode: &str, rules: &[Rule]) -> Self {
        match mode {
            "relational" => {
                Self::Relational(chr_relational::execute::Prepared::new(rules).unwrap())
            }
            "integrated" => Self::Integrated(chr_integrated::PreparedRuleset::new(rules).unwrap()),
            "global-index" | "global-scan" => Self::Dedicated(
                chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap(),
                if mode == "global-index" {
                    chr_compiled::Access::Indexed
                } else {
                    chr_compiled::Access::Scan
                },
            ),
            _ => panic!("unknown mode"),
        }
    }
    fn start(&self, q: Query) -> Running {
        match self {
            Self::Relational(p) => Running::Relational(p.start(&q)),
            Self::Integrated(p) => Running::Integrated(p.start(&q)),
            Self::Dedicated(p, a) => {
                Running::Dedicated(p.start(q, chr_compiled::Policy::Global, *a).unwrap())
            }
        }
    }
}
impl Running {
    fn collect(&mut self) -> (Answer, u128) {
        let start = Instant::now();
        match self {
            Self::Relational(e) => {
                for _ in 0..2_000_000 {
                    match e.advance() {
                        chr_relational::execute::Step::Answer(a) => {
                            let first = start.elapsed().as_nanos();
                            assert!(matches!(
                                e.advance(),
                                chr_relational::execute::Step::Exhausted
                            ));
                            return (a, first);
                        }
                        chr_relational::execute::Step::Exhausted => panic!("missing answer"),
                        _ => (),
                    }
                }
                panic!("service cutoff")
            }
            Self::Integrated(e) => {
                assert_eq!(e.run(2_000_000), chr_integrated::Step::Complete);
                let a = e.answer().unwrap();
                (a, start.elapsed().as_nanos())
            }
            Self::Dedicated(e) => {
                let status = e.advance(2_000_000);
                assert!(status.exhausted && !status.failed);
                let a = e.observe().unwrap();
                (a, start.elapsed().as_nanos())
            }
        }
    }
}
fn gate() {
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    for family in FAMILIES {
        let source = rules(family);
        for n in [4, 16, 64] {
            let (q, expected) = input(family, n);
            oracle::same_raw(oracle::run(&source, &q, 2_000_000), vec![expected.clone()]);
            for mode in MODES {
                let p = Prepared::new(mode, &source);
                let mut e = p.start(q.clone());
                oracle::same_raw(vec![e.collect().0], vec![expected.clone()]);
            }
        }
    }
    println!("semantic gate: 60 engine cases and 15 scalar cases pass");
}
fn cell(mode: &str, family: &str) {
    assert!(MODES.contains(&mode) && FAMILIES.contains(&family));
    let source = rules(family);
    let inputs = [4, 16, 64, 16].map(|n| (n, input(family, n)));
    let warm = Prepared::new(mode, &source);
    for (_, (q, expected)) in &inputs {
        let mut e = warm.start(q.clone());
        oracle::same_raw(vec![e.collect().0], vec![expected.clone()]);
    }
    drop(warm);
    let (p, preparation) = measure(|| Prepared::new(mode, &source));
    let mut rows = Vec::with_capacity(16);
    for _ in 0..4 {
        for (n, (q, expected)) in &inputs {
            let q = q.clone();
            let (mut e, setup) = measure(|| p.start(q));
            let ((answer, first), execution) = measure(|| e.collect());
            let (_, disposal) = measure(|| drop(e));
            oracle::same_raw(vec![answer.clone()], vec![expected.clone()]);
            let (_, answer_disposal) = measure(|| drop(answer));
            rows.push(format!("{{\"n\":{n},\"setup\":{},\"execution_observation\":{},\"first_ns\":{first},\"engine_disposal\":{},\"answer_disposal\":{}}}",setup.json(),execution.json(),disposal.json(),answer_disposal.json()));
        }
    }
    let (_, disposal) = measure(|| drop(p));
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"meter\":{},\"preparation\":{},\"prepared_disposal\":{},\"queries\":[{}]}}",
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
        assert_eq!(args.len(), 3);
        cell(&args[1], &args[2]);
    }
}
