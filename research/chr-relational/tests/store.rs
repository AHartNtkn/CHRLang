use chr_relational::store::Store;
use chr_relational::{HeadPlan, Match, Occurrence};
use chr_syntax::{atom, c, t, v};
use std::collections::BTreeMap;
fn settle(s: &mut Store) {
    for _ in 0..10000 {
        if !s.step() {
            assert_eq!(s.pending(), 0);
            return;
        }
    }
    panic!("owner service bound");
}
#[test]
fn equality_enables_indexed_join_before_unrelated_work_finishes() {
    let mut s = Store::default();
    let x = s.unknown();
    let u = s.unknown();
    let a = s.constructor("a", &[]);
    let f = s.constructor("f", &[a]);
    let open = s.post("open", &[x]);
    let ticket = s.post("ticket", &[a]);
    let plan = HeadPlan::compile(&[], &[c("open", [t("f", [v(0)])]), c("ticket", [v(0)])]);
    assert!(s.matches(&plan).matches.is_empty());
    s.equate(x, f);
    s.equate(u, a);
    assert!(s.export(&[x, u]).is_none());
    assert!(s.step());
    assert!(s.pending() > 0);
    let matches = s.matches(&plan).matches;
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].removed, vec![open, ticket]);
    assert!(s.consume(&matches[0]));
    assert!(s.matches(&plan).matches.is_empty());
    settle(&mut s);
    assert_eq!(
        s.export(&[x, u]),
        Some(vec![t("f", [atom("a")]), atom("a")])
    );
}
#[test]
fn parent_congruence_and_incident_key_repair_enable_repeated_variables() {
    let mut s = Store::default();
    let x = s.unknown();
    let y = s.unknown();
    let fx = s.constructor("f", &[x]);
    let fy = s.constructor("f", &[y]);
    s.post("p", &[fx]);
    s.post("q", &[fy]);
    let plan = HeadPlan::compile(&[c("p", [v(0)])], &[c("q", [v(0)])]);
    assert!(s.matches(&plan).matches.is_empty());
    s.equate(x, y);
    settle(&mut s);
    assert_eq!(s.matches(&plan).matches.len(), 1);
    let a = s.constructor("a", &[]);
    s.equate(y, a);
    settle(&mut s);
    assert_eq!(s.export(&[fx, fy]), Some(vec![t("f", [atom("a")]); 2]));
}
#[test]
fn finite_tree_failure_and_forked_consumption_are_isolated() {
    let mut base = Store::default();
    let x = base.unknown();
    let a = base.constructor("a", &[]);
    let b = base.constructor("b", &[]);
    let fx = base.constructor("f", &[x]);
    let token = base.post("token", &[x]);
    let mut good = base.clone();
    let mut bad = base.clone();
    bad.equate(x, fx);
    settle(&mut bad);
    assert!(bad.failed());
    assert!(bad.export(&[a]).is_none());
    good.equate(x, a);
    settle(&mut good);
    let plan = HeadPlan::compile(&[], &[c("token", [atom("a")])]);
    let claim = good.matches(&plan).matches.pop().unwrap();
    assert!(good.consume(&claim));
    assert!(good.matches(&plan).matches.is_empty());
    assert!(!base.failed());
    base.equate(x, b);
    settle(&mut base);
    assert_eq!(
        base.matches(&HeadPlan::compile(&[], &[c("token", [atom("b")])]))
            .matches[0]
            .removed,
        vec![token]
    );
    assert_eq!(good.export(&[x]), Some(vec![atom("a")]));
    assert_eq!(base.export(&[x]), Some(vec![atom("b")]));
    for arity in [false, true] {
        let mut s = Store::default();
        let a = s.constructor("a", &[]);
        let other = if arity {
            s.constructor("a", &[a])
        } else {
            s.constructor("b", &[])
        };
        s.equate(a, other);
        settle(&mut s);
        assert!(s.failed());
    }
    let mut s = Store::default();
    let x = s.unknown();
    let y = s.unknown();
    let fy = s.constructor("f", &[y]);
    s.equate(x, fy);
    s.equate(y, x);
    settle(&mut s);
    assert!(s.failed());
    let mut s = Store::default();
    let x = s.unknown();
    let y = s.unknown();
    s.equate(x, y);
    s.equate(y, x);
    settle(&mut s);
    assert!(!s.failed());
    let out = s.export(&[x, y]).unwrap();
    assert_eq!(out[0], out[1]);
}
#[test]
fn stale_atomic_claims_and_distinct_equal_valued_resources() {
    let mut s = Store::default();
    let a = s.constructor("a", &[]);
    let first = s.post("p", &[a]);
    let second = s.post("p", &[a]);
    let plan = HeadPlan::compile(&[c("p", [v(0)])], &[c("p", [v(0)])]);
    let matches = s.matches(&plan).matches;
    assert_eq!(matches.len(), 2);
    assert!(!s.consume(&Match {
        kept: vec![first],
        removed: vec![first],
        bindings: BTreeMap::new()
    }));
    assert!(s.consume(&matches[0]));
    assert!(!s.consume(&matches[1]));
    assert_eq!(
        s.matches(&HeadPlan::compile(&[], &[c("p", [v(0)])]))
            .matches
            .len(),
        1
    );
    let fresh = s.post("p", &[a]);
    assert_ne!(fresh, first);
    assert_ne!(fresh, second);
    assert!(!s.consume(&Match {
        kept: vec![],
        removed: vec![Occurrence(99999), fresh],
        bindings: BTreeMap::new()
    }));
    assert_eq!(s.matches(&plan).matches.len(), 2);
}

// Owned substitutions are independent of the candidate's union-find and indexes.
fn substitute(
    term: &chr_syntax::Term,
    env: &BTreeMap<chr_syntax::Var, chr_syntax::Term>,
) -> chr_syntax::Term {
    use chr_syntax::Term;
    match term {
        Term::Var(x) => env
            .get(x)
            .map_or_else(|| term.clone(), |t| substitute(t, env)),
        Term::App(n, xs) => Term::App(n.clone(), xs.iter().map(|x| substitute(x, env)).collect()),
    }
}
fn owned_equate(
    left: &chr_syntax::Term,
    right: &chr_syntax::Term,
    env: &mut BTreeMap<chr_syntax::Var, chr_syntax::Term>,
) -> bool {
    use chr_syntax::{Term, Var};
    fn occurs(x: Var, t: &Term) -> bool {
        match t {
            Term::Var(y) => x == *y,
            Term::App(_, xs) => xs.iter().any(|t| occurs(x, t)),
        }
    }
    let (left, right) = (substitute(left, env), substitute(right, env));
    if left == right {
        return true;
    }
    match (left, right) {
        (Term::Var(x), t) | (t, Term::Var(x)) => {
            if occurs(x, &t) {
                return false;
            }
            env.insert(x, t);
            true
        }
        (Term::App(n, xs), Term::App(m, ys)) => {
            n == m
                && xs.len() == ys.len()
                && xs.iter().zip(&ys).all(|(x, y)| owned_equate(x, y, env))
        }
    }
}
fn canonical(terms: &[chr_syntax::Term]) -> Vec<chr_syntax::Term> {
    use chr_syntax::{Term, Var};
    fn walk(t: &Term, names: &mut BTreeMap<Var, Var>) -> Term {
        match t {
            Term::Var(x) => {
                let next = Var(names.len() as u64);
                Term::Var(*names.entry(*x).or_insert(next))
            }
            Term::App(n, xs) => Term::App(n.clone(), xs.iter().map(|x| walk(x, names)).collect()),
        }
    }
    let mut names = BTreeMap::new();
    terms.iter().map(|t| walk(t, &mut names)).collect()
}
#[test]
fn ordered_equation_pairs_match_owned_substitution_and_repaired_joins() {
    let terms = [
        v(0),
        v(1),
        atom("a"),
        atom("b"),
        t("f", [v(0)]),
        t("f", [v(1)]),
        t("f", [atom("a")]),
        t("g", [v(0)]),
    ];
    let pairs = (0..terms.len())
        .flat_map(|i| (i..terms.len()).map(move |j| (i, j)))
        .collect::<Vec<_>>();
    let mut checked = 0;
    for first in &pairs {
        for second in &pairs {
            let mut s = Store::default();
            let x = s.unknown();
            let y = s.unknown();
            let a = s.constructor("a", &[]);
            let b = s.constructor("b", &[]);
            let fx = s.constructor("f", &[x]);
            let fy = s.constructor("f", &[y]);
            let fa = s.constructor("f", &[a]);
            let gx = s.constructor("g", &[x]);
            let values = [x, y, a, b, fx, fy, fa, gx];
            for value in values {
                s.post("p", &[value]);
            }
            let plan = HeadPlan::compile(&[c("p", [v(0)])], &[c("p", [v(0)])]);
            let mut env = BTreeMap::new();
            let mut valid = true;
            for &(i, j) in [first, second] {
                if valid {
                    valid = owned_equate(&terms[i], &terms[j], &mut env);
                }
                s.equate(values[i], values[j]);
                settle(&mut s);
                assert_eq!(!s.failed(), valid, "equations {first:?} {second:?}");
                if valid {
                    let expected = terms
                        .iter()
                        .map(|t| substitute(t, &env))
                        .collect::<Vec<_>>();
                    assert_eq!(canonical(&s.export(&values).unwrap()), canonical(&expected));
                    let expected_pairs = (0..8)
                        .flat_map(|i| (0..8).map(move |j| (i, j)))
                        .filter(|(i, j)| i != j && expected[*i] == expected[*j])
                        .collect::<std::collections::BTreeSet<_>>();
                    let actual = s
                        .matches(&plan)
                        .matches
                        .into_iter()
                        .map(|m| (m.kept[0].0, m.removed[0].0))
                        .collect();
                    assert_eq!(
                        expected_pairs, actual,
                        "index repair for {first:?} {second:?}"
                    );
                } else {
                    assert!(s.export(&values).is_none());
                    assert!(s.matches(&plan).matches.is_empty());
                }
            }
            checked += 1;
        }
    }
    assert_eq!(checked, 1296);
}
