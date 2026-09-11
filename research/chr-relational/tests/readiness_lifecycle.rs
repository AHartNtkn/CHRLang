#[cfg(feature = "admission-profile")]
#[path = "../examples/support/deduction_profile.rs"]
mod profile;
// One isolated, fully disposed lifecycle sample; validation is outside phases.
#[cfg(feature = "alloc-meter")]
#[allow(dead_code, unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
#[path = "support/readiness_source.rs"]
mod source;
use chr_relational::execute::{Engine, Prepared, Step};
use chr_syntax::{Answer, Query, Rule};
use std::{hint::black_box, sync::Arc, time::Instant};

#[derive(Clone, Copy)]
struct Phase {
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    heap: meter::Reading,
}
fn phase<T>(f: impl FnOnce() -> T) -> (T, Phase) {
    #[cfg(feature = "alloc-meter")]
    let start = meter::begin();
    let clock = Instant::now();
    let result = black_box(f());
    let ns = clock.elapsed().as_nanos();
    #[cfg(feature = "alloc-meter")]
    let heap = meter::end(start);
    (
        result,
        Phase {
            ns,
            #[cfg(feature = "alloc-meter")]
            heap,
        },
    )
}
impl Phase {
    fn json(self) -> String {
        #[cfg(feature = "alloc-meter")]
        return format!("{{\"ns\":{},\"heap\":{}}}", self.ns, self.heap.json());
        #[cfg(not(feature = "alloc-meter"))]
        format!("{{\"ns\":{}}}", self.ns)
    }
}
trait Backend {
    type Prepared;
    type Engine;
    fn prepare(rules: &[Rule]) -> Self::Prepared;
    fn start(p: &Self::Prepared, q: &Query) -> Self::Engine;
    fn advance(e: &mut Self::Engine) -> Step;
}
struct Relational<const MODE: usize>;
impl<const MODE: usize> Backend for Relational<MODE> {
    type Prepared = Arc<Prepared>;
    type Engine = Engine;
    fn prepare(rules: &[Rule]) -> Self::Prepared {
        if MODE == 0 {
            Prepared::new(rules)
        } else {
            Prepared::new_ready(rules)
        }
        .unwrap()
    }
    fn start(p: &Self::Prepared, q: &Query) -> Engine {
        p.start(q)
    }
    fn advance(e: &mut Engine) -> Step {
        match MODE {
            0 => e.advance_settled(),
            1 => e.advance_selective(),
            2 => e.advance_ready(8),
            3 => e.advance_ready(256),
            _ => unreachable!(),
        }
    }
}
struct Compiled;
impl Backend for Compiled {
    type Prepared = chr_compiled::PreparedRuleset;
    type Engine = chr_compiled::Engine;
    fn prepare(rules: &[Rule]) -> Self::Prepared {
        chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap()
    }
    fn start(p: &Self::Prepared, q: &Query) -> Self::Engine {
        p.start(
            q.clone(),
            chr_compiled::Policy::Global,
            chr_compiled::Access::Indexed,
        )
        .unwrap()
    }
    fn advance(e: &mut Self::Engine) -> Step {
        let status = e.advance(1);
        assert!(!status.pending_split);
        if status.exhausted {
            e.observe().map_or(Step::Exhausted, Step::Answer)
        } else {
            Step::Progress
        }
    }
}
struct QuerySample {
    setup: Phase,
    execution_observation: Phase,
    engine_drop: Phase,
    // At most one answer per qualified source; keep it after producer disposal.
    answer: Option<Answer>,
    advances: usize,
}
fn run<B: Backend>(depth: usize, shared: bool, outcome: &str, count: usize) {
    let cancel = outcome == "cancel";
    let source_outcome = if cancel { "success" } else { outcome };
    let (rules, _) = source::separated_source(depth, false, shared, source_outcome, 1);
    let queries: Vec<_> = (0..count)
        .map(|i| {
            source::separated_source(depth + i % 2, false, shared, source_outcome, 1 + i % 2).1
        })
        .collect();
    // Scalar validation inputs/answers and result slots are resident before timing.
    let expected: Vec<_> = queries
        .iter()
        .map(|q| scalar::run(&rules, q, 100_000))
        .collect();
    let mut samples = Vec::with_capacity(count);
    #[cfg(feature = "alloc-meter")]
    let baseline = meter::end(meter::begin()).live_end;
    #[cfg(feature = "admission-profile")]
    profile::enable_admission();
    let (prepared, preparation) = phase(|| B::prepare(&rules));
    for q in &queries {
        let (mut engine, setup) = phase(|| B::start(&prepared, q));
        let ((answer, advances), execution_observation) = phase(|| {
            if cancel {
                return (None, 0);
            }
            for turn in 1..=100_000 {
                match B::advance(&mut engine) {
                    Step::Answer(a) => return (Some(a), turn),
                    Step::Exhausted => return (None, turn),
                    Step::Progress => (),
                }
            }
            panic!("source service cutoff")
        });
        let (_, engine_drop) = phase(|| drop(engine));
        samples.push(QuerySample {
            setup,
            execution_observation,
            engine_drop,
            answer,
            advances,
        });
    }
    let (_, preparation_drop) = phase(|| drop(prepared));
    // Full answers are still owned by the consumer after every producer is gone.
    for (sample, oracle) in samples.iter().zip(&expected) {
        if !cancel {
            scalar::same_raw(sample.answer.clone().into_iter().collect(), oracle.clone());
        }
    }
    let (_, consumer_drop) = phase(|| {
        for sample in &mut samples {
            drop(sample.answer.take());
        }
    });
    #[cfg(feature = "alloc-meter")]
    assert_eq!(
        meter::end(meter::begin()).live_end,
        baseline,
        "session owners must be fully released"
    );
    #[cfg(feature = "admission-profile")]
    println!("{{\"admission_profile\":{}}}", profile::json());
    let rows = samples
        .iter()
        .map(|s| {
            format!(
                "{{\"setup\":{},\"execution_observation\":{},\"engine_drop\":{},\"advances\":{}}}",
                s.setup.json(),
                s.execution_observation.json(),
                s.engine_drop.json(),
                s.advances
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "{{\"metered\":{},\"validated\":true,\"preparation\":{},\"queries\":[{}],\"preparation_drop\":{},\"consumer_drop\":{}}}",
        cfg!(feature = "alloc-meter"),
        preparation.json(),
        rows,
        preparation_drop.json(),
        consumer_drop.json()
    );
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    #[cfg(feature = "alloc-meter")]
    if args.get(1).is_some_and(|s| s == "meter-check") {
        meter::self_check().unwrap();
        println!("meter verified");
        return;
    }
    if args.get(1).is_some_and(|s| s == "clock-floor") {
        let mut samples = Vec::with_capacity(1000);
        for _ in 0..1000 {
            samples.push(phase(|| black_box(())).1.ns);
        }
        samples.sort();
        println!(
            "{{\"min_ns\":{},\"median_ns\":{},\"max_ns\":{}}}",
            samples[0], samples[500], samples[999]
        );
        return;
    }
    assert!(
        [6, 7].contains(&args.len()),
        "mode depth shared outcome query-count [complete-sessions]"
    );
    let depth = args[2].parse().unwrap();
    let shared = args[3].parse().unwrap();
    let outcome = &args[4];
    assert!(["success", "fail", "clash", "cancel"].contains(&outcome.as_str()));
    let count = args[5].parse().unwrap();
    assert!(count > 0);
    let sessions = args.get(6).map_or(1, |n| n.parse::<usize>().unwrap());
    assert!(sessions > 0);
    for _ in 0..sessions {
        match args[1].as_str() {
            "full" => run::<Relational<0>>(depth, shared, outcome, count),
            "selective" => run::<Relational<1>>(depth, shared, outcome, count),
            "batch8" => run::<Relational<2>>(depth, shared, outcome, count),
            "batch256" => run::<Relational<3>>(depth, shared, outcome, count),
            "compiled" => run::<Compiled>(depth, shared, outcome, count),
            _ => panic!("unknown schedule"),
        }
    }
}
