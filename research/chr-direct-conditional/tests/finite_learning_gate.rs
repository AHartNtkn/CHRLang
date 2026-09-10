#[allow(dead_code)]
#[path = "../../chr-compiled/experiments/finite_phase.rs"]
mod phase;
use chr_syntax::{Goal, Query, Rule, Var, atom, c, eq, or, v};
use phase::learning::{Learner, Pruning};
use phase::{Limits, Prepared};
fn rules() -> Vec<Rule> {
    vec![
        Rule::simplify(
            "small",
            [c("small", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify(
            "wide",
            [c("wide", [v(0)])],
            or(
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
                eq(v(0), atom("c")),
            ),
        ),
        Rule::simplify("accept", [c("test", [atom("c"), v(1)])], Goal::True),
        Rule::simplify("reject", [c("test", [v(0), v(1)])], Goal::Fail),
    ]
}
fn query(producer: &str, id: u64) -> Query {
    Query {
        constraints: vec![c(producer, [v(id)]), c("test", [v(id), atom("tag")])],
        outputs: vec![("x".into(), Var(id))],
    }
}
#[test]
fn failed_region_prunes_a_successful_wider_renamed_query() {
    let prepared = Prepared::new(&rules(), 4).unwrap();
    let mut learner = Learner::new(&prepared, 8, Pruning::Eager);
    let seed = learner
        .solve(&query("small", 7), Limits::default())
        .unwrap();
    assert!(seed.solutions.is_empty());
    assert_eq!(learner.retained(), 1);
    let baseline = prepared
        .solve(&query("wide", 91), Limits::default())
        .unwrap();
    let learned = learner
        .solve(&query("wide", 91), Limits::default())
        .unwrap();
    assert_eq!(learned.solutions.len(), 1);
    assert_eq!(
        learned.solutions[0].equations,
        baseline.solutions[0].equations
    );
    assert_eq!(
        learned.solutions[0].multiplicity,
        baseline.solutions[0].multiplicity
    );
    assert!(learned.steps < baseline.steps);
    assert!(learner.stats().excluded_regions > 0);
}

#[allow(dead_code)]
mod composition_support;
#[allow(dead_code)]
mod runtime_support;
use phase as finite_phase;
#[path = "../examples/support/finite_bridge.rs"]
mod bridge;
use chr_syntax::{Answer, and};
use std::collections::BTreeMap;
fn observe(rules: &[Rule], report: phase::Report) -> Vec<Answer> {
    let bridge = bridge::Bridge::new(rules.to_vec());
    let mut answers = vec![];
    for solution in report.solutions {
        let (query, weight) = bridge.transport(solution);
        let mut caller = bridge.start(query, weight);
        loop {
            match caller.step() {
                bridge::Event::Progress => (),
                bridge::Event::Answer(a) => answers.push(a),
                bridge::Event::Exhausted => break,
            }
        }
    }
    answers
}
fn copied(r: &phase::Report) -> phase::Report {
    phase::Report {
        steps: r.steps,
        partitions: r.partitions,
        solutions: r
            .solutions
            .iter()
            .map(|s| phase::Solution {
                query: s.query.clone(),
                equations: s.equations.clone(),
                multiplicity: s.multiplicity,
            })
            .collect(),
    }
}
#[derive(Default)]
struct ExactCache(BTreeMap<String, phase::Report>);
impl ExactCache {
    fn solve(&mut self, p: &Prepared, q: &Query) -> (phase::Report, usize) {
        let key = format!("{q:?}");
        if let Some(r) = self.0.get(&key) {
            return (copied(r), 0);
        }
        let r = p.solve(q, Limits::default()).unwrap();
        let steps = r.steps;
        self.0.insert(key, copied(&r));
        (r, steps)
    }
}
fn domain(mask: u8, weight: usize) -> Goal {
    let arms: Vec<_> = ["a", "b", "c"]
        .into_iter()
        .enumerate()
        .filter(|(i, _)| mask & (1 << i) != 0)
        .flat_map(|(_, a)| (0..weight).map(move |_| eq(v(0), atom(a))))
        .collect();
    arms.into_iter().reduce(or).unwrap()
}
fn matrix_rules(accepted: u16, weight: usize) -> Vec<Rule> {
    let mut out: Vec<_> = (1..8)
        .map(|mask| {
            Rule::simplify(
                &format!("d{mask}"),
                [c(&format!("d{mask}"), [v(0)])],
                domain(mask, weight),
            )
        })
        .collect();
    out.push(Rule::simplify(
        "entry",
        [c("entry", [v(0), v(1)])],
        c("pair", [v(0), v(1)]).into(),
    ));
    for (i, a) in ["a", "b", "c"].iter().enumerate() {
        for (j, b) in ["a", "b", "c"].iter().enumerate() {
            if accepted & (1 << (3 * i + j)) != 0 {
                out.push(Rule::simplify(
                    &format!("yes{i}{j}"),
                    [c("pair", [atom(a), atom(b)])],
                    Goal::True,
                ));
            }
        }
    }
    out.push(Rule::simplify("no", [c("pair", [v(0), v(1)])], Goal::Fail));
    out.push(Rule::simplify(
        "caller",
        [c("out", [v(0)]), c("token", [])],
        c("result", [v(0)]).into(),
    ));
    out
}
fn matrix_query(left: u8, right: u8, aliased: bool, base: u64) -> Query {
    let y = if aliased { base } else { base + 1 };
    Query {
        constraints: vec![
            c(&format!("d{left}"), [v(base)]),
            c(&format!("d{right}"), [v(y)]),
            c("entry", [v(base), v(y)]),
            c("out", [v(base)]),
            c("token", []),
            c("noise", [atom("retained")]),
        ],
        outputs: vec![("x".into(), Var(base)), ("y".into(), Var(y))],
    }
}
fn check_source(rules: &[Rule], q: &Query, report: phase::Report) -> usize {
    let expected = runtime_support::run(rules, q, 200_000);
    let count = expected.len();
    runtime_support::same_raw(
        composition_support::Engine::new(0, rules, q).collect(),
        expected.clone(),
    );
    runtime_support::same_raw(observe(rules, report), expected);
    count
}
#[test]
fn independent_domain_alias_weight_and_caller_matrix() {
    let mut seeds = 0;
    let mut queries = 0;
    let mut savings = 0;
    let mut misses = 0;
    for accepted in [0, 0b100010001, 0b011101110, 511] {
        for weight in [1, 2] {
            let rules = matrix_rules(accepted, weight);
            let prepared = Prepared::new(&rules, rules.len() - 1).unwrap();
            for capacity in [0, 1, 4] {
                for seed_mask in [1, 3] {
                    for seed_alias in [false, true] {
                        let mut learner = Learner::new(&prepared, capacity, Pruning::Eager);
                        let mut lazy = Learner::new(&prepared, capacity, Pruning::WhenCovered);
                        let mut cache = ExactCache::default();
                        let seed = matrix_query(seed_mask, seed_mask, seed_alias, 3);
                        let r = learner.solve(&seed, Limits::default()).unwrap();
                        check_source(&rules, &seed, r);
                        let (cached, _) = cache.solve(&prepared, &seed);
                        check_source(&rules, &seed, cached);
                        check_source(&rules, &seed, lazy.solve(&seed, Limits::default()).unwrap());
                        seeds += 1;
                        for (left, right) in [(1, 1), (3, 3), (7, 7), (1, 4), (4, 1)] {
                            for alias in [false, true] {
                                for base in [10, 1000] {
                                    let q = matrix_query(left, right, alias, base);
                                    let baseline = prepared.solve(&q, Limits::default()).unwrap();
                                    let baseline_steps = baseline.steps;
                                    let learned = learner.solve(&q, Limits::default()).unwrap();
                                    let learned_steps = learned.steps;
                                    let before = lazy.stats();
                                    let late = lazy.solve(&q, Limits::default()).unwrap();
                                    let late_steps = late.steps;
                                    let late_parts = late.partitions;
                                    let count = check_source(&rules, &q, learned);
                                    assert_eq!(check_source(&rules, &q, late), count);
                                    println!(
                                        "LATE,{accepted},{weight},{capacity},{seed_mask},{seed_alias},{left},{right},{alias},{base},{baseline_steps},{late_steps},{late_parts},{},{},{count}",
                                        lazy.stats().probes - before.probes,
                                        lazy.stats().excluded_regions - before.excluded_regions
                                    );
                                    runtime_support::same_raw(
                                        observe(&rules, baseline),
                                        runtime_support::run(&rules, &q, 200_000),
                                    );
                                    let (exact, exact_steps) = cache.solve(&prepared, &q);
                                    check_source(&rules, &q, exact);
                                    if learned_steps < baseline_steps {
                                        savings += 1;
                                    }
                                    if learned_steps == baseline_steps {
                                        misses += 1;
                                    }
                                    println!(
                                        "REGION,{accepted},{weight},{capacity},{seed_mask},{seed_alias},{left},{right},{alias},{base},{baseline_steps},{learned_steps},{exact_steps},{count}"
                                    );
                                    queries += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!((seeds, queries), (96, 1920));
    assert!(savings > 0 && misses > 0);
    println!(
        "SUMMARY,seeds={seeds},queries={queries},step_savings={savings},unchanged_steps={misses}"
    );
}
#[test]
fn cancellation_errors_and_zero_capacity_never_install_a_failed_region() {
    let rules = rules();
    let p = Prepared::new(&rules, 4).unwrap();
    let q = query("small", 7);
    let mut l = Learner::new(&p, 8, Pruning::Eager);
    {
        let mut session = l.start(&q, Limits::default()).unwrap();
        assert!(matches!(session.advance(), Ok(phase::Event::Progress)));
    }
    assert_eq!(l.retained(), 0);
    assert!(matches!(
        l.solve(
            &q,
            Limits {
                steps: 0,
                ..Limits::default()
            }
        ),
        Err(phase::Error::Limit(_))
    ));
    assert_eq!(l.retained(), 0);
    let mut zero = Learner::new(&p, 0, Pruning::Eager);
    zero.solve(&q, Limits::default()).unwrap();
    assert_eq!(zero.retained(), 0);
    let mut unknown = q.clone();
    unknown.constraints[1].args[0] = v(900);
    assert!(matches!(
        l.solve(&unknown, Limits::default()),
        Err(phase::Error::Source(_))
    ));
    assert_eq!(l.retained(), 0);
    let suspended_rules = vec![
        rules[0].clone(),
        Rule::simplify("partial", [c("test", [atom("a"), v(1)])], Goal::True),
    ];
    let suspended = Prepared::new(&suspended_rules, 2).unwrap();
    let mut s = Learner::new(&suspended, 4, Pruning::Eager);
    assert!(matches!(
        s.solve(&q, Limits::default()),
        Err(phase::Error::Suspended)
    ));
    assert_eq!(s.retained(), 0);
}
#[test]
fn changed_goals_miss_and_full_caller_can_create_new_private_work() {
    let mut rules = rules();
    rules.push(Rule::simplify(
        "again",
        [c("again", [v(0)]), c("token", [])],
        and([
            c("wide", [v(0)]).into(),
            c("test", [v(0), atom("tag")]).into(),
        ]),
    ));
    let p = Prepared::new(&rules, 4).unwrap();
    let mut l = Learner::new(&p, 1, Pruning::Eager);
    l.solve(&query("small", 7), Limits::default()).unwrap();
    let before = l.stats().excluded_regions;
    let mut different = query("wide", 9);
    different.constraints[1].args[1] = atom("changed");
    check_source(
        &rules,
        &different,
        l.solve(&different, Limits::default()).unwrap(),
    );
    assert_eq!(before, l.stats().excluded_regions);
    let mut later = query("wide", 100);
    later
        .constraints
        .extend([c("again", [v(200)]), c("token", [])]);
    later.outputs.push(("later".into(), Var(200)));
    assert_eq!(
        check_source(&rules, &later, l.solve(&later, Limits::default()).unwrap()),
        1
    );
    assert!(l.stats().excluded_regions > before);
}

#[test]
fn bounded_eviction_partition_errors_and_exact_cache_control() {
    let rules = rules();
    let p = Prepared::new(&rules, 4).unwrap();
    let seed = query("small", 7);
    let wider = query("wide", 91);
    let mut exact = ExactCache::default();
    let (first, steps) = exact.solve(&p, &seed);
    assert!(steps > 0 && first.solutions.is_empty());
    let (repeat, steps) = exact.solve(&p, &seed);
    assert_eq!(steps, 0);
    assert!(repeat.solutions.is_empty());
    let (changed, steps) = exact.solve(&p, &wider);
    assert!(steps > 0);
    assert_eq!(changed.solutions.len(), 1);
    let mut l = Learner::new(&p, 1, Pruning::Eager);
    l.solve(&seed, Limits::default()).unwrap();
    assert!(matches!(
        l.start(
            &wider,
            Limits {
                partitions: 0,
                ..Limits::default()
            }
        ),
        Err(phase::Error::Limit("partitions"))
    ));
    assert_eq!(l.retained(), 1);
    assert_eq!(
        check_source(&rules, &wider, l.solve(&wider, Limits::default()).unwrap()),
        1
    );
    let mut other_seed = seed.clone();
    other_seed.constraints[1].args[1] = atom("other");
    assert!(
        l.solve(&other_seed, Limits::default())
            .unwrap()
            .solutions
            .is_empty()
    );
    assert_eq!(l.retained(), 1);
    let before = l.stats().excluded_regions;
    assert_eq!(
        l.solve(&wider, Limits::default()).unwrap().steps,
        p.solve(&wider, Limits::default()).unwrap().steps
    );
    assert_eq!(l.stats().excluded_regions, before);
    let mut other_wide = wider;
    other_wide.constraints[1].args[1] = atom("other");
    check_source(
        &rules,
        &other_wide,
        l.solve(&other_wide, Limits::default()).unwrap(),
    );
    assert!(l.stats().excluded_regions > before);
}

#[test]
fn independent_producer_domains_survive_projection_and_loops_never_learn() {
    let rules = rules();
    let p = Prepared::new(&rules, 4).unwrap();
    let mut learner = Learner::new(&p, 4, Pruning::Eager);
    learner
        .solve(&query("small", 7), Limits::default())
        .unwrap();
    let mut q = query("wide", 100);
    q.constraints
        .extend([c("small", [v(300)]), c("payload", [v(300)])]);
    q.outputs.push(("independent".into(), Var(300)));
    let baseline = p.solve(&q, Limits::default()).unwrap();
    let result = learner.solve(&q, Limits::default()).unwrap();
    assert!(result.steps < baseline.steps);
    assert_eq!(check_source(&rules, &q, result), 2);
    let loop_rules = vec![
        rules[0].clone(),
        Rule::simplify(
            "loop",
            [c("loop", [v(0), v(1)])],
            c("loop", [v(0), v(1)]).into(),
        ),
    ];
    let p = Prepared::new(&loop_rules, 2).unwrap();
    let mut learner = Learner::new(&p, 4, Pruning::Eager);
    let mut q = query("small", 7);
    q.constraints[1].name = "loop".into();
    assert!(matches!(
        learner.solve(
            &q,
            Limits {
                steps: 3,
                ..Limits::default()
            }
        ),
        Err(phase::Error::Limit("steps"))
    ));
    assert_eq!(learner.retained(), 0);
}

#[test]
fn successful_query_common_work_attribution() {
    let mut directions = [0; 3];
    for depth in [0, 1, 4, 16] {
        for weight in [1, 2] {
            let mut rules = vec![
                Rule::simplify("small", [c("small", [v(0)])], domain(3, weight)),
                Rule::simplify("wide", [c("wide", [v(0)])], domain(7, weight)),
            ];
            for i in 0..depth {
                rules.push(Rule::simplify(
                    &format!("chain{i}"),
                    [c(&format!("p{i}"), [v(0), v(1)])],
                    c(&format!("p{}", i + 1), [v(0), v(1)]).into(),
                ));
            }
            let name = format!("p{depth}");
            rules.push(Rule::simplify(
                "left",
                [c(&name, [atom("c"), v(1)])],
                Goal::True,
            ));
            rules.push(Rule::simplify(
                "right",
                [c(&name, [v(0), atom("c")])],
                Goal::True,
            ));
            rules.push(Rule::simplify("no", [c(&name, [v(0), v(1)])], Goal::Fail));
            let p = Prepared::new(&rules, rules.len()).unwrap();
            let mut l = Learner::new(&p, 4, Pruning::Eager);
            let make = |producer: &str| Query {
                constraints: vec![
                    c(producer, [v(10)]),
                    c(producer, [v(20)]),
                    c("p0", [v(10), v(20)]),
                ],
                outputs: vec![("x".into(), Var(10)), ("y".into(), Var(20))],
            };
            let mut lazy = Learner::new(&p, 4, Pruning::WhenCovered);
            let seed = make("small");
            assert!(
                lazy.solve(&seed, Limits::default())
                    .unwrap()
                    .solutions
                    .is_empty()
            );
            assert_eq!(
                check_source(&rules, &seed, l.solve(&seed, Limits::default()).unwrap()),
                0
            );
            let q = make("wide");
            let baseline = p.solve(&q, Limits::default()).unwrap();
            let learned = l.solve(&q, Limits::default()).unwrap();
            let late = lazy.solve(&q, Limits::default()).unwrap();
            let late_steps = late.steps;
            assert_eq!(check_source(&rules, &q, late), 5 * weight * weight);
            assert!(late_steps < baseline.steps);
            println!(
                "COVERED,{depth},{weight},{late_steps},{},{}",
                lazy.stats().probes,
                lazy.stats().excluded_regions
            );
            let bs = baseline.steps;
            let ls = learned.steps;
            let bp = baseline.partitions;
            let lp = learned.partitions;
            assert_eq!(check_source(&rules, &q, baseline), 5 * weight * weight);
            assert_eq!(check_source(&rules, &q, learned), 5 * weight * weight);
            directions[if ls < bs {
                0
            } else if ls == bs {
                1
            } else {
                2
            }] += 1;
            println!("COMMON,{depth},{weight},{bs},{ls},{bp},{lp}");
        }
    }
    assert!(directions[0] > 0 && directions[2] > 0);
}

#[test]
fn covered_learning_resolves_late_aliases_and_constructor_calls() {
    for policy in [Pruning::Eager, Pruning::WhenCovered] {
        let mut source = rules();
        source.insert(
            2,
            Rule::simplify(
                "enter",
                [c("enter", [v(0), v(1)])],
                and([
                    eq(v(0), v(1)),
                    c(
                        "wrapped",
                        [chr_syntax::Term::App("box".into(), vec![v(0)]), v(1)],
                    )
                    .into(),
                ]),
            ),
        );
        source.insert(
            3,
            Rule::simplify(
                "unwrap",
                [c(
                    "wrapped",
                    [chr_syntax::Term::App("box".into(), vec![v(0)]), v(1)],
                )],
                c("test", [v(0), v(1)]).into(),
            ),
        );
        let p = Prepared::new(&source, source.len()).unwrap();
        let mut l = Learner::new(&p, 4, policy);
        let make = |producer: &str| Query {
            constraints: vec![
                c(producer, [v(10)]),
                c(producer, [v(20)]),
                c("enter", [v(10), v(20)]),
            ],
            outputs: vec![("x".into(), Var(10)), ("y".into(), Var(20))],
        };
        let seed = make("small");
        assert_eq!(
            check_source(&source, &seed, l.solve(&seed, Limits::default()).unwrap()),
            0
        );
        let q = make("wide");
        assert_eq!(
            check_source(&source, &q, l.solve(&q, Limits::default()).unwrap()),
            1
        );
        let mut changed = q.clone();
        changed.constraints[2] = c("test", [v(10), v(20)]);
        assert_eq!(
            check_source(
                &source,
                &changed,
                l.solve(&changed, Limits::default()).unwrap()
            ),
            3
        );
    }
}
#[test]
fn covered_cancellation_and_traversal_errors_never_learn() {
    let mut source = rules();
    source.insert(
        0,
        Rule::simplify("one", [c("one", [v(0)])], eq(v(0), atom("a"))),
    );
    let p = Prepared::new(&source, 5).unwrap();
    let mut l = Learner::new(&p, 4, Pruning::WhenCovered);
    let seed = query("small", 7);
    l.solve(&seed, Limits::default()).unwrap();
    let installed = l.stats().learned_regions;
    let narrow = query("one", 7);
    {
        let mut session = l.start(&narrow, Limits::default()).unwrap();
        // This event discards a covered state, but is not a completed proof.
        assert!(matches!(session.advance().unwrap(), phase::Event::Progress));
    }
    assert_eq!(l.stats().learned_regions, installed);
    // Find a budget where source preparation fits but coverage traversal fails.
    let mut witnessed = false;
    for nodes in 1..100 {
        let limits = Limits {
            term_nodes: nodes,
            ..Limits::default()
        };
        if let Ok(mut session) = l.start(&narrow, limits)
            && matches!(session.advance(), Err(phase::Error::Limit("term nodes")))
        {
            assert!(matches!(
                session.advance().unwrap(),
                phase::Event::Exhausted
            ));
            witnessed = true;
            break;
        }
    }
    assert!(witnessed);
    assert_eq!(l.stats().learned_regions, installed);
    let q = query("wide", 100);
    assert_eq!(
        check_source(&source, &q, l.solve(&q, Limits::default()).unwrap()),
        1
    );
}
