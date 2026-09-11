#[cfg(feature = "alloc-meter")]
#[allow(unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "support/projection_cost.rs"]
mod runtime;
use chr_structural::{
    joint_region::{Observation, Predicate, Region},
    projection::{Problem, Relation},
};
use chr_syntax::{Term, Var, atom, v};
use std::time::Instant;
type Record = (Vec<Term>, u128);
struct Row {
    phase: &'static str,
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn measure<T>(rows: &mut Vec<Row>, phase: &'static str, f: impl FnOnce() -> T) -> T {
    #[cfg(feature = "alloc-meter")]
    let start = meter::begin();
    let clock = Instant::now();
    let value = f();
    let ns = clock.elapsed().as_nanos();
    #[cfg(feature = "alloc-meter")]
    let memory = meter::end(start);
    rows.push(Row {
        phase,
        ns,
        #[cfg(feature = "alloc-meter")]
        memory,
    });
    value
}
fn source(family: &str, n: usize, duplicate: bool) -> Region {
    let mut domain = vec![atom("a"), atom("b"), atom("c")];
    if duplicate {
        domain.push(atom("a"));
    }
    let predicates = (0..n)
        .flat_map(|a| {
            (a + 1..n)
                .filter(move |_| family == "clique" || a == 0)
                .map(move |b| {
                    if family == "different" {
                        Predicate::Different(v(a as u64), v(b as u64))
                    } else {
                        Predicate::Equal(v(a as u64), v(b as u64))
                    }
                })
        })
        .collect();
    Region {
        domains: (0..n).map(|i| (Var(i as u64), domain.clone())).collect(),
        predicates,
    }
}
// Numeric control lowering retains every domain alternative. Relation creation
// and this temporary representation's disposal are inside preparation.
fn lower(r: &Region) -> Problem {
    let encode = |t: &Term| {
        let Term::App(name, args) = t else {
            panic!("ground source")
        };
        assert!(args.is_empty());
        match name.as_str() {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            _ => panic!("source name"),
        }
    };
    Problem {
        domains: r
            .domains
            .values()
            .map(|d| d.iter().map(encode).collect())
            .collect(),
        filters: r
            .predicates
            .iter()
            .map(|p| {
                let (Predicate::Equal(Term::Var(a), Term::Var(b))
                | Predicate::Different(Term::Var(a), Term::Var(b))) = p
                else {
                    panic!("source predicate")
                };
                Relation {
                    scope: vec![a.0 as usize, b.0 as usize],
                    rows: (0..3)
                        .flat_map(|x| {
                            (0..3)
                                .filter(move |y| {
                                    if matches!(p, Predicate::Equal(..)) {
                                        x == *y
                                    } else {
                                        x != *y
                                    }
                                })
                                .map(move |y| vec![x, y])
                        })
                        .collect(),
                }
            })
            .collect(),
    }
}
fn decode(row: Vec<u8>, alias: bool) -> Vec<Term> {
    let mut out = Vec::with_capacity(row.len() + usize::from(alias));
    out.extend(row.into_iter().map(|v| atom(["a", "b", "c"][v as usize])));
    if alias {
        out.push(out[0].clone());
    }
    out
}
enum Prepared {
    Projection(chr_structural::joint_region::Prepared),
    Enumeration(runtime::Prepared),
}
enum Output<'a> {
    Projection(std::collections::btree_map::IntoIter<Vec<Term>, u128>),
    Enumeration(runtime::Output<'a>, bool),
}
impl Iterator for Output<'_> {
    type Item = Record;
    fn next(&mut self) -> Option<Record> {
        match self {
            Self::Projection(x) => x.next(),
            Self::Enumeration(x, alias) => x.next().map(|(row, w)| (decode(row, *alias), w)),
        }
    }
}
impl Prepared {
    fn new(mode: &str, r: &Region, visible: &[Var]) -> Self {
        match mode {
            "sparse" => Self::Projection(
                r.prepare_sparse(visible, &[], &[], Observation::Counted, 100_000)
                    .unwrap(),
            ),
            "projection" => Self::Projection(
                r.prepare(visible, &[], &[], Observation::Counted, 100_000)
                    .unwrap(),
            ),
            "enumerate" => Self::Enumeration(runtime::Prepared::new(
                "enumerate",
                &lower(r),
                &visible[..2]
                    .iter()
                    .map(|x| x.0 as usize)
                    .collect::<Vec<_>>(),
            )),
            _ => panic!("mode"),
        }
    }
    fn start(&self, restrictions: &[(Var, Term)], alias: bool) -> Output<'_> {
        match self {
            Self::Projection(p) => Output::Projection(
                p.weighted_answers(restrictions, 100_000)
                    .unwrap()
                    .into_iter(),
            ),
            Self::Enumeration(p) => Output::Enumeration(
                p.start(
                    restrictions
                        .first()
                        .map(|(_, t)| if *t == atom("a") { 0 } else { 1 }),
                    false,
                ),
                alias,
            ),
        }
    }
}
fn main() {
    if cfg!(feature = "metrics") {
        panic!("metrics-off required");
    }
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 9);
    let (mode, family) = (&*args[1], &*args[2]);
    assert!(matches!(family, "star" | "clique" | "different"));
    let n = args[3].parse::<usize>().unwrap();
    assert!(matches!(n, 4 | 6));
    let flag = |i: usize| match &*args[i] {
        "0" => false,
        "1" => true,
        _ => panic!("flag"),
    };
    let duplicate = flag(4);
    let alias = flag(5);
    let queries = args[6].parse::<usize>().unwrap();
    assert!(matches!(queries, 1 | 4));
    let keep = flag(7);
    let cancel = flag(8);
    let mut visible = vec![Var((n - 2) as u64), Var((n - 1) as u64)];
    if alias {
        visible.push(visible[0]);
    }
    let restriction = |q| {
        if q == 1 || q == 2 {
            vec![(visible[0], atom(if q == 1 { "a" } else { "b" }))]
        } else {
            vec![]
        }
    };
    let r = source(family, n, duplicate);
    let p = lower(&r);
    let expected = (0..queries)
        .map(|q| {
            runtime::oracle(
                &p,
                &[n - 2, n - 1],
                if q == 1 || q == 2 {
                    Some((q - 1) as u8)
                } else {
                    None
                },
            )
            .into_iter()
            .map(|(row, w)| (decode(row, alias), w))
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let preflight = Prepared::new(mode, &r, &visible);
    for (q, want) in expected.iter().enumerate() {
        assert_eq!(
            &preflight.start(&restriction(q), alias).collect::<Vec<_>>(),
            want
        );
    }
    drop((preflight, p, r));
    let mut clocks = (0..1000)
        .map(|_| {
            let c = Instant::now();
            c.elapsed().as_nanos()
        })
        .collect::<Vec<_>>();
    clocks.sort_unstable();
    let floor = 100 * clocks[500];
    drop(clocks);
    let mut rows = Vec::with_capacity(64);
    #[cfg(feature = "alloc-meter")]
    let owner = meter::begin();
    let mut held = measure(&mut rows, "consumer_create", || Vec::with_capacity(queries));
    let r = measure(&mut rows, "source", || source(family, n, duplicate));
    let prepared = measure(&mut rows, "prepare", || Prepared::new(mode, &r, &visible));
    #[cfg(feature = "phase-clock")]
    let preparation_ns = match &prepared {
        Prepared::Projection(p) => p.preparation_ns,
        _ => [0; 4],
    };
    measure(&mut rows, "source_dispose", || drop(r));
    for (q, want) in expected.iter().enumerate() {
        let input = measure(&mut rows, "input", || restriction(q));
        let mut output = measure(&mut rows, "setup", || prepared.start(&input, alias));
        let mut consumed = measure(&mut rows, "consumer_buffer", || Vec::with_capacity(9));
        measure(&mut rows, "first", || {
            if let Some(x) = output.next() {
                consumed.push(x)
            }
        });
        measure(&mut rows, "remaining", || {
            if !cancel {
                consumed.extend(output.by_ref());
            }
        });
        measure(&mut rows, "engine_dispose", || drop(output));
        measure(&mut rows, "input_dispose", || drop(input));
        assert_eq!(
            consumed,
            want[..if cancel {
                want.len().min(1)
            } else {
                want.len()
            }]
        );
        measure(&mut rows, "consume", || {
            if keep {
                held.push(consumed)
            } else {
                drop(consumed)
            }
        });
    }
    measure(&mut rows, "prepared_dispose", || drop(prepared));
    if keep {
        for (q, actual) in held.iter().enumerate() {
            assert_eq!(
                actual,
                &expected[q][..if cancel {
                    expected[q].len().min(1)
                } else {
                    expected[q].len()
                }]
            );
        }
    }
    measure(&mut rows, "consumer_dispose", || drop(held));
    #[cfg(feature = "alloc-meter")]
    {
        let total = meter::end(owner);
        assert_eq!(total.live_start, total.live_end);
    }
    println!(
        "{{\"total_ns\":{},\"clock_floor\":{},\"metered\":{}}}",
        rows.iter().map(|r| r.ns).sum::<u128>(),
        floor,
        cfg!(feature = "alloc-meter")
    );
    #[cfg(feature = "phase-clock")]
    println!("{{\"preparation_ns\":{preparation_ns:?}}}");
    for row in rows {
        #[cfg(feature = "alloc-meter")]
        println!(
            "{{\"phase\":\"{}\",\"ns\":{},\"memory\":{}}}",
            row.phase,
            row.ns,
            row.memory.json()
        );
        #[cfg(not(feature = "alloc-meter"))]
        println!("{{\"phase\":\"{}\",\"ns\":{}}}", row.phase, row.ns);
    }
}
