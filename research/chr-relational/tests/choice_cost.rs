//! Same qualified search adapters with independently varied source work and lifecycle costs.
#[allow(dead_code)]
#[path = "support/local_ports.rs"]
mod local;
#[cfg(feature = "alloc-meter")]
#[allow(dead_code, unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use chr_syntax::{Answer, Query, Rule, Var, and, atom, c, eq, or, t, v};
use std::sync::Arc;
trait Backend {
    type Search;
    fn prepare(r: &[Rule]) -> Self;
    fn start(&self, q: &Query) -> Self::Search;
    fn tick(s: &mut Self::Search) -> Option<Option<Answer>>;
}
struct Local(local::multihead::search::Prepared);
impl Backend for Local {
    type Search = local::multihead::search::Search<false>;
    fn prepare(r: &[Rule]) -> Self {
        Self(local::multihead::search::Prepared::compile(r).unwrap())
    }
    fn start(&self, q: &Query) -> Self::Search {
        self.0.start(q)
    }
    fn tick(s: &mut Self::Search) -> Option<Option<Answer>> {
        use local::multihead::search::Event;
        match s.tick() {
            Event::Answer(a) => Some(Some(a)),
            Event::Progress => Some(None),
            Event::Exhausted => None,
        }
    }
}
struct Compiled<const INDEX: bool>(chr_compiled::PreparedRuleset);
impl<const INDEX: bool> Backend for Compiled<INDEX> {
    type Search = chr_compiled::SearchEngine;
    fn prepare(r: &[Rule]) -> Self {
        Self(chr_compiled::PreparedRuleset::new(r.to_vec(), None).unwrap())
    }
    fn start(&self, q: &Query) -> Self::Search {
        self.0
            .start_search(
                q.clone(),
                chr_compiled::Policy::Global,
                if INDEX {
                    chr_compiled::Access::Indexed
                } else {
                    chr_compiled::Access::Scan
                },
            )
            .unwrap()
    }
    fn tick(s: &mut Self::Search) -> Option<Option<Answer>> {
        match s.tick() {
            chr_compiled::SearchEvent::Complete(mut b) => Some(b.engine.observe()),
            chr_compiled::SearchEvent::Exhausted => None,
            _ => Some(None),
        }
    }
}
struct Context(Arc<chr_relational::contextual_execute::Prepared>);
impl Backend for Context {
    type Search = chr_relational::contextual_execute::Engine;
    fn prepare(r: &[Rule]) -> Self {
        Self(chr_relational::contextual_execute::Prepared::new(r).unwrap())
    }
    fn start(&self, q: &Query) -> Self::Search {
        self.0.start(q)
    }
    fn tick(s: &mut Self::Search) -> Option<Option<Answer>> {
        use chr_relational::contextual_execute::Step;
        match s.advance() {
            Step::Answer(a) => Some(Some(a)),
            Step::Exhausted => None,
            _ => Some(None),
        }
    }
}
fn rules(family: &str, depth: usize, work: usize) -> Vec<Rule> {
    let mut consume = Rule::simplify(
        "consume",
        [c("request", [v(0), v(1)]), c("ticket", [])],
        and([
            eq(v(1), t("result", [v(2), v(2)])),
            c("kept", [v(2)]).into(),
        ]),
    );
    consume
        .guards
        .push(chr_syntax::Guard::Equal(v(0), atom("ready")));
    let count = (0..work).fold(atom("zero"), |n, _| t("next", [n]));
    let mut body = and([
        c("request", [v(0), v(1)]).into(),
        c("ticket", []).into(),
        c("work", [count, v(0)]).into(),
    ]);
    for _ in 0..depth {
        body = or(body.clone(), body);
    }
    let bind = eq(v(0), atom("ready"));
    let fail = eq(v(8), t("cycle", [v(8)]));
    body = match family {
        "early" => and([bind, body]),
        "late" => body,
        "early-fail" => or(and([fail, body.clone()]), and([bind, body])),
        "late-fail" => or(and([body.clone(), bind.clone(), fail]), and([body, bind])),
        _ => panic!("unknown family"),
    };
    vec![
        consume,
        Rule::simplify("go", [c("go", [v(0), v(1)])], body),
        Rule::simplify(
            "step",
            [c("work", [t("next", [v(0)]), v(1)])],
            c("work", [v(0), v(1)]).into(),
        ),
        Rule::simplify(
            "ready",
            [c("work", [atom("zero"), v(0)])],
            eq(v(0), atom("ready")),
        ),
    ]
}
fn query(payload: usize, i: usize) -> Query {
    let base = 100 + i as u64 * 10;
    let caller = if i.is_multiple_of(2) {
        base + 2
    } else {
        base + 1
    };
    let mut constraints = vec![c("go", [v(base), v(base + 1)])];
    for k in 0..payload {
        constraints.push(c(
            "inert",
            [t("pair", [v(caller), atom(&format!("value-{i}-{k}"))])],
        ));
    }
    Query {
        constraints,
        outputs: vec![
            ("out".into(), Var(base + 1)),
            ("caller".into(), Var(caller)),
        ],
    }
}
fn finish<B: Backend>(s: &mut B::Search, out: &mut Vec<Answer>) {
    for _ in 0..200_000 {
        match B::tick(s) {
            None => return,
            Some(Some(a)) => out.push(a),
            _ => (),
        }
    }
    panic!("finite service cutoff")
}
#[cfg(feature = "alloc-meter")]
fn measure<T>(f: impl FnOnce() -> T) -> (T, meter::Reading) {
    let start = meter::begin();
    let value = f();
    (value, meter::end(start))
}
fn same(actual: &[Answer], expected: &[Answer]) {
    scalar::same_raw(actual.to_vec(), expected.to_vec());
}
#[cfg(feature = "alloc-meter")]
fn run<B: Backend>(
    family: &str,
    depth: usize,
    payload: usize,
    reuse: usize,
    keep: usize,
    work: usize,
    _batches: usize,
) {
    let rules = rules(family, depth, work);
    let queries = (0..reuse).map(|i| query(payload, i)).collect::<Vec<_>>();
    let expected = queries
        .iter()
        .map(|q| scalar::run(&rules, q, 200_000))
        .collect::<Vec<_>>();
    {
        let p = B::prepare(&rules);
        for (q, e) in queries.iter().zip(&expected) {
            let mut s = p.start(q);
            let mut a = vec![];
            finish::<B>(&mut s, &mut a);
            same(&a, e);
        }
    }
    // Container capacities and source/oracle fixtures are outside the ownership interval.
    let mut answers = Vec::with_capacity(32);
    let mut retained: Vec<Answer> = Vec::with_capacity(reuse * 32);
    let expected_all = expected.iter().flatten().cloned().collect::<Vec<_>>();
    let mut observed = 0;
    let mut rows = Vec::with_capacity(reuse * 5 + 4);
    #[cfg(feature = "alloc-meter")]
    let baseline = meter::end(meter::begin()).live_end;
    let (p, prep) = measure(|| B::prepare(&rules));
    let prepared_live = prep.live_end;
    rows.push(("prepare", prep));
    for (q, e) in queries.iter().zip(&expected) {
        let (mut s, row) = measure(|| p.start(q));
        rows.push(("setup", row));
        let (_, row) = measure(|| finish::<B>(&mut s, &mut answers));
        rows.push(("service-and-observation", row));
        let (_, row) = measure(|| drop(s));
        rows.push(("search-dispose", row));
        same(&answers, e);
        observed += answers.len();
        retained.append(&mut answers);
        let remove = retained.len().saturating_sub(keep);
        let (_, row) = measure(|| {
            retained.drain(..remove);
        });
        rows.push(("consumer-release", row));
        same(
            &retained,
            &expected_all[observed.saturating_sub(keep)..observed],
        );
        if keep == 0 {
            assert_eq!(
                row.live_end, prepared_live,
                "query owners survived immediate release"
            );
        }
    }
    same(
        &retained,
        &expected_all[observed.saturating_sub(keep)..observed],
    );
    let (_, row) = measure(|| drop(p));
    rows.push(("prepare-dispose", row));
    // Consumer answers must remain valid independently of preparation and search.
    same(
        &retained,
        &expected_all[expected_all.len().saturating_sub(keep)..],
    );
    let (_, row) = measure(|| retained.clear());
    rows.push(("consumer-final-release", row));
    assert_eq!(row.live_end, baseline, "ownership not restored");
    if keep == 0 {
        assert_eq!(
            row.live_start, baseline,
            "immediate-release owners survived preparation"
        );
    }
    assert!(prepared_live >= baseline);
    for (phase, row) in rows {
        println!("{{\"phase\":\"{phase}\",\"allocation\":{}}}", row.json());
    }
}
#[cfg(not(feature = "alloc-meter"))]
fn run<B: Backend>(
    family: &str,
    depth: usize,
    payload: usize,
    reuse: usize,
    keep: usize,
    work: usize,
    batches: usize,
) {
    use std::time::Instant;
    let rules = rules(family, depth, work);
    let queries = (0..reuse).map(|i| query(payload, i)).collect::<Vec<_>>();
    let expected = queries
        .iter()
        .map(|q| scalar::run(&rules, q, 200_000))
        .collect::<Vec<_>>();
    {
        let p = B::prepare(&rules);
        for (q, e) in queries.iter().zip(&expected) {
            let mut s = p.start(q);
            let mut a = vec![];
            finish::<B>(&mut s, &mut a);
            same(&a, e);
        }
    }
    let expected_all = expected.iter().flatten().cloned().collect::<Vec<_>>();
    let mut answers = Vec::with_capacity(32);
    let mut retained = Vec::with_capacity(reuse * 32);
    let mut totals = [0u128; 7];
    let mut first_service = 0u128;
    let mut first_query = 0u128;
    for _ in 0..batches {
        let clock = Instant::now();
        let p = B::prepare(&rules);
        let prep = clock.elapsed().as_nanos();
        totals[0] += prep;
        for (i, (q, e)) in queries.iter().zip(&expected).enumerate() {
            let clock = Instant::now();
            let mut s = p.start(q);
            let setup = clock.elapsed().as_nanos();
            totals[1] += setup;
            let clock = Instant::now();
            let mut first = None;
            let mut exhausted = false;
            for _ in 0..200_000 {
                match B::tick(&mut s) {
                    None => {
                        exhausted = true;
                        break;
                    }
                    Some(Some(a)) => {
                        if first.is_none() {
                            first = Some(clock.elapsed().as_nanos());
                        }
                        answers.push(a)
                    }
                    _ => (),
                }
            }
            totals[2] += clock.elapsed().as_nanos();
            assert!(exhausted);
            if i == 0 {
                let first = first.expect("registered sources have an answer");
                first_service += first;
                first_query += prep + setup + first;
            }
            let clock = Instant::now();
            drop(s);
            totals[3] += clock.elapsed().as_nanos();
            same(&answers, e);
            let clock = Instant::now();
            retained.append(&mut answers);
            let remove = retained.len().saturating_sub(keep);
            retained.drain(..remove);
            totals[4] += clock.elapsed().as_nanos();
        }
        let clock = Instant::now();
        drop(p);
        totals[5] += clock.elapsed().as_nanos();
        same(
            &retained,
            &expected_all[expected_all.len().saturating_sub(keep)..],
        );
        let clock = Instant::now();
        retained.clear();
        totals[6] += clock.elapsed().as_nanos();
    }
    println!(
        "{{\"batches\":{batches},\"total_ns\":{},\"phases_ns\":{totals:?},\"first_service_ns\":{first_service},\"first_query_ns\":{first_query}}}",
        totals.iter().sum::<u128>()
    );
}
#[cfg(not(feature = "alloc-meter"))]
fn cancel_cost<B: Backend>() {
    use std::time::Instant;
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [])],
            or(c("again", []).into(), c("answer", [v(0), v(0)]).into()),
        ),
        Rule::simplify("again", [c("again", [])], c("again", []).into()),
    ];
    let q = Query {
        constraints: vec![c("start", [])],
        outputs: vec![],
    };
    let expected = Answer {
        outputs: vec![],
        residual: vec![c("answer", [v(90), v(90)])],
    };
    let clock = Instant::now();
    let p = B::prepare(&rules);
    let prep = clock.elapsed().as_nanos();
    let clock = Instant::now();
    let mut s = p.start(&q);
    let setup = clock.elapsed().as_nanos();
    let clock = Instant::now();
    let mut answer = None;
    for _ in 0..2000 {
        match B::tick(&mut s) {
            Some(Some(a)) => {
                answer = Some(a);
                break;
            }
            None => panic!("ongoing branch exhausted"),
            _ => (),
        }
    }
    let first = clock.elapsed().as_nanos();
    same(&[answer.expect("finite answer missing")], &[expected]);
    for _ in 0..32 {
        assert!(B::tick(&mut s).is_some());
    }
    let clock = Instant::now();
    drop(s);
    let cancel = clock.elapsed().as_nanos();
    let clock = Instant::now();
    drop(p);
    let dispose = clock.elapsed().as_nanos();
    println!(
        "{{\"cancel_ns\":{cancel},\"prepare_ns\":{prep},\"setup_ns\":{setup},\"first_ns\":{first},\"dispose_ns\":{dispose}}}"
    );
}
fn main() {
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    let a = std::env::args().collect::<Vec<_>>();
    if a[2] == "cancel" {
        #[cfg(feature = "alloc-meter")]
        panic!("timing request in diagnostic build");
        #[cfg(not(feature = "alloc-meter"))]
        {
            match a[1].as_str() {
                "local" => cancel_cost::<Local>(),
                "scan" => cancel_cost::<Compiled<false>>(),
                "indexed" => cancel_cost::<Compiled<true>>(),
                "context" => cancel_cost::<Context>(),
                _ => panic!(),
            };
            return;
        }
    }
    let depth = a[3].parse().unwrap();
    let payload = a[4].parse().unwrap();
    let reuse = a[5].parse().unwrap();
    let keep = a[6].parse().unwrap();
    let work = a[7].parse().unwrap();
    let batches = a[8].parse().unwrap();
    #[cfg(feature = "alloc-meter")]
    assert_eq!(
        batches, 1,
        "allocation diagnostics cannot run timing batches"
    );
    match a[1].as_str() {
        "local" => run::<Local>(&a[2], depth, payload, reuse, keep, work, batches),
        "scan" => run::<Compiled<false>>(&a[2], depth, payload, reuse, keep, work, batches),
        "indexed" => run::<Compiled<true>>(&a[2], depth, payload, reuse, keep, work, batches),
        "context" => run::<Context>(&a[2], depth, payload, reuse, keep, work, batches),
        _ => panic!(),
    }
}
