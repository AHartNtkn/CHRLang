#[path = "../examples/support/finite_bridge.rs"]
mod bridge;
#[path = "../../chr-compiled/experiments/finite_phase.rs"]
#[allow(dead_code)]
mod finite_phase;
#[allow(dead_code)]
mod runtime_support;
#[path = "../examples/support/finite_cost_source.rs"]
mod source;
use bridge::Event;
#[test]
fn prepared_bridge_and_weighted_raw_delivery_match_independent_source() {
    let mut count = 0;
    for family in ["oldest", "newest", "all", "duplicates"] {
        for fail in [false, true] {
            let s = source::Schema {
                family,
                work: 2,
                payload: 2,
                fail,
            };
            let rules = s.rules();
            let p = finite_phase::Prepared::new(&rules, 6).unwrap();
            let bridge = bridge::Bridge::new(rules.clone());
            for n in 0..=4 {
                for tag in [false, true] {
                    let q = s.query(n, tag);
                    let expected = runtime_support::run(&rules, &q, 2_000_000);
                    runtime_support::same_raw(expected.clone(), s.expected(n, tag));
                    let mut answers = vec![];
                    for solution in p.solve(&q, Default::default()).unwrap().solutions {
                        let (q, w) = bridge.transport(solution);
                        let mut caller = bridge.start(q, w);
                        let mut done = false;
                        for _ in 0..200_000 {
                            match caller.step() {
                                Event::Progress => (),
                                Event::Answer(a) => answers.push(a),
                                Event::Exhausted => {
                                    done = true;
                                    break;
                                }
                            }
                        }
                        assert!(done);
                    }
                    runtime_support::same_raw(answers, expected);
                    count += 1;
                }
            }
        }
    }
    assert_eq!(count, 80);
    println!("bridge_source_configurations={count}");
}

#[test]
fn bridge_transports_nontrivial_aliases_and_avoids_source_name_collisions() {
    use chr_syntax::{Query, Rule, Var, atom, c, eq, or, v};
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
            "watch",
            [c("watch", [atom("a"), v(0)]), c("token", [])],
            eq(v(0), atom("seen")),
        ),
        Rule::simplify(
            "names",
            [c("emit", [v(0)])],
            c("$finite-bind", [v(0), v(0)]).into(),
        ),
    ];
    let p = finite_phase::Prepared::new(&rules, 1).unwrap();
    let bridge = bridge::Bridge::new(rules.clone());
    let q = Query {
        constraints: vec![
            c("choice", [v(100)]),
            c("watch", [v(100), v(101)]),
            c("token", []),
            c("emit", [v(100)]),
        ],
        outputs: vec![
            ("x".into(), Var(100)),
            ("again".into(), Var(100)),
            ("y".into(), Var(101)),
        ],
    };
    let mut got = vec![];
    for row in p.solve(&q, Default::default()).unwrap().solutions {
        let (q, w) = bridge.transport(row);
        let mut e = bridge.start(q, w);
        let mut done = false;
        for _ in 0..10000 {
            match e.step() {
                Event::Progress => (),
                Event::Answer(a) => got.push(a),
                Event::Exhausted => {
                    done = true;
                    break;
                }
            }
        }
        assert!(done);
    }
    assert_eq!(got.len(), 3);
    runtime_support::same_raw(got, runtime_support::run(&rules, &q, 10000));
}
