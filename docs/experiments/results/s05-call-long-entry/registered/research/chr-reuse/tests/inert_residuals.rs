#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_syntax::{Query, Rule, Var, and, atom, c, eq, or, t, v};
#[test]
fn different_callers_reuse_active_work_and_keep_their_own_tags() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(
                and([c("caller", [atom("a")]).into(), c("work", [v(0)]).into()]),
                and([c("caller", [atom("b")]).into(), c("work", [v(0)]).into()]),
            ),
        ),
        Rule::simplify(
            "finish",
            [c("work", [v(0)])],
            eq(v(0), t("pair", [v(99), v(99)])),
        ),
    ];
    let q = Query {
        constraints: vec![c("start", [v(7)])],
        outputs: vec![("out".into(), Var(7))],
    };
    let prepared = chr_reuse::residuals::Prepared::new(rules.clone(), true).unwrap();
    let mut run = prepared.start(q.clone()).unwrap();
    let batch = run.advance(200_000);
    assert!(batch.exhausted);
    oracle::same_raw(batch.answers, oracle::run(&rules, &q, 200_000));
    if chr_reuse::continuations::COLLECT_METRICS {
        assert!(
            run.stats().hits > 0,
            "different inert callers must reuse execution"
        );
    }
}

fn nat(n: usize) -> chr_syntax::Term {
    (0..n).fold(atom("z"), |n, _| t("s", [n]))
}
fn ordered(a: &[chr_syntax::Answer], b: &[chr_syntax::Answer]) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b)
            .all(|(a, b)| chr_observe::equivalent(a, b, &mut Default::default()))
}
fn source(kind: usize, depth: usize, distinct: bool, offset: u64) -> (Vec<Rule>, Query) {
    use chr_syntax::Goal;
    let body = |tag: &str, fail: bool| {
        let marker = if kind == 2 { v(1) } else { atom(tag) };
        let mut parts = vec![c("caller", [marker]).into()];
        if fail {
            parts.push(Goal::Fail);
        } else {
            parts.push(c("work", [nat(depth), v(0), v(1)]).into());
            if kind == 2 {
                parts.push(c("bind", [v(1), atom(tag)]).into());
            }
        }
        and(parts)
    };
    let mut step = vec![c("work", [v(1), v(0), v(2)]).into()];
    if kind == 4 {
        step.extend([
            c("note", [t("s", [v(1)])]).into(),
            c("note", [t("s", [v(1)])]).into(),
        ]);
    }
    let mut rules = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(
                body("a", kind == 5),
                body(if distinct { "b" } else { "a" }, false),
            ),
        ),
        Rule::simplify("step", [c("work", [t("s", [v(1)]), v(0), v(2)])], and(step)),
        Rule::simplify(
            "finish",
            [c("work", [atom("z"), v(0), v(2)])],
            eq(
                v(0),
                if kind == 2 {
                    t("pair", [v(2), v(99), v(99)])
                } else {
                    t("pair", [v(99), v(99)])
                },
            ),
        ),
    ];
    if kind == 1 {
        rules.push(Rule::propagate(
            "read-caller",
            [c("caller", [v(3)])],
            c("seen", [v(3)]).into(),
        ));
    }
    if kind == 2 {
        rules.push(Rule::simplify(
            "late-bind",
            [c("bind", [v(0), v(1)])],
            eq(v(0), v(1)),
        ));
    }
    if kind == 3 {
        rules.push(Rule::simplify(
            "read-other-arity",
            [c("caller", [v(0), v(1)])],
            Goal::Fail,
        ));
    }
    (
        rules,
        Query {
            constraints: vec![c("start", [v(offset)])],
            outputs: vec![("out".into(), Var(offset))],
        },
    )
}
fn check(source: &[Rule], q: &Query) -> (u64, u64, usize) {
    use chr_reuse::{
        continuations::{COLLECT_METRICS, Mode, Prepared as Whole},
        residuals::Prepared,
    };
    let expected = oracle::run(source, q, 200_000);
    let direct = Whole::new(source.to_vec(), Mode::Direct).unwrap();
    let whole = Whole::new(source.to_vec(), Mode::AlphaLive).unwrap();
    let direct_sep = Prepared::new(source.to_vec(), false).unwrap();
    let shared_sep = Prepared::new(source.to_vec(), true).unwrap();
    let coarse4 = Prepared::new(source.to_vec(), true)
        .unwrap()
        .with_recognition_stride(4);
    let coarse16 = Prepared::new(source.to_vec(), true)
        .unwrap()
        .with_recognition_stride(16);
    let mut coarse_runs = [
        coarse4.start(q.clone()).unwrap(),
        coarse16.start(q.clone()).unwrap(),
    ];
    let (mut a, mut b, mut c, mut d) = (
        direct.start(q.clone()).unwrap(),
        whole.start(q.clone()).unwrap(),
        direct_sep.start(q.clone()).unwrap(),
        shared_sep.start(q.clone()).unwrap(),
    );
    let mut actual = vec![];
    let mut steps = 0;
    let mut done = false;
    for _ in 0..200_000 {
        let aa = a.advance(1);
        let bb = b.advance(1);
        let cc = c.advance(1);
        let dd = d.advance(1);
        for run in &mut coarse_runs {
            let batch = run.advance(1);
            assert!(ordered(&aa.answers, &batch.answers));
            assert_eq!(aa.exhausted, batch.exhausted);
        }
        assert!(ordered(&aa.answers, &bb.answers));
        assert!(ordered(&aa.answers, &cc.answers));
        assert!(ordered(&aa.answers, &dd.answers));
        assert_eq!(aa.exhausted, bb.exhausted);
        assert_eq!(aa.exhausted, cc.exhausted);
        assert_eq!(aa.exhausted, dd.exhausted);
        actual.extend(dd.answers);
        steps += 1;
        if aa.exhausted {
            done = true;
            break;
        }
    }
    assert!(done);
    oracle::same_raw(actual.clone(), expected.clone());
    let hits = (b.stats().hits, d.stats().hits);
    if COLLECT_METRICS {
        assert_eq!(a.stats().logical_steps, d.stats().logical_steps);
        assert!(
            c.stats().states <= c.stats().max_frontier + 2,
            "uncached separation must not retain a slot for every past transition"
        );
        println!("whole={:?} separated={:?}", b.stats(), d.stats());
    }
    drop(a);
    drop(b);
    drop(c);
    drop(d);
    for prepared in [&direct_sep, &shared_sep, &coarse4, &coarse16] {
        for cutoff in [0, 1, 5] {
            let mut cancelled = prepared.start(q.clone()).unwrap();
            let batch = cancelled.advance(cutoff);
            assert!(!batch.exhausted);
            drop(cancelled);
            let mut fresh = prepared.start(q.clone()).unwrap();
            let result = fresh.advance(200_000);
            assert!(result.exhausted);
            oracle::same_raw(result.answers, expected.clone());
        }
    }
    drop(direct);
    drop(whole);
    drop(direct_sep);
    drop(shared_sep);
    oracle::same_raw(actual, expected);
    (hits.0, hits.1, steps)
}
#[test]
fn matrix_preserves_every_delivery_and_challenges_inertness() {
    let (mut cases, mut comparisons, mut whole, mut separated) = (0, 0, 0, 0);
    for kind in 0..6 {
        for depth in [0, 1, 4] {
            for offset in [0, 100] {
                for distinct in [false, true] {
                    let (rules, q) = source(kind, depth, distinct, offset);
                    let (w, s, steps) = check(&rules, &q);
                    if chr_reuse::continuations::COLLECT_METRICS
                        && distinct
                        && [0, 3, 4].contains(&kind)
                    {
                        assert_eq!(w, 0);
                        assert!(s > 0);
                    }
                    println!(
                        "case kind={kind} depth={depth} offset={offset} distinct={distinct} whole_hits={w} separated_hits={s} steps={steps}"
                    );
                    cases += 1;
                    comparisons += steps * 5;
                    whole += w;
                    separated += s;
                }
            }
        }
    }
    assert_eq!(cases, 72);
    println!(
        "matrix cases={cases} per-step comparisons={comparisons} whole_hits={whole} separated_hits={separated} cancellation/restart={}",
        cases * 12
    );
}
#[test]
fn active_consumption_and_history_remain_in_resumable_state() {
    for competition in [false, true] {
        let body = |tag| {
            and([
                c("caller", [atom(tag)]).into(),
                c("work", [v(0)]).into(),
                c("flag", []).into(),
                c("token", []).into(),
            ])
        };
        let mut rules = vec![
            Rule::simplify("start", [c("start", [v(0)])], or(body("a"), body("b"))),
            Rule::propagate("record", [c("flag", [])], c("recorded", []).into()),
            Rule::simplify(
                "step",
                [c("work", [v(0)])],
                and([c("ready", [v(0)]).into(), c("pulse", []).into()]),
            ),
        ];
        if competition {
            rules.push(Rule::simplify(
                "steal",
                [c("pulse", []), c("token", [])],
                c("stolen", []).into(),
            ));
        }
        rules.push(Rule::simplify(
            "finish",
            [c("ready", [v(0)]), c("token", [])],
            eq(v(0), t("pair", [v(99), v(99)])),
        ));
        let q = Query {
            constraints: vec![c("start", [v(7)])],
            outputs: vec![("out".into(), Var(7))],
        };
        let (w, s, _) = check(&rules, &q);
        if chr_reuse::continuations::COLLECT_METRICS {
            assert_eq!(w, 0);
            assert!(s > 0);
        }
    }
}
#[test]
#[should_panic(expected = "cursor belongs to a different machine")]
fn extraction_rejects_another_machines_cursor() {
    use chr_persistent::continuations::PreparedMachine;
    let p = PreparedMachine::new(vec![]).unwrap();
    let q = Query {
        constraints: vec![c("caller", [atom("a")])],
        outputs: vec![],
    };
    let (mut a, _) = p.start(q.clone()).unwrap();
    let (_, mut cursor) = p.start(q).unwrap();
    a.detach_inert_ground(&mut cursor);
}

#[test]
fn coarser_recognition_reduces_key_work_without_inventing_reuse() {
    for single_branch in [false, true] {
        let (mut rules, q) = source(0, 16, true, 100);
        if single_branch {
            let chr_syntax::Goal::Or(left, _) = &rules[0].body else {
                unreachable!()
            };
            rules[0].body = *left.clone();
        }
        let expected = oracle::run(&rules, &q, 200_000);
        let mut counts = vec![];
        for stride in [1, 4, 16] {
            let mut run = chr_reuse::residuals::Prepared::new(rules.clone(), true)
                .unwrap()
                .with_recognition_stride(stride)
                .start(q.clone())
                .unwrap();
            let result = run.advance(200_000);
            assert!(result.exhausted);
            assert!(
                result
                    .answers
                    .iter()
                    .all(|a| a.residual.capacity() == a.residual.len()),
                "delivered residual retains unused growth capacity"
            );
            oracle::same_raw(result.answers, expected.clone());
            counts.push((
                run.stats().key_requests,
                run.stats().hits,
                run.stats().logical_steps,
                run.stats().states,
            ));
        }
        if chr_reuse::continuations::COLLECT_METRICS {
            assert!(counts[1].0 < counts[0].0 && counts[2].0 < counts[1].0);
            assert!(counts.iter().all(|v| v.2 == counts[0].2));
            if single_branch {
                assert!(counts.iter().all(|v| v.1 == 0));
            } else {
                assert!(counts.iter().all(|v| v.1 > 0));
            }
        }
        println!("single_branch={single_branch} stride1/4/16 (keys,hits,steps,states): {counts:?}");
    }
}
#[test]
#[should_panic(expected = "recognition stride must be positive")]
fn zero_recognition_stride_is_rejected() {
    chr_reuse::residuals::Prepared::new(vec![], true)
        .unwrap()
        .with_recognition_stride(0);
}
