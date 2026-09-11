#[path = "../tests/runtime_support/mod.rs"]
#[allow(dead_code)]
mod oracle;
#[path = "support/order_source.rs"]
mod source;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEngine, SearchEvent, Stats};
use chr_syntax::{Answer, Query, Rule};
#[derive(Default, Debug, PartialEq, Eq)]
struct Work {
    splits: usize,
    failed: usize,
    source: u64,
    applications: u64,
    dispatch: u64,
    specialized: u64,
}
fn add(w: &mut Work, s: &Stats) {
    w.source += s.source_steps;
    w.applications += s.applications;
    w.dispatch += s.rule_dispatches;
    w.specialized += s.specialized_applications;
}
fn collect(mut e: SearchEngine) -> (Vec<Answer>, Work) {
    let mut a = vec![];
    let mut w = Work::default();
    for _ in 0..2_000_000 {
        match e.tick() {
            SearchEvent::Split { work, .. } => {
                w.splits += 1;
                if let Some(s) = work {
                    add(&mut w, &s);
                }
            }
            SearchEvent::Failed(b) => {
                w.failed += 1;
                add(&mut w, b.engine.stats());
            }
            SearchEvent::Complete(mut b) => {
                add(&mut w, b.engine.stats());
                a.push(b.engine.observe().unwrap());
            }
            SearchEvent::Exhausted => return (a, w),
            SearchEvent::Progress => (),
        }
    }
    panic!("source gate exhausted service bound")
}
fn run(p: &PreparedRuleset, q: Query) -> (Vec<Answer>, Work) {
    collect(p.start_search(q, Policy::Global, Access::Scan).unwrap())
}
fn check(rules: &[Rule], q: &Query, expected: &[Answer], label: &str) -> Vec<(String, Work)> {
    oracle::same_raw(oracle::run(rules, q, 2_000_000), expected.to_vec());
    let p = PreparedRuleset::new(rules.to_vec(), None).unwrap();
    let mut rows = vec![];
    for (name, p) in [
        ("scan", p.clone()),
        ("specialized", p.specialize_inferred()),
    ] {
        let (a, w) = run(&p, q.clone());
        oracle::same_raw(a, expected.to_vec());
        rows.push((name.to_string(), w));
    }
    if let Ok(program) = chr_compiled::pure_prefix::Program::new(rules) {
        let (r, input) = program.lower(q).unwrap();
        let (a, w) = run(&PreparedRuleset::new(r, None).unwrap(), input);
        oracle::same_raw(a, expected.to_vec());
        rows.push(("prefix".into(), w));
        let shape = q
            .constraints
            .iter()
            .map(|c| (c.name.clone(), c.args.len()))
            .collect::<Vec<_>>();
        let artifact = program.prepare_shape(&shape).unwrap();
        let (a, w) = collect(artifact.start(q, Access::Scan).unwrap());
        oracle::same_raw(a, expected.to_vec());
        rows.push(("prepared-prefix".into(), w));
        let mut bad = q.clone();
        bad.constraints.clear();
        assert!(artifact.start(&bad, Access::Scan).is_err());
        println!(
            "eligible {label} prefix {:?}",
            program.eliminated_predicates()
        );
    }
    for (name, w) in &rows {
        println!("work {label} {name} {w:?}");
    }
    rows
}
fn rename(q: &mut Query, from: u64, to: u64) {
    fn term(t: &mut chr_syntax::Term, from: u64, to: u64) {
        match t {
            chr_syntax::Term::Var(v) => {
                if v.0 == from {
                    v.0 = to;
                }
            }
            chr_syntax::Term::App(_, args) => {
                for x in args {
                    term(x, from, to);
                }
            }
        }
    }
    for c in &mut q.constraints {
        for t in &mut c.args {
            term(t, from, to);
        }
    }
    for (_, v) in &mut q.outputs {
        if v.0 == from {
            v.0 = to;
        }
    }
}
fn probes() {
    for family in ["oldest-first", "newest-first"] {
        let schema = source::Schema {
            family,
            resource: true,
            fail_tail: false,
            work: 4,
            payload: 8,
        };
        let rules = schema.rules();
        let base = schema.query(4, false);
        let mut missing = base.clone();
        missing.constraints.retain(|c| c.name != "token");
        let mut duplicate = base.clone();
        duplicate.constraints.push(chr_syntax::c("token", []));
        let mut aliased = base.clone();
        rename(&mut aliased, 101, 100);
        for (label, q) in [
            ("missing-token", missing),
            ("duplicate-token", duplicate),
            ("aliased-coins", aliased),
        ] {
            let expected = oracle::run(&rules, &q, 2_000_000);
            assert_eq!(expected.len(), 1);
            if label == "missing-token" {
                assert!(expected[0].residual.iter().any(|c| c.name == "finish"));
            }
            if label == "duplicate-token" {
                assert_eq!(
                    expected[0]
                        .residual
                        .iter()
                        .filter(|c| c.name == "token")
                        .count(),
                    1
                );
            }
            check(&rules, &q, &expected, &format!("probe/{family}/{label}"));
        }
        let program = chr_compiled::pure_prefix::Program::new(&rules).unwrap();
        let shape = base
            .constraints
            .iter()
            .map(|c| (c.name.clone(), c.args.len()))
            .collect::<Vec<_>>();
        let artifact = program.prepare_shape(&shape).unwrap();
        let changed = source::Schema {
            work: 0,
            payload: 0,
            ..schema
        };
        assert_eq!(rules, changed.rules());
        let mut q = changed.query(4, false);
        for from in [100, 101, 102, 103, 10000, 10001] {
            rename(&mut q, from, from + 100000);
        }
        for input in [base, q] {
            let expected = oracle::run(&rules, &input, 2_000_000);
            let (a, w) = collect(artifact.start(&input, Access::Scan).unwrap());
            oracle::same_raw(a, expected);
            println!("reused {family} {w:?}");
        }
    }
}
fn main() {
    probes();
    for family in ["oldest-first", "newest-first", "aliases", "distinct"] {
        let depths: &[usize] = if family.ends_with("-first") {
            &[0, 1, 4, 8]
        } else {
            &[0, 4, 8]
        };
        for resource in [false, true] {
            for fail_tail in [false, true] {
                let schema = source::Schema {
                    family,
                    resource,
                    fail_tail,
                    work: 4,
                    payload: 8,
                };
                let rules = schema.rules();
                let recursive = chr_compiled::recursive::Prepared::new(rules.clone()).err();
                let fusion = chr_compiled::resource_fusion::Program::infer(&rules).err();
                println!(
                    "admission {family} resource={resource} fail={fail_tail} recursive={recursive:?} fusion={fusion:?}"
                );
                for &n in depths {
                    for reverse in [false, true] {
                        let q = schema.query(n, reverse);
                        let expected = schema.expected_query(n, reverse);
                        let label = format!("{family}/{n}/{resource}/{fail_tail}/{reverse}");
                        let rows = check(&rules, &q, &expected, &label);
                        if family.ends_with("-first") {
                            assert_eq!(rows.len(), 4);
                            for (_, w) in rows {
                                assert_eq!(w.splits, (1usize << n) - 1);
                                assert_eq!(w.failed, (1usize << n) - usize::from(!fail_tail));
                            }
                        }
                    }
                }
            }
        }
    }
}
