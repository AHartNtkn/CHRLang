use chr_structural::joint_region::{Observation, Region};
use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, atom, c, eq, or, t, v};
use std::collections::BTreeMap;
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
#[allow(dead_code)]
mod scalar;

fn choices(xs: &[Term], delay: usize) -> Goal {
    let branch = |x: &Term| {
        if x == &atom("a") && delay > 0 {
            let n = (0..delay).fold(atom("z"), |n, _| t("s", [n]));
            c("wait", [n, x.clone(), v(0)]).into()
        } else {
            eq(v(0), x.clone())
        }
    };
    let (last, rest) = xs.split_last().unwrap();
    rest.iter()
        .rev()
        .fold(branch(last), |tail, x| or(branch(x), tail))
}
fn rules(xs: &[Term], priority: usize, delay: usize) -> Vec<Rule> {
    let mut rules = vec![
        Rule::propagate("history", [c("watch", [v(0)])], c("seen", [v(0)]).into()),
        Rule::simplify(
            "a_wins",
            [c("watch", [atom("a")]), c("token", [])],
            c("winner", [atom("a")]).into(),
        ),
        Rule::simplify("launch", [c("start", [v(0)])], choices(xs, delay)),
    ];
    rules.insert(
        priority + 1,
        Rule::simplify(
            "other_wins",
            [c("token", [])],
            c("winner", [atom("other")]).into(),
        ),
    );
    rules.extend([
        Rule::simplify(
            "wait_step",
            [c("wait", [t("s", [v(0)]), v(1), v(2)])],
            c("wait", [v(0), v(1), v(2)]).into(),
        ),
        Rule::simplify(
            "wait_done",
            [c("wait", [atom("z"), v(0), v(1)])],
            eq(v(1), v(0)),
        ),
    ]);
    rules
}
fn bag(xs: Vec<Answer>) -> BTreeMap<Answer, usize> {
    let mut result = BTreeMap::new();
    for mut a in xs {
        a.residual.sort();
        *result.entry(a).or_default() += 1;
    }
    result
}
fn reference(rs: &[Rule], q: &Query) -> Vec<Answer> {
    let mut s = chr_reference::Search::new(rs.to_vec(), q.clone()).unwrap();
    let b = s.advance(10_000);
    assert!(b.exhausted);
    b.answers
}
#[test]
fn weighted_answers_keep_consumption_and_source_delivery_with_a_choice_plan() {
    let (mut cases, mut early_differences, mut order_differences, mut events) = (0, 0, 0, 0);
    for names in [vec!["a", "b"], vec!["b", "a"], vec!["a", "b", "a"]] {
        for reverse in [false, true] {
            let mut domain = names.iter().map(|x| atom(x)).collect::<Vec<_>>();
            if reverse {
                domain.reverse();
            }
            for priority in 0..3 {
                for alias in [false, true] {
                    for offset in [10, 1000] {
                        for delay in [0, 1, 4] {
                            let source = rules(&domain, priority, delay);
                            let mut q = Query {
                                constraints: vec![
                                    c("start", [v(offset)]),
                                    c("watch", [v(offset)]),
                                    c("token", []),
                                ],
                                outputs: vec![("out".into(), Var(offset))],
                            };
                            if alias {
                                q.outputs.push(("alias".into(), Var(offset)));
                            }
                            let region = Region {
                                domains: BTreeMap::from([(Var(0), domain.clone())]),
                                predicates: vec![],
                            };
                            let prepared = region
                                .prepare_sparse(&[Var(0)], &[], &[], Observation::Counted, 100)
                                .unwrap();
                            let rows = prepared
                                .weighted_iter(&[], 100)
                                .unwrap()
                                .collect::<Vec<_>>();
                            let sorted = rows
                                .iter()
                                .flat_map(|(row, n)| {
                                    std::iter::repeat_n(
                                        row[0].clone(),
                                        usize::try_from(*n).unwrap(),
                                    )
                                })
                                .collect::<Vec<_>>();
                            let expected_bag = bag(scalar::run(&source, &q, 10_000));
                            let expected_order = reference(&source, &q);
                            let sorted_rules = rules(&sorted, priority, delay);
                            assert_eq!(bag(scalar::run(&sorted_rules, &q, 10_000)), expected_bag);
                            order_differences +=
                                usize::from(reference(&sorted_rules, &q) != expected_order);

                            // Run each projected occurrence in its own branch-local caller.
                            // This deliberately tests binding before the original launch point.
                            let mut early = vec![];
                            for x in &sorted {
                                let mut bound = q.clone();
                                bound.constraints = vec![
                                    c("watch", [x.clone()]),
                                    c("token", []),
                                    c("result", [v(offset)]),
                                ];
                                let mut rs = source.clone();
                                rs.push(Rule::simplify(
                                    "result",
                                    [c("result", [v(0)])],
                                    eq(v(0), x.clone()),
                                ));
                                early.extend(scalar::run(&rs, &bound, 10_000));
                            }
                            early_differences += usize::from(bag(early) != expected_bag);

                            // The relation supplies values/counts; the source plan supplies
                            // each alternative's position. No raw ordering is inferred from weights.
                            let mut remaining = rows
                                .into_iter()
                                .map(|(row, n)| (row[0].clone(), n))
                                .collect::<BTreeMap<_, _>>();
                            let ordered = domain
                                .iter()
                                .map(|x| {
                                    let (value, count) = remaining.get_key_value(x).unwrap();
                                    assert!(*count > 0);
                                    let value = value.clone();
                                    *remaining.get_mut(x).unwrap() -= 1;
                                    value
                                })
                                .collect::<Vec<_>>();
                            assert!(remaining.values().all(|n| *n == 0));
                            let repaired = rules(&ordered, priority, delay);
                            assert_eq!(bag(scalar::run(&repaired, &q, 10_000)), expected_bag);
                            let mut a =
                                chr_reference::Search::new(source.clone(), q.clone()).unwrap();
                            let mut b =
                                chr_reference::Search::new(repaired.clone(), q.clone()).unwrap();
                            let mut held = vec![];
                            let mut exhausted = false;
                            for _ in 0..10_000 {
                                let (x, y) = (a.advance(1), b.advance(1));
                                assert_eq!(x.answers, y.answers);
                                assert_eq!(x.exhausted, y.exhausted);
                                held.extend(y.answers);
                                events += 1;
                                if x.exhausted {
                                    exhausted = true;
                                    break;
                                }
                            }
                            assert!(exhausted);
                            drop((a, b, prepared, region));
                            assert_eq!(held, expected_order);
                            for cutoff in [0, 1, 5] {
                                let mut cancelled =
                                    chr_reference::Search::new(repaired.clone(), q.clone())
                                        .unwrap();
                                let prefix = cancelled.advance(cutoff).answers;
                                drop(cancelled);
                                assert!(expected_order.starts_with(&prefix));
                                assert_eq!(reference(&repaired, &q), expected_order);
                            }
                            cases += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(cases, 216);
    assert_eq!(early_differences, 120);
    assert_eq!(order_differences, 24);
    eprintln!(
        "cases={cases} early_binding_bag_differences={early_differences} sorted_order_differences={order_differences} repaired_checkpoints={events}"
    );
}
