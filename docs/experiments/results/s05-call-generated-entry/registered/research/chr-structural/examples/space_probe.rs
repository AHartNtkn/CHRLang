#[path = "../../chr-persistent/examples/support/allocator.rs"]
mod allocator;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
use chr_structural::{
    regular::{Automaton, Transition as T},
    space::{Mode, Pattern, Request, Space},
};
use chr_syntax::{Term, atom, t};
fn names() -> Vec<String> {
    let mut out = vec![];
    for shape in [
        "independent",
        "repeated",
        "merged",
        "intersect_k",
        "disjoint",
        "overlap",
    ] {
        for holes in [1, 2, 4] {
            for bound in [1, 3, 5, 7, 9] {
                out.push(format!("{shape}-h{holes}-n{bound}"));
            }
        }
    }
    out
}
fn domain(kind: &str) -> Automaton {
    let any = vec![T::new("k", []), T::new("s", []), T::new("a", [0, 0])];
    match kind {
        "N" => Automaton::new(vec![any], 0).unwrap(),
        "K" | "S" => Automaton::new(
            vec![vec![
                T::new(if kind == "K" { "k" } else { "s" }, []),
                T::new("a", [0, 0]),
            ]],
            0,
        )
        .unwrap(),
        "hasK" | "hasS" => Automaton::new(
            vec![
                any,
                vec![
                    T::new(if kind == "hasK" { "k" } else { "s" }, []),
                    T::new("a", [1, 0]),
                    T::new("a", [0, 1]),
                ],
            ],
            1,
        )
        .unwrap(),
        _ => panic!("domain"),
    }
}
fn pattern(h: usize, repeated: bool) -> Pattern {
    let mut p = Pattern::Hole(if repeated { 0 } else { h - 1 });
    for i in (0..h - 1).rev() {
        p = Pattern::App(
            "a".into(),
            vec![Pattern::Hole(if repeated { 0 } else { i }), p],
        );
    }
    p
}
fn request(shape: &str, h: usize) -> Request {
    let kinds = match shape {
        "intersect_k" => vec!["N", "hasK", "K"],
        "disjoint" => vec!["hasK", "S"],
        "overlap" => vec!["hasK", "hasS"],
        _ => vec!["N"],
    };
    Request {
        pattern: pattern(h, shape == "repeated"),
        domains: (0..if shape == "repeated" { 1 } else { h })
            .map(|_| kinds.iter().map(|k| domain(k)).collect())
            .collect(),
        equalities: if shape == "merged" {
            (1..h).map(|i| (0, i)).collect()
        } else {
            vec![]
        },
    }
}
fn has(x: &Term, name: &str) -> bool {
    match x {
        Term::App(n, args) => n == name || args.iter().any(|a| has(a, name)),
        _ => false,
    }
}
// Independent tree construction, without automata or candidate space services.
fn expected(shape: &str, h: usize, bound: usize) -> (u128, Vec<Term>) {
    let mut sizes = vec![vec![]; bound + 1];
    sizes[1] = vec![atom("k"), atom("s")];
    for n in 2..=bound {
        let mut values = vec![];
        for l in 1..n - 1 {
            for a in &sizes[l] {
                for b in &sizes[n - 1 - l] {
                    values.push(t("a", [a.clone(), b.clone()]));
                }
            }
        }
        values.sort();
        sizes[n] = values;
    }
    let options = sizes
        .into_iter()
        .flatten()
        .filter(|x| match shape {
            "intersect_k" => !has(x, "s"),
            "disjoint" => false,
            "overlap" => has(x, "s") && has(x, "k"),
            _ => true,
        })
        .collect::<Vec<_>>();
    let shared = matches!(shape, "repeated" | "merged");
    let classes = if shared { 1 } else { h };
    let count = (options.len() as u128).pow(classes as u32);
    let terms = (0..count.min(1024) as usize)
        .map(|mut rank| {
            let mut indices = vec![0; classes];
            for i in (0..classes).rev() {
                indices[i] = rank % options.len();
                rank /= options.len();
            }
            let mut out = options[indices[if shared { 0 } else { h - 1 }]].clone();
            for i in (0..h - 1).rev() {
                out = t(
                    "a",
                    [options[indices[if shared { 0 } else { i }]].clone(), out],
                );
            }
            out
        })
        .collect();
    (count, terms)
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--list"] {
        for n in names() {
            println!("{n}");
        }
        return;
    }
    assert_eq!(args.len(), 2);
    assert!(names().contains(&args[0]));
    let parts = args[0].split('-').collect::<Vec<_>>();
    let shape = parts[0];
    let h = parts[1][1..].parse().unwrap();
    let bound = parts[2][1..].parse().unwrap();
    let mode = match args[1].as_str() {
        "FilterFirst" => Mode::FilterFirst,
        "FilterSmallest" => Mode::FilterSmallest,
        "Intersect" => Mode::Intersect,
        _ => panic!("mode"),
    };
    let start = allocator::start();
    let clock = std::time::Instant::now();
    let req = request(shape, h);
    let built = allocator::read();
    let build_us = clock.elapsed().as_micros();
    let clock = std::time::Instant::now();
    let mut space = Space::compile(req, bound, mode).unwrap();
    let compiled = allocator::read();
    let compile_us = clock.elapsed().as_micros();
    let clock = std::time::Instant::now();
    let result = space.advance(1024);
    let end = allocator::read();
    let output_us = clock.elapsed().as_micros();
    let (count, terms) = expected(shape, h, bound);
    let pass = space.solution_count() == Some(count)
        && result.terms == terms
        && result.exhausted == (count <= 1024);
    let s = space.stats();
    let r = &s.regular;
    println!(
        "case\tmode\tpass\tsolutions\texhausted\tclasses\tretained_candidates\tpattern_nodes\tunions\tdomain_compares\tenumeration_cache_hits\tdomain_cache_hits\tfilter_candidates\toutput_nodes\temitted\tintersection_pairs\ttransition_pairs\tproductivity_checks\twitness_nodes\tground_checks\tmembership_steps\tmembership_hits\tenumeration_checks\tsize_partitions\tenumeration_candidates\tallocation_calls\tbuild_requested\tcompile_requested\toutput_requested\trequested_bytes\tbaseline_live\tpeak_live\tfinal_live\tbuild_us\tcompile_us\toutput_us"
    );
    let mut row = vec![
        args[0].clone(),
        args[1].clone(),
        pass.to_string(),
        count.to_string(),
        result.exhausted.to_string(),
    ];
    row.extend(
        [
            space.classes() as u64,
            space.retained_candidates() as u64,
            s.pattern_nodes,
            s.unions,
            s.domain_compares,
            s.enumeration_cache_hits,
            s.domain_cache_hits,
            s.filter_candidates,
            s.output_nodes,
            s.emitted,
            r.intersection_pairs,
            r.transition_pairs,
            r.productivity_checks,
            r.witness_nodes,
            r.ground_checks,
            r.membership_steps,
            r.membership_hits,
            r.enumeration_checks,
            r.size_partitions,
            r.enumeration_candidates,
            end.calls - start.calls,
            built.requested - start.requested,
            compiled.requested - built.requested,
            end.requested - compiled.requested,
            end.requested - start.requested,
            start.live,
            end.peak,
            end.live,
            build_us as u64,
            compile_us as u64,
            output_us as u64,
        ]
        .map(|v| v.to_string()),
    );
    println!("{}", row.join("\t"));
    assert!(pass);
}
