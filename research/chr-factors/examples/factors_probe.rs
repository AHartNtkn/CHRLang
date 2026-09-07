#[path = "../../chr-persistent/examples/support/allocator.rs"]
mod allocator;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
use chr_factors::{Mode, Search};
use chr_syntax::{Rule, Term, Var, atom, c, eq, or, t, v};
fn unary(n: usize) -> Term {
    (0..n).fold(atom("z"), |n, _| t("s", [n]))
}
fn names() -> Vec<String> {
    let mut names = chr_cases::registry()
        .into_iter()
        .filter(|c| c.exhausted)
        .map(|c| c.id)
        .collect::<Vec<_>>();
    for k in [1, 2, 4, 8] {
        for w in [0, 1, 8, 64, 256] {
            for n in [0, 16] {
                names.push(format!("product-k{k}-w{w}-n{n}"));
            }
        }
    }
    names.push("mixed-add-infer".into());
    names
}
fn shift(t: &Term) -> Term {
    match t {
        Term::Var(Var(v)) => chr_syntax::v(v + 1000),
        Term::App(n, args) => chr_syntax::t(n, args.iter().map(shift).collect::<Vec<_>>()),
    }
}
fn input(id: &str) -> chr_cases::Case {
    if let Some(case) = chr_cases::registry().into_iter().find(|c| c.id == id) {
        return case;
    }
    if id == "mixed-add-infer" {
        let mut add = chr_cases::application_cases()
            .into_iter()
            .find(|c| c.id == "app-add-forward")
            .unwrap();
        let infer = chr_cases::application_cases()
            .into_iter()
            .find(|c| c.id == "app-infer-identity")
            .unwrap();
        add.id = id.into();
        add.rules.extend(infer.rules);
        add.query
            .constraints
            .extend(
                infer
                    .query
                    .constraints
                    .into_iter()
                    .map(|c| chr_syntax::Constraint {
                        name: c.name,
                        args: c.args.iter().map(shift).collect(),
                    }),
            );
        add.query.outputs.push(("out1".into(), Var(1000)));
        add.expected = vec![chr_cases::answer(
            vec![unary(5), t("fun", [v(0), v(0)])],
            vec![],
        )];
        add.budget = 100000;
        return add;
    }
    for k in [1, 2, 4, 8] {
        for w in [0, 1, 8, 64, 256] {
            for n in [0, 16] {
                if id == format!("product-k{k}-w{w}-n{n}") {
                    let mut rules = vec![];
                    let mut constraints = vec![];
                    for i in 0..k {
                        rules.push(Rule::simplify(
                            &format!("pick-{i}"),
                            [c(&format!("pick{i}"), [v(0)])],
                            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
                        ));
                    }
                    for i in 0..k {
                        rules.push(Rule::simplify(
                            &format!("carry-{i}"),
                            [c(&format!("carry{i}"), [v(0), t("s", [v(1)])])],
                            c(&format!("carry{i}"), [v(0), v(1)]).into(),
                        ));
                    }
                    for i in 0..k {
                        constraints.push(c(&format!("pick{i}"), [v(i)]));
                        constraints.push(c(&format!("carry{i}"), [v(i), unary(w)]));
                    }
                    let noise = (0..n)
                        .map(|i| c("noise", [unary(i + 1)]))
                        .collect::<Vec<_>>();
                    constraints.extend(noise.clone());
                    let expected = (0..1u64 << k)
                        .map(|bits| {
                            let values = (0..k)
                                .map(|i| atom(if bits & (1 << i) == 0 { "a" } else { "b" }))
                                .collect::<Vec<_>>();
                            let mut residual = (0..k)
                                .map(|i| {
                                    c(
                                        &format!("carry{i}"),
                                        [values[i as usize].clone(), atom("z")],
                                    )
                                })
                                .collect::<Vec<_>>();
                            residual.extend(noise.clone());
                            chr_cases::answer(values, residual)
                        })
                        .collect();
                    return chr_cases::Case {
                        id: id.into(),
                        rules,
                        query: chr_cases::query(constraints, &(0..k).collect::<Vec<_>>()),
                        expected,
                        exhausted: true,
                        raw_answers: 1 << k,
                        budget: 5_000_000,
                        answer_limit: None,
                    };
                }
            }
        }
    }
    panic!("unknown case");
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args == ["--list"] {
        for n in names() {
            println!("{n}");
        }
        return;
    }
    assert_eq!(args.len(), 2, "factors_probe CASE Scalar|Factored");
    let mode = match args[1].as_str() {
        "Scalar" => Mode::Scalar,
        "Factored" => Mode::Factored,
        _ => panic!("unknown mode"),
    };
    let case = input(&args[0]);
    assert!(case.exhausted);
    let start = allocator::start();
    let clock = std::time::Instant::now();
    let mut s = Search::new(case.rules.clone(), case.query.clone(), mode).unwrap();
    let b = s.advance(case.budget * 8);
    let elapsed = clock.elapsed().as_micros();
    let end = allocator::read();
    let pass = b.exhausted
        && b.answers.len() == case.expected.len()
        && s.raw_count() == Some(case.raw_answers as u128)
        && case.expected.iter().all(|e| {
            b.answers
                .iter()
                .any(|a| chr_observe::equivalent(a, e, &mut Default::default()))
        });
    let stats = s.stats();
    let source = s.source_stats();
    let sum = |f: fn(&chr_persistent::Stats) -> u64| source.iter().map(|s| f(s)).sum::<u64>();
    let observer = s.observation_stats();
    let regional = s.regional_observation_stats();
    println!(
        "case\tmode\tpass\tfactors\tcertificate_predicates\tcertificate_edges\tcertificate_terms\tsteps\tsource_steps\tapplications\tequations\tpairs\thead_candidates\tmap_visits\tmap_allocations\tterm_requests\tpending_allocations\tregional_raw\traw_product\tanswers\tproduct_jobs\tproducts\trenamed_nodes\tmax_jobs\tcache_answers\tpending_jobs\tempty_refutations\tobserver_pairs\tobserver_scans\tregional_observer_pairs\tregional_observer_scans\tregional_observer_backtracks\tallocation_calls\trequested_bytes\tbaseline_live\tpeak_live\tfinal_live\telapsed_us"
    );
    let mut row = vec![case.id, format!("{mode:?}"), pass.to_string()];
    row.extend(
        [
            s.factor_count() as u64,
            stats.certificate_predicates as u64,
            stats.certificate_edges,
            stats.certificate_terms,
            stats.steps,
            stats.source_steps,
            s.source_applications(),
            sum(|s| s.equations),
            sum(|s| s.pairs),
            sum(|s| s.head_candidates),
            sum(|s| s.storage.visits),
            sum(|s| s.storage.allocations),
            sum(|s| s.term_requests),
            sum(|s| s.pending_allocations),
            sum(|s| s.completed),
            b.answers.len() as u64,
            stats.product_jobs,
            stats.products,
            stats.renamed_nodes,
            stats.max_jobs as u64,
            s.cached_answers() as u64,
            s.pending_jobs() as u64,
            stats.empty_refutations,
            observer.term_pairs,
            observer.occurrence_scans,
            regional.term_pairs,
            regional.occurrence_scans,
            regional.backtracks,
            end.calls - start.calls,
            end.requested - start.requested,
            start.live,
            end.peak,
            end.live,
            elapsed as u64,
        ]
        .map(|x| x.to_string()),
    );
    row.insert(
        18,
        s.raw_count()
            .map(|n| n.to_string())
            .unwrap_or_else(|| "unavailable".into()),
    );
    println!("{}", row.join("\t"));
    assert!(pass);
}
