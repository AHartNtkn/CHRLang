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
fn measured<T>(
    phases: &mut Vec<(&'static str, meter::Reading)>,
    name: &'static str,
    f: impl FnOnce() -> T,
) -> T {
    let begin = meter::begin();
    let result = f();
    let reading = meter::end(begin);
    phases.push((name, reading));
    result
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 7, "mode family fail resource depth reverse");
    let family = match args[2].as_str() {
        "repeated" => "repeated",
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
    let depth = args[5].parse().unwrap();
    let reverse = args[6].parse().unwrap();
    let rules = schema.rules();
    let queries = (0..2)
        .map(|seed| {
            let mut q = schema.query(depth, reverse);
            q.constraints
                .push(c("query_marker", [atom(&format!("q{seed}"))]));
            q
        })
        .collect::<Vec<_>>();
    let expected = queries
        .iter()
        .map(|q| oracle::run(&rules, q, 2_000_000))
        .collect::<Vec<_>>();
    let mut phases = Vec::with_capacity(9);
    #[cfg(feature = "validity-profile")]
    profile::enable();
    let root = meter::begin();
    let prepared = measured(&mut phases, "prepare", || {
        runtime::Prepared::new(&args[1], schema, rules.clone())
    });
    let mut held = Vec::new();
    for query in &queries {
        let mut engine = measured(&mut phases, "setup", || prepared.start(query.clone()));
        measured(&mut phases, "execute", || {
            let mut answers = Vec::new();
            let mut exhausted = false;
            for _ in 0..2_000_000 {
                match engine.tick() {
                    runtime::Event::Progress => (),
                    runtime::Event::Answer(a) => answers.push(a),
                    runtime::Event::Done => {
                        exhausted = true;
                        break;
                    }
                }
            }
            assert!(exhausted, "source service cutoff");
            held.push(answers);
        });
        measured(&mut phases, "producer_drop", || drop(engine));
    }
    measured(&mut phases, "prepared_drop", || drop(prepared));
    for (answers, wanted) in held.iter().zip(&expected) {
        oracle::same_raw(answers.clone(), wanted.clone());
    }
    measured(&mut phases, "consumer_drop", || drop(held));
    let end = meter::end(root);
    assert_eq!(end.live_start, end.live_end);
    let rows = phases
        .iter()
        .map(|(name, r)| format!("{{\"phase\":\"{name}\",\"heap\":{}}}", r.json()))
        .collect::<Vec<_>>()
        .join(",");
    #[cfg(feature = "validity-profile")]
    let diagnostics = profile::json();
    #[cfg(not(feature = "validity-profile"))]
    let diagnostics = String::new();
    println!(
        "{{\"validated\":true,\"profiled\":{},\"answers\":{},\"phases\":[{rows}],\"profile\":[{diagnostics}]}}",
        cfg!(feature = "validity-profile"),
        expected.iter().map(Vec::len).sum::<usize>()
    );
}
