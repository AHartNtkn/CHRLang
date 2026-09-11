//! T041 deterministic feasibility gate. The only controller intervention is the
//! observation-turn flag; source execution and joint export remain Engine code.
use super::*;
use chr_syntax::{atom, c, or, t, v};

const COUNTDOWN: usize = 16;
const TICK_LIMIT: usize = 5_000_000;

fn fixture(choices: usize) -> (Vec<Rule>, Query) {
    let mut bulk = vec![or(Goal::True, Goal::True); choices];
    bulk.push(c("bulk", [v(9), v(9)]).into());
    let countdown = (0..COUNTDOWN).fold(atom("z"), |n, _| t("s", [n]));
    (
        vec![
            Rule::simplify(
                "start",
                [c("start", [])],
                or(Goal::And(bulk), c("count", [countdown]).into()),
            ),
            Rule::simplify(
                "step",
                [c("count", [t("s", [v(0)])])],
                c("count", [v(0)]).into(),
            ),
            Rule::simplify("base", [c("count", [atom("z")])], c("sibling", []).into()),
        ],
        Query {
            constraints: vec![c("start", [])],
            outputs: vec![],
        },
    )
}

// This oracle is independent of observer traversal, trace, and support algebra.
fn is_bulk(answer: &Answer) -> bool {
    assert!(answer.outputs.is_empty());
    assert_eq!(answer.residual.len(), 1);
    let residual = &answer.residual[0];
    match residual.name.as_str() {
        "bulk" => {
            assert_eq!(residual.args.len(), 2);
            assert!(matches!(residual.args[0], Source::Var(_)));
            assert_eq!(residual.args[0], residual.args[1]);
            true
        }
        "sibling" => {
            assert!(residual.args.is_empty());
            false
        }
        other => panic!("unexpected residual {other}"),
    }
}

fn applications(engine: &Engine) -> usize {
    engine
        .trace()
        .iter()
        .filter(|e| matches!(e, Trace::Application { .. }))
        .count()
}

// Check actual source state, including equality/resource mutations, during each
// forced observer tick. Trace records also detect source births without relying
// on optional metrics. Observer internals are intentionally absent.
#[derive(Debug, PartialEq, Eq)]
struct SourceSnapshot {
    trace_len: usize,
    version: u64,
    live: Vec<Support>,
    pending: Dirty,
    round: Dirty,
    births: usize,
}

fn source_snapshot(engine: &Engine) -> SourceSnapshot {
    SourceSnapshot {
        trace_len: engine.trace().len(),
        version: engine.store.version(),
        live: engine
            .resources
            .occurrences()
            .iter()
            .map(|o| o.live)
            .collect(),
        pending: engine.pending.clone(),
        round: engine.round.clone(),
        births: engine.arena.variable_count(),
    }
}

#[derive(Debug)]
struct Receipt {
    choices: usize,
    strict: bool,
    bulk: usize,
    sibling: usize,
    bulk_before_sibling: Option<usize>,
    queued_apps: Option<usize>,
    drained_apps: Option<usize>,
    queued_pending: usize,
    sibling_discovered_with_bulk_owned: bool,
    exhausted: bool,
}

fn run(choices: usize, strict: bool) -> Receipt {
    let (rules, query) = fixture(choices);
    let mut engine = PreparedRuleset::new(rules).unwrap().start(query).unwrap();
    engine.enable_trace();
    let mut receipt = Receipt {
        choices,
        strict,
        bulk: 0,
        sibling: 0,
        bulk_before_sibling: None,
        queued_apps: None,
        drained_apps: None,
        queued_pending: 0,
        sibling_discovered_with_bulk_owned: false,
        exhausted: false,
    };
    let mut bulk_limit = None;
    for _ in 0..TICK_LIMIT {
        let suspended = strict && !engine.observations.is_empty();
        let before = suspended.then(|| source_snapshot(&engine));
        if suspended {
            // tick toggles this flag before dispatching the real observer.
            engine.observation_turn = false;
        }
        let event = engine.tick();
        if let Some(before) = before {
            assert_eq!(before, source_snapshot(&engine));
        }
        if bulk_limit.is_none() && !engine.observations.is_empty() {
            assert_eq!(engine.observations.len(), 1);
            bulk_limit = Some(engine.observations[0].occurrences);
            receipt.queued_apps = Some(applications(&engine));
            receipt.queued_pending = engine.pending.len();
            assert!(receipt.queued_pending > 0, "no queued source continuation");
            assert!(matches!(engine.phase, Phase::Begin));
            assert!(
                engine
                    .resources
                    .occurrences()
                    .iter()
                    .any(|o| { o.predicate == "count" && o.live != Support::FALSE }),
                "countdown sibling is not runnable at certification"
            );
            assert!(
                !engine
                    .resources
                    .occurrences()
                    .iter()
                    .any(|o| o.predicate == "sibling")
            );
        }
        if let Some(limit) = bulk_limit {
            let owned = engine.observations.iter().any(|o| o.occurrences == limit);
            let sibling_discovered = engine
                .resources
                .occurrences()
                .iter()
                .any(|o| o.predicate == "sibling");
            receipt.sibling_discovered_with_bulk_owned |= owned && sibling_discovered;
            if !owned && receipt.drained_apps.is_none() {
                receipt.drained_apps = Some(applications(&engine));
                assert_eq!(receipt.bulk, 1 << choices);
            }
        }
        match event {
            Event::Answer(answer) => {
                if is_bulk(&answer) {
                    receipt.bulk += 1;
                } else {
                    assert_eq!(receipt.sibling, 0);
                    receipt.sibling += 1;
                    receipt.bulk_before_sibling = Some(receipt.bulk);
                }
            }
            Event::Exhausted => {
                receipt.exhausted = true;
                break;
            }
            Event::Progress => (),
        }
    }
    eprintln!("T041 {receipt:?}");
    assert!(receipt.exhausted, "registered tick cap hit: {receipt:?}");
    assert_eq!(receipt.bulk, 1 << choices);
    assert_eq!(receipt.sibling, 1);
    assert_eq!(applications(&engine), COUNTDOWN + 2);
    assert!(receipt.drained_apps.is_some());
    if strict {
        assert_eq!(receipt.queued_apps, receipt.drained_apps);
        assert!(applications(&engine) > receipt.drained_apps.unwrap());
        assert_eq!(receipt.bulk_before_sibling, Some(1 << choices));
        assert!(!receipt.sibling_discovered_with_bulk_owned);
    }
    receipt
}

#[test]
fn publication_priority_preserves_answers_but_withholds_runnable_sibling() {
    for choices in [0, 2, 4, 8] {
        let ordinary = run(choices, false);
        let strict = run(choices, true);
        assert_eq!((ordinary.choices, ordinary.strict), (choices, false));
        assert_eq!((strict.choices, strict.strict), (choices, true));
        if choices == 8 {
            assert!(ordinary.sibling_discovered_with_bulk_owned);
            assert!(ordinary.bulk_before_sibling.unwrap() < 1 << choices);
            assert!(ordinary.drained_apps.unwrap() > ordinary.queued_apps.unwrap());
        }
    }
}

#[test]
fn cancellation_with_bulk_observer_owned_preserves_delivered_answer() {
    for strict in [false, true] {
        let (rules, query) = fixture(8);
        let mut engine = PreparedRuleset::new(rules).unwrap().start(query).unwrap();
        let mut initial_limit = None;
        let mut retained = None;
        for _ in 0..TICK_LIMIT {
            if strict && !engine.observations.is_empty() {
                engine.observation_turn = false;
            }
            let event = engine.tick();
            if initial_limit.is_none() {
                initial_limit = engine.observations.front().map(|o| o.occurrences);
            }
            match event {
                Event::Answer(answer) => {
                    assert!(is_bulk(&answer));
                    let limit = initial_limit.expect("answer without certified observer");
                    assert!(engine.observations.iter().any(|o| o.occurrences == limit));
                    retained = Some(answer);
                    break;
                }
                Event::Exhausted => panic!("premature exhaustion before bulk answer"),
                Event::Progress => (),
            }
        }
        let answer = retained.expect("registered tick cap hit before cancellation");
        drop(engine);
        // Validate the owned answer after cancelling a still-pending observer.
        // This makes no allocation or reclamation bound claim.
        assert!(is_bulk(&answer));
    }
}
