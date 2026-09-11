use chr_compiled::{
    Access, CompletedBranch, Policy, PreparedQuery, PreparedRuleset, SearchEngine, SearchEvent,
    search_bundled, search_fixtures,
};
use chr_finite::{
    cnf::{Cnf, Encoding},
    model::{Endpoint, Forbidden, Problem},
    native, z3,
};
use chr_syntax::{Answer, Constraint, Query, Term, Var, atom, c, v};
use std::time::Instant;
const LIMIT: usize = 1_000_000;
fn elapsed(t: Instant) -> u128 {
    t.elapsed().as_nanos()
}
#[derive(Clone)]
struct Case {
    n: usize,
    forbidden: Vec<Forbidden>,
}
fn case(family: &str, n: usize) -> Case {
    assert!((4..=8).contains(&n));
    let mut forbidden = vec![];
    let mut edge = |x, y, a, b| {
        forbidden.push(Forbidden {
            left: Endpoint::Var(x),
            right: Endpoint::Var(y),
            a,
            b,
        })
    };
    match family {
        "weak" => (),
        "chain" => {
            for x in 0..n - 1 {
                for a in 0..3 {
                    for b in 0..3 {
                        if a != b {
                            edge(x, x + 1, a, b)
                        }
                    }
                }
            }
        }
        "unsupported" => {
            for b in 0..3 {
                edge(0, 1, 0, b)
            }
        }
        "contradiction" => {
            for x in 0..4 {
                for y in x + 1..4 {
                    for a in 0..3 {
                        edge(x, y, a, a)
                    }
                }
            }
        }
        "asymmetric" => {
            for x in 0..n - 1 {
                for a in 0..3 {
                    edge(x, x + 1, a, (a + 1) % 3)
                }
                edge(x, x + 1, 0, 0)
            }
        }
        _ => panic!("unknown family"),
    }
    Case { n, forbidden }
}
fn ground(a: u8) -> Term {
    atom(&format!("a{a}"))
}
fn source(e: Endpoint) -> Term {
    match e {
        Endpoint::Var(x) => v(x as u64),
        Endpoint::Value(a) => ground(a),
    }
}
fn initial(case: &Case) -> Query {
    let mut constraints: Vec<_> = case
        .forbidden
        .iter()
        .map(|f| {
            c(
                "forbid",
                [source(f.left), source(f.right), ground(f.a), ground(f.b)],
            )
        })
        .collect();
    constraints.extend((0..case.n).map(|x| c("choose", [v(x as u64)])));
    Query {
        constraints,
        outputs: (0..case.n)
            .map(|x| (format!("o{x}"), Var(x as u64)))
            .collect(),
    }
}
fn givens(case: &Case, index: usize) -> Vec<(Endpoint, u8)> {
    match index % 4 {
        0 | 3 => vec![],
        1 => vec![(Endpoint::Var(0), 0)],
        2 => vec![(Endpoint::Var(case.n - 1), 1)],
        _ => unreachable!(),
    }
}
fn extras(givens: &[(Endpoint, u8)]) -> Vec<Constraint> {
    givens
        .iter()
        .map(|(e, a)| c("given", [source(*e), ground(*a)]))
        .collect()
}
fn value(e: Endpoint, row: &[u8]) -> u8 {
    match e {
        Endpoint::Var(x) => row[x],
        Endpoint::Value(a) => a,
    }
}
fn observation(case: &Case, row: &[u8]) -> Answer {
    Answer {
        outputs: row
            .iter()
            .enumerate()
            .map(|(x, a)| (format!("o{x}"), ground(*a)))
            .collect(),
        residual: case
            .forbidden
            .iter()
            .map(|f| {
                c(
                    "forbid",
                    [
                        ground(value(f.left, row)),
                        ground(value(f.right, row)),
                        ground(f.a),
                        ground(f.b),
                    ],
                )
            })
            .collect(),
    }
}
fn expected(case: &Case, givens: &[(Endpoint, u8)]) -> Vec<Vec<u8>> {
    let mut rows = vec![];
    for mut encoded in 0..3usize.pow(case.n as u32) {
        let row: Vec<_> = (0..case.n)
            .map(|_| {
                let a = (encoded % 3) as u8;
                encoded /= 3;
                a
            })
            .collect();
        if givens.iter().any(|(e, a)| value(*e, &row) != *a)
            || case
                .forbidden
                .iter()
                .any(|f| value(f.left, &row) == f.a && value(f.right, &row) == f.b)
        {
            continue;
        }
        rows.push(row)
    }
    rows.sort();
    rows
}
fn validate(case: &Case, answers: &[Answer], expected: &[Vec<u8>], complete: bool) {
    let mut rows = vec![];
    for answer in answers {
        assert_eq!(answer.outputs.len(), case.n);
        let row: Vec<_> = answer
            .outputs
            .iter()
            .enumerate()
            .map(|(x, (name, t))| {
                assert_eq!(name, &format!("o{x}"));
                match t {
                    Term::App(n, args) if args.is_empty() => match n.as_str() {
                        "a0" => 0,
                        "a1" => 1,
                        "a2" => 2,
                        _ => panic!("invalid value"),
                    },
                    _ => panic!("nonground value"),
                }
            })
            .collect();
        let mut actual = answer.residual.clone();
        actual.sort();
        let mut wanted = observation(case, &row).residual;
        wanted.sort();
        assert_eq!(actual, wanted);
        assert!(expected.binary_search(&row).is_ok());
        rows.push(row)
    }
    rows.sort();
    assert!(rows.windows(2).all(|p| p[0] != p[1]));
    if complete {
        assert_eq!(rows, expected)
    }
}
#[allow(clippy::large_enum_variant)]
enum Backend<'a> {
    Native(native::Prepared<'a>),
    Boolean(z3::Prepared),
    Compiled(PreparedQuery),
    Cold(Option<chr_compiled::Engine>),
}
enum Running<'a> {
    Native(native::NativeSearch<'a>),
    Boolean(z3::Query<'a>),
    Compiled(SearchEngine),
}
#[allow(clippy::large_enum_variant)]
enum Event {
    Row(Vec<u8>),
    Branch(CompletedBranch),
    End,
    Cutoff,
}
impl Backend<'_> {
    fn start(&mut self, givens: &[(Endpoint, u8)], case: &Case) -> Running<'_> {
        match self {
            Self::Native(p) => Running::Native(p.start(givens).unwrap()),
            Self::Boolean(p) => {
                let mut masks = vec![7; case.n];
                let mut inconsistent = false;
                for (e, a) in givens {
                    match e {
                        Endpoint::Var(x) => masks[*x] &= 1 << a,
                        Endpoint::Value(v) => inconsistent |= v != a,
                    }
                }
                Running::Boolean(
                    p.start(&chr_finite::model::QueryDomains {
                        masks,
                        inconsistent,
                    })
                    .unwrap(),
                )
            }
            Self::Compiled(p) => Running::Compiled(p.start(extras(givens)).unwrap()),
            Self::Cold(p) => {
                assert!(givens.is_empty());
                Running::Compiled(p.take().expect("single cold query").into_search())
            }
        }
    }
}
impl Running<'_> {
    fn next(&mut self, steps: &mut usize) -> Event {
        match self {
            Self::Native(s) => s.next().map_or(Event::End, Event::Row),
            Self::Boolean(s) => s.next_solution().unwrap().map_or(Event::End, Event::Row),
            Self::Compiled(s) => loop {
                if *steps == LIMIT {
                    return Event::Cutoff;
                }
                *steps += 1;
                match s.tick() {
                    SearchEvent::Complete(b) => return Event::Branch(b),
                    SearchEvent::Exhausted => return Event::End,
                    SearchEvent::Progress | SearchEvent::Split { .. } | SearchEvent::Failed(_) => {}
                }
            },
        }
    }
}
fn rss() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .unwrap()
        .lines()
        .find_map(|s| {
            s.strip_prefix("VmRSS:")
                .map(|s| s.split_whitespace().next().unwrap().parse().unwrap())
        })
        .unwrap()
}
fn run(family: &str, n: usize, variant: &str, queries: usize, memory: bool) -> String {
    const { assert!(!chr_compiled::COLLECT_METRICS && !chr_persistent::COLLECT_KERNEL_METRICS) };
    let harness = Instant::now();
    let base = case(family, n);
    let query_givens: Vec<_> = (0..queries).map(|i| givens(&base, i)).collect();
    let expected: Vec<_> = query_givens.iter().map(|g| expected(&base, g)).collect();
    let harness_ns = elapsed(harness);
    let before_rss = if memory { rss() } else { 0 };
    let preparation = Instant::now();
    let problem = if variant == "native" {
        Some(Problem::new(n, base.forbidden.clone()).unwrap())
    } else {
        None
    };
    let mut backend = match variant {
        "native" => Backend::Native(native::Prepared::new(problem.as_ref().unwrap())),
        "support" | "conflict" => {
            let problem = Problem::new(n, base.forbidden.clone()).unwrap();
            let encoding = if variant == "support" {
                Encoding::Support
            } else {
                Encoding::Conflict
            };
            Backend::Boolean(z3::Prepared::new(&Cnf::encode(&problem, encoding)).unwrap())
        }
        "generated-global-scan" | "generic-global-scan" | "generated-active-indexed" => {
            let rules = search_fixtures::finite_rules(3);
            let code = if variant.starts_with("generic") {
                None
            } else {
                Some(search_bundled(search_fixtures::FINITE_START + 3))
            };
            let prepared = PreparedRuleset::new(rules, code).unwrap();
            let (policy, access) = if variant.contains("active") {
                (Policy::Active, Access::Indexed)
            } else {
                (Policy::Global, Access::Scan)
            };
            if queries == 1 {
                Backend::Cold(Some(
                    prepared.start(initial(&base), policy, access).unwrap(),
                ))
            } else {
                Backend::Compiled(
                    prepared
                        .prepare_query(initial(&base), policy, access)
                        .unwrap(),
                )
            }
        }
        _ => panic!("invalid variant"),
    };
    let prepare_ns = elapsed(preparation);
    let prepared_rss = if memory { rss() } else { 0 };
    let mut samples = vec![];
    for i in 0..queries {
        let request = Instant::now();
        let setup = Instant::now();
        let mut running = backend.start(&query_givens[i], &base);
        let setup_ns = elapsed(setup);
        let (mut execution_ns, mut observe_ns, mut result_drop_ns) = (0, 0, 0);
        let mut first_answer_ns = None;
        let mut steps = 0;
        let mut answers = vec![];
        let complete = loop {
            let t = Instant::now();
            let mut event = running.next(&mut steps);
            execution_ns += elapsed(t);
            match event {
                Event::End => break true,
                Event::Cutoff => break false,
                _ => (),
            };
            let t = Instant::now();
            let answer = match &mut event {
                Event::Row(row) => observation(&base, row),
                Event::Branch(branch) => branch.engine.observe().unwrap(),
                _ => unreachable!(),
            };
            answers.push(answer);
            observe_ns += elapsed(t);
            if first_answer_ns.is_none() {
                first_answer_ns = Some(elapsed(request))
            }
            let t = Instant::now();
            drop(event);
            result_drop_ns += elapsed(t);
        };
        let t = Instant::now();
        drop(running);
        let query_drop_ns = elapsed(t);
        let request_ns = elapsed(request);
        let before_validation_rss = if memory { rss() } else { 0 };
        let t = Instant::now();
        validate(&base, &answers, &expected[i], complete);
        let validation_ns = elapsed(t);
        let raw = answers.len();
        let t = Instant::now();
        drop(answers);
        let answer_drop_ns = elapsed(t);
        let after_answer_rss = if memory { rss() } else { 0 };
        samples.push(format!("{{\"complete\":{complete},\"raw\":{raw},\"setup_ns\":{setup_ns},\"execution_ns\":{execution_ns},\"observe_ns\":{observe_ns},\"result_drop_ns\":{result_drop_ns},\"query_drop_ns\":{query_drop_ns},\"request_ns\":{request_ns},\"answer_drop_ns\":{answer_drop_ns},\"validation_ns\":{validation_ns},\"first_answer_ns\":{},\"retained_answers_rss_kib\":{before_validation_rss},\"after_answer_rss_kib\":{after_answer_rss}}}",first_answer_ns.map_or("null".into(),|x|x.to_string())));
        if !complete {
            break;
        }
    }
    let t = Instant::now();
    drop(backend);
    drop(problem);
    let prepared_drop_ns = elapsed(t);
    let final_rss = if memory { rss() } else { 0 };
    format!(
        "{{\"family\":{family:?},\"size\":{n},\"variant\":{variant:?},\"queries\":{queries},\"engine_metrics\":{},\"kernel_metrics\":{},\"memory_run\":{memory},\"prepare_ns\":{prepare_ns},\"prepared_drop_ns\":{prepared_drop_ns},\"harness_ns\":{harness_ns},\"before_rss_kib\":{before_rss},\"prepared_rss_kib\":{prepared_rss},\"final_rss_kib\":{final_rss},\"samples\":[{}]}}",
        chr_compiled::COLLECT_METRICS,
        chr_persistent::COLLECT_KERNEL_METRICS,
        samples.join(",")
    )
}
fn main() {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a == ["clock-check"] {
        let mut samples: Vec<_> = (0..10000)
            .map(|_| {
                let t = Instant::now();
                elapsed(t)
            })
            .collect();
        samples.sort();
        println!(
            "{{\"median_ns\":{},\"min_ns\":{}}}",
            samples[5000], samples[0]
        );
        return;
    }
    assert_eq!(a.len(), 5);
    assert!(a[0] == "time" || a[0] == "memory");
    let queries: usize = a[4].parse().unwrap();
    assert!(queries == 1 || queries == 4);
    println!(
        "{}",
        run(
            &a[1],
            a[2].parse().unwrap(),
            &a[3],
            queries,
            a[0] == "memory"
        )
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_paths_validate_cold_and_reused_observations() {
        for family in [
            "weak",
            "chain",
            "unsupported",
            "contradiction",
            "asymmetric",
        ] {
            for variant in [
                "native",
                "support",
                "conflict",
                "generated-global-scan",
                "generic-global-scan",
                "generated-active-indexed",
            ] {
                for queries in [1, 4] {
                    let report = run(family, 4, variant, queries, false);
                    assert!(!report.contains("\"complete\":false"));
                }
            }
        }
    }
}
