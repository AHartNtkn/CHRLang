//! T070: source effects plus the work the native continuation must eliminate.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use chr_compiled::{Access, Execution, Policy, PreparedRuleset, fixtures};

#[test]
fn native_continuations_preserve_finite_source_effects_without_frame_snapshots() {
    let mut copies = 0;
    let mut pools = 0;
    let mut template_visits = 0;
    let mut control_copies = 0;
    let mut configurations = 0;
    for (id, rules) in fixtures::programs()
        .into_iter()
        .take(fixtures::ANALYTIC_PROGRAMS)
        .enumerate()
    {
        let native =
            PreparedRuleset::new(rules.clone(), Some(chr_compiled::access_bundled(id))).unwrap();
        let generated = PreparedRuleset::bundled(id, Execution::Generated).unwrap();
        let planned = PreparedRuleset::new_with_update_plan(rules.clone(), None).unwrap();
        let plain = PreparedRuleset::new(rules.clone(), None).unwrap();
        for n in 0..4 {
            let case = fixtures::case(id, n);
            let expected = scalar::run_traced(&rules, &case.query, 100_000);
            for policy in [Policy::Global, Policy::Active] {
                for access in [Access::Scan, Access::Indexed] {
                    let mut control = plain.start(case.query.clone(), policy, access).unwrap();
                    let mut actual = native.start(case.query.clone(), policy, access).unwrap();
                    control.enable_trace();
                    actual.enable_trace();
                    assert!(control.advance(1_000_000).exhausted);
                    let status = actual.advance(1_000_000);
                    assert!(status.exhausted);
                    assert_eq!(status.failed, control.status().failed);
                    assert_eq!(
                        actual.trace(),
                        control.trace(),
                        "id={id} n={n} {policy:?} {access:?}"
                    );
                    let got = actual.observe().into_iter().collect::<Vec<_>>();
                    scalar::same_raw(got.clone(), control.observe().into_iter().collect());
                    if policy == Policy::Global {
                        scalar::same_raw(got, expected.iter().map(|(a, _)| a.clone()).collect());
                    }
                    let mut previous = generated.start(case.query.clone(), policy, access).unwrap();
                    previous.enable_trace();
                    assert!(previous.advance(1_000_000).exhausted);
                    assert_eq!(actual.trace(), previous.trace());
                    scalar::same_raw(
                        actual.observe().into_iter().collect(),
                        previous.observe().into_iter().collect(),
                    );
                    let mut data = planned.start(case.query.clone(), policy, access).unwrap();
                    data.enable_trace();
                    assert!(data.advance(1_000_000).exhausted);
                    assert_eq!(data.trace(), actual.trace());
                    scalar::same_raw(
                        data.observe().into_iter().collect(),
                        actual.observe().into_iter().collect(),
                    );
                    control_copies += previous.stats().binding_slot_copies;
                    template_visits += actual.stats().key_template_visits;
                    configurations += 1;
                    copies += actual.stats().binding_slot_copies;
                    pools += actual.stats().cursor_pool_entries;
                }
            }
        }
    }
    println!(
        "configurations={configurations} native_slot_copies={copies} native_pool_entries={pools} native_template_visits={template_visits} control_slot_copies={control_copies}"
    );
    if chr_compiled::COLLECT_METRICS {
        assert!(control_copies > 0);
        assert_eq!(template_visits, 0);
        assert_eq!(copies, 0, "native access still copies binding frames");
        assert_eq!(
            pools, 0,
            "native access still materializes candidate vectors"
        );
    }
}

#[test]
fn independent_native_queries_outlive_preparation_and_abandoned_work() {
    let prepared = PreparedRuleset::new(
        fixtures::programs()[1].clone(),
        Some(chr_compiled::access_bundled(1)),
    )
    .unwrap();
    let cases = [
        fixtures::flat_chain_case(4, false),
        fixtures::flat_chain_case(7, false),
    ];
    let mut engines = cases
        .iter()
        .map(|c| {
            prepared
                .start(c.query.clone(), Policy::Global, Access::Indexed)
                .unwrap()
        })
        .collect::<Vec<_>>();
    let mut abandoned = prepared
        .start(
            fixtures::flat_chain_case(32, false).query,
            Policy::Active,
            Access::Indexed,
        )
        .unwrap();
    for _ in 0..30 {
        for engine in &mut engines {
            engine.step();
        }
        abandoned.step();
    }
    drop(abandoned);
    drop(prepared);
    for (mut engine, case) in engines.into_iter().zip(cases) {
        assert!(engine.advance(100_000).exhausted);
        scalar::same_raw(
            engine.observe().into_iter().collect(),
            case.expected.into_iter().collect(),
        );
    }
}

#[test]
fn source_derived_updates_do_not_index_head_local_output_arguments() {
    let prepared = PreparedRuleset::new(
        fixtures::programs()[1].clone(),
        Some(chr_compiled::access_bundled(1)),
    )
    .unwrap();
    let case = fixtures::flat_chain_case(3, false);
    for policy in [Policy::Global, Policy::Active] {
        let mut e = prepared
            .start(case.query.clone(), policy, Access::Indexed)
            .unwrap();
        assert!(e.advance(100_000).exhausted);
        scalar::same_raw(
            e.observe().into_iter().collect(),
            case.expected.clone().into_iter().collect(),
        );
        if chr_compiled::COLLECT_METRICS {
            // Three edge source keys and four reach keys. An edge's target is
            // never available before this head is selected; indexing it is unused.
            assert_eq!(e.stats().index_entries, 7);
        }
    }
}

#[allow(dead_code)]
#[path = "../experiments/access_source.rs"]
mod update_source;
#[test]
fn projected_updates_preserve_late_keys_and_opaque_payload_bindings() {
    let rules = update_source::payload_rules();
    let code = chr_compiled::access_payload_bundled(0);
    let native = PreparedRuleset::new(rules.clone(), Some(code)).unwrap();
    let planned = PreparedRuleset::new_with_update_plan(rules.clone(), None).unwrap();
    let mut unprojected = code;
    unprojected.updates = None;
    let control = PreparedRuleset::new(rules.clone(), Some(unprojected)).unwrap();
    for width in [0, 1, 16, 64] {
        for late in [false, true] {
            let query = update_source::payload_query(width, late);
            let expected = scalar::run_traced(&rules, &query, 100_000);
            for policy in [Policy::Global, Policy::Active] {
                for access in [Access::Scan, Access::Indexed] {
                    let mut a = native.start(query.clone(), policy, access).unwrap();
                    let mut b = control.start(query.clone(), policy, access).unwrap();
                    a.enable_trace();
                    b.enable_trace();
                    assert!(a.advance(100_000).exhausted);
                    assert!(b.advance(100_000).exhausted);
                    assert_eq!(a.trace(), b.trace());
                    scalar::same_raw(
                        a.observe().into_iter().collect(),
                        expected.iter().map(|(a, _)| a.clone()).collect(),
                    );
                    scalar::same_raw(
                        b.observe().into_iter().collect(),
                        expected.iter().map(|(a, _)| a.clone()).collect(),
                    );
                    let mut data = planned.start(query.clone(), policy, access).unwrap();
                    data.enable_trace();
                    assert!(data.advance(100_000).exhausted);
                    assert_eq!(data.trace(), a.trace());
                    scalar::same_raw(
                        data.observe().into_iter().collect(),
                        a.observe().into_iter().collect(),
                    );
                    assert_eq!(data.stats().dependency_visits, a.stats().dependency_visits);
                    assert_eq!(data.stats().index_inserts, a.stats().index_inserts);
                    if chr_compiled::COLLECT_METRICS && access == Access::Indexed {
                        assert_eq!(a.stats().index_entries, 2);
                        assert_eq!(b.stats().index_entries, 3);
                        if policy == Policy::Global && width > 0 {
                            assert!(a.stats().dependency_visits < b.stats().dependency_visits);
                        }
                        println!(
                            "payload width={width} late={late} policy={policy:?} native_visits={} control_visits={} native_refresh={} control_refresh={} native_indexes={} control_indexes={}",
                            a.stats().dependency_visits,
                            b.stats().dependency_visits,
                            a.stats().dependency_refreshes,
                            b.stats().dependency_refreshes,
                            a.stats().index_inserts,
                            b.stats().index_inserts
                        );
                    }
                }
            }
        }
    }
}
