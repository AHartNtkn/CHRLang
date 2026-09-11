#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[allow(dead_code)]
#[path = "../examples/support/stream_run.rs"]
mod runtime;
#[path = "../examples/support/stream_source.rs"]
mod source;
use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, c, eq, or, t, v};
use runtime::{Event, Prepared, Running};
use source::Schema;
use std::collections::VecDeque;

const MODES: [&str; 8] = [
    "direct",
    "compact-live",
    "scan",
    "sealed",
    "dependencies",
    "templates",
    "lowered",
    "conditional",
];
enum Owner {
    Existing(Prepared),
    Conditional(chr_direct_conditional::engine::PreparedRuleset),
}
enum Service {
    Existing(Running),
    Conditional(Box<chr_direct_conditional::engine::Engine>),
}
impl Owner {
    fn new(mode: &str, schema: Schema, rules: Vec<Rule>) -> Self {
        if mode == "conditional" {
            Self::Conditional(chr_direct_conditional::engine::PreparedRuleset::new(rules).unwrap())
        } else {
            Self::Existing(Prepared::new(mode, schema, rules))
        }
    }
    fn start(&self, q: Query) -> Service {
        match self {
            Self::Existing(p) => Service::Existing(p.start(q)),
            Self::Conditional(p) => Service::Conditional(Box::new(p.start(q).unwrap())),
        }
    }
}
impl Service {
    fn tick(&mut self) -> Event {
        match self {
            Self::Existing(s) => s.tick(),
            Self::Conditional(s) => match s.tick() {
                chr_direct_conditional::engine::Event::Answer(a) => Event::Answer(a),
                chr_direct_conditional::engine::Event::Exhausted => Event::Done,
                _ => Event::Progress,
            },
        }
    }
}
struct Delivery {
    producer: Service,
    queued: VecDeque<Answer>,
    capacity: usize,
    calls: usize,
    blocked: usize,
    exhausted: bool,
}
impl Delivery {
    fn new(producer: Service, capacity: usize) -> Self {
        Self {
            producer,
            queued: VecDeque::new(),
            capacity,
            calls: 0,
            blocked: 0,
            exhausted: false,
        }
    }
    fn pump(&mut self) {
        if self.queued.len() == self.capacity {
            self.blocked += 1;
            return;
        }
        if self.exhausted {
            return;
        }
        self.calls += 1;
        assert!(self.calls <= 2_000_000, "producer service cutoff");
        match self.producer.tick() {
            Event::Answer(a) => self.queued.push_back(a),
            Event::Done => self.exhausted = true,
            Event::Progress => (),
        }
        assert!(self.queued.len() <= self.capacity);
    }
}
#[test]
fn bounded_delivery_preserves_changed_query_answers_and_retained_ownership() {
    let mut sessions = 0;
    let mut queries = 0;
    for family in ["repeated", "distinct", "aliases"] {
        for fail_tail in [false, true] {
            let schema = Schema {
                family,
                fail_tail,
                resource: true,
                work: 2,
                payload: 4,
            };
            for growing in [false, true] {
                // Independent full observations are established before candidate sessions.
                let expected = (0..32)
                    .map(|q| {
                        let n = if growing { 4 + q } else { 4 + q % 2 };
                        let e =
                            oracle::run(&schema.rules(), &schema.query(n, q % 2 == 1), 2_000_000);
                        oracle::same_raw(e.clone(), schema.expected(n));
                        e
                    })
                    .collect::<Vec<_>>();
                for mode in MODES {
                    for keep in [0, 4, usize::MAX] {
                        for burst in [false, true] {
                            let p = Owner::new(mode, schema, schema.rules());
                            let mut retained: VecDeque<Answer> = VecDeque::new();
                            let mut retained_expected: VecDeque<Answer> = VecDeque::new();
                            let mut blocked = 0;
                            for (q, expected) in expected.iter().enumerate() {
                                let n = if growing { 4 + q } else { 4 + q % 2 };
                                let mut delivery = Delivery::new(
                                    p.start(schema.query(n, q % 2 == 1)),
                                    if burst { 4 } else { 1 },
                                );
                                let mut observed = vec![];
                                let mut complete = false;
                                for _opportunity in 0..20_000_000 {
                                    delivery.pump();
                                    if delivery.queued.len() == delivery.capacity {
                                        let calls = delivery.calls;
                                        for _ in 0..5 {
                                            delivery.pump();
                                        }
                                        assert_eq!(
                                            calls, delivery.calls,
                                            "producer advanced against backpressure"
                                        );
                                    }
                                    if !burst
                                        || delivery.queued.len() == delivery.capacity
                                        || delivery.exhausted
                                    {
                                        if burst {
                                            let calls = delivery.calls;
                                            for _ in 0..8 {
                                                delivery.pump();
                                            }
                                            assert_eq!(calls, delivery.calls);
                                        }
                                        for _ in 0..if burst { 3 } else { 1 } {
                                            let Some(a) = delivery.queued.pop_front() else {
                                                break;
                                            };
                                            observed.push(a.clone());
                                            retained_expected.push_back(a.clone());
                                            retained.push_back(a);
                                            if retained.len() > keep {
                                                retained.pop_front();
                                                retained_expected.pop_front();
                                            }
                                        }
                                    }
                                    if delivery.exhausted && delivery.queued.is_empty() {
                                        complete = true;
                                        break;
                                    }
                                }
                                assert!(complete, "logical pump cutoff: {mode} {family} {q}");
                                oracle::same_raw(observed, expected.clone());
                                blocked += delivery.blocked;
                                drop(delivery);
                                assert_eq!(
                                    retained, retained_expected,
                                    "retained answer changed after engine disposal"
                                );
                                assert!(retained.len() <= keep);
                                queries += 1;
                            }
                            if burst {
                                assert!(
                                    blocked > 0,
                                    "burst schedule did not exercise backpressure"
                                );
                            }
                            drop(p);
                            assert_eq!(
                                retained, retained_expected,
                                "retained answer changed after prepared disposal"
                            );
                            sessions += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(sessions, 576);
    assert_eq!(queries, 18_432);
}
fn ongoing(mode: &str, rules: Vec<Rule>, query: Query, expected: Answer) {
    let schema = Schema {
        family: "aliases",
        fail_tail: false,
        resource: true,
        work: 2,
        payload: 4,
    };
    let p = Owner::new(mode, schema, rules);
    let mut delivery = Delivery::new(p.start(query.clone()), 1);
    for _ in 0..200_000 {
        delivery.pump();
        if !delivery.queued.is_empty() {
            break;
        }
        assert!(!delivery.exhausted, "ongoing source exhausted");
    }
    assert_eq!(delivery.queued.len(), 1, "finite answer starved: {mode}");
    let calls = delivery.calls;
    for _ in 0..100 {
        delivery.pump();
    }
    assert_eq!(calls, delivery.calls);
    let answer = delivery.queued.pop_front().unwrap();
    for _ in 0..100 {
        delivery.pump();
        assert!(!delivery.exhausted);
        assert!(delivery.queued.is_empty());
    }
    drop(delivery);
    let mut replay = Delivery::new(p.start(query), 1);
    for _ in 0..200_000 {
        replay.pump();
        if !replay.queued.is_empty() {
            break;
        }
        assert!(!replay.exhausted);
    }
    let second = replay
        .queued
        .pop_front()
        .expect("cancelled preparation failed reuse");
    oracle::same_raw(vec![second], vec![expected.clone()]);
    drop(replay);
    drop(p);
    oracle::same_raw(vec![answer], vec![expected]);
}
#[test]
fn paused_finite_answer_survives_ongoing_sibling_and_cancellation() {
    let rules = vec![
        Rule::simplify(
            "choose",
            [c("start", [v(0)])],
            or(eq(v(0), t("pair", [v(1), v(1)])), c("spin", [v(0)]).into()),
        ),
        Rule::simplify("spin", [c("spin", [v(0)])], c("spin", [v(0)]).into()),
        Rule::simplify("bad", [c("bad", [v(0)])], or(Goal::Fail, Goal::Fail)),
    ];
    let schema = Schema {
        family: "aliases",
        fail_tail: false,
        resource: true,
        work: 2,
        payload: 4,
    };
    for mode in MODES.into_iter().filter(|m| *m != "lowered") {
        ongoing(
            mode,
            rules.clone(),
            Query {
                constraints: vec![c("start", [v(10)])],
                outputs: vec![("answer".into(), Var(10))],
            },
            Answer {
                outputs: vec![("answer".into(), t("pair", [v(99), v(99)]))],
                residual: vec![],
            },
        );
        let p = Owner::new(mode, schema, rules.clone());
        let mut failed = Delivery::new(
            p.start(Query {
                constraints: vec![c("bad", [v(10)])],
                outputs: vec![],
            }),
            1,
        );
        for _ in 0..200_000 {
            failed.pump();
            assert!(failed.queued.is_empty());
            if failed.exhausted {
                break;
            }
        }
        assert!(failed.exhausted);
    }
}
#[test]
fn residual_only_ongoing_source_keeps_graph_admission_boundary_visible() {
    let rules = vec![
        Rule::simplify(
            "choose",
            [c("start", [])],
            or(c("ok", [v(0), v(0)]).into(), c("spin", []).into()),
        ),
        Rule::simplify("spin", [c("spin", [])], c("spin", []).into()),
    ];
    assert!(
        matches!(chr_direct_choice::demand::Prepared::with_reuse(rules.clone(),chr_direct_choice::demand::Reuse::MatchDependencies),Err(e) if e=="head needs distinct variable output")
    );
    for mode in ["direct", "compact-live", "scan", "sealed", "conditional"] {
        ongoing(
            mode,
            rules.clone(),
            Query {
                constraints: vec![c("start", [])],
                outputs: vec![],
            },
            Answer {
                outputs: vec![],
                residual: vec![c("ok", [Term::Var(Var(99)), Term::Var(Var(99))])],
            },
        );
    }
}

#[test]
fn continuing_emission_obeys_bounded_demand_and_retention() {
    let rules = vec![Rule::simplify(
        "emit-or-recur",
        [c("stream", [v(0)])],
        or(
            eq(v(0), t("pair", [v(1), v(1)])),
            c("stream", [v(0)]).into(),
        ),
    )];
    let schema = Schema {
        family: "aliases",
        fail_tail: false,
        resource: false,
        work: 0,
        payload: 0,
    };
    let expected = Answer {
        outputs: vec![("answer".into(), t("pair", [v(99), v(99)]))],
        residual: vec![],
    };
    for mode in MODES.into_iter().filter(|m| *m != "lowered") {
        for keep in [0, 4, usize::MAX] {
            for burst in [false, true] {
                let p = Owner::new(mode, schema, rules.clone());
                let mut d = Delivery::new(
                    p.start(Query {
                        constraints: vec![c("stream", [v(10)])],
                        outputs: vec![("answer".into(), Var(10))],
                    }),
                    if burst { 4 } else { 1 },
                );
                let mut retained = VecDeque::new();
                let mut delivered = 0;
                for target in [1, 8, 32, 64] {
                    while delivered < target {
                        while d.queued.len() < d.capacity {
                            d.pump();
                            assert!(!d.exhausted, "continuing emission exhausted: {mode}");
                        }
                        let calls = d.calls;
                        for _ in 0..8 {
                            d.pump();
                        }
                        assert_eq!(d.calls, calls);
                        for _ in 0..if burst { 3 } else { 1 } {
                            if delivered == target {
                                break;
                            }
                            let a = d.queued.pop_front().unwrap();
                            oracle::same_raw(vec![a.clone()], vec![expected.clone()]);
                            retained.push_back(a);
                            if retained.len() > keep {
                                retained.pop_front();
                            }
                            delivered += 1;
                        }
                    }
                    assert_eq!(retained.len(), keep.min(target));
                }
                assert!(d.blocked > 0);
                drop(d);
                drop(p);
                for a in retained {
                    oracle::same_raw(vec![a], vec![expected.clone()]);
                }
            }
        }
    }
}
