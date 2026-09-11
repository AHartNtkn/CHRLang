#[allow(dead_code)]
#[path = "support/local_ports.rs"]
mod local;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use chr_syntax::{Answer, Goal, Query, Rule, Var, atom, c, eq, t, v};
fn wrap(depth: usize, value: chr_syntax::Term) -> chr_syntax::Term {
    (0..depth).fold(value, |x, _| t("f", [x]))
}
fn source(family: &str, n: usize, depth: usize, seed: usize) -> (Vec<Rule>, Query) {
    let mut rules = vec![];
    if family == "kill" {
        rules.push(Rule {
            name: "kill".into(),
            kept: vec![c("kill", [])],
            removed: vec![c("left", [v(0)])],
            guards: vec![],
            body: Goal::True,
        });
    }
    rules.push(Rule {
        name: "join".into(),
        kept: vec![
            c("left", [wrap(depth, v(0))]),
            c("middle", [wrap(depth, v(1))]),
        ],
        removed: vec![c("right", [v(0), v(1), v(2)])],
        guards: vec![],
        body: eq(v(2), atom("hit")),
    });
    let key = |i: usize| atom(&format!("k{seed}_{i}"));
    let base = 1000 + seed as u64 * 100;
    let mut facts = vec![];
    for i in 0..n {
        facts.push(c("left", [wrap(depth, key(i))]));
        if family != "miss" || depth > 0 {
            let middle = if family == "miss" {
                wrap(depth - 1, t("g", [key(i)]))
            } else {
                wrap(depth, key(i))
            };
            facts.push(c("middle", [middle]));
        }
        let target = if family == "keyed" { i } else { n - 1 };
        facts.push(c("right", [key(target), key(target), v(base + i as u64)]));
    }
    if family == "kill" {
        facts.push(c("kill", []));
    }
    (
        rules,
        Query {
            constraints: facts,
            outputs: (0..n)
                .map(|i| (format!("out{i}"), Var(base + i as u64)))
                .collect(),
        },
    )
}
fn same(got: Option<Answer>, expected: &[Answer]) -> Answer {
    scalar::same_raw(got.clone().into_iter().collect(), expected.to_vec());
    got.unwrap()
}
fn finish<const M: bool>(e: &mut local::multihead::Execution<M>) {
    for _ in 0..200_000 {
        e.assert_cache_integrity();
        if e.advance() {
            return;
        }
    }
    panic!("local cutoff");
}
fn start<const M: bool>(
    p: &std::sync::Arc<local::multihead::Program>,
    q: &Query,
    mode: usize,
) -> local::multihead::Execution<M> {
    match mode {
        0 => p.start_mode(q, false),
        1 => p.start_mode(q, true),
        2 => p.start_partial(q),
        _ => p.start_intermediate(q),
    }
}
#[test]
fn structural_prefix_source_and_work() {
    for family in ["sparse", "keyed", "kill", "miss"] {
        for n in [4, 8] {
            for depth in [0, 8, 32] {
                let rules = source(family, n, depth, 0).0;
                let p = local::multihead::Program::compile(&rules).unwrap();
                let ordinary = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
                let specialized = chr_compiled::PreparedRuleset::new(rules.clone(), None)
                    .unwrap()
                    .specialize_inferred();
                let mut held = vec![];
                for seed in [0, 1] {
                    let q = source(family, n, depth, seed).1;
                    let expected = scalar::run(&rules, &q, 200_000);
                    assert_eq!(expected.len(), 1);
                    let hits = usize::from(family == "sparse" || family == "keyed") * n;
                    assert_eq!(
                        expected[0]
                            .outputs
                            .iter()
                            .filter(|(_, x)| *x == atom("hit"))
                            .count(),
                        hits
                    );
                    assert_eq!(
                        expected[0]
                            .residual
                            .iter()
                            .filter(|c| c.name == "right")
                            .count(),
                        n - hits
                    );
                    assert_eq!(
                        expected[0]
                            .residual
                            .iter()
                            .filter(|c| c.name == "left")
                            .count(),
                        if family == "kill" { 0 } else { n }
                    );
                    for mode in 0..4 {
                        let mut cancelled = start::<false>(&p, &q, mode);
                        cancelled.advance();
                        drop(cancelled);
                        let mut e = start::<true>(&p, &q, mode);
                        finish(&mut e);
                        let (patterns, equalities) = e.matching_work();
                        println!(
                            "STRUCTURE,{family},{n},{depth},{seed},local-{mode},{patterns},{equalities},{},{},{},{},{}",
                            e.work.head_attempts.get(),
                            e.work.fact_visits.get(),
                            e.work.intermediate_visits.get(),
                            e.work.peak_tuples,
                            e.firings
                        );
                        held.push((same(e.answer(), &expected), expected.clone()));
                        let mut plain = start::<false>(&p, &q, mode);
                        finish(&mut plain);
                        assert_eq!(plain.matching_work(), (0, 0));
                        held.push((same(plain.answer(), &expected), expected.clone()));
                    }
                    for (special, prep) in [(false, &ordinary), (true, &specialized)] {
                        for indexed in [false, true] {
                            let access = if indexed {
                                chr_compiled::Access::Indexed
                            } else {
                                chr_compiled::Access::Scan
                            };
                            let mut cancelled = prep
                                .start(q.clone(), chr_compiled::Policy::Global, access)
                                .unwrap();
                            cancelled.advance(1);
                            drop(cancelled);
                            let mut e = prep
                                .start(q.clone(), chr_compiled::Policy::Global, access)
                                .unwrap();
                            let mut done = false;
                            for _ in 0..200_000 {
                                if e.advance(1).exhausted {
                                    done = true;
                                    break;
                                }
                            }
                            assert!(done, "compiled cutoff");
                            let w = e.stats();
                            println!(
                                "STRUCTURE,{family},{n},{depth},{seed},compiled-{special}-{indexed},{},{},{},{},{}",
                                w.structural_tests,
                                w.candidate_visits,
                                w.generic_ast_visits,
                                w.key_visits,
                                w.specialized_applications
                            );
                            held.push((same(e.observe(), &expected), expected.clone()));
                        }
                    }
                }
                drop(p);
                drop(ordinary);
                drop(specialized);
                drop(rules);
                for (answer, expected) in held {
                    same(Some(answer), &expected);
                }
            }
        }
    }
}
