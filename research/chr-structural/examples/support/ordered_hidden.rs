use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, atom, c, eq, or, v};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};
// Condition each star arm on the visible value; merge Cartesian streams by
// source occurrence indices. Duplicate values retain different indices.
type Pending = (usize, Vec<usize>, usize, Vec<usize>);
pub struct Ordered {
    axes: Vec<Vec<Vec<usize>>>,
    heap: BinaryHeap<Reverse<Pending>>,
    seen: BTreeSet<(usize, Vec<usize>)>,
    padding: usize,
}
impl Ordered {
    pub fn new(
        domain: &[Term],
        n: usize,
        visible: usize,
        weights: &BTreeMap<Vec<Term>, u128>,
        padding: usize,
    ) -> Self {
        Self::with_dense(domain, n, visible, weights, padding, false)
    }
    pub fn with_dense(
        domain: &[Term],
        n: usize,
        visible: usize,
        weights: &BTreeMap<Vec<Term>, u128>,
        padding: usize,
        dense: bool,
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
                        .filter_map(|(j, x)| {
                            (if i == visible {
                                x == &value[0]
                            } else {
                                dense || x != &value[0]
                            })
                            .then_some(j)
                        })
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
pub fn host(body: Goal, heads: Vec<Term>, priority: usize, visible: usize) -> Vec<Rule> {
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
pub fn source(
    domain: &[Term],
    n: usize,
    visible: usize,
    priority: usize,
    alias: bool,
    padding: usize,
) -> (Vec<Rule>, Query) {
    source_dense(domain, n, visible, priority, alias, padding, false)
}
pub fn source_dense(
    domain: &[Term],
    n: usize,
    visible: usize,
    priority: usize,
    alias: bool,
    padding: usize,
    dense: bool,
) -> (Vec<Rule>, Query) {
    source_mode(domain, n, visible, priority, alias, padding, (dense, true))
}
pub fn source_mode(
    domain: &[Term],
    n: usize,
    visible: usize,
    priority: usize,
    alias: bool,
    padding: usize,
    mode: (bool, bool),
) -> (Vec<Rule>, Query) {
    let (dense, tagged) = mode;
    let mut body = vec![];
    for i in 0..n {
        let leaves = domain
            .iter()
            .enumerate()
            .map(|(j, x)| {
                Goal::And(
                    std::iter::repeat_n(Goal::True, (j % 2) * padding)
                        .chain(vec![eq(v(i as u64), x.clone())])
                        .chain(tagged.then(|| eq(v((n + i) as u64), atom(&format!("i{j}")))))
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
        if i != visible && !dense {
            body.push(c("neq", [v(visible as u64), v(i as u64)]).into());
        }
    }
    let mut rs = host(
        Goal::And(body),
        (0..if tagged { 2 * n } else { n })
            .map(|i| v(i as u64))
            .collect(),
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
    outputs.extend(
        (0..if tagged { n } else { 0 }).map(|i| (format!("index{i}"), Var((n + i) as u64))),
    );
    (
        rs,
        Query {
            constraints: vec![
                c(
                    "start",
                    (0..if tagged { 2 * n } else { n })
                        .map(|i| v(i as u64))
                        .collect::<Vec<_>>(),
                ),
                c("watch", [v(visible as u64)]),
                c("token", []),
            ],
            outputs,
        },
    )
}
pub fn run(rs: Vec<Rule>, q: Query) -> Vec<Answer> {
    let mut search = chr_reference::Search::new(rs, q).unwrap();
    let batch = search.advance(1_000_000);
    assert!(batch.exhausted);
    batch.answers
}
pub fn strip(mut a: Answer) -> Answer {
    a.outputs.retain(|(name, _)| !name.starts_with("index"));
    a.residual.sort();
    a
}
pub fn bag(xs: impl IntoIterator<Item = Answer>) -> BTreeMap<Answer, usize> {
    let mut bag = BTreeMap::new();
    for x in xs {
        *bag.entry(strip(x)).or_default() += 1;
    }
    bag
}
