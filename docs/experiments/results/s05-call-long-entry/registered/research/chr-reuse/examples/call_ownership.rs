//! Prospective requested-heap ownership gate; no timing claims.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use chr_compiled::experiment::meter;
use chr_reuse::calls::{Caller, CallerEvent};
use chr_syntax::{atom, c, eq, or, t, v, Query, Rule, Var};
fn main() {
    if chr_reuse::continuations::COLLECT_METRICS || chr_persistent::COLLECT_METRICS {
        panic!("ownership probe requires counters disabled");
    }
    meter::self_check().unwrap();
    let family = vec![
        Rule::simplify(
            "step",
            [c("work", [t("s", [v(0)]), v(1), v(2)])],
            c("work", [v(0), v(1), v(2)]).into(),
        ),
        Rule::simplify(
            "finish",
            [c("work", [atom("z"), v(0), v(1)])],
            or(
                eq(v(1), t("pair", [v(0), v(99), v(99)])),
                eq(v(1), t("pair", [v(0), v(99), v(99)])),
            ),
        ),
        Rule::simplify("supply", [c("supply", [v(0)])], eq(v(0), atom("a"))),
    ];
    for memo in [false, true] {
        for n in [0, 8] {
            for cancel in 0..4 {
                let depth = (0..n).fold(atom("z"), |x, _| t("s", [x]));
                let query = Query {
                    constraints: vec![c("work", [depth, v(7), v(8)]), c("supply", [v(7)])],
                    outputs: vec![("input".into(), Var(7)), ("result".into(), Var(8))],
                };
                let expected = oracle::run(&family, &query, 200_000);
                assert_eq!(expected.len(), 2);
                let mut answers = Vec::with_capacity(2);
                let mut retained = [0usize; 2];
                let mut released = [0usize; 2];
                let mut counts = [0usize; 2];
                let owner = meter::begin();
                let mut caller = Caller::new(family.clone(), 2, memo).unwrap();
                let prepared = meter::end(owner).live_end;
                for q in 0..2 {
                    let mut run = caller.start(query.clone(), 200_000, 200_000);
                    if cancel > 0 {
                        assert!(matches!(
                            caller.step(&mut run).unwrap(),
                            CallerEvent::Progress
                        ));
                        if cancel > 1 {
                            let mut done = false;
                            for _ in 0..200_000 {
                                match caller.step(&mut run).unwrap() {
                                    CallerEvent::Progress => (),
                                    CallerEvent::Answer(a) => {
                                        answers.push(a);
                                        if cancel == 2 {
                                            break;
                                        }
                                    }
                                    CallerEvent::Done => {
                                        done = true;
                                        break;
                                    }
                                }
                            }
                            if cancel == 3 {
                                assert!(done);
                            }
                        }
                    }
                    drop(run);
                    retained[q] = meter::end(owner).live_end;
                    counts[q] = answers.len();
                    assert_eq!(
                        answers.len(),
                        match cancel {
                            0 | 1 => 0,
                            2 => 1,
                            _ => 2,
                        }
                    );
                    // Validate between live-heap snapshots. Only live bytes
                    // are reported; comparator traffic and peaks are excluded.
                    for answer in &answers {
                        assert!(chr_observe::equivalent(
                            answer,
                            &expected[0],
                            &mut Default::default()
                        ));
                    }
                    answers.clear();
                    released[q] = meter::end(owner).live_end;
                }
                assert_eq!(
                    released[0], released[1],
                    "identical repeated query retained more state"
                );
                if !memo {
                    assert_eq!(released[0], prepared, "direct query retained state");
                }
                drop(caller);
                let end = meter::end(owner);
                assert_eq!(end.live_end, end.live_start, "prepared owner leaked");
                println!("{{\"memo\":{memo},\"depth\":{n},\"cancel\":{cancel},\"baseline\":{},\"prepared\":{prepared},\"retained\":{retained:?},\"released\":{released:?},\"answers\":{counts:?},\"disposed\":{}}}",end.live_start,end.live_end);
            }
        }
    }
}
