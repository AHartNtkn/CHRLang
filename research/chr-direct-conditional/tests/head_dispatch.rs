#![cfg(feature = "head-dispatch")]
#[allow(dead_code)]
mod runtime_support;
use chr_direct_conditional::engine::{Event, HeadAdmission, HeadDeclaration, PreparedRuleset};
use chr_syntax::{Answer, Query, Rule, atom, c, eq, v};
#[test]
fn inferred_and_declared_unary_sources_remove_discovery_owners() {
    let rules = vec![
        Rule::simplify("a", [c("p", [atom("a")])], c("done_a", []).into()),
        Rule::simplify("b", [c("p", [atom("b")])], c("done_b", []).into()),
        Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("b"))),
    ];
    let q = Query {
        constraints: vec![
            c("p", [v(100)]),
            c("bind", [v(100)]),
            c("p", [atom("a")]),
            c("p", [atom("a")]),
        ],
        outputs: vec![],
    };
    let expected = vec![Answer {
        outputs: vec![],
        residual: vec![c("done_a", []), c("done_a", []), c("done_b", [])],
    }];
    runtime_support::same_raw(runtime_support::run(&rules, &q, 100_000), expected.clone());
    for (declaration, admission) in [
        (None, HeadAdmission::Optional),
        (Some(HeadDeclaration::Single), HeadAdmission::Optional),
        (Some(HeadDeclaration::Single), HeadAdmission::Required),
    ] {
        let p = PreparedRuleset::with_head_contract(rules.clone(), declaration, admission).unwrap();
        assert!(p.has_only_direct_head_dispatch());
        let mut e = p.start(q.clone()).unwrap();
        drop(p);
        let mut answers = vec![];
        let mut done = false;
        for _ in 0..100_000 {
            assert_eq!(e.discovery_owners(), (0, 0, 0));
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
        runtime_support::same_raw(answers, expected.clone());
        println!(
            "declaration={declaration:?} admission={admission:?} ticks={}",
            e.stats().ticks
        );
    }
    assert!(PreparedRuleset::with_head_contract(rules, None, HeadAdmission::Required).is_err());
}
#[test]
fn linked_multihead_source_must_be_rechecked() {
    let unary = vec![Rule::simplify("p", [c("p", [])], c("done", []).into())];
    let prior = PreparedRuleset::with_head_contract(
        unary.clone(),
        Some(HeadDeclaration::Single),
        HeadAdmission::Required,
    )
    .unwrap();
    let mut extended = unary;
    extended.push(Rule::simplify(
        "join",
        [c("q", []), c("r", [])],
        c("joined", []).into(),
    ));
    assert!(
        PreparedRuleset::with_head_contract(
            extended.clone(),
            Some(HeadDeclaration::Single),
            HeadAdmission::Optional
        )
        .is_err()
    );
    assert!(
        PreparedRuleset::with_head_contract(
            extended.clone(),
            Some(HeadDeclaration::Single),
            HeadAdmission::Required
        )
        .is_err()
    );
    let inferred =
        PreparedRuleset::with_head_contract(extended, None, HeadAdmission::Optional).unwrap();
    assert!(!inferred.has_only_direct_head_dispatch());
    assert_eq!(inferred.direct_head_rule_count(), 1);
    collect(
        inferred
            .start(Query {
                constraints: vec![c("p", []), c("q", []), c("r", [])],
                outputs: vec![],
            })
            .unwrap(),
        vec![Answer {
            outputs: vec![],
            residual: vec![c("done", []), c("joined", [])],
        }],
    );
    assert!(prior.has_only_direct_head_dispatch());
}

fn collect(mut e: chr_direct_conditional::engine::Engine, expected: Vec<Answer>) -> (u64, u64) {
    let mut answers = vec![];
    let mut done = false;
    for _ in 0..100_000 {
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
    runtime_support::same_raw(answers, expected);
    (e.stats().ticks, e.stats().discovered_tuples)
}
#[test]
fn reused_preparation_preserves_history_priority_and_changed_queries() {
    let rules = vec![Rule::propagate(
        "visit",
        [c("p", [v(0)])],
        c("mark", [v(0)]).into(),
    )];
    let direct =
        PreparedRuleset::with_head_contract(rules.clone(), None, HeadAdmission::Optional).unwrap();
    let ordinary = PreparedRuleset::new(rules.clone()).unwrap();
    for n in [0, 1, 16] {
        let x = atom(&format!("a{n}"));
        let q = Query {
            constraints: vec![c("p", [x.clone()]); n],
            outputs: vec![],
        };
        let mut residual = q.constraints.clone();
        residual.extend(vec![c("mark", [x]); n]);
        let expected = vec![Answer {
            outputs: vec![],
            residual,
        }];
        runtime_support::same_raw(runtime_support::run(&rules, &q, 100_000), expected.clone());
        let a = collect(ordinary.start(q.clone()).unwrap(), expected.clone());
        let b = collect(direct.start(q).unwrap(), expected);
        #[cfg(feature = "metrics")]
        {
            assert_eq!(a.1, b.1);
            if n > 0 {
                assert!(b.0 < a.0);
            }
        }
        println!("history n={n} ordinary={a:?} direct={b:?}");
    }
    let mixed = vec![Rule::simplify(
        "join",
        [c("p", []), c("q", [])],
        c("joined", []).into(),
    )];
    let p = PreparedRuleset::with_head_contract(mixed, None, HeadAdmission::Optional).unwrap();
    collect(
        p.start(Query {
            constraints: vec![c("p", []), c("q", [])],
            outputs: vec![],
        })
        .unwrap(),
        vec![Answer {
            outputs: vec![],
            residual: vec![c("joined", [])],
        }],
    );
}
#[test]
fn finite_answers_and_query_owners_survive_continuing_unary_work() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [])],
            chr_syntax::or(c("done", []).into(), c("spin", []).into()),
        ),
        Rule::simplify("spin", [c("spin", [])], c("spin", []).into()),
    ];
    let p = PreparedRuleset::with_head_contract(
        rules,
        Some(HeadDeclaration::Single),
        HeadAdmission::Required,
    )
    .unwrap();
    let mut e = p
        .start(Query {
            constraints: vec![c("start", [])],
            outputs: vec![],
        })
        .unwrap();
    drop(p);
    let mut answer = None;
    for _ in 0..10_000 {
        assert_eq!(e.discovery_owners(), (0, 0, 0));
        match e.tick() {
            Event::Answer(a) => {
                answer = Some(a);
                break;
            }
            Event::Exhausted => panic!("false exhaustion"),
            Event::Progress => (),
        }
    }
    let answer = answer.expect("finite service");
    for _ in 0..1024 {
        assert!(matches!(e.tick(), Event::Progress));
    }
    drop(e);
    runtime_support::same_raw(
        vec![answer],
        vec![Answer {
            outputs: vec![],
            residual: vec![c("done", [])],
        }],
    );
}
