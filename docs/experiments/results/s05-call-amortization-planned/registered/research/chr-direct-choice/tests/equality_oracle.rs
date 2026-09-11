//! Independent owned-term substitution oracle; no candidate equality, contexts,
//! graph traversal or reference-interpreter routines derive expectations.
use chr_direct_choice::{Context, Graph, Label, NodeId};
use chr_syntax::{Term, Var};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
enum Expr {
    Unknown(usize),
    App(&'static str, Vec<Expr>),
    Choice(usize, Box<Expr>, Box<Expr>),
}
use Expr::{App as A, Choice as C, Unknown as U};
fn a() -> Expr {
    A("a", vec![])
}
fn b() -> Expr {
    A("b", vec![])
}
fn f(x: Expr) -> Expr {
    A("f", vec![x])
}
fn choice(label: usize, left: Expr, right: Expr) -> Expr {
    C(label, Box::new(left), Box::new(right))
}
fn ground_choices(e: &Expr, bits: usize) -> Term {
    match e {
        U(x) => Term::Var(Var(*x as u64)),
        A(name, xs) => Term::App(
            (*name).into(),
            xs.iter().map(|x| ground_choices(x, bits)).collect(),
        ),
        C(label, left, right) => ground_choices(
            if bits & (1 << label) == 0 {
                left
            } else {
                right
            },
            bits,
        ),
    }
}
fn import(g: &mut Graph, labels: &[Label], e: &Expr) -> NodeId {
    match e {
        U(x) => g.unknown(*x as u64),
        A(name, xs) => {
            let children = xs.iter().map(|x| import(g, labels, x)).collect();
            g.app(name, children)
        }
        C(label, left, right) => {
            let l = import(g, labels, left);
            let r = import(g, labels, right);
            g.choice(labels[*label], l, r)
        }
    }
}
fn substitute(t: &Term, env: &BTreeMap<Var, Term>) -> Term {
    match t {
        Term::Var(x) => env
            .get(x)
            .map(|t| substitute(t, env))
            .unwrap_or_else(|| t.clone()),
        Term::App(n, xs) => Term::App(n.clone(), xs.iter().map(|x| substitute(x, env)).collect()),
    }
}
fn contains(t: &Term, var: Var) -> bool {
    match t {
        Term::Var(x) => *x == var,
        Term::App(_, xs) => xs.iter().any(|x| contains(x, var)),
    }
}
fn equate(left: Term, right: Term, env: &mut BTreeMap<Var, Term>) -> bool {
    let left = substitute(&left, env);
    let right = substitute(&right, env);
    if left == right {
        return true;
    }
    match (left, right) {
        (Term::Var(x), t) | (t, Term::Var(x)) => {
            if contains(&t, x) {
                false
            } else {
                env.insert(x, t);
                true
            }
        }
        (Term::App(a, xs), Term::App(b, ys)) => {
            a == b && xs.len() == ys.len() && xs.into_iter().zip(ys).all(|(x, y)| equate(x, y, env))
        }
    }
}
fn canonical(terms: Vec<Term>) -> Vec<Term> {
    fn walk(t: Term, names: &mut BTreeMap<Var, Var>) -> Term {
        match t {
            Term::Var(x) => {
                let next = Var(names.len() as u64);
                Term::Var(*names.entry(x).or_insert(next))
            }
            Term::App(n, xs) => Term::App(n, xs.into_iter().map(|x| walk(x, names)).collect()),
        }
    }
    let mut names = BTreeMap::new();
    terms.into_iter().map(|t| walk(t, &mut names)).collect()
}

#[test]
fn equation_pairs_match_scalar_finite_tree_solutions_in_every_assignment() {
    // Includes aliases, direct/indirect cycles, context-only effects and choices
    // below constructors, independent/correlated choices and equal arms.
    let catalogue = vec![
        (U(0), U(1), None),
        (U(1), U(2), None),
        (U(0), a(), None),
        (U(0), b(), Some((0, false))),
        (U(1), a(), Some((1, true))),
        (U(0), f(U(1)), None),
        (U(1), f(U(0)), None),
        (U(0), f(U(0)), None),
        (U(0), choice(0, U(0), a()), None),
        (U(0), choice(0, f(U(0)), a()), None),
        (U(0), choice(0, U(1), a()), None),
        (U(1), choice(1, f(U(0)), b()), None),
        (f(U(0)), f(choice(1, a(), b())), None),
        (choice(0, a(), b()), choice(1, a(), b()), None),
        (choice(0, a(), b()), choice(0, a(), b()), None),
        (U(0), f(choice(0, U(1), U(0))), None),
        (U(2), choice(0, a(), a()), None),
        (A("f", vec![]), f(a()), Some((1, false))),
    ];
    let roots = [U(0), U(1), U(2), A("pair", vec![U(0), U(1), U(0)])];
    for (i, first) in catalogue.iter().enumerate() {
        for (j, second) in catalogue.iter().enumerate() {
            let mut graph = Graph::default();
            let labels: Vec<_> = (0..2).map(|_| graph.birth(Context::all())).collect();
            let nodes: Vec<_> = roots
                .iter()
                .map(|e| import(&mut graph, &labels, e))
                .collect();
            for (l, r, scope) in [first, second] {
                let l = import(&mut graph, &labels, l);
                let r = import(&mut graph, &labels, r);
                let context = scope
                    .map(|(label, arm)| Context::all().select(labels[label], arm).unwrap())
                    .unwrap_or_default();
                graph.unify(l, r, &context);
            }
            let actual = graph.observe(&nodes, &Context::all());
            for bits in 0..4 {
                let mut env = BTreeMap::new();
                let mut live = true;
                for (l, r, scope) in [first, second] {
                    if scope.is_none_or(|(label, arm)| (bits & (1 << label) != 0) == arm) {
                        live = live
                            && equate(ground_choices(l, bits), ground_choices(r, bits), &mut env);
                    }
                }
                let values: Vec<_> = (0..2).map(|label| bits & (1 << label) != 0).collect();
                let observed: Vec<_> = actual
                    .iter()
                    .filter(|(context, _)| context.contains_assignment(&values))
                    .collect();
                assert_eq!(
                    observed.len(),
                    usize::from(live),
                    "equations {i},{j}; bits {bits}"
                );
                if live {
                    let expected = roots
                        .iter()
                        .map(|e| substitute(&ground_choices(e, bits), &env))
                        .collect();
                    assert_eq!(
                        canonical(observed[0].1.clone()),
                        canonical(expected),
                        "equations {i},{j}; bits {bits}"
                    );
                    // Entailment uses the existing substitution without binding it.
                    let expected_equal = substitute(&Term::Var(Var(0)), &env)
                        == substitute(&Term::Var(Var(1)), &env);
                    let equal = graph.equal(nodes[0], nodes[1], &Context::all());
                    assert_eq!(
                        equal
                            .iter()
                            .filter(|r| r.contains_assignment(&values))
                            .count(),
                        usize::from(expected_equal),
                        "entailment {i},{j}; bits {bits}"
                    );
                }
            }
        }
    }
    eprintln!(
        "scalar equality oracle: 324 ordered equation pairs, 1296 assignments; full joint values and entailment checked"
    );
}
