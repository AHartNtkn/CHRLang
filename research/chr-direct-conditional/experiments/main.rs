//! Isolated-process lifecycle runner. Comparative invocations require registration.
use chr_syntax::{Answer, Query, Rule, Var, and, atom, c, eq, or, t, v};
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
fn numeral(n: usize) -> chr_syntax::Term {
    (0..n).fold(atom("z"), |n, _| t("s", [n]))
}
fn rules(family: &str) -> Vec<Rule> {
    if family == "output" {
        return vec![
            Rule::simplify(
                "base",
                [c("build", [atom("z"), v(0)])],
                eq(v(0), atom("nil")),
            ),
            Rule::simplify(
                "step",
                [c("build", [t("s", [v(0)]), v(1)])],
                and(vec![
                    or(
                        eq(v(1), t("pair", [atom("a"), v(2)])),
                        eq(v(1), t("pair", [atom("b"), v(2)])),
                    ),
                    c("build", [v(0), v(2)]).into(),
                ]),
            ),
        ];
    }
    let mut rules = vec![Rule::simplify(
        "choose",
        [c("choose", [v(0)])],
        or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
    )];
    if family == "discriminate" {
        for bits in 0..16 {
            let fields = (0..4)
                .map(|i| atom(if bits & (1 << i) == 0 { "a" } else { "b" }))
                .collect::<Vec<_>>();
            let value = t("pack", fields);
            rules.push(Rule::simplify(
                &format!("gate{bits}"),
                [c("gate", [v(0), value.clone()])],
                c("work", [v(0), value]).into(),
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
    if family == "output" {
        return Query {
            constraints: vec![c("build", [numeral(n), v(0)])],
            outputs: vec![("out".into(), Var(0))],
        };
    }
    let mut constraints = vec![];
    if family != "plain" {
        constraints.extend((0..4).map(|i| c("choose", [v(i)])));
    }
    constraints.push(c(
        if family == "discriminate" {
            "gate"
        } else {
            "work"
        },
        [numeral(n), t("pack", (0..4).map(v).collect::<Vec<_>>())],
    ));
    Query {
        constraints,
        outputs: (0..4).map(|i| (format!("o{i}"), Var(i))).collect(),
    }
}
enum Prepared {
    Conditional(chr_direct_conditional::engine::PreparedRuleset),
    Explicit(chr_compiled::PreparedRuleset),
}
// Keep the measurement wrapper on the stack; no extra engine allocation.
#[allow(clippy::large_enum_variant)]
enum Running {
    Conditional(chr_direct_conditional::engine::Engine),
    Explicit(chr_compiled::SearchEngine),
}
impl Prepared {
    fn start(&self, q: Query) -> Running {
        match self {
            Self::Conditional(p) => Running::Conditional(p.start(q).unwrap()),
            Self::Explicit(p) => Running::Explicit(
                p.start(
                    q,
                    chr_compiled::Policy::Global,
                    chr_compiled::Access::Indexed,
                )
                .unwrap()
                .into_search(),
            ),
        }
    }
}
#[derive(Default)]
struct Work {
    applications: u64,
    choices: u64,
    candidates: u64,
    specialized_applications: u64,
    cursor_steps: u64,
    history_checks: u64,
}
impl Work {
    fn add(&mut self, stats: &chr_compiled::Stats) {
        if cfg!(feature = "metrics") {
            self.applications += stats.applications;
            self.candidates += stats.candidate_visits + stats.specialized_candidates;
            self.specialized_applications += stats.specialized_applications;
            self.cursor_steps += stats.cursor_steps;
            self.history_checks += stats.history_checks;
        }
    }
    fn json(&self) -> String {
        if cfg!(feature = "metrics") {
            format!(
                "{{\"applications\":{},\"choices\":{},\"candidates\":{},\"specialized_applications\":{},\"cursor_steps\":{},\"history_checks\":{}}}",
                self.applications,
                self.choices,
                self.candidates,
                self.specialized_applications,
                self.cursor_steps,
                self.history_checks
            )
        } else {
            "null".into()
        }
    }
}
impl Running {
    fn collect(&mut self) -> (Vec<Answer>, u128, Work) {
        let start = Instant::now();
        let mut work = Work::default();
        let mut first = None;
        let mut answers = vec![];
        for _ in 0..20_000_000 {
            let answer = match self {
                Self::Conditional(e) => match e.tick() {
                    chr_direct_conditional::engine::Event::Progress => None,
                    chr_direct_conditional::engine::Event::Answer(a) => Some(a),
                    chr_direct_conditional::engine::Event::Exhausted => {
                        if cfg!(feature = "metrics") {
                            work.applications = e.stats().applications;
                            work.choices = e.stats().births;
                            work.candidates = e.stats().discovered_tuples;
                        }
                        return (answers, first.unwrap_or(0), work);
                    }
                },
                Self::Explicit(e) => match e.tick() {
                    chr_compiled::SearchEvent::Complete(mut b) => {
                        let answer = b.engine.observe().expect("complete branch");
                        work.add(b.engine.stats());
                        Some(answer)
                    }
                    chr_compiled::SearchEvent::Exhausted => {
                        return (answers, first.unwrap_or(0), work);
                    }
                    chr_compiled::SearchEvent::Split { work: segment, .. } => {
                        if let Some(segment) = segment {
                            work.add(&segment);
                            work.choices += 1;
                        }
                        None
                    }
                    chr_compiled::SearchEvent::Failed(b) => {
                        work.add(b.engine.stats());
                        None
                    }
                    chr_compiled::SearchEvent::Progress => None,
                },
            };
            if let Some(a) = answer {
                if first.is_none() {
                    first = Some(start.elapsed().as_nanos());
                }
                answers.push(a);
            }
        }
        panic!("service bound exceeded; sample invalid");
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("meter-check") {
        #[cfg(feature = "alloc-meter")]
        {
            meter::self_check().unwrap();
            println!("meter check passed");
            return;
        }
        #[cfg(not(feature = "alloc-meter"))]
        panic!("allocation meter is not enabled");
    }
    assert_eq!(args.len(), 5, "backend family size queries");
    let backend = args[1].as_str();
    let family = args[2].as_str();
    assert!(["conditional", "explicit", "specialized"].contains(&backend));
    assert!(["plain", "shared", "discriminate", "output"].contains(&family));
    let n: usize = args[3].parse().unwrap();
    let queries: usize = args[4].parse().unwrap();
    assert!((1..=8).contains(&queries));
    assert!(n + usize::from(queries > 1) <= if family == "output" { 8 } else { 256 });
    let source = rules(family);
    let (prepared, preparation) = measure(|| match backend {
        "conditional" => Prepared::Conditional(
            chr_direct_conditional::engine::PreparedRuleset::new(source.clone()).unwrap(),
        ),
        "specialized" => Prepared::Explicit(
            chr_compiled::PreparedRuleset::new(source.clone(), None)
                .unwrap()
                .specialize_inferred(),
        ),
        _ => Prepared::Explicit(chr_compiled::PreparedRuleset::new(source.clone(), None).unwrap()),
    });
    let mut reports = vec![];
    let mut retained = Vec::with_capacity(queries);
    for i in 0..queries {
        let size = n + i % 2;
        let input = query(family, size);
        let count = if family == "plain" {
            1
        } else if family == "output" {
            1 << size
        } else {
            16
        };
        let (mut running, setup) = measure(|| prepared.start(input));
        let ((answers, first_ns, work), execution_observation) = measure(|| running.collect());
        let (_, disposal) = measure(|| drop(running));
        retained.push(answers);
        reports.push((
            size,
            count,
            setup,
            execution_observation,
            first_ns,
            disposal,
            work,
        ));
    }
    let (_, prepared_disposal) = measure(|| drop(prepared));
    // Batch validation cannot affect any backend setup/execution/engine disposal.
    // Answer disposal follows validation and is reported separately from backend lifecycle.
    let mut samples = vec![];
    for ((size, count, setup, run, first, dispose, work), answers) in
        reports.into_iter().zip(retained)
    {
        let expected = oracle::run(&source, &query(family, size), 2_000_000);
        assert_eq!(expected.len(), count);
        oracle::same_raw(answers.clone(), expected);
        let (_, answer_disposal) = measure(|| drop(answers));
        samples.push(format!("{{\"size\":{size},\"answers\":{count},\"setup\":{},\"execution_observation\":{},\"first_answer_ns\":{first},\"engine_disposal\":{},\"answer_disposal_after_validation\":{},\"work\":{}}}",setup.json(),run.json(),dispose.json(),answer_disposal.json(),work.json()));
    }
    let samples = samples.join(",");
    println!(
        "{{\"backend\":\"{backend}\",\"family\":\"{family}\",\"metrics\":{},\"allocation_meter\":{},\"preparation\":{},\"prepared_disposal\":{},\"samples\":[{samples}]}}",
        cfg!(feature = "metrics"),
        cfg!(feature = "alloc-meter"),
        preparation.json(),
        prepared_disposal.json()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_paths_match_independent_workload_answers() {
        for family in ["plain", "shared", "discriminate", "output"] {
            let source = rules(family);
            let prepared = [
                Prepared::Explicit(
                    chr_compiled::PreparedRuleset::new(source.clone(), None)
                        .unwrap()
                        .specialize_inferred(),
                ),
                Prepared::Conditional(
                    chr_direct_conditional::engine::PreparedRuleset::new(source.clone()).unwrap(),
                ),
                Prepared::Explicit(
                    chr_compiled::PreparedRuleset::new(source.clone(), None).unwrap(),
                ),
            ];
            for n in [0, 2, 4] {
                let input = query(family, n);
                let expected = oracle::run(&source, &input, 2_000_000);
                assert_eq!(
                    expected.len(),
                    if family == "plain" {
                        1
                    } else if family == "output" {
                        1 << n
                    } else {
                        16
                    }
                );
                for p in &prepared {
                    oracle::same_raw(p.start(input.clone()).collect().0, expected.clone());
                }
            }
        }
    }
}
