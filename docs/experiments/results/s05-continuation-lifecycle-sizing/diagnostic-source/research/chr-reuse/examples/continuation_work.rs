#[path = "support/continuation_source.rs"]
mod source;
use chr_reuse::continuations::{COLLECT_METRICS, Mode, Prepared};
fn main() {
    if !COLLECT_METRICS {
        panic!("work diagnostics require metrics");
    }
    for family in ["exact", "rename", "history", "distinct"] {
        for resource in [false, true] {
            for mode in [Mode::Direct, Mode::ExactIds, Mode::Alpha, Mode::AlphaLive] {
                let schema = source::Schema::new(family, resource);
                let prepared = Prepared::new(schema.rules(), mode).unwrap();
                let mut run = prepared.start(schema.query(64, false)).unwrap();
                let batch = run.advance(2_000_000);
                assert!(batch.exhausted);
                assert_eq!(batch.answers.len(), schema.answer_count());
                println!("{family} resource={resource} {mode:?} {:?}", run.stats());
            }
        }
    }
}
