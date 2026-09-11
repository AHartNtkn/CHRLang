//! Isolated ownership qualification; execution and answer construction are inseparable.
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
fn rules(family: &str, depth: usize) -> Vec<Rule> {
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
    let mut body = and([c("request", [v(0), v(1)]).into(), c("ticket", []).into()]);
    for _ in 0..depth {
        body = or(body.clone(), body);
    }
    let bind = eq(v(0), atom("ready"));
    let fail = eq(v(8), t("cycle", [v(8)]));
    body = match family {
        "early" => and([bind, body]),
        "late" => and([body, bind]),
        "early-fail" => or(and([fail, body.clone()]), and([bind, body])),
        "late-fail" => or(and([body.clone(), bind.clone(), fail]), and([body, bind])),
        _ => panic!("unknown family"),
    };
    vec![consume, Rule::simplify("go", [c("go", [v(0), v(1)])], body)]
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
fn run<B: Backend>(family: &str, depth: usize, payload: usize, reuse: usize, keep: usize) {
    let rules = rules(family, depth);
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
fn run<B: Backend>(family: &str, depth: usize, payload: usize, reuse: usize, keep: usize) {
    let rules = rules(family, depth);
    let queries = (0..reuse).map(|i| query(payload, i)).collect::<Vec<_>>();
    let expected = queries
        .iter()
        .flat_map(|q| scalar::run(&rules, q, 200_000))
        .collect::<Vec<_>>();
    let p = B::prepare(&rules);
    let mut retained = vec![];
    for q in &queries {
        let mut s = p.start(q);
        finish::<B>(&mut s, &mut retained);
        drop(s);
        let remove = retained.len().saturating_sub(keep);
        retained.drain(..remove);
    }
    drop(p);
    same(&retained, &expected[expected.len().saturating_sub(keep)..]);
    println!("{{\"ordinary_semantics\":true}}");
}
fn cancel<B: Backend>() {
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
    let empty = Query {
        constraints: vec![],
        outputs: vec![],
    };
    let mut answers = Vec::with_capacity(4);
    #[cfg(feature = "alloc-meter")]
    let baseline = meter::end(meter::begin()).live_end;
    let p = B::prepare(&rules);
    #[cfg(feature = "alloc-meter")]
    let prepared = meter::end(meter::begin()).live_end;
    let mut s = p.start(&q);
    for _ in 0..2_000 {
        match B::tick(&mut s) {
            Some(Some(a)) => {
                answers.push(a);
                break;
            }
            None => panic!("ongoing sibling exhausted"),
            _ => (),
        }
    }
    same(
        &answers,
        &[Answer {
            outputs: vec![],
            residual: vec![c("answer", [v(90), v(90)])],
        }],
    );
    for _ in 0..32 {
        assert!(B::tick(&mut s).is_some());
    }
    drop(s);
    answers.clear();
    #[cfg(feature = "alloc-meter")]
    assert_eq!(meter::end(meter::begin()).live_end, prepared);
    let mut s = p.start(&empty);
    finish::<B>(&mut s, &mut answers);
    assert_eq!(answers.len(), 1);
    assert!(answers[0].residual.is_empty());
    drop(s);
    drop(p);
    answers.clear();
    #[cfg(feature = "alloc-meter")]
    assert_eq!(meter::end(meter::begin()).live_end, baseline);
    println!("{{\"cancel_and_reuse\":true}}");
}
fn main() {
    #[cfg(feature = "alloc-meter")]
    meter::self_check().unwrap();
    let args = std::env::args().collect::<Vec<_>>();
    let mode = &args[1];
    if args[2] == "cancel" {
        match mode.as_str() {
            "local" => cancel::<Local>(),
            "scan" => cancel::<Compiled<false>>(),
            "indexed" => cancel::<Compiled<true>>(),
            "context" => cancel::<Context>(),
            _ => panic!(),
        };
        return;
    }
    let depth = args[3].parse().unwrap();
    let payload = args[4].parse().unwrap();
    let reuse = args[5].parse().unwrap();
    let keep = args[6].parse().unwrap();
    match mode.as_str() {
        "local" => run::<Local>(&args[2], depth, payload, reuse, keep),
        "scan" => run::<Compiled<false>>(&args[2], depth, payload, reuse, keep),
        "indexed" => run::<Compiled<true>>(&args[2], depth, payload, reuse, keep),
        "context" => run::<Context>(&args[2], depth, payload, reuse, keep),
        _ => panic!(),
    }
}
