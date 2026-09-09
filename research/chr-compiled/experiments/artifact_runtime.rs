//! Shared runtime for independently compiled source artifacts. This boundary
//! probe is not a comparative timing registration.
#[allow(dead_code)]
#[path = "access_source.rs"]
mod access_source;
#[allow(dead_code)]
#[path = "subscription_join.rs"]
mod join;
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
        "subscription" => Ok(source::source_rules(false)),
        _ => Err("unknown artifact source family".into()),
    }
}
fn query(family: &str, size: usize, round: usize) -> Query {
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
fn run(mut code: Option<Compiled>) -> Result<(), String> {
    if crate::COLLECT_METRICS || crate::COLLECT_KERNEL_METRICS || chr_observe::COLLECT_METRICS {
        return Err("artifact probe requires counter-free ordinary allocation".into());
    }
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 4 {
        return Err("usage: MODE FAMILY SIZE QUERIES".into());
    }
    let mode = args[0].as_str();
    let family = args[1].as_str();
    let size: usize = args[2].parse().map_err(|_| "invalid size")?;
    let queries: usize = args[3].parse().map_err(|_| "invalid query count")?;
    if size > 4096 || !(1..=1024).contains(&queries) {
        return Err("artifact probe bounds exceeded".into());
    }
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
    if mode.starts_with("retained-") && family != "subscription" {
        return Err("retained mode requires subscription source".into());
    }
    if mode == "native-generic-repair" {
        code.as_mut().unwrap().updates = None;
    }
    let oracle_source = rules(family)?;
    let start = Instant::now();
    let source = rules(family)?;
    let source_ns = start.elapsed().as_nanos();
    let start = Instant::now();
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
    let prepare_ns = start.elapsed().as_nanos();
    let mut total = source_ns + prepare_ns;
    for round in 0..queries {
        // Inputs and independent complete answers are constructed outside runtime phases.
        let input = query(family, size + round % 2, round);
        let expected = oracle::run(&oracle_source, &input, 2_000_000);
        let start = Instant::now();
        let mut engine = prepared.start(input)?;
        let setup_ns = start.elapsed().as_nanos();
        let start = Instant::now();
        let status = engine.advance(2_000_000);
        let execute_ns = start.elapsed().as_nanos();
        if !status {
            return Err("unfinished artifact query at service bound".into());
        }
        let start = Instant::now();
        let answer = engine.observe()?;
        let observation_ns = start.elapsed().as_nanos();
        let start = Instant::now();
        drop(engine);
        let engine_drop_ns = start.elapsed().as_nanos();
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
        let start = Instant::now();
        drop(answer);
        let answer_drop_ns = start.elapsed().as_nanos();
        let query_ns = setup_ns + execute_ns + observation_ns + engine_drop_ns + answer_drop_ns;
        total += query_ns;
        println!(
            "{{\"query\":{round},\"size\":{},\"setup_ns\":{setup_ns},\"execute_ns\":{execute_ns},\"observation_ns\":{observation_ns},\"engine_drop_ns\":{engine_drop_ns},\"answer_drop_ns\":{answer_drop_ns},\"query_ns\":{query_ns},\"validated\":true}}",
            size + round % 2
        );
    }
    let start = Instant::now();
    drop(prepared);
    let prepared_drop_ns = start.elapsed().as_nanos();
    total += prepared_drop_ns;
    println!(
        "{{\"mode\":{mode:?},\"family\":{family:?},\"queries\":{queries},\"source_ns\":{source_ns},\"prepare_ns\":{prepare_ns},\"prepared_drop_ns\":{prepared_drop_ns},\"source_disposal\":\"included in preparation\",\"lifecycle_ns\":{total},\"counters\":false,\"allocator\":\"ordinary\"}}"
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
