use chr_structural::{
    normal_forms::{Requirement as R, Summary},
    regular::{Automaton, Transition},
};
use chr_syntax::{Answer, Query, Term, Var, atom, c, t, v};
use std::collections::{BTreeMap, BTreeSet};
fn trees(leaves: &[Term], limit: usize) -> Vec<Term> {
    let mut sizes = vec![vec![]; limit + 1];
    sizes[1] = leaves.to_vec();
    for n in 2..=limit {
        let mut ts = vec![];
        for left in 1..n - 1 {
            for a in &sizes[left] {
                for b in &sizes[n - 1 - left] {
                    for constructor in ["app", "lam"] {
                        ts.push(t(constructor, [a.clone(), b.clone()]));
                    }
                }
            }
        }
        sizes[n] = ts;
    }
    sizes.into_iter().flatten().collect()
}
fn domain(t: &Term) -> bool {
    match t {
        Term::Var(_) => panic!("ground oracle"),
        Term::App(n, args) => {
            args.is_empty()
                || ((n == "app" || n == "lam") && args.len() == 2 && args.iter().all(domain))
        }
    }
}
fn no_redex(t: &Term) -> bool {
    match t {
        Term::App(n, args) if n == "app" && args.len() == 2 => {
            !matches!(&args[0],Term::App(s,a) if s=="lam"&&a.len()==2) && args.iter().all(no_redex)
        }
        Term::App(n, args) if n == "lam" && args.len() == 2 => no_redex(&args[1]),
        Term::App(_, args) if args.is_empty() => true,
        _ => false,
    }
}
fn expected(t: &Term, r: R) -> bool {
    domain(t)
        && (r == R::Domain
            || (no_redex(t)
                && (r != R::Neutral || !matches!(t,Term::App(n,a) if n=="lam"&&a.len()==2))))
}
fn substitute(t: &Term, values: &BTreeMap<Var, Term>) -> Term {
    match t {
        Term::Var(x) => values[x].clone(),
        Term::App(n, args) => Term::App(
            n.clone(),
            args.iter().map(|t| substitute(t, values)).collect(),
        ),
    }
}
fn grammar(r: R) -> Automaton {
    let mut states = vec![vec![Transition::new("a", []), Transition::new("b", [])]; 3];
    states[0].extend([
        Transition::new("app", [0, 0]),
        Transition::new("lam", [0, 0]),
    ]);
    states[1].extend([
        Transition::new("app", [2, 1]),
        Transition::new("lam", [0, 1]),
    ]);
    states[2].push(Transition::new("app", [2, 1]));
    Automaton::new(
        states,
        match r {
            R::Domain => 0,
            R::Normal => 1,
            R::Neutral => 2,
        },
    )
    .unwrap()
}
#[test]
fn full_and_partial_denotations_match_independent_redex_checks() {
    let ground = trees(&[atom("a"), atom("b")], 7);
    assert_eq!(ground.len(), 714);
    let partial = trees(&[atom("a"), atom("b"), v(0), v(1)], 5);
    assert_eq!(partial.len(), 548);
    let values = trees(&[atom("a"), atom("b")], 3);
    assert_eq!(values.len(), 10);
    let mut complete_checks = 0;
    let mut partial_checks = 0;
    for r in [R::Domain, R::Normal, R::Neutral] {
        let automaton = grammar(r);
        for tree in &ground {
            let wanted = expected(tree, r);
            assert_eq!(Summary::compile(&[(tree.clone(), r)]).is_some(), wanted);
            assert_eq!(
                automaton.contains_term(tree, &mut Default::default()),
                Some(wanted)
            );
            complete_checks += 1;
        }
        for tree in &partial {
            let summary = Summary::compile(&[(tree.clone(), r)]);
            for a in &values {
                for b in &values {
                    let env = BTreeMap::from([(Var(0), a.clone()), (Var(1), b.clone())]);
                    let wanted = expected(&substitute(tree, &env), r);
                    let actual = summary.as_ref().and_then(|s| s.refine(&env).unwrap());
                    assert_eq!(
                        actual.is_some(),
                        wanted,
                        "tree={tree:?} requirement={r:?} values={env:?}"
                    );
                    if let Some(s) = actual {
                        assert_eq!(s.variables().count(), 0);
                    }
                    partial_checks += 1;
                }
            }
        }
    }
    assert_eq!(complete_checks, 2142);
    assert_eq!(partial_checks, 164400);
    println!("ground_memberships={complete_checks} partial_assignment_checks={partial_checks}");
}
// Independent literal reduction equations, under a fixed set of companion atoms.
// All companion occurrences survive; no source scheduler or matcher is reused here.
fn residual(tree: &Term, companions: &BTreeSet<String>) -> Option<Vec<Term>> {
    if let Term::App(n, a) = tree {
        if n == "app" && a.len() == 2 {
            if matches!(&a[0],Term::App(s,b) if s=="lam"&&b.len()==2) {
                return None;
            }
            if matches!(&a[0],Term::App(s,b) if s=="app"&&b.len()==2) {
                let mut left = residual(&a[0], companions)?;
                left.extend(residual(&a[1], companions)?);
                return Some(left);
            }
            if matches!(&a[0],Term::App(s,b) if b.is_empty()&&companions.contains(s)) {
                return residual(&a[1], companions);
            }
        }
        if n == "lam" && a.len() == 2 {
            return residual(&a[1], companions);
        }
        if a.is_empty() && companions.contains(n) {
            return Some(vec![]);
        }
    }
    Some(vec![tree.clone()])
}
fn source(tree: Term, companions: Vec<Term>) -> Vec<Answer> {
    let mut constraints = vec![c("norm", [tree])];
    constraints.extend(companions.into_iter().map(|t| c("var", [t])));
    let mut search = chr_reference::Search::new(
        chr_programs::lambda(),
        Query {
            constraints,
            outputs: vec![],
        },
    )
    .unwrap();
    let r = search.advance(2000);
    assert!(r.exhausted);
    r.answers
}
fn compare(a: &[Answer], b: &[Answer]) {
    assert_eq!(a.len(), b.len());
    for (a, b) in a.iter().zip(b) {
        assert!(
            chr_observe::equivalent(a, b, &mut Default::default()),
            "{a:?} != {b:?}"
        );
    }
}
#[test]
fn literal_ground_source_preserves_companion_and_residual_boundaries() {
    let mut checks = 0;
    for tree in trees(&[atom("a"), atom("b")], 7) {
        for mask in 0..4 {
            let companions: BTreeSet<String> = ["a", "b"]
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, s)| s.to_string())
                .collect();
            let expected = residual(&tree, &companions)
                .map(|remaining| {
                    let mut constraints: Vec<_> =
                        companions.iter().map(|s| c("var", [atom(s)])).collect();
                    constraints.extend(remaining.into_iter().map(|t| c("norm", [t])));
                    Answer {
                        outputs: vec![],
                        residual: constraints,
                    }
                })
                .into_iter()
                .collect::<Vec<_>>();
            compare(
                &source(tree.clone(), companions.iter().map(|s| atom(s)).collect()),
                &expected,
            );
            checks += 1;
        }
    }
    assert_eq!(checks, 2856);
    println!("literal_source_queries={checks}");
}
#[test]
fn repeated_holes_and_late_bindings_keep_the_strongest_requirement() {
    let summary = Summary::compile(&[(t("app", [v(0), v(0)]), R::Normal)]).unwrap();
    assert_eq!(
        summary.variables().collect::<Vec<_>>(),
        vec![(Var(0), R::Neutral)]
    );
    let lambda = t("lam", [atom("a"), atom("a")]);
    assert!(Summary::compile(&[(lambda.clone(), R::Normal)]).is_some());
    assert!(
        summary
            .refine(&BTreeMap::from([(Var(0), lambda.clone())]))
            .unwrap()
            .is_none()
    );
    let shared =
        Summary::compile(&[(v(0), R::Domain), (v(0), R::Normal), (v(0), R::Neutral)]).unwrap();
    assert_eq!(shared, summary);
    assert!(
        shared
            .refine(&BTreeMap::from([(Var(0), v(1)), (Var(1), lambda)]))
            .unwrap()
            .is_none()
    );
    assert!(
        shared
            .refine(&BTreeMap::from([(Var(0), t("app", [v(0), atom("a")]))]))
            .is_err()
    );
    assert!(Summary::compile(&[(t("app", [t("lam", [v(0), v(1)]), v(2)]), R::Normal)]).is_none());
}
#[test]
fn stronger_logical_recognition_cannot_replace_source_observation() {
    let tree = t(
        "app",
        [
            atom("a"),
            t("app", [t("lam", [atom("b"), atom("b")]), atom("a")]),
        ],
    );
    assert!(Summary::compile(&[(tree.clone(), R::Normal)]).is_none());
    compare(
        &source(tree.clone(), vec![]),
        &[Answer {
            outputs: vec![],
            residual: vec![c("norm", [tree.clone()])],
        }],
    );
    assert!(source(tree, vec![atom("a")]).is_empty());
    compare(
        &source(v(0), vec![v(0)]),
        &[Answer {
            outputs: vec![],
            residual: vec![c("var", [v(0)])],
        }],
    );
    let other = t("other", [atom("a")]);
    assert!(Summary::compile(&[(other.clone(), R::Domain)]).is_none());
    compare(
        &source(other.clone(), vec![]),
        &[Answer {
            outputs: vec![],
            residual: vec![c("norm", [other])],
        }],
    );
}
