//! Isolated attribution of excluded oracle and answer-validation work.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use std::time::Instant;
fn main() {
    let rules = chr_compiled::artifact_runtime::rules("chain").unwrap();
    let prepared = chr_compiled::PreparedRuleset::new_with_update_plan(rules.clone(), None).unwrap();
    let (mut expected_ns, mut validation_ns, mut runtime_ns) = (0, 0, 0);
    for round in 0..1024 {
        let input = chr_compiled::fixtures::flat_chain_case(32 + round % 2, false).query;
        let start = Instant::now();
        let expected = oracle::run(&rules, &input, 2_000_000);
        expected_ns += start.elapsed().as_nanos();
        let start = Instant::now();
        let mut engine = prepared.start(input, chr_compiled::Policy::Global, chr_compiled::Access::Indexed).unwrap();
        assert!(engine.advance(2_000_000).exhausted);
        let answer = engine.observe().unwrap();
        runtime_ns += start.elapsed().as_nanos();
        let start = Instant::now();
        assert_eq!(expected.len(), 1);
        assert!(chr_observe::equivalent(&answer, &expected[0], &mut Default::default()));
        validation_ns += start.elapsed().as_nanos();
    }
    println!("{{\"queries\":1024,\"oracle_ns\":{expected_ns},\"validation_ns\":{validation_ns},\"setup_execution_observation_ns\":{runtime_ns}}}");
}
