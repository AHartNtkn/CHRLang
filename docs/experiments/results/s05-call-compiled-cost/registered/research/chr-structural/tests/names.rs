use chr_structural::names::{NameFormula, Theory};
use chr_syntax::{Answer, Query, Rule, Term, Var, atom, c, t, v};
use std::collections::BTreeMap;

fn templates() -> Vec<Term> {
    let mut terms = vec![
        v(0),
        v(1),
        atom("x"),
        atom("y"),
        atom("app"),
        atom("lam"),
        atom("other"),
    ];
    for name in ["app", "lam", "other"] {
        terms.push(t(name, [v(0)]));
        terms.push(t(name, [v(0), v(1)]));
    }
    terms
}
fn lists() -> Vec<Vec<Term>> {
    let ts = templates();
    let mut lists = vec![vec![]];
    for a in &ts {
        lists.push(vec![a.clone()]);
        for b in &ts {
            lists.push(vec![a.clone(), b.clone()]);
        }
    }
    lists
}
// Independent substitution traverses the entire finite tree, unlike the root summary.
fn substitute(term: &Term, values: &BTreeMap<Var, Term>) -> Term {
    match term {
        Term::Var(x) => values.get(x).cloned().unwrap_or_else(|| term.clone()),
        Term::App(n, args) => Term::App(
            n.clone(),
            args.iter().map(|x| substitute(x, values)).collect(),
        ),
    }
}
fn ground_denotation(term: &Term, atomic: bool) -> bool {
    match term {
        Term::Var(_) => panic!("oracle requires ground input"),
        Term::App(n, args) => {
            if atomic {
                args.is_empty()
            } else {
                !matches!((n.as_str(), args.as_slice()), ("app" | "lam", [_, _]))
            }
        }
    }
}
fn source(terms: &[Term], extra: Vec<Rule>) -> Vec<Answer> {
    let mut rules = extra;
    rules.extend(chr_programs::lambda());
    let mut s = chr_reference::Search::new(
        rules,
        Query {
            constraints: terms.iter().cloned().map(|x| c("var", [x])).collect(),
            outputs: vec![],
        },
    )
    .unwrap();
    let result = s.advance(1000);
    assert!(result.exhausted);
    result.answers
}
fn expected_source(terms: &[Term]) -> Vec<Answer> {
    if terms
        .iter()
        .any(|t| matches!(t, Term::App(n,args) if (n=="app" || n=="lam") && args.len()==2))
    {
        vec![]
    } else {
        vec![Answer {
            outputs: vec![],
            residual: terms.iter().cloned().map(|x| c("var", [x])).collect(),
        }]
    }
}
fn compare_answers(actual: &[Answer], expected: &[Answer]) {
    assert_eq!(actual.len(), expected.len());
    for (a, b) in actual.iter().zip(expected) {
        assert!(
            chr_observe::equivalent(a, b, &mut Default::default()),
            "{a:?} != {b:?}"
        );
    }
}
#[test]
fn finite_denotations_and_literal_source_residuals() {
    let terms = [
        atom("x"),
        atom("y"),
        atom("app"),
        atom("lam"),
        t("other", [atom("x")]),
        t("app", [atom("x"), atom("y")]),
        t("lam", [atom("x"), atom("y")]),
    ];
    let mut checks = 0;
    let mut source_checks = 0;
    for obligations in lists() {
        compare_answers(
            &source(&obligations, vec![]),
            &expected_source(&obligations),
        );
        for a in &terms {
            for b in &terms {
                let env = BTreeMap::from([(Var(0), a.clone()), (Var(1), b.clone())]);
                let ground: Vec<_> = obligations.iter().map(|t| substitute(t, &env)).collect();
                compare_answers(&source(&ground, vec![]), &expected_source(&ground));
                source_checks += 1;
                for (theory, atomic) in [(Theory::LiteralRoots, false), (Theory::AtomicNames, true)]
                {
                    let wanted = ground.iter().all(|t| ground_denotation(t, atomic));
                    let actual = NameFormula::compile(theory, &obligations)
                        .and_then(|f| f.refine(&env).unwrap());
                    assert_eq!(
                        actual.is_some(),
                        wanted,
                        "{theory:?}: {obligations:?}, {env:?}"
                    );
                    if let Some(f) = actual {
                        assert_eq!(f.variables().count(), 0);
                    }
                    checks += 1;
                }
            }
        }
    }
    assert_eq!(checks, 17934);
    assert_eq!(source_checks, 8967);
    println!(
        "denotation_checks={checks} ground_source_checks={source_checks} partial_source_checks=183"
    );
}
#[test]
fn late_binding_aliases_and_fresh_caller_transport() {
    let f = NameFormula::compile(Theory::AtomicNames, &[v(0), v(1), v(0)]).unwrap();
    let alias = f
        .refine(&BTreeMap::from([(Var(0), v(1))]))
        .unwrap()
        .unwrap();
    assert_eq!(alias.variables().collect::<Vec<_>>(), vec![Var(1)]);
    assert!(
        alias
            .refine(&BTreeMap::from([(
                Var(1),
                t("app", [atom("x"), atom("y")])
            )]))
            .unwrap()
            .is_none()
    );
    let renamed = f
        .transport(&BTreeMap::from([(Var(0), Var(10)), (Var(1), Var(11))]))
        .unwrap();
    assert_eq!(
        renamed.variables().collect::<Vec<_>>(),
        vec![Var(10), Var(11)]
    );
    assert!(
        f.transport(&BTreeMap::from([(Var(0), Var(10)), (Var(1), Var(10))]))
            .is_err()
    );
    assert!(f.transport(&BTreeMap::from([(Var(0), Var(10))])).is_err());
    assert!(
        renamed
            .refine(&BTreeMap::from([
                (Var(10), atom("x")),
                (Var(11), atom("x"))
            ]))
            .unwrap()
            .is_some()
    );
    assert!(
        f.refine(&BTreeMap::from([(Var(0), v(1)), (Var(1), v(0))]))
            .is_err()
    );
    assert!(
        f.refine(&BTreeMap::from([
            (Var(0), v(1)),
            (Var(1), t("lam", [atom("x"), atom("y")]))
        ]))
        .unwrap()
        .is_none()
    );
}
#[test]
fn source_domain_and_observation_are_distinct_contracts() {
    for term in [t("other", [atom("x")]), t("app", [atom("x")])] {
        assert!(NameFormula::compile(Theory::LiteralRoots, std::slice::from_ref(&term)).is_some());
        assert!(NameFormula::compile(Theory::AtomicNames, &[term]).is_none());
    }
    for name in ["app", "lam", "x"] {
        assert!(NameFormula::compile(Theory::AtomicNames, &[atom(name)]).is_some());
    }
    let observers = vec![Rule::propagate(
        "observe",
        [c("var", [v(0)])],
        c("seen", [v(0)]).into(),
    )];
    let answers = source(&[atom("x")], observers);
    compare_answers(
        &answers,
        &[Answer {
            outputs: vec![],
            residual: vec![c("var", [atom("x")]), c("seen", [atom("x")])],
        }],
    );
    let a = Answer {
        outputs: vec![("out".into(), t("lam", [atom("x"), atom("x")]))],
        residual: vec![],
    };
    let b = Answer {
        outputs: vec![("out".into(), t("lam", [atom("y"), atom("y")]))],
        residual: vec![],
    };
    assert!(!chr_observe::equivalent(&a, &b, &mut Default::default()));
}
#[test]
fn compact_restrictions_do_not_replace_required_assignments() {
    let mut assignment_checks = 0;
    for n in 0usize..=3 {
        for k in 1u32..=6 {
            let obligations: Vec<_> = (0..k).map(|i| v(u64::from(i))).collect();
            let f = NameFormula::compile(Theory::AtomicNames, &obligations).unwrap();
            assert_eq!(f.variables().count(), k as usize);
            // Explicitly enumerate the declared finite alphabet; no source branch is invented.
            let mut valid = 0;
            for mut index in 0..n.pow(k) {
                let mut env = BTreeMap::new();
                for i in 0..k {
                    env.insert(Var(u64::from(i)), atom(&format!("name{}", index % n)));
                    index /= n;
                }
                assert!(f.refine(&env).unwrap().is_some());
                valid += 1;
            }
            assert_eq!(valid, n.pow(k));
            assignment_checks += valid;
        }
    }
    println!("explicit_assignment_checks={assignment_checks}");
}
