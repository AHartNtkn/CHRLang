//! Independent plain-syntax oracle and synthetic graph representation for A6.
use chr_observe::graph::{AnswerView, Stats, TermView, equivalent};
use chr_syntax::{Answer, Constraint, Term, Var, atom, c, t, v};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

#[derive(Clone)]
enum Node {
    Variable(Var),
    Constructor(String, Vec<usize>),
}
struct Dag {
    nodes: Rc<Vec<Node>>,
    bindings: BTreeMap<Var, usize>,
    outputs: Vec<(String, usize)>,
    residual: Vec<(String, Vec<usize>)>,
}
impl AnswerView for Dag {
    type Handle = usize;
    type Variable = Var;
    fn output_count(&self) -> usize {
        self.outputs.len()
    }
    fn output(&self, i: usize) -> (&str, usize) {
        (&self.outputs[i].0, self.outputs[i].1)
    }
    fn residual_count(&self) -> usize {
        self.residual.len()
    }
    fn residual_name(&self, i: usize) -> &str {
        &self.residual[i].0
    }
    fn residual_arity(&self, i: usize) -> usize {
        self.residual[i].1.len()
    }
    fn residual_arg(&self, i: usize, j: usize) -> usize {
        self.residual[i].1[j]
    }
    fn resolve(&self, mut h: usize, stats: &mut Stats) -> TermView<'_, usize, Var> {
        loop {
            stats.dereferences += 1;
            match &self.nodes[h] {
                Node::Variable(v) => {
                    stats.binding_visits += 1;
                    if let Some(next) = self.bindings.get(v) {
                        h = *next;
                    } else {
                        return TermView::Variable(*v);
                    }
                }
                Node::Constructor(name, args) => return TermView::Constructor(name, args),
            }
        }
    }
}
fn graph(answer: &Answer, shared: bool, aliases: bool) -> Dag {
    fn import(
        x: &Term,
        nodes: &mut Vec<Node>,
        cache: &mut BTreeMap<Term, usize>,
        bindings: &mut BTreeMap<Var, usize>,
        shared: bool,
        aliases: bool,
    ) -> usize {
        if shared && let Some(h) = cache.get(x) {
            return *h;
        }
        let node = match x {
            Term::Var(v) => {
                if aliases {
                    let base = nodes.len();
                    nodes.push(Node::Variable(*v));
                    let alias = Var(v.0 + 10000);
                    bindings.insert(alias, base);
                    Node::Variable(alias)
                } else {
                    Node::Variable(*v)
                }
            }
            Term::App(n, args) => Node::Constructor(
                n.clone(),
                args.iter()
                    .map(|x| import(x, nodes, cache, bindings, shared, aliases))
                    .collect(),
            ),
        };
        let h = nodes.len();
        nodes.push(node);
        if shared {
            cache.insert(x.clone(), h);
        }
        h
    }
    let mut nodes = Vec::new();
    let mut cache = BTreeMap::new();
    let mut bindings = BTreeMap::new();
    let outputs = answer
        .outputs
        .iter()
        .map(|(n, x)| {
            (
                n.clone(),
                import(x, &mut nodes, &mut cache, &mut bindings, shared, aliases),
            )
        })
        .collect();
    let residual = answer
        .residual
        .iter()
        .map(|p| {
            (
                p.name.clone(),
                p.args
                    .iter()
                    .map(|x| import(x, &mut nodes, &mut cache, &mut bindings, shared, aliases))
                    .collect(),
            )
        })
        .collect();
    Dag {
        nodes: Rc::new(nodes),
        bindings,
        outputs,
        residual,
    }
}
fn answer(outputs: Vec<Term>, residual: Vec<Constraint>) -> Answer {
    Answer {
        outputs: outputs
            .into_iter()
            .enumerate()
            .map(|(i, x)| (i.to_string(), x))
            .collect(),
        residual,
    }
}
fn variables(a: &Answer) -> Vec<Var> {
    fn visit(t: &Term, s: &mut BTreeSet<Var>) {
        match t {
            Term::Var(v) => {
                s.insert(*v);
            }
            Term::App(_, xs) => {
                for x in xs {
                    visit(x, s)
                }
            }
        }
    }
    let mut found = BTreeSet::new();
    for (_, x) in &a.outputs {
        visit(x, &mut found)
    }
    for p in &a.residual {
        for x in &p.args {
            visit(x, &mut found)
        }
    }
    found.into_iter().collect()
}
fn rename(a: &Answer, m: &BTreeMap<Var, Var>) -> Answer {
    fn term(x: &Term, m: &BTreeMap<Var, Var>) -> Term {
        match x {
            Term::Var(v) => Term::Var(m[v]),
            Term::App(n, xs) => Term::App(n.clone(), xs.iter().map(|x| term(x, m)).collect()),
        }
    }
    let mut out = Answer {
        outputs: a
            .outputs
            .iter()
            .map(|(n, x)| (n.clone(), term(x, m)))
            .collect(),
        residual: a
            .residual
            .iter()
            .map(|p| Constraint {
                name: p.name.clone(),
                args: p.args.iter().map(|x| term(x, m)).collect(),
            })
            .collect(),
    };
    out.residual.sort();
    out
}
fn oracle(a: &Answer, b: &Answer) -> bool {
    let av = variables(a);
    let bv = variables(b);
    assert!(av.len() <= 4 && bv.len() <= 4);
    if av.len() != bv.len() {
        return false;
    }
    let mut expected = b.clone();
    expected.residual.sort();
    fn permute(i: usize, choices: &mut [Var], av: &[Var], a: &Answer, b: &Answer) -> bool {
        if i == choices.len() {
            return rename(
                a,
                &av.iter().copied().zip(choices.iter().copied()).collect(),
            ) == *b;
        }
        for j in i..choices.len() {
            choices.swap(i, j);
            let yes = permute(i + 1, choices, av, a, b);
            choices.swap(i, j);
            if yes {
                return true;
            }
        }
        false
    }
    permute(0, &mut bv.clone(), &av, a, &expected)
}
fn templates() -> Vec<(Answer, Answer, bool)> {
    let mut named = answer(vec![v(9)], vec![]);
    named.outputs[0].0 = "different".into();
    fn deep(x: Term) -> Term {
        (0..6).fold(x, |x, _| t("pair", [x.clone(), x]))
    }
    vec![
        (
            answer(vec![atom("a")], vec![c("p", [])]),
            answer(vec![atom("a")], vec![c("p", [])]),
            true,
        ),
        (
            answer(vec![atom("a")], vec![]),
            answer(vec![atom("b")], vec![]),
            false,
        ),
        (
            answer(vec![v(0), v(0)], vec![]),
            answer(vec![v(8), v(8)], vec![]),
            true,
        ),
        (
            answer(vec![v(0), v(0)], vec![]),
            answer(vec![v(8), v(9)], vec![]),
            false,
        ),
        (
            answer(vec![v(0)], vec![c("e", [v(0), v(1)])]),
            answer(vec![v(8)], vec![c("e", [v(8), v(9)])]),
            true,
        ),
        (
            answer(vec![v(0)], vec![c("e", [v(0), v(1)])]),
            answer(vec![v(8)], vec![c("e", [v(9), v(8)])]),
            false,
        ),
        (
            answer(
                vec![],
                vec![c("e", [v(0), v(1)]), c("e", [v(1), v(2)]), c("n", [v(2)])],
            ),
            answer(
                vec![],
                vec![c("e", [v(7), v(8)]), c("n", [v(8)]), c("e", [v(9), v(7)])],
            ),
            true,
        ),
        (
            answer(vec![], vec![c("p", [v(0)]), c("p", [v(0)])]),
            answer(vec![], vec![c("p", [v(8)]), c("p", [v(9)])]),
            false,
        ),
        (
            answer(
                vec![],
                vec![
                    c("e", [v(0), v(1)]),
                    c("e", [v(1), v(2)]),
                    c("e", [v(2), v(3)]),
                    c("e", [v(3), v(0)]),
                ],
            ),
            answer(
                vec![],
                vec![
                    c("e", [v(0), v(1)]),
                    c("e", [v(1), v(0)]),
                    c("e", [v(2), v(3)]),
                    c("e", [v(3), v(2)]),
                ],
            ),
            false,
        ),
        (
            answer(vec![t("f", [atom("a"), v(0)])], vec![]),
            answer(vec![t("f", [v(8), atom("a")])], vec![]),
            false,
        ),
        (answer(vec![v(0)], vec![]), named, false),
        (
            answer(vec![deep(v(0))], vec![c("p", [v(0)])]),
            answer(vec![deep(v(8))], vec![c("p", [v(8)])]),
            true,
        ),
    ]
}
fn record(id: &str, left: &Dag, right: &Dag, wanted: bool) {
    let mut stats = Stats::default();
    let actual = equivalent(left, right, &mut stats);
    println!(
        "GRAPH\t{id}\t{wanted}\t{actual}\t{}\t{}\t{}\t{}\t{}\t{}",
        stats.term_pairs,
        stats.occurrence_scans,
        stats.occurrence_candidates,
        stats.backtracks,
        stats.dereferences,
        stats.binding_visits
    );
    assert_eq!(actual, wanted, "{id}");
}
#[test]
fn targeted_templates() {
    for (i, (a, b, manual)) in templates().into_iter().enumerate() {
        assert_eq!(oracle(&a, &b), manual, "template oracle {i}");
        for layout in 0..4 {
            for aliases in [false, true] {
                for reverse in [false, true] {
                    let left = graph(&a, layout & 1 != 0, aliases);
                    let right = graph(&b, layout & 2 != 0, aliases);
                    let id = format!(
                        "target-{i}-{layout}-{}-{}",
                        u8::from(aliases),
                        u8::from(reverse)
                    );
                    if reverse {
                        record(&id, &right, &left, manual)
                    } else {
                        record(&id, &left, &right, manual)
                    }
                }
            }
        }
    }
}
#[test]
fn common_roots_different_environments() {
    let nodes = Rc::new(vec![
        Node::Variable(Var(0)),
        Node::Constructor("f".into(), vec![0]),
        Node::Constructor("a".into(), vec![]),
        Node::Constructor("b".into(), vec![]),
        Node::Constructor("a".into(), vec![]),
    ]);
    let make = |target| Dag {
        nodes: Rc::clone(&nodes),
        bindings: [(Var(0), target)].into(),
        outputs: vec![("x".into(), 1)],
        residual: vec![],
    };
    record("same-root-equal", &make(2), &make(4), true);
    record("same-root-unequal", &make(2), &make(3), false);
}
#[test]
fn all_directed_graph_pairs() {
    const EDGES: [(usize, usize); 6] = [(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)];
    const PERMS: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    let fixture = |bits: u8| {
        let mut residual = (0..3).map(|i| c("node", [v(i)])).collect::<Vec<_>>();
        for (i, (x, y)) in EDGES.iter().enumerate() {
            if bits & (1 << i) != 0 {
                residual.push(c("edge", [v(*x as u64), v(*y as u64)]));
            }
        }
        answer(vec![], residual)
    };
    for a in 0..64u8 {
        for b in 0..64u8 {
            let left = fixture(a);
            let right = fixture(b);
            let expected = PERMS.iter().any(|p| {
                EDGES.iter().enumerate().all(|(i, (x, y))| {
                    let j = EDGES
                        .iter()
                        .position(|&(u, v)| u == p[*x] && v == p[*y])
                        .unwrap();
                    ((a >> i) & 1) == ((b >> j) & 1)
                })
            });
            assert_eq!(oracle(&left, &right), expected);
            record(
                &format!("graph-{a}-{b}"),
                &graph(&left, a & 1 != 0, a & 2 != 0),
                &graph(&right, b & 1 != 0, b & 2 != 0),
                expected,
            );
        }
    }
}
