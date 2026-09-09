//! Full finite observations for the stronger serial control of the regional pilot.
//! Expectations come from source fixtures, independently of either executor.
#[allow(dead_code)]
#[path = "../../chr-factors/examples/support/region_cost_cases.rs"]
mod cases;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};

#[test]
fn specialized_serial_preserves_regional_finite_contracts() {
    for id in [
        "one-zero",
        "one-work",
        "two-work",
        "owner-product",
        "asym-first",
        "asym-last",
        "duplicate-eight",
        "mixed-add-infer",
    ] {
        let case = cases::case(id);
        let prepared = PreparedRuleset::new(case.rules, None)
            .unwrap()
            .specialize_inferred();
        let mut search = prepared
            .start_search(case.query, Policy::Global, Access::Indexed)
            .unwrap();
        let mut raw = 0;
        let mut unique = chr_observe::AnswerSet::default();
        let mut exhausted = false;
        for _ in 0..20_000_000 {
            match search.tick() {
                SearchEvent::Complete(mut branch) => {
                    raw += 1;
                    let answer = branch.engine.observe().unwrap();
                    assert!(
                        case.expected.iter().any(|expected| chr_observe::equivalent(
                            expected,
                            &answer,
                            &mut Default::default()
                        )),
                        "{id}: unexpected full answer"
                    );
                    unique.insert(answer);
                }
                SearchEvent::Exhausted => {
                    exhausted = true;
                    break;
                }
                SearchEvent::Failed(_) | SearchEvent::Progress | SearchEvent::Split { .. } => {}
            }
        }
        assert!(
            exhausted,
            "{id}: finite source did not exhaust within the gate bound"
        );
        assert_eq!(raw, case.raw_answers, "{id}: raw multiplicity");
        assert_eq!(
            unique.into_answers().len(),
            case.expected.len(),
            "{id}: unique coverage"
        );
    }
}
