#[path = "support/support_meter.rs"]
#[allow(dead_code)]
mod meter;
#[allow(dead_code)]
#[path = "../tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../../chr-reuse/examples/support/stream_source.rs"]
mod source;
use chr_direct_conditional::engine::{Engine, Event, HeadAdmission, PreparedRuleset};
use chr_direct_conditional::support::Support;
fn owners(e: &Engine) -> [usize; 11] {
    let s = e.store();
    let r = e.resources();
    let (index, known, discovery) = e.discovery_owners();
    [
        e.supports().node_count(),
        s.variable_count(),
        (0..s.variable_count()).map(|v| s.bindings(v).len()).sum(),
        s.changes().len(),
        r.occurrences().len(),
        r.occurrences()
            .iter()
            .filter(|o| o.live != Support::FALSE)
            .count(),
        r.history().len(),
        r.pending_bodies().len(),
        index,
        known,
        discovery,
    ]
}
#[allow(clippy::assertions_on_constants)]
fn main() {
    // Keep the diagnostic feature rejection at runtime, like other cost runners.
    assert!(!cfg!(feature = "metrics"));
    meter::self_check().unwrap();
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 4);
    let mode = args[1].as_str();
    let family = match args[2].as_str() {
        "aliases" => "aliases",
        "distinct" => "distinct",
        _ => panic!("family"),
    };
    let n: usize = args[3].parse().unwrap();
    assert!(matches!(n, 16 | 64));
    let schema = source::Schema {
        family,
        resource: true,
        fail_tail: false,
        work: 4,
        payload: 8,
    };
    let prepare = || match mode {
        "conditional" => PreparedRuleset::new(schema.rules()).unwrap(),
        "inferred" => {
            PreparedRuleset::with_head_contract(schema.rules(), None, HeadAdmission::Optional)
                .unwrap()
        }
        _ => panic!("mode"),
    };
    {
        let p = prepare();
        let query = schema.query(n, false);
        let expected = oracle::run(&schema.rules(), &query, 2_000_000);
        oracle::same_raw(expected.clone(), schema.expected(n));
        let mut e = p.start(query).unwrap();
        let mut answers = vec![];
        let mut done = false;
        for _ in 0..2_000_000 {
            match e.tick() {
                Event::Answer(a) => answers.push(a),
                Event::Exhausted => {
                    done = true;
                    break;
                }
                Event::Progress => (),
            }
        }
        assert!(done);
        oracle::same_raw(answers, expected);
    }
    println!("{{\"event\":\"start\"}}");
    let root = meter::begin();
    let p = prepare();
    let mut e = p.start(schema.query(n, false)).unwrap();
    let mut stages = [[0usize; 3]; 7];
    let mut support = [[0usize; 2]; 7];
    let mut snapshots = [(0usize, [0usize; 11]); 5];
    let mut ns = 0;
    let execution = meter::begin();
    let mut answers = 0;
    let mut done = false;
    for _ in 0..2_000_000 {
        let stage = e.allocation_stage();
        let before = meter::domains();
        let tick = meter::begin();
        let event = e.tick();
        let reading = meter::end(tick);
        let after = meter::domains();
        for k in 0..2 {
            support[stage][k] += after[k] - before[k];
        }
        stages[stage][0] += 1;
        stages[stage][1] += reading.requested_bytes;
        stages[stage][2] += reading.allocation_calls;
        match event {
            Event::Answer(a) => {
                answers += 1;
                drop(a);
                if [1, 4, 16, 64].contains(&answers) {
                    snapshots[ns] = (answers, owners(&e));
                    ns += 1;
                }
            }
            Event::Exhausted => {
                done = true;
                break;
            }
            Event::Progress => (),
        }
    }
    assert!(done);
    assert_eq!(answers, n + 1);
    snapshots[ns] = (answers, owners(&e));
    ns += 1;
    let measured = meter::end(execution);
    assert_eq!(
        stages.iter().map(|s| s[1]).sum::<usize>(),
        measured.requested_bytes
    );
    assert_eq!(
        stages.iter().map(|s| s[2]).sum::<usize>(),
        measured.allocation_calls
    );
    drop(e);
    drop(p);
    let restored = meter::end(root);
    assert_eq!(restored.live_start, restored.live_end);
    let snapshots = snapshots[..ns]
        .iter()
        .map(|(a, o)| format!("{{\"answers\":{a},\"owners\":{o:?}}}"))
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"depth\":{n},\"stages\":{stages:?},\"support_bytes\":{support:?},\"execution\":{},\"snapshots\":[{snapshots}],\"restored\":{}}}",
        measured.json(),
        restored.json()
    );
}
