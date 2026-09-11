use chr_structural::{joint::feasible, normal_forms::Requirement as R};
use chr_syntax::{Term, Var, atom, t, v};
use std::collections::BTreeMap;
fn ground(t: &Term, values: &BTreeMap<Var, Term>) -> Term {
    match t {
        Term::Var(x) => values[x].clone(),
        Term::App(n, a) => Term::App(n.clone(), a.iter().map(|t| ground(t, values)).collect()),
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
fn no_redex(t: &Term) -> bool {
    match t {
        Term::App(n, a) if n == "app" && a.len() == 2 => {
            !matches!(&a[0],Term::App(n,a)if n=="lam"&&a.len()==2) && a.iter().all(no_redex)
        }
        Term::App(n, a) if n == "lam" && a.len() == 2 => no_redex(&a[1]),
        Term::App(_, a) if a.is_empty() => true,
        _ => false,
    }
}
fn accepts(t: &Term, r: R) -> bool {
    domain(t)
        && (r == R::Domain
            || (no_redex(t)
                && (r != R::Neutral || !matches!(t,Term::App(n,a)if n=="lam"&&a.len()==2))))
}
fn name(t: &Term, alphabet: Option<&[String]>) -> bool {
    matches!(t,Term::App(n,a)if a.is_empty()&&alphabet.is_none_or(|xs|xs.contains(n)))
}
fn oracle(
    names: &[Term],
    structure: &[(Term, R)],
    neq: &[(Term, Term)],
    bindings: &BTreeMap<Var, Term>,
    alphabet: Option<&[String]>,
) -> bool {
    let mut values = ["a", "b", "c", "d", "e"].map(atom).to_vec();
    values.extend([
        t("app", [atom("a"), atom("a")]),
        t("lam", [atom("a"), atom("a")]),
        t("app", [t("lam", [atom("a"), atom("a")]), atom("a")]),
    ]);
    for x in &values {
        for y in &values {
            for h in &values {
                let given = BTreeMap::from([
                    (Var(0), x.clone()),
                    (Var(1), y.clone()),
                    (Var(2), h.clone()),
                ]);
                if !bindings.iter().all(|(x, t)| given[x] == ground(t, &given)) {
                    continue;
                }
                if names.iter().all(|t| name(&ground(t, &given), alphabet))
                    && structure
                        .iter()
                        .all(|(t, r)| accepts(&ground(t, &given), *r))
                    && neq.iter().all(|(a, b)| {
                        let (a, b) = (ground(a, &given), ground(b, &given));
                        name(&a, alphabet) && name(&b, alphabet) && a != b
                    })
                {
                    return true;
                }
            }
        }
    }
    false
}
#[test]
fn conjunction_matches_independent_partial_denotations() {
    let templates = [
        v(0),
        t("app", [v(0), v(1)]),
        t("app", [v(0), v(0)]),
        t("lam", [v(0), v(1)]),
    ];
    let neqs = [
        vec![],
        vec![(v(0), v(1))],
        vec![(v(0), atom("a"))],
        vec![(v(2), v(0)), (v(2), v(1))],
    ];
    let bindings = [
        BTreeMap::new(),
        BTreeMap::from([(Var(0), v(1))]),
        BTreeMap::from([(Var(0), atom("a"))]),
        BTreeMap::from([(Var(0), t("lam", [atom("a"), atom("a")]))]),
        BTreeMap::from([(Var(0), v(1)), (Var(1), atom("a"))]),
    ];
    let alphabets = [
        Some(vec![]),
        Some(vec!["a".into()]),
        Some(vec!["a".into(), "b".into()]),
        None,
    ];
    let mut checks = 0;
    for term in templates {
        for r in [R::Domain, R::Normal, R::Neutral] {
            for mask in 0..4 {
                let names = (0..2)
                    .filter(|i| mask & (1 << i) != 0)
                    .map(v)
                    .collect::<Vec<_>>();
                for neq in &neqs {
                    for bindings in &bindings {
                        for alphabet in &alphabets {
                            let structure = [(term.clone(), r)];
                            let before = bindings.clone();
                            assert_eq!(
                                feasible(&names, &structure, neq, bindings, alphabet.as_deref())
                                    .unwrap(),
                                oracle(&names, &structure, neq, bindings, alphabet.as_deref()),
                                "{names:?} {structure:?} {neq:?} {bindings:?} {alphabet:?}"
                            );
                            assert_eq!(&before, bindings);
                            checks += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(checks, 3840);
    println!("joint_feasibility_checks={checks}");
}
#[test]
fn alias_freshness_cycles_and_binding_order_are_explicit() {
    let alphabet = vec!["a".into(), "b".into()];
    let neq = [(v(2), v(0)), (v(2), v(1))];
    let distinct = BTreeMap::from([(Var(0), atom("a")), (Var(1), atom("b"))]);
    assert!(!feasible(&[], &[], &neq, &distinct, Some(&alphabet)).unwrap());
    assert!(feasible(&[], &[], &neq, &distinct, None).unwrap());
    assert!(
        !feasible(
            &[],
            &[],
            &[(v(0), v(1))],
            &BTreeMap::from([(Var(0), v(1))]),
            None
        )
        .unwrap()
    );
    let steps = [(Var(0), v(1)), (Var(1), atom("a")), (Var(2), atom("b"))];
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let mut b = BTreeMap::new();
        for i in order {
            b.insert(steps[i].0, steps[i].1.clone());
            assert_eq!(
                feasible(
                    &[v(0)],
                    &[(t("app", [v(0), v(0)]), R::Normal)],
                    &neq,
                    &b,
                    Some(&alphabet)
                )
                .unwrap(),
                oracle(
                    &[v(0)],
                    &[(t("app", [v(0), v(0)]), R::Normal)],
                    &neq,
                    &b,
                    Some(&alphabet)
                )
            );
        }
    }
    assert!(feasible(&[v(0)], &[], &[], &BTreeMap::from([(Var(0), v(0))]), None).unwrap());
    assert!(
        feasible(
            &[v(0)],
            &[],
            &[],
            &BTreeMap::from([(Var(0), t("app", [v(0), atom("a")]))]),
            None
        )
        .is_err()
    );
    assert!(
        feasible(
            &[v(0)],
            &[],
            &[],
            &BTreeMap::from([(Var(9), v(10)), (Var(10), v(9))]),
            None
        )
        .unwrap()
    );
}
fn source(
    constraints: Vec<chr_syntax::Constraint>,
    extra: Vec<chr_syntax::Rule>,
) -> Vec<chr_syntax::Answer> {
    let mut rules = extra;
    rules.extend(chr_programs::lambda());
    let mut search = chr_reference::Search::new(
        rules,
        chr_syntax::Query {
            constraints,
            outputs: vec![],
        },
    )
    .unwrap();
    let result = search.advance(2000);
    assert!(result.exhausted);
    result.answers
}
#[test]
fn closed_ground_existence_agrees_but_raw_host_effects_remain() {
    use chr_syntax::{Rule, c};
    let templates = [
        v(0),
        t("app", [v(0), v(1)]),
        t("app", [v(0), v(0)]),
        t("lam", [v(0), v(1)]),
        t("app", [t("lam", [v(0), v(0)]), v(1)]),
    ];
    let neqs = [
        vec![],
        vec![(v(0), v(1))],
        vec![(v(0), atom("a"))],
        vec![(v(2), v(0)), (v(2), v(1))],
    ];
    let alphabet = vec!["a".into(), "b".into()];
    let mut count = 0;
    for tree in templates {
        for mask in 0..4 {
            let names = (0..2)
                .filter(|i| mask & (1 << i) != 0)
                .map(v)
                .collect::<Vec<_>>();
            for neq in &neqs {
                for bits in 0..8 {
                    let given = (0..3)
                        .map(|i| (Var(i), atom(if bits & (1 << i) == 0 { "a" } else { "b" })))
                        .collect::<BTreeMap<_, _>>();
                    let wanted = feasible(
                        &names,
                        &[(tree.clone(), R::Normal)],
                        neq,
                        &given,
                        Some(&alphabet),
                    )
                    .unwrap();
                    assert_eq!(
                        wanted,
                        oracle(
                            &names,
                            &[(tree.clone(), R::Normal)],
                            neq,
                            &given,
                            Some(&alphabet)
                        )
                    );
                    let mut constraints = vec![
                        c("var", [atom("a")]),
                        c("var", [atom("b")]),
                        c("norm", [ground(&tree, &given)]),
                    ];
                    constraints.extend(names.iter().map(|t| c("var", [ground(t, &given)])));
                    constraints.extend(
                        neq.iter()
                            .map(|(a, b)| c("neq", [ground(a, &given), ground(b, &given)])),
                    );
                    assert_eq!(!source(constraints, vec![]).is_empty(), wanted);
                    count += 1;
                }
            }
        }
    }
    assert_eq!(count, 640);
    println!("joint_closed_source_queries={count}");
    let constraints = vec![
        c("var", [atom("a")]),
        c("norm", [atom("a")]),
        c("neq", [atom("a"), atom("b")]),
    ];
    assert!(
        feasible(
            &[atom("a")],
            &[(atom("a"), R::Normal)],
            &[(atom("a"), atom("b"))],
            &BTreeMap::new(),
            None
        )
        .unwrap()
    );
    let raw = source(constraints.clone(), vec![]);
    assert_eq!(raw.len(), 1);
    assert!(raw[0].residual.iter().any(|c| c.name == "var"));
    assert!(raw[0].residual.iter().any(|c| c.name == "neq"));
    let seen = source(
        constraints.clone(),
        vec![Rule::propagate(
            "observe",
            [c("var", [v(0)])],
            c("seen", [v(0)]).into(),
        )],
    );
    assert!(seen[0].residual.iter().any(|c| c.name == "seen"));
    let consume = Rule::simplify(
        "consume",
        [c("var", [v(0)]), c("permit", [])],
        c("taken", [v(0)]).into(),
    );
    let mut with_permit = constraints;
    with_permit.push(c("permit", []));
    let executed = source(with_permit, vec![consume.clone()]);
    let summary = source(vec![c("permit", [])], vec![consume]);
    assert!(executed[0].residual.iter().any(|c| c.name == "taken"));
    assert!(summary[0].residual.iter().all(|c| c.name != "taken"));
}
