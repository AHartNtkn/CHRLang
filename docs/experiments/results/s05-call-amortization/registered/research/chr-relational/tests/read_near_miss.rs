#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[cfg(feature = "deduction-work")]
use chr_relational::contextual::Store;
use chr_relational::contextual_execute::{Prepared, Step};
#[cfg(feature = "deduction-work")]
use chr_syntax::atom;
#[cfg(feature = "deduction-work")]
fn settle(s: &mut Store) {
    for _ in 0..200_000 {
        if !s.step() {
            return;
        }
    }
    panic!("service cutoff");
}
#[cfg(feature = "deduction-work")]
#[test]
fn same_inputs_examine_each_prior_incompatible_context() {
    for n in [1, 8, 32] {
        let mut base = Store::default().with_validated_deductions();
        let x = base.unknown();
        let y = base.unknown();
        let fx = base.constructor("f", &[x]);
        let fy = base.constructor("f", &[y]);
        let values = (0..n)
            .map(|i| base.constructor(&format!("a{i:02}"), &[]))
            .collect::<Vec<_>>();
        for (i, a) in values.iter().enumerate() {
            let mut branch = base.clone();
            branch.equate(x, *a);
            branch.equate(y, *a);
            settle(&mut branch);
            let before = branch.relevant_validation_work();
            branch.equate(fx, fy);
            settle(&mut branch);
            let after = branch.relevant_validation_work();
            assert_eq!(
                after.0 - before.0,
                i,
                "must inspect all incompatible prior target records"
            );
            assert_eq!(
                branch.export(&[x, y]),
                Some(vec![atom(&format!("a{i:02}")); 2])
            );
            println!(
                "NEAR_OPERATION,{n},{i},{},{}",
                after.0 - before.0,
                after.1 - before.1
            );
        }
        let mut again = base.clone();
        again.equate(x, values[n - 1]);
        again.equate(y, values[n - 1]);
        settle(&mut again);
        let before = again.relevant_deduction_hits();
        again.equate(fx, fy);
        settle(&mut again);
        assert!(again.relevant_deduction_hits() > before);
        assert_eq!(
            again.export(&[x, y]),
            Some(vec![atom(&format!("a{:02}", n - 1)); 2])
        );
    }
}
#[path = "../examples/support/read_near_source.rs"]
mod near_source;
use near_source::source;

#[test]
fn near_misses_preserve_complete_source_answers() {
    for n in [1, 8, 32] {
        for depth in [1, 8] {
            for kind in ["unique", "repeated", "mixed"] {
                for token in [false, true] {
                    for reverse in [false, true] {
                        let (rules, q, expected) = source(n, depth, kind, token, reverse);
                        oracle::same_raw(oracle::run(&rules, &q, 200_000), expected.clone());
                        let p = Prepared::new(&rules).unwrap();
                        let mut held = vec![];
                        for (mode, mut engine) in [
                            ("plain", p.start(&q)),
                            ("exact", p.start_shared_deductions(&q)),
                            ("relevant", p.start_relevant_deductions(&q)),
                            (
                                "persistent-relevant",
                                p.start_persistent_relevant_deductions(&q),
                            ),
                            ("validated", p.start_validated_deductions(&q)),
                            (
                                "persistent-validated",
                                p.start_persistent_validated_deductions(&q),
                            ),
                        ] {
                            let mut actual = vec![];
                            let mut complete = false;
                            for _ in 0..200_000 {
                                match engine.advance() {
                                    Step::Answer(a) => actual.push(a),
                                    Step::Progress => (),
                                    Step::Exhausted => {
                                        complete = true;
                                        break;
                                    }
                                }
                            }
                            #[cfg(feature = "deduction-work")]
                            let work = {
                                let w = engine.relevant_validation_work();
                                (w.0, w.1, engine.relevant_deduction_hits())
                            };
                            #[cfg(feature = "deduction-work")]
                            {
                                if mode.ends_with("validated") {
                                    if kind == "unique" {
                                        assert_eq!(work.0, (depth + 1) * n * (n - 1) / 2);
                                    }
                                    if kind == "repeated" {
                                        assert_eq!(work.0, (depth + 1) * (n - 1));
                                        assert_eq!(work.2, work.0);
                                    }
                                }
                            }
                            assert!(complete, "source cutoff {mode}");
                            drop(engine);
                            held.push(actual);
                            #[cfg(feature = "deduction-work")]
                            println!(
                                "NEAR_SOURCE,{n},{depth},{kind},{token},{reverse},{mode},{},{},{}",
                                work.0, work.1, work.2
                            );
                        }
                        drop(p);
                        for actual in held {
                            oracle::same_raw(actual, expected.clone());
                        }
                        let p = chr_compiled::PreparedRuleset::new(rules, None).unwrap();
                        let mut held = vec![];
                        for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                            let mut e = p
                                .start_search(q.clone(), chr_compiled::Policy::Global, access)
                                .unwrap();
                            let mut actual = vec![];
                            let mut done = false;
                            for _ in 0..200_000 {
                                match e.tick() {
                                    chr_compiled::SearchEvent::Complete(mut b) => {
                                        actual.push(b.engine.observe().unwrap())
                                    }
                                    chr_compiled::SearchEvent::Exhausted => {
                                        done = true;
                                        break;
                                    }
                                    _ => (),
                                }
                            }
                            assert!(done);
                            drop(e);
                            held.push(actual);
                        }
                        drop(p);
                        for actual in held {
                            oracle::same_raw(actual, expected.clone());
                        }
                    }
                }
            }
        }
    }
}
