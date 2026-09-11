#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use chr_relational::contextual_execute::{Prepared, Step};
use chr_syntax::{Answer, Query, Rule, Var, and, c, eq, t, v};
fn collect(mut e: chr_relational::contextual_execute::Engine, expected: &[Answer]) -> u64 {
    let mut out = vec![];
    let mut exhausted = false;
    for _ in 0..500_000 {
        match e.advance() {
            Step::Answer(a) => out.push(a),
            Step::Exhausted => {
                exhausted = true;
                break;
            }
            Step::Progress => (),
        }
    }
    assert!(exhausted);
    scalar::same_raw(out, expected.to_vec());
    e.discovery_stats().offered
}
#[test]
fn source_elimination_and_dynamic_detection_have_different_scope() {
    for kind in ["self", "constructor", "dynamic"] {
        let dynamic = kind == "dynamic";
        let mut rules = vec![];
        if dynamic {
            rules.push(Rule::simplify(
                "alias",
                [c("alias", [v(0), v(1)])],
                eq(v(0), v(1)),
            ));
        }
        rules.push(Rule::propagate(
            "visit",
            [c("item", [v(0), v(1)])],
            and([
                c("mark", [v(0)]).into(),
                if kind == "constructor" {
                    eq(t("box", [v(0)]), t("box", [v(0)]))
                } else {
                    eq(v(0), if dynamic { v(1) } else { v(0) })
                },
            ]),
        ));
        let mut constraints = vec![];
        if dynamic {
            constraints.push(c("alias", [v(100), v(101)]));
        }
        constraints.extend(vec![
            c(
                "item",
                [v(100), if dynamic { v(101) } else { v(100) }]
            );
            16
        ]);
        let q = Query {
            constraints,
            outputs: vec![("x".into(), Var(100))],
        };
        let mut residual = vec![c("item", [v(999), v(999)]); 16];
        residual.extend(vec![c("mark", [v(999)]); 16]);
        let expected = vec![Answer {
            outputs: vec![("x".into(), v(999))],
            residual,
        }];
        let transformed = chr_compiled::reflexive::eliminate(&rules);
        assert_eq!(transformed == rules, dynamic);
        for (eliminated, r) in [(false, &rules), (true, &transformed)] {
            scalar::same_raw(scalar::run(r, &q, 500_000), expected.clone());
            let pscan = chr_compiled::PreparedRuleset::new(r.to_vec(), None).unwrap();
            let mut scan = pscan
                .start_search(
                    q.clone(),
                    chr_compiled::Policy::Global,
                    chr_compiled::Access::Scan,
                )
                .unwrap();
            let mut actual = vec![];
            let mut exhausted = false;
            for _ in 0..500_000 {
                match scan.tick() {
                    chr_compiled::SearchEvent::Complete(mut b) => {
                        actual.push(b.engine.observe().unwrap())
                    }
                    chr_compiled::SearchEvent::Exhausted => {
                        exhausted = true;
                        break;
                    }
                    _ => (),
                }
            }
            assert!(exhausted);
            scalar::same_raw(actual, expected.clone());
            let p = Prepared::new(r).unwrap();
            let counts = [
                collect(p.start(&q), &expected),
                collect(p.start_demand(&q), &expected),
                collect(p.start_resumable(&q), &expected),
            ];
            #[cfg(feature = "local-work")]
            {
                let cached = if (cfg!(feature = "precise-invalidation") && kind != "constructor")
                    || (!dynamic && eliminated)
                {
                    16
                } else {
                    152
                };
                let setup = u64::from(dynamic);
                // Constructor self-equality queues a child step. The final
                // unsuccessful demand search repeats while that step is pending.
                let extra = if kind == "constructor" && !eliminated {
                    16
                } else {
                    0
                };
                let cached = cached
                    + if cfg!(feature = "precise-invalidation") {
                        0
                    } else {
                        extra
                    };
                assert_eq!(
                    counts,
                    [cached + setup, 152 + extra + setup, cached + setup]
                );
            }
            println!("kind={kind} eliminated={eliminated} offered={counts:?}");
        }
    }
}
#[cfg(feature = "precise-invalidation")]
#[test]
fn queue_progress_is_distinct_from_matching_information_change() {
    use chr_relational::contextual::Store;
    let mut s = Store::default();
    let x = s.unknown();
    let y = s.unknown();
    s.equate(x, x);
    s.equate(x, y);
    assert!(s.step());
    assert!(!s.last_step_changed());
    assert_eq!(s.pending(), 1);
    assert!(s.step());
    assert!(s.last_step_changed());
    s.equate(x, y);
    assert!(s.step());
    assert!(!s.last_step_changed());
    let a = s.constructor("a", &[]);
    s.equate(x, a);
    assert!(s.step());
    assert!(s.last_step_changed());
    assert!(!s.step());
    assert!(!s.last_step_changed());
    let b = s.constructor("b", &[]);
    s.equate(x, b);
    assert!(s.step());
    assert!(s.last_step_changed());
    assert!(s.failed());
}

#[cfg(feature = "precise-invalidation")]
#[test]
fn decomposition_keeps_intermediate_information_changes_visible() {
    use chr_relational::contextual::Store;
    let mut s = Store::default();
    let x = s.unknown();
    let y = s.unknown();
    let fx = s.constructor("f", &[x]);
    let fy = s.constructor("f", &[y]);
    s.equate(fx, fy);
    assert!(s.step());
    assert!(s.last_step_changed());
    assert!(s.pending() > 0);
    let heads = [c("p", [t("f", [v(0)])])];
    s.post("p", &[fx]);
    assert_eq!(s.matches(&heads, &[]).len(), 2);
    assert_ne!(s.root(x), s.root(y));
    assert!(s.step());
    assert!(s.last_step_changed());
    assert_eq!(s.root(x), s.root(y));
    // Environment values preserve both stored descriptor identities; they now
    // denote aliases. Invalidation must not discard either representation.
    assert_eq!(s.matches(&heads, &[]).len(), 2);
    s.equate(fx, fy);
    assert!(s.step());
    assert!(!s.last_step_changed());
}

#[test]
fn elimination_preserves_choices_failure_and_fresh_aliases() {
    use chr_syntax::{atom, or};
    let rules = vec![Rule::simplify(
        "start",
        [c("start", [v(0)])],
        and([
            eq(v(10), v(10)),
            or(
                and([
                    eq(t("box", [v(11)]), t("box", [v(11)])),
                    eq(v(0), atom("a")),
                    c("fresh", [v(10), v(11), v(10)]).into(),
                ]),
                and([eq(atom("a"), atom("b")), c("bad", []).into()]),
            ),
        ]),
    )];
    let q = Query {
        constraints: vec![c("start", [v(100)])],
        outputs: vec![("x".into(), Var(100))],
    };
    let expected = vec![Answer {
        outputs: vec![("x".into(), atom("a"))],
        residual: vec![c("fresh", [v(900), v(901), v(900)])],
    }];
    for rules in [rules.clone(), chr_compiled::reflexive::eliminate(&rules)] {
        scalar::same_raw(scalar::run(&rules, &q, 500_000), expected.clone());
        let p = Prepared::new(&rules).unwrap();
        for e in [p.start(&q), p.start_demand(&q), p.start_resumable(&q)] {
            collect(e, &expected);
        }
    }
}

#[test]
fn elimination_preserves_finite_service_beside_a_reflexive_loop() {
    use chr_syntax::{atom, or};
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(
                and([eq(v(0), atom("a")), c("done", []).into()]),
                c("spin", []).into(),
            ),
        ),
        Rule::simplify(
            "spin",
            [c("spin", [])],
            and([eq(v(10), v(10)), c("spin", []).into()]),
        ),
    ];
    let q = Query {
        constraints: vec![c("start", [v(100)])],
        outputs: vec![("x".into(), Var(100))],
    };
    let expected = vec![Answer {
        outputs: vec![("x".into(), atom("a"))],
        residual: vec![c("done", [])],
    }];
    for rules in [rules.clone(), chr_compiled::reflexive::eliminate(&rules)] {
        let p = Prepared::new(&rules).unwrap();
        for mut e in [p.start(&q), p.start_demand(&q), p.start_resumable(&q)] {
            let mut answer = None;
            for _ in 0..10000 {
                match e.advance() {
                    Step::Answer(a) => {
                        answer = Some(a);
                        break;
                    }
                    Step::Exhausted => panic!("ongoing source exhausted"),
                    Step::Progress => (),
                }
            }
            scalar::same_raw(
                vec![answer.expect("finite sibling starved")],
                expected.clone(),
            );
            for _ in 0..1024 {
                assert!(matches!(e.advance(), Step::Progress));
            }
        }
    }
}
