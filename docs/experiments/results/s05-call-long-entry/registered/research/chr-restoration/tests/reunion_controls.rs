#[path = "../examples/support/reunion_source.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../examples/support/reunion_shortcut.rs"]
mod shortcut;
use chr_restoration::{Mode, Prepared, Step};
use chr_syntax::{Answer, Query, Rule, atom, c, t, v};
fn compiled(
    p: &chr_compiled::PreparedRuleset,
    q: &Query,
    a: chr_compiled::Access,
    specialized: bool,
) -> Vec<Answer> {
    let mut e = p
        .start_search(q.clone(), chr_compiled::Policy::Global, a)
        .unwrap();
    let mut out = vec![];
    let mut specialized_candidates = 0;
    for _ in 0..200_000 {
        match e.tick() {
            chr_compiled::SearchEvent::Split { work: Some(s), .. } => {
                specialized_candidates += s.specialized_candidates
            }
            chr_compiled::SearchEvent::Failed(b) => {
                specialized_candidates += b.engine.stats().specialized_candidates
            }
            chr_compiled::SearchEvent::Complete(mut b) => {
                specialized_candidates += b.engine.stats().specialized_candidates;
                out.push(b.engine.observe().unwrap());
            }
            chr_compiled::SearchEvent::Exhausted => {
                if chr_compiled::COLLECT_METRICS {
                    assert_eq!(
                        specialized_candidates > 0,
                        specialized,
                        "specialization did not execute as configured"
                    );
                }
                return out;
            }
            _ => (),
        }
    }
    panic!("compiled service bound");
}
fn copied(rules: &[Rule], q: &Query) -> (Vec<Answer>, usize) {
    let p = Prepared::new(rules).unwrap();
    let mut e = p.start(q, Mode::Copy).unwrap();
    let mut out = vec![];
    for steps in 0..200_000 {
        match e.advance() {
            Step::Answer(a) => out.push(a),
            Step::Exhausted => return (out, steps),
            _ => (),
        }
    }
    panic!("copy service bound");
}
#[test]
fn qualify_complete_controls_and_shortcut_against_independent_source_answers() {
    for f in ["plain", "equal", "late", "payload"] {
        for owners in [2, 4] {
            let (rules, local_rules) = fixture::source(f, owners);
            let p = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
            let specialized = p.specialize_inferred();
            let eligibility = p.region_eligibility();
            for name in ["job", "walk"] {
                assert!(
                    eligibility
                        .iter()
                        .find(|e| e.predicate.0 == name)
                        .unwrap()
                        .eligible
                );
            }
            assert!(
                !eligibility
                    .iter()
                    .find(|e| e.predicate.0 == "ready")
                    .unwrap()
                    .eligible
            );
            let reunion =
                chr_restoration::reunion::PreparedPhase::new(&rules, local_rules).unwrap();
            #[cfg(feature = "carrier-control")]
            {
                let eligibility = specialized.carrier_eligibility();
                let walk = eligibility
                    .iter()
                    .find(|e| e.predicate.0 == "walk")
                    .unwrap();
                assert!(!walk.eligible);
                println!("{f}/{owners} carrier walk: {:?}", walk.reason);
            }
            let (verified_rules, short) = shortcut::Checked::new(f, owners, &rules)
                .unwrap()
                .into_parts();
            for depth in [0, 12, 48] {
                for seed in 0..4 {
                    let q = fixture::query(f, owners, depth, seed);
                    let expected = oracle::run(&rules, &q, 200_000);
                    for (plan, is_specialized) in [(&p, false), (&specialized, true)] {
                        for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                            oracle::same_raw(
                                compiled(plan, &q, access, is_specialized),
                                expected.clone(),
                            );
                        }
                    }
                    let reunited = reunion.run(&q, 200_000).unwrap();
                    oracle::same_raw(reunited.answers, expected.clone());
                    let lowered = short.lower(&q).unwrap();
                    let shortened_reunion = reunion.run(&lowered, 200_000).unwrap();
                    oracle::same_raw(shortened_reunion.answers, expected.clone());
                    let (actual, after) = copied(&verified_rules, &lowered);
                    let (original, before) = copied(&rules, &q);
                    oracle::same_raw(actual, expected.clone());
                    oracle::same_raw(original, expected.clone());
                    for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                        oracle::same_raw(
                            compiled(&specialized, &lowered, access, true),
                            expected.clone(),
                        );
                    }
                    if depth == 48 && seed == 0 {
                        assert!(after < before);
                        println!("{f}/{owners}: source transitions {before} -> {after}");
                        #[cfg(feature = "replay-diagnostic")]
                        println!(
                            "{f}/{owners}: reunion source transitions {}",
                            reunited.local_steps + reunited.reunion_steps
                        );
                        #[cfg(feature = "replay-diagnostic")]
                        println!(
                            "{f}/{owners}: shortened reunion transitions {}",
                            shortened_reunion.local_steps + shortened_reunion.reunion_steps
                        );
                    }
                }
            }
        }
    }
}
#[test]
fn shortcut_rejects_queries_or_effects_outside_its_proof() {
    let (mut rules, _) = fixture::source("plain", 2);
    let (_, short) = shortcut::Checked::new("plain", 2, &rules)
        .unwrap()
        .into_parts();
    let q = fixture::query("plain", 2, 12, 0);
    let mut bad = q.clone();
    bad.constraints[0].args[1] = v(9000);
    assert!(short.lower(&bad).is_err());
    bad = q.clone();
    bad.constraints[0].args[1] = t("s", [v(9000)]);
    assert!(short.lower(&bad).is_err());
    bad = q.clone();
    bad.constraints[0].args[1] = atom("not-a-natural");
    assert!(short.lower(&bad).is_err());
    bad = q.clone();
    bad.constraints[1].args[2] = bad.constraints[0].args[2].clone();
    assert!(short.lower(&bad).is_err());
    bad = q.clone();
    bad.constraints.push(bad.constraints[0].clone());
    assert!(short.lower(&bad).is_err());
    bad = q.clone();
    bad.constraints
        .push(c("ready", [atom("owner0"), atom("a"), atom("a")]));
    assert!(short.lower(&bad).is_err());
    rules[1].body = c("effect", [atom("owner0")]).into();
    assert!(shortcut::Checked::new("plain", 2, &rules).is_err());
}

#[test]
fn passive_data_and_observations_are_preserved_but_late_watchers_are_checked() {
    let (rules, _) = fixture::source("payload", 2);
    let (verified_rules, short) = shortcut::Checked::new("payload", 2, &rules)
        .unwrap()
        .into_parts();
    let mut q = fixture::query("payload", 2, 4, 0);
    q.constraints
        .push(c("payload", [atom("owner0"), v(7000), v(7000)]));
    q.outputs.push(("alias".into(), chr_syntax::Var(7000)));
    q.outputs.push(("unbound".into(), chr_syntax::Var(8000)));
    let lower = short.lower(&q).unwrap();
    oracle::same_raw(
        copied(&verified_rules, &lower).0,
        oracle::run(&rules, &q, 200_000),
    );
    let (rules, _) = fixture::source("late", 2);
    let (_, short) = shortcut::Checked::new("late", 2, &rules)
        .unwrap()
        .into_parts();
    let mut q = fixture::query("late", 2, 4, 0);
    q.constraints
        .iter_mut()
        .find(|c| c.name == "wait")
        .unwrap()
        .args[1] = v(9000);
    assert!(short.lower(&q).is_err());
}
