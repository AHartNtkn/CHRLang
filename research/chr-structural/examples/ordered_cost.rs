#[path = "support/completed_caller.rs"]
mod completed;
#[cfg(feature = "alloc-meter")]
#[allow(unexpected_cfgs, dead_code)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "support/ordered_hidden.rs"]
mod runtime;
use chr_reuse::continuations::{Mode, Prepared, Search};
use chr_structural::joint_region::{Observation, Predicate, Region};
use chr_syntax::{Answer, Query, Term, Var, atom, c, eq, v};
use std::collections::BTreeMap;
use std::time::Instant;
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
// Keep the measured Direct state inline; boxing would add allocator work.
#[allow(clippy::large_enum_variant)]
enum Output {
    Projected(runtime::Ordered),
    Lazy(runtime::Ordered, bool),
    Enumerated(std::vec::IntoIter<Vec<usize>>),
    Direct(Search),
}
fn next(
    output: &mut Output,
    caller: &mut Option<completed::Caller>,
    domain: &[Term],
    reuse: bool,
) -> Option<Answer> {
    let row = match output {
        Output::Projected(x) => x.next(),
        Output::Lazy(x, dense) => {
            x.find(|row| *dense || row[1..].iter().all(|i| domain[*i] != domain[row[0]]))
        }
        Output::Enumerated(x) => x.next(),
        Output::Direct(x) => {
            for _ in 0..1_000_000 {
                let mut b = x.advance(1);
                if let Some(a) = b.answers.pop() {
                    return Some(runtime::strip(a));
                }
                if b.exhausted {
                    return None;
                }
            }
            panic!("source step bound");
        }
    }?;
    let q = Query {
        constraints: vec![
            c("start", [v(0), domain[row[0]].clone()]),
            c("watch", [v(0)]),
            c("token", []),
        ],
        outputs: vec![("out".into(), Var(0))],
    };
    let mut answers = caller.as_mut().unwrap().run(q, reuse, 10000).unwrap();
    assert_eq!(answers.len(), 1);
    Some(runtime::strip(answers.pop().unwrap()))
}
fn enumerate(
    domain: &[Term],
    n: usize,
    padding: usize,
    dense: bool,
) -> std::vec::IntoIter<Vec<usize>> {
    let mut rows = (0..4usize.pow(n as u32))
        .filter_map(|mut k| {
            let mut row = vec![0; n];
            for x in row.iter_mut().rev() {
                *x = k % 4;
                k /= 4;
            }
            (dense || row[1..].iter().all(|x| domain[*x] != domain[row[0]])).then_some(row)
        })
        .collect::<Vec<_>>();
    if padding > 0 {
        rows.sort_unstable_by(|a, b| {
            a.iter()
                .map(|j| (j % 2) * padding)
                .sum::<usize>()
                .cmp(&b.iter().map(|j| (j % 2) * padding).sum::<usize>())
                .then_with(|| a.cmp(b))
        });
    }
    rows.into_iter()
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 8);
    let reuse = args[1].starts_with("memo-");
    let mode = args[1].strip_prefix("memo-").unwrap_or(&args[1]);
    assert!(!reuse || mode != "direct");
    assert!(["projected", "enumerate", "lazy-enumerate", "direct"].contains(&mode));
    let n: usize = args[2].parse().unwrap();
    assert!([2, 4].contains(&n));
    let dense = args[3] == "1";
    let padding: usize = args[4].parse().unwrap();
    assert!([0, 3].contains(&padding));
    let queries: usize = args[5].parse().unwrap();
    assert!([1, 8].contains(&queries));
    let keep = args[6] == "1";
    let cancel = args[7] == "1";
    let reuse = reuse && !(cancel && queries == 1);
    const { assert!(!chr_reuse::continuations::COLLECT_METRICS) };
    let domain = ["a", "b", "c", "a"].map(atom).to_vec();
    let (rs, q) = runtime::source_dense(&domain, n, 0, 2, false, padding, dense);
    let expected = runtime::run(rs, q)
        .into_iter()
        .map(runtime::strip)
        .collect::<Vec<_>>();
    drop(domain);
    let mut clocks = (0..1000)
        .map(|_| {
            let t = Instant::now();
            t.elapsed().as_nanos()
        })
        .collect::<Vec<_>>();
    clocks.sort_unstable();
    let floor = clocks[990] * 100;
    let mut rows = Vec::with_capacity(6 + 8 * queries);
    #[cfg(feature = "alloc-meter")]
    let owner = meter::begin();
    let mut held = measure(&mut rows, "consumer_create", || Vec::with_capacity(queries));
    let region = measure(&mut rows, "source", || Region {
        domains: (0..n)
            .map(|i| (Var(i as u64), ["a", "b", "c", "a"].map(atom).to_vec()))
            .collect(),
        predicates: (1..n)
            .filter(|_| !dense)
            .map(|i| Predicate::Different(v(0), v(i as u64)))
            .collect(),
    });
    let (domain, mut caller, source, weights) = measure(&mut rows, "prepare", || {
        let domain = region.domains[&Var(0)].clone();
        let caller = if mode == "direct" {
            None
        } else {
            Some(
                completed::Caller::new(runtime::host(eq(v(0), v(1)), vec![v(0), v(1)], 2, 0))
                    .unwrap(),
            )
        };
        let source = if mode == "direct" {
            let (rs, q) = runtime::source_mode(&domain, n, 0, 2, false, padding, (dense, false));
            Some((Prepared::new(rs, Mode::Direct).unwrap(), q))
        } else {
            None
        };
        let weights = if mode == "projected" {
            region
                .prepare_sparse(&[Var(0)], &[], &[], Observation::Counted, 100000)
                .unwrap()
                .weighted_iter(&[], 100000)
                .unwrap()
                .collect::<BTreeMap<_, _>>()
        } else if mode == "lazy-enumerate" {
            let mut weights = BTreeMap::new();
            for value in &domain {
                *weights.entry(vec![value.clone()]).or_default() += 4u128.pow((n - 1) as u32);
            }
            weights
        } else {
            BTreeMap::new()
        };
        (domain, caller, source, weights)
    });
    measure(&mut rows, "source_dispose", || drop(region));
    for _ in 0..queries {
        let input = measure(&mut rows, "input", || {
            source.as_ref().map(|(_, q)| q.clone())
        });
        let mut output = measure(&mut rows, "setup", || match mode {
            "projected" => Output::Projected(runtime::Ordered::with_dense(
                &domain, n, 0, &weights, padding, dense,
            )),
            "lazy-enumerate" => Output::Lazy(
                runtime::Ordered::with_dense(&domain, n, 0, &weights, padding, true),
                dense,
            ),
            "enumerate" => Output::Enumerated(enumerate(&domain, n, padding, dense)),
            _ => Output::Direct(
                source
                    .as_ref()
                    .unwrap()
                    .0
                    .start(input.as_ref().unwrap().clone())
                    .unwrap(),
            ),
        });
        let mut actual = measure(&mut rows, "consumer_buffer", Vec::new);
        measure(&mut rows, "first", || {
            if let Some(a) = next(&mut output, &mut caller, &domain, reuse) {
                actual.push(a);
            }
        });
        measure(&mut rows, "remaining", || {
            if !cancel {
                while let Some(a) = next(&mut output, &mut caller, &domain, reuse) {
                    actual.push(a);
                }
            }
        });
        assert_eq!(
            actual,
            expected[..if cancel {
                expected.len().min(1)
            } else {
                expected.len()
            }]
        );
        measure(&mut rows, "engine_dispose", || drop(output));
        measure(&mut rows, "input_dispose", || drop(input));
        measure(&mut rows, "consume", || {
            if keep {
                held.push(actual)
            } else {
                drop(actual)
            }
        });
    }
    measure(&mut rows, "prepared_dispose", || {
        drop((domain, caller, source, weights))
    });
    for a in &held {
        assert_eq!(
            a,
            &expected[..if cancel {
                expected.len().min(1)
            } else {
                expected.len()
            }]
        );
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
