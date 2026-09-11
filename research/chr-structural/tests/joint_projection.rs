use chr_structural::{
    joint_region::{Observation as O, Predicate as P, Region},
    normal_forms::Requirement as R,
};
use chr_syntax::{Rule, Term, Var, atom, c, t, v};
use std::collections::{BTreeMap, BTreeSet};
fn value(t: &Term, b: &BTreeMap<Var, Term>) -> Term {
    match t {
        Term::Var(x) => b[x].clone(),
        Term::App(n, a) => Term::App(n.clone(), a.iter().map(|t| value(t, b)).collect()),
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
fn accepted(p: &P, b: &BTreeMap<Var, Term>) -> bool {
    let name = |x: &Term| matches!(x,Term::App(_,a)if a.is_empty());
    match p {
        P::Name(t) => name(&value(t, b)),
        P::Different(a, z) => {
            let (a, z) = (value(a, b), value(z, b));
            name(&a) && name(&z) && a != z
        }
        P::Equal(a, z) => value(a, b) == value(z, b),
        P::Structure(t, R::Normal) => normal(&value(t, b)),
        _ => panic!("matrix oracle scope"),
    }
}
fn enumeration(r: &Region, visible: &[Var]) -> BTreeSet<Vec<Term>> {
    enumeration_weights(r, visible).into_keys().collect()
}
fn enumeration_weights(r: &Region, visible: &[Var]) -> BTreeMap<Vec<Term>, u128> {
    fn walk(
        r: &Region,
        vars: &[Var],
        b: &mut BTreeMap<Var, Term>,
        visible: &[Var],
        out: &mut BTreeMap<Vec<Term>, u128>,
    ) {
        if let Some((x, rest)) = vars.split_first() {
            for t in &r.domains[x] {
                b.insert(*x, t.clone());
                walk(r, rest, b, visible, out)
            }
        } else if r.predicates.iter().all(|p| accepted(p, b)) {
            *out.entry(visible.iter().map(|x| b[x].clone()).collect())
                .or_default() += 1;
        }
    }
    let mut out = BTreeMap::new();
    walk(
        r,
        &r.domains.keys().copied().collect::<Vec<_>>(),
        &mut BTreeMap::new(),
        visible,
        &mut out,
    );
    out
}
#[test]
fn exact_joint_sets_match_independent_full_assignments() {
    let terms = [atom("a"), atom("b"), t("lam", [atom("a"), atom("a")])];
    let domains = (0..3)
        .map(|i| (Var(i), terms.to_vec()))
        .collect::<BTreeMap<_, _>>();
    let mut count = 0;
    for tree in [
        t("app", [v(0), v(1)]),
        t("app", [v(0), v(0)]),
        t("lam", [v(0), v(1)]),
    ] {
        for mask in 0..4 {
            for edges in 0..8 {
                for output in 0..8 {
                    for alias in [false, true] {
                        let mut predicates = vec![P::Structure(tree.clone(), R::Normal)];
                        for i in 0..2 {
                            if mask & (1 << i) != 0 {
                                predicates.push(P::Name(v(i)))
                            }
                        }
                        for (bit, (a, b)) in [(0, 1), (0, 2), (1, 2)].into_iter().enumerate() {
                            if edges & (1 << bit) != 0 {
                                predicates.push(P::Different(v(a), v(b)))
                            }
                        }
                        if alias {
                            predicates.push(P::Equal(v(0), v(1)))
                        }
                        let region = Region {
                            domains: domains.clone(),
                            predicates,
                        };
                        let visible = (0..3)
                            .rev()
                            .filter(|i| output & (1 << i) != 0)
                            .map(Var)
                            .collect::<Vec<_>>();
                        let prepared = region
                            .prepare(&visible, &[], &[], O::LogicalSet, 100_000)
                            .unwrap();
                        assert_eq!(
                            prepared.answers(&[], 100_000).unwrap(),
                            enumeration(&region, &visible)
                        );
                        count += 1;
                    }
                }
            }
        }
    }
    assert_eq!(count, 1536);
    println!("joint_projection_sets={count}");
}
#[test]
fn hidden_dependencies_and_changed_caller_restrictions_survive() {
    let region = Region {
        domains: (0..3)
            .map(|i| (Var(i), vec![atom("a"), atom("b")]))
            .collect(),
        predicates: vec![P::Different(v(2), v(0)), P::Different(v(2), v(1))],
    };
    let p = region
        .prepare(&[Var(1), Var(0)], &[], &[], O::LogicalSet, 100_000)
        .unwrap();
    assert_eq!(
        p.answers(&[], 100_000).unwrap(),
        BTreeSet::from([vec![atom("a"), atom("a")], vec![atom("b"), atom("b")]])
    );
    for name in ["a", "b"] {
        assert_eq!(
            p.answers(&[(Var(0), atom(name))], 100_000).unwrap(),
            BTreeSet::from([vec![atom(name), atom(name)]])
        );
    }
    assert!(
        p.answers(&[(Var(0), atom("a")), (Var(0), atom("b"))], 100_000)
            .unwrap()
            .is_empty()
    );
    assert!(p.answers(&[(Var(2), atom("a"))], 100_000).is_err());
    assert!(
        p.answers(&[(Var(0), atom("outside"))], 100_000)
            .unwrap()
            .is_empty()
    );
    assert!(
        p.answers(&[(Var(0), atom("outside")), (Var(99), atom("a"))], 100_000)
            .is_err()
    );
    assert!(p.answers(&[], 0).is_err());
    assert!(
        region
            .prepare(&[Var(0)], &[c("host", [v(2)])], &[], O::LogicalSet, 100_000)
            .is_err()
    );
    assert!(
        region
            .prepare(
                &[Var(0), Var(2)],
                &[c("host", [v(2)])],
                &[],
                O::LogicalSet,
                100_000
            )
            .is_ok()
    );
}
#[test]
fn observation_groundness_and_resource_bounds_are_checked() {
    let mut region = Region {
        domains: BTreeMap::from([(Var(0), vec![atom("a"), atom("a")])]),
        predicates: vec![P::Name(v(0))],
    };
    assert_eq!(
        region
            .prepare(&[Var(0)], &[], &[], O::LogicalSet, 100_000)
            .unwrap()
            .answers(&[], 100_000)
            .unwrap(),
        BTreeSet::from([vec![atom("a")]])
    );
    assert!(
        region
            .prepare(&[Var(0)], &[], &[], O::RawAnswers, 100_000)
            .is_err()
    );
    for rule in [
        Rule::propagate("observe", [c("var", [v(0)])], c("seen", []).into()),
        Rule::simplify("post", [c("trigger", [])], c("norm", [v(0)]).into()),
    ] {
        assert!(
            region
                .prepare(&[Var(0)], &[], &[rule], O::LogicalSet, 100_000)
                .is_err()
        );
    }
    assert!(
        region
            .prepare(&[Var(0)], &[], &[], O::LogicalSet, 0)
            .is_err()
    );
    region.domains.insert(Var(0), vec![v(9)]);
    assert!(
        region
            .prepare(&[Var(0)], &[], &[], O::LogicalSet, 100_000)
            .is_err()
    );
    region.domains.insert(Var(0), vec![]);
    assert!(
        region
            .prepare(&[Var(0)], &[], &[], O::LogicalSet, 100_000)
            .unwrap()
            .answers(&[], 100_000)
            .unwrap()
            .is_empty()
    );
    region
        .domains
        .insert(Var(0), (0..257).map(|i| atom(&format!("a{i}"))).collect());
    assert!(
        region
            .prepare(&[Var(0)], &[], &[], O::LogicalSet, 100_000)
            .is_err()
    );
}

#[test]
fn counted_aliases_preserve_domain_choice_weights() {
    let region = Region {
        domains: BTreeMap::from([(Var(0), vec![atom("a"), atom("a"), atom("b")])]),
        predicates: vec![],
    };
    let prepared = region
        .prepare(&[Var(0), Var(0)], &[], &[], O::Counted, 100)
        .unwrap();
    assert_eq!(
        prepared.weighted_answers(&[], 100).unwrap(),
        BTreeMap::from([
            (vec![atom("a"), atom("a")], 2),
            (vec![atom("b"), atom("b")], 1),
        ])
    );
}

#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
#[allow(dead_code)]
mod scalar;

// Truth-table lowering is a source control, independent of the production
// predicate compiler. Each satisfying assignment has exactly one filter branch.
fn source(r: &Region, visible: &[Var]) -> (Vec<Rule>, chr_syntax::Query) {
    use chr_syntax::{Goal, eq, or};
    fn choices(mut gs: Vec<Goal>) -> Goal {
        let mut result = gs.pop().unwrap_or(Goal::Fail);
        while let Some(g) = gs.pop() {
            result = or(g, result);
        }
        result
    }
    let mut rules = vec![];
    let mut constraints = vec![];
    for (x, domain) in &r.domains {
        let name = format!("choose{}", x.0);
        rules.push(Rule::simplify(
            &name,
            [c(&name, [v(0)])],
            choices(domain.iter().map(|t| eq(v(0), t.clone())).collect()),
        ));
        constraints.push(c(&name, [Term::Var(*x)]));
    }
    let all = r.domains.keys().copied().collect::<Vec<_>>();
    for (i, predicate) in r.predicates.iter().enumerate() {
        let table = enumeration(
            &Region {
                domains: r.domains.clone(),
                predicates: vec![predicate.clone()],
            },
            &all,
        );
        let name = format!("filter{i}");
        rules.push(Rule::simplify(
            &name,
            [c(
                &name,
                all.iter().map(|x| Term::Var(*x)).collect::<Vec<_>>(),
            )],
            choices(
                table
                    .into_iter()
                    .map(|row| {
                        Goal::And(
                            all.iter()
                                .zip(row)
                                .map(|(x, t)| eq(Term::Var(*x), t))
                                .collect(),
                        )
                    })
                    .collect(),
            ),
        ));
        constraints.push(c(
            &name,
            all.iter().map(|x| Term::Var(*x)).collect::<Vec<_>>(),
        ));
    }
    (
        rules,
        chr_syntax::Query {
            constraints,
            outputs: visible
                .iter()
                .enumerate()
                .map(|(i, x)| (format!("out{i}"), *x))
                .collect(),
        },
    )
}

#[test]
fn connected_counted_sources_match_scalar_and_reference() {
    let mut configurations = 0;
    let mut observations = 0;
    for duplicate in [false, true] {
        let mut choices = vec![atom("a"), atom("b"), t("lam", [atom("a"), atom("a")])];
        if duplicate {
            choices.push(atom("a"));
        }
        for pattern in 0..4 {
            let mut predicates = vec![P::Different(v(2), v(0)), P::Different(v(2), v(1))];
            match pattern {
                0 => (),
                1 => predicates.push(P::Equal(v(0), v(1))),
                2 => predicates.push(predicates[0].clone()),
                3 => predicates.push(P::Equal(v(2), v(0))),
                _ => unreachable!(),
            }
            let r = Region {
                domains: (0..3).map(|i| (Var(i), choices.clone())).collect(),
                predicates,
            };
            for mask in 0..8 {
                for alias in [false, true] {
                    let mut visible = (0..3)
                        .rev()
                        .filter(|i| mask & (1 << i) != 0)
                        .map(Var)
                        .collect::<Vec<_>>();
                    if alias && !visible.is_empty() {
                        visible.push(visible[0]);
                    }
                    let expected = enumeration_weights(&r, &visible);
                    let (rules, query) = source(&r, &visible);
                    let raw = scalar::run(&rules, &query, 1_000_000);
                    let mut bag = BTreeMap::new();
                    for answer in raw {
                        assert!(answer.residual.is_empty());
                        *bag.entry(
                            answer
                                .outputs
                                .into_iter()
                                .map(|(_, t)| t)
                                .collect::<Vec<_>>(),
                        )
                        .or_insert(0u128) += 1;
                    }
                    assert_eq!(bag, expected);
                    let mut reference = chr_reference::Search::new(rules, query).unwrap();
                    let batch = reference.advance(1_000_000);
                    assert!(batch.exhausted);
                    let set = batch
                        .answers
                        .into_iter()
                        .map(|a| {
                            assert!(a.residual.is_empty());
                            a.outputs.into_iter().map(|(_, t)| t).collect::<Vec<_>>()
                        })
                        .collect::<BTreeSet<_>>();
                    assert_eq!(set, expected.keys().cloned().collect());
                    for mode in [O::LogicalSet, O::Counted] {
                        let prepared = r.prepare(&visible, &[], &[], mode, 100_000).unwrap();
                        for restricted in [None, Some(atom("a")), Some(atom("b")), None] {
                            let restrictions = visible
                                .first()
                                .zip(restricted)
                                .map(|(x, t)| vec![(*x, t)])
                                .unwrap_or_default();
                            let wanted = expected
                                .iter()
                                .filter(|(row, _)| {
                                    restrictions.is_empty() || row[0] == restrictions[0].1
                                })
                                .map(|(row, n)| {
                                    (row.clone(), if matches!(mode, O::Counted) { *n } else { 1 })
                                })
                                .collect::<BTreeMap<_, _>>();
                            assert_eq!(
                                prepared.weighted_answers(&restrictions, 100_000).unwrap(),
                                wanted
                            );
                            observations += 1;
                        }
                    }
                    configurations += 1;
                }
            }
        }
    }
    assert_eq!(configurations, 128);
    assert_eq!(observations, 1024);
}
