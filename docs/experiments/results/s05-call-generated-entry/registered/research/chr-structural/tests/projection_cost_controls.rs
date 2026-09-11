#[path = "../examples/support/projection_cost.rs"]
mod runtime;
#[test]
fn all_registered_sources_queries_endpoints_and_prefixes_match_independent_oracle() {
    let mut checks = 0;
    for family in ["independent", "star", "dense"] {
        for n in [4, 10] {
            let (p, visible) = runtime::source(family, n);
            for mode in runtime::MODES {
                if mode == "separable" && family == "star" {
                    continue;
                }
                let prepared = runtime::Prepared::new(mode, &p, &visible);
                for restriction in [None, Some(0), Some(1), None] {
                    for expanded in [false, true] {
                        let want = runtime::oracle(&p, &visible, restriction);
                        let want = if expanded {
                            runtime::expand(want)
                        } else {
                            want
                        };
                        assert_eq!(
                            prepared.start(restriction, expanded).collect::<Vec<_>>(),
                            want,
                            "{family} {n} {mode} {restriction:?} {expanded}"
                        );
                        assert_eq!(
                            prepared
                                .start(restriction, expanded)
                                .take(8)
                                .collect::<Vec<_>>(),
                            want.into_iter().take(8).collect::<Vec<_>>()
                        );
                        checks += 1;
                    }
                }
            }
        }
    }
    assert_eq!(checks, 272);
}
#[test]
#[should_panic(expected = "separability control excludes relational filters")]
fn separability_control_does_not_silently_run_another_engine() {
    let (p, v) = runtime::source("star", 4);
    runtime::Prepared::new("separable", &p, &v);
}
