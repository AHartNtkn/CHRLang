use chr_relational::contextual::Store;
use chr_syntax::{atom, c, t, v};
#[test]
fn shared_nodes_do_not_share_bindings_or_consumption() {
    let mut base = Store::default();
    let x = base.unknown();
    let a = base.constructor("a", &[]);
    let b = base.constructor("b", &[]);
    let fx = base.constructor("f", &[x]);
    base.post("open", &[fx]);
    base.post("ticket", &[x]);
    base.post("ticket", &[x]);
    let mut left = base.clone();
    let mut right = base.clone();
    assert!(base.shares_nodes(&left) && left.shares_nodes(&right));
    left.equate(x, a);
    right.equate(x, b);
    while left.step() {}
    while right.step() {}
    let heads = [c("open", [t("f", [v(0)])]), c("ticket", [v(0)])];
    let claims = left.matches(&[], &heads);
    assert_eq!(claims.len(), 2);
    assert!(left.consume(&claims[0]));
    assert!(!left.consume(&claims[1]));
    assert_eq!(right.matches(&[], &heads).len(), 2);
    assert_eq!(base.matches(&[], &heads).len(), 2);
    assert_eq!(left.export(&[x]), Some(vec![atom("a")]));
    assert_eq!(right.export(&[x]), Some(vec![atom("b")]));
    assert!(matches!(
        &base.export(&[x]).unwrap()[0],
        chr_syntax::Term::Var(_)
    ));
    left.equate(x, b);
    while left.step() {}
    assert!(left.failed());
    assert!(!right.failed());
    assert_eq!(right.matches(&[], &heads).len(), 2);
}

fn settle(s: &mut Store) {
    for _ in 0..1000 {
        if !s.step() {
            return;
        }
    }
    panic!("finite equality bound");
}
#[test]
fn posting_cycles_and_partial_visibility_remain_context_local() {
    let mut base = Store::default();
    let x = base.unknown();
    let y = base.unknown();
    let a = base.constructor("a", &[]);
    let fx = base.constructor("f", &[x]);
    let fa = base.constructor("f", &[a]);
    base.post("open", &[fx]);
    let mut early = base.clone();
    early.equate(fx, fa);
    early.equate(y, a);
    assert!(early.step());
    assert!(early.pending() > 0);
    assert!(early.export(&[x]).is_none());
    assert_eq!(
        early
            .matches(&[], &[c("open", [t("f", [atom("a")])])])
            .len(),
        1
    );
    assert!(
        base.matches(&[], &[c("open", [t("f", [atom("a")])])])
            .is_empty()
    );
    early.post("new", &[x]);
    assert!(base.matches(&[], &[c("new", [v(0)])]).is_empty());
    settle(&mut early);
    assert_eq!(early.export(&[x, y]), Some(vec![atom("a"), atom("a")]));
    let mut cycle = base.clone();
    cycle.equate(x, fx);
    settle(&mut cycle);
    assert!(cycle.failed());
    assert!(!base.failed());
    assert!(!early.failed());
    let one = base.post("one", &[x]);
    assert!(
        base.matches(&[c("one", [v(0)])], &[c("one", [v(0)])])
            .is_empty()
    );
    let bad = chr_relational::Match {
        kept: vec![one],
        removed: vec![one],
        bindings: Default::default(),
    };
    assert!(!base.consume(&bad));
    assert_eq!(base.matches(&[], &[c("one", [v(0)])]).len(), 1);
    let two = base.post("one", &[x]);
    assert_ne!(one, two);
    let claim = base
        .matches(&[c("one", [v(0)])], &[c("one", [v(0)])])
        .pop()
        .unwrap();
    assert!(base.consume(&claim));
    assert_eq!(base.matches(&[], &[c("one", [v(0)])]).len(), 1);
    assert!(!base.consume(&claim));
}

// Owned substitution has no arena, parent classes, context maps or matching index.
use chr_syntax::{Term, Var};
use std::collections::BTreeMap;
fn subst(t: &Term, env: &BTreeMap<Var, Term>) -> Term {
    match t {
        Term::Var(v) => env.get(v).map_or_else(|| t.clone(), |t| subst(t, env)),
        Term::App(n, xs) => Term::App(n.clone(), xs.iter().map(|t| subst(t, env)).collect()),
    }
}
fn unify(a: &Term, b: &Term, env: &mut BTreeMap<Var, Term>) -> bool {
    fn occurs(v: Var, t: &Term) -> bool {
        match t {
            Term::Var(x) => v == *x,
            Term::App(_, xs) => xs.iter().any(|x| occurs(v, x)),
        }
    }
    let (a, b) = (subst(a, env), subst(b, env));
    if a == b {
        return true;
    }
    match (a, b) {
        (Term::Var(v), t) | (t, Term::Var(v)) => {
            if occurs(v, &t) {
                false
            } else {
                env.insert(v, t);
                true
            }
        }
        (Term::App(n, xs), Term::App(m, ys)) => {
            n == m && xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| unify(x, &y, env))
        }
    }
}
fn canonical(ts: Vec<Term>) -> Vec<Term> {
    fn walk(t: Term, names: &mut BTreeMap<Var, Var>) -> Term {
        match t {
            Term::Var(v) => {
                let next = Var(names.len() as u64);
                Term::Var(*names.entry(v).or_insert(next))
            }
            Term::App(n, xs) => Term::App(n, xs.into_iter().map(|t| walk(t, names)).collect()),
        }
    }
    let mut names = BTreeMap::new();
    ts.into_iter().map(|t| walk(t, &mut names)).collect()
}
#[test]
fn ordered_equations_and_forks_match_independent_substitution() {
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
    let pairs = (0..8)
        .flat_map(|a| (a..8).map(move |b| (a, b)))
        .collect::<Vec<_>>();
    for first in &pairs {
        let mut base = Store::default();
        let x = base.unknown();
        let y = base.unknown();
        let a = base.constructor("a", &[]);
        let b = base.constructor("b", &[]);
        let fx = base.constructor("f", &[x]);
        let fy = base.constructor("f", &[y]);
        let fa = base.constructor("f", &[a]);
        let gx = base.constructor("g", &[x]);
        let ids = [x, y, a, b, fx, fy, fa, gx];
        for id in ids {
            base.post("p", &[id]);
        }
        base.equate(ids[first.0], ids[first.1]);
        settle(&mut base);
        for second in &pairs {
            let mut branch = base.clone();
            let mut env = BTreeMap::new();
            let valid = unify(&terms[first.0], &terms[first.1], &mut env)
                && unify(&terms[second.0], &terms[second.1], &mut env);
            branch.equate(ids[second.0], ids[second.1]);
            settle(&mut branch);
            assert_eq!(!branch.failed(), valid, "{first:?} {second:?}");
            if !valid {
                assert!(branch.export(&ids).is_none());
                continue;
            }
            let expected = terms.iter().map(|t| subst(t, &env)).collect::<Vec<_>>();
            assert_eq!(
                canonical(branch.export(&ids).unwrap()),
                canonical(expected.clone())
            );
            let actual = branch
                .matches(&[c("p", [v(0)])], &[c("p", [v(0)])])
                .into_iter()
                .map(|m| (m.kept[0].0, m.removed[0].0))
                .collect::<std::collections::BTreeSet<_>>();
            let wanted = (0..8)
                .flat_map(|i| (0..8).map(move |j| (i, j)))
                .filter(|(i, j)| i != j && expected[*i] == expected[*j])
                .collect();
            assert_eq!(actual, wanted, "{first:?} {second:?}");
            assert!(base.shares_nodes(&branch));
        }
    }
}
