#[path = "../examples/support/inert_source.rs"]
mod source;
use chr_reuse::continuations::{Mode, Prepared};
#[test]
fn owned_and_compact_key_requests_are_accounted_inside_total() {
    use chr_compiled::experiment::meter;
    let mut key_bytes = vec![];
    for mode in [Mode::Direct, Mode::AlphaLive, Mode::CompactLive] {
        let (rules, query) = source::source(4, 8, true, 7);
        let mark = meter::begin();
        let prepared = Prepared::new(rules, mode).unwrap();
        let mut run = prepared.start(query).unwrap();
        let answers = run.advance(200_000);
        assert!(answers.exhausted);
        assert_eq!(answers.answers.len(), 2);
        let profile = run.allocation_profile();
        let total = meter::end(mark).requested_bytes;
        assert!(profile.bytes.iter().sum::<usize>() <= total);
        key_bytes.push(profile.bytes[0]);
    }
    assert_eq!(key_bytes[0], 0);
    assert!(key_bytes[1] > key_bytes[2] && key_bytes[2] > 0);
}
