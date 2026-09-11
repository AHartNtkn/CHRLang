#![cfg(all(feature = "observation-backpressure", feature = "equality-probe"))]
use chr_direct_conditional::engine::{Event, PreparedRuleset};
use chr_syntax::{Answer, Query, Rule, Var, c, eq, or, t, v};
#[test]
fn bounded_lead_preserves_continuing_alias_answers_and_progress() {
    let rules = vec![Rule::simplify(
        "emit-or-recur",
        [c("stream", [v(0)])],
        or(
            eq(v(0), t("pair", [v(1), v(1)])),
            c("stream", [v(0)]).into(),
        ),
    )];
    let p = PreparedRuleset::new(rules).unwrap();
    let mut e = p
        .start(Query {
            constraints: vec![c("stream", [v(10)])],
            outputs: vec![("answer".into(), Var(10))],
        })
        .unwrap();
    let wanted = Answer {
        outputs: vec![("answer".into(), t("pair", [v(99), v(99)]))],
        residual: vec![],
    };
    let mut count = 0;
    for _ in 0..2_000_000 {
        let event = e.tick();
        assert!(
            e.observation_backlog() <= 2,
            "producer lead exceeds two observations"
        );
        match event {
            Event::Answer(a) => {
                assert!(chr_observe::equivalent(
                    &a,
                    &wanted,
                    &mut Default::default()
                ));
                count += 1;
                if count == 64 {
                    return;
                }
            }
            Event::Progress => (),
            Event::Exhausted => panic!("continuing source exhausted"),
        }
    }
    panic!("finite prefix failed to progress");
}
