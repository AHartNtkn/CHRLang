#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[allow(dead_code)]
#[path = "support/effectful_probe.rs"]
mod probe;
use chr_syntax::{Answer, Goal, Query, Rule, Var, and, atom, c, eq, or, t, v};
use std::collections::BTreeSet;

fn ordered(a: &[Answer], b: &[Answer]) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b)
            .all(|(a, b)| chr_observe::equivalent(a, b, &mut Default::default()))
}
fn nat(n: usize) -> chr_syntax::Term {
    (0..n).fold(atom("z"), |n, _| t("s", [n]))
}
fn prefix_hypothesis(source: &[Rule], selected: &[Rule]) -> Option<Vec<String>> {
    if selected.is_empty()
        || source.get(..selected.len()) != Some(selected)
        || selected.iter().any(|r| r.removed.is_empty())
    {
        return None;
    }
    Some(
        selected
            .iter()
            .flat_map(|r| r.kept.iter().chain(&r.removed))
            .map(|c| c.name.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
    )
}
fn ordered_source(a: usize, b: usize, reverse: bool, offset: u64) -> (Vec<Rule>, Query) {
    let mut branches: [Goal; 2] = [
        c("tail", [nat(a), atom("a"), v(0)]).into(),
        c("tail", [nat(b), atom("b"), v(0)]).into(),
    ];
    if reverse {
        branches.swap(0, 1);
    }
    let source = vec![
        Rule::simplify(
            "phase",
            [c("work", [v(0)]), c("token", [])],
            or(branches[0].clone(), branches[1].clone()),
        ),
        Rule::simplify(
            "tail-step",
            [c("tail", [t("s", [v(0)]), v(1), v(2)])],
            c("tail", [v(0), v(1), v(2)]).into(),
        ),
        Rule::simplify(
            "tail-finish",
            [c("tail", [atom("z"), v(0), v(1)])],
            eq(v(1), v(0)),
        ),
    ];
    (
        source,
        Query {
            constraints: vec![c("work", [v(offset)]), c("token", [])],
            outputs: vec![("result".into(), Var(offset))],
        },
    )
}
#[test]
fn a_checked_priority_phase_can_change_the_first_caller_answer() {
    let (mut checked, mut changed) = (0, 0);
    for a in [0, 1, 4, 8] {
        for b in [0, 1, 4, 8] {
            for reverse in [false, true] {
                for offset in [0, 100] {
                    let (source, q) = ordered_source(a, b, reverse, offset);
                    let selected = &source[..1];
                    let names = prefix_hypothesis(&source, selected).unwrap();
                    let expected = oracle::run(&source, &q, 200_000);
                    let direct = probe::direct(&source, &q);
                    oracle::same_raw(direct.clone(), expected.clone());
                    for mode in [
                        chr_reuse::continuations::Mode::AlphaLive,
                        chr_reuse::continuations::Mode::CompactLive,
                    ] {
                        let mut resumed =
                            chr_reuse::continuations::Search::new(source.clone(), q.clone(), mode)
                                .unwrap();
                        let batch = resumed.advance(200_000);
                        assert!(batch.exhausted);
                        assert!(ordered(&batch.answers, &direct));
                    }

                    for mode in [probe::Key::None, probe::Key::Region] {
                        let mut p = probe::Probe::new(
                            selected.to_vec(),
                            names.iter().map(String::as_str).collect(),
                            mode,
                        );
                        let got = p.run(&source, &q);
                        oracle::same_raw(got.clone(), expected.clone());
                        let second = p.run(&source, &q);
                        assert!(ordered(&got, &second));
                        let difference = !ordered(&got, &direct);
                        changed += usize::from(difference);
                        checked += 1;
                        println!(
                            "phase a={a} b={b} reverse={reverse} offset={offset} mode={mode:?} changed={difference} direct={direct:?} contracted={got:?}"
                        );
                    }
                }
            }
        }
    }
    assert_eq!(checked, 128);
    assert!(changed > 0);
    // Both branches have the same prefix; eight consuming tail steps ensure b
    // reaches its output before a under direct FIFO, regardless of a tie convention.
    let (source, q) = ordered_source(8, 0, false, 0);
    let direct = probe::direct(&source, &q);
    let expected = oracle::run(&source, &q, 200_000);
    assert_eq!(direct[0].outputs[0].1, atom("b"));
    assert_eq!(expected[0].outputs[0].1, atom("b"));
    let mut p = probe::Probe::new(
        source[..1].to_vec(),
        vec!["work", "token"],
        probe::Key::Region,
    );
    assert_eq!(p.run(&source, &q)[0].outputs[0].1, atom("a"));
    println!("phase total={checked} ordered differences={changed}");
}
#[test]
fn boundary_preconditions_do_not_admit_intervening_rules_or_surviving_history() {
    let (source, _) = ordered_source(0, 0, false, 0);
    assert!(prefix_hypothesis(&source, &source[..1]).is_some());
    assert!(prefix_hypothesis(&source, &source[1..]).is_none());
    let propagation = vec![Rule::propagate(
        "record",
        [c("fact", [])],
        c("seen", []).into(),
    )];
    assert!(prefix_hypothesis(&propagation, &propagation).is_none());
}

fn effect_source(kind: usize, distinct: bool, offset: u64) -> (Vec<Rule>, Query) {
    let body = |tag: &str, second: bool| {
        let work = match kind {
            0 => and([c("work", [v(0)]).into(), c("token", []).into()]),
            1 => c("work", [v(0)]).into(),
            2 => and([c("work", [v(1), v(0)]).into(), c("supply", [v(1)]).into()]),
            3 => and([c("fact", [v(0)]).into(), c("work", [v(0)]).into()]),
            4 => c("work", [v(0)]).into(),
            5 => {
                if second {
                    eq(v(0), t("pair", [v(99), v(99)]))
                } else {
                    Goal::Fail
                }
            }
            _ => unreachable!(),
        };
        and([c("caller", [atom(tag)]).into(), work])
    };
    let mut rules = vec![Rule::simplify(
        "start",
        [c("start", [v(0)])],
        or(
            body("a", false),
            body(if distinct { "b" } else { "a" }, true),
        ),
    )];
    match kind {
        0 => rules.extend([
            Rule::simplify(
                "step",
                [c("work", [v(0)])],
                and([c("ready", [v(0)]).into(), c("pulse", []).into()]),
            ),
            Rule::simplify(
                "steal",
                [c("pulse", []), c("token", [])],
                c("stolen", []).into(),
            ),
            Rule::simplify(
                "finish",
                [c("ready", [v(0)]), c("token", [])],
                eq(v(0), atom("won")),
            ),
        ]),
        1 => rules.extend([
            Rule::simplify(
                "step",
                [c("work", [v(0)])],
                and([c("ready", [v(0)]).into(), c("pulse", []).into()]),
            ),
            Rule::propagate("observe", [c("pulse", [])], c("observed", []).into()),
            Rule::simplify(
                "finish",
                [c("ready", [v(0)]), c("pulse", [])],
                eq(v(0), t("pair", [v(99), v(99)])),
            ),
        ]),
        2 => rules.extend([
            Rule::simplify("supply", [c("supply", [v(0)])], eq(v(0), atom("known"))),
            Rule::simplify(
                "known",
                [c("work", [atom("known"), v(0)])],
                eq(v(0), atom("known")),
            ),
            Rule::simplify(
                "generic",
                [c("work", [v(1), v(0)])],
                eq(v(0), atom("generic")),
            ),
        ]),
        3 => rules.extend([
            Rule::propagate("record", [c("fact", [v(0)])], c("recorded", [v(0)]).into()),
            Rule::simplify(
                "finish",
                [c("work", [v(0)])],
                eq(v(0), t("pair", [v(99), v(99)])),
            ),
        ]),
        4 => rules.push(Rule::simplify(
            "need-token",
            [c("work", [v(0)]), c("token", [])],
            eq(v(0), atom("present")),
        )),
        5 => (),
        _ => unreachable!(),
    }
    (
        rules,
        Query {
            constraints: vec![c("start", [v(offset)])],
            outputs: vec![("out".into(), Var(offset))],
        },
    )
}
#[test]
fn resumable_transition_reuse_preserves_effects_and_each_fifo_delivery() {
    use chr_reuse::continuations::{COLLECT_METRICS, Mode, Prepared};
    let (mut cases, mut compared_steps, mut hits_same, mut hits_distinct) = (0, 0, 0, 0);
    let mut unfinished_drops = 0;
    for kind in 0..6 {
        for distinct in [false, true] {
            for offset in [0, 100] {
                let (source, q) = effect_source(kind, distinct, offset);
                let expected = oracle::run(&source, &q, 200_000);
                let mut retained = vec![];
                for mode in [Mode::AlphaLive, Mode::CompactLive] {
                    let direct_prepared = Prepared::new(source.clone(), Mode::Direct).unwrap();
                    let prepared = Prepared::new(source.clone(), mode).unwrap();
                    let mut direct = direct_prepared.start(q.clone()).unwrap();
                    let mut reuse = prepared.start(q.clone()).unwrap();
                    let mut all = vec![];
                    let mut done = false;
                    for _ in 0..200_000 {
                        let a = direct.advance(1);
                        let b = reuse.advance(1);
                        assert!(
                            ordered(&a.answers, &b.answers),
                            "kind={kind} distinct={distinct} mode={mode:?}"
                        );
                        assert_eq!(a.exhausted, b.exhausted);
                        all.extend(b.answers);
                        compared_steps += 1;
                        if a.exhausted {
                            done = true;
                            break;
                        }
                    }
                    assert!(done);
                    oracle::same_raw(all.clone(), expected.clone());
                    if COLLECT_METRICS {
                        assert_eq!(reuse.stats().logical_steps, direct.stats().logical_steps);
                        if distinct {
                            hits_distinct += reuse.stats().hits;
                        } else {
                            hits_same += reuse.stats().hits;
                        }
                        println!(
                            "resumable kind={kind} distinct={distinct} offset={offset} mode={mode:?} direct={:?} reuse={:?}",
                            direct.stats(),
                            reuse.stats()
                        );
                    }
                    drop(reuse);
                    drop(direct);
                    retained.push(all);
                    // Cancellation is a real pending Search disposal. Prepared rules remain
                    // available; transition tables are deliberately per-query owners.
                    for cut in [0, 1, 5, 13] {
                        let mut abandoned = prepared.start(q.clone()).unwrap();
                        let cancelled = abandoned.advance(cut);
                        unfinished_drops += usize::from(!cancelled.exhausted);
                        let delivered = cancelled.answers;
                        drop(abandoned);
                        let mut restarted = prepared.start(q.clone()).unwrap();
                        let full = restarted.advance(200_000);
                        assert!(full.exhausted);
                        oracle::same_raw(full.answers.clone(), expected.clone());
                        for answer in delivered {
                            assert!(expected.iter().any(|e| chr_observe::equivalent(
                                &answer,
                                e,
                                &mut Default::default()
                            )));
                        }
                        retained.push(full.answers);
                    }
                    drop(prepared);
                    drop(direct_prepared);
                }
                for answers in retained {
                    oracle::same_raw(answers, expected.clone());
                }
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 24);
    if COLLECT_METRICS {
        assert!(hits_same > 0);
        assert_eq!(hits_distinct, 0);
    }
    println!(
        "resumable sources={cases} logical delivery comparisons={compared_steps} same-caller hits={hits_same} distinct-caller hits={hits_distinct} disposal/restart pairs={} unfinished drops={unfinished_drops}",
        cases * 2 * 4
    );
}
