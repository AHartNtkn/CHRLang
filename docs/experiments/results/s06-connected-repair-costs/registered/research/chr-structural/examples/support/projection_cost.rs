use chr_structural::finite::{Grammar, Request, Search, Transition};
use chr_structural::projection::{Problem, Projection, Relation, Semantics};
use chr_syntax::Term;
use std::collections::BTreeMap;
pub type Record = (Vec<u8>, u128);
pub const MODES: [&str; 6] = [
    "ascending",
    "descending",
    "greedy",
    "enumerate",
    "separable",
    "structural",
];
pub fn source(family: &str, n: usize) -> (Problem, Vec<usize>) {
    assert!(matches!(n, 4 | 10));
    let domains = vec![vec![0, 1]; n];
    match family {
        "independent" => (
            Problem {
                domains,
                filters: vec![],
            },
            vec![0],
        ),
        "dense" => (
            Problem {
                domains,
                filters: vec![],
            },
            (0..n).collect(),
        ),
        "star" => (
            Problem {
                domains,
                filters: (1..n)
                    .map(|i| Relation {
                        scope: vec![0, i],
                        rows: vec![vec![0, 0], vec![0, 1], vec![1, 0]],
                    })
                    .collect(),
            },
            vec![n - 2, n - 1],
        ),
        _ => panic!("unknown family"),
    }
}
pub fn oracle(p: &Problem, visible: &[usize], restriction: Option<u8>) -> Vec<Record> {
    let mut out = BTreeMap::new();
    for mut word in 0..p.domains.iter().map(Vec::len).product::<usize>() {
        let mut row = vec![0; p.domains.len()];
        for i in (0..row.len()).rev() {
            row[i] = p.domains[i][word % p.domains[i].len()];
            word /= p.domains[i].len();
        }
        if restriction.is_some_and(|v| row[visible[0]] != v)
            || p.filters.iter().any(|r| {
                !r.rows
                    .iter()
                    .any(|v| r.scope.iter().zip(v).all(|(&i, &x)| row[i] == x))
            })
        {
            continue;
        }
        *out.entry(visible.iter().map(|&i| row[i]).collect())
            .or_insert(0) += 1;
    }
    out.into_iter().collect()
}
pub fn expand(records: Vec<Record>) -> Vec<Record> {
    records
        .into_iter()
        .flat_map(|(v, w)| (0..w).map(move |_| (v.clone(), 1)))
        .collect()
}
pub struct EnumPlan {
    domains: Vec<Vec<(u8, u128)>>,
    order: Vec<usize>,
    ready: Vec<Vec<Relation>>,
    visible: Vec<usize>,
}
impl EnumPlan {
    fn new(p: &Problem, visible: &[usize]) -> Self {
        let domains = p
            .domains
            .iter()
            .map(|d| {
                let mut counts = BTreeMap::new();
                for &v in d {
                    *counts.entry(v).or_insert(0) += 1;
                }
                counts.into_iter().collect()
            })
            .collect();
        let order = visible
            .iter()
            .copied()
            .chain((0..p.domains.len()).filter(|i| !visible.contains(i)))
            .collect::<Vec<_>>();
        let mut ready = vec![vec![]; order.len()];
        for r in &p.filters {
            let last = r
                .scope
                .iter()
                .map(|i| order.iter().position(|j| i == j).unwrap())
                .max()
                .unwrap();
            ready[last].push(r.clone());
        }
        Self {
            domains,
            order,
            ready,
            visible: visible.to_vec(),
        }
    }
}
pub struct Enumeration<'a> {
    plan: &'a EnumPlan,
    next: Vec<usize>,
    values: Vec<u8>,
    weights: Vec<u128>,
    depth: usize,
    done: bool,
    restriction: Option<u8>,
    steps: usize,
}
impl<'a> Enumeration<'a> {
    fn new(plan: &'a EnumPlan, restriction: Option<u8>, scale: u128) -> Self {
        let n = plan.order.len();
        let mut weights = vec![0; n + 1];
        weights[0] = scale;
        Self {
            plan,
            next: vec![0; n],
            values: vec![0; n],
            weights,
            depth: 0,
            done: scale == 0,
            restriction,
            steps: 0,
        }
    }
    fn leaf(&mut self) -> Option<Record> {
        while !self.done {
            self.steps += 1;
            assert!(self.steps <= 5_000_000, "enumeration service bound");
            if self.depth == self.plan.order.len() {
                let row = self.plan.visible.iter().map(|&i| self.values[i]).collect();
                let weight = self.weights[self.depth];
                if self.depth == 0 {
                    self.done = true;
                } else {
                    self.depth -= 1;
                }
                return Some((row, weight));
            }
            let i = self.plan.order[self.depth];
            if self.next[self.depth] == self.plan.domains[i].len() {
                self.next[self.depth] = 0;
                if self.depth == 0 {
                    self.done = true;
                } else {
                    self.depth -= 1;
                }
                continue;
            }
            let (value, weight) = self.plan.domains[i][self.next[self.depth]];
            self.next[self.depth] += 1;
            if i == self.plan.visible[0] && self.restriction.is_some_and(|v| v != value) {
                continue;
            }
            self.values[i] = value;
            if self.plan.ready[self.depth].iter().any(|r| {
                !r.rows
                    .iter()
                    .any(|row| r.scope.iter().zip(row).all(|(&j, &v)| self.values[j] == v))
            }) {
                continue;
            }
            self.weights[self.depth + 1] = self.weights[self.depth].checked_mul(weight).unwrap();
            self.depth += 1;
        }
        None
    }
}
pub enum Prepared {
    Projected(Projection, usize),
    Enumerated(EnumPlan),
    Separable(EnumPlan, u128),
    Structural(Grammar, Vec<usize>, bool),
}
impl Prepared {
    pub fn new(mode: &str, p: &Problem, visible: &[usize]) -> Self {
        match mode {
            "ascending" | "descending" | "greedy" => {
                let mut order = (0..p.domains.len())
                    .filter(|i| !visible.contains(i))
                    .collect::<Vec<_>>();
                if mode == "descending" {
                    order.reverse();
                }
                if mode == "greedy" {
                    order = p.elimination_order(visible).unwrap();
                }
                Self::Projected(
                    p.project(visible, &[], &order, Semantics::Counted, 100_000)
                        .unwrap(),
                    visible[0],
                )
            }
            "enumerate" => Self::Enumerated(EnumPlan::new(p, visible)),
            "separable" => {
                assert!(
                    p.filters.is_empty(),
                    "separability control excludes relational filters"
                );
                let scale = (0..p.domains.len())
                    .filter(|i| !visible.contains(i))
                    .map(|i| p.domains[i].len() as u128)
                    .try_fold(1u128, |a, b| a.checked_mul(b))
                    .unwrap();
                let q = Problem {
                    domains: visible.iter().map(|&i| p.domains[i].clone()).collect(),
                    filters: vec![],
                };
                Self::Separable(
                    EnumPlan::new(&q, &(0..visible.len()).collect::<Vec<_>>()),
                    scale,
                )
            }
            "structural" => {
                // Boolean pilot lowering. Source domains and star membership remain separate.
                assert!(p.domains.iter().all(|d| d == &[0, 1]));
                let n = p.domains.len();
                let mut states = vec![
                    vec![Transition::new("a", [])],
                    vec![Transition::new("b", [])],
                    vec![Transition::new("a", []), Transition::new("b", [])],
                    vec![Transition::new("tuple", vec![2; n])],
                ];
                let star = !p.filters.is_empty();
                if star {
                    assert_eq!(p.filters.len(), n - 1);
                    for (i, r) in p.filters.iter().enumerate() {
                        assert_eq!(r.scope, vec![0, i + 1]);
                        assert_eq!(r.rows, vec![vec![0, 0], vec![0, 1], vec![1, 0]]);
                    }
                }
                let mut zero = vec![2; n];
                zero[0] = 0;
                let mut one = vec![0; n];
                one[0] = 1;
                states.push(if star {
                    vec![
                        Transition::new("tuple", zero),
                        Transition::new("tuple", one),
                    ]
                } else {
                    vec![Transition::new("tuple", vec![2; n])]
                });
                for v in 0..2 {
                    let mut children = vec![2; n];
                    children[visible[0]] = v;
                    states.push(vec![Transition::new("tuple", children)]);
                }
                Self::Structural(Grammar::new(states).unwrap(), visible.to_vec(), star)
            }
            _ => panic!("unknown mode"),
        }
    }
    pub fn start(&self, restriction: Option<u8>, expanded: bool) -> Output<'_> {
        let source = match self {
            Self::Projected(p, i) => {
                let r = restriction.map(|v| vec![(*i, v)]).unwrap_or_default();
                Source::Map(p.answers(&r, 100_000).unwrap().into_iter())
            }
            Self::Enumerated(p) => Source::Enumeration(Enumeration::new(p, restriction, 1)),
            Self::Separable(p, w) => Source::Enumeration(Enumeration::new(p, restriction, *w)),
            Self::Structural(g, visible, star) => {
                let mut roots = vec![3];
                if *star {
                    roots.push(4);
                }
                if let Some(v) = restriction {
                    roots.push(5 + v as usize);
                }
                let mut search = Search::new(
                    g,
                    Request {
                        roots,
                        equalities: vec![],
                    },
                )
                .unwrap();
                let mut map = BTreeMap::new();
                let mut steps = 0;
                loop {
                    let batch = search.advance(1).unwrap();
                    steps += 1;
                    assert!(steps <= 5_000_000, "structural service bound");
                    for answer in batch.answers {
                        let Term::App(_, args) = answer.term else {
                            panic!("tuple")
                        };
                        let key = visible
                            .iter()
                            .map(|&i| match &args[i] {
                                Term::App(s, a) if a.is_empty() && s == "a" => 0,
                                Term::App(s, a) if a.is_empty() && s == "b" => 1,
                                _ => panic!("Boolean"),
                            })
                            .collect::<Vec<_>>();
                        let count = map.entry(key).or_insert(0u128);
                        *count = count.checked_add(answer.multiplicity).unwrap();
                    }
                    if batch.exhausted {
                        break;
                    }
                }
                Source::Map(map.into_iter())
            }
        };
        Output {
            source,
            pending: None,
            repeat: None,
            expanded,
        }
    }
}
enum Source<'a> {
    Map(std::collections::btree_map::IntoIter<Vec<u8>, u128>),
    Enumeration(Enumeration<'a>),
}
impl Source<'_> {
    fn next(&mut self) -> Option<Record> {
        match self {
            Self::Map(m) => m.next(),
            Self::Enumeration(e) => e.leaf(),
        }
    }
}
pub struct Output<'a> {
    source: Source<'a>,
    pending: Option<Record>,
    repeat: Option<Record>,
    expanded: bool,
}
impl Iterator for Output<'_> {
    type Item = Record;
    fn next(&mut self) -> Option<Record> {
        if self.expanded {
            if self.repeat.is_none() {
                self.repeat = self.source.next();
            }
            let (row, w) = self.repeat.as_mut()?;
            if *w == 1 {
                return self.repeat.take();
            }
            *w -= 1;
            return Some((row.clone(), 1));
        }
        let (row, mut weight) = self.pending.take().or_else(|| self.source.next())?;
        // Map records are already aggregated; do not add lookahead work to that path.
        if matches!(self.source, Source::Map(_)) {
            return Some((row, weight));
        }
        while let Some((next, w)) = self.source.next() {
            if next != row {
                self.pending = Some((next, w));
                break;
            }
            weight = weight.checked_add(w).unwrap();
        }
        Some((row, weight))
    }
}
