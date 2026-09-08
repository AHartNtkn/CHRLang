//! Native phase measurement; comparative runs require a frozen registration.
use crate::{Access, Execution, Policy, PreparedRuleset, fixtures};
use chr_syntax::Answer;
use std::time::Instant;
#[cfg(feature = "alloc-meter")]
#[path = "meter.rs"]
pub mod meter;
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

#[derive(Clone, Copy, Debug)]
enum Family {
    Build,
    Chain,
    Delayed,
    Collision,
    Repair,
}
impl Family {
    fn program(self) -> usize {
        match self {
            Self::Build => 0,
            Self::Chain => 1,
            Self::Delayed => 2,
            Self::Collision => 13,
            Self::Repair => 11,
        }
    }
    fn case(self, size: usize) -> fixtures::Case {
        match self {
            Self::Build => fixtures::case(0, size),
            Self::Chain => fixtures::flat_chain_case(size, false),
            Self::Delayed => fixtures::flat_chain_case(size, true),
            Self::Collision => fixtures::collision_case(size),
            Self::Repair => fixtures::repair_case(size),
        }
    }
}
struct Config {
    family: Family,
    size: usize,
    queries: usize,
    execution: Execution,
    policy: Policy,
    access: Access,
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
fn work(stats: &crate::Stats) -> Vec<(&'static str, u64)> {
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
    let mut programs = fixtures::programs();
    let rules = programs.swap_remove(config.family.program());
    drop(programs);
    let code = match config.execution {
        Execution::Generic => None,
        Execution::Generated => Some(super::bundled(config.family.program())),
    };
    let mut harness_ns = harness_start.elapsed().as_nanos();
    let preparation_start = memory_begin();
    let before = Instant::now();
    let prepared = PreparedRuleset::new(rules, code)?;
    let prepare_ns = before.elapsed().as_nanos();
    let preparation_memory = memory_end(preparation_start);
    let mut samples = Vec::with_capacity(config.queries);
    let mut completed = 0;
    for i in 0..config.queries {
        let harness_start = Instant::now();
        let size = config.size + i % 2;
        let case = config.family.case(size);
        let query = case.query;
        // Do not retain an oracle-sized expected answer in the engine's memory baseline.
        drop(case.expected);
        harness_ns += harness_start.elapsed().as_nanos();
        let m0 = memory_begin();
        let t0 = Instant::now();
        let mut engine = prepared.start(query, config.policy, config.access)?;
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
        let counters = if crate::COLLECT_METRICS {
            Some(work(engine.stats()))
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
            let expected = config.family.case(size).expected.unwrap();
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
            format!("{{\"size\":{},\"outputs\":{},\"residual\":{},\"exhausted\":{},\"failed\":{},\"setup_ns\":{},\"execution_ns\":{},\"observe_ns\":{},\"engine_drop_ns\":{},\"request_ns\":{},\"answer_drop_ns\":{},\"work\":{},\"memory\":[{}]}}",
                    s.size,s.outputs,s.residual,s.exhausted,s.failed,s.setup_ns,s.execution_ns,s.observe_ns,s.engine_drop_ns,s.request_ns,s.answer_drop_ns,counters,s.memory.iter().map(|m|memory_json(*m)).collect::<Vec<_>>().join(","))
        }).collect();
        format!(
            "{{\"schema\":2,\"family\":{:?},\"size\":{},\"queries\":{},\"execution\":{:?},\"policy\":{:?},\"access\":{:?},\"engine_metrics\":{},\"kernel_metrics\":{},\"completed\":{},\"prepare_ns\":{},\"harness_ns\":{},\"prepared_drop_ns\":{},\"allocator_meter\":{},\"preparation_memory\":{},\"prepared_drop_memory\":{},\"samples\":[{}]}}",
            format!("{:?}", config.family),
            config.size,
            config.queries,
            format!("{:?}", config.execution),
            format!("{:?}", config.policy),
            format!("{:?}", config.access),
            crate::COLLECT_METRICS,
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
    if args.len() != 7 {
        return Err("usage: chr-compiled-cost time|work|memory build|chain|delayed|collision|repair SIZE QUERIES generic|generated global|active scan|indexed".into());
    }
    let counted = match args[0].as_str() {
        "time" | "memory" => false,
        "work" => true,
        _ => return Err("unknown measurement mode".into()),
    };
    if crate::COLLECT_METRICS != counted || chr_persistent::COLLECT_KERNEL_METRICS != counted {
        return Err(
            "measurement mode disagrees with resolved engine/kernel counter features".into(),
        );
    }
    if cfg!(feature = "alloc-meter") != (args[0] == "memory") {
        return Err("measurement mode disagrees with allocator meter configuration".into());
    }
    let config = Config {
        family: match args[1].as_str() {
            "build" => Family::Build,
            "chain" => Family::Chain,
            "delayed" => Family::Delayed,
            "collision" => Family::Collision,
            "repair" => Family::Repair,
            _ => return Err("unknown family".into()),
        },
        size: args[2].parse().map_err(|_| "invalid size")?,
        queries: args[3].parse().map_err(|_| "invalid query count")?,
        execution: match args[4].as_str() {
            "generic" => Execution::Generic,
            "generated" => Execution::Generated,
            _ => return Err("unknown execution".into()),
        },
        policy: match args[5].as_str() {
            "global" => Policy::Global,
            "active" => Policy::Active,
            _ => return Err("unknown policy".into()),
        },
        access: match args[6].as_str() {
            "scan" => Access::Scan,
            "indexed" => Access::Indexed,
            _ => return Err("unknown access".into()),
        },
    };
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
    fn reused_preparation_runs_changed_queries_and_checks_full_answers() {
        let result = run_config(&Config {
            family: Family::Chain,
            size: 1,
            queries: 2,
            execution: Execution::Generated,
            policy: Policy::Active,
            access: Access::Indexed,
        })
        .unwrap();
        assert_eq!(result.completed, 2);
        assert_eq!(
            result
                .samples
                .iter()
                .map(|s| (s.size, s.outputs, s.residual))
                .collect::<Vec<_>>(),
            vec![(1, 2, 5), (2, 3, 8)]
        );
    }
}
