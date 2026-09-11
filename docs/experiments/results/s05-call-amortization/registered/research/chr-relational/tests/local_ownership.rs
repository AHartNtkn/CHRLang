//! Isolated requested-heap ownership gate; never run in a threaded test harness.
#[allow(dead_code)]
#[path = "support/local_ports.rs"]
mod local;
// The shared meter also contains optional fork-diagnostic checks used elsewhere.
#[allow(dead_code, unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use chr_syntax::{Goal, Query, Rule, Var, and, atom, c, eq, t, v};
use std::sync::Arc;
const METRICS: bool = cfg!(feature = "local-work");
fn main() {
    meter::self_check().unwrap();
    println!(
        "{{\"metrics\":{METRICS},\"run_bytes\":{},\"inert_diagnostic_field_bytes\":{}}}",
        std::mem::size_of::<local::Run<METRICS>>(),
        std::mem::size_of::<local::DependencyWork>()
            + 2 * std::mem::size_of::<usize>()
            + std::mem::size_of::<Vec<usize>>()
    );
    for fail in [false, true] {
        let rule = Rule::simplify(
            "chain",
            [c("take", [t("f", [v(0)]), v(1)]), c("token", [])],
            if fail {
                and([eq(v(1), t("g", [v(0)])), Goal::Fail])
            } else {
                and([
                    eq(v(1), t("g", [v(0)])),
                    c("take", [v(0), v(2)]).into(),
                    c("token", []).into(),
                    c("link", [v(1), v(2), v(2)]).into(),
                ])
            },
        );
        let queries = [(4, atom("a")), (12, atom("b")), (8, v(90))].map(|(depth, leaf)| Query {
            constraints: vec![
                c("take", [(0..depth).fold(leaf, |x, _| t("f", [x])), v(100)]),
                c("token", []),
            ],
            outputs: vec![("out".into(), Var(100)), ("leaf".into(), Var(90))],
        });
        let expected = queries
            .each_ref()
            .map(|q| scalar::run(std::slice::from_ref(&rule), q, 200_000));
        for mode in [
            local::DependencyMode::Endpoint,
            local::DependencyMode::Filtered,
            local::DependencyMode::Indexed,
        ] {
            // All recording storage exists before ownership measurements.
            let mut records = [[None; 6]; 12];
            let lifetime = meter::begin();
            let plan = local::Plan::compile(&rule).unwrap();
            let preparation = meter::end(lifetime);
            for (qi, q) in queries.iter().enumerate() {
                for (ci, cancel) in ["setup", "equation", "body", "complete"]
                    .into_iter()
                    .enumerate()
                {
                    let row = &mut records[qi * 4 + ci];
                    let query_baseline = meter::begin();
                    let mut run = plan.start_with_metrics::<METRICS>(q, mode);
                    row[0] = Some(meter::end(query_baseline));
                    let execution = meter::begin();
                    match cancel {
                        "setup" => (),
                        "equation" => {
                            let mut pending = false;
                            for _ in 0..200_000 {
                                assert!(!run.advance(1));
                                if run.pending_equations() > 0 {
                                    pending = true;
                                    break;
                                }
                            }
                            assert!(pending);
                        }
                        "body" => {
                            let mut pending = false;
                            for _ in 0..200_000 {
                                assert!(!run.advance(1));
                                if run.body_pending() {
                                    pending = true;
                                    break;
                                }
                            }
                            assert!(pending);
                        }
                        "complete" => run.settle(),
                        _ => unreachable!(),
                    }
                    row[1] = Some(meter::end(execution));
                    let observing = meter::begin();
                    let answer = if cancel == "complete" {
                        run.observe()
                    } else {
                        None
                    };
                    row[2] = Some(meter::end(observing));
                    let disposing = meter::begin();
                    drop(run);
                    row[3] = Some(meter::end(disposing));
                    assert_eq!(Arc::strong_count(&plan), 1);
                    // Exact answer validation is outside every phase interval.
                    // It occurs after query disposal, exercising independent output ownership.
                    if cancel == "complete" {
                        scalar::same_raw(answer.iter().cloned().collect(), expected[qi].clone());
                    }
                    let releasing = meter::begin();
                    drop(answer);
                    row[4] = Some(meter::end(releasing));
                    let restored = meter::end(query_baseline);
                    assert_eq!(restored.live_start, restored.live_end);
                    // This last interval is a restoration check, not a cost total:
                    // it includes independent validation and nested meter resets.
                    row[5] = Some(restored);
                }
            }
            let disposing = meter::begin();
            drop(plan);
            let plan_disposal = meter::end(disposing);
            let restored = meter::end(lifetime);
            assert_eq!(restored.live_start, restored.live_end);
            println!(
                "{{\"metrics\":{METRICS},\"fail\":{fail},\"mode\":\"{mode:?}\",\"preparation\":{},\"plan_disposal\":{},\"live_restored\":true}}",
                preparation.json(),
                plan_disposal.json()
            );
            for (index, row) in records.into_iter().enumerate() {
                print!(
                    "{{\"query\":{},\"cancel\":{},\"phases\":[",
                    index / 4,
                    index % 4
                );
                for (i, phase) in row.into_iter().enumerate() {
                    if i > 0 {
                        print!(",");
                    }
                    print!("{}", phase.unwrap().json());
                }
                println!("]}}");
            }
        }
    }
}
