#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use chr_syntax::{Answer, Query, Rule};
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
#[path = "../experiments/subscription_cases.rs"]
mod cases;
#[path = "../experiments/subscription_join.rs"]
mod join;
#[path = "../experiments/subscription_runtime.rs"]
mod runtime;
#[path = "../experiments/subscription_source.rs"]
mod source;
const MODES: [&str; 4] = ["indexed", "eager", "subscribed", "compiled"];
enum Prepared {
    Lower(runtime::Prepared, join::Mode),
    Generic(chr_compiled::PreparedRuleset),
}
#[allow(clippy::large_enum_variant)]
enum Running {
    Lower(runtime::Engine),
    Generic(chr_compiled::Engine),
}
impl Prepared {
    fn new(mode: &str, rules: &[Rule]) -> Self {
        match mode {
            "indexed" | "eager" | "subscribed" => Self::Lower(
                runtime::Prepared::new(rules).unwrap(),
                match mode {
                    "indexed" => join::Mode::Indexed,
                    "eager" => join::Mode::Eager,
                    _ => join::Mode::Subscribed,
                },
            ),
            "compiled" => {
                Self::Generic(chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap())
            }
            _ => panic!("unknown mode"),
        }
    }
    fn start(&self, q: Query) -> Running {
        match self {
            Self::Lower(p, m) => Running::Lower(p.start(q, *m).unwrap()),
            Self::Generic(p) => Running::Generic(
                p.start(
                    q,
                    chr_compiled::Policy::Global,
                    chr_compiled::Access::Indexed,
                )
                .unwrap(),
            ),
        }
    }
}
impl Running {
    fn advance(&mut self, budget: usize) -> bool {
        match self {
            Self::Lower(e) => e.advance(budget),
            Self::Generic(e) => {
                let s = e.advance(budget);
                assert!(!s.failed);
                s.exhausted
            }
        }
    }
    fn execute(&mut self) {
        assert!(self.advance(2_000_000), "source service cutoff");
    }
    fn observe(&mut self) -> Answer {
        match self {
            Self::Lower(e) => {
                let mut a = e.observe().unwrap();
                assert_eq!(a.len(), 1);
                a.pop().unwrap()
            }
            Self::Generic(e) => e.observe().unwrap(),
        }
    }
    fn work(&self) -> (usize, usize, usize, usize) {
        match self {
            Self::Lower(e) => (
                e.stats().constructed,
                e.stats().invalidated,
                e.stats().probes,
                e.retained(),
            ),
            Self::Generic(_) => (0, 0, 0, 0),
        }
    }
}
fn inputs(f: &str, n: usize, r: usize, rules: &[Rule]) -> Vec<(Query, Answer)> {
    (0..2)
        .map(|seed| {
            let q = cases::query(f, n, r, seed);
            let mut a = oracle::run(rules, &q, 2_000_000);
            assert_eq!(a.len(), 1);
            let a = a.pop().unwrap();
            assert_eq!(
                a.residual.iter().filter(|c| c.name == "receipt").count(),
                cases::receipts(f, n, r)
            );
            (q, a)
        })
        .collect()
}
fn cell(mode: &str, f: &str, n: usize, r: usize, gate: bool, work: bool) {
    assert!(MODES.contains(&mode));
    let rules = source::source_rules(f == "consuming");
    let inputs = inputs(f, n, r, &rules);
    if gate || work {
        let p = Prepared::new(mode, &rules);
        for (q, a) in &inputs {
            let mut e = p.start(q.clone());
            e.execute();
            oracle::same_raw(vec![e.observe()], vec![a.clone()]);
            if work {
                let (built, invalidated, probes, retained) = e.work();
                println!(
                    "{{\"constructed\":{built},\"invalidated\":{invalidated},\"probes\":{probes},\"retained\":{retained}}}"
                );
            }
        }
        if gate {
            println!("complete answers pass");
        }
        return;
    }
    let warm = Prepared::new(mode, &rules);
    for (q, a) in &inputs {
        let mut e = warm.start(q.clone());
        e.execute();
        oracle::same_raw(vec![e.observe()], vec![a.clone()]);
    }
    drop(warm);
    let mut rows = Vec::with_capacity(2);
    let (p, preparation) = measure(|| Prepared::new(mode, &rules));
    for (q, a) in &inputs {
        let (mut e, setup) = measure(|| p.start(q.clone()));
        let (_, execution) = measure(|| e.execute());
        let (answer, observation) = measure(|| e.observe());
        let (_, disposal) = measure(|| drop(e));
        oracle::same_raw(vec![answer.clone()], vec![a.clone()]);
        let (_, answer_disposal) = measure(|| drop(answer));
        rows.push((setup, execution, observation, disposal, answer_disposal));
    }
    let (mut cancel, cancel_setup) = measure(|| p.start(inputs[0].0.clone()));
    let (_, cancel_execution) = measure(|| cancel.advance(32));
    let (_, cancel_disposal) = measure(|| drop(cancel));
    let (_, prepared_disposal) = measure(|| drop(p));
    let rows=rows.into_iter().map(|(s,e,o,d,a)|format!("{{\"setup\":{},\"execution\":{},\"observation\":{},\"engine_disposal\":{},\"answer_disposal\":{}}}",s.json(),e.json(),o.json(),d.json(),a.json())).collect::<Vec<_>>();
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{f}\",\"n\":{n},\"rounds\":{r},\"meter\":{},\"preparation\":{},\"prepared_disposal\":{},\"cancellation\":{{\"setup\":{},\"execution\":{},\"disposal\":{}}},\"queries\":[{}]}}",
        cfg!(feature = "alloc-meter"),
        preparation.json(),
        prepared_disposal.json(),
        cancel_setup.json(),
        cancel_execution.json(),
        cancel_disposal.json(),
        rows.join(",")
    );
}
fn main() {
    let a = std::env::args().collect::<Vec<_>>();
    assert_eq!(a.len(), 6);
    let work = a[1] == "work";
    let gate = a[1] == "gate";
    assert!(work || gate || a[1] == "cell");
    assert_eq!(std::hint::black_box(chr_compiled::COLLECT_METRICS), work);
    if !work {
        assert!(!std::hint::black_box(
            chr_persistent::COLLECT_KERNEL_METRICS
        ));
    }
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    cell(
        &a[2],
        &a[3],
        a[4].parse().unwrap(),
        a[5].parse().unwrap(),
        gate,
        work,
    );
}
