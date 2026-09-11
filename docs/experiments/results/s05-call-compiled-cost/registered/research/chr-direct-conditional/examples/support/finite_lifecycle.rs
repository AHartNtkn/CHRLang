#[path = "finite_bridge.rs"]
mod bridge;
#[allow(dead_code)]
#[path = "../../tests/composition_support/mod.rs"]
mod engines;
#[allow(dead_code)]
#[path = "../../../chr-compiled/experiments/finite_phase.rs"]
mod finite_phase;
#[cfg(feature = "alloc-meter")]
#[path = "../../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../../tests/runtime_support/mod.rs"]
mod oracle;
#[path = "finite_cost_source.rs"]
mod source;
use chr_syntax::{Answer, Query, Rule};
use source::Schema;
use std::{
    collections::{BTreeMap, VecDeque},
    time::Instant,
};
struct Phase {
    name: &'static str,
    query: usize,
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn mark<T>(phases: &mut Vec<Phase>, query: usize, name: &'static str, f: impl FnOnce() -> T) -> T {
    assert!(phases.len() < phases.capacity(), "phase buffer");
    #[cfg(feature = "alloc-meter")]
    let mem = meter::begin();
    let start = Instant::now();
    let result = f();
    let ns = start.elapsed().as_nanos();
    #[cfg(feature = "alloc-meter")]
    let memory = meter::end(mem);
    phases.push(Phase {
        name,
        query,
        ns,
        #[cfg(feature = "alloc-meter")]
        memory,
    });
    result
}
impl Phase {
    fn json(&self) -> String {
        #[cfg(feature = "alloc-meter")]
        let mem = self.memory.json();
        #[cfg(not(feature = "alloc-meter"))]
        let mem = "null";
        format!(
            "{{\"name\":\"{}\",\"query\":{},\"ns\":{},\"memory\":{mem}}}",
            self.name, self.query, self.ns
        )
    }
}
#[allow(clippy::large_enum_variant)]
enum Prepared {
    Finite {
        phase: finite_phase::Prepared,
        caller: bridge::Bridge,
    },
    Compiled(chr_compiled::PreparedRuleset),
    Conditional(chr_direct_conditional::engine::PreparedRuleset),
    Prefix {
        program: chr_compiled::pure_prefix::Program,
        cache: BTreeMap<Vec<(String, usize)>, chr_compiled::pure_prefix::Artifact>,
    },
}
impl Prepared {
    fn new(mode: &str, rules: Vec<Rule>) -> Self {
        match mode {
            "finite" => Self::Finite {
                phase: finite_phase::Prepared::new(&rules, 6).unwrap(),
                caller: bridge::Bridge::new(rules),
            },
            "scan" | "specialized" => {
                let p = chr_compiled::PreparedRuleset::new(rules, None).unwrap();
                Self::Compiled(if mode == "specialized" {
                    p.specialize_inferred()
                } else {
                    p
                })
            }
            "conditional" => Self::Conditional(
                chr_direct_conditional::engine::PreparedRuleset::with_head_contract(
                    rules,
                    None,
                    chr_direct_conditional::engine::HeadAdmission::Optional,
                )
                .unwrap(),
            ),
            "prepared-prefix" => Self::Prefix {
                program: chr_compiled::pure_prefix::Program::new(&rules).unwrap(),
                cache: BTreeMap::new(),
            },
            _ => panic!("mode"),
        }
    }
    fn ordinary(&mut self, q: Query) -> engines::Engine {
        match self {
            Self::Compiled(p) => engines::Engine::Compiled(
                p.start_search(q, chr_compiled::Policy::Global, chr_compiled::Access::Scan)
                    .unwrap(),
            ),
            Self::Conditional(p) => engines::Engine::Conditional(p.start(q).unwrap()),
            Self::Prefix { program, cache } => {
                let signature = q
                    .constraints
                    .iter()
                    .map(|c| (c.name.clone(), c.args.len()))
                    .collect::<Vec<_>>();
                let a = match cache.entry(signature) {
                    std::collections::btree_map::Entry::Occupied(e) => e.into_mut(),
                    std::collections::btree_map::Entry::Vacant(e) => {
                        let a = program.prepare_shape(e.key()).unwrap();
                        e.insert(a)
                    }
                };
                engines::Engine::Compiled(a.start(&q, chr_compiled::Access::Scan).unwrap())
            }
            Self::Finite { .. } => unreachable!(),
        }
    }
    fn artifacts(&self) -> usize {
        if let Self::Prefix { cache, .. } = self {
            cache.len()
        } else {
            0
        }
    }
    fn clear(&mut self) {
        if let Self::Prefix { cache, .. } = self {
            cache.clear();
        }
    }
}
struct Delivery<'a> {
    consumer: &'a mut VecDeque<Answer>,
    keep: usize,
    cancel: &'a str,
    answers: usize,
    calls: usize,
    first: Option<u128>,
    start: Instant,
}
impl Delivery<'_> {
    fn stop(&self) -> bool {
        match self.cancel {
            "none" => false,
            "step4" => self.calls >= 4,
            "answer2" => self.answers >= 2,
            _ => panic!("cancellation"),
        }
    }
    fn answer(&mut self, a: Answer) {
        self.answers += 1;
        if self.first.is_none() {
            self.first = Some(self.start.elapsed().as_nanos());
        }
        assert!(
            self.consumer.len() < self.consumer.capacity(),
            "consumer buffer"
        );
        self.consumer.push_back(a);
        if self.consumer.len() > self.keep {
            self.consumer.pop_front();
        }
    }
}
struct Sample {
    depth: usize,
    answers: usize,
    calls: usize,
    complete: bool,
    first: Option<u128>,
    artifacts: usize,
}
struct QueryRun<'a> {
    depth: usize,
    index: usize,
    cancel: &'a str,
    keep: usize,
}
fn run_query(
    p: &mut Prepared,
    s: Schema,
    options: QueryRun<'_>,
    consumer: &mut VecDeque<Answer>,
    phases: &mut Vec<Phase>,
) -> Sample {
    let QueryRun {
        depth: n,
        index,
        cancel,
        keep,
    } = options;
    let start = Instant::now();
    let input = mark(phases, index, "input_build", || s.query(n, index % 2 == 1));
    let mut output = Delivery {
        consumer,
        keep,
        cancel,
        answers: 0,
        calls: 0,
        first: None,
        start,
    };
    let complete;
    if let Prepared::Finite { phase, caller } = p {
        let mut machine = mark(phases, index, "solver_setup", || {
            let m = phase.start(&input, Default::default()).unwrap();
            drop(input);
            m
        });
        let report = mark(phases, index, "private_solve", || {
            loop {
                if output.stop() {
                    break None;
                }
                assert!(output.calls < 8_000_000, "query service cutoff");
                output.calls += 1;
                match machine.advance().unwrap() {
                    finite_phase::Event::Progress => (),
                    finite_phase::Event::Complete(r) => break Some(r),
                    finite_phase::Event::Exhausted => panic!("missing phase result"),
                }
            }
        });
        mark(phases, index, "solver_drop", || drop(machine));
        let admitted = report.is_some();
        let mut rows = report.map(|r| r.solutions.into_iter());
        if let Some(rows) = rows.as_mut() {
            while !output.stop() {
                let Some(solution) = rows.next() else {
                    break;
                };
                let (q, w) = mark(phases, index, "transport", || caller.transport(solution));
                let mut run = mark(phases, index, "caller_setup", || caller.start(q, w));
                mark(phases, index, "caller_execute_observe", || {
                    loop {
                        if output.stop() {
                            break;
                        }
                        assert!(output.calls < 8_000_000, "query service cutoff");
                        output.calls += 1;
                        match run.step() {
                            bridge::Event::Progress => (),
                            bridge::Event::Answer(a) => output.answer(a),
                            bridge::Event::Exhausted => break,
                        }
                    }
                });
                mark(phases, index, "caller_drop", || drop(run));
            }
        }
        complete = admitted && !output.stop();
        mark(phases, index, "query_drop", || drop(rows));
    } else {
        let mut run = mark(phases, index, "query_setup", || p.ordinary(input));
        complete = mark(phases, index, "execute_observe", || {
            loop {
                if output.stop() {
                    break false;
                }
                assert!(output.calls < 8_000_000, "query service cutoff");
                output.calls += 1;
                match run.step() {
                    engines::Event::Progress => (),
                    engines::Event::Answer(a) => output.answer(a),
                    engines::Event::Exhausted => break true,
                }
            }
        });
        mark(phases, index, "query_drop", || drop(run));
    }
    Sample {
        depth: n,
        answers: output.answers,
        calls: output.calls,
        complete,
        first: output.first,
        artifacts: p.artifacts(),
    }
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.get(1).is_some_and(|s| s == "meter-check") {
        #[cfg(feature = "alloc-meter")]
        {
            meter::self_check().unwrap();
            println!("meter passed");
            return;
        }
        #[cfg(not(feature = "alloc-meter"))]
        panic!("meter required");
    }
    assert_eq!(
        args.len(),
        9,
        "mode family depth reuse keep cancel fail work"
    );
    const {
        assert!(
            !cfg!(feature = "metrics")
                && !chr_compiled::COLLECT_METRICS
                && !chr_compiled::COLLECT_KERNEL_METRICS
                && !chr_observe::COLLECT_METRICS
                && !chr_direct_choice::demand::COLLECT_WORK_DIAGNOSTICS
        );
    }
    let mode = args[1].as_str();
    let family = match args[2].as_str() {
        "oldest" => "oldest",
        "newest" => "newest",
        "all" => "all",
        "duplicates" => "duplicates",
        _ => panic!("family"),
    };
    let n: usize = args[3].parse().unwrap();
    assert!(matches!(n, 0 | 4 | 8));
    let reuse: usize = args[4].parse().unwrap();
    assert!(matches!(reuse, 1 | 4));
    assert!(reuse == 1 || n <= 4);
    let keep = match args[5].as_str() {
        "0" => 0,
        "4" => 4,
        "all" => usize::MAX,
        _ => panic!("keep"),
    };
    let cancel = args[6].as_str();
    assert!(matches!(cancel, "none" | "step4" | "answer2"));
    assert!(cancel == "none" || n >= 4);
    assert!(cancel != "answer2" || matches!(family, "all" | "duplicates"));
    assert!(matches!(args[7].as_str(), "0" | "1"));
    let fail = args[7] == "1";
    assert!(!fail || cancel != "answer2");
    let work: usize = args[8].parse().unwrap();
    assert!(matches!(work, 0 | 2));
    let s = Schema {
        family,
        work,
        payload: 4,
        fail,
    };
    let max_answers = (1 << (n + reuse)) * reuse;
    let phase_capacity = 32 + reuse * (16 + 4 * (1 << (n + reuse)));
    // Independent complete replay on precisely the same mode and changing queries.
    {
        let mut p = Prepared::new(mode, s.rules());
        let mut phases = Vec::with_capacity(phase_capacity);
        let mut consumer = VecDeque::with_capacity(max_answers + 1);
        for q in 0..reuse {
            phases.clear();
            let result = run_query(
                &mut p,
                s,
                QueryRun {
                    depth: n + q,
                    index: q,
                    cancel: "none",
                    keep: usize::MAX,
                },
                &mut consumer,
                &mut phases,
            );
            assert!(result.complete);
            let expected = oracle::run(&s.rules(), &s.query(n + q, q % 2 == 1), 2_000_000);
            oracle::same_raw(expected.clone(), s.expected(n + q, q % 2 == 1));
            oracle::same_raw(consumer.drain(..).collect(), expected);
        }
    }
    let mut phases = Vec::with_capacity(phase_capacity);
    let mut consumer = VecDeque::with_capacity(max_answers + 1);
    let mut samples = Vec::with_capacity(reuse);
    println!("{{\"event\":\"start\"}}");
    #[cfg(feature = "alloc-meter")]
    let root = meter::begin();
    let rules = mark(&mut phases, reuse, "source_build", || s.rules());
    let mut p = mark(&mut phases, reuse, "preparation", || {
        Prepared::new(mode, rules)
    });
    #[cfg(feature = "alloc-meter")]
    let prepared_live = meter::end(root).live_end;
    for q in 0..reuse {
        let cancellation = if q == 0 { cancel } else { "none" };
        let sample = run_query(
            &mut p,
            s,
            QueryRun {
                depth: n + q,
                index: q,
                cancel: cancellation,
                keep,
            },
            &mut consumer,
            &mut phases,
        );
        assert_eq!(sample.complete, cancellation == "none");
        if sample.complete {
            assert_eq!(sample.answers, s.count(n + q));
        }
        if cancellation == "answer2" {
            assert_eq!(sample.answers, 2);
        }
        if cancellation == "step4" {
            assert_eq!(sample.calls, 4);
        }
        samples.push(sample);
    }
    let retained = consumer.len();
    mark(&mut phases, reuse, "consumer_drop", || consumer.clear());
    let artifacts = p.artifacts();
    mark(&mut phases, reuse, "artifact_drop", || p.clear());
    #[cfg(feature = "alloc-meter")]
    assert_eq!(
        meter::end(root).live_end,
        prepared_live,
        "query/consumer owner remains"
    );
    mark(&mut phases, reuse, "prepared_drop", || drop(p));
    #[cfg(feature = "alloc-meter")]
    {
        let r = meter::end(root);
        assert_eq!(r.live_start, r.live_end, "prepared owner remains");
    }
    let pp = phases.iter().map(Phase::json).collect::<Vec<_>>().join(",");
    let rows=samples.iter().map(|s|format!("{{\"depth\":{},\"answers\":{},\"calls\":{},\"complete\":{},\"first_answer_ns\":{},\"artifacts\":{}}}",s.depth,s.answers,s.calls,s.complete,s.first.map_or("null".into(),|x|x.to_string()),s.artifacts)).collect::<Vec<_>>().join(",");
    let order = if cfg!(feature = "support-reverse-order") {
        "reverse"
    } else {
        "ascending"
    };
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"depth\":{n},\"reuse\":{reuse},\"keep\":\"{}\",\"cancel\":\"{cancel}\",\"fail\":{fail},\"work\":{work},\"order\":\"{order}\",\"meter\":{},\"counters\":false,\"retained\":{retained},\"artifacts\":{artifacts},\"samples\":[{rows}],\"phases\":[{pp}]}}",
        args[5],
        cfg!(feature = "alloc-meter")
    );
}
