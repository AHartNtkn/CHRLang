#![cfg(feature = "lifecycle")]
use harness::cases;
#[allow(dead_code)]
#[path = "../examples/support/region_lifecycle.rs"]
mod harness;
struct Unmetered;
impl harness::Metering for Unmetered {
    const ENABLED: bool = false;
    fn start() -> harness::Reading {
        harness::Reading::default()
    }
    fn read() -> harness::Reading {
        harness::Reading::default()
    }
}
#[test]
fn complete_lifecycles_validate_unchanged_fixtures_in_every_supported_mode() {
    for id in cases::ids() {
        let case = cases::case(&id);
        for mode in [
            harness::Mode::Inline,
            harness::Mode::Threads1,
            harness::Mode::Threads2,
            harness::Mode::Specialized,
        ] {
            if mode == harness::Mode::Specialized
                && matches!(id.as_str(), "stream-prefix" | "refute-loop")
            {
                continue;
            }
            let (q, k) = if mode == harness::Mode::Specialized {
                (0, 0)
            } else {
                (8, 4)
            };
            let row = harness::session::<Unmetered>(&case, mode, q, k).unwrap();
            let json = row.json::<Unmetered>(&id);
            assert!(json.contains(if id == "stream-prefix" {
                "\"status\":\"prefix\""
            } else {
                "\"status\":\"complete\""
            }));
        }
    }
}
#[test]
fn cutoff_is_explicit_and_unsupported_specialized_prefix_is_rejected() {
    let mut case = cases::case("two-work");
    case.budget = 0;
    for mode in [
        harness::Mode::Inline,
        harness::Mode::Threads1,
        harness::Mode::Threads2,
        harness::Mode::Specialized,
    ] {
        let (q, k) = if mode == harness::Mode::Specialized {
            (0, 0)
        } else {
            (64, 4)
        };
        let row = harness::session::<Unmetered>(&case, mode, q, k).unwrap();
        let json = row.json::<Unmetered>(&case.id);
        if mode == harness::Mode::Specialized {
            assert!(json.contains("\"status\":\"complete\""));
            assert!(json.contains("\"budget\":20000000"));
            continue;
        }
        assert!(json.contains("\"status\":\"cutoff\""));
        assert!(json.contains("\"answers\":0"));
        assert!(json.contains("\"raw\":null"));
    }
    for id in ["stream-prefix", "refute-loop"] {
        assert!(
            harness::session::<Unmetered>(&cases::case(id), harness::Mode::Specialized, 0, 0)
                .is_err()
        );
    }
}
