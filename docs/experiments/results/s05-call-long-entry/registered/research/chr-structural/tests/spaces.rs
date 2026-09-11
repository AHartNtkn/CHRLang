use chr_structural::{
    regular::{Automaton, Transition},
    space::{Mode, Pattern, Request, Space},
};
fn domain(leaf: &str) -> Automaton {
    let mut transitions = vec![Transition::new("a", [0, 0])];
    if leaf == "both" {
        transitions.extend([Transition::new("k", []), Transition::new("s", [])]);
    } else {
        transitions.push(Transition::new(leaf, []));
    }
    Automaton::new(vec![transitions], 0).unwrap()
}
#[test]
fn sharing_a_domain_is_not_equality_of_its_independent_holes() {
    for repeated in [false, true] {
        for mode in [Mode::FilterFirst, Mode::FilterSmallest, Mode::Intersect] {
            let request = Request {
                pattern: Pattern::App(
                    "a".into(),
                    vec![
                        Pattern::Hole(0),
                        Pattern::Hole(if repeated { 0 } else { 1 }),
                    ],
                ),
                domains: vec![vec![domain("both")]; if repeated { 1 } else { 2 }],
                equalities: vec![],
            };
            let mut space = Space::compile(request, 3, mode).unwrap();
            assert_eq!(space.solution_count(), Some(if repeated { 6 } else { 36 }));
            let b = space.advance(100);
            assert!(b.exhausted);
            assert_eq!(b.terms.len(), if repeated { 6 } else { 36 });
            if repeated {
                for t in b.terms {
                    let chr_syntax::Term::App(_, args) = t else {
                        panic!()
                    };
                    assert_eq!(args[0], args[1]);
                }
            }
        }
    }
}
#[test]
fn merging_holes_intersects_their_candidate_languages() {
    for mode in [Mode::FilterFirst, Mode::FilterSmallest, Mode::Intersect] {
        let request = Request {
            pattern: Pattern::App("a".into(), vec![Pattern::Hole(0), Pattern::Hole(1)]),
            domains: vec![vec![domain("k")], vec![domain("s")]],
            equalities: vec![(0, 1)],
        };
        let mut space = Space::compile(request, 7, mode).unwrap();
        assert_eq!(space.solution_count(), Some(0));
        assert!(space.advance(10).exhausted);
    }
}

#[test]
fn finite_skeletons_match_an_explicit_chr_generator() {
    use chr_syntax::{Goal, Query, Rule, Term, Var, and, atom, c, eq, or, t, v};
    fn term(p: &Pattern) -> Term {
        match p {
            Pattern::Hole(i) => v(10 + *i as u64),
            Pattern::App(n, args) => t(n, args.iter().map(term).collect::<Vec<_>>()),
        }
    }
    for restrictions in [["both", "both"], ["k", "both"], ["k", "s"], ["k", "k"]] {
        for equal in [false, true] {
            let pattern = Pattern::App(
                "f".into(),
                vec![
                    Pattern::App("a".into(), vec![Pattern::Hole(0), Pattern::Hole(1)]),
                    Pattern::Hole(0),
                ],
            );
            let mut goals = vec![eq(v(0), term(&pattern))];
            if equal {
                goals.push(eq(v(10), v(11)));
            }
            for (i, r) in restrictions.iter().enumerate() {
                goals.push(c("gen", [v(10 + i as u64)]).into());
                if *r != "both" {
                    goals.push(c(&format!("only-{r}"), [v(10 + i as u64)]).into());
                }
            }
            let mut rules = vec![
                Rule::simplify("request", [c("request", [v(0)])], and(goals)),
                Rule::simplify(
                    "gen",
                    [c("gen", [v(0)])],
                    or(c("gen1", [v(0)]).into(), c("gen3", [v(0)]).into()),
                ),
                Rule::simplify(
                    "gen1",
                    [c("gen1", [v(0)])],
                    or(eq(v(0), atom("k")), eq(v(0), atom("s"))),
                ),
                Rule::simplify(
                    "gen3",
                    [c("gen3", [v(0)])],
                    and([
                        eq(v(0), t("a", [v(1), v(2)])),
                        c("gen1", [v(1)]).into(),
                        c("gen1", [v(2)]).into(),
                    ]),
                ),
            ];
            for accepted in ["k", "s"] {
                let name = format!("only-{accepted}");
                for leaf in ["k", "s"] {
                    rules.push(Rule::simplify(
                        &format!("{name}-{leaf}"),
                        [c(&name, [atom(leaf)])],
                        if leaf == accepted {
                            Goal::True
                        } else {
                            Goal::Fail
                        },
                    ));
                }
                rules.push(Rule::simplify(
                    &format!("{name}-app"),
                    [c(&name, [t("a", [v(0), v(1)])])],
                    and([c(&name, [v(0)]).into(), c(&name, [v(1)]).into()]),
                ));
            }
            let mut reference = chr_reference::Search::new(
                rules,
                Query {
                    constraints: vec![c("request", [v(0)])],
                    outputs: vec![("out0".into(), Var(0))],
                },
            )
            .unwrap();
            let expected = reference.advance(100000);
            assert!(expected.exhausted);
            let expected = expected
                .answers
                .into_iter()
                .map(|a| a.outputs[0].1.clone())
                .collect::<std::collections::BTreeSet<_>>();
            for mode in [Mode::FilterFirst, Mode::FilterSmallest, Mode::Intersect] {
                let mut space = Space::compile(
                    Request {
                        pattern: pattern.clone(),
                        domains: restrictions.iter().map(|r| vec![domain(r)]).collect(),
                        equalities: if equal { vec![(0, 1)] } else { vec![] },
                    },
                    3,
                    mode,
                )
                .unwrap();
                let b = space.advance(1000);
                assert!(b.exhausted);
                assert_eq!(
                    b.terms
                        .into_iter()
                        .collect::<std::collections::BTreeSet<_>>(),
                    expected
                );
            }
        }
    }
}
