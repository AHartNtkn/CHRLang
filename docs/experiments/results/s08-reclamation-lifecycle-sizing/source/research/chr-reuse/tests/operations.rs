use chr_reuse::{EquationTable, Mode};
use chr_syntax::{Var, atom, t, v};

#[test]
fn renamed_hits_preserve_caller_variables_and_aliases() {
    let mut table = EquationTable::new(Mode::Memo);
    let a = table.solve(&t("p", [v(10), v(11)]), &t("p", [v(11), atom("z")]));
    assert_eq!(
        a.unwrap(),
        [(Var(10), atom("z")), (Var(11), atom("z"))].into()
    );
    let b = table.solve(&t("p", [v(90), v(70)]), &t("p", [v(70), atom("z")]));
    assert_eq!(
        b.unwrap(),
        [(Var(70), atom("z")), (Var(90), atom("z"))].into()
    );
    assert_eq!(table.stats().hits, 1);
    assert_eq!(table.stats().computed, 1);
}

#[test]
fn repeated_holes_and_stronger_callers_have_distinct_keys() {
    let mut table = EquationTable::new(Mode::Memo);
    assert!(
        table
            .solve(&t("p", [v(0), v(1)]), &t("p", [atom("a"), atom("b")]))
            .is_some()
    );
    assert!(
        table
            .solve(&t("p", [v(0), v(0)]), &t("p", [atom("a"), atom("b")]))
            .is_none()
    );
    assert!(
        table
            .solve(&t("p", [atom("c"), v(1)]), &t("p", [atom("a"), atom("b")]))
            .is_none()
    );
    assert_eq!(table.stats().hits, 0);
}

#[test]
fn cached_failure_is_local_to_the_equation_and_freshening_is_exact() {
    let mut table = EquationTable::new(Mode::Memo);
    assert!(table.solve(&v(0), &t("f", [v(0)])).is_none());
    assert!(table.solve(&v(99), &t("f", [v(99)])).is_none());
    assert_eq!(
        table.solve(&v(99), &v(98)).unwrap(),
        [(Var(98), v(98)), (Var(99), v(98))].into()
    );
    assert_eq!(table.stats().hits, 1);
}

#[test]
fn a_later_pair_detects_a_cycle_through_an_earlier_alias() {
    let mut table = EquationTable::new(Mode::Memo);
    for ids in [[0, 1], [40, 50]] {
        assert!(
            table
                .solve(
                    &t("p", [v(ids[0]), v(ids[1])]),
                    &t("p", [v(ids[1]), t("f", [v(ids[0])])]),
                )
                .is_none()
        );
    }
    assert_eq!(table.stats().hits, 1);
}

#[test]
fn exhaustive_finite_pairs_match_independent_reference_observations() {
    use chr_reference::Search;
    use chr_syntax::{Answer, Query, Rule, Term, c, eq};
    fn rename(t: &Term, ids: [u64; 2]) -> Term {
        match t {
            Term::Var(w) => v(ids[w.0 as usize]),
            Term::App(n, args) => {
                Term::App(n.clone(), args.iter().map(|t| rename(t, ids)).collect())
            }
        }
    }
    let base = vec![v(0), v(1), atom("a"), atom("b")];
    let mut terms = base.clone();
    terms.extend(base.iter().map(|x| t("f", [x.clone()])));
    for a in &base {
        for b in &base {
            terms.push(t("p", [a.clone(), b.clone()]));
        }
    }
    let mut table = EquationTable::new(Mode::Memo);
    let mut control = EquationTable::new(Mode::Direct);
    for a in &terms {
        for b in &terms {
            let mut reference = Search::new(
                vec![Rule::simplify(
                    "equation",
                    [c("run", [v(10), v(11)])],
                    eq(v(10), v(11)),
                )],
                Query {
                    constraints: vec![c("run", [a.clone(), b.clone()])],
                    outputs: vec![("x".into(), Var(0)), ("y".into(), Var(1))],
                },
            )
            .unwrap();
            let expected = reference.advance(100);
            assert!(expected.exhausted);
            for ids in [[0, 1], [90, 70]] {
                let left = rename(a, ids);
                let right = rename(b, ids);
                let direct = control.solve(&left, &right);
                let cached = table.solve(&left, &right);
                assert_eq!(direct, cached);
                match cached {
                    None => assert!(expected.answers.is_empty()),
                    Some(sub) => {
                        let actual = Answer {
                            outputs: vec![
                                (
                                    "x".into(),
                                    sub.get(&Var(ids[0])).cloned().unwrap_or_else(|| v(ids[0])),
                                ),
                                (
                                    "y".into(),
                                    sub.get(&Var(ids[1])).cloned().unwrap_or_else(|| v(ids[1])),
                                ),
                            ],
                            residual: vec![],
                        };
                        assert_eq!(expected.answers.len(), 1);
                        assert!(
                            chr_observe::equivalent(
                                &actual,
                                &expected.answers[0],
                                &mut chr_observe::Stats::default()
                            ),
                            "{left:?} = {right:?}"
                        );
                    }
                }
            }
        }
    }
    assert!(table.stats().hits >= 576);
}
