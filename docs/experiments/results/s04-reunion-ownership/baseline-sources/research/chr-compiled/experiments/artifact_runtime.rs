//! Shared runtime for independently compiled source artifacts. This boundary
//! probe is not a comparative timing registration.
#[allow(dead_code)]
#[path = "access_source.rs"]
mod access_source;
#[allow(dead_code)]
#[path = "subscription_join.rs"]
mod join;
#[path = "subscription_low_yield.rs"]
mod low_yield;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[allow(dead_code)]
#[path = "subscription_runtime.rs"]
mod retained;
#[allow(dead_code)]
#[path = "subscription_source.rs"]
mod source;
// The retained source runtime refers to its common source module by this name.
// Keep access-source fixtures separate from that source contract.
use crate::{Access, Compiled, Policy, PreparedRuleset, fixtures};
use chr_syntax::{Query, Rule, atom, c, t, v};
use std::time::Instant;
fn dispatch_width(family: &str) -> Option<usize> {
    family
        .strip_prefix("dispatch")?
        .parse::<usize>()
        .ok()
        .filter(|n| (1..=256).contains(n))
}
pub fn rules(family: &str) -> Result<Vec<Rule>, String> {
    if let Some(width) = dispatch_width(family) {
        return Ok((0..width)
            .map(|i| {
                Rule::simplify(
                    &format!("rule{i}"),
                    [c(&format!("p{i}"), [v(0)]), c("q", [v(0)])],
                    c("out", [atom(&format!("r{i}")), v(0)]).into(),
                )
            })
            .collect());
    }
    match family {
        "chain" => Ok(fixtures::programs()[1].clone()),
        "payload" => Ok(access_source::payload_rules()),
        "subscription" | "low-stable" | "low-reopen" | "low-churn" => {
            Ok(source::source_rules(false))
        }
        _ => Err("unknown artifact source family".into()),
    }
}
fn query(family: &str, size: usize, round: usize) -> Query {
    if low_yield::FAMILIES.contains(&family) {
        return low_yield::query(family, size, 64, round);
    }
    if let Some(width) = dispatch_width(family) {
        return Query {
            constraints: vec![
                c(&format!("p{}", size % width), [atom("k")]),
                c("q", [atom("k")]),
            ],
            outputs: vec![],
        };
    }
    match family {
        "chain" => fixtures::flat_chain_case(size, false).query,
        "payload" => access_source::payload_query(size, round % 2 == 1),
        "subscription" => {
            let mut rows = Vec::new();
            for i in 0..size {
                let middle = atom(&format!("middle{i}"));
                let end = atom(&format!("end{i}"));
                rows.extend([
                    c("left", [atom("k"), t("f", [middle.clone()])]),
                    c("middle", [middle, t("g", [end.clone()])]),
                    c("right", [end, t("h", [atom("yes")])]),
                ]);
            }
            source::query(
                rows,
                vec![
                    t("open", [atom("k"), atom("yes"), atom("d")]),
                    t("ask", [atom("d"), atom("one")]),
                    t("ask", [atom("d"), atom("two")]),
                    t("close", [atom("d")]),
                ],
                round % 2 == 1,
            )
        }
        _ => unreachable!(),
    }
}
pub fn entry(code: Option<Compiled>) {
    if let Err(error) = run(code) {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
#[cfg(feature = "alloc-meter")]
use crate::experiment::meter;
struct Phase {
    clock: Instant,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Start,
}
impl Phase {
    fn begin() -> Self {
        #[cfg(feature = "alloc-meter")]
        let memory = meter::begin();
        Self {
            clock: Instant::now(),
            #[cfg(feature = "alloc-meter")]
            memory,
        }
    }
    fn finish(self) -> Measurement {
        let ns = self.clock.elapsed().as_nanos();
        Measurement {
            ns,
            #[cfg(feature = "alloc-meter")]
            memory: meter::end(self.memory),
        }
    }
}
struct Measurement {
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
#[cfg(feature = "alloc-meter")]
fn live() -> usize {
    let start = meter::begin();
    meter::end(start).live_start
}
#[cfg(feature = "alloc-meter")]
fn memory_json(phases: &[(&str, &Measurement)]) -> String {
    phases
        .iter()
        .map(|(name, phase)| format!("{name:?}:{}", phase.memory.json()))
        .collect::<Vec<_>>()
        .join(",")
}
fn run(mut code: Option<Compiled>) -> Result<(), String> {
    if crate::COLLECT_METRICS || crate::COLLECT_KERNEL_METRICS || chr_observe::COLLECT_METRICS {
        return Err("artifact probe requires disabled engine/kernel/observer counters".into());
    }
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if !(4..=5).contains(&args.len()) {
        return Err("usage: MODE FAMILY SIZE QUERIES [CANCEL_STEPS]".into());
    }
    let mode = args[0].as_str();
    let family = args[1].as_str();
    let size: usize = args[2].parse().map_err(|_| "invalid size")?;
    let queries: usize = args[3].parse().map_err(|_| "invalid query count")?;
    if size > 4096 || !(1..=16_384).contains(&queries) {
        return Err("artifact probe bounds exceeded".into());
    }
    let cancel_steps = args
        .get(4)
        .map(|s| {
            s.parse::<usize>()
                .map_err(|_| "invalid cancellation budget")
        })
        .transpose()?;
    if cancel_steps.is_some_and(|steps| steps > 2_000_000) {
        return Err("cancellation budget exceeds service bound".into());
    }
    if low_yield::FAMILIES.contains(&family) && ![4, 8, 16].contains(&size) {
        return Err("low-yield size must be 4, 8 or 16".into());
    }
    let mut cancelled_queries = 0;
    let native = mode.starts_with("native");
    if native != code.is_some() {
        return Err("artifact and execution mode disagree".into());
    }
    if ![
        "generic",
        "planned",
        "specialized",
        "native",
        "native-generic-repair",
        "native-planned",
        "native-specialized",
        "retained-indexed",
        "retained-eager",
        "retained-subscribed",
    ]
    .contains(&mode)
    {
        return Err("unknown mode".into());
    }
    if mode.starts_with("retained-")
        && family != "subscription"
        && !low_yield::FAMILIES.contains(&family)
    {
        return Err("retained mode requires subscription source".into());
    }
    if mode == "native-generic-repair" {
        code.as_mut().unwrap().updates = None;
    }
    #[cfg(feature = "alloc-meter")]
    {
        meter::self_check()?;
        // Initialize process-owned output buffering before checking runtime ownership.
        print!("");
    }
    let oracle_source = rules(family)?;
    // Oracle answers are reusable only after exact source-query equality. Engine
    // answers remain independently checked on every completed query.
    let oracle_cases = if ["chain", "payload"].contains(&family) {
        Some(
            [query(family, size, 0), query(family, size + 1, 1)].map(|input| {
                let answer = oracle::run(&oracle_source, &input, 2_000_000);
                (input, answer)
            }),
        )
    } else {
        None
    };
    #[cfg(feature = "alloc-meter")]
    let prepared_baseline = live();
    let start = Phase::begin();
    let source = rules(family)?;
    let source_measurement = start.finish();
    let source_ns = source_measurement.ns;
    let start = Phase::begin();
    let prepare_source = source;
    let prepared = if mode.starts_with("retained-") {
        let plan = retained::Prepared::new(&prepare_source)?;
        drop(prepare_source);
        Prepared::Retained(
            plan,
            match mode {
                "retained-indexed" => join::Mode::Indexed,
                "retained-eager" => join::Mode::Eager,
                "retained-subscribed" => join::Mode::Subscribed,
                _ => unreachable!(),
            },
        )
    } else {
        let mut plan = if ["planned", "specialized", "native-planned"].contains(&mode) {
            PreparedRuleset::new_with_update_plan(prepare_source, code)?
        } else {
            PreparedRuleset::new(prepare_source, code)?
        };
        if ["specialized", "native-specialized"].contains(&mode) {
            plan = plan.specialize_inferred();
        }
        Prepared::Compiled(plan)
    };
    let prepare_measurement = start.finish();
    let prepare_ns = prepare_measurement.ns;
    let mut total = source_ns + prepare_ns;
    for round in 0..queries {
        // Inputs and independent complete answers are constructed outside runtime phases.
        #[cfg(feature = "alloc-meter")]
        let query_baseline = live();
        let current_size = if low_yield::FAMILIES.contains(&family) {
            size
        } else {
            size + round % 2
        };
        let input = query(family, current_size, round);
        let cancel_this = cancel_steps.is_some() && round % 2 == 0;
        let expected_owned = if !cancel_this && oracle_cases.is_none() {
            oracle::run(&oracle_source, &input, 2_000_000)
        } else {
            Vec::new()
        };
        let expected = if cancel_this {
            &[][..]
        } else if let Some(cases) = &oracle_cases {
            &cases
                .iter()
                .find(|(known, _)| known == &input)
                .ok_or("query does not match reusable independent oracle input")?
                .1[..]
        } else {
            &expected_owned[..]
        };
        let start = Phase::begin();
        let mut engine = prepared.start(input)?;
        let setup_measurement = start.finish();
        let setup_ns = setup_measurement.ns;
        let start = Phase::begin();
        let status = engine.advance(if cancel_this {
            cancel_steps.unwrap()
        } else {
            2_000_000
        });
        let execute_measurement = start.finish();
        let execute_ns = execute_measurement.ns;
        if cancel_this {
            let start = Phase::begin();
            drop(engine);
            let engine_drop_measurement = start.finish();
            let engine_drop_ns = engine_drop_measurement.ns;
            drop(expected_owned);
            #[cfg(feature = "alloc-meter")]
            if live() != query_baseline {
                return Err(format!(
                    "cancelled query ownership mismatch: baseline {query_baseline}, end {}",
                    live()
                ));
            }
            let query_ns = setup_ns + execute_ns + engine_drop_ns;
            total += query_ns;
            cancelled_queries += 1;
            #[cfg(feature = "alloc-meter")]
            let memory = format!(
                ",\"memory\":{{{}}},\"query_restored\":true",
                memory_json(&[
                    ("setup", &setup_measurement),
                    ("execute", &execute_measurement),
                    ("engine_drop", &engine_drop_measurement)
                ])
            );
            #[cfg(not(feature = "alloc-meter"))]
            let memory = "";
            println!(
                "{{\"query\":{round},\"size\":{},\"cancelled\":true,\"exhausted_before_cancel\":{status},\"setup_ns\":{setup_ns},\"execute_ns\":{execute_ns},\"engine_drop_ns\":{engine_drop_ns},\"query_ns\":{query_ns}{memory}}}",
                current_size
            );
            continue;
        }
        if !status {
            return Err("unfinished artifact query at service bound".into());
        }
        let start = Phase::begin();
        let answer = engine.observe()?;
        let observation_measurement = start.finish();
        let observation_ns = observation_measurement.ns;
        let start = Phase::begin();
        drop(engine);
        let engine_drop_measurement = start.finish();
        let engine_drop_ns = engine_drop_measurement.ns;
        // Exact observations, including aliases and residual multiplicity, are checked
        // before timing answer disposal. Expected answers are never passed to the engine.
        if expected.len() != usize::from(answer.is_some()) {
            return Err("independent outcome mismatch".into());
        }
        if let Some(answer) = &answer
            && !chr_observe::equivalent(answer, &expected[0], &mut Default::default())
        {
            return Err("independent full-answer mismatch".into());
        }
        let start = Phase::begin();
        drop(answer);
        let answer_drop_measurement = start.finish();
        let answer_drop_ns = answer_drop_measurement.ns;
        let query_ns = setup_ns + execute_ns + observation_ns + engine_drop_ns + answer_drop_ns;
        total += query_ns;
        drop(expected_owned);
        #[cfg(feature = "alloc-meter")]
        if live() != query_baseline {
            return Err(format!(
                "query ownership mismatch: baseline {query_baseline}, end {}",
                live()
            ));
        }
        #[cfg(feature = "alloc-meter")]
        let memory = format!(
            ",\"memory\":{{{}}},\"query_restored\":true",
            memory_json(&[
                ("setup", &setup_measurement),
                ("execute", &execute_measurement),
                ("observation", &observation_measurement),
                ("engine_drop", &engine_drop_measurement),
                ("answer_drop", &answer_drop_measurement)
            ])
        );
        #[cfg(not(feature = "alloc-meter"))]
        let memory = "";
        println!(
            "{{\"query\":{round},\"size\":{},\"setup_ns\":{setup_ns},\"execute_ns\":{execute_ns},\"observation_ns\":{observation_ns},\"engine_drop_ns\":{engine_drop_ns},\"answer_drop_ns\":{answer_drop_ns},\"query_ns\":{query_ns},\"validated\":true{memory}}}",
            current_size
        );
    }
    let start = Phase::begin();
    drop(prepared);
    let prepared_drop_measurement = start.finish();
    let prepared_drop_ns = prepared_drop_measurement.ns;
    total += prepared_drop_ns;
    #[cfg(feature = "alloc-meter")]
    if live() != prepared_baseline {
        return Err(format!(
            "prepared ownership mismatch: baseline {prepared_baseline}, end {}",
            live()
        ));
    }
    #[cfg(feature = "alloc-meter")]
    let memory = format!(
        ",\"memory\":{{{}}},\"prepared_restored\":true",
        memory_json(&[
            ("source", &source_measurement),
            ("prepare", &prepare_measurement),
            ("prepared_drop", &prepared_drop_measurement)
        ])
    );
    #[cfg(not(feature = "alloc-meter"))]
    let memory = "";
    let allocator = if cfg!(feature = "alloc-meter") {
        "requested-meter"
    } else {
        "ordinary"
    };
    println!(
        "{{\"mode\":{mode:?},\"family\":{family:?},\"queries\":{queries},\"cancelled_queries\":{cancelled_queries},\"source_ns\":{source_ns},\"prepare_ns\":{prepare_ns},\"prepared_drop_ns\":{prepared_drop_ns},\"source_disposal\":\"included in preparation\",\"lifecycle_ns\":{total},\"counters\":false,\"allocator\":{allocator:?}{memory}}}"
    );
    Ok(())
}

enum Prepared {
    Compiled(PreparedRuleset),
    Retained(retained::Prepared, join::Mode),
}
impl Prepared {
    fn start(&self, input: Query) -> Result<Engine, String> {
        match self {
            Self::Compiled(plan) => plan
                .start(input, Policy::Global, Access::Indexed)
                .map(Engine::Compiled),
            Self::Retained(plan, mode) => plan.start(input, *mode).map(Engine::Retained),
        }
    }
}
#[allow(clippy::large_enum_variant)]
enum Engine {
    Compiled(crate::Engine),
    Retained(retained::Engine),
}
impl Engine {
    fn advance(&mut self, budget: usize) -> bool {
        match self {
            Self::Compiled(engine) => engine.advance(budget).exhausted,
            Self::Retained(engine) => engine.advance(budget),
        }
    }
    fn observe(&mut self) -> Result<Option<chr_syntax::Answer>, String> {
        match self {
            Self::Compiled(engine) => Ok(engine.observe()),
            Self::Retained(engine) => {
                let mut answers = engine.observe()?;
                if answers.len() > 1 {
                    return Err("deterministic artifact source returned multiple answers".into());
                }
                Ok(answers.pop())
            }
        }
    }
}
