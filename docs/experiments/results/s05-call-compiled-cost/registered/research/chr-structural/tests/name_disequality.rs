use chr_structural::name_disequality::{Formula, Name};
use chr_syntax::{Answer, Goal, Query, Rule, atom, c, eq, v};
use std::collections::{BTreeMap, BTreeSet};
fn assignments(n: usize, alphabet: &[String]) -> Vec<Vec<String>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut out = vec![];
    for prefix in assignments(n - 1, alphabet) {
        for a in alphabet {
            let mut row = prefix.clone();
            row.push(a.clone());
            out.push(row);
        }
    }
    out
}
fn value<'a>(name: &'a Name, assignment: &'a [String]) -> &'a str {
    match name {
        Name::Variable(i) => &assignment[*i],
        Name::Atom(s) => s,
    }
}
fn valid(assignment: &[String], equal: &[(Name, Name)], unequal: &[(Name, Name)]) -> bool {
    equal
        .iter()
        .all(|(a, b)| value(a, assignment) == value(b, assignment))
        && unequal
            .iter()
            .all(|(a, b)| value(a, assignment) != value(b, assignment))
}
fn graph(bits: usize) -> Vec<(Name, Name)> {
    let mut pairs = vec![];
    let mut bit = 0;
    for i in 0..4 {
        for j in i + 1..4 {
            if bits & (1 << bit) != 0 {
                pairs.push((Name::Variable(i), Name::Variable(j)));
            }
            bit += 1;
        }
    }
    pairs
}
fn visible(mask: usize) -> Vec<usize> {
    (0..4).filter(|i| mask & (1 << i) != 0).collect()
}
#[test]
fn finite_and_unbounded_projection_match_independent_complete_assignments() {
    let mut finite = 0;
    let mut unbounded = 0;
    for bits in 0..64 {
        let unequal = graph(bits);
        let f = Formula::compile(4, &[], &unequal).unwrap();
        for size in 0..=3 {
            let alphabet: Vec<_> = (0..size).map(|i| format!("a{i}")).collect();
            let oracle: Vec<_> = assignments(4, &alphabet)
                .into_iter()
                .filter(|a| valid(a, &[], &unequal))
                .collect();
            for mask in 0..16 {
                let vars = visible(mask);
                let projected: BTreeSet<Vec<String>> = oracle
                    .iter()
                    .map(|row| vars.iter().map(|i| row[*i].clone()).collect())
                    .collect();
                for row in assignments(vars.len(), &alphabet) {
                    let given: BTreeMap<_, _> = vars.iter().copied().zip(row.clone()).collect();
                    assert_eq!(
                        f.finite(&alphabet, &given).satisfiable,
                        projected.contains(&row),
                        "graph={bits} mask={mask} alphabet={size}"
                    );
                    finite += 1;
                }
            }
        }
        let pool: Vec<_> = ["a", "b", "fresh0", "fresh1", "fresh2", "fresh3"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let oracle: Vec<_> = assignments(4, &pool)
            .into_iter()
            .filter(|a| valid(a, &[], &unequal))
            .collect();
        for mask in 0..16 {
            let vars = visible(mask);
            let projected: BTreeSet<Vec<String>> = oracle
                .iter()
                .map(|row| vars.iter().map(|i| row[*i].clone()).collect())
                .collect();
            for row in assignments(vars.len(), &pool[..2]) {
                let given = vars.iter().copied().zip(row.clone()).collect();
                assert_eq!(f.unbounded(&given), projected.contains(&row));
                unbounded += 1;
            }
        }
    }
    assert_eq!(finite, 22656);
    assert_eq!(unbounded, 5184);
    println!("finite_projection_checks={finite} unbounded_projection_checks={unbounded}");
}
fn syntax(name: &Name) -> chr_syntax::Term {
    match name {
        Name::Variable(i) => v(*i as u64),
        Name::Atom(s) => atom(s),
    }
}
#[test]
fn late_equalities_and_source_residuals_match_the_original_formula() {
    let endpoints = [
        Name::Variable(0),
        Name::Variable(1),
        Name::Variable(2),
        Name::Atom("a".into()),
        Name::Atom("b".into()),
    ];
    let alphabet = vec!["a".into(), "b".into()];
    let mut count = 0;
    for a in &endpoints {
        for b in &endpoints {
            for x in &endpoints {
                for y in &endpoints {
                    let equal = vec![(a.clone(), b.clone())];
                    let unequal = vec![(x.clone(), y.clone())];
                    let f = Formula::compile(3, &equal, &unequal);
                    for row in assignments(3, &alphabet) {
                        let wanted = valid(&row, &equal, &unequal);
                        let given = row.iter().cloned().enumerate().collect();
                        assert_eq!(
                            f.as_ref()
                                .is_some_and(|f| f.finite(&alphabet, &given).satisfiable),
                            wanted
                        );
                        assert_eq!(f.as_ref().is_some_and(|f| f.unbounded(&given)), wanted);
                        let mut body = vec![eq(syntax(a), syntax(b))];
                        body.extend(
                            row.iter()
                                .enumerate()
                                .map(|(i, s)| eq(v(i as u64), atom(s))),
                        );
                        let mut rules = chr_programs::lambda();
                        rules.push(Rule::simplify(
                            "bind",
                            [c("bind", [v(0), v(1), v(2)])],
                            Goal::And(body),
                        ));
                        let mut search = chr_reference::Search::new(
                            rules,
                            Query {
                                constraints: vec![
                                    c("neq", [syntax(x), syntax(y)]),
                                    c("bind", [v(0), v(1), v(2)]),
                                ],
                                outputs: vec![],
                            },
                        )
                        .unwrap();
                        let result = search.advance(2000);
                        assert!(result.exhausted);
                        let expected = if wanted {
                            vec![Answer {
                                outputs: vec![],
                                residual: vec![c(
                                    "neq",
                                    [atom(value(x, &row)), atom(value(y, &row))],
                                )],
                            }]
                        } else {
                            vec![]
                        };
                        assert_eq!(result.answers, expected);
                        count += 1;
                    }
                }
            }
        }
    }
    assert_eq!(count, 5000);
    println!("equality_and_source_checks={count}");
}
#[test]
fn hidden_names_change_finite_projection_and_late_aliases() {
    let x = Name::Variable(0);
    let y = Name::Variable(1);
    let hidden = Name::Variable(2);
    let f = Formula::compile(3, &[], &[(x.clone(), hidden.clone()), (y.clone(), hidden)]).unwrap();
    let alphabet = vec!["a".into(), "b".into()];
    for same in [false, true] {
        let given = BTreeMap::from([
            (0, "a".into()),
            (1, if same { "a".into() } else { "b".into() }),
        ]);
        assert_eq!(f.finite(&alphabet, &given).satisfiable, same);
        assert!(f.unbounded(&given));
    }
    let edge = vec![(x.clone(), y.clone()), (x.clone(), y.clone())];
    assert!(Formula::compile(3, &[(x.clone(), y.clone())], &edge).is_none());
    let pending = Formula::compile(2, &[(x.clone(), x)], &edge).unwrap();
    assert!(pending.unbounded(&BTreeMap::new()));
    assert!(!pending.unbounded(&BTreeMap::from([(0, "a".into()), (1, "a".into())])));
    // Caller transport is explicit substitution of variable indices, never fixed names.
    let moved = Formula::compile(
        12,
        &[],
        &[
            (Name::Variable(10), Name::Variable(11)),
            (Name::Variable(10), Name::Atom("a".into())),
        ],
    )
    .unwrap();
    assert!(!moved.unbounded(&BTreeMap::from([(10, "a".into())])));
    assert!(moved.unbounded(&BTreeMap::from([(10, "b".into()), (11, "a".into())])));
    for (args, success) in [(vec![v(0), v(1)], true), (vec![v(0), v(0)], false)] {
        let constraint = chr_syntax::Constraint {
            name: "neq".into(),
            args,
        };
        let mut s = chr_reference::Search::new(
            chr_programs::lambda(),
            Query {
                constraints: vec![constraint.clone()],
                outputs: vec![],
            },
        )
        .unwrap();
        let r = s.advance(2000);
        assert!(r.exhausted);
        assert_eq!(r.answers.len(), usize::from(success));
        if success {
            assert!(chr_observe::equivalent(
                &r.answers[0],
                &Answer {
                    outputs: vec![],
                    residual: vec![constraint]
                },
                &mut Default::default()
            ));
        }
    }
}
#[test]
fn feasibility_work_is_distinct_from_required_output() {
    for n in [3, 4] {
        let mut edges = vec![];
        for i in 0..n {
            for j in i + 1..n {
                edges.push((Name::Variable(i), Name::Variable(j)));
            }
        }
        let f = Formula::compile(n, &[], &edges).unwrap();
        for size in [2usize, 3] {
            let alphabet: Vec<_> = (0..size).map(|i| format!("a{i}")).collect();
            let r = f.finite(&alphabet, &BTreeMap::new());
            assert_eq!(r.satisfiable, size >= n);
            println!(
                "clique={n} alphabet={size} attempts={} full_assignments={}",
                r.assignment_attempts,
                size.pow(n as u32)
            );
        }
        assert!(f.unbounded(&BTreeMap::new()));
    }
    assert!(
        Formula::compile(0, &[(Name::Atom("a".into()), Name::Atom("b".into()))], &[]).is_none()
    );
}
