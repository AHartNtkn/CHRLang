#![cfg(feature = "scheduled-templates")]
#[allow(dead_code)]
#[path = "../../chr-reuse/examples/support/stream_source.rs"]
mod source;
use chr_direct_choice::demand::{Event, Prepared};
#[test]
fn scheduled_derivation_retains_one_template_instead_of_every_suffix_key() {
    let schema = source::Schema {
        family: "repeated",
        resource: false,
        fail_tail: false,
        work: 32,
        payload: 1,
    };
    let mut counts = vec![];
    for limit in [0, 64] {
        let mut run = Prepared::new(schema.rules())
            .unwrap()
            .with_template_follow_limit(limit)
            .start(schema.query(0, false))
            .unwrap();
        let mut answers = vec![];
        let mut done = false;
        for _ in 0..200_000 {
            match run.tick() {
                Event::Answer(a) => answers.push(a),
                Event::Exhausted => {
                    done = true;
                    break;
                }
                Event::Progress => {}
            }
        }
        assert!(done);
        assert_eq!(answers, schema.expected(0));
        counts.push(run.retained_derivation_templates());
    }
    eprintln!("zero-follow/scheduled (templates, followed calls): {counts:?}");
    assert_eq!(counts, [(34, 0), (1, 33)]);
}
