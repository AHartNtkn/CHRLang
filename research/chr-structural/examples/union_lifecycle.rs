//! Lifecycle of the qualified finite logical-set union; no raw source contract.
#[cfg(feature = "alloc-meter")]
#[allow(dead_code, unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[path = "support/useful_union_source.rs"]
mod source;
use chr_structural::{
    graph_simplification::Reduced,
    name_disequality::{Formula, Name},
    reusable_diagram::{Diagram, Expr},
};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    time::Instant,
};
const LIMIT: usize = 1_000_000;
type Branch = Vec<(usize, usize)>;
type Rows = BTreeSet<Vec<usize>>;
struct Model {
    n: usize,
    branches: Vec<Branch>,
}
fn model(family: &str, n: usize) -> Model {
    Model {
        n,
        branches: source::family(n, family),
    }
}
struct Request {
    caller: usize,
    member: Option<Vec<usize>>,
}
fn request(n: usize, q: usize, full: bool) -> Request {
    let mut code = q * 71 + 17;
    let member = (!full).then(|| {
        (0..n)
            .map(|_| {
                let x = code % 3;
                code /= 3;
                x
            })
            .collect()
    });
    Request {
        caller: q % 5,
        member,
    }
}
fn admitted(r: &Request, a: &[usize]) -> bool {
    match r.caller {
        0 => true,
        1 => a[0] == 0,
        2 => a[0] == 1,
        3 => a.len() < 2 || a[0] == a[1],
        _ => false,
    }
}
fn partial_caller(r: &Request, a: &[usize]) -> bool {
    a.is_empty() || admitted(r, a)
}
#[derive(Debug, PartialEq, Eq)]
enum Output {
    Member(bool),
    Rows(Rows),
}
fn oracle(m: &Model, r: &Request) -> Output {
    let mut rows = Rows::new();
    for mut code in 0..3usize.pow(m.n as u32) {
        let a = (0..m.n)
            .map(|_| {
                let x = code % 3;
                code /= 3;
                x
            })
            .collect::<Vec<_>>();
        if admitted(r, &a)
            && r.member.as_ref().is_none_or(|b| b == &a)
            && m.branches
                .iter()
                .any(|b| b.iter().all(|&(i, j)| a[i] != a[j]))
        {
            rows.insert(a);
        }
    }
    if r.member.is_some() {
        Output::Member(!rows.is_empty())
    } else {
        Output::Rows(rows)
    }
}
enum Prepared {
    Union(Diagram),
    Separate(Vec<Diagram>),
    Reduced(Reduced),
    Names(Vec<Formula>, Vec<String>),
    Explicit(Vec<Branch>),
}
fn prepare(mode: &str, m: &Model) -> Prepared {
    match mode {
        "union" | "separate" => {
            let ds = m
                .branches
                .iter()
                .map(|b| {
                    Diagram::compile(
                        m.n,
                        3,
                        &Expr::And(b.iter().map(|&(i, j)| Expr::Different(i, j)).collect()),
                        LIMIT,
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();
            if mode == "separate" {
                Prepared::Separate(ds)
            } else {
                let mut it = ds.into_iter();
                let first = it.next().unwrap();
                Prepared::Union(it.fold(first, |a, b| a.union(&b, LIMIT).unwrap()))
            }
        }
        "reduced" => Prepared::Reduced(
            Reduced::compile(m.n, 3, &m.branches, &(0..m.n).collect::<Vec<_>>(), LIMIT).unwrap(),
        ),
        "names" => Prepared::Names(
            m.branches
                .iter()
                .map(|b| {
                    Formula::compile(
                        m.n,
                        &[],
                        &b.iter()
                            .map(|&(i, j)| (Name::Variable(i), Name::Variable(j)))
                            .collect::<Vec<_>>(),
                    )
                    .unwrap()
                })
                .collect(),
            vec!["a0".into(), "a1".into(), "a2".into()],
        ),
        "explicit" => Prepared::Explicit(m.branches.clone()),
        "dedup" => {
            let mut bs = m.branches.clone();
            for b in &mut bs {
                for (i, j) in b.iter_mut() {
                    if *i > *j {
                        std::mem::swap(i, j);
                    }
                }
                b.sort_unstable();
                b.dedup();
            }
            bs.sort();
            bs.dedup();
            Prepared::Explicit(bs)
        }
        _ => panic!("mode"),
    }
}
fn names_accept(fs: &[Formula], alphabet: &[String], a: &[usize]) -> bool {
    let given = a
        .iter()
        .enumerate()
        .map(|(i, x)| (i, alphabet[*x].clone()))
        .collect::<BTreeMap<_, _>>();
    fs.iter().any(|f| f.finite(alphabet, &given).satisfiable)
}
fn contains(p: &Prepared, a: &[usize]) -> bool {
    match p {
        Prepared::Union(d) => d.contains(a).unwrap(),
        Prepared::Separate(ds) => ds.iter().any(|d| d.contains(a).unwrap()),
        Prepared::Reduced(d) => d.contains(a).unwrap(),
        Prepared::Names(fs, alphabet) => names_accept(fs, alphabet, a),
        Prepared::Explicit(bs) => bs.iter().any(|b| b.iter().all(|&(i, j)| a[i] != a[j])),
    }
}
fn generate(
    n: usize,
    r: &Request,
    valid: &impl Fn(&[usize]) -> bool,
    a: &mut Vec<usize>,
    emit: &mut impl FnMut(&[usize]) -> bool,
) -> bool {
    if !partial_caller(r, a) || !valid(a) {
        return true;
    }
    if a.len() == n {
        return emit(a);
    }
    for x in 0..3 {
        a.push(x);
        let go = generate(n, r, valid, a, emit);
        a.pop();
        if !go {
            return false;
        }
    }
    true
}
fn visit(p: &Prepared, n: usize, r: &Request, mut emit: impl FnMut(&[usize]) -> bool) -> bool {
    match p {
        Prepared::Union(d) => d
            .visit_assignments(LIMIT, |a| partial_caller(r, a), |a| emit(a))
            .unwrap(),
        Prepared::Separate(ds) => {
            for d in ds {
                if !d
                    .visit_assignments(LIMIT, |a| partial_caller(r, a), |a| emit(a))
                    .unwrap()
                {
                    return false;
                }
            }
            true
        }
        Prepared::Explicit(bs) => {
            for b in bs {
                if !generate(
                    n,
                    r,
                    &|a| {
                        b.iter()
                            .all(|&(i, j)| i >= a.len() || j >= a.len() || a[i] != a[j])
                    },
                    &mut Vec::with_capacity(n),
                    &mut emit,
                ) {
                    return false;
                }
            }
            true
        }
        Prepared::Reduced(d) => {
            for b in d.constraints() {
                if !generate(
                    n,
                    r,
                    &|a| {
                        b.iter().all(|e| {
                            e.left >= a.len()
                                || e.right >= a.len()
                                || (a[e.left] == a[e.right]) == e.equal
                        })
                    },
                    &mut Vec::with_capacity(n),
                    &mut emit,
                ) {
                    return false;
                }
            }
            true
        }
        Prepared::Names(fs, alphabet) => generate(
            n,
            r,
            &|a| names_accept(fs, alphabet, a),
            &mut Vec::with_capacity(n),
            &mut emit,
        ),
    }
}
fn evaluate(p: &Prepared, n: usize, r: &Request) -> Output {
    if let Some(a) = &r.member {
        Output::Member(admitted(r, a) && contains(p, a))
    } else {
        let mut rows = Rows::new();
        assert!(visit(p, n, r, |a| {
            rows.insert(a.to_vec());
            true
        }));
        Output::Rows(rows)
    }
}
struct Phase {
    label: &'static str,
    ns: Option<u128>,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn measure<T>(label: &'static str, f: impl FnOnce() -> T) -> (T, Phase) {
    #[cfg(feature = "alloc-meter")]
    let start = meter::begin();
    #[cfg(not(feature = "alloc-meter"))]
    let start = Instant::now();
    let v = std::hint::black_box(f());
    #[cfg(feature = "alloc-meter")]
    let memory = meter::end(start);
    #[cfg(feature = "alloc-meter")]
    let ns = None;
    #[cfg(not(feature = "alloc-meter"))]
    let ns = Some(start.elapsed().as_nanos());
    (
        v,
        Phase {
            label,
            ns,
            #[cfg(feature = "alloc-meter")]
            memory,
        },
    )
}
fn main() {
    if cfg!(feature = "metrics") {
        panic!("metrics-off required");
    }
    let args = std::env::args().collect::<Vec<_>>();
    if args[1] == "clock-check" {
        let mut samples = Vec::new();
        for _ in 0..10000 {
            let t = Instant::now();
            std::hint::black_box(());
            samples.push(t.elapsed().as_nanos());
        }
        samples.sort();
        println!(
            "{{\"median_ns\":{},\"p99_ns\":{}}}",
            samples[5000], samples[9900]
        );
        return;
    }
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    assert_eq!(
        args.len(),
        7,
        "mode family width queries member/full immediate/window/all"
    );
    let (mode, family) = (&args[1], &args[2]);
    let n = args[3].parse().unwrap();
    assert!([4, 8].contains(&n));
    let queries = args[4].parse::<usize>().unwrap();
    assert!([1, 8, 12, 13, 14, 15, 64].contains(&queries));
    let full = match args[5].as_str() {
        "member" => false,
        "full" => true,
        _ => panic!("endpoint"),
    };
    let retention = &args[6];
    assert!(["immediate", "window", "all"].contains(&retention.as_str()));
    let expected = {
        let m = model(family, n);
        (0..queries)
            .map(|q| oracle(&m, &request(n, q, full)))
            .collect::<Vec<_>>()
    };
    #[cfg(feature = "alloc-meter")]
    let preflight_baseline = meter::end(meter::begin()).live_end;
    // Qualify cancellation after first accepted row, then reuse the same prepared object.
    {
        let m = model(family, n);
        let p = prepare(mode, &m);
        for (q, want) in expected.iter().enumerate() {
            let r = request(n, q, full);
            if full {
                let mut prefix = None;
                let complete = visit(&p, n, &r, |a| {
                    prefix = Some(a.to_vec());
                    false
                });
                let Output::Rows(rows) = want else { panic!() };
                assert_eq!(complete, rows.is_empty());
                if let Some(a) = prefix {
                    assert!(rows.contains(&a));
                }
            }
            assert_eq!(&evaluate(&p, n, &r), want);
        }
    }
    #[cfg(feature = "alloc-meter")]
    assert_eq!(
        meter::end(meter::begin()).live_end,
        preflight_baseline,
        "preflight ownership restored"
    );
    let mut phases = Vec::with_capacity(queries * 4 + 5);
    let mut held = VecDeque::with_capacity(queries);
    #[cfg(feature = "alloc-meter")]
    let baseline = meter::end(meter::begin()).live_end;
    let (m, row) = measure("source", || model(family, n));
    phases.push(row);
    let (p, row) = measure("prepare", || prepare(mode, &m));
    phases.push(row);
    let (_, row) = measure("source-drop", || drop(m));
    phases.push(row);
    for (q, want) in expected.iter().enumerate() {
        let (r, row) = measure("request", || request(n, q, full));
        phases.push(row);
        let (output, row) = measure("execute-observe", || evaluate(&p, n, &r));
        phases.push(row);
        assert_eq!(&output, want);
        let (_, row) = measure("request-drop", || drop(r));
        phases.push(row);
        let (_, row) = measure("consumer", || match retention.as_str() {
            "immediate" => drop(output),
            "window" => {
                if held.len() == 2 {
                    held.pop_front();
                }
                held.push_back((q, output));
            }
            "all" => held.push_back((q, output)),
            _ => unreachable!(),
        });
        phases.push(row);
    }
    let (_, row) = measure("prepared-drop", || drop(p));
    phases.push(row);
    for (q, output) in &held {
        assert_eq!(output, &expected[*q]);
    }
    let (_, row) = measure("consumer-drop", || held.clear());
    #[cfg(feature = "alloc-meter")]
    assert_eq!(
        row.memory.live_end, baseline,
        "all owned allocation restored"
    );
    phases.push(row);
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"width\":{n},\"queries\":{queries},\"full\":{full},\"retention\":\"{retention}\",\"meter\":{}}}",
        cfg!(feature = "alloc-meter")
    );
    for row in phases {
        #[cfg(feature = "alloc-meter")]
        let memory = row.memory.json();
        #[cfg(not(feature = "alloc-meter"))]
        let memory = "null".to_string();
        println!(
            "{{\"phase\":\"{}\",\"ns\":{},\"memory\":{memory}}}",
            row.label,
            row.ns.map_or("null".into(), |x| x.to_string())
        );
    }
}
