//! Failure after reservation must remain source failure and not contaminate reuse.
#[allow(dead_code)]
#[path = "support/chr_forest.rs"]
mod forest;
#[allow(dead_code)]
#[path = "support/chr_constructors.rs"]
mod kernel;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "support/chr_selection.rs"]
mod selection;
#[path = "support/selection_source.rs"]
mod source;
use chr_syntax::{Query, atom, c, t, v};
#[test]
fn clashes_and_cycles_after_reservation_preserve_failure_and_reuse() {
    let cases = [
        vec![c("take", [t("f", [atom("a")]), atom("b")]), c("token", [])],
        vec![
            c("take", [t("f", [t("f", [atom("a")])]), v(0)]),
            c("take", [v(0), atom("b")]),
            c("token", []),
            c("token", []),
        ],
        vec![c("take", [t("f", [t("f", [v(0)])]), v(0)]), c("token", [])],
    ];
    for reserved in [false, true] {
        let rules = if reserved {
            selection::reserved_rules()
        } else {
            selection::rules(true)
        };
        for (access, special) in [
            (chr_compiled::Access::Scan, false),
            (chr_compiled::Access::Indexed, false),
            (chr_compiled::Access::Scan, true),
        ] {
            let mut prepared = chr_compiled::PreparedRuleset::new(rules.clone(), None).unwrap();
            if special {
                prepared = prepared.specialize_inferred();
            }
            for constraints in &cases {
                let q = Query {
                    constraints: constraints.clone(),
                    outputs: vec![],
                };
                assert!(oracle::run(&[source::rule()], &q, 200_000).is_empty());
                let encoded = source::encode(&q);
                assert!(oracle::run(&rules, &encoded.query, 200_000).is_empty());
                let mut search = prepared
                    .start(encoded.query, chr_compiled::Policy::Global, access)
                    .unwrap();
                search.enable_trace();
                let status = search.advance(200_000);
                assert!(status.exhausted && status.failed);
                assert!(search.observe().is_none());
                assert!(
                    search
                        .trace()
                        .iter()
                        .any(|(i, _)| rules[*i].name == "commit-earliest")
                );
                if reserved {
                    assert!(
                        search
                            .trace()
                            .iter()
                            .any(|(i, _)| rules[*i].name == "open-round")
                    );
                }
                drop(search);
                let good = source::query("chain", 2, "b");
                let expected = oracle::run(&[source::rule()], &good, 200_000);
                let encoded = source::encode(&good);
                let mut search = prepared
                    .start(encoded.query.clone(), chr_compiled::Policy::Global, access)
                    .unwrap();
                assert!(search.advance(200_000).exhausted);
                let answer = source::decode(search.observe().unwrap(), &encoded);
                drop(search);
                oracle::same_raw(vec![answer], expected);
            }
        }
    }
}
