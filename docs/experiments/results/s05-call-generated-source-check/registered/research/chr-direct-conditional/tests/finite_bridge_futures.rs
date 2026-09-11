//! Supplementary check of weighted caller replay with later choices and fresh aliases.
#[path = "../examples/support/finite_bridge.rs"]
mod bridge;
#[allow(dead_code)]
#[path = "../../chr-compiled/experiments/finite_phase.rs"]
mod finite_phase;
#[allow(dead_code)]
mod runtime_support;
use chr_syntax::{Query, Rule, Var, and, atom, c, eq, or, t, v};
#[test]
fn weights_multiply_later_choices_and_preserve_fresh_output_aliases() {
    let rules = vec![
        Rule::simplify(
            "choice",
            [c("choice", [v(0)])],
            or(
                eq(v(0), atom("a")),
                or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
            ),
        ),
        Rule::simplify(
            "caller",
            [c("caller", [v(0), v(1)])],
            and(vec![
                c("choice", [v(2)]).into(),
                eq(v(1), t("result", [v(0), v(2), v(3), v(3)])),
            ]),
        ),
    ];
    let q = Query {
        constraints: vec![c("choice", [v(100)]), c("caller", [v(100), v(101)])],
        outputs: vec![("x".into(), Var(100)), ("out".into(), Var(101))],
    };
    let phase = finite_phase::Prepared::new(&rules, 1).unwrap();
    let caller = bridge::Bridge::new(rules.clone());
    let mut answers = vec![];
    for solution in phase.solve(&q, Default::default()).unwrap().solutions {
        let (q, weight) = caller.transport(solution);
        let mut engine = caller.start(q, weight);
        let mut done = false;
        for _ in 0..10000 {
            match engine.step() {
                bridge::Event::Progress => (),
                bridge::Event::Answer(a) => answers.push(a),
                bridge::Event::Exhausted => {
                    done = true;
                    break;
                }
            }
        }
        assert!(done);
    }
    assert_eq!(answers.len(), 9);
    runtime_support::same_raw(answers, runtime_support::run(&rules, &q, 10000));
}
