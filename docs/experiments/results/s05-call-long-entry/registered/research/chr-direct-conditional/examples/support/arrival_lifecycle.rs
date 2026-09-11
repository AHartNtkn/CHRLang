// Order/observation lifecycle; independent full replay before measurement.
#[path = "../../tests/composition_support/mod.rs"]
#[allow(dead_code)]
mod engines;
#[cfg(feature = "alloc-meter")]
#[path = "../../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../../tests/runtime_support/mod.rs"]
mod oracle;
#[allow(dead_code)]
#[path = "sustained_run.rs"]
mod ordinary;
#[path = "arrival_run.rs"]
mod runtime;
#[path = "order_source.rs"]
mod source;
use runtime::{Event, MODES, Prepared};
use source::Schema;
use std::{collections::VecDeque, time::Instant};
struct Phase {
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn measure<T>(f: impl FnOnce() -> T) -> (T, Phase) {
    #[cfg(feature = "alloc-meter")]
    let m = meter::begin();
    let start = Instant::now();
    let value = f();
    let ns = start.elapsed().as_nanos();
    (
        value,
        Phase {
            ns,
            #[cfg(feature = "alloc-meter")]
            memory: meter::end(m),
        },
    )
}
impl Phase {
    fn json(&self) -> String {
        #[cfg(feature = "alloc-meter")]
        let memory = self.memory.json();
        #[cfg(not(feature = "alloc-meter"))]
        let memory = "null";
        format!("{{\"ns\":{},\"memory\":{memory}}}", self.ns)
    }
}
struct Sample {
    depth: usize,
    answers: usize,
    complete: bool,
    first: Option<u128>,
    input: Phase,
    setup: Phase,
    execute: Phase,
    engine_drop: Phase,
}
#[cfg(feature = "alloc-meter")]
struct Snapshot {
    query: usize,
    label: &'static str,
    answers: usize,
    retained: usize,
    baseline: usize,
    live: usize,
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.get(1).is_some_and(|x| x == "meter-check") {
        #[cfg(feature = "alloc-meter")]
        {
            meter::self_check().unwrap();
            println!("meter self-check passed");
            return;
        }
        #[cfg(not(feature = "alloc-meter"))]
        panic!("meter required");
    }
    if cfg!(feature = "metrics")
        || chr_direct_choice::demand::COLLECT_WORK_DIAGNOSTICS
        || chr_compiled::COLLECT_METRICS
        || chr_compiled::COLLECT_KERNEL_METRICS
        || chr_observe::COLLECT_METRICS
    {
        panic!("cost runner requires counters disabled");
    }
    assert_eq!(
        args.len(),
        12,
        "mode family depth work payload resource keep cancel reuse fail_tail signature"
    );
    let mode = args[1].as_str();
    assert!(MODES.contains(&mode));
    let family = match args[2].as_str() {
        "repeated" => "repeated",
        "distinct" => "distinct",
        "aliases" => "aliases",
        "oldest-first" => "oldest-first",
        "newest-first" => "newest-first",
        _ => panic!("family"),
    };
    let n = args[3].parse::<usize>().unwrap();
    assert!(matches!(n, 0 | 4 | 8 | 16 | 64));
    let work = args[4].parse::<usize>().unwrap();
    assert!(matches!(work, 0 | 4));
    let payload = args[5].parse::<usize>().unwrap();
    assert!(matches!(payload, 0 | 8));
    let resource = match args[6].as_str() {
        "0" => false,
        "1" => true,
        _ => panic!("resource"),
    };
    let keep = match args[7].as_str() {
        "0" => 0,
        "4" => 4,
        "all" => usize::MAX,
        _ => panic!("consumer"),
    };
    let cancel = match args[8].as_str() {
        "0" => false,
        "1" | "2" => true,
        _ => panic!("cancel"),
    };
    assert!(!cancel || (args[8]=="1" && matches!(family,"aliases"|"distinct"|"repeated")) || (args[8]=="2" && family.ends_with("-first") && n>=4));
    let reuse: usize = args[9].parse().unwrap();
    assert!(matches!(reuse, 1 | 4));
    assert!(matches!(args[10].as_str(), "0" | "1"));
    let signature=args[11].as_str();
    assert!(matches!(signature,"same"|"changing"));
    let depth=|q:usize| if signature=="same" {n} else {n+q};
    let query_schema=|q:usize| Schema{work:if signature=="same" && q%2==1 {0}else{work},payload:if signature=="same" && q%2==1 {0}else{payload},family,resource,fail_tail:args[10]=="1"};
    let input=|q:usize| {
        let sc=query_schema(q);
        let mut query=sc.query(depth(q),signature=="changing" && q%2==1);
        if signature=="same" && q%2==1 {
            for c in &mut query.constraints {if c.name=="tag" {c.args=vec![chr_syntax::atom("second")];}}
        }
        query
    };
    let schema = Schema {
        family,
        resource,
        fail_tail: args[10] == "1",
        work,
        payload,
    };

    {
        let mut p = Prepared::new(mode, schema.rules());
        for q in 0..reuse {
            let input = input(q);
            let expected = oracle::run(&schema.rules(), &input, 2_000_000);
            oracle::same_raw(expected.clone(), query_schema(q).expected_query(depth(q),q%2==1));
            let mut run = p.start(input);
            let mut answers = vec![];
            let mut done = false;
            for _ in 0..2_000_000 {
                match run.step() {
                    Event::Answer(a) => {
                        answers.push(a);
                    }
                    Event::Exhausted => {
                        done = true;
                        break;
                    }
                    Event::Progress => (),
                }
            }
            assert!(done);
            oracle::same_raw(answers, expected);
        }
    }
    let mut consumer = VecDeque::with_capacity(reuse * (n + reuse + 1));
    let mut samples = Vec::with_capacity(reuse);
    #[cfg(feature = "alloc-meter")]
    let mut snapshots = Vec::with_capacity(10 * reuse + 4);
    println!("{{\"event\":\"start\"}}");
    #[cfg(feature = "alloc-meter")]
    let root = meter::begin();
    let (rules, source_build) = measure(|| schema.rules());
    let (mut p, preparation) = measure(|| Prepared::new(mode, rules));
    for q in 0..reuse {
        #[cfg(feature = "alloc-meter")]
        let baseline = meter::end(root).live_end;
        let (input, input_time) = measure(|| input(q));
        let (mut run, setup) = measure(|| p.start(input));
        #[cfg(feature = "alloc-meter")]
        snapshots.push(Snapshot {
            query: q,
            label: "setup",
            answers: 0,
            retained: consumer.len(),
            baseline,
            live: meter::end(root).live_end,
        });
        let ((answers, complete, first), execute) = measure(|| {
            let start = Instant::now();
            let mut answers = 0;
            let mut first = None;
            let mut complete = false;
            for service in 0..2_000_000 {
                if cancel && args[8]=="2" && q==0 && service==100 {break;}
                match run.step() {
                    Event::Answer(a) => {
                        answers += 1;
                        if first.is_none() {
                            first = Some(start.elapsed().as_nanos());
                        }
                        consumer.push_back(a);
                        if consumer.len() > keep {
                            consumer.pop_front();
                        }
                        #[cfg(feature = "alloc-meter")]
                        if [1, 4, 16, 64].contains(&answers) {
                            snapshots.push(Snapshot {
                                query: q,
                                label: "delivery",
                                answers,
                                retained: consumer.len(),
                                baseline,
                                live: meter::end(root).live_end,
                            });
                        }
                        if cancel && args[8]=="1" && q == 0 && answers == 4 {
                            break;
                        }
                    }
                    Event::Exhausted => {
                        complete = true;
                        break;
                    }
                    Event::Progress => (),
                }
            }
            (answers, complete, first)
        });
        assert_eq!(complete, !(cancel && q == 0));
        if !(cancel && args[8]=="2" && q==0) {assert_eq!(
            answers,
            if cancel && q == 0 {
                4
            } else {
                query_schema(q).expected_query(depth(q),q%2==1).len()
            }
        );
        }
        #[cfg(feature = "alloc-meter")]
        snapshots.push(Snapshot {
            query: q,
            label: if complete { "exhausted" } else { "cancelled" },
            answers,
            retained: consumer.len(),
            baseline,
            live: meter::end(root).live_end,
        });
        let (_, engine_drop) = measure(|| drop(run));
        #[cfg(feature = "alloc-meter")]
        {
            let live = meter::end(root).live_end;

            snapshots.push(Snapshot {
                query: q,
                label: "engine-disposed",
                answers,
                retained: consumer.len(),
                baseline,
                live,
            });
        }
        samples.push(Sample {
            depth: depth(q),
            answers,
            complete,
            first,
            input: input_time,
            setup,
            execute,
            engine_drop,
        });
    }
    let artifacts=p.artifact_count();
    let (_, artifact_drop)=measure(||p.clear_artifacts());
    let (_, consumer_drop) = measure(|| consumer.clear());
    #[cfg(feature = "alloc-meter")]
    assert_eq!(
        meter::end(root).live_end,
        preparation.memory.live_end,
        "query/consumer owners remain"
    );
    let (_, prepared_drop) = measure(|| drop(p));
    #[cfg(feature = "alloc-meter")]
    {
        let r = meter::end(root);
        assert_eq!(r.live_start, r.live_end, "prepared owners remain");
    }
    let rows=samples.iter().map(|s|format!("{{\"depth\":{},\"answers\":{},\"complete\":{},\"first_answer_ns\":{},\"input_build\":{},\"setup\":{},\"execute_observe\":{},\"engine_drop\":{}}}",s.depth,s.answers,s.complete,s.first.map_or("null".into(),|n|n.to_string()),s.input.json(),s.setup.json(),s.execute.json(),s.engine_drop.json())).collect::<Vec<_>>().join(",");
    #[cfg(feature="alloc-meter")]
    let snaps=snapshots.iter().map(|s|format!("{{\"query\":{},\"label\":\"{}\",\"answers\":{},\"retained\":{},\"baseline\":{},\"live\":{}}}",s.query,s.label,s.answers,s.retained,s.baseline,s.live)).collect::<Vec<_>>().join(",");
    #[cfg(not(feature = "alloc-meter"))]
    let snaps = "";
    let order = if cfg!(feature = "support-reverse-order") { "reverse" } else { "ascending" };
    let observer = if cfg!(feature = "support-generic-histories") { "general" } else { "direct" };
    let policy = match (
        cfg!(feature = "support-identities"),
        cfg!(feature = "support-result-cache"),
    ) {
        (false, false) => "ordinary",
        (true, false) => "identities",
        (false, true) => "cache",
        (true, true) => "combined",
    };
    println!(
        "{{\"signature\":\"{signature}\",\"artifacts\":{artifacts},\"artifact_drop\":{},\"order\":\"{order}\",\"observer\":\"{observer}\",\"mode\":\"{mode}\",\"policy\":\"{policy}\",\"reuse\":{reuse},\"fail_tail\":{},\"family\":\"{family}\",\"depth\":{n},\"work\":{work},\"payload\":{payload},\"resource\":{resource},\"keep\":\"{}\",\"cancel\":{cancel},\"meter\":{},\"counters\":false,\"source_build\":{},\"preparation\":{},\"prepared_drop\":{},\"consumer_drop\":{},\"samples\":[{rows}],\"snapshots\":[{snaps}]}}",
        artifact_drop.json(),
        schema.fail_tail,
        args[7],
        cfg!(feature = "alloc-meter"),
        source_build.json(),
        preparation.json(),
        prepared_drop.json(),
        consumer_drop.json()
    );
}
