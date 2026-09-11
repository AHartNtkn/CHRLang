#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use chr_relational::contextual_execute::{Prepared, Step};
use chr_syntax::{Answer, Query, Rule, atom, c, v};
fn check(rules: &[Rule], query: &Query, expected: Vec<Answer>) -> Vec<u64> {
    scalar::same_raw(scalar::run(rules, query, 500_000), expected.clone());
    let p = Prepared::new(rules).unwrap();
    let mut counts = vec![];
    for mut e in [
        p.start(query),
        p.start_demand(query),
        p.start_resumable(query),
    ] {
        let mut answers = vec![];
        let mut exhausted = false;
        for _ in 0..500_000 {
            match e.advance() {
                Step::Answer(a) => answers.push(a),
                Step::Exhausted => {
                    exhausted = true;
                    break;
                }
                Step::Progress => (),
            }
        }
        assert!(exhausted);
        scalar::same_raw(answers, expected.clone());
        counts.push(e.discovery_stats().offered);
    }
    counts
}
#[test]
fn retained_progress_avoids_repeated_source_history() {
    for n in [1, 16, 48] {
        let rules = vec![Rule::propagate(
            "visit",
            [c("item", [v(0)])],
            c("mark", [v(0)]).into(),
        )];
        let mut initial = vec![];
        let mut residual = vec![];
        for i in 0..n {
            let x = atom(&format!("v{i}"));
            initial.push(c("item", [x.clone()]));
            residual.push(c("item", [x.clone()]));
            residual.push(c("mark", [x]));
        }
        let counts = check(
            &rules,
            &Query {
                constraints: initial,
                outputs: vec![],
            },
            vec![Answer {
                outputs: vec![],
                residual,
            }],
        );
        #[cfg(feature = "local-work")]
        assert_eq!(counts, vec![n, n * (n + 1) / 2 + n, n]);
        println!("history n={n} eager/demand/resumable offered={counts:?}");
    }
}
#[test]
fn inserting_a_head_reopens_an_exhausted_earlier_rule() {
    let rules = vec![
        Rule::simplify("take", [c("p", [atom("a")])], c("done", []).into()),
        Rule::simplify("create", [c("start", [])], c("p", [atom("a")]).into()),
    ];
    check(
        &rules,
        &Query {
            constraints: vec![c("start", [])],
            outputs: vec![],
        },
        vec![Answer {
            outputs: vec![],
            residual: vec![c("done", [])],
        }],
    );
}

#[test]
fn consuming_a_suffix_preserves_the_live_prefix_and_distinctness() {
    let rules = vec![Rule {
        name: "consume".into(),
        kept: vec![c("p", [v(0)])],
        removed: vec![c("p", [v(1)])],
        guards: vec![],
        body: c("seen", [v(0), v(1)]).into(),
    }];
    let constraints = (0..6).map(|i| c("p", [atom(&format!("v{i}"))])).collect();
    let mut residual = vec![c("p", [atom("v0")])];
    for i in 1..6 {
        residual.push(c("seen", [atom("v0"), atom(&format!("v{i}"))]));
    }
    check(
        &rules,
        &Query {
            constraints,
            outputs: vec![],
        },
        vec![Answer {
            outputs: vec![],
            residual,
        }],
    );
}

#[test]
fn dense_duplicate_occurrences_keep_ordered_pair_multiplicity() {
    let rules = vec![Rule::propagate(
        "pair",
        [c("p", [v(0)]), c("p", [v(0)])],
        c("seen", [v(0), v(0)]).into(),
    )];
    let constraints = vec![c("p", [atom("x")]); 6];
    let mut residual = constraints.clone();
    residual.extend(vec![c("seen", [atom("x"), atom("x")]); 30]);
    let counts = check(
        &rules,
        &Query {
            constraints,
            outputs: vec![],
        },
        vec![Answer {
            outputs: vec![],
            residual,
        }],
    );
    #[cfg(feature = "local-work")]
    assert_eq!(counts, vec![30, 495, 30]);
    println!("dense eager/demand/resumable offered={counts:?}");
}

#[test]
fn consuming_the_first_prefix_advances_to_the_next_live_pair() {
    let rules = vec![Rule::simplify(
        "pair",
        [c("p", [v(0)]), c("p", [v(1)])],
        c("seen", [v(0), v(1)]).into(),
    )];
    let constraints = (0..5).map(|i| c("p", [atom(&format!("v{i}"))])).collect();
    let residual = vec![
        c("seen", [atom("v0"), atom("v1")]),
        c("seen", [atom("v2"), atom("v3")]),
        c("p", [atom("v4")]),
    ];
    check(
        &rules,
        &Query {
            constraints,
            outputs: vec![],
        },
        vec![Answer {
            outputs: vec![],
            residual,
        }],
    );
}

#[test]
fn redundant_equality_exposes_invalidation_policy() {
    use chr_syntax::{and, eq};
    let rules = vec![Rule::propagate(
        "visit",
        [c("item", [v(0)])],
        and([c("mark", [v(0)]).into(), eq(v(0), v(0))]),
    )];
    let constraints = vec![c("item", [atom("x")]); 16];
    let mut residual = constraints.clone();
    residual.extend(vec![c("mark", [atom("x")]); 16]);
    let counts = check(
        &rules,
        &Query {
            constraints,
            outputs: vec![],
        },
        vec![Answer {
            outputs: vec![],
            residual,
        }],
    );
    #[cfg(feature = "local-work")]
    assert_eq!(
        counts,
        if cfg!(feature = "precise-invalidation") {
            vec![16, 152, 16]
        } else {
            vec![152, 152, 152]
        }
    );
    println!("equality-reset eager/demand/resumable offered={counts:?}");
}
