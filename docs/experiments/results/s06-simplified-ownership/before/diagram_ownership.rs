//! Isolated allocation qualification for reusable finite logical sets.
#[cfg(feature = "alloc-meter")]
#[allow(dead_code, unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
use chr_structural::{
    joint_region::{Observation, Predicate, Prepared as Projection, Region},
    name_disequality::{Formula, Name},
    reusable_diagram::{Diagram, Expr},
    symbolic::{Answer as Symbolic, Fresh, Instance},
};
use chr_syntax::{Term, Var, atom, v};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
const LIMIT: usize = 1_000_000;
type Branch = Vec<(usize, usize)>;
#[derive(Clone)]
struct Model {
    n: usize,
    alphabet: Vec<String>,
    branches: Vec<Branch>,
}
fn model(family: &str, n: usize, k: usize) -> Model {
    let star = (2..n).flat_map(|h| [(0, h), (1, h)]).collect::<Vec<_>>();
    let clique = (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
        .collect::<Vec<_>>();
    let branches = match family {
        "free" => vec![vec![]],
        "star" => vec![star],
        "clique" => vec![clique],
        "duplicate" => vec![vec![], vec![]],
        "overlap" => vec![vec![(0, 1)], vec![(0, 2)]],
        "union" => vec![star, clique],
        _ => panic!("unknown family"),
    };
    Model {
        n,
        alphabet: (0..k).map(|i| format!("a{i}")).collect(),
        branches,
    }
}
struct Request {
    restrictions: Vec<String>,
    alias: bool,
    member: Option<(usize, usize)>,
}
fn request(m: &Model, q: usize, full: bool) -> Request {
    Request {
        restrictions: match q % 5 {
            0 => vec![],
            1 => vec![m.alphabet[0].clone()],
            2 => vec![m.alphabet[1].clone()],
            3 => vec![m.alphabet[0].clone(), m.alphabet[1].clone()],
            _ => vec!["outside".into()],
        },
        alias: q % 2 == 1,
        member: (!full).then_some((
            q % m.alphabet.len(),
            (q / m.alphabet.len()) % m.alphabet.len(),
        )),
    }
}
#[derive(Debug, PartialEq, Eq)]
enum Output {
    Member(bool),
    Rows(BTreeSet<Vec<Term>>),
}
fn oracle(m: &Model, r: &Request) -> Output {
    let mut rows = BTreeSet::new();
    let k = m.alphabet.len();
    for edges in &m.branches {
        for mut code in 0..k.pow(m.n as u32) {
            let mut a = vec![0; m.n];
            for x in &mut a {
                *x = code % k;
                code /= k;
            }
            if (!r.alias || a[0] == a[1])
                && r.restrictions.iter().all(|x| x == &m.alphabet[a[0]])
                && r.member.is_none_or(|p| p == (a[0], a[1]))
                && edges.iter().all(|(x, y)| a[*x] != a[*y])
            {
                rows.insert(vec![atom(&m.alphabet[a[0]]), atom(&m.alphabet[a[1]])]);
            }
        }
    }
    if r.member.is_some() {
        Output::Member(!rows.is_empty())
    } else {
        Output::Rows(rows)
    }
}
#[allow(clippy::large_enum_variant)]
enum Prepared {
    Diagram(Diagram),
    Names(Vec<Formula>),
    Projected(Vec<Projection>),
    Symbolic(Vec<Symbolic>, Fresh),
    Explicit(Model),
}
fn prepare(mode: &str, m: &Model) -> Prepared {
    match mode {
        "diagram" => {
            let mut ds = m.branches.iter().map(|b| {
                Diagram::compile(
                    m.n,
                    m.alphabet.len(),
                    &Expr::And(b.iter().map(|(a, b)| Expr::Different(*a, *b)).collect()),
                    LIMIT,
                )
                .unwrap()
            });
            let first = ds.next().unwrap();
            Prepared::Diagram(
                ds.fold(first, |a, b| a.union(&b, LIMIT).unwrap())
                    .exists(&(2..m.n).collect::<Vec<_>>(), LIMIT)
                    .unwrap(),
            )
        }
        "names" => Prepared::Names(
            m.branches
                .iter()
                .map(|b| {
                    Formula::compile(
                        m.n,
                        &[],
                        &b.iter()
                            .map(|(a, b)| (Name::Variable(*a), Name::Variable(*b)))
                            .collect::<Vec<_>>(),
                    )
                    .unwrap()
                })
                .collect(),
        ),
        "projected" => Prepared::Projected(
            m.branches
                .iter()
                .map(|b| {
                    Region {
                        domains: (0..m.n)
                            .map(|i| (Var(i as u64), m.alphabet.iter().map(|x| atom(x)).collect()))
                            .collect(),
                        predicates: b
                            .iter()
                            .map(|(a, b)| Predicate::Different(v(*a as u64), v(*b as u64)))
                            .collect(),
                    }
                    .prepare(&[Var(0), Var(1)], &[], &[], Observation::LogicalSet, LIMIT)
                    .unwrap()
                })
                .collect(),
        ),
        "symbolic" => Prepared::Symbolic(
            m.branches
                .iter()
                .map(|b| Symbolic {
                    imports: BTreeSet::from([Var(0)]),
                    outputs: vec![v(0), v(1)],
                    names: (0..m.n).map(|i| v(i as u64)).collect(),
                    structure: vec![],
                    unequal: b
                        .iter()
                        .map(|(a, b)| (v(*a as u64), v(*b as u64)))
                        .collect(),
                    alphabet: Some(m.alphabet.clone()),
                })
                .collect(),
            Fresh {
                next: Some(0),
                occupied: BTreeSet::new(),
            },
        ),
        "explicit" => Prepared::Explicit(m.clone()),
        _ => panic!("unknown mode"),
    }
}
fn instantiate(p: &mut Prepared, q: usize) -> Vec<Instance> {
    if let Prepared::Symbolic(ss, f) = p {
        let caller = BTreeMap::from([(Var(0), Var(1_000_000 + q as u64))]);
        ss.iter()
            .map(|s| s.instantiate(&caller, f).unwrap())
            .collect()
    } else {
        vec![]
    }
}
fn explicit(m: &Model, x: usize, y: usize) -> bool {
    fn walk(m: &Model, edges: &Branch, a: &mut Vec<usize>) -> bool {
        if a.len() == m.n {
            return true;
        }
        let i = a.len();
        for z in 0..m.alphabet.len() {
            if edges.iter().any(|(u, v)| {
                (*u == i && *v < i && a[*v] == z) || (*v == i && *u < i && a[*u] == z)
            }) {
                continue;
            }
            a.push(z);
            if walk(m, edges, a) {
                return true;
            }
            a.pop();
        }
        false
    }
    m.branches.iter().any(|b| {
        !b.iter()
            .any(|(u, v)| *u < 2 && *v < 2 && [x, y][*u] == [x, y][*v])
            && walk(m, b, &mut vec![x, y])
    })
}
fn evaluate(p: &Prepared, instances: &[Instance], m: &Model, r: &Request) -> Output {
    // Projected paths request rows directly rather than re-enumerating hidden witnesses.
    if let Prepared::Projected(ps) = p {
        let mut given = r
            .restrictions
            .iter()
            .map(|a| (Var(0), atom(a)))
            .collect::<Vec<_>>();
        if let Some((x, y)) = r.member {
            given.extend([
                (Var(0), atom(&m.alphabet[x])),
                (Var(1), atom(&m.alphabet[y])),
            ]);
        }
        let mut rows = BTreeSet::new();
        for p in ps {
            rows.extend(
                p.answers(&given, LIMIT)
                    .unwrap()
                    .into_iter()
                    .filter(|a| !r.alias || a[0] == a[1]),
            );
        }
        return if r.member.is_some() {
            Output::Member(!rows.is_empty())
        } else {
            Output::Rows(rows)
        };
    }
    let mut assignment = if matches!(p, Prepared::Diagram(_)) {
        vec![0; m.n]
    } else {
        vec![]
    };
    let mut accepts = |x: usize, y: usize| {
        if (r.alias && x != y) || r.restrictions.iter().any(|a| a != &m.alphabet[x]) {
            return false;
        }
        match p {
            Prepared::Diagram(d) => {
                assignment[0] = x;
                assignment[1] = y;
                d.contains(&assignment).unwrap()
            }
            Prepared::Names(fs) => fs.iter().any(|f| {
                f.finite(
                    &m.alphabet,
                    &BTreeMap::from([(0, m.alphabet[x].clone()), (1, m.alphabet[y].clone())]),
                )
                .satisfiable
            }),
            Prepared::Symbolic(..) => instances.iter().any(|i| {
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
            }),
            Prepared::Explicit(m) => explicit(m, x, y),
            _ => unreachable!(),
        }
    };
    if let Some((x, y)) = r.member {
        Output::Member(accepts(x, y))
    } else {
        Output::Rows(
            (0..m.alphabet.len())
                .flat_map(|x| (0..m.alphabet.len()).map(move |y| (x, y)))
                .filter(|(x, y)| accepts(*x, *y))
                .map(|(x, y)| vec![atom(&m.alphabet[x]), atom(&m.alphabet[y])])
                .collect(),
        )
    }
}
struct Reading {
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn measure<T>(f: impl FnOnce() -> T) -> (T, Reading) {
    #[cfg(feature = "alloc-meter")]
    let start = meter::begin();
    let value = f();
    (
        value,
        Reading {
            #[cfg(feature = "alloc-meter")]
            memory: meter::end(start),
        },
    )
}
impl Reading {
    fn json(&self) -> String {
        #[cfg(feature = "alloc-meter")]
        {
            self.memory.json()
        }
        #[cfg(not(feature = "alloc-meter"))]
        {
            "null".into()
        }
    }
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args[1] == "meter-check" {
        #[cfg(feature = "alloc-meter")]
        {
            meter::self_check().unwrap();
            println!("meter self-check passed");
            return;
        }
        #[cfg(not(feature = "alloc-meter"))]
        panic!("meter-check requires allocation build");
    }
    let mode = &args[1];
    let n = args[3].parse().unwrap();
    let k = args[4].parse().unwrap();
    let queries: usize = args[5].parse().unwrap();
    let full = args[6] == "full";
    let retention = &args[7];
    assert!(["immediate", "window", "all"].contains(&retention.as_str()));
    let m = model(&args[2], n, k);
    let expected = (0..queries)
        .map(|q| oracle(&m, &request(&m, q, full)))
        .collect::<Vec<_>>();
    let mut held = VecDeque::with_capacity(queries);
    let mut phases = Vec::with_capacity(queries * 5 + 4);
    #[cfg(feature = "alloc-meter")]
    let baseline = meter::end(meter::begin()).live_end;
    let (mut p, row) = measure(|| prepare(mode, &m));
    phases.push(("prepare", row));
    for (q, want) in expected.iter().enumerate() {
        let (r, row) = measure(|| request(&m, q, full));
        phases.push(("request", row));
        let (instances, row) = measure(|| instantiate(&mut p, q));
        phases.push(("transport", row));
        let (output, row) = measure(|| evaluate(&p, &instances, &m, &r));
        phases.push(("observe", row));
        assert_eq!(&output, want, "query {q}");
        let (_, row) = measure(|| {
            drop(instances);
            drop(r);
        });
        phases.push(("query-dispose", row));
        let (_, row) = measure(|| match retention.as_str() {
            "immediate" => drop(output),
            "window" => {
                if held.len() == 4 {
                    held.pop_front();
                }
                held.push_back((q, output));
            }
            "all" => held.push_back((q, output)),
            _ => unreachable!(),
        });
        phases.push(("consumer", row));
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
    phases.push(("identity-dispose", row));
    let (_, row) = measure(|| drop(p));
    phases.push(("prepare-dispose", row));
    for (q, output) in &held {
        assert_eq!(output, &expected[*q]);
    }
    let (_, row) = measure(|| held.clear());
    #[cfg(feature = "alloc-meter")]
    assert_eq!(row.memory.live_end, baseline, "final owner restoration");
    phases.push(("consumer-dispose", row));
    println!(
        "{{\"queries\":{queries},\"identity_records\":{ids},\"meter\":{}}}",
        cfg!(feature = "alloc-meter")
    );
    for (phase, row) in phases {
        println!("{{\"phase\":\"{phase}\",\"memory\":{}}}", row.json());
    }
}
