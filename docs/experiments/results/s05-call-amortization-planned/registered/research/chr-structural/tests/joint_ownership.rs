//! Matched finite-name observations; allocation qualification only.
#[allow(dead_code, unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
use chr_structural::{
    joint_region::{Observation, Predicate, Prepared as Projected, Region},
    name_disequality::{Formula, Name},
    normal_forms::Requirement,
    symbolic::{Answer, Fresh, Instance},
};
use chr_syntax::{Term, Var, atom, t, v};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone)]
struct Model {
    n: usize,
    alphabet: Vec<String>,
    edges: Vec<(usize, usize)>,
}
#[derive(Debug, PartialEq, Eq)]
enum Output {
    Member(bool),
    Rows(BTreeSet<Vec<Term>>),
}
fn model(family: &str, n: usize, size: usize) -> Model {
    let edges = match family {
        "free" => vec![],
        "star" => (2..n).flat_map(|h| [(h, 0), (h, 1)]).collect(),
        "clique" => (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .collect(),
        _ => panic!(),
    };
    Model {
        n,
        alphabet: (0..size).map(|i| format!("a{i}")).collect(),
        edges,
    }
}
fn oracle(m: &Model, x: usize, y: Option<usize>) -> Output {
    let mut rows = BTreeSet::new();
    let size = m.alphabet.len();
    for mut k in 0..size.pow(m.n as u32) {
        let mut values = vec![0; m.n];
        for v in &mut values {
            *v = k % size;
            k /= size;
        }
        if values[0] == x
            && y.is_none_or(|y| values[1] == y)
            && m.edges.iter().all(|(a, b)| values[*a] != values[*b])
        {
            rows.insert(vec![
                atom(&m.alphabet[values[0]]),
                atom(&m.alphabet[values[1]]),
            ]);
        }
    }
    if y.is_some() {
        Output::Member(!rows.is_empty())
    } else {
        Output::Rows(rows)
    }
}
fn enumerate(m: &Model, given: &BTreeMap<usize, String>) -> bool {
    fn go(m: &Model, given: &BTreeMap<usize, String>, row: &mut Vec<String>) -> bool {
        if row.len() == m.n {
            return true;
        }
        let i = row.len();
        for a in &m.alphabet {
            if given.get(&i).is_some_and(|v| v != a) {
                continue;
            }
            if m.edges.iter().any(|(x, y)| {
                (*x == i && *y < i && row[*y] == *a) || (*y == i && *x < i && row[*x] == *a)
            }) {
                continue;
            }
            row.push(a.clone());
            if go(m, given, row) {
                return true;
            }
            row.pop();
        }
        false
    }
    go(m, given, &mut Vec::with_capacity(m.n))
}
#[allow(clippy::large_enum_variant)]
enum Prepared {
    Symbolic(Answer, Fresh),
    Projected(Projected),
    Names(Formula),
    Explicit(Model),
}
fn prepare(mode: &str, m: &Model) -> Prepared {
    match mode {
        "symbolic" => Prepared::Symbolic(
            Answer {
                imports: BTreeSet::from([Var(0)]),
                outputs: vec![v(0), v(1)],
                names: (0..m.n).map(|i| v(i as u64)).collect(),
                structure: vec![(t("app", [v(0), v(1)]), Requirement::Normal)],
                unequal: m
                    .edges
                    .iter()
                    .map(|(a, b)| (v(*a as u64), v(*b as u64)))
                    .collect(),
                alphabet: Some(m.alphabet.clone()),
            },
            Fresh {
                next: Some(0),
                occupied: BTreeSet::new(),
            },
        ),
        "projected" => {
            let mut predicates = (0..m.n)
                .map(|i| Predicate::Name(v(i as u64)))
                .collect::<Vec<_>>();
            predicates.push(Predicate::Structure(
                t("app", [v(0), v(1)]),
                Requirement::Normal,
            ));
            predicates.extend(
                m.edges
                    .iter()
                    .map(|(a, b)| Predicate::Different(v(*a as u64), v(*b as u64))),
            );
            Prepared::Projected(
                Region {
                    domains: (0..m.n)
                        .map(|i| (Var(i as u64), m.alphabet.iter().map(|n| atom(n)).collect()))
                        .collect(),
                    predicates,
                }
                .prepare(
                    &[Var(0), Var(1)],
                    &[],
                    &[],
                    Observation::LogicalSet,
                    100_000,
                )
                .unwrap(),
            )
        }
        "names" => Prepared::Names(
            Formula::compile(
                m.n,
                &[],
                &m.edges
                    .iter()
                    .map(|(a, b)| (Name::Variable(*a), Name::Variable(*b)))
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
        ),
        "explicit" => Prepared::Explicit(m.clone()),
        _ => panic!(),
    }
}
fn instance(p: &mut Prepared, caller: &BTreeMap<Var, Var>) -> Option<Instance> {
    if let Prepared::Symbolic(a, f) = p {
        Some(a.instantiate(caller, f).unwrap())
    } else {
        None
    }
}
fn evaluate(
    p: &Prepared,
    instance: Option<&Instance>,
    m: &Model,
    x: usize,
    y: Option<usize>,
) -> Output {
    if let Prepared::Projected(p) = p {
        let mut given = vec![(Var(0), atom(&m.alphabet[x]))];
        if let Some(y) = y {
            given.push((Var(1), atom(&m.alphabet[y])))
        }
        let rows = p.answers(&given, 100_000).unwrap();
        return if y.is_some() {
            Output::Member(!rows.is_empty())
        } else {
            Output::Rows(rows)
        };
    }
    let accepts = |y: usize| {
        let given = BTreeMap::from([(0, m.alphabet[x].clone()), (1, m.alphabet[y].clone())]);
        match p {
            Prepared::Symbolic(_, _) => {
                let i = instance.unwrap();
                let ids = i
                    .outputs()
                    .iter()
                    .map(|t| {
                        let Term::Var(x) = t else { panic!() };
                        *x
                    })
                    .collect::<Vec<_>>();
                i.consistent(&BTreeMap::from([
                    (ids[0], atom(&m.alphabet[x])),
                    (ids[1], atom(&m.alphabet[y])),
                ]))
                .unwrap()
            }
            Prepared::Names(f) => f.finite(&m.alphabet, &given).satisfiable,
            Prepared::Explicit(m) => enumerate(m, &given),
            _ => unreachable!(),
        }
    };
    if let Some(y) = y {
        Output::Member(accepts(y))
    } else {
        Output::Rows(
            (0..m.alphabet.len())
                .filter(|y| accepts(*y))
                .map(|y| vec![atom(&m.alphabet[x]), atom(&m.alphabet[y])])
                .collect(),
        )
    }
}
fn measure<T>(f: impl FnOnce() -> T) -> (T, meter::Reading) {
    let start = meter::begin();
    let value = f();
    (value, meter::end(start))
}
fn main() {
    meter::self_check().unwrap();
    let args = std::env::args().collect::<Vec<_>>();
    let mode = &args[1];
    let n = args[3].parse().unwrap();
    let size = args[4].parse().unwrap();
    let queries: usize = args[5].parse().unwrap();
    let full = args[6] == "full";
    let retain = args[7] == "all";
    let m = model(&args[2], n, size);
    let expected = (0..queries)
        .map(|i| oracle(&m, i % size, (!full).then_some((i / size) % size)))
        .collect::<Vec<_>>();
    let callers = (0..queries)
        .map(|i| BTreeMap::from([(Var(0), Var(1_000_000 + i as u64))]))
        .collect::<Vec<_>>();
    {
        let mut p = prepare(mode, &m);
        for (i, e) in expected.iter().enumerate() {
            let instance = instance(&mut p, &callers[i]);
            assert_eq!(
                &evaluate(
                    &p,
                    instance.as_ref(),
                    &m,
                    i % size,
                    (!full).then_some((i / size) % size)
                ),
                e
            );
        }
    }
    let mut held = Vec::with_capacity(queries);
    let mut rows = Vec::with_capacity(queries * 4 + 5);
    let baseline = meter::end(meter::begin()).live_end;
    let (mut p, row) = measure(|| prepare(mode, &m));
    rows.push(("prepare", row));
    for (i, e) in expected.iter().enumerate() {
        let (instance, row) = measure(|| instance(&mut p, &callers[i]));
        rows.push(("transport", row));
        let (output, row) = measure(|| {
            evaluate(
                &p,
                instance.as_ref(),
                &m,
                i % size,
                (!full).then_some((i / size) % size),
            )
        });
        rows.push(("observe", row));
        assert_eq!(&output, e);
        let (_, row) = measure(|| drop(instance));
        rows.push(("instance-dispose", row));
        let (_, row) = measure(|| {
            if retain {
                held.push(output)
            } else {
                drop(output)
            }
        });
        rows.push(("consumer", row));
    }
    let (ids, row) = measure(|| {
        if let Prepared::Symbolic(_, fresh) = &mut p {
            let old = std::mem::replace(
                fresh,
                Fresh {
                    next: Some(0),
                    occupied: BTreeSet::new(),
                },
            );
            let n = old.occupied.len();
            drop(old);
            n
        } else {
            0
        }
    });
    rows.push(("identity-owner-dispose", row));
    let (_, row) = measure(|| drop(p));
    rows.push(("prepare-dispose", row));
    if retain {
        assert_eq!(held, expected)
    }
    let (_, row) = measure(|| held.clear());
    rows.push(("final-consumer-dispose", row));
    assert_eq!(row.live_end, baseline, "final owner restoration");
    println!("{{\"identity_records\":{ids}}}");
    for (phase, row) in rows {
        println!("{{\"phase\":\"{phase}\",\"allocation\":{}}}", row.json());
    }
}
