use chr_structural::finite::{Grammar, Request, Search, Transition};
use chr_syntax::{atom, t};

fn grammar() -> Grammar {
    Grammar::new(vec![
        vec![Transition::new("a", []), Transition::new("b", [])],
        vec![Transition::new("pair", [0, 0])],
    ])
    .unwrap()
}
#[test]
fn path_equality_correlates_occurrences_without_losing_source_multiplicity() {
    let g = grammar();
    let request = Request {
        roots: vec![1],
        equalities: vec![(vec![0], vec![1])],
    };
    let mut search = Search::new(&g, request).unwrap();
    let result = search.advance(100).unwrap();
    assert!(result.exhausted);
    let actual = result
        .answers
        .into_iter()
        .map(|a| (a.term, a.multiplicity))
        .collect::<Vec<_>>();
    assert_eq!(
        actual,
        vec![
            (t("pair", [atom("a"), atom("a")]), 1),
            (t("pair", [atom("b"), atom("b")]), 1)
        ]
    );
}

fn collect(
    g: &Grammar,
    roots: Vec<usize>,
    equalities: Vec<(Vec<usize>, Vec<usize>)>,
) -> Vec<(chr_syntax::Term, u128)> {
    let run = |g: &Grammar| {
        let mut search = Search::new(
            g,
            Request {
                roots: roots.clone(),
                equalities: equalities.clone(),
            },
        )
        .unwrap();
        let batch = search.advance(100_000).unwrap();
        assert!(batch.exhausted);
        let mut values = batch
            .answers
            .into_iter()
            .map(|a| (a.term, a.multiplicity))
            .collect::<Vec<_>>();
        values.sort();
        values
    };
    let original = run(g);
    let reduced = run(&g.clone().reduce_membership());
    assert_eq!(
        original, reduced,
        "membership reduction changes value/count"
    );
    original
}
#[test]
fn source_duplicates_count_but_overlapping_filter_proofs_do_not() {
    let g = Grammar::new(vec![
        vec![
            Transition::new("a", []),
            Transition::new("a", []),
            Transition::new("b", []),
        ],
        vec![Transition::new("pair", [0, 0])],
        vec![Transition::new("a", []), Transition::new("a", [])],
        vec![
            Transition::new("pair", [2, 2]),
            Transition::new("pair", [2, 2]),
        ],
    ])
    .unwrap();
    assert_eq!(
        collect(&g, vec![1, 3], vec![]),
        vec![(t("pair", [atom("a"), atom("a")]), 4)]
    );
    assert_eq!(
        collect(&g, vec![1], vec![])
            .iter()
            .map(|(_, n)| n)
            .sum::<u128>(),
        9
    );
    assert_eq!(
        collect(&g, vec![1], vec![(vec![0], vec![1])])
            .iter()
            .map(|(_, n)| n)
            .sum::<u128>(),
        5
    );
}
#[test]
fn absent_paths_and_proper_subtree_equality_have_no_finite_witness() {
    let g = grammar();
    for equalities in [
        vec![(vec![2], vec![2])],
        vec![(vec![], vec![0])],
        vec![(vec![0, 0], vec![1])],
    ] {
        assert!(collect(&g, vec![1], equalities).is_empty());
    }
    assert_eq!(collect(&g, vec![1], vec![]).len(), 4);
}
#[test]
fn nested_and_transitive_path_equalities_preserve_all_solutions() {
    let g = Grammar::new(vec![
        vec![Transition::new("a", []), Transition::new("b", [])],
        vec![Transition::new("f", [0])],
        vec![Transition::new("triple", [1, 1, 0])],
    ])
    .unwrap();
    assert_eq!(
        collect(&g, vec![2], vec![(vec![0], vec![1]), (vec![1, 0], vec![2])]),
        vec![
            (
                t(
                    "triple",
                    [t("f", [atom("a")]), t("f", [atom("a")]), atom("a")]
                ),
                1
            ),
            (
                t(
                    "triple",
                    [t("f", [atom("b")]), t("f", [atom("b")]), atom("b")]
                ),
                1
            )
        ]
    );
}
#[test]
fn service_budget_is_resumable_and_zero_is_not_exhaustion() {
    let g = grammar();
    let mut search = Search::new(
        &g,
        Request {
            roots: vec![1],
            equalities: vec![],
        },
    )
    .unwrap();
    let zero = search.advance(0).unwrap();
    assert!(!zero.exhausted);
    assert!(zero.answers.is_empty());
    let mut seen = vec![];
    let mut done = false;
    for _ in 0..100 {
        let b = search.advance(1).unwrap();
        seen.extend(b.answers.into_iter().map(|a| (a.term, a.multiplicity)));
        if b.exhausted {
            done = true;
            break;
        }
    }
    assert!(done);
    seen.sort();
    assert_eq!(seen, collect(&g, vec![1], vec![]));
    assert!(search.advance(1).unwrap().exhausted);
}
#[test]
fn invalid_grammar_and_request_are_errors_not_empty_languages() {
    assert!(Grammar::new(vec![vec![Transition::new("f", [0])]]).is_err());
    assert!(Grammar::new(vec![vec![Transition::new("f", [1])]]).is_err());
    let g = grammar();
    assert!(
        Search::new(
            &g,
            Request {
                roots: vec![],
                equalities: vec![]
            }
        )
        .is_err()
    );
    assert!(
        Search::new(
            &g,
            Request {
                roots: vec![2],
                equalities: vec![]
            }
        )
        .is_err()
    );
}

// This control constructs all source derivations as owned trees. It does not
// share the solver's position graph, intersection propagation or counting code.
fn enumerate(g: &Grammar, state: usize) -> Vec<chr_syntax::Term> {
    let mut out = vec![];
    for transition in &g.states()[state] {
        let mut tuples = vec![vec![]];
        for &child in &transition.children {
            let values = enumerate(g, child);
            tuples = tuples
                .into_iter()
                .flat_map(|prefix| {
                    values.iter().map(move |value| {
                        let mut args = prefix.clone();
                        args.push(value.clone());
                        args
                    })
                })
                .collect();
        }
        out.extend(tuples.into_iter().map(|args| t(&transition.symbol, args)));
    }
    out
}
fn at<'a>(term: &'a chr_syntax::Term, path: &[usize]) -> Option<&'a chr_syntax::Term> {
    let mut term = term;
    for &index in path {
        let chr_syntax::Term::App(_, args) = term else {
            return None;
        };
        term = args.get(index)?;
    }
    Some(term)
}
fn exhaustive(
    g: &Grammar,
    roots: &[usize],
    equalities: &[(Vec<usize>, Vec<usize>)],
) -> Vec<(chr_syntax::Term, u128)> {
    let filters = roots[1..]
        .iter()
        .map(|&r| {
            enumerate(g, r)
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>()
        })
        .collect::<Vec<_>>();
    let mut counts = std::collections::BTreeMap::new();
    for term in enumerate(g, roots[0]) {
        if filters.iter().all(|f| f.contains(&term))
            && equalities
                .iter()
                .all(|(a, b)| match (at(&term, a), at(&term, b)) {
                    (Some(x), Some(y)) => x == y,
                    _ => false,
                })
        {
            *counts.entry(term).or_insert(0) += 1;
        }
    }
    counts.into_iter().collect()
}
#[test]
fn exhaustive_7840_requests_match_independent_enumeration() {
    let paths = [
        vec![],
        vec![0],
        vec![1],
        vec![0, 0],
        vec![0, 1],
        vec![1, 0],
        vec![2],
    ];
    let mut checked = 0;
    for leaves in [vec![], vec!["a"], vec!["a", "b"], vec!["a", "a", "b"]] {
        for mask in 0..8 {
            let mut middle = vec![];
            if mask & 1 != 0 {
                middle.push(Transition::new("a", []));
            }
            if mask & 2 != 0 {
                middle.push(Transition::new("f", [0]));
            }
            if mask & 4 != 0 {
                middle.push(Transition::new("pair", [0, 0]));
            }
            let g = Grammar::new(vec![
                leaves.iter().map(|a| Transition::new(a, [])).collect(),
                middle,
                vec![Transition::new("pair", [1, 0]), Transition::new("f", [1])],
            ])
            .unwrap();
            for roots in [vec![1], vec![1, 1], vec![1, 2], vec![2], vec![2, 1]] {
                for a in &paths {
                    for b in &paths {
                        let eqs = vec![(a.clone(), b.clone())];
                        assert_eq!(
                            collect(&g, roots.clone(), eqs.clone()),
                            exhaustive(&g, &roots, &eqs),
                            "mask={mask} roots={roots:?} paths={eqs:?}"
                        );
                        checked += 1;
                    }
                }
            }
        }
    }
    assert_eq!(checked, 7840);
}
#[test]
fn equality_selects_two_values_from_a_64_bit_description() {
    let mut states = vec![
        vec![Transition::new("a", []), Transition::new("b", [])],
        vec![Transition::new("nil", [])],
    ];
    for i in 0..64 {
        states.push(vec![Transition::new("cons", [0, i + 1])]);
    }
    let g = Grammar::new(states).unwrap();
    let eqs = (1..64)
        .map(|i| {
            let mut p = vec![1; i];
            p.push(0);
            (vec![0], p)
        })
        .collect();
    let mut search = Search::new(
        &g,
        Request {
            roots: vec![65],
            equalities: eqs,
        },
    )
    .unwrap();
    let b = search.advance(10_000).unwrap();
    assert!(b.exhausted);
    assert_eq!(b.answers.len(), 2);
    for answer in b.answers {
        assert_eq!(answer.multiplicity, 1);
        for i in 1..64 {
            let mut p = vec![1; i];
            p.push(0);
            assert_eq!(at(&answer.term, &[0]), at(&answer.term, &p));
        }
    }
    assert!(search.stats().requirements < 300);
    println!(
        "64-bit selective requirements={} trials={} duplicates={} peak_frontier={}",
        search.stats().requirements,
        search.stats().transition_trials,
        search.stats().duplicate_values,
        search.stats().peak_frontier
    );
    // The same description without equalities must still emit every value.
    let mut states = vec![
        vec![Transition::new("a", []), Transition::new("b", [])],
        vec![Transition::new("nil", [])],
    ];
    for i in 0..8 {
        states.push(vec![Transition::new("cons", [0, i + 1])]);
    }
    let g = Grammar::new(states).unwrap();
    assert_eq!(collect(&g, vec![9], vec![]), exhaustive(&g, &[9], &[]));
    assert_eq!(collect(&g, vec![9], vec![]).len(), 256);
}

#[test]
fn impossible_derivations_do_not_overflow_and_real_overflow_is_sticky() {
    let mut states = vec![
        vec![Transition::new("a", []), Transition::new("a", [])],
        vec![Transition::new("a", [])],
        vec![Transition::new("b", [])],
    ];
    let (mut duplicate, mut unique) = (0, 1);
    for _ in 0..7 {
        let d = states.len();
        states.push(vec![Transition::new("f", [duplicate, duplicate])]);
        duplicate = d;
        let u = states.len();
        states.push(vec![Transition::new("f", [unique, unique])]);
        unique = u;
    }
    let source = states.len();
    states.push(vec![
        Transition::new("pair", [duplicate, 2]),
        Transition::new("pair", [unique, 1]),
    ]);
    let filter = states.len();
    states.push(vec![Transition::new("pair", [unique, 1])]);
    let g = Grammar::new(states).unwrap();
    let answers = collect(&g, vec![source, filter], vec![]);
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0].1, 1);
    let mut search = Search::new(
        &g,
        Request {
            roots: vec![duplicate],
            equalities: vec![],
        },
    )
    .unwrap();
    let e = search.advance(100_000).unwrap_err();
    assert!(e.contains("overflow"));
    assert_eq!(search.advance(1).unwrap_err(), e);
}

#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;

fn source(
    g: &Grammar,
    root: usize,
    equalities: &[(Vec<usize>, Vec<usize>)],
) -> (Vec<chr_syntax::Rule>, chr_syntax::Query) {
    use chr_syntax::{Goal, Query, Rule, Var, and, c, eq, or, v};
    fn path(p: &[usize]) -> chr_syntax::Term {
        p.iter().rev().fold(atom("nil"), |tail, i| {
            t("cons", [atom(&i.to_string()), tail])
        })
    }
    let mut rules = vec![];
    for (id, transitions) in g.states().iter().enumerate() {
        let alternatives = transitions
            .iter()
            .map(|tr| {
                let vars = (0..tr.children.len())
                    .map(|i| v(1 + i as u64))
                    .collect::<Vec<_>>();
                and(std::iter::once(eq(v(0), t(&tr.symbol, vars.clone())))
                    .chain(
                        tr.children
                            .iter()
                            .zip(vars)
                            .map(|(s, arg)| c(&format!("gen{s}"), [arg]).into()),
                    )
                    .collect::<Vec<_>>())
            })
            .collect::<Vec<Goal>>();
        let body = alternatives.into_iter().reduce(or).unwrap_or(Goal::Fail);
        rules.push(Rule::simplify(
            &format!("gen{id}"),
            [c(&format!("gen{id}"), [v(0)])],
            body,
        ));
    }
    rules.push(Rule::simplify(
        "walk-nil",
        [c("walk", [atom("nil"), v(0), v(1)])],
        eq(v(0), v(1)),
    ));
    let signatures = g
        .states()
        .iter()
        .flatten()
        .map(|tr| (tr.symbol.clone(), tr.children.len()))
        .collect::<std::collections::BTreeSet<_>>();
    let indices = equalities
        .iter()
        .flat_map(|(a, b)| a.iter().chain(b))
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    for (symbol, arity) in signatures {
        for &index in &indices {
            let vars = (0..arity).map(|i| v(10 + i as u64)).collect::<Vec<_>>();
            rules.push(Rule::simplify(
                &format!("walk-{symbol}-{arity}-{index}"),
                [c(
                    "walk",
                    [
                        t("cons", [atom(&index.to_string()), v(0)]),
                        t(&symbol, vars.clone()),
                        v(1),
                    ],
                )],
                if index < arity {
                    c("walk", [v(0), vars[index].clone(), v(1)]).into()
                } else {
                    Goal::Fail
                },
            ));
        }
    }
    let mut goals = vec![c(&format!("gen{root}"), [v(0)]).into()];
    for (i, (a, b)) in equalities.iter().enumerate() {
        // Both walks bind the same endpoint variable: unequal ground subtrees fail.
        let result = v(i as u64 + 1);
        goals.push(c("walk", [path(a), v(0), result.clone()]).into());
        goals.push(c("walk", [path(b), v(0), result]).into());
    }
    rules.push(Rule::simplify(
        "request",
        [c("request", [v(0)])],
        and(goals),
    ));
    (
        rules,
        Query {
            constraints: vec![c("request", [v(0)]), c("marker", [atom("caller")])],
            outputs: vec![("out".into(), Var(0)), ("again".into(), Var(0))],
        },
    )
}
#[test]
fn finite_path_values_and_counts_match_160_actual_chr_sources() {
    use chr_syntax::{Answer, c};
    let mut checked = 0;
    for leaves in [vec![], vec!["a"], vec!["a", "b"], vec!["a", "a", "b"]] {
        for mask in [0, 3, 5, 7] {
            let mut middle = vec![];
            if mask & 1 != 0 {
                middle.push(Transition::new("a", []));
            }
            if mask & 2 != 0 {
                middle.push(Transition::new("f", [0]));
            }
            if mask & 4 != 0 {
                middle.push(Transition::new("pair", [0, 0]));
            }
            let g = Grammar::new(vec![
                leaves.iter().map(|a| Transition::new(a, [])).collect(),
                middle,
                vec![Transition::new("pair", [1, 0]), Transition::new("f", [1])],
            ])
            .unwrap();
            for root in [1, 2] {
                for eqs in [
                    vec![],
                    vec![(vec![0], vec![1])],
                    vec![(vec![0, 0], vec![1])],
                    vec![(vec![], vec![0])],
                    vec![(vec![2], vec![2])],
                ] {
                    let answers = collect(&g, vec![root], eqs.clone());
                    let raw = answers
                        .iter()
                        .flat_map(|(term, n)| {
                            std::iter::repeat_n(
                                Answer {
                                    outputs: vec![
                                        ("out".into(), term.clone()),
                                        ("again".into(), term.clone()),
                                    ],
                                    residual: vec![c("marker", [atom("caller")])],
                                },
                                *n as usize,
                            )
                        })
                        .collect::<Vec<_>>();
                    let (rules, query) = source(&g, root, &eqs);
                    scalar::same_raw(raw.clone(), scalar::run(&rules, &query, 100_000));
                    let mut reference = chr_reference::Search::new(rules, query).unwrap();
                    let batch = reference.advance(100_000);
                    assert!(batch.exhausted);
                    assert_eq!(reference.stats().completed_branches as usize, raw.len());
                    let distinct = raw.into_iter().collect::<std::collections::BTreeSet<_>>();
                    assert_eq!(
                        batch
                            .answers
                            .into_iter()
                            .collect::<std::collections::BTreeSet<_>>(),
                        distinct
                    );
                    checked += 1;
                }
            }
        }
    }
    assert_eq!(checked, 160);
}

#[test]
fn reducing_membership_proofs_preserves_source_counts() {
    let mut states = vec![
        vec![Transition::new("a", [])],
        vec![Transition::new("a", []), Transition::new("a", [])],
        vec![Transition::new("nil", [])],
    ];
    let (mut source, mut filter) = (2, 2);
    for _ in 0..8 {
        let id = states.len();
        states.push(vec![Transition::new("cons", [0, source])]);
        source = id;
        let id = states.len();
        states.push(vec![Transition::new("cons", [1, filter])]);
        filter = id;
    }
    let original = Grammar::new(states).unwrap();
    let reduced = original.clone().reduce_membership();
    let run = |g: &Grammar, roots| {
        let mut s = Search::new(
            g,
            Request {
                roots,
                equalities: vec![],
            },
        )
        .unwrap();
        let batch = s.advance(100_000).unwrap();
        assert!(batch.exhausted);
        (
            batch
                .answers
                .into_iter()
                .map(|a| (a.term, a.multiplicity))
                .collect::<Vec<_>>(),
            s.stats().duplicate_values,
            s.stats().requirements,
        )
    };
    let (a, duplicates, before) = run(&original, vec![source, filter]);
    let (b, reduced_duplicates, after) = run(&reduced, vec![source, filter]);
    assert_eq!(a, b);
    assert_eq!(a.len(), 1);
    assert_eq!(a[0].1, 1);
    if cfg!(feature = "metrics") {
        assert_eq!(duplicates, 255);
    }
    assert_eq!(reduced_duplicates, 0);
    if cfg!(feature = "metrics") {
        assert!(after < before);
    }
    // Swapping the first state changes source derivations, not membership.
    let (b, _, _) = run(&reduced, vec![filter, source]);
    assert_eq!(b[0].1, 256);
    println!(
        "redundant filter requirements={before}->{after}; duplicates={duplicates}->{reduced_duplicates}"
    );
}

#[test]
fn cancellation_preserves_reusable_grammar_and_returned_values_are_owned() {
    let g = grammar();
    let mut cancelled = Search::new(
        &g,
        Request {
            roots: vec![1],
            equalities: vec![],
        },
    )
    .unwrap();
    assert!(!cancelled.advance(1).unwrap().exhausted);
    drop(cancelled);
    let mut next = Search::new(
        &g,
        Request {
            roots: vec![1],
            equalities: vec![],
        },
    )
    .unwrap();
    let mut held = None;
    for _ in 0..100 {
        let mut batch = next.advance(1).unwrap();
        if let Some(a) = batch.answers.pop() {
            held = Some(a);
            break;
        }
    }
    drop(next);
    drop(g);
    let held = held.unwrap();
    assert_eq!(held.term, t("pair", [atom("a"), atom("a")]));
    assert_eq!(held.multiplicity, 1);
}

#[test]
fn merging_existing_nested_paths_is_independent_of_equality_order() {
    let g = Grammar::new(vec![
        vec![Transition::new("a", []), Transition::new("b", [])],
        vec![Transition::new("pair", [0, 0])],
        vec![Transition::new("pair", [1, 1])],
    ])
    .unwrap();
    let eqs = vec![
        (vec![0, 0], vec![1, 1]),
        (vec![0], vec![1]),
        (vec![0, 1], vec![1, 0]),
    ];
    let expected = exhaustive(&g, &[2], &eqs);
    assert_eq!(expected.len(), 2);
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let constraints = order.into_iter().map(|i| eqs[i].clone()).collect();
        assert_eq!(collect(&g, vec![2], constraints), expected);
    }
}

#[test]
fn feature_off_preserves_answers_without_collecting_solver_counts() {
    let g = grammar();
    let mut search = Search::new(
        &g,
        Request {
            roots: vec![1],
            equalities: vec![],
        },
    )
    .unwrap();
    let b = search.advance(1000).unwrap();
    assert!(b.exhausted);
    assert_eq!(b.answers.len(), 4);
    assert_eq!(
        chr_structural::finite::COLLECT_METRICS,
        cfg!(feature = "metrics")
    );
    if !cfg!(feature = "metrics") {
        let s = search.stats();
        assert_eq!(
            (
                s.requirements,
                s.transition_trials,
                s.duplicate_values,
                s.emitted,
                s.peak_frontier
            ),
            (0, 0, 0, 0, 0)
        );
    }
}
