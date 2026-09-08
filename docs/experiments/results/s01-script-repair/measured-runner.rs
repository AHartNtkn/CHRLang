#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use chr_syntax::{Answer, Query, Rule, Var, atom, c, t, v};
use std::time::Instant;
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
use chr_compiled::selective_join::{self, Mode};
const MODES: [&str; 4] = ["direct", "retained", "global-scan", "global-index"];
const FAMILIES: [&str; 8] = [
    "stable-selective",
    "stable-dense",
    "replace-sparse",
    "replace-broad",
    "bind-sparse",
    "bind-broad",
    "consume-selective",
    "consume-dense",
];
fn query(f: &str, n: usize, rounds: usize) -> Query {
    let dense = f.ends_with("dense");
    let binding = f.starts_with("bind");
    let broad = f.ends_with("broad");
    let consuming = f.starts_with("consume");
    let mut rows = Vec::new();
    for i in 0..n {
        let child = if binding && (broad || i == 0) {
            v(50)
        } else {
            atom(&format!(
                "v{}",
                if dense {
                    0
                } else if binding {
                    i + 1
                } else {
                    i
                }
            ))
        };
        rows.push(c("left", [atom("k"), t("f", [child])]));
        rows.push(c("right", [atom("k"), t("g", [atom("v0")])]));
    }
    let mut ops = Vec::new();
    if binding {
        ops.push(t("req", [atom("k"), atom("before")]));
        ops.push(t("bind", [v(50), atom("v0")]));
    }
    for round in 0..rounds {
        if f.starts_with("replace") {
            for _ in 0..if broad { n } else { 1 } {
                let (old, new) = if round % 2 == 0 {
                    ("v0", "v1")
                } else {
                    ("v1", "v0")
                };
                ops.push(t(
                    "replace",
                    [atom("k"), t("g", [atom(old)]), t("g", [atom(new)])],
                ));
            }
        }
        if consuming && round > 0 {
            for _ in 0..n {
                ops.push(t("insert", [atom("k"), t("g", [atom("v0")])]));
            }
        }
        ops.push(t("req", [atom("k"), atom(&format!("r{round}"))]));
    }
    let script = ops
        .into_iter()
        .rev()
        .fold(atom("nil"), |tail, op| t("cons", [op, tail]));
    rows.push(c("drive", [script]));
    Query {
        constraints: rows,
        outputs: vec![("unknown".into(), Var(50))],
    }
}
enum Prepared {
    Lower(selective_join::Prepared, Mode),
    Generic(chr_compiled::PreparedRuleset, chr_compiled::Access),
}
#[allow(clippy::large_enum_variant)]
enum Running {
    Lower(selective_join::Engine),
    Generic(chr_compiled::Engine),
}
impl Prepared {
    fn new(mode: &str, rules: &[Rule]) -> Self {
        match mode {
            "direct" | "retained" => Self::Lower(
                selective_join::Prepared::new(rules).unwrap(),
                if mode == "direct" {
                    Mode::Direct
                } else {
                    Mode::Retained
                },
            ),
            _ => Self::Generic(
                chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap(),
                if mode == "global-scan" {
                    chr_compiled::Access::Scan
                } else {
                    chr_compiled::Access::Indexed
                },
            ),
        }
    }
    fn start(&self, q: Query) -> Running {
        match self {
            Self::Lower(p, m) => Running::Lower(p.start(q, *m).unwrap()),
            Self::Generic(p, a) => {
                Running::Generic(p.start(q, chr_compiled::Policy::Global, *a).unwrap())
            }
        }
    }
}
impl Running {
    fn execute(&mut self) {
        match self {
            Self::Lower(e) => assert!(e.advance(2_000_000)),
            Self::Generic(e) => {
                let s = e.advance(2_000_000);
                assert!(s.exhausted && !s.failed);
            }
        }
    }
    fn observe(&mut self) -> Answer {
        match self {
            Self::Lower(e) => e.observe().unwrap(),
            Self::Generic(e) => e.observe().unwrap(),
        }
    }
}
fn inputs(f: &str, rules: &[Rule]) -> Vec<(usize, usize, Query, Answer)> {
    [(4, 1), (4, 8), (8, 1), (8, 8)]
        .into_iter()
        .map(|(n, r)| {
            let q = query(f, n, r);
            let mut expected = oracle::run(rules, &q, 2_000_000);
            assert_eq!(expected.len(), 1);
            (n, r, q, expected.pop().unwrap())
        })
        .collect()
}
fn gate() {
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    for f in FAMILIES {
        let rules = selective_join::source_rules(f.starts_with("consume"));
        let inputs = inputs(f, &rules);
        for mode in MODES {
            let p = Prepared::new(mode, &rules);
            for (_, _, q, expected) in &inputs {
                let mut e = p.start(q.clone());
                e.execute();
                oracle::same_raw(vec![e.observe()], vec![expected.clone()]);
            }
        }
    }
    println!("128 engine configurations pass complete independent source observations");
}
fn cell(mode: &str, f: &str) {
    assert!(MODES.contains(&mode) && FAMILIES.contains(&f));
    let rules = selective_join::source_rules(f.starts_with("consume"));
    let inputs = inputs(f, &rules);
    let warm = Prepared::new(mode, &rules);
    for (_, _, q, expected) in &inputs {
        let mut e = warm.start(q.clone());
        e.execute();
        oracle::same_raw(vec![e.observe()], vec![expected.clone()]);
    }
    drop(warm);
    let (p, preparation) = measure(|| Prepared::new(mode, &rules));
    let mut rows = Vec::with_capacity(8);
    for _ in 0..2 {
        for (n, r, q, expected) in &inputs {
            let q = q.clone();
            let (mut e, setup) = measure(|| p.start(q));
            let (_, execution) = measure(|| e.execute());
            let (answer, observation) = measure(|| e.observe());
            let (_, disposal) = measure(|| drop(e));
            oracle::same_raw(vec![answer.clone()], vec![expected.clone()]);
            let (_, answer_disposal) = measure(|| drop(answer));
            rows.push(format!("{{\"n\":{n},\"rounds\":{r},\"setup\":{},\"execution\":{},\"observation\":{},\"engine_disposal\":{},\"answer_disposal\":{}}}",setup.json(),execution.json(),observation.json(),disposal.json(),answer_disposal.json()));
        }
    }
    let (_, disposal) = measure(|| drop(p));
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{f}\",\"meter\":{},\"preparation\":{},\"prepared_disposal\":{},\"queries\":[{}]}}",
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
