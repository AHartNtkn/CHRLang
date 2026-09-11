use chr_compiled::{Access, Policy, PreparedRuleset};
use chr_syntax::{Query, Rule, atom, c, v};
#[test]
fn selective_probe_avoids_unkeyed_prefix_and_keeps_source_result() {
    let rules = vec![Rule {
        name: "join".into(),
        kept: vec![
            c("request", [v(2)]),
            c("left", [v(0), v(1)]),
            c("right", [v(1), v(2)]),
        ],
        removed: vec![c("token", [])],
        guards: vec![],
        body: c("receipt", [v(0), v(1), v(2)]).into(),
    }];
    let mut facts = vec![c("request", [atom("b127")])];
    for i in 0..128 {
        facts.push(c("left", [atom(&format!("a{i}")), atom(&format!("x{i}"))]));
        facts.push(c("right", [atom(&format!("x{i}")), atom(&format!("b{i}"))]));
    }
    let mut expected = facts.clone();
    expected.push(c("receipt", [atom("a127"), atom("x127"), atom("b127")]));
    facts.push(c("token", []));
    let prepared = PreparedRuleset::new(rules, None).unwrap();
    let mut engine = prepared
        .start(
            Query {
                constraints: facts,
                outputs: vec![],
            },
            Policy::Global,
            Access::Indexed,
        )
        .unwrap();
    assert!(engine.advance(2000000).exhausted);
    let mut answer = engine.observe().unwrap();
    answer.residual.sort();
    expected.sort();
    assert_eq!(answer.residual, expected);
    if chr_compiled::COLLECT_METRICS && cfg!(feature = "selective-probe") {
        assert!(
            engine.stats().candidate_visits < 20,
            "candidate visits: {}",
            engine.stats().candidate_visits
        );
    }
}

fn execute(
    rules: Vec<Rule>,
    query: Query,
    policy: Policy,
    access: Access,
) -> (chr_syntax::Answer, Vec<(usize, Vec<u64>)>) {
    let p = PreparedRuleset::new(rules, None).unwrap();
    let mut e = p.start(query, policy, access).unwrap();
    e.enable_trace();
    let status = e.advance(2_000_000);
    assert!(status.exhausted && !status.failed && !status.pending_split);
    let mut answer = e.observe().unwrap();
    answer.residual.sort();
    (answer, e.trace().to_vec())
}
fn join_rule(reverse: bool, guard: bool) -> Rule {
    let mut kept = vec![c("left", [v(0), v(1)]), c("right", [v(1), v(2)])];
    if reverse {
        kept.reverse();
    }
    kept.insert(0, c("request", [v(2)]));
    Rule {
        name: "join".into(),
        kept,
        removed: vec![c("token", [])],
        guards: if guard {
            vec![chr_syntax::Guard::Equal(v(0), atom("a0"))]
        } else {
            vec![]
        },
        body: c("receipt", [v(0), v(1), v(2)]).into(),
    }
}
#[test]
fn all_small_ground_relations_keep_first_legal_tuple() {
    let mut cases = 0;
    for lm in 0..16 {
        for rm in 0..16 {
            for order in [false, true] {
                for guard in [false, true] {
                    let mut facts = vec![c("request", [atom("b")])];
                    for i in 0..4 {
                        if lm & (1 << i) != 0 {
                            facts.push(c(
                                "left",
                                [atom(&format!("a{}", i / 2)), atom(&format!("x{}", i % 2))],
                            ));
                        }
                    }
                    // Reverse right occurrence order to expose tuple-priority changes.
                    for i in (0..4).rev() {
                        if rm & (1 << i) != 0 {
                            facts.push(c(
                                "right",
                                [
                                    atom(&format!("x{}", i % 2)),
                                    atom(if i / 2 == 0 { "b" } else { "other" }),
                                ],
                            ));
                        }
                    }
                    let token = facts.len() as u64;
                    facts.push(c("token", []));
                    let left: Vec<_> = facts
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| c.name == "left")
                        .collect();
                    let right: Vec<_> = facts
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| c.name == "right")
                        .collect();
                    let mut matches = vec![];
                    for (li, l) in &left {
                        for (ri, r) in &right {
                            if l.args[1] == r.args[0]
                                && r.args[1] == atom("b")
                                && (!guard || l.args[0] == atom("a0"))
                            {
                                matches.push((
                                    if order {
                                        vec![0, *ri as u64, *li as u64, token]
                                    } else {
                                        vec![0, *li as u64, *ri as u64, token]
                                    },
                                    c(
                                        "receipt",
                                        [l.args[0].clone(), l.args[1].clone(), r.args[1].clone()],
                                    ),
                                ));
                            }
                        }
                    }
                    matches.sort_by(|a, b| a.0.cmp(&b.0));
                    let mut expected = facts.clone();
                    let mut trace = vec![];
                    if let Some((ids, receipt)) = matches.first() {
                        expected.pop();
                        expected.push(receipt.clone());
                        trace.push((0, ids.clone()));
                    }
                    expected.sort();
                    for access in [Access::Scan, Access::Indexed] {
                        let (answer, actual_trace) = execute(
                            vec![join_rule(order, guard)],
                            Query {
                                constraints: facts.clone(),
                                outputs: vec![],
                            },
                            Policy::Global,
                            access,
                        );
                        assert_eq!(
                            answer.residual, expected,
                            "masks={lm},{rm} order={order} guard={guard}"
                        );
                        assert_eq!(actual_trace, trace);
                        cases += 1;
                    }
                }
            }
        }
    }
    assert_eq!(cases, 2048);
    println!("PROBE_GROUND,cases={cases}");
}
#[test]
fn aliases_constructors_duplicates_and_binding_updates_keep_traces() {
    use chr_syntax::{Var, and, eq, t};
    let mut cases = 0;
    for wrapped in [false, true] {
        for distinct in [false, true] {
            for duplicates in [false, true] {
                for bind in [false, true] {
                    for policy in [Policy::Global, Policy::Active] {
                        let a = v(10);
                        let b = if distinct { v(11) } else { v(10) };
                        let wrap = |x| if wrapped { t("box", [x]) } else { x };
                        let mut facts = vec![
                            c("request", [atom("b")]),
                            c("left", [atom("a0"), wrap(a.clone())]),
                            c("left", [atom("a1"), wrap(b.clone())]),
                            c("right", [wrap(b.clone()), atom("b")]),
                            c("token", []),
                        ];
                        if duplicates {
                            facts.insert(3, c("left", [atom("a0"), wrap(a.clone())]));
                        }
                        let mut rules = vec![join_rule(false, false)];
                        if bind {
                            rules.insert(
                                0,
                                Rule::simplify(
                                    "bind",
                                    [c("bind", [])],
                                    and(vec![
                                        eq(a.clone(), atom("value")),
                                        eq(b.clone(), atom("value")),
                                    ]),
                                ),
                            );
                            facts.push(c("bind", []));
                        }
                        let query = Query {
                            constraints: facts,
                            outputs: vec![
                                ("a".into(), Var(10)),
                                ("b".into(), if distinct { Var(11) } else { Var(10) }),
                            ],
                        };
                        let control = execute(rules.clone(), query.clone(), policy, Access::Scan);
                        let candidate = execute(rules, query, policy, Access::Indexed);
                        assert_eq!(
                            candidate, control,
                            "wrapped={wrapped} distinct={distinct} duplicates={duplicates} bind={bind} policy={policy:?}"
                        );
                        cases += 1;
                    }
                }
            }
        }
    }
    assert_eq!(cases, 32);
    println!("PROBE_OPEN,cases={cases}");
}

#[test]
fn broad_probe_retains_duplicates_and_exposes_extra_work() {
    for n in [8, 32, 128] {
        let mut facts = vec![c("request", [atom("b")])];
        for i in 0..n {
            facts.push(c("left", [atom(&format!("a{i}")), atom(&format!("x{i}"))]));
            facts.push(c("right", [atom(&format!("x{i}")), atom("b")]));
        }
        let mut expected = facts.clone();
        expected.push(c("receipt", [atom("a0"), atom("x0"), atom("b")]));
        expected.sort();
        facts.push(c("token", []));
        for access in [Access::Scan, Access::Indexed] {
            let prepared = PreparedRuleset::new(vec![join_rule(false, false)], None).unwrap();
            let mut engine = prepared
                .start(
                    Query {
                        constraints: facts.clone(),
                        outputs: vec![],
                    },
                    Policy::Global,
                    access,
                )
                .unwrap();
            assert!(engine.advance(2_000_000).exhausted);
            let mut answer = engine.observe().unwrap();
            answer.residual.sort();
            assert_eq!(answer.residual, expected);
            let s = engine.stats();
            println!(
                "PROBE_BROAD,n={n},access={access:?},metrics={},candidate={},structural={},index={},probe_starts={},probe_visits={},probe_candidates={}",
                chr_compiled::COLLECT_METRICS,
                s.candidate_visits,
                s.structural_tests,
                s.index_lookups,
                s.probe_starts,
                s.probe_visits,
                s.probe_candidates
            );
        }
    }
}
