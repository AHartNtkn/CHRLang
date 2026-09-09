//! Complete lifecycle comparison of qualified original and shortened reunion controls.
#[cfg(feature = "alloc-meter")]
use chr_compiled::experiment::meter;
use chr_restoration::{Mode, Prepared, Step, reunion::PreparedPhase};
use chr_syntax::{Answer, Query, Rule};
use std::{sync::Arc, time::Instant};
#[path = "support/reunion_source.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "support/reunion_shortcut.rs"]
mod shortcut;
use fixture::{query, source};
struct Plan {
    backend: Backend,
    check: Option<shortcut::QueryCheck>,
}
enum Backend {
    Copy(Arc<Prepared>),
    Reunion(Arc<PreparedPhase>),
    Compiled(chr_compiled::PreparedRuleset, chr_compiled::Access),
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
    fn new(mode: &str, family: &str, owners: usize, rules: &[Rule], n: usize) -> Self {
        let short = mode.strip_prefix("short-");
        let (verified, check) = if short.is_some() {
            let (r, c) = shortcut::Checked::new(family, owners, rules)
                .unwrap()
                .into_parts();
            (Some(r), Some(c))
        } else {
            (None, None)
        };
        let rules = verified.as_deref().unwrap_or(rules);
        let mode = short.unwrap_or(mode);
        let backend = match mode {
            "copy" => Backend::Copy(Prepared::new(rules).unwrap()),
            "reunion" => Backend::Reunion(PreparedPhase::new(rules, n).unwrap()),
            "factored" => Backend::Factored(rules.to_vec()),
            "scan" | "indexed" | "specialized-scan" | "specialized-indexed" => {
                let p = chr_compiled::PreparedRuleset::new(rules.to_vec(), None).unwrap();
                let p = if mode.starts_with("specialized-") {
                    p.specialize_inferred()
                } else {
                    p
                };
                let access = if mode.ends_with("scan") {
                    chr_compiled::Access::Scan
                } else {
                    chr_compiled::Access::Indexed
                };
                Backend::Compiled(p, access)
            }
            _ => panic!("unknown mode"),
        };
        Self { backend, check }
    }
    fn lower(&self, q: &Query) -> Option<Query> {
        self.check.as_ref().map(|c| c.lower(q).unwrap())
    }
    fn start(&self, q: &Query) -> Running {
        match &self.backend {
            Backend::Copy(p) => Running::Copy(p.start(q, Mode::Copy).unwrap()),
            Backend::Reunion(p) => Running::Reunion(p.start(q).unwrap()),
            Backend::Compiled(p, a) => Running::Indexed(
                p.start_search(q.clone(), chr_compiled::Policy::Global, *a)
                    .unwrap(),
            ),
            Backend::Factored(r) => {
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
fn execute(e: &mut Running, first_only: bool) -> (Vec<Answer>, u128) {
    let clock = Instant::now();
    let mut first = None;
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
        if first.is_none() && !answers.is_empty() {
            first = Some(clock.elapsed().as_nanos());
        }
        if done || (first_only && !answers.is_empty()) {
            return (answers, first.unwrap_or(0));
        }
    }
    panic!("service budget exhausted");
}
#[derive(Clone, Copy)]
struct Reading {
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn measure<T>(f: impl FnOnce() -> T) -> (T, Reading) {
    #[cfg(feature = "alloc-meter")]
    let m = meter::begin();
    let clock = Instant::now();
    let result = f();
    let ns = clock.elapsed().as_nanos();
    (
        result,
        Reading {
            ns,
            #[cfg(feature = "alloc-meter")]
            memory: meter::end(m),
        },
    )
}
fn main() {
    assert!(!std::hint::black_box(
        cfg!(feature = "replay-diagnostic")
            || cfg!(feature = "compiled-work")
            || chr_compiled::COLLECT_METRICS
            || chr_factors::COLLECT_METRICS
    ));
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    assert_eq!(args.len(), 5);
    let mode = &args[0];
    let family = &args[1];
    let owners = args[2].parse::<usize>().unwrap();
    let depth = args[3].parse::<usize>().unwrap();
    let reuse = args[4].parse::<usize>().unwrap();
    assert!(
        [
            "copy",
            "reunion",
            "scan",
            "indexed",
            "specialized-scan",
            "specialized-indexed",
            "short-copy",
            "short-reunion",
            "short-specialized-scan",
            "short-specialized-indexed",
            "factored"
        ]
        .contains(&mode.as_str())
    );
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
    let warm = Plan::new(mode, family, owners, &rules, n);
    for (i, expected) in expected.iter().enumerate() {
        let q = query(family, owners, depth, i);
        let lowered = warm.lower(&q);
        oracle::same_raw(
            execute(&mut warm.start(lowered.as_ref().unwrap_or(&q)), false).0,
            expected.clone(),
        );
    }
    drop(warm);
    // Receipt storage is allocated before the ownership baseline; no growth during phases.
    let mut phases: Vec<(&str, Reading)> = Vec::with_capacity(2 + (reuse + 1) * 8);
    let mut firsts = Vec::with_capacity(reuse + 1);
    let (p, prep) = measure(|| Plan::new(mode, family, owners, &rules, n));
    phases.push(("prepare", prep));
    for (i, expected) in expected.iter().enumerate() {
        let (q, m) = measure(|| query(family, owners, depth, i));
        phases.push(("input", m));
        let (lowered, m) = measure(|| p.lower(&q));
        phases.push(("lower", m));
        let (mut e, m) = measure(|| p.start(lowered.as_ref().unwrap_or(&q)));
        phases.push(("setup", m));
        let ((answers, first), m) = measure(|| execute(&mut e, false));
        phases.push(("execute", m));
        assert!(first > 0 && first <= m.ns);
        firsts.push(first);
        oracle::same_raw(answers.clone(), expected.clone());
        let (_, m) = measure(|| drop(e));
        phases.push(("engine-drop", m));
        let (_, m) = measure(|| drop(answers));
        phases.push(("answers-drop", m));
        let (_, m) = measure(|| drop(lowered));
        phases.push(("lowered-drop", m));
        let (_, m) = measure(|| drop(q));
        phases.push(("input-drop", m));
        #[cfg(feature = "alloc-meter")]
        assert_eq!(
            m.memory.live_end, prep.memory.live_end,
            "query state retained"
        );
    }
    let (q, m) = measure(|| query(family, owners, depth, 0));
    phases.push(("cancel-input", m));
    let (lowered, m) = measure(|| p.lower(&q));
    phases.push(("cancel-lower", m));
    let (mut e, m) = measure(|| p.start(lowered.as_ref().unwrap_or(&q)));
    phases.push(("cancel-setup", m));
    let ((answers, first), m) = measure(|| execute(&mut e, true));
    phases.push(("cancel-first", m));
    assert!(first > 0 && first <= m.ns);
    firsts.push(first);
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
    let (_, m) = measure(|| drop(lowered));
    phases.push(("cancel-lowered-drop", m));
    let (_, m) = measure(|| drop(q));
    phases.push(("cancel-input-drop", m));
    #[cfg(feature = "alloc-meter")]
    assert_eq!(
        m.memory.live_end, prep.memory.live_end,
        "cancellation state retained"
    );
    let (_, m) = measure(|| drop(p));
    phases.push(("prepared-drop", m));
    #[cfg(feature = "alloc-meter")]
    {
        assert!(
            phases
                .windows(2)
                .all(|xs| xs[0].1.memory.live_end == xs[1].1.memory.live_start),
            "unmeasured retained allocation"
        );
        assert_eq!(
            phases.last().unwrap().1.memory.live_end,
            prep.memory.live_start,
            "prepared lifecycle not released"
        );
    }
    let readings = phases
        .iter()
        .map(|(name, m)| {
            #[cfg(feature = "alloc-meter")]
            let memory = m.memory.json();
            #[cfg(not(feature = "alloc-meter"))]
            let memory = "null";
            format!(
                "{{\"phase\":\"{name}\",\"ns\":{},\"memory\":{memory}}}",
                m.ns
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"owners\":{owners},\"depth\":{depth},\"reuse\":{reuse},\"cow\":{},\"meter\":{},\"first_ns\":{firsts:?},\"phases\":[{readings}]}}",
        cfg!(feature = "arena-cow"),
        cfg!(feature = "alloc-meter")
    );
}
