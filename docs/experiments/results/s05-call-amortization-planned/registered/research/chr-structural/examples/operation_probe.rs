#[path = "../../chr-persistent/examples/support/allocator.rs"]
mod allocator;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
use chr_structural::{Datum, Mode, NoC, no_c_rules};
fn names() -> Vec<String> {
    let mut names = vec![];
    for shape in ["ground", "hole", "unknown", "failure", "unshared"] {
        for depth in [0, 1, 4, 8, 12] {
            for repeats in [1, 8, 64] {
                names.push(format!("{shape}-d{depth}-r{repeats}"));
            }
        }
    }
    names
}
fn app(n: &str, args: impl IntoIterator<Item = Datum>, nodes: &mut u64) -> Datum {
    *nodes += 1;
    Datum::app(n, args)
}
fn leaf(shape: &str, nodes: &mut u64) -> Datum {
    match shape {
        "hole" => {
            *nodes += 1;
            Datum::var(0)
        }
        "unknown" => {
            let z = app("z", [], nodes);
            let c = app("c", [z], nodes);
            app("unknown", [c], nodes)
        }
        "failure" => {
            let z = app("z", [], nodes);
            app("c", [z], nodes)
        }
        _ => app("k", [], nodes),
    }
}
fn unshared(depth: usize, nodes: &mut u64) -> Datum {
    if depth == 0 {
        return leaf("ground", nodes);
    }
    let a = unshared(depth - 1, nodes);
    let b = unshared(depth - 1, nodes);
    app("a", [a, b], nodes)
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--list"] {
        for n in names() {
            println!("{n}");
        }
        return;
    }
    assert_eq!(args.len(), 2, "operation_probe CASE Direct|Memo");
    assert!(names().contains(&args[0]));
    let parts = args[0].split('-').collect::<Vec<_>>();
    let shape = parts[0];
    let depth = parts[1][1..].parse::<usize>().unwrap();
    let repeats = parts[2][1..].parse::<usize>().unwrap();
    let mode = match args[1].as_str() {
        "Direct" => Mode::Direct,
        "Memo" => Mode::Memo,
        _ => panic!("unknown mode"),
    };
    let start = allocator::start();
    let clock = std::time::Instant::now();
    let mut nodes = 0;
    let inputs = if shape == "unshared" {
        (0..repeats)
            .map(|_| unshared(depth, &mut nodes))
            .collect::<Vec<_>>()
    } else {
        let mut root = leaf(shape, &mut nodes);
        for _ in 0..depth {
            root = app("a", [root.clone(), root], &mut nodes);
        }
        vec![root; repeats]
    };
    let mut solver = NoC::new(&no_c_rules(), mode).unwrap();
    let built = allocator::read();
    let build_us = clock.elapsed().as_micros();
    let clock = std::time::Instant::now();
    let result = solver.reduce(&inputs);
    let solved = allocator::read();
    let solve_us = clock.elapsed().as_micros();
    let clock = std::time::Instant::now();
    let answer = result.as_ref().map(|residual| chr_syntax::Answer {
        outputs: vec![("out0".into(), chr_syntax::v(0))],
        residual: residual
            .iter()
            .map(|d| chr_syntax::c("no_c", [d.export()]))
            .collect(),
    });
    let end = allocator::read();
    let export_us = clock.elapsed().as_micros();
    let residual = answer.as_ref().map_or(0, |a| a.residual.len());
    let expected = if matches!(shape, "hole" | "unknown") {
        repeats * (1 << depth)
    } else {
        0
    };
    let expected_term = if shape == "hole" {
        chr_syntax::v(0)
    } else {
        chr_syntax::t("unknown", [chr_syntax::t("c", [chr_syntax::atom("z")])])
    };
    let pass = answer.is_some() != (shape == "failure")
        && residual == expected
        && answer.as_ref().is_none_or(|a| {
            a.residual
                .iter()
                .all(|c| c.name == "no_c" && c.args == [expected_term.clone()])
        });
    let s = solver.stats();
    println!(
        "case\tmode\tpass\tsuccess\tinput_nodes\tcalls\tcomputed\thits\tcache_entries\tresidual_refs\tcertificate_rules\tcertificate_terms\tresiduals\tallocation_calls\tbuild_requested\tsolve_requested\texport_requested\trequested_bytes\tbaseline_live\tpeak_live\tfinal_live\tbuild_us\tsolve_us\texport_us"
    );
    let mut row = vec![
        args[0].clone(),
        format!("{mode:?}"),
        pass.to_string(),
        answer.is_some().to_string(),
    ];
    row.extend(
        [
            nodes,
            s.calls,
            s.computed,
            s.hits,
            s.cache_entries as u64,
            s.residual_refs,
            s.certificate_rules,
            s.certificate_terms,
            residual as u64,
            end.calls - start.calls,
            built.requested - start.requested,
            solved.requested - built.requested,
            end.requested - solved.requested,
            end.requested - start.requested,
            start.live,
            end.peak,
            end.live,
            build_us as u64,
            solve_us as u64,
            export_us as u64,
        ]
        .map(|x| x.to_string()),
    );
    println!("{}", row.join("\t"));
    assert!(pass);
}
