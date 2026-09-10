use chr_structural::{
    normal_forms::Requirement as R,
    symbolic::{Answer, Fresh},
};
use chr_syntax::{Term, Var, atom, t, v};
use std::collections::{BTreeMap, BTreeSet};
fn value(t: &Term, b: &BTreeMap<Var, Term>) -> Term {
    match t {
        Term::Var(x) => b[x].clone(),
        Term::App(n, a) => Term::App(n.clone(), a.iter().map(|t| value(t, b)).collect()),
    }
}
fn domain(t: &Term) -> bool {
    match t {
        Term::App(n, a) => {
            a.is_empty() || ((n == "app" || n == "lam") && a.len() == 2 && a.iter().all(domain))
        }
        _ => false,
    }
}
fn normal(t: &Term) -> bool {
    match t {
        Term::App(_, a) if a.is_empty() => true,
        Term::App(n, a) if n == "app" && a.len() == 2 => {
            !matches!(&a[0],Term::App(n,a)if n=="lam"&&a.len()==2) && a.iter().all(normal)
        }
        Term::App(n, a) if n == "lam" && a.len() == 2 => normal(&a[1]),
        _ => false,
    }
}
fn allowed(t: &Term, r: R) -> bool {
    domain(t)
        && (r == R::Domain
            || (normal(t)
                && (r != R::Neutral || !matches!(t,Term::App(n,a)if n=="lam"&&a.len()==2))))
}
fn oracle(a: &Answer, x: &Term, y: &Term) -> bool {
    let name = |t: &Term| matches!(t,Term::App(n,args)if args.is_empty()&&a.alphabet.as_ref().is_none_or(|xs|xs.contains(n)));
    for h in [atom("a"), atom("b"), atom("c")] {
        let b = BTreeMap::from([(Var(0), x.clone()), (Var(1), y.clone()), (Var(2), h)]);
        if a.names.iter().all(|t| name(&value(t, &b)))
            && a.structure.iter().all(|(t, r)| allowed(&value(t, &b), *r))
            && a.unequal.iter().all(|(x, y)| {
                let (x, y) = (value(x, &b), value(y, &b));
                name(&x) && name(&y) && x != y
            })
        {
            return true;
        }
    }
    false
}
fn id(t: &Term) -> Var {
    let Term::Var(x) = t else {
        panic!("expected output hole")
    };
    *x
}
fn base() -> Answer {
    Answer {
        imports: BTreeSet::new(),
        outputs: vec![v(0), v(1), t("pair", [v(3), v(3)])],
        names: vec![],
        structure: vec![],
        unequal: vec![],
        alphabet: None,
    }
}
#[test]
fn transported_symbolic_membership_matches_independent_denotations() {
    let mut checks = 0;
    for mask in 0..4 {
        for edges in 0..8 {
            for template in [v(0), t("app", [v(0), v(1)]), t("lam", [v(0), v(1)])] {
                for r in [R::Domain, R::Normal, R::Neutral] {
                    for mode in 0..3 {
                        for finite in [false, true] {
                            let mut a = base();
                            a.names = (0..2).filter(|i| mask & (1 << i) != 0).map(v).collect();
                            a.structure = vec![(template.clone(), r)];
                            for (bit, (x, y)) in [(0, 1), (0, 2), (1, 2)].into_iter().enumerate() {
                                if edges & (1 << bit) != 0 {
                                    a.unequal.push((v(x), v(y)))
                                }
                            }
                            if finite {
                                a.alphabet = Some(vec!["a".into(), "b".into()])
                            }
                            let caller = match mode {
                                0 => BTreeMap::new(),
                                1 => BTreeMap::from([(Var(0), Var(50))]),
                                _ => BTreeMap::from([(Var(0), Var(50)), (Var(1), Var(50))]),
                            };
                            a.imports = caller.keys().copied().collect();
                            let mut fresh = Fresh {
                                next: Some(50),
                                occupied: BTreeSet::from([Var(50), Var(51)]),
                            };
                            let answer = a.instantiate(&caller, &mut fresh).unwrap();
                            let x = id(&answer.outputs()[0]);
                            let y = id(&answer.outputs()[1]);
                            let Term::App(_, holes) = &answer.outputs()[2] else {
                                panic!()
                            };
                            assert_eq!(holes[0], holes[1]);
                            assert!(![x, y, Var(50), Var(51)].contains(&id(&holes[0])));
                            for xv in [atom("a"), atom("b"), t("lam", [atom("a"), atom("a")])] {
                                for yv in [atom("a"), atom("b"), t("lam", [atom("a"), atom("a")])] {
                                    if x == y && xv != yv {
                                        continue;
                                    }
                                    let b = BTreeMap::from([(x, xv.clone()), (y, yv.clone())]);
                                    assert_eq!(
                                        answer.consistent(&b).unwrap(),
                                        oracle(&a, &xv, &yv)
                                    );
                                    checks += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(checks, 12096);
    println!("symbolic_membership_checks={checks}");
}
#[test]
fn independent_answers_preserve_only_declared_caller_sharing() {
    let mut a = base();
    a.imports.insert(Var(0));
    a.names = vec![v(0), v(1)];
    a.unequal = vec![(v(2), v(0)), (v(2), v(1))];
    a.alphabet = Some(vec!["a".into(), "b".into()]);
    let caller = BTreeMap::from([(Var(0), Var(9))]);
    let mut fresh = Fresh {
        next: Some(9),
        occupied: BTreeSet::from([Var(9)]),
    };
    let one = a.instantiate(&caller, &mut fresh).unwrap();
    let two = a.instantiate(&caller, &mut fresh).unwrap();
    assert_eq!(id(&one.outputs()[0]), Var(9));
    assert_eq!(id(&two.outputs()[0]), Var(9));
    assert_ne!(one.outputs()[1], two.outputs()[1]);
    assert_ne!(one.outputs()[2], two.outputs()[2]);
    let y = id(&one.outputs()[1]);
    assert!(
        one.consistent(&BTreeMap::from([(Var(9), atom("a"))]))
            .unwrap()
    );
    assert!(
        !one.consistent(&BTreeMap::from([(Var(9), atom("a")), (y, atom("b"))]))
            .unwrap()
    );
    let snapshot = fresh.clone();
    assert!(a.instantiate(&BTreeMap::new(), &mut fresh).is_err());
    assert_eq!(fresh, snapshot);
    assert!(
        a.instantiate(
            &BTreeMap::from([(Var(0), Var(9)), (Var(8), Var(10))]),
            &mut fresh
        )
        .is_err()
    );
    assert_eq!(fresh, snapshot);
    let mut end = Fresh {
        next: Some(u64::MAX),
        occupied: BTreeSet::new(),
    };
    let snapshot = end.clone();
    assert!(a.instantiate(&caller, &mut end).is_err());
    assert_eq!(end, snapshot);
}
#[test]
fn hidden_capture_is_rejected_and_nonground_output_is_not_ground_decoding() {
    use chr_structural::joint_region::{Observation, Region};
    let mut a = base();
    a.outputs.push(atom("fixed"));
    a.unequal = vec![(v(2), v(0)), (v(2), v(1))];
    a.alphabet = Some(vec!["a".into(), "b".into()]);
    let mut fresh = Fresh {
        next: Some(100),
        occupied: BTreeSet::new(),
    };
    let instance = a.instantiate(&BTreeMap::new(), &mut fresh).unwrap();
    assert_eq!(instance.outputs()[3], atom("fixed"));
    // Formal IDs 0,1,2,3 receive 100,101,102,103; 102 occurs only in the conjunction.
    assert!(
        instance
            .consistent(&BTreeMap::from([(Var(102), atom("a"))]))
            .is_err()
    );
    assert!(
        instance
            .consistent(&BTreeMap::from([(Var(100), v(102))]))
            .is_err()
    );
    assert!(instance.consistent(&BTreeMap::new()).unwrap());
    assert!(
        instance
            .consistent(&BTreeMap::from([(Var(103), t("pair", [v(103), v(103)]))]))
            .is_err()
    );
    let region = Region {
        domains: BTreeMap::from([(Var(0), vec![instance.outputs()[2].clone()])]),
        predicates: vec![],
    };
    assert!(
        region
            .prepare(&[Var(0)], &[], &[], Observation::LogicalSet, 100_000)
            .is_err()
    );
    let returned = instance.outputs().to_vec();
    drop(instance);
    drop(a);
    let Term::App(name, holes) = &returned[2] else {
        panic!()
    };
    assert_eq!(name, "pair");
    assert_eq!(holes[0], holes[1]);
}
