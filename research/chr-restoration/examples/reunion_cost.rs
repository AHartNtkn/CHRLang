//! Allocation-only complete-source reunion ownership gate.
use chr_compiled::experiment::meter;
use chr_restoration::{Mode, Prepared, Step, reunion::PreparedPhase};
use chr_syntax::{Answer, Query, Rule};
use std::sync::Arc;
#[path = "support/reunion_source.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use fixture::{query, source};
enum Plan {
    Copy(Arc<Prepared>),
    Reunion(Arc<PreparedPhase>),
    Indexed(chr_compiled::PreparedRuleset),
    Factored(Vec<Rule>),
}
#[allow(clippy::large_enum_variant)]
enum Running {
    Copy(chr_restoration::Engine),
    Reunion(chr_restoration::reunion::ReunionEngine),
    Indexed(chr_compiled::SearchEngine),
    Factored(chr_factors::Search),
}
impl Plan {
    fn new(mode: &str, rules: &[Rule], n: usize) -> Self {
        match mode {
            "copy" => Self::Copy(Prepared::new(rules).unwrap()),
            "reunion" => Self::Reunion(PreparedPhase::new(rules, n).unwrap()),
            "indexed" => {
                Self::Indexed(chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap())
            }
            "factored" => Self::Factored(rules.to_vec()),
            _ => panic!("unknown mode"),
        }
    }
    fn start(&self, q: &Query) -> Running {
        match self {
            Self::Copy(p) => Running::Copy(p.start(q, Mode::Copy).unwrap()),
            Self::Reunion(p) => Running::Reunion(p.start(q).unwrap()),
            Self::Indexed(p) => Running::Indexed(
                p.start_search(
                    q.clone(),
                    chr_compiled::Policy::Global,
                    chr_compiled::Access::Indexed,
                )
                .unwrap(),
            ),
            Self::Factored(r) => {
                let s = chr_factors::Search::new(r.clone(), q.clone(), chr_factors::Mode::Factored)
                    .unwrap();
                assert_eq!(
                    s.factor_count(),
                    if q.constraints.iter().any(|c| c.name == "payload") {
                        2
                    } else {
                        1
                    }
                );
                Running::Factored(s)
            }
        }
    }
}
fn execute(e: &mut Running, first_only: bool) -> Vec<Answer> {
    let mut answers = vec![];
    for _ in 0..200_000 {
        let (mut out, done) = match e {
            Running::Copy(e) => match e.advance() {
                Step::Answer(a) => (vec![a], false),
                Step::Exhausted => (vec![], true),
                Step::Progress => (vec![], false),
            },
            Running::Reunion(e) => match e.advance().unwrap() {
                Step::Answer(a) => (vec![a], false),
                Step::Exhausted => (vec![], true),
                Step::Progress => (vec![], false),
            },
            Running::Indexed(e) => match e.tick() {
                chr_compiled::SearchEvent::Complete(mut b) => {
                    (vec![b.engine.observe().unwrap()], false)
                }
                chr_compiled::SearchEvent::Exhausted => (vec![], true),
                _ => (vec![], false),
            },
            Running::Factored(e) => {
                let b = e.advance(1);
                (b.answers, b.exhausted)
            }
        };
        answers.append(&mut out);
        if done || (first_only && !answers.is_empty()) {
            return answers;
        }
    }
    panic!("service budget exhausted");
}
fn measure<T>(f: impl FnOnce() -> T) -> (T, meter::Reading) {
    let s = meter::begin();
    let x = f();
    (x, meter::end(s))
}
fn main() {
    assert!(
        !std::hint::black_box(cfg!(feature = "replay-diagnostic")),
        "allocation gate must have work diagnostics disabled"
    );
    meter::self_check().unwrap();
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    assert_eq!(args.len(), 5);
    let mode = &args[0];
    let family = &args[1];
    let owners = args[2].parse::<usize>().unwrap();
    let depth = args[3].parse::<usize>().unwrap();
    let reuse = args[4].parse::<usize>().unwrap();
    assert!(["copy", "reunion", "indexed", "factored"].contains(&mode.as_str()));
    assert!(["plain", "equal", "late", "payload"].contains(&family.as_str()));
    assert!([2, 4].contains(&owners) && [0, 12, 48].contains(&depth) && [1, 4].contains(&reuse));
    assert!(!cfg!(feature = "arena-cow") || mode == "indexed");
    let (rules, n) = source(family, owners);
    let expected = (0..reuse)
        .map(|i| oracle::run(&rules, &query(family, owners, depth, i), 200_000))
        .collect::<Vec<_>>();
    for xs in &expected {
        assert_eq!(xs.len(), if family == "equal" { 2 } else { 1 << owners });
    }
    let warm = Plan::new(mode, &rules, n);
    for (i, expected) in expected.iter().enumerate() {
        oracle::same_raw(
            execute(&mut warm.start(&query(family, owners, depth, i)), false),
            expected.clone(),
        );
    }
    drop(warm);
    // Fixed bookkeeping storage is outside the measured ownership baseline.
    let mut phases: Vec<(&str, meter::Reading)> = Vec::with_capacity(2 + reuse * 6 + 6);
    let (p, prep) = measure(|| Plan::new(mode, &rules, n));
    phases.push(("prepare", prep));
    for (i, expected) in expected.iter().enumerate() {
        let (q, m) = measure(|| query(family, owners, depth, i));
        phases.push(("input", m));
        let (mut e, m) = measure(|| p.start(&q));
        phases.push(("setup", m));
        let (answers, m) = measure(|| execute(&mut e, false));
        phases.push(("execute", m));
        oracle::same_raw(answers.clone(), expected.clone());
        let (_, m) = measure(|| drop(e));
        phases.push(("engine-drop", m));
        let (_, m) = measure(|| drop(answers));
        phases.push(("answers-drop", m));
        let (_, m) = measure(|| drop(q));
        phases.push(("input-drop", m));
        assert_eq!(
            m.live_end, prep.live_end,
            "query retained state in prepared owner"
        );
    }
    let (q, m) = measure(|| query(family, owners, depth, 0));
    phases.push(("cancel-input", m));
    let (mut e, m) = measure(|| p.start(&q));
    phases.push(("cancel-setup", m));
    let (answers, m) = measure(|| execute(&mut e, true));
    phases.push(("cancel-first", m));
    assert_eq!(answers.len(), 1);
    assert!(expected[0].iter().any(|x| chr_observe::equivalent(
        x,
        &answers[0],
        &mut Default::default()
    )));
    let (_, m) = measure(|| drop(e));
    phases.push(("cancel-engine-drop", m));
    let (_, m) = measure(|| drop(answers));
    phases.push(("cancel-answers-drop", m));
    let (_, m) = measure(|| drop(q));
    phases.push(("cancel-input-drop", m));
    assert_eq!(
        m.live_end, prep.live_end,
        "cancellation retained query state"
    );
    let (_, m) = measure(|| drop(p));
    phases.push(("prepared-drop", m));
    assert!(
        phases
            .windows(2)
            .all(|xs| xs[0].1.live_end == xs[1].1.live_start),
        "unmeasured retained allocation"
    );
    assert_eq!(
        phases.last().unwrap().1.live_end,
        prep.live_start,
        "prepared lifecycle not released"
    );
    let readings = phases
        .iter()
        .map(|(name, m)| format!("{{\"phase\":\"{name}\",\"memory\":{}}}", m.json()))
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"owners\":{owners},\"depth\":{depth},\"reuse\":{reuse},\"cow\":{},\"phases\":[{readings}]}}",
        cfg!(feature = "arena-cow")
    );
}
