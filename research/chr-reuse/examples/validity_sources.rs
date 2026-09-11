//! Complete finite source and ownership controls for validity attribution.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[allow(dead_code)]
#[path = "support/stream_run.rs"]
mod runtime;
#[allow(dead_code)]
#[path = "support/stream_source.rs"]
mod source;
#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use chr_syntax::{atom, c};
#[cfg(feature = "validity-profile")]
mod profile {
    use super::meter;
    use chr_direct_choice::demand::validity_profile::{self as work, Site};
    use std::cell::RefCell;
    #[derive(Default)]
    struct Profile {
        active: Option<(usize, usize, usize, usize)>,
        totals: [(usize, usize, usize); 12],
    }
    thread_local! {static PROFILE: RefCell<Profile> = RefCell::new(Profile::default());}
    fn event(site: Site, enter: bool) {
        let n = meter::checkpoint();
        PROFILE.with(|p| {
            let mut p = p.borrow_mut();
            if enter {
                assert!(p.active.is_none());
                p.active = Some((
                    site as usize,
                    n.requested_bytes,
                    n.allocation_calls,
                    n.deallocation_calls,
                ));
            } else {
                let (i, bytes, calls, frees) = p.active.take().unwrap();
                assert_eq!(i, site as usize);
                p.totals[i].0 += n.requested_bytes - bytes;
                p.totals[i].1 += n.allocation_calls - calls;
                p.totals[i].2 += n.deallocation_calls - frees;
            }
        });
    }
    pub fn enable() {
        PROFILE.with(|p| *p.borrow_mut() = Profile::default());
        work::reset();
        work::set_callback(event);
    }
    pub fn json() -> String {
        let rows = work::snapshot();
        PROFILE.with(|p| {let p=p.borrow();assert!(p.active.is_none());
            rows.iter().enumerate().map(|(i,r)| {let (bytes,calls,frees)=p.totals[i];format!("{{\"site\":\"{}\",\"calls\":{},\"accepted\":{},\"support\":{},\"context\":{},\"visited\":{},\"steps\":{},\"seeks\":{},\"requested_bytes\":{bytes},\"allocation_calls\":{calls},\"deallocation_calls\":{frees}}}",work::NAMES[i],r.calls,r.accepted,r.support,r.context,r.visited,r.steps,r.seeks)}).collect::<Vec<_>>().join(",")
        })
    }
}
struct Reading {
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    heap: meter::Reading,
}
impl Reading {
    fn json(&self) -> String {
        #[cfg(feature = "alloc-meter")]
        let heap = self.heap.json();
        #[cfg(not(feature = "alloc-meter"))]
        let heap = "null";
        format!("\"ns\":{},\"heap\":{heap}", self.ns)
    }
}
fn measured<T>(
    phases: &mut Vec<(&'static str, Reading)>,
    name: &'static str,
    f: impl FnOnce() -> T,
) -> T {
    #[cfg(feature = "alloc-meter")]
    let begin = meter::begin();
    let clock = std::time::Instant::now();
    let result = f();
    let ns = clock.elapsed().as_nanos();
    let reading = Reading {
        ns,
        #[cfg(feature = "alloc-meter")]
        heap: meter::end(begin),
    };
    phases.push((name, reading));
    result
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert!(
        [7, 8, 9, 10].contains(&args.len()),
        "mode family fail resource depth reverse [complete-sessions] [report-capacity] [cancel-first-query]"
    );
    let sessions = args.get(7).map_or(1, |v| v.parse::<usize>().unwrap());
    assert!(sessions > 0 && sessions <= 100);
    let family = match args[2].as_str() {
        "repeated" | "chain" => "repeated",
        "distinct" => "distinct",
        _ => panic!("family"),
    };
    let schema = source::Schema {
        family,
        fail_tail: args[3].parse().unwrap(),
        resource: args[4].parse().unwrap(),
        work: 2,
        payload: 1,
    };
    let depth: usize = args[5].parse().unwrap();
    let reverse = args[6].parse().unwrap();
    let chain = args[2] == "chain";
    let cancel = args.get(9).is_some_and(|v| v.parse::<bool>().unwrap());
    let mut rules = schema.rules();
    if chain {
        rules.retain(|r| r.name != "emit-or-continue" && r.name != "last");
        if schema.fail_tail {
            rules
                .iter_mut()
                .find(|r| r.name == "wait-done")
                .unwrap()
                .body = chr_syntax::Goal::Fail;
        }
    }
    let queries = (0..2)
        .map(|seed| {
            let mut q = schema.query(depth + seed, reverse);
            if chain {
                let head = q
                    .constraints
                    .iter_mut()
                    .find(|c| c.name == "stream")
                    .unwrap();
                head.name = "wait".into();
                head.args.remove(2);
            }
            q.constraints
                .push(c("query_marker", [atom(&format!("q{seed}"))]));
            q
        })
        .collect::<Vec<_>>();
    let expected = queries
        .iter()
        .map(|q| oracle::run(&rules, q, 2_000_000))
        .collect::<Vec<_>>();
    for _ in 0..sessions {
        let mut phases = Vec::with_capacity(args.get(8).map_or(9, |v| v.parse::<usize>().unwrap()));
        #[cfg(feature = "validity-profile")]
        profile::enable();
        #[cfg(feature = "alloc-meter")]
        let root = meter::begin();
        let prepared = measured(&mut phases, "prepare", || {
            runtime::Prepared::new(&args[1], schema, rules.clone())
        });
        let mut held = Vec::new();
        for (query_index, query) in queries.iter().enumerate() {
            let mut engine = measured(&mut phases, "setup", || prepared.start(query.clone()));
            measured(&mut phases, "execute", || {
                let mut answers = Vec::new();
                let mut exhausted = false;
                for tick in 0..2_000_000 {
                    match engine.tick() {
                        runtime::Event::Progress => (),
                        runtime::Event::Answer(a) => answers.push(a),
                        runtime::Event::Done => {
                            exhausted = true;
                            break;
                        }
                    }
                    if cancel && query_index == 0 && tick == 0 {
                        break;
                    }
                }
                assert!(
                    exhausted || (cancel && query_index == 0),
                    "source service cutoff"
                );
                held.push(answers);
            });
            measured(&mut phases, "producer_drop", || drop(engine));
        }
        measured(&mut phases, "prepared_drop", || drop(prepared));
        for (index, (answers, wanted)) in held.iter().zip(&expected).enumerate() {
            if cancel && index == 0 {
                assert!(
                    answers.is_empty(),
                    "one source tick must precede any answer"
                );
            } else {
                oracle::same_raw(answers.clone(), wanted.clone());
            }
        }
        let answer_count = held.iter().map(Vec::len).sum::<usize>();
        measured(&mut phases, "consumer_drop", || drop(held));
        #[cfg(feature = "alloc-meter")]
        {
            let end = meter::end(root);
            assert_eq!(end.live_start, end.live_end);
        }
        let rows = phases
            .iter()
            .map(|(name, r)| format!("{{\"phase\":\"{name}\",{}}}", r.json()))
            .collect::<Vec<_>>()
            .join(",");
        #[cfg(feature = "validity-profile")]
        let diagnostics = profile::json();
        #[cfg(not(feature = "validity-profile"))]
        let diagnostics = String::new();
        println!(
            "{{\"validated\":true,\"profiled\":{},\"answers\":{},\"phases\":[{rows}],\"profile\":[{diagnostics}]}}",
            cfg!(feature = "validity-profile"),
            answer_count
        );
    }
}
