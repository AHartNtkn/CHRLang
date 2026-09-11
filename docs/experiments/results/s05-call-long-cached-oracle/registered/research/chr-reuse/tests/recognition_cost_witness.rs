#[allow(dead_code)]
#[path = "../examples/support/inert_source.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[test]
fn short_source_exposes_missed_reuse_at_wider_stride() {
    let (rules, query) = fixture::source(0, 4, false, 7);
    let expected = oracle::run(&rules, &query, 200_000);
    for cancel in [false, true] {
        let mut rows = vec![];
        for stride in [1, 4, 16] {
            let mut run = chr_reuse::residuals::Prepared::new(rules.clone(), true)
                .unwrap()
                .with_recognition_stride(stride)
                .start(query.clone())
                .unwrap();
            let mut actual = vec![];
            let mut completed = false;
            for _ in 0..200_000 {
                let batch = run.advance(1);
                actual.extend(batch.answers);
                if batch.exhausted || (cancel && !actual.is_empty()) {
                    completed = true;
                    break;
                }
            }
            assert!(completed);
            oracle::same_raw(
                actual,
                if cancel {
                    expected[..1].to_vec()
                } else {
                    expected.clone()
                },
            );
            rows.push((
                run.stats().key_requests,
                run.stats().executed,
                run.stats().hits,
                run.stats().states,
            ));
        }
        if chr_reuse::continuations::COLLECT_METRICS {
            assert!(rows[2].0 < rows[1].0);
            assert!(rows[2].1 > rows[1].1);
            assert!(rows[2].3 > rows[1].3);
        }
        println!("cancel={cancel} stride1/4/16 (keys,executed,hits,states): {rows:?}");
    }
}
