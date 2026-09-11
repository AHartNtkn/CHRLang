use chr_structural::joint_region::{Observation, Predicate, Region};
use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, atom, c, eq, or, v};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
#[allow(dead_code)]
mod scalar;

// Condition each star arm on the visible value; merge Cartesian streams by
// source occurrence indices. Duplicate values retain different indices.
type Pending = (usize, Vec<usize>, usize, Vec<usize>);
struct Ordered {
    axes: Vec<Vec<Vec<usize>>>,
    heap: BinaryHeap<Reverse<Pending>>,
    seen: BTreeSet<(usize, Vec<usize>)>,
    padding: usize,
}
impl Ordered {
    fn new(
        domain: &[Term],
        n: usize,
        visible: usize,
        weights: &BTreeMap<Vec<Term>, u128>,
        padding: usize,
    ) -> Self {
        let mut result = Self {
            axes: vec![],
            heap: BinaryHeap::new(),
            seen: BTreeSet::new(),
            padding,
        };
        for (value, count) in weights {
            let axes = (0..n)
                .map(|i| {
                    let mut axis = domain
                        .iter()
                        .enumerate()
                        .filter_map(|(j, x)| ((x == &value[0]) == (i == visible)).then_some(j))
                        .collect::<Vec<_>>();
                    axis.sort_by_key(|j| ((j % 2) * padding, *j));
                    axis
                })
                .collect::<Vec<_>>();
            assert_eq!(
                axes.iter().map(|x| x.len() as u128).product::<u128>(),
                *count
            );
            if axes.iter().any(Vec::is_empty) {
                continue;
            }
            let stream = result.axes.len();
            result.axes.push(axes);
            result.push(stream, vec![0; n]);
        }
        result
    }
    fn push(&mut self, stream: usize, positions: Vec<usize>) {
        // ponytail: retain visited successful positions; replace with a unique
        // Cartesian-parent traversal if this ownership dominates measured costs.
        if !self.seen.insert((stream, positions.clone())) {
            return;
        }
        let row = self.axes[stream]
            .iter()
            .zip(&positions)
            .map(|(axis, p)| axis[*p])
            .collect::<Vec<_>>();
        let cost = row.iter().map(|j| (j % 2) * self.padding).sum();
        self.heap.push(Reverse((cost, row, stream, positions)));
    }
}
impl Iterator for Ordered {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let Reverse((_, row, stream, positions)) = self.heap.pop()?;
        for i in 0..positions.len() {
            if positions[i] + 1 < self.axes[stream][i].len() {
                let mut next = positions.clone();
                next[i] += 1;
                self.push(stream, next);
            }
        }
        Some(row)
    }
}
fn host(body: Goal, heads: Vec<Term>, priority: usize, visible: usize) -> Vec<Rule> {
    let mut rs = vec![
        Rule::propagate("history", [c("watch", [v(0)])], c("seen", [v(0)]).into()),
        Rule::simplify(
            "a_wins",
            [c("watch", [atom("a")]), c("token", [])],
            c("winner", [atom("a")]).into(),
        ),
        Rule::simplify("launch", [c("start", heads)], body),
    ];
    rs.insert(
        priority + 1,
        Rule::simplify(
            "other_wins",
            [c("token", [])],
            c("winner", [atom("other")]).into(),
        ),
    );
    assert!(visible < 4);
    rs
}
fn source(
    domain: &[Term],
    n: usize,
    visible: usize,
    priority: usize,
    alias: bool,
    padding: usize,
) -> (Vec<Rule>, Query) {
    let mut body = vec![];
    for i in 0..n {
        let leaves = domain
            .iter()
            .enumerate()
            .map(|(j, x)| {
                Goal::And(
                    std::iter::repeat_n(Goal::True, (j % 2) * padding)
                        .chain(vec![
                            eq(v(i as u64), x.clone()),
                            eq(v((n + i) as u64), atom(&format!("i{j}"))),
                        ])
                        .collect(),
                )
            })
            .collect::<Vec<_>>();
        body.push(or(
            or(leaves[0].clone(), leaves[1].clone()),
            or(leaves[2].clone(), leaves[3].clone()),
        ));
    }
    for i in 0..n {
        if i != visible {
            body.push(c("neq", [v(visible as u64), v(i as u64)]).into());
        }
    }
    let mut rs = host(
        Goal::And(body),
        (0..2 * n).map(|i| v(i as u64)).collect(),
        priority,
        visible,
    );
    for a in ["a", "b", "c", "d"] {
        for b in ["a", "b", "c", "d"] {
            if a != b {
                rs.push(Rule::simplify(
                    &format!("neq_{a}_{b}"),
                    [c("neq", [atom(a), atom(b)])],
                    Goal::True,
                ));
            }
        }
    }
    rs.push(Rule::simplify(
        "neq_fail",
        [c("neq", [v(0), v(1)])],
        Goal::Fail,
    ));
    let mut outputs = vec![("out".into(), Var(visible as u64))];
    if alias {
        outputs.push(("alias".into(), Var(visible as u64)));
    }
    outputs.extend((0..n).map(|i| (format!("index{i}"), Var((n + i) as u64))));
    (
        rs,
        Query {
            constraints: vec![
                c("start", (0..2 * n).map(|i| v(i as u64)).collect::<Vec<_>>()),
                c("watch", [v(visible as u64)]),
                c("token", []),
            ],
            outputs,
        },
    )
}
fn run(rs: Vec<Rule>, q: Query) -> Vec<Answer> {
    let mut search = chr_reference::Search::new(rs, q).unwrap();
    let batch = search.advance(1_000_000);
    assert!(batch.exhausted);
    batch.answers
}
fn strip(mut a: Answer) -> Answer {
    a.outputs.retain(|(name, _)| !name.starts_with("index"));
    a.residual.sort();
    a
}
fn bag(xs: impl IntoIterator<Item = Answer>) -> BTreeMap<Answer, usize> {
    let mut bag = BTreeMap::new();
    for x in xs {
        *bag.entry(strip(x)).or_default() += 1;
    }
    bag
}
#[test]
fn contracted_hidden_choices_preserve_ordered_consuming_answers() {
    let (mut cases, mut cartesian, mut admitted) = (0, 0, 0);
    for n in [2, 4] {
        for duplicate in [false, true] {
            for reverse in [false, true] {
                let mut domain = ["a", "b", "c", if duplicate { "a" } else { "d" }]
                    .map(atom)
                    .to_vec();
                if reverse {
                    domain.reverse();
                }
                for visible in [0, n - 1] {
                    let region = Region {
                        domains: (0..n).map(|i| (Var(i as u64), domain.clone())).collect(),
                        predicates: (0..n)
                            .filter(|i| *i != visible)
                            .map(|i| Predicate::Different(v(visible as u64), v(i as u64)))
                            .collect(),
                    };
                    let prepared = region
                        .prepare_sparse(
                            &[Var(visible as u64)],
                            &[],
                            &[],
                            Observation::Counted,
                            100_000,
                        )
                        .unwrap();
                    let weights = prepared
                        .weighted_iter(&[], 100_000)
                        .unwrap()
                        .collect::<BTreeMap<_, _>>();
                    drop((prepared, region));
                    for priority in 0..3 {
                        for alias in [false, true] {
                            for padding in [0, 3] {
                                let (rs, q) = source(&domain, n, visible, priority, alias, padding);
                                let scalar = bag(scalar::run(&rs, &q, 1_000_000));
                                let raw = run(rs, q);
                                assert_eq!(bag(raw.clone()), scalar);
                                let source_indices = raw
                                    .iter()
                                    .map(|a| {
                                        a.outputs
                                            .iter()
                                            .filter(|(name, _)| name.starts_with("index"))
                                            .map(|(_, t)| {
                                                let Term::App(name, args) = t else { panic!() };
                                                assert!(args.is_empty());
                                                name.strip_prefix('i')
                                                    .unwrap()
                                                    .parse::<usize>()
                                                    .unwrap()
                                            })
                                            .collect::<Vec<_>>()
                                    })
                                    .collect::<Vec<_>>();
                                let ordered = Ordered::new(&domain, n, visible, &weights, padding)
                                    .collect::<Vec<_>>();
                                assert_eq!(
                                    ordered, source_indices,
                                    "n={n} dup={duplicate} reverse={reverse} visible={visible} priority={priority}"
                                );
                                let expected = raw.into_iter().map(strip).collect::<Vec<_>>();
                                let actual = ordered
                                    .iter()
                                    .map(|row| {
                                        let x = domain[row[visible]].clone();
                                        let rs = host(eq(v(0), x), vec![v(0)], priority, 0);
                                        let mut outputs = vec![("out".into(), Var(0))];
                                        if alias {
                                            outputs.push(("alias".into(), Var(0)));
                                        }
                                        let mut answer = run(
                                            rs,
                                            Query {
                                                constraints: vec![
                                                    c("start", [v(0)]),
                                                    c("watch", [v(0)]),
                                                    c("token", []),
                                                ],
                                                outputs,
                                            },
                                        );
                                        assert_eq!(answer.len(), 1);
                                        strip(answer.pop().unwrap())
                                    })
                                    .collect::<Vec<_>>();
                                assert_eq!(actual, expected);
                                // Every answer-boundary cancellation, including zero and exhaustion.
                                for cutoff in 0..=ordered.len() {
                                    let held = Ordered::new(&domain, n, visible, &weights, padding)
                                        .take(cutoff)
                                        .collect::<Vec<_>>();
                                    assert_eq!(held, source_indices[..cutoff]);
                                }
                                assert_eq!(
                                    Ordered::new(&domain, n, visible, &weights, padding)
                                        .collect::<Vec<_>>(),
                                    ordered
                                );
                                cartesian += 4usize.pow(n as u32);
                                admitted += ordered.len();
                                cases += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(cases, 192);
    assert_eq!((cartesian, admitted), (26_112, 9_600));
    eprintln!("cases={cases} source_assignments={cartesian} admitted_assignments={admitted}");
}
