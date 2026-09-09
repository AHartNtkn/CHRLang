//! Allocation-only complete-source reunion ownership gate.
use chr_compiled::experiment::meter;
use chr_restoration::{Mode, Prepared, Step, reunion::PreparedPhase};
use chr_syntax::{Answer, Query, Rule};
use std::sync::Arc;
#[path = "support/repeated_source.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
use fixture::{query, source};
enum Plan {
    Copy(Arc<Prepared>),
    Reunion(Arc<PreparedPhase>),
    Repeated(Arc<PreparedPhase>),
}
#[allow(clippy::large_enum_variant)]
enum Running {
    Copy(chr_restoration::Engine),
    Reunion(chr_restoration::reunion::ReunionEngine),
    Repeated(chr_restoration::reunion::RepeatedEngine),
}
impl Plan {
    fn new(mode: &str, rules: &[Rule], n: usize) -> Self {
        match mode {
            "copy" => Self::Copy(Prepared::new(rules).unwrap()),
            "reunion" => Self::Reunion(PreparedPhase::new(rules, n).unwrap()),
            "repeated" => Self::Repeated(PreparedPhase::new(rules, n).unwrap()),
            _ => panic!("unknown mode"),
        }
    }
    fn start(&self, q: &Query) -> Running {
        match self {
            Self::Copy(p) => Running::Copy(p.start(q, Mode::Copy).unwrap()),
            Self::Reunion(p) => Running::Reunion(p.start(q).unwrap()),
            Self::Repeated(p) => Running::Repeated(p.start_repeated(q).unwrap()),
        }
    }
}
fn execute(e: &mut Running, first_only: bool) -> Vec<Answer> {
    let mut answers = vec![];
    for _ in 0..200_000 {
        let event = match e {
            Running::Copy(e) => e.advance(),
            Running::Reunion(e) => e.advance().unwrap(),
            Running::Repeated(e) => e.advance().unwrap(),
        };
        match event {
            Step::Answer(a) => answers.push(a),
            Step::Exhausted => return answers,
            Step::Progress => (),
        }
        if first_only && !answers.is_empty() {
            return answers;
        }
    }
    panic!("service budget exhausted")
}
fn same_borrowed(actual: &[Answer], expected: &[Answer]) {
    assert_eq!(actual.len(), expected.len());
    let mut used = vec![false; expected.len()];
    for a in actual {
        let i = expected
            .iter()
            .enumerate()
            .position(|(i, b)| !used[i] && chr_observe::equivalent(a, b, &mut Default::default()))
            .expect("answer mismatch");
        used[i] = true;
    }
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
    let rounds = args[2].parse::<usize>().unwrap();
    let depth = args[3].parse::<usize>().unwrap();
    let reuse = args[4].parse::<usize>().unwrap();
    assert!(["copy", "reunion", "repeated"].contains(&mode.as_str()));
    assert!(["plain", "history", "late"].contains(&family.as_str()));
    assert!([0, 1, 3].contains(&rounds) && [0, 12].contains(&depth) && [1, 4].contains(&reuse));
    assert!(!cfg!(feature = "arena-cow"));
    let (rules, n) = source(family, depth);
    let expected = (0..reuse)
        .map(|i| oracle::run(&rules, &query(family, rounds, depth, i), 200_000))
        .collect::<Vec<_>>();
    for xs in &expected {
        assert_eq!(xs.len(), 2usize.pow(rounds as u32 + 1));
    }
    let warm = Plan::new(mode, &rules, n);
    for (i, expected) in expected.iter().enumerate() {
        oracle::same_raw(
            execute(&mut warm.start(&query(family, rounds, depth, i)), false),
            expected.clone(),
        );
    }
    drop(warm);
    // Fixed bookkeeping storage is outside the measured ownership baseline.
    let mut phases: Vec<(&str, meter::Reading)> = Vec::with_capacity(2 + reuse * 6 + 6);
    let (p, prep) = measure(|| Plan::new(mode, &rules, n));
    phases.push(("prepare", prep));
    for (i, expected) in expected.iter().enumerate() {
        let (q, m) = measure(|| query(family, rounds, depth, i));
        phases.push(("input", m));
        let (mut e, m) = measure(|| p.start(&q));
        phases.push(("setup", m));
        let (answers, m) = measure(|| execute(&mut e, false));
        phases.push(("execute", m));
        same_borrowed(&answers, expected);
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
    let (q, m) = measure(|| query(family, rounds, depth, 0));
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
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"rounds\":{rounds},\"depth\":{depth},\"reuse\":{reuse},\"cow\":{},\"phases\":[{readings}]}}",
        cfg!(feature = "arena-cow")
    );
}
