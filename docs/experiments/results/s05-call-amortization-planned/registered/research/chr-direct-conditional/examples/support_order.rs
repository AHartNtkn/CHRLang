#[allow(dead_code)]
#[path = "../tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../../chr-reuse/examples/support/stream_source.rs"]
mod source;
use chr_direct_conditional::engine::{Event, HeadAdmission, PreparedRuleset};
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 4);
    let mode = args[1].as_str();
    let family = match args[2].as_str() {
        "aliases" => "aliases",
        "distinct" => "distinct",
        _ => panic!("family"),
    };
    let n: usize = args[3].parse().unwrap();
    assert!(matches!(n, 0 | 1 | 16 | 64));
    let schema = source::Schema {
        family,
        resource: true,
        fail_tail: false,
        work: 4,
        payload: 8,
    };
    let rules = schema.rules();
    let query = schema.query(n, false);
    let expected = oracle::run(&rules, &query, 2_000_000);
    oracle::same_raw(expected.clone(), schema.expected(n));
    let p = match mode {
        "conditional" => PreparedRuleset::new(rules).unwrap(),
        "inferred" => {
            PreparedRuleset::with_head_contract(rules, None, HeadAdmission::Optional).unwrap()
        }
        _ => panic!("mode"),
    };
    let mut e = p.start(query).unwrap();
    let mut answers = vec![];
    let mut done = false;
    let mut ticks = 0;
    for _ in 0..2_000_000 {
        ticks += 1;
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
    assert_eq!(answers.len(), n + 1);
    oracle::same_raw(answers, expected);
    println!(
        "{{\"mode\":\"{mode}\",\"family\":\"{family}\",\"depth\":{n},\"ticks\":{ticks},\"nodes\":{},\"trace\":{:?}}}",
        e.supports().node_count(),
        *e.supports().operation_trace()
    );
}
