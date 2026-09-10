//! Source correspondence: complete observations and branch counts precede projection.
use chr_structural::{
    joint_region::{Observation, Predicate, Prepared as Projection, Region},
    name_disequality::{Formula, Name},
    reusable_diagram::{Diagram, Expr},
    symbolic::{Answer as Symbolic, Fresh},
};
use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, atom, c, eq, or, v};
use std::collections::{BTreeMap, BTreeSet};
const LIMIT: usize = 1_000_000;
type Branch = Vec<(usize, usize)>;
type Rows = BTreeSet<Vec<Term>>;
fn families() -> Vec<Vec<Branch>> {
    let star = vec![(0, 2), (1, 2)];
    let clique = vec![(0, 1), (0, 2), (1, 2)];
    vec![
        vec![vec![]],
        vec![star.clone()],
        vec![clique.clone()],
        vec![vec![], vec![]],
        vec![vec![(0, 1)], vec![(0, 2)]],
        vec![star, clique],
    ]
}
fn disjunction(goals: Vec<Goal>) -> Goal {
    goals.into_iter().reduce(or).unwrap_or(Goal::Fail)
}
fn rules(branches: &[Branch], alphabet: &[String]) -> Vec<Rule> {
    let alternatives = branches
        .iter()
        .map(|edges| {
            let mut body = (0..3)
                .map(|i| disjunction(alphabet.iter().map(|a| eq(v(i), atom(a))).collect()))
                .collect::<Vec<_>>();
            body.extend(
                edges
                    .iter()
                    .map(|(a, b)| c("different", [v(*a as u64), v(*b as u64)]).into()),
            );
            Goal::And(body)
        })
        .collect();
    vec![
        Rule::simplify(
            "choose",
            [c("start", [v(0), v(1), v(2)])],
            disjunction(alternatives),
        ),
        Rule::simplify("reject-equal", [c("different", [v(0), v(0)])], Goal::Fail),
        Rule::simplify("caller-bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
    ]
}
fn source(
    rules: &[Rule],
    output: &[usize],
    alias: bool,
    request: &[String],
) -> (Vec<Answer>, usize) {
    let mut constraints = vec![c("start", [v(0), v(1), v(2)])];
    if alias {
        constraints.push(c("bind", [v(0), v(1)]));
    }
    constraints.extend(request.iter().map(|s| c("bind", [v(0), atom(s)])));
    let mut search = chr_reference::Search::new(
        rules.to_vec(),
        Query {
            constraints,
            outputs: output
                .iter()
                .enumerate()
                .map(|(i, x)| (format!("o{i}"), Var(*x as u64)))
                .collect(),
        },
    )
    .unwrap();
    let batch = search.advance(100_000);
    assert!(batch.exhausted, "source cutoff");
    (batch.answers, search.stats().completed_branches as usize)
}
fn oracle(
    branches: &[Branch],
    alphabet: &[String],
    output: &[usize],
    alias: bool,
    request: &[String],
) -> (BTreeMap<Vec<Term>, usize>, BTreeSet<Answer>) {
    let mut rows = BTreeMap::new();
    let mut observations = BTreeSet::new();
    for edges in branches {
        for x in alphabet {
            for y in alphabet {
                for h in alphabet {
                    let a = [x, y, h];
                    if (!alias || x == y)
                        && request.iter().all(|r| r == x)
                        && edges.iter().all(|(i, j)| a[*i] != a[*j])
                    {
                        *rows
                            .entry(output.iter().map(|i| atom(a[*i])).collect())
                            .or_insert(0) += 1;
                        let mut residual = edges
                            .iter()
                            .map(|(i, j)| c("different", [atom(a[*i]), atom(a[*j])]))
                            .collect::<Vec<_>>();
                        residual.sort();
                        observations.insert(Answer {
                            outputs: output
                                .iter()
                                .enumerate()
                                .map(|(i, x)| (format!("o{i}"), atom(a[*x])))
                                .collect(),
                            residual,
                        });
                    }
                }
            }
        }
    }
    (rows, observations)
}
struct Prepared {
    diagram: Diagram,
    names: Vec<Formula>,
    projected: Vec<Projection>,
    symbolic: Vec<Symbolic>,
    fresh: Fresh,
}
impl Prepared {
    fn new(branches: &[Branch], alphabet: &[String]) -> Self {
        let mut diagrams = branches.iter().map(|edges| {
            Diagram::compile(
                3,
                alphabet.len(),
                &Expr::And(edges.iter().map(|(a, b)| Expr::Different(*a, *b)).collect()),
                LIMIT,
            )
            .unwrap()
        });
        let first = diagrams.next().unwrap();
        let diagram = diagrams
            .fold(first, |a, b| a.union(&b, LIMIT).unwrap())
            .exists(&[2], LIMIT)
            .unwrap();
        let names = branches
            .iter()
            .map(|edges| {
                Formula::compile(
                    3,
                    &[],
                    &edges
                        .iter()
                        .map(|(a, b)| (Name::Variable(*a), Name::Variable(*b)))
                        .collect::<Vec<_>>(),
                )
                .unwrap()
            })
            .collect();
        let projected = branches
            .iter()
            .map(|edges| {
                Region {
                    domains: (0..3)
                        .map(|i| (Var(i), alphabet.iter().map(|a| atom(a)).collect()))
                        .collect(),
                    predicates: edges
                        .iter()
                        .map(|(a, b)| Predicate::Different(v(*a as u64), v(*b as u64)))
                        .collect(),
                }
                .prepare(&[Var(0), Var(1)], &[], &[], Observation::LogicalSet, LIMIT)
                .unwrap()
            })
            .collect();
        let symbolic = branches
            .iter()
            .map(|edges| Symbolic {
                imports: BTreeSet::from([Var(0)]),
                outputs: vec![v(0), v(1)],
                names: (0..3).map(v).collect(),
                structure: vec![],
                unequal: edges
                    .iter()
                    .map(|(a, b)| (v(*a as u64), v(*b as u64)))
                    .collect(),
                alphabet: Some(alphabet.to_vec()),
            })
            .collect();
        Self {
            diagram,
            names,
            projected,
            symbolic,
            fresh: Fresh {
                next: Some(0),
                occupied: BTreeSet::new(),
            },
        }
    }
    fn answers(
        &mut self,
        alphabet: &[String],
        output: &[usize],
        alias: bool,
        request: &[String],
        query: usize,
    ) -> [Rows; 4] {
        let caller = BTreeMap::from([(Var(0), Var(1000 + query as u64))]);
        let instances = self
            .symbolic
            .iter()
            .map(|s| s.instantiate(&caller, &mut self.fresh).unwrap())
            .collect::<Vec<_>>();
        let projected = self
            .projected
            .iter()
            .map(|p| {
                p.answers(
                    &request
                        .iter()
                        .map(|a| (Var(0), atom(a)))
                        .collect::<Vec<_>>(),
                    LIMIT,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let mut result: [Rows; 4] = std::array::from_fn(|_| BTreeSet::new());
        for (x, a) in alphabet.iter().enumerate() {
            for (y, b) in alphabet.iter().enumerate() {
                if (alias && x != y) || request.iter().any(|r| r != a) {
                    continue;
                }
                let values = [a, b];
                let row = output.iter().map(|i| atom(values[*i])).collect::<Vec<_>>();
                let accepted = [
                    self.diagram.contains(&[x, y, 0]).unwrap(),
                    self.names.iter().any(|f| {
                        f.finite(alphabet, &BTreeMap::from([(0, a.clone()), (1, b.clone())]))
                            .satisfiable
                    }),
                    projected
                        .iter()
                        .any(|p| p.contains(&vec![atom(a), atom(b)])),
                    instances.iter().any(|i| {
                        let ids = i
                            .outputs()
                            .iter()
                            .map(|t| {
                                let Term::Var(x) = t else { panic!() };
                                *x
                            })
                            .collect::<Vec<_>>();
                        i.consistent(&BTreeMap::from([(ids[0], atom(a)), (ids[1], atom(b))]))
                            .unwrap()
                    }),
                ];
                for (r, ok) in result.iter_mut().zip(accepted) {
                    if ok {
                        r.insert(row.clone());
                    }
                }
            }
        }
        result
    }
}
#[test]
fn raw_source_and_all_four_prepared_paths_match_independent_assignments() {
    let mut configurations = 0;
    let mut multiplicity_cases = 0;
    for k in [2, 3] {
        let alphabet = (0..k).map(|i| format!("a{i}")).collect::<Vec<_>>();
        let requests = [
            vec![],
            vec![alphabet[0].clone()],
            vec![alphabet[1].clone()],
            vec![alphabet[0].clone(), alphabet[1].clone()],
            vec!["outside".into()],
        ];
        for branches in families() {
            let rules = rules(&branches, &alphabet);
            for alias in [false, true] {
                for output in [vec![0, 1], vec![1, 0], vec![0, 0]] {
                    let mut prepared = Prepared::new(&branches, &alphabet);
                    for (query, request) in requests.iter().enumerate() {
                        let (expected, observations) =
                            oracle(&branches, &alphabet, &output, alias, request);
                        let (answers, completed) = source(&rules, &output, alias, request);
                        assert_eq!(completed, expected.values().sum::<usize>());
                        assert_eq!(
                            answers.into_iter().collect::<BTreeSet<_>>(),
                            observations,
                            "source k={k} branches={branches:?} alias={alias} output={output:?} request={request:?}"
                        );
                        let want = expected.keys().cloned().collect::<Rows>();
                        for (mode, result) in prepared
                            .answers(&alphabet, &output, alias, request, query)
                            .into_iter()
                            .enumerate()
                        {
                            assert_eq!(
                                result, want,
                                "mode={mode} k={k} branches={branches:?} alias={alias} output={output:?} request={request:?}"
                            );
                        }
                        if expected.values().sum::<usize>() > want.len() {
                            multiplicity_cases += 1;
                        }
                        configurations += 1;
                    }
                }
            }
        }
    }
    assert_eq!(configurations, 360);
    assert!(multiplicity_cases > 0);
    println!(
        "source_configurations={configurations} candidate_set_checks={} multiplicity_cases={multiplicity_cases}",
        configurations * 4
    );
}
#[test]
fn duplicate_work_and_residual_observation_are_distinct() {
    let alphabet = vec!["a".into(), "b".into()];
    let duplicate = rules(&[vec![], vec![]], &alphabet);
    let (raw, completed) = source(&duplicate, &[0, 1], false, &[]);
    assert_eq!(completed, 16);
    assert_eq!(raw.len(), 4);
    assert_eq!(
        raw.iter()
            .map(|a| a.outputs.clone())
            .collect::<BTreeSet<_>>()
            .len(),
        4
    );
    let mut observed = rules(&[vec![(0, 1)]], &alphabet);
    observed.push(Rule::propagate(
        "observe",
        [c("different", [v(0), v(1)])],
        c("seen", [v(0), v(1)]).into(),
    ));
    let (raw, _) = source(&observed, &[0, 1], false, &[]);
    assert!(!raw.is_empty());
    assert!(
        raw.iter()
            .all(|a| a.residual.iter().any(|c| c.name == "seen"))
    );
    // Emit the same visible logical set while retaining the observer rule.
    let mut replacement = observed.clone();
    replacement[0].body = or(
        Goal::And(vec![eq(v(0), atom("a")), eq(v(1), atom("b"))]),
        Goal::And(vec![eq(v(0), atom("b")), eq(v(1), atom("a"))]),
    );
    let (summary, _) = source(&replacement, &[0, 1], false, &[]);
    assert_eq!(
        raw.iter()
            .map(|a| a.outputs.clone())
            .collect::<BTreeSet<_>>(),
        summary.iter().map(|a| a.outputs.clone()).collect()
    );
    assert!(
        summary
            .iter()
            .all(|a| a.residual.iter().all(|c| c.name != "seen"))
    );
}
