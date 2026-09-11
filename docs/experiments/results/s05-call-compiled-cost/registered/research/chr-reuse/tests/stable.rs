use chr_reuse::stable::{Policy, Session};

#[test]
fn hits_replay_bindings_and_unrelated_changes_distinguish_validity_policies() {
    for policy in [Policy::Exact, Policy::Dependencies] {
        let mut s = Session::new(8);
        let x = s.variable();
        let y = s.variable();
        let a = s.make("a", &[]).unwrap();
        let b = s.make("b", &[]).unwrap();
        let fx = s.make("f", std::slice::from_ref(&x)).unwrap();
        let fa = s.make("f", std::slice::from_ref(&a)).unwrap();
        let initial = s.context();
        let mut first = initial.clone();
        let r = s.unify(&fx, &fa, &mut first, policy).unwrap();
        assert!(r.success && !r.hit && !r.changed.is_empty());
        let mut sibling = initial.clone();
        let r = s.unify(&fx, &fa, &mut sibling, policy).unwrap();
        assert!(r.success && r.hit && !r.changed.is_empty());
        assert_eq!(s.export(&x, &sibling).unwrap(), chr_syntax::atom("a"));
        let mut unrelated = initial.clone();
        s.unify(&y, &b, &mut unrelated, Policy::Direct).unwrap();
        let r = s.unify(&fx, &fa, &mut unrelated, policy).unwrap();
        assert!(r.success);
        assert_eq!(r.hit, matches!(policy, Policy::Dependencies));
        assert_eq!(s.export(&y, &unrelated).unwrap(), chr_syntax::atom("b"));
    }
}

#[test]
fn changed_bindings_occurs_failure_and_foreign_handles_are_not_reused_unsafely() {
    let mut s = Session::new(4);
    let x = s.variable();
    let y = s.variable();
    let a = s.make("a", &[]).unwrap();
    let b = s.make("b", &[]).unwrap();
    let left = s.make("pair", &[x.clone(), x.clone()]).unwrap();
    let right = s.make("pair", &[a.clone(), b.clone()]).unwrap();
    let mut c = s.context();
    let r = s
        .unify(&left, &right, &mut c, Policy::Dependencies)
        .unwrap();
    assert!(!r.success && !r.hit && r.changed.is_empty());
    assert_eq!(s.export(&x, &c).unwrap(), chr_syntax::v(0));
    assert!(
        s.unify(&left, &right, &mut c, Policy::Dependencies)
            .unwrap()
            .hit
    );
    let fy = s.make("f", std::slice::from_ref(&y)).unwrap();
    let empty = c.clone();
    assert!(
        s.unify(&x, &fy, &mut c, Policy::Dependencies)
            .unwrap()
            .success
    );
    let mut cycle = empty;
    s.unify(&x, &y, &mut cycle, Policy::Direct).unwrap();
    let r = s.unify(&x, &fy, &mut cycle, Policy::Dependencies).unwrap();
    assert!(!r.success && !r.hit);
    let mut other = Session::new(4);
    let foreign = other.make("different", &[]).unwrap();
    assert!(s.make("f", std::slice::from_ref(&foreign)).is_err());
    assert!(s.unify(&a, &foreign, &mut c, Policy::Exact).is_err());
    assert!(
        s.unify(&a, &a, &mut other.context(), Policy::Exact)
            .is_err()
    );
}

#[test]
fn bounded_eviction_recomputes_without_changing_results() {
    let mut s = Session::new(1);
    let a = s.make("a", &[]).unwrap();
    let b = s.make("b", &[]).unwrap();
    let c = s.make("c", &[]).unwrap();
    let mut ctx = s.context();
    assert!(!s.unify(&a, &b, &mut ctx, Policy::Exact).unwrap().success);
    assert!(!s.unify(&a, &c, &mut ctx, Policy::Exact).unwrap().success);
    assert_eq!(s.entries(), 1);
    let r = s.unify(&a, &b, &mut ctx, Policy::Exact).unwrap();
    assert!(!r.hit && !r.success);
    s.clear();
    assert_eq!(s.entries(), 0);
}

fn import(
    s: &mut Session,
    vars: &[chr_reuse::stable::Handle],
    t: &chr_syntax::Term,
) -> chr_reuse::stable::Handle {
    match t {
        chr_syntax::Term::Var(v) => vars[v.0 as usize].clone(),
        chr_syntax::Term::App(n, args) => {
            let args: Vec<_> = args.iter().map(|t| import(s, vars, t)).collect();
            s.make(n, &args).unwrap()
        }
    }
}
fn reference(equations: &[(chr_syntax::Term, chr_syntax::Term)]) -> Vec<chr_syntax::Answer> {
    use chr_syntax::{Query, Rule, Var, and, c, eq, v};
    let rules = vec![Rule::simplify(
        "equations",
        [c("seed", [v(0), v(1), v(2)])],
        and(equations
            .iter()
            .map(|(a, b)| eq(a.clone(), b.clone()))
            .collect::<Vec<_>>()),
    )];
    let q = Query {
        constraints: vec![c("seed", [v(0), v(1), v(2)])],
        outputs: (0..3).map(|i| (format!("v{i}"), Var(i))).collect(),
    };
    let mut r = chr_reference::Search::new(rules, q).unwrap();
    let batch = r.advance(1000);
    assert!(batch.exhausted);
    batch.answers
}
#[test]
fn ordered_equations_and_changed_contexts_match_independent_reference() {
    use chr_syntax::{Answer, atom, t, v};
    let terms = vec![
        v(0),
        v(1),
        v(2),
        atom("a"),
        atom("b"),
        t("f", [v(0)]),
        t("f", [atom("a")]),
        t("pair", [v(0), v(1)]),
        t("pair", [v(0), v(0)]),
    ];
    let mut comparisons = 0;
    let mut hits = 0;
    for policy in [Policy::Direct, Policy::Exact, Policy::Dependencies] {
        for prefix in [
            None,
            Some((v(0), v(1))),
            Some((v(0), atom("a"))),
            Some((v(1), t("f", [v(2)]))),
        ] {
            for left in &terms {
                for right in &terms {
                    for alteration in [
                        None,
                        Some((v(2), atom("b"))),
                        Some((v(0), v(1))),
                        Some((v(0), t("f", [v(1)]))),
                    ] {
                        let mut s = Session::new(8);
                        let variables: Vec<_> = (0..3).map(|_| s.variable()).collect();
                        let mut equations = vec![];
                        let mut initial = s.context();
                        if let Some((a, b)) = &prefix {
                            let x = import(&mut s, &variables, a);
                            let y = import(&mut s, &variables, b);
                            assert!(
                                s.unify(&x, &y, &mut initial, Policy::Direct)
                                    .unwrap()
                                    .success
                            );
                            equations.push((a.clone(), b.clone()));
                        }
                        let a = import(&mut s, &variables, left);
                        let b = import(&mut s, &variables, right);
                        let mut prime = initial.clone();
                        s.unify(&a, &b, &mut prime, policy).unwrap();
                        let mut context = initial;
                        let mut success = true;
                        if let Some((x, y)) = &alteration {
                            let xh = import(&mut s, &variables, x);
                            let yh = import(&mut s, &variables, y);
                            success = s
                                .unify(&xh, &yh, &mut context, Policy::Direct)
                                .unwrap()
                                .success;
                            equations.push((x.clone(), y.clone()));
                        }
                        if success {
                            let r = s.unify(&a, &b, &mut context, policy).unwrap();
                            success = r.success;
                            hits += usize::from(r.hit);
                        }
                        equations.push((left.clone(), right.clone()));
                        let expected = reference(&equations);
                        assert_eq!(success, !expected.is_empty());
                        if success {
                            let actual = Answer {
                                outputs: variables
                                    .iter()
                                    .enumerate()
                                    .map(|(i, v)| (format!("v{i}"), s.export(v, &context).unwrap()))
                                    .collect(),
                                residual: vec![],
                            };
                            assert!(
                                chr_observe::equivalent(
                                    &actual,
                                    &expected[0],
                                    &mut Default::default()
                                ),
                                "{equations:?}"
                            );
                        }
                        comparisons += 1;
                    }
                }
            }
        }
    }
    assert_eq!(comparisons, 3888);
    assert!(hits > 500);
}
