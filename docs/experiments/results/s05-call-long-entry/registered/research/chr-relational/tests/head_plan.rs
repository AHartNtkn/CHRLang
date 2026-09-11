use chr_relational::{HeadPlan, Match, Occurrence, Relation, Value, View};
use chr_syntax::{Constraint, Term, Var, atom, c, t, v};
use std::collections::{BTreeMap, BTreeSet};

fn import(term: &Term, view: &mut View, values: &mut BTreeMap<Term, Value>) -> Value {
    if let Some(value) = values.get(term) {
        return *value;
    }
    let children = match term {
        Term::App(_, xs) => xs
            .iter()
            .map(|x| import(x, view, values))
            .collect::<Vec<_>>(),
        _ => vec![],
    };
    let value = Value(values.len());
    values.insert(term.clone(), value);
    if let Term::App(name, _) = term {
        view.constructor(name, value, &children);
    }
    value
}
fn view(store: &[Constraint]) -> (View, BTreeMap<Term, Value>) {
    let mut view = View::default();
    let mut values = BTreeMap::new();
    for (id, c) in store.iter().enumerate() {
        let args = c
            .args
            .iter()
            .map(|x| import(x, &mut view, &mut values))
            .collect::<Vec<_>>();
        view.occurrence(Occurrence(id), &c.name, &args);
    }
    (view, values)
}
// Exhaustive owned syntax matching, with no relational operators or candidate plan.
type ScalarMatches = BTreeMap<Vec<usize>, BTreeMap<Var, Term>>;
fn scalar(heads: &[Constraint], store: &[Constraint]) -> ScalarMatches {
    fn term(p: &Term, x: &Term, env: &mut BTreeMap<Var, Term>) -> bool {
        match p {
            Term::Var(v) => match env.get(v) {
                Some(old) => old == x,
                None => {
                    env.insert(*v, x.clone());
                    true
                }
            },
            Term::App(n, ps) => {
                matches!(x,Term::App(m,xs) if n==m && ps.len()==xs.len() && ps.iter().zip(xs).all(|(p,x)|term(p,x,env)))
            }
        }
    }
    fn go(
        heads: &[Constraint],
        store: &[Constraint],
        ids: Vec<usize>,
        env: BTreeMap<Var, Term>,
        out: &mut ScalarMatches,
    ) {
        if ids.len() == heads.len() {
            out.insert(ids, env);
            return;
        }
        let head = &heads[ids.len()];
        for (id, c) in store.iter().enumerate() {
            if ids.contains(&id) || head.name != c.name || head.args.len() != c.args.len() {
                continue;
            }
            let mut next = env.clone();
            if head
                .args
                .iter()
                .zip(&c.args)
                .all(|(p, x)| term(p, x, &mut next))
            {
                let mut chosen = ids.clone();
                chosen.push(id);
                go(heads, store, chosen, next, out);
            }
        }
    }
    let mut out = BTreeMap::new();
    go(heads, store, vec![], BTreeMap::new(), &mut out);
    out
}
#[test]
fn selective_constructor_can_drive_the_source_join() {
    let store = (0..20)
        .map(|i| {
            c(
                "open",
                [t("f", [atom(if i == 7 { "a" } else { "b" }), v(i)])],
            )
        })
        .collect::<Vec<_>>();
    let (view, values) = view(&store);
    let plan = HeadPlan::compile(&[], &[c("open", [t("f", [atom("a"), v(100)])])]);
    let result = plan.evaluate(&view);
    assert_eq!(
        result.first_relation,
        Some(Relation::Constructor("a".into(), 0))
    );
    assert_eq!(
        result.matches,
        vec![Match {
            kept: vec![],
            removed: vec![Occurrence(7)],
            bindings: BTreeMap::from([(Var(100), values[&v(7)])])
        }]
    );
}
#[test]
fn plans_match_exhaustive_heads_across_unknown_and_equal_value_snapshots() {
    let patterns = [
        vec![c("p", [v(0)])],
        vec![c("p", [t("f", [v(0)])]), c("q", [v(0)])],
        vec![c("p", [t("pair", [v(0), v(0)])])],
        vec![c("p", [v(0)]), c("p", [v(0)])],
        vec![c("p", [t("f", [atom("a")])]), c("q", [atom("a")])],
    ];
    let terms = [
        v(20),
        v(21),
        atom("a"),
        atom("b"),
        t("f", [atom("a")]),
        t("f", [v(20)]),
        t("pair", [v(20), v(20)]),
        t("pair", [v(20), v(21)]),
    ];
    for left in &terms {
        for right in &terms {
            for reverse in [false, true] {
                let mut store = vec![
                    c("p", [left.clone()]),
                    c("p", [right.clone()]),
                    c("q", [v(20)]),
                    c("q", [atom("a")]),
                ];
                if reverse {
                    store.reverse();
                }
                let (view, values) = view(&store);
                let terms = values
                    .into_iter()
                    .map(|(term, value)| (value, term))
                    .collect::<BTreeMap<_, _>>();
                for heads in &patterns {
                    for kept in 0..=heads.len() {
                        let plan = HeadPlan::compile(&heads[..kept], &heads[kept..]);
                        let result = plan.evaluate(&view);
                        let tuples = result
                            .matches
                            .iter()
                            .map(|m| m.kept.iter().chain(&m.removed).map(|id| id.0).collect())
                            .collect::<BTreeSet<Vec<usize>>>();
                        let actual = result
                            .matches
                            .iter()
                            .map(|m| {
                                (
                                    m.kept
                                        .iter()
                                        .chain(&m.removed)
                                        .map(|id| id.0)
                                        .collect::<Vec<_>>(),
                                    m.bindings
                                        .iter()
                                        .map(|(var, value)| (*var, terms[value].clone()))
                                        .collect::<BTreeMap<_, _>>(),
                                )
                            })
                            .collect::<BTreeMap<_, _>>();
                        assert_eq!(
                            actual,
                            scalar(heads, &store),
                            "heads={heads:?} store={store:?}"
                        );
                        assert_eq!(
                            tuples.len(),
                            result.matches.len(),
                            "proof multiplicity must not duplicate source applications"
                        );
                        assert!(
                            result
                                .matches
                                .iter()
                                .all(|m| m.kept.len() == kept
                                    && m.removed.len() == heads.len() - kept)
                        );
                    }
                }
            }
        }
    }
}
#[test]
fn duplicate_constructor_proofs_do_not_duplicate_resources() {
    let mut view = View::default();
    view.constructor("a", Value(0), &[]);
    view.constructor("a", Value(0), &[]);
    view.occurrence(Occurrence(0), "p", &[Value(0)]);
    view.occurrence(Occurrence(1), "p", &[Value(0)]);
    let plan = HeadPlan::compile(&[], &[c("p", [atom("a")]), c("p", [atom("a")])]);
    let result = plan.evaluate(&view);
    assert_eq!(result.matches.len(), 2);
    assert!(result.matches.iter().all(|m| m.removed[0] != m.removed[1]));
}

#[test]
fn changed_equality_snapshots_preserve_nonbinding_eligibility() {
    let plan = HeadPlan::compile(&[c("p", [t("f", [v(0)])])], &[c("q", [v(0)])]);
    for (store, count) in [
        (vec![c("p", [v(20)]), c("q", [atom("a")])], 0),
        (vec![c("p", [t("f", [atom("a")])]), c("q", [atom("a")])], 1),
        (vec![c("p", [t("f", [v(20)])]), c("q", [v(21)])], 0),
        (vec![c("p", [t("f", [v(20)])]), c("q", [v(20)])], 1),
    ] {
        let (view, _) = view(&store);
        assert_eq!(plan.evaluate(&view).matches.len(), count);
    }
    // Same source variable has different constructor knowledge in two alternatives.
    // These views must not be combined into an unqualified global equality table.
    let plan = HeadPlan::compile(&[], &[c("p", [t("f", [atom("a")])])]);
    let (left, _) = view(&[c("p", [t("f", [atom("a")])])]);
    let (right, _) = view(&[c("p", [t("f", [atom("b")])])]);
    assert_eq!(plan.evaluate(&left).matches.len(), 1);
    assert!(plan.evaluate(&right).matches.is_empty());
}
