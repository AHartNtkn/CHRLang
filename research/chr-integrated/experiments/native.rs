//! Native phase measurement; comparative runs require a frozen registration.
use chr_compiled::{Access, Policy};
use chr_syntax::{Query, Rule};
mod workloads;
use workloads::Family;
#[allow(unused_imports, unused_variables)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
    pub fn selected(id: usize) -> chr_compiled::Compiled {
        code(id)
    }
}
#[derive(Clone, Copy, Debug)]
enum Variant {
    Integrated,
    GenericIndexed,
    GeneratedIndexed,
    GeneratedScan,
    GeneratedGlobalScan,
}
impl Variant {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "integrated" => Self::Integrated,
            "generic-indexed" => Self::GenericIndexed,
            "generated-indexed" => Self::GeneratedIndexed,
            "generated-scan" => Self::GeneratedScan,
            "generated-global-scan" => Self::GeneratedGlobalScan,
            _ => return None,
        })
    }
    fn name(self) -> &'static str {
        match self {
            Self::Integrated => "integrated",
            Self::GenericIndexed => "generic-indexed",
            Self::GeneratedIndexed => "generated-indexed",
            Self::GeneratedScan => "generated-scan",
            Self::GeneratedGlobalScan => "generated-global-scan",
        }
    }
}
enum Prepared {
    Integrated(chr_integrated::PreparedRuleset),
    Dedicated(chr_compiled::PreparedRuleset),
}
impl Prepared {
    fn new(
        rules: Vec<Rule>,
        code: Option<chr_compiled::Compiled>,
        variant: Variant,
    ) -> Result<Self, String> {
        if matches!(variant, Variant::Integrated) {
            let prepared = chr_integrated::PreparedRuleset::new(&rules)?;
            drop(rules);
            Ok(Self::Integrated(prepared))
        } else {
            chr_compiled::PreparedRuleset::new(rules, code).map(Self::Dedicated)
        }
    }
    fn start(&self, query: Query, variant: Variant) -> Result<Engine, String> {
        match self {
            Self::Integrated(p) => {
                let engine = p.start(&query);
                drop(query);
                Ok(Engine::Integrated(engine))
            }
            Self::Dedicated(p) => {
                let policy = if matches!(variant, Variant::GeneratedGlobalScan) {
                    Policy::Global
                } else {
                    Policy::Active
                };
                let access = if matches!(
                    variant,
                    Variant::GeneratedScan | Variant::GeneratedGlobalScan
                ) {
                    Access::Scan
                } else {
                    Access::Indexed
                };
                p.start(query, policy, access).map(Engine::Dedicated)
            }
        }
    }
}
#[allow(clippy::large_enum_variant)]
enum Engine {
    Integrated(chr_integrated::Engine),
    Dedicated(chr_compiled::Engine),
}
struct Status {
    exhausted: bool,
    failed: bool,
}
impl Engine {
    fn advance(&mut self, budget: usize) -> Status {
        match self {
            Self::Integrated(e) => {
                let status = e.run(budget);
                Status {
                    exhausted: status == chr_integrated::Step::Complete,
                    failed: status == chr_integrated::Step::Failed,
                }
            }
            Self::Dedicated(e) => {
                let status = e.advance(budget);
                Status {
                    exhausted: status.exhausted,
                    failed: status.failed,
                }
            }
        }
    }
    fn observe(&mut self) -> Option<Answer> {
        match self {
            Self::Integrated(e) => e.answer(),
            Self::Dedicated(e) => e.observe(),
        }
    }
    fn work(&self) -> Vec<(&'static str, u64)> {
        match self {
            Self::Dedicated(e) => work(e.stats()),
            Self::Integrated(e) => {
                let s = e.stats();
                vec![
                    ("steps", s.steps as u64),
                    ("equality_steps", s.equality_steps as u64),
                    ("descriptor_repairs", s.descriptor_repairs as u64),
                    ("index_migrations", s.index_migrations as u64),
                    ("activations", s.activations as u64),
                    ("candidate_visits", s.candidate_visits as u64),
                    ("indexed_lookups", s.indexed_lookups as u64),
                    ("applications", s.applications as u64),
                    (
                        "speculative_applications",
                        s.speculative_applications as u64,
                    ),
                ]
            }
        }
    }
}
use chr_syntax::Answer;
use std::time::Instant;
#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[cfg(feature = "alloc-meter")]
use meter::{begin as memory_begin, end as memory_end};
#[cfg(not(feature = "alloc-meter"))]
fn memory_begin() -> MemoryStart {
    MemoryStart
}
#[cfg(not(feature = "alloc-meter"))]
fn memory_end(_: MemoryStart) -> Memory {
    Memory
}
#[cfg(feature = "alloc-meter")]
type Memory = meter::Reading;
#[cfg(not(feature = "alloc-meter"))]
#[derive(Clone, Copy)]
struct Memory;
#[cfg(not(feature = "alloc-meter"))]
struct MemoryStart;
#[cfg(feature = "alloc-meter")]
fn memory_json(reading: Memory) -> String {
    reading.json()
}
#[cfg(not(feature = "alloc-meter"))]
fn memory_json(_: Memory) -> String {
    "null".into()
}

struct Config {
    mode: String,
    family: Family,
    family_name: String,
    size: usize,
    depth: usize,
    queries: usize,
    variant: Variant,
}
struct Sample {
    size: usize,
    outputs: usize,
    residual: usize,
    exhausted: bool,
    failed: bool,
    setup_ns: u128,
    execution_ns: u128,
    observe_ns: u128,
    engine_drop_ns: u128,
    request_ns: u128,
    answer_drop_ns: u128,
    memory: [Memory; 5],
    work: Option<Vec<(&'static str, u64)>>,
}
struct Report {
    completed: usize,
    prepare_ns: u128,
    harness_ns: u128,
    prepared_drop_ns: u128,
    samples: Vec<Sample>,
    preparation_memory: Memory,
    prepared_drop_memory: Memory,
}
fn validate_answer(actual: &Answer, expected: &Answer) -> Result<(), String> {
    if chr_observe::equivalent(actual, expected, &mut Default::default()) {
        Ok(())
    } else {
        Err(format!(
            "full answer mismatch: {actual:?}, expected {expected:?}"
        ))
    }
}
fn work(stats: &chr_compiled::Stats) -> Vec<(&'static str, u64)> {
    vec![
        ("applications", stats.applications),
        ("candidate_visits", stats.candidate_visits),
        ("generic_template_visits", stats.generic_ast_visits),
        ("structural_tests", stats.structural_tests),
        ("binding_slot_copies", stats.binding_slot_copies),
        ("pool_entries", stats.pool_visits),
        ("index_bucket_entries", stats.index_bucket_entries),
        ("index_lookups", stats.index_lookups),
        ("index_repairs", stats.index_repairs),
        ("index_inserts", stats.index_inserts),
        ("index_removes", stats.index_removes),
        ("key_visits", stats.key_visits),
        ("key_template_visits", stats.key_template_visits),
        (
            "key_normalization_requests",
            stats.key_normalization_requests,
        ),
        (
            "key_normalization_allocations",
            stats.key_normalization_allocations,
        ),
        ("dependency_visits", stats.dependency_visits),
        ("activation_pushes", stats.activation_pushes),
        ("history_checks", stats.history_checks),
        ("pairs", stats.kernel.pairs),
        ("dereferences", stats.kernel.dereferences),
        ("map_visits", stats.kernel.storage.visits),
        ("map_allocations", stats.kernel.storage.allocations),
        ("term_requests", stats.kernel.term_requests),
    ]
}
fn run_config(config: &Config) -> Result<Report, String> {
    if config.queries == 0 {
        return Err("query count must be positive".into());
    }
    let harness_start = Instant::now();
    // Source AST construction is harness work. Only the chosen ruleset enters preparation.
    let mut programs = workloads::programs();
    let rules = programs.swap_remove(config.family.program());
    drop(programs);
    let code = match config.variant {
        Variant::Integrated | Variant::GenericIndexed => None,
        _ => Some(generated::selected(config.family.program())),
    };
    let mut harness_ns = harness_start.elapsed().as_nanos();
    let preparation_start = memory_begin();
    let before = Instant::now();
    let prepared = Prepared::new(rules, code, config.variant)?;
    let prepare_ns = before.elapsed().as_nanos();
    let preparation_memory = memory_end(preparation_start);
    let mut samples = Vec::with_capacity(config.queries);
    let mut completed = 0;
    for i in 0..config.queries {
        let harness_start = Instant::now();
        let size = config.size + i % 2;
        let (query, expected) = workloads::case(config.family, size, config.depth);
        // Expected observations must not remain in the engine's allocation baseline.
        drop(expected);
        harness_ns += harness_start.elapsed().as_nanos();
        let m0 = memory_begin();
        let t0 = Instant::now();
        let mut engine = prepared.start(query, config.variant)?;
        let t1 = Instant::now();
        let memory_setup = memory_end(m0);
        let m1 = memory_begin();
        let execution_start = Instant::now();
        let status = engine.advance(1_000_000);
        let t2 = Instant::now();
        let memory_execution = memory_end(m1);
        let m2 = memory_begin();
        let observation_start = Instant::now();
        let answer = engine.observe();
        let t3 = Instant::now();
        let memory_observe = memory_end(m2);
        // Collected builds explain work; their timing is not the primary timing result.
        let counters = if cfg!(feature = "metrics") {
            Some(engine.work())
        } else {
            None
        };
        let m3 = memory_begin();
        let before_drop = Instant::now();
        drop(engine);
        let t4 = Instant::now();
        let memory_engine_drop = memory_end(m3);
        if status.failed {
            return Err("registered successful fixture failed".into());
        }
        let harness_start = Instant::now();
        let (outputs, residual) = if status.exhausted {
            let (fixture, expected) = workloads::case(config.family, size, config.depth);
            drop(fixture);
            let actual = answer
                .as_ref()
                .ok_or("completed fixture has no observation")?;
            validate_answer(actual, &expected)?;
            completed += 1;
            (actual.outputs.len(), actual.residual.len())
        } else {
            if answer.is_some() {
                return Err("cutoff published an answer".into());
            }
            (0, 0)
        };
        harness_ns += harness_start.elapsed().as_nanos();
        let m4 = memory_begin();
        let before_answer_drop = Instant::now();
        drop(answer);
        let answer_drop_ns = before_answer_drop.elapsed().as_nanos();
        let memory_answer_drop = memory_end(m4);
        samples.push(Sample {
            size,
            outputs,
            residual,
            exhausted: status.exhausted,
            failed: status.failed,
            setup_ns: t1.duration_since(t0).as_nanos(),
            execution_ns: t2.duration_since(execution_start).as_nanos(),
            observe_ns: t3.duration_since(observation_start).as_nanos(),
            engine_drop_ns: t4.duration_since(before_drop).as_nanos(),
            request_ns: t4.duration_since(t0).as_nanos(),
            answer_drop_ns,
            work: counters,
            memory: [
                memory_setup,
                memory_execution,
                memory_observe,
                memory_engine_drop,
                memory_answer_drop,
            ],
        });
        if !status.exhausted {
            break;
        }
    }
    let md = memory_begin();
    let before_drop = Instant::now();
    drop(prepared);
    let prepared_drop_ns = before_drop.elapsed().as_nanos();
    let prepared_drop_memory = memory_end(md);
    Ok(Report {
        completed,
        prepare_ns,
        harness_ns,
        prepared_drop_ns,
        samples,
        preparation_memory,
        prepared_drop_memory,
    })
}
impl Report {
    fn json(&self, config: &Config) -> String {
        let samples: Vec<_> = self.samples.iter().map(|s| {
            let counters = s.work.as_ref().map_or("null".into(), |fields| {
                format!("{{{}}}", fields.iter().map(|(key, value)| format!("{key:?}:{value}")).collect::<Vec<_>>().join(","))
            });
            format!(r#"{{"size":{},"outputs":{},"residual":{},"exhausted":{},"failed":{},"setup_ns":{},"execution_ns":{},"observe_ns":{},"engine_drop_ns":{},"request_ns":{},"answer_drop_ns":{},"work":{},"memory":[{}]}}"#,
                s.size,s.outputs,s.residual,s.exhausted,s.failed,s.setup_ns,s.execution_ns,s.observe_ns,s.engine_drop_ns,s.request_ns,s.answer_drop_ns,counters,s.memory.iter().map(|m|memory_json(*m)).collect::<Vec<_>>().join(","))
        }).collect();
        format!(
            r#"{{"schema":2,"family":{:?},"size":{},"depth":{},"queries":{},"variant":{:?},"mode":{:?},"metrics":{},"compiled_metrics":{},"kernel_metrics":{},"completed":{},"prepare_ns":{},"harness_ns":{},"prepared_drop_ns":{},"allocator_meter":{},"preparation_memory":{},"prepared_drop_memory":{},"samples":[{}]}}"#,
            config.family_name,
            config.size,
            config.depth,
            config.queries,
            config.variant.name(),
            config.mode,
            cfg!(feature = "metrics"),
            chr_compiled::COLLECT_METRICS,
            chr_persistent::COLLECT_KERNEL_METRICS,
            self.completed,
            self.prepare_ns,
            self.harness_ns,
            self.prepared_drop_ns,
            cfg!(feature = "alloc-meter"),
            memory_json(self.preparation_memory),
            memory_json(self.prepared_drop_memory),
            samples.join(",")
        )
    }
}
pub fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["clock-check"] {
        let mut readings = Vec::with_capacity(10000);
        for _ in 0..10000 {
            let start = Instant::now();
            std::hint::black_box(());
            readings.push(start.elapsed().as_nanos());
        }
        readings.sort_unstable();
        println!(
            "{{\"samples\":10000,\"min_ns\":{},\"median_ns\":{},\"p99_ns\":{}}}",
            readings[0], readings[5000], readings[9900]
        );
        return Ok(());
    }
    #[cfg(feature = "alloc-meter")]
    if args == ["meter-check"] {
        meter::self_check()?;
        println!("allocator self-check passed");
        return Ok(());
    }
    if args.len() != 6 {
        return Err("usage: chr-integrated-cost time|work|memory independent|fanout|repair|build SIZE DEPTH QUERIES integrated|generic-indexed|generated-indexed|generated-scan|generated-global-scan".into());
    }
    let counted = match args[0].as_str() {
        "time" | "memory" => false,
        "work" => true,
        _ => return Err("unknown measurement mode".into()),
    };
    if cfg!(feature = "metrics") != counted
        || chr_compiled::COLLECT_METRICS != counted
        || chr_persistent::COLLECT_KERNEL_METRICS != counted
    {
        return Err(
            "measurement mode disagrees with resolved engine/kernel counter features".into(),
        );
    }
    if cfg!(feature = "alloc-meter") != (args[0] == "memory") {
        return Err("measurement mode disagrees with allocator meter configuration".into());
    }
    let config = Config {
        mode: args[0].clone(),
        family: Family::parse(&args[1]).ok_or("unknown family")?,
        family_name: args[1].clone(),
        size: args[2].parse().map_err(|_| "invalid size")?,
        depth: args[3].parse().map_err(|_| "invalid depth")?,
        queries: args[4].parse().map_err(|_| "invalid query count")?,
        variant: Variant::parse(&args[5]).ok_or("unknown variant")?,
    };
    if config.depth == 0 || config.size.checked_add(1).is_none() {
        return Err("depth must be positive and size must permit n+1".into());
    }
    println!("{}", run_config(&config)?.json(&config));
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use chr_syntax::{atom, c};

    #[test]
    fn validation_checks_values_and_residual_multiplicity() {
        let expected = Answer {
            outputs: vec![("out".into(), atom("a"))],
            residual: vec![c("p", [atom("a")]), c("p", [atom("a")])],
        };
        let wrong_value = Answer {
            outputs: vec![("out".into(), atom("b"))],
            residual: expected.residual.clone(),
        };
        let wrong_multiset = Answer {
            outputs: expected.outputs.clone(),
            residual: vec![c("p", [atom("a")])],
        };
        assert!(validate_answer(&wrong_value, &expected).is_err());
        assert!(validate_answer(&wrong_multiset, &expected).is_err());
    }

    #[test]
    fn variants_preserve_complete_answers_across_preparation_reuse() {
        for name in [
            "integrated",
            "generic-indexed",
            "generated-indexed",
            "generated-scan",
            "generated-global-scan",
        ] {
            let variant = Variant::parse(name).unwrap();
            let family = Family::Fanout;
            let rules = workloads::programs().swap_remove(family.program());
            let code = match variant {
                Variant::Integrated | Variant::GenericIndexed => None,
                _ => Some(generated::selected(family.program())),
            };
            let prepared = Prepared::new(rules, code, variant).unwrap();
            for size in [2, 3] {
                let (query, expected) = workloads::case(family, size, 2);
                let mut engine = prepared.start(query, variant).unwrap();
                assert!(engine.observe().is_none());
                let status = engine.advance(1_000_000);
                assert!(status.exhausted && !status.failed, "{name}");
                validate_answer(&engine.observe().unwrap(), &expected).unwrap();
            }
        }
    }
}
