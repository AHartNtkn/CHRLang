//! Independent complete-product check for the existing serial lowering control.
#[path = "../experiments/worker_cases.rs"]
mod cases;
use chr_compiled::{Access, Policy, PreparedRuleset, search::SearchEvent};
fn main() {
    assert!(!std::hint::black_box(chr_compiled::COLLECT_METRICS));
    let prepared = PreparedRuleset::new(cases::source(), None).unwrap();
    let eligibility = prepared.carrier_eligibility();
    assert_eq!(eligibility.len(), 4);
    assert!(eligibility.iter().all(|e| e.eligible));
    let lowered = prepared
        .specialize_inferred()
        .contract_carriers_inferred()
        .unwrap();
    let mut checked = 0;
    for count in [1, 2, 4] {
        let expected = cases::expected(count);
        for depth in [0, 16, 64, 256] {
            for skew in [false, true] {
                let mut search = lowered
                    .start_search(
                        cases::query(count, depth, skew),
                        Policy::Global,
                        Access::Scan,
                    )
                    .unwrap();
                let mut actual = Vec::new();
                let mut exhausted = false;
                for _ in 0..10_000_000 {
                    match search.tick() {
                        SearchEvent::Complete(mut branch) => {
                            actual.push(branch.engine.observe().unwrap())
                        }
                        SearchEvent::Exhausted => {
                            exhausted = true;
                            break;
                        }
                        SearchEvent::Failed(_) => panic!("unexpected failed branch"),
                        _ => {}
                    }
                }
                assert!(exhausted);
                actual.sort_unstable_by(|a, b| a.outputs.cmp(&b.outputs));
                assert_eq!(actual, expected);
                checked += 1;
            }
        }
    }
    println!("four eligible carriers; {checked} complete source-product checks pass");
}
