//! Separate counters-only companion to recursive_cost; never used for timings.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "support/recursive_source.rs"]
mod recursive_source;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent, Stats};
use recursive_source::Schema;
#[derive(Default)]
struct Work {
    applications: u64,
    candidates: u64,
    steps: u64,
    checks: u64,
}
impl Work {
    fn add(&mut self, s: &Stats) {
        self.applications += s.applications;
        self.candidates += s.candidate_visits + s.specialized_candidates;
        #[cfg(feature = "carrier-contraction")]
        {
            self.steps += s.carrier_steps;
            self.checks += s.carrier_checks;
        }
    }
}
fn prepare(mode: &str, schema: Schema) -> PreparedRuleset {
    let p = PreparedRuleset::new(schema.rules(), None).unwrap();
    match mode {
        "original" => p,
        "sealed" => p.specialize_inferred(),
        "contracted" => {
            #[cfg(feature = "carrier-contraction")]
            {
                p.specialize_inferred()
                    .contract_carriers_checked(&[("fold".into(), 3)])
                    .unwrap()
            }
            #[cfg(not(feature = "carrier-contraction"))]
            {
                panic!("contraction feature required")
            }
        }
        _ => panic!("unknown mode"),
    }
}
fn main() {
    assert!(
        chr_compiled::COLLECT_METRICS
            && chr_compiled::COLLECT_KERNEL_METRICS
            && chr_observe::COLLECT_METRICS
    );
    assert!(!cfg!(feature = "alloc-meter"));
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 6);
    let mode = &args[1];
    let family = &args[2];
    let n = args[3].parse::<usize>().unwrap();
    let count = args[4].parse::<usize>().unwrap();
    assert!(n <= 128 && (1..=64).contains(&count));
    let resource = match args[5].as_str() {
        "0" => false,
        "1" => true,
        _ => panic!("resource flag"),
    };
    let schema = Schema::new(family, resource, n);
    let p = prepare(mode, schema);
    let mut samples = vec![];
    for i in 0..count {
        let q = schema.query(n + i % 2, i % 2 == 0);
        let expected = oracle::run(&schema.rules(), &q, 2_000_000);
        assert_eq!(expected.len(), schema.answer_count());
        let mut e = p.start_search(q, Policy::Global, Access::Indexed).unwrap();
        let mut work = Work::default();
        let mut answers = vec![];
        let mut exhausted = false;
        for _ in 0..2_000_000 {
            match e.tick() {
                SearchEvent::Split { work: Some(s), .. } => work.add(&s),
                SearchEvent::Failed(b) => work.add(b.engine.stats()),
                SearchEvent::Complete(mut b) => {
                    work.add(b.engine.stats());
                    answers.push(b.engine.observe().unwrap());
                }
                SearchEvent::Exhausted => {
                    exhausted = true;
                    break;
                }
                _ => (),
            }
        }
        assert!(exhausted);
        let answer_count = answers.len();
        oracle::same_raw(answers, expected);
        samples.push(format!("{{\"applications\":{},\"candidates\":{},\"carrier_steps\":{},\"carrier_checks\":{},\"service_steps\":{},\"forks\":{},\"failed\":{},\"answers\":{}}}",work.applications,work.candidates,work.steps,work.checks,e.stats().service_steps,e.stats().forks,e.stats().failed_branches,answer_count));
    }
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"metrics\":true,\"samples\":[{}]}}",
        samples.join(",")
    );
}
#[cfg(all(test, feature = "carrier-contraction"))]
mod tests {
    use super::*;
    #[test]
    fn cancellation_at_32_ticks_reaches_live_update_inspection() {
        for family in [
            "pass",
            "unary",
            "nested",
            "open",
            "late",
            "malformed",
            "choice",
            "fail",
        ] {
            let schema = Schema::new(family, true, 64);
            let p = prepare("contracted", schema);
            let mut e = p
                .start(schema.query(64, true), Policy::Global, Access::Indexed)
                .unwrap();
            for _ in 0..32 {
                e.step();
            }
            assert!(!e.status().exhausted);
            assert!(
                e.stats().carrier_checks > 0 && e.stats().carrier_steps == 0,
                "{family}"
            );
        }
    }
}
