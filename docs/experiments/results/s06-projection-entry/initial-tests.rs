use chr_structural::projection::{Problem, Relation, Semantics};
use std::collections::BTreeMap;
fn full(p: &Problem) -> Vec<Vec<u8>> {
    let count = p.domains.iter().map(Vec::len).product::<usize>();
    let mut out = vec![];
    for mut index in 0..count {
        let mut row = vec![0; p.domains.len()];
        for i in (0..row.len()).rev() {
            row[i] = p.domains[i][index % p.domains[i].len()];
            index /= p.domains[i].len();
        }
        if p.filters.iter().all(|f| {
            f.rows
                .iter()
                .any(|r| f.scope.iter().enumerate().all(|(j, &i)| r[j] == row[i]))
        }) {
            out.push(row);
        }
    }
    out
}
fn expected(rows: &[Vec<u8>], visible: &[usize], semantics: Semantics) -> BTreeMap<Vec<u8>, u128> {
    let mut out = BTreeMap::new();
    for row in rows {
        let key = visible.iter().map(|&i| row[i]).collect();
        let count = out.entry(key).or_insert(0);
        if semantics == Semantics::Set {
            *count = 1
        } else {
            *count += 1
        }
    }
    out
}
fn check(p: &Problem, visible: &[usize]) {
    let rows = full(p);
    let hidden = (0..p.domains.len())
        .filter(|i| !visible.contains(i))
        .collect::<Vec<_>>();
    for semantics in [Semantics::Set, Semantics::Counted] {
        for order in [hidden.clone(), hidden.iter().rev().copied().collect()] {
            let projected = p.project(visible, &[], &order, semantics, 100_000).unwrap();
            assert_eq!(
                projected.answers(&[], 100_000).unwrap(),
                expected(&rows, visible, semantics)
            );
            for &v in visible {
                for value in 0..3 {
                    let want = rows
                        .iter()
                        .filter(|r| r[v] == value)
                        .cloned()
                        .collect::<Vec<_>>();
                    assert_eq!(
                        projected.answers(&[(v, value)], 100_000).unwrap(),
                        expected(&want, visible, semantics)
                    );
                }
            }
        }
    }
}
#[test]
fn hidden_witnesses_filters_and_visible_correlation() {
    let mut p = Problem {
        domains: vec![vec![0, 1], vec![0, 0, 1], vec![0, 1]],
        filters: vec![Relation {
            scope: vec![0, 1, 2],
            rows: vec![vec![0, 0, 1], vec![0, 0, 1], vec![1, 1, 0]],
        }],
    };
    check(&p, &[0, 2]);
    let q = p
        .project(&[0, 2], &[], &[1], Semantics::Counted, 100)
        .unwrap();
    assert_eq!(
        q.answers(&[], 100).unwrap(),
        BTreeMap::from([(vec![0, 1], 2), (vec![1, 0], 1)])
    );
    // Eliminating the correlation factor instead of its coordinate would admit four pairs.
    assert_eq!(
        p.project(&[0, 2], &[], &[1], Semantics::Set, 100)
            .unwrap()
            .answers(&[], 100)
            .unwrap()
            .len(),
        2
    );
    check(&p, &[]);
    p.domains[1].clear();
    check(&p, &[0, 2]);
    check(&p, &[]);
    check(
        &Problem {
            domains: vec![],
            filters: vec![],
        },
        &[],
    );
}
#[test]
fn shared_coordinates_and_later_hidden_constraints_are_explicit_boundaries() {
    let p = Problem {
        domains: vec![vec![0, 1], vec![0, 1]],
        filters: vec![],
    };
    assert!(p.project(&[0], &[1], &[1], Semantics::Set, 100).is_err());
    let q = p
        .project(&[0, 1], &[1], &[], Semantics::Counted, 100)
        .unwrap();
    assert_eq!(q.answers(&[(1, 1)], 100).unwrap().len(), 2);
    let hidden = p.project(&[0], &[], &[1], Semantics::Counted, 100).unwrap();
    assert!(hidden.answers(&[(1, 1)], 100).is_err());
    assert!(p.project(&[0], &[], &[], Semantics::Set, 100).is_err());
    assert!(p.project(&[0, 0], &[], &[1], Semantics::Set, 100).is_err());
    let bad = Problem {
        domains: p.domains.clone(),
        filters: vec![Relation {
            scope: vec![2],
            rows: vec![vec![0]],
        }],
    };
    assert!(bad.project(&[0], &[], &[1], Semantics::Set, 100).is_err());
    assert!(p.project(&[], &[], &[0, 1], Semantics::Set, 1).is_err());
}
#[test]
fn weighted_overflow_is_not_an_empty_answer() {
    let p = Problem {
        domains: vec![vec![0, 1]; 129],
        filters: vec![],
    };
    let order = (0..129).collect::<Vec<_>>();
    let counted = p
        .project(&[], &[], &order, Semantics::Counted, 100)
        .unwrap();
    assert!(counted.answers(&[], 100).unwrap_err().contains("overflow"));
    assert_eq!(
        p.project(&[], &[], &order, Semantics::Set, 100)
            .unwrap()
            .answers(&[], 100)
            .unwrap(),
        BTreeMap::from([(vec![], 1)])
    );
}
#[test]
fn generated_relations_match_independent_full_choice_enumeration() {
    let mut rng = 0x12345678_u64;
    let mut next = || {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        (rng >> 32) as usize
    };
    let mut checked = 0;
    for seed in 0..256 {
        let n = 3 + seed % 4;
        let domains = (0..n)
            .map(|i| {
                let length = if seed % 17 == 0 && i == 0 {
                    0
                } else {
                    2 + next() % 3
                };
                (0..length).map(|_| (next() % 3) as u8).collect()
            })
            .collect();
        let mut filters = vec![];
        for _ in 0..seed % 4 {
            let width = 1 + next() % 3;
            let offset = next() % n;
            let scope = (0..width).map(|j| (offset + j) % n).collect::<Vec<_>>();
            let mut rows = vec![];
            for mut code in 0..3usize.pow(width as u32) {
                let mut row = vec![];
                for _ in 0..width {
                    row.push((code % 3) as u8);
                    code /= 3;
                }
                if next() % 2 == 0 {
                    rows.push(row);
                }
            }
            filters.push(Relation { scope, rows });
        }
        let p = Problem { domains, filters };
        for offset in 0..8 {
            let mask = (seed + offset) % (1 << n);
            let visible = (0..n).filter(|i| mask & (1 << i) != 0).collect::<Vec<_>>();
            check(&p, &visible);
            checked += 1;
        }
    }
    assert_eq!(checked, 2048);
}
#[test]
fn eliminating_independent_hidden_coordinates_does_not_enumerate_their_product() {
    let p = Problem {
        domains: vec![vec![0, 1]; 24],
        filters: vec![],
    };
    let q = p
        .project(
            &[0],
            &[],
            &(1..24).collect::<Vec<_>>(),
            Semantics::Counted,
            100,
        )
        .unwrap();
    assert_eq!(
        q.answers(&[], 100).unwrap(),
        BTreeMap::from([(vec![0], 1 << 23), (vec![1], 1 << 23)])
    );
    if cfg!(feature = "metrics") {
        assert_eq!(q.elimination_visits, 46);
        assert_eq!(q.peak_entries, 2);
    }
}
#[test]
fn existing_structural_solver_preserves_the_same_projected_weights() {
    use chr_structural::finite::{Grammar, Request, Search, Transition};
    use chr_syntax::Term;
    let domains = vec![vec![0, 0, 1], vec![0, 1], vec![0, 1]];
    let p = Problem {
        domains: domains.clone(),
        filters: vec![Relation {
            scope: vec![1, 2],
            rows: vec![vec![0, 0], vec![1, 1]],
        }],
    };
    let mut states = domains
        .iter()
        .map(|d| {
            d.iter()
                .map(|v| Transition::new(if *v == 0 { "a" } else { "b" }, []))
                .collect()
        })
        .collect::<Vec<_>>();
    states.push(vec![Transition::new("tuple", [0, 1, 2])]);
    let grammar = Grammar::new(states).unwrap();
    let mut search = Search::new(
        &grammar,
        Request {
            roots: vec![3],
            equalities: vec![(vec![1], vec![2])],
        },
    )
    .unwrap();
    let batch = search.advance(100_000).unwrap();
    assert!(batch.exhausted);
    let mut observed = BTreeMap::new();
    for answer in batch.answers {
        let Term::App(_, args) = answer.term else {
            panic!("tuple")
        };
        let key = args[1..]
            .iter()
            .map(|t| match t {
                Term::App(n, _) if n == "a" => 0,
                Term::App(n, _) if n == "b" => 1,
                _ => panic!("atom"),
            })
            .collect::<Vec<_>>();
        *observed.entry(key).or_insert(0) += answer.multiplicity;
    }
    assert_eq!(
        p.project(&[1, 2], &[], &[0], Semantics::Counted, 100)
            .unwrap()
            .answers(&[], 100)
            .unwrap(),
        observed
    );
}
#[test]
fn actual_chr_hidden_choices_match_visible_sets_and_per_answer_multiplicity() {
    use chr_syntax::{Goal, Query, Rule, Var, atom, c, eq, or, v};
    let p = Problem {
        domains: vec![vec![0, 0, 1], vec![0, 1], vec![0, 1]],
        filters: vec![Relation {
            scope: vec![1, 2],
            rows: vec![vec![0, 0], vec![1, 1]],
        }],
    };
    let projected = p
        .project(&[1, 2], &[], &[0], Semantics::Counted, 100)
        .unwrap()
        .answers(&[], 100)
        .unwrap();
    for restriction in [None, Some(0), Some(1)] {
        let mut rules = vec![];
        let mut constraints = vec![];
        for (i, domain) in p.domains.iter().enumerate() {
            let name = format!("choose{i}");
            let mut choices = domain
                .iter()
                .map(|&x| eq(v(0), atom(if x == 0 { "a" } else { "b" })))
                .collect::<Vec<_>>();
            let mut body = choices.pop().unwrap_or(Goal::Fail);
            while let Some(left) = choices.pop() {
                body = or(left, body)
            }
            rules.push(Rule::simplify(&name, [c(&name, [v(0)])], body));
            constraints.push(c(&name, [v(i as u64)]));
        }
        let mut body = vec![eq(v(0), v(1))];
        if let Some(value) = restriction {
            body.push(eq(v(0), atom(if value == 0 { "a" } else { "b" })));
        }
        rules.push(Rule::simplify(
            "filter",
            [c("filter", [v(0), v(1)])],
            Goal::And(body),
        ));
        constraints.push(c("filter", [v(1), v(2)]));
        let mut search = chr_reference::Search::new(
            rules,
            Query {
                constraints,
                outputs: vec![("x".into(), Var(1)), ("y".into(), Var(2))],
            },
        )
        .unwrap();
        let batch = search.advance(100_000);
        assert!(batch.exhausted);
        let expected = projected
            .iter()
            .filter(|(k, _)| restriction.is_none_or(|x| k[0] == x))
            .collect::<Vec<_>>();
        assert_eq!(
            search.stats().completed_branches as u128,
            expected.iter().map(|(_, n)| **n).sum::<u128>()
        );
        assert_eq!(batch.answers.len(), expected.len());
        for answer in batch.answers {
            assert!(answer.residual.is_empty());
            let key = answer
                .outputs
                .iter()
                .map(|(_, t)| {
                    if *t == atom("a") {
                        0
                    } else {
                        assert_eq!(*t, atom("b"));
                        1
                    }
                })
                .collect::<Vec<_>>();
            assert!(expected.iter().any(|(k, _)| ***k == key));
        }
    }
}

#[test]
fn connected_projection_order_changes_intermediate_width_not_denotation() {
    let p = Problem {
        domains: vec![vec![0, 1]; 8],
        filters: (1..8)
            .map(|i| Relation {
                scope: vec![0, i],
                rows: vec![vec![0, 0], vec![0, 1], vec![1, 0]],
            })
            .collect(),
    };
    let first = p
        .project(
            &[6, 7],
            &[],
            &[0, 1, 2, 3, 4, 5],
            Semantics::Counted,
            100_000,
        )
        .unwrap();
    let last = p
        .project(
            &[6, 7],
            &[],
            &[1, 2, 3, 4, 5, 0],
            Semantics::Counted,
            100_000,
        )
        .unwrap();
    let want = BTreeMap::from([
        (vec![0, 0], 33),
        (vec![0, 1], 32),
        (vec![1, 0], 32),
        (vec![1, 1], 32),
    ]);
    assert_eq!(first.answers(&[], 100).unwrap(), want);
    assert_eq!(last.answers(&[], 100).unwrap(), want);
    assert_eq!(expected(&full(&p), &[6, 7], Semantics::Counted), want);
    if cfg!(feature = "metrics") {
        assert_eq!((first.elimination_visits, first.peak_entries), (504, 128));
        assert_eq!((last.elimination_visits, last.peak_entries), (28, 4));
    }
}
