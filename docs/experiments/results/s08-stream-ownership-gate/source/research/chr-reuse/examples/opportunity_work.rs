//! Work diagnostics for the opportunity pilot; never used for timing.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[allow(dead_code)]
#[path = "support/opportunity_source.rs"]
mod source;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
use chr_reuse::continuations::{Mode, Prepared};

fn main() {
    if !chr_reuse::continuations::COLLECT_METRICS
        || !chr_compiled::COLLECT_METRICS
        || cfg!(feature = "alloc-meter")
    {
        panic!("work diagnostics require counters and reject allocation metering");
    }
    for family in ["exact", "rename", "history", "history-early", "distinct"] {
        for resource in [false, true] {
            for width in [1, 4, 16] {
                for payload in [0, 32] {
                    let schema = source::Schema::new(family, resource, width, payload);
                    let query = schema.query(32, false);
                    let expected = oracle::run(&schema.rules(), &query, 2_000_000);
                    oracle::same_raw(expected.clone(), schema.expected());
                    for mode in [
                        Mode::Direct,
                        Mode::ExactIds,
                        Mode::Alpha,
                        Mode::AlphaLive,
                        Mode::CompactExact,
                        Mode::CompactAlpha,
                        Mode::CompactLive,
                    ] {
                        let p = Prepared::new(schema.rules(), mode).unwrap();
                        let mut run = p.start(query.clone()).unwrap();
                        let batch = run.advance(2_000_000);
                        assert!(batch.exhausted);
                        oracle::same_raw(batch.answers, expected.clone());
                        let s = run.stats();
                        println!(
                            "{{\"family\":\"{family}\",\"resource\":{resource},\"width\":{width},\"payload\":{payload},\"mode\":\"{mode:?}\",\"logical\":{},\"executed\":{},\"hits\":{}}}",
                            s.logical_steps, s.executed, s.hits
                        );
                    }
                    for contracted in [false, true] {
                        let p = PreparedRuleset::new(schema.rules(), None)
                            .unwrap()
                            .specialize_inferred();
                        let p = if contracted {
                            #[cfg(feature = "carrier-contraction")]
                            {
                                p.contract_carriers_checked(&[("work".into(), 4)]).unwrap()
                            }
                            #[cfg(not(feature = "carrier-contraction"))]
                            {
                                panic!("requires carrier-contraction");
                            }
                        } else {
                            p
                        };
                        let mut run = p
                            .start_search(query.clone(), Policy::Global, Access::Scan)
                            .unwrap();
                        let (mut applications, mut steps, mut checks) = (0, 0, 0);
                        let mut add = |s: &chr_compiled::Stats| {
                            applications += s.applications;
                            #[cfg(feature = "carrier-contraction")]
                            {
                                steps += s.carrier_steps;
                                checks += s.carrier_checks;
                            }
                        };
                        let mut answers = vec![];
                        let mut done = false;
                        for _ in 0..2_000_000 {
                            match run.tick() {
                                SearchEvent::Split { work: Some(s), .. } => add(&s),
                                SearchEvent::Failed(b) => add(b.engine.stats()),
                                SearchEvent::Complete(mut b) => {
                                    add(b.engine.stats());
                                    answers.push(b.engine.observe().unwrap());
                                }
                                SearchEvent::Exhausted => {
                                    done = true;
                                    break;
                                }
                                _ => (),
                            }
                        }
                        assert!(done);
                        oracle::same_raw(answers, expected.clone());
                        println!(
                            "{{\"family\":\"{family}\",\"resource\":{resource},\"width\":{width},\"payload\":{payload},\"mode\":\"{}\",\"applications\":{applications},\"carrier_steps\":{steps},\"carrier_checks\":{checks}}}",
                            if contracted { "contracted" } else { "sealed" }
                        );
                    }
                }
            }
        }
    }
}
