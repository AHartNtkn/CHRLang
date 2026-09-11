#[allow(dead_code)]
mod runtime_support;
#[path = "../examples/support/static_posts.rs"]
mod transform;
use chr_syntax::{Answer, Query, Rule, atom, c, eq, v};
#[test]
fn static_occurrences_initialize_once_without_losing_multiplicity() {
    let rules = vec![Rule::propagate(
        "once",
        [c("permit", [])],
        c("mark", []).into(),
    )];
    let p = transform::Prepared::infer(&rules).unwrap();
    for n in [0, 1, 2, 4] {
        let q = Query {
            constraints: vec![c("permit", []); n],
            outputs: vec![],
        };
        let expected = vec![Answer {
            outputs: vec![],
            residual: [vec![c("permit", []); n], vec![c("mark", []); n]].concat(),
        }];
        runtime_support::same_raw(runtime_support::run(&rules, &q, 200000), expected.clone());
        runtime_support::same_raw(
            runtime_support::run(p.rules(), &p.initialize(&q), 200000),
            expected,
        );
    }
}
#[test]
fn source_dependencies_prevent_premature_initialization() {
    let propagation = Rule::propagate("once", [c("permit", [])], c("mark", []).into());
    for interfering in [
        Rule::simplify("new", [c("new", [v(0)])], c("permit", []).into()),
        Rule::simplify("take", [c("permit", [])], eq(v(0), atom("ok"))),
        Rule::simplify("watch", [c("mark", [])], eq(v(0), atom("ok"))),
    ] {
        assert!(transform::Prepared::infer(&[propagation.clone(), interfering]).is_err());
    }
}
