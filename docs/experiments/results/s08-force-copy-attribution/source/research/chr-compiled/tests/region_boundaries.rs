//! Independent boundary witnesses for region specialization. No cost measurements.
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
use chr_syntax::{Answer, Goal, Guard, Query, Rule, Var, and, atom, c, eq, or, t, v};
fn input(cs: Vec<chr_syntax::Constraint>, vars: &[u64]) -> Query {
    Query {
        constraints: cs,
        outputs: vars.iter().map(|&i| (format!("o{i}"), Var(i))).collect(),
    }
}
fn execute(rules: Vec<Rule>, query: Query) -> Vec<Answer> {
    let expected = scalar::run(&rules, &query, 100_000);
    let mut e = PreparedRuleset::new(rules, None)
        .unwrap()
        .start(query, Policy::Global, Access::Indexed)
        .unwrap()
        .into_search();
    let mut answers = vec![];
    for _ in 0..100_000 {
        match e.tick() {
            SearchEvent::Complete(mut b) => answers.push(b.engine.observe().unwrap()),
            SearchEvent::Exhausted => {
                scalar::same_raw(answers.clone(), expected);
                return answers;
            }
            _ => (),
        }
    }
    panic!("finite witness cutoff");
}
fn different(a: Vec<Answer>, mut b: Vec<Answer>) {
    if a.len() != b.len() {
        return;
    }
    for x in a {
        let Some(i) = b
            .iter()
            .position(|y| chr_observe::equivalent(&x, y, &mut Default::default()))
        else {
            return;
        };
        b.swap_remove(i);
    }
    panic!("expected distinct complete raw observations");
}

#[test]
fn unique_predicate_does_not_authorize_effect_reordering() {
    let qa = Rule::simplify("qa", [c("q", [atom("a")])], c("A", []).into());
    let qb = Rule::simplify("qb", [c("q", [v(0)])], c("B", []).into());
    let p = Rule::simplify("p", [c("p", [v(0)])], eq(v(0), atom("a")));
    let q = input(vec![c("q", [v(0)]), c("p", [v(0)])], &[0]);
    let source = execute(vec![qa.clone(), qb.clone(), p.clone()], q.clone());
    assert_eq!(
        source,
        vec![Answer {
            outputs: vec![("o0".into(), atom("a"))],
            residual: vec![c("B", [])]
        }]
    );
    let eager = execute(vec![p, qa, qb], q);
    assert_eq!(
        eager,
        vec![Answer {
            outputs: vec![("o0".into(), atom("a"))],
            residual: vec![c("A", [])]
        }]
    );
    different(source, eager);
}
#[test]
fn constructor_and_repeated_heads_cannot_become_binding_equations() {
    for repeated in [false, true] {
        let (head, terms, equation, vars) = if repeated {
            (
                c("p", [v(0), v(0)]),
                vec![v(0), v(1)],
                eq(v(0), v(1)),
                vec![0, 1],
            )
        } else {
            (
                c("p", [t("f", [v(0)])]),
                vec![v(0)],
                eq(v(0), t("f", [v(9)])),
                vec![0],
            )
        };
        let q = input(vec![c("p", terms.clone())], &vars);
        let source = execute(
            vec![Rule::simplify("source", [head], Goal::True)],
            q.clone(),
        );
        assert_eq!(source.len(), 1);
        assert_eq!(source[0].residual.len(), 1);
        let binding = execute(
            vec![Rule::simplify("binding", [c("p", terms)], equation)],
            q,
        );
        different(source, binding);
    }
}
#[test]
fn entailment_guards_cannot_be_used_as_solver_assertions() {
    let mut r = Rule::simplify("guard", [c("p", [v(0)])], Goal::True);
    r.guards = vec![Guard::Equal(v(0), atom("a"))];
    let q = input(vec![c("p", [v(0)])], &[0]);
    let source = execute(vec![r], q.clone());
    let assertion = execute(
        vec![Rule::simplify(
            "assert",
            [c("p", [v(0)])],
            eq(v(0), atom("a")),
        )],
        q,
    );
    different(source, assertion);
}
#[test]
fn external_observation_requires_occurrence_boundary() {
    let observer = Rule::propagate("observe", [c("p", [v(0)])], c("seen", [v(0)]).into());
    let relation = Rule::simplify("relation", [c("p", [v(0)])], c("out", [v(0)]).into());
    let q = input(vec![c("p", [atom("a")]), c("p", [atom("a")])], &[]);
    let source = execute(vec![observer.clone(), relation.clone()], q.clone());
    assert_eq!(source[0].residual.len(), 4);
    let eager = execute(vec![relation, observer], q);
    assert_eq!(eager[0].residual.len(), 2);
    different(source, eager);
}
#[test]
fn equal_choice_arms_and_open_constructors_remain_observable() {
    let q = input(vec![c("p", [])], &[]);
    let duplicate = execute(
        vec![Rule::simplify(
            "two",
            [c("p", [])],
            or(Goal::True, Goal::True),
        )],
        q.clone(),
    );
    let single = execute(vec![Rule::simplify("one", [c("p", [])], Goal::True)], q);
    assert_eq!(duplicate.len(), 2);
    assert_eq!(single.len(), 1);
    different(duplicate, single);
    let rules = vec![
        Rule::simplify("a", [c("p", [atom("a")])], Goal::True),
        Rule::simplify("b", [c("p", [atom("b")])], Goal::True),
    ];
    let q = input(vec![c("p", [atom("c")])], &[]);
    let open = execute(rules, q.clone());
    assert_eq!(open[0].residual, vec![c("p", [atom("c")])]);
    let closed = execute(
        vec![Rule::simplify("closed", [c("p", [v(0)])], Goal::Fail)],
        q,
    );
    assert!(closed.is_empty());
    different(open, closed);
}
#[test]
fn closed_query_inlining_preserves_opaque_unknowns_and_fresh_locals() {
    // This closed query has no interfering context between entry and p.
    // It demonstrates concrete inlining, not a general boundary-preserving lowering.
    for argument in [v(0), t("pair", [v(0), v(0)]), atom("a")] {
        let body = and(vec![
            eq(v(0), v(1)),
            or(c("out", [v(1), v(2)]).into(), c("out", [v(1), v(2)]).into()),
        ]);
        let wrapper = Rule::simplify("entry", [c("entry", [v(0)])], c("p", [v(0)]).into());
        let relation = Rule::simplify("p", [c("p", [v(0)])], body.clone());
        let q = input(vec![c("entry", [argument])], &[0]);
        let source = execute(vec![wrapper, relation], q.clone());
        let specialized = execute(vec![Rule::simplify("entry", [c("entry", [v(0)])], body)], q);
        scalar::same_raw(specialized, source);
    }
}
#[test]
fn equal_answer_sets_do_not_establish_equal_exhaustion_behavior() {
    let loop_rule = Rule::simplify("loop", [c("p", [])], c("p", []).into());
    let failure = Rule::simplify("failure", [c("q", [])], Goal::Fail);
    let q = input(vec![c("p", []), c("q", [])], &[]);
    let mut source = PreparedRuleset::new(vec![loop_rule.clone(), failure.clone()], None)
        .unwrap()
        .start(q.clone(), Policy::Global, Access::Indexed)
        .unwrap()
        .into_search();
    for _ in 0..10_000 {
        assert!(!matches!(
            source.tick(),
            SearchEvent::Exhausted | SearchEvent::Complete(_) | SearchEvent::Failed(_)
        ));
    }
    assert!(execute(vec![failure, loop_rule], q).is_empty());
}

#[test]
fn recursive_choice_unfolding_preserves_observed_finite_prefix_in_closed_query() {
    let one = Rule::simplify(
        "gen",
        [c("gen", [v(0)])],
        or(
            eq(v(0), atom("z")),
            and(vec![eq(v(0), t("s", [v(1)])), c("gen", [v(1)]).into()]),
        ),
    );
    let two = Rule::simplify(
        "gen",
        [c("gen", [v(0)])],
        or(
            eq(v(0), atom("z")),
            and(vec![
                eq(v(0), t("s", [v(1)])),
                or(
                    eq(v(1), atom("z")),
                    and(vec![eq(v(1), t("s", [v(2)])), c("gen", [v(2)]).into()]),
                ),
            ]),
        ),
    );
    for rule in [one, two] {
        let mut e = PreparedRuleset::new(vec![rule], None)
            .unwrap()
            .start(
                input(vec![c("gen", [v(0)])], &[0]),
                Policy::Global,
                Access::Indexed,
            )
            .unwrap()
            .into_search();
        let mut count = 0;
        for _ in 0..100_000 {
            match e.tick() {
                SearchEvent::Complete(mut b) => {
                    let expected = (0..count).fold(atom("z"), |t0, _| t("s", [t0]));
                    assert_eq!(
                        b.engine.observe(),
                        Some(Answer {
                            outputs: vec![("o0".into(), expected)],
                            residual: vec![]
                        })
                    );
                    count += 1;
                    if count == 8 {
                        break;
                    }
                }
                SearchEvent::Exhausted => panic!("unbounded generator reported exhaustion"),
                _ => (),
            }
        }
        assert_eq!(count, 8, "finite prefix starved");
    }
}
