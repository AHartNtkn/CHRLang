use chr_direct_choice::{Context, Graph};
use chr_syntax::{atom, v};

#[test]
fn bindings_aliases_and_failures_are_context_local() {
    let mut g = Graph::default();
    let c = g.birth(Context::all());
    let left = Context::all().select(c, false).unwrap();
    let right = Context::all().select(c, true).unwrap();
    let x = g.unknown(0);
    let y = g.unknown(1);
    let z = g.unknown(2);
    let a = g.app("a", vec![]);
    let b = g.app("b", vec![]);
    assert_eq!(g.unify(x, y, &Context::all()), vec![Context::all()]);
    assert_eq!(g.unify(y, a, &left), vec![left.clone()]);
    assert_eq!(g.unify(z, x, &Context::all()).len(), 2);
    assert_eq!(g.unify(y, b, &right), vec![right.clone()]);
    let actual: Vec<_> = g
        .observe(&[x, y, z], &Context::all())
        .into_iter()
        .map(|(_, x)| x)
        .collect();
    assert_eq!(actual, vec![vec![atom("a"); 3], vec![atom("b"); 3]]);
    assert!(g.unify(a, b, &left).is_empty());
    assert_eq!(
        g.observe(&[x, y, z], &Context::all()),
        vec![(right, vec![atom("b"); 3])]
    );
}

#[test]
fn finite_tree_occurs_checks_follow_aliases_and_preserve_good_arms() {
    let mut g = Graph::default();
    let label = g.birth(Context::all());
    let x = g.unknown(0);
    let y = g.unknown(1);
    let a = g.app("a", vec![]);
    let fy = g.app("f", vec![y]);
    let rhs = g.choice(label, fy, a);
    g.unify(x, y, &Context::all());
    let good = Context::all().select(label, true).unwrap();
    assert_eq!(g.unify(y, rhs, &Context::all()), vec![good.clone()]);
    assert_eq!(
        g.observe(&[x, y], &Context::all()),
        vec![(good, vec![atom("a"), atom("a")])]
    );
    // A late indirect cycle must be found through both aliases and constructors.
    let mut g = Graph::default();
    let x = g.unknown(0);
    let y = g.unknown(1);
    let fy = g.app("f", vec![y]);
    g.unify(x, fy, &Context::all());
    assert!(g.unify(y, x, &Context::all()).is_empty());
    assert!(g.observe(&[], &Context::all()).is_empty());
}

#[test]
fn reflexive_choice_arm_is_not_a_constructor_cycle() {
    let mut g = Graph::default();
    let label = g.birth(Context::all());
    let x = g.unknown(0);
    let a = g.app("a", vec![]);
    let rhs = g.choice(label, x, a);
    assert_eq!(g.unify(x, rhs, &Context::all()).len(), 2);
    let mut values = g.observe(&[x], &Context::all());
    values.sort();
    assert_eq!(
        values,
        vec![
            (Context::all().select(label, false).unwrap(), vec![v(0)]),
            (Context::all().select(label, true).unwrap(), vec![atom("a")]),
        ]
    );
}

#[test]
fn nonbinding_equality_never_instantiates_or_fails_a_region() {
    let mut g = Graph::default();
    let label = g.birth(Context::all());
    let x = g.unknown(0);
    let y = g.unknown(1);
    let same_x = g.unknown(0);
    let a = g.app("a", vec![]);
    let b = g.app("b", vec![]);
    let choice = g.choice(label, a, b);
    let before = g.observe(&[x, y], &Context::all());
    assert!(g.equal(x, y, &Context::all()).is_empty());
    assert!(g.equal(x, a, &Context::all()).is_empty());
    assert_eq!(g.equal(x, same_x, &Context::all()), vec![Context::all()]);
    assert_eq!(g.observe(&[x, y], &Context::all()), before);
    // An acyclic choice-valued binding stays one undemanded region.
    assert_eq!(g.unify(x, choice, &Context::all()), vec![Context::all()]);
    assert_eq!(
        g.equal(x, a, &Context::all()),
        vec![Context::all().select(label, false).unwrap()]
    );
    let f1 = g.app("f", vec![x]);
    let f2 = g.app("f", vec![choice]);
    assert_eq!(g.equal(f1, f2, &Context::all()), vec![Context::all()]);
    let wrong = g.app("f", vec![x, y]);
    assert!(g.equal(f1, wrong, &Context::all()).is_empty());
    assert_eq!(g.observe(&[f1, y], &Context::all()).len(), 2);
}

#[test]
fn disconnected_failure_and_constructor_arity_affect_complete_observations() {
    let mut g = Graph::default();
    let label = g.birth(Context::all());
    let x = g.unknown(4);
    let a = g.app("a", vec![]);
    let fa = g.app("f", vec![a]);
    let f = g.app("f", vec![]);
    let left = Context::all().select(label, false).unwrap();
    let right = Context::all().select(label, true).unwrap();
    assert!(g.unify(fa, f, &left).is_empty());
    assert_eq!(
        g.observe(&[x], &Context::all()),
        vec![(right.clone(), vec![v(4)])]
    );
    g.fail(&right);
    assert!(g.observe(&[a], &Context::all()).is_empty());
    assert!(g.histories(&Context::all()).is_empty());
}
