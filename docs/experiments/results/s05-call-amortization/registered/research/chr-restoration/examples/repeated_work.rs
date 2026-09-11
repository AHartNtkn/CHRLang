//! Separate ordinary-allocator work attribution for repeated reunion.
#[path = "support/repeated_source.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_restoration::{Mode, Prepared, Step, reunion::PreparedPhase};
fn main() {
    assert!(!cfg!(feature = "alloc-meter"));
    for family in ["plain", "history", "late"] {
        for rounds in [0, 1, 3] {
            for depth in [0, 12] {
                let (rules, n) = fixture::source(family, depth);
                let q = fixture::query(family, rounds, depth, 0);
                let expected = oracle::run(&rules, &q, 200_000);
                let mut e = Prepared::new(&rules)
                    .unwrap()
                    .start(&q, Mode::Copy)
                    .unwrap();
                let mut answers = vec![];
                let mut calls = 0;
                loop {
                    assert!(calls < 200_000);
                    match e.advance() {
                        Step::Exhausted => break,
                        Step::Answer(a) => answers.push(a),
                        _ => (),
                    };
                    calls += 1;
                }
                oracle::same_raw(answers, expected.clone());
                println!(
                    "{{\"mode\":\"copy\",\"family\":\"{family}\",\"rounds\":{rounds},\"depth\":{depth},\"steps\":{calls}}}"
                );
                let p = PreparedPhase::new(&rules, n).unwrap();
                let mut e = p.start(&q).unwrap();
                let mut answers = vec![];
                let mut calls = 0;
                loop {
                    calls += 1;
                    assert!(calls < 200_000);
                    match e.advance().unwrap() {
                        Step::Exhausted => break,
                        Step::Answer(a) => answers.push(a),
                        _ => (),
                    };
                }
                oracle::same_raw(answers, expected.clone());
                println!(
                    "{{\"mode\":\"reunion\",\"family\":\"{family}\",\"rounds\":{rounds},\"depth\":{depth},\"private\":{},\"coupled\":{}}}",
                    e.local_steps, e.reunion_steps
                );
                let mut e = p.start_repeated(&q).unwrap();
                let mut answers = vec![];
                let mut calls = 0;
                loop {
                    calls += 1;
                    assert!(calls < 200_000);
                    match e.advance().unwrap() {
                        Step::Exhausted => break,
                        Step::Answer(a) => answers.push(a),
                        _ => (),
                    };
                }
                oracle::same_raw(answers, expected);
                println!(
                    "{{\"mode\":\"repeated\",\"family\":\"{family}\",\"rounds\":{rounds},\"depth\":{depth},\"private\":{},\"coupled\":{},\"checks\":{},\"epochs\":{},\"bindings\":{},\"history\":{}}}",
                    e.work.private_steps,
                    e.work.coupled_steps,
                    e.work.boundary_checks,
                    e.epochs,
                    e.work.inherited_bindings,
                    e.work.inherited_history
                );
            }
        }
    }
}
