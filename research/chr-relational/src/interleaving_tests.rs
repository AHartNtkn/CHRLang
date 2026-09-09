//! Structural control: count actual serviced equations outside production code.
use super::*;
use chr_syntax::{atom, c, eq, t, v};
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;

fn nest(depth: usize, mut leaf: Term) -> Term {
    for _ in 0..depth {
        leaf = t("f", [leaf]);
    }
    leaf
}
fn source(depth: usize, deep: bool, fail: bool) -> (Vec<Rule>, Query) {
    let pattern = if deep {
        nest(depth, atom("a"))
    } else {
        t("f", [v(0)])
    };
    let mut rules = vec![
        Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
        Rule::simplify(
            "use",
            [c("open", [pattern, v(1)])],
            if fail {
                Goal::Fail
            } else {
                c("done", []).into()
            },
        ),
    ];
    if deep {
        rules[1]
            .guards
            .push(chr_syntax::Guard::Equal(v(1), atom("a")));
    }
    let query = Query {
        constraints: vec![
            c("bind", [nest(depth, v(10)), nest(depth, atom("a"))]),
            c("open", [nest(depth, v(10)), v(10)]),
        ],
        outputs: vec![("leaf".into(), Var(10))],
    };
    (rules, query)
}
fn run(rules: &[Rule], query: &Query, barrier: bool) -> (Vec<Answer>, usize) {
    let mut engine = Prepared::new(rules).unwrap().start(query);
    let mut answers = vec![];
    let mut deductions = 0;
    for _ in 0..10_000 {
        if let Some(state) = engine.frontier.front_mut() {
            if barrier {
                while state.store.step() {
                    deductions += 1;
                    state.candidates.fill(None);
                }
            } else if state.store.pending() > 0 {
                deductions += 1;
            }
        }
        match engine.advance() {
            Step::Answer(a) => answers.push(a),
            Step::Exhausted => return (answers, deductions),
            Step::Progress => (),
        }
    }
    panic!("finite experiment exceeded service bound");
}
#[test]
fn useful_early_failure_and_settlement_dependent_control() {
    for depth in [4, 16, 64] {
        for deep in [false, true] {
            for fail in [false, true] {
                let (rules, query) = source(depth, deep, fail);
                let expected = oracle::run(&rules, &query, 10_000);
                assert_eq!(expected.len(), usize::from(!fail));
                let (early, early_work) = run(&rules, &query, false);
                let (settled, settled_work) = run(&rules, &query, true);
                oracle::same_raw(early, expected.clone());
                oracle::same_raw(settled, expected);
                if fail && !deep {
                    assert!(early_work < settled_work);
                } else {
                    assert_eq!(early_work, settled_work);
                }
                println!(
                    "depth={depth} deep={deep} fail={fail} interleaved={early_work} settled={settled_work}"
                );
            }
        }
    }
}

#[test]
fn merged_constructor_facts_are_unique_but_source_occurrences_are_not() {
    let mut store = Store::default();
    let x = store.unknown();
    let a = store.constructor("a", &[]);
    let fx = store.constructor("f", &[x]);
    let fa = store.constructor("f", &[a]);
    store.post("p", &[fx]);
    store.post("p", &[fa]);
    store.equate(x, a);
    while store.step() {}
    assert_eq!(store.view.descriptors(store.root(fx)).len(), 1);
    store
        .view
        .constructor("f", store.root(fx), &[store.root(a)]);
    assert_eq!(store.view.descriptors(store.root(fx)).len(), 1);
    let plan = HeadPlan::compile(&[], &[c("p", [t("f", [atom("a")])])]);
    assert_eq!(store.matches(&plan).matches.len(), 2);
}
