#[allow(dead_code)]
mod runtime_support;
use chr_direct_conditional::engine::{Event, PreparedRuleset};
use chr_syntax::{Answer, Goal, Query, Rule, Term, and, atom, c, eq, or, t, v};

#[derive(Clone, Copy, Debug)]
enum Alphabet {
    Binary,
    Duplicate,
    Nested,
}
fn source(alphabet: Alphabet, reject: bool) -> Vec<Rule> {
    let assign = |name: &str| eq(v(1), t("cons", [atom(name), v(2)]));
    let choice = match alphabet {
        Alphabet::Binary => or(assign("a"), assign("b")),
        Alphabet::Duplicate => or(assign("a"), assign("a")),
        Alphabet::Nested => or(assign("a"), or(assign("b"), assign("c"))),
    };
    let mut rules = vec![
        Rule::propagate("watch", [c("token", [v(0)])], c("seen", [v(0)]).into()),
        Rule::simplify(
            "base",
            [c("build", [atom("z"), v(0)])],
            eq(v(0), atom("nil")),
        ),
        Rule::simplify(
            "step",
            [c("build", [t("s", [v(0)]), v(1)])],
            and([choice, c("build", [v(0), v(2)]).into()]),
        ),
        Rule::simplify(
            "use",
            [c("task", [v(0), v(1)]), c("token", [v(2)])],
            c("out", [v(0), v(0), v(1), v(1), v(2), v(2)]).into(),
        ),
    ];
    if reject {
        rules.push(Rule::simplify(
            "reject",
            [c(
                "out",
                [t("cons", [atom("b"), v(0)]), v(1), v(2), v(3), v(4), v(5)],
            )],
            Goal::Fail,
        ));
    }
    rules
}
fn query(depth: usize, two: bool, token_first: bool) -> Query {
    let n = (0..depth).fold(atom("z"), |x, _| t("s", [x]));
    let mut constraints = vec![
        c("build", [n.clone(), v(100)]),
        c("task", [v(100), v(if two { 101 } else { 100 })]),
    ];
    if two {
        constraints.push(c("build", [n, v(101)]));
    }
    if token_first {
        constraints.insert(0, c("token", [v(200)]));
    } else {
        constraints.push(c("token", [v(200)]));
    }
    Query {
        constraints,
        outputs: vec![],
    }
}
fn words(depth: usize, alphabet: Alphabet) -> Vec<Term> {
    if depth == 0 {
        return vec![atom("nil")];
    }
    let letters: &[&str] = match alphabet {
        Alphabet::Binary => &["a", "b"],
        Alphabet::Duplicate => &["a", "a"],
        Alphabet::Nested => &["a", "b", "c"],
    };
    let tails = words(depth - 1, alphabet);
    letters
        .iter()
        .flat_map(|letter| {
            tails
                .iter()
                .map(move |tail| t("cons", [atom(letter), tail.clone()]))
        })
        .collect()
}
fn expected(depth: usize, two: bool, alphabet: Alphabet, reject: bool) -> Vec<Answer> {
    let words = words(depth, alphabet);
    let mut result = vec![];
    for x in &words {
        if reject && matches!(x,Term::App(n,xs) if n=="cons" && xs[0]==atom("b")) {
            continue;
        }
        let ys = if two { words.clone() } else { vec![x.clone()] };
        for y in ys {
            result.push(Answer {
                outputs: vec![],
                residual: vec![
                    c("seen", [v(900)]),
                    c("out", [x.clone(), x.clone(), y.clone(), y, v(900), v(900)]),
                ],
            });
        }
    }
    result
}
fn run(prepared: &PreparedRuleset, input: Query) -> Vec<Answer> {
    let mut engine = prepared.start(input).unwrap();
    let mut result = vec![];
    for _ in 0..2_000_000 {
        match engine.tick() {
            Event::Progress => {}
            Event::Answer(a) => result.push(a),
            Event::Exhausted => return result,
        }
    }
    panic!("registered Conditional gate cutoff")
}
#[test]
fn dynamic_births_resources_and_complete_observations() {
    for (alphabet, reject, cases) in [
        (
            Alphabet::Binary,
            false,
            vec![
                (0, false),
                (1, false),
                (2, false),
                (3, false),
                (1, true),
                (2, true),
            ],
        ),
        (
            Alphabet::Nested,
            false,
            vec![(1, false), (2, false), (2, true)],
        ),
        (
            Alphabet::Duplicate,
            false,
            vec![(1, false), (2, false), (3, false)],
        ),
        (Alphabet::Binary, true, vec![(1, false), (2, false)]),
    ] {
        let rules = source(alphabet, reject);
        let prepared = PreparedRuleset::new(rules.clone()).unwrap();
        for (depth, two) in cases {
            for first in [false, true] {
                let input = query(depth, two, first);
                let oracle = expected(depth, two, alphabet, reject);
                runtime_support::same_raw(
                    runtime_support::run(&rules, &input, 200_000),
                    oracle.clone(),
                );
                let actual = run(&prepared, input);
                let raw = actual.len();
                runtime_support::same_raw(actual, oracle);
                println!(
                    "alphabet={alphabet:?} reject={reject} depth={depth} two={two} token_first={first} raw={raw}"
                );
            }
        }
    }
}
#[test]
fn oracle_preserves_correlation_aliases_and_raw_multiplicity() {
    let single = expected(2, false, Alphabet::Binary, false);
    let independent = expected(2, true, Alphabet::Binary, false);
    assert_eq!(single.len(), 4);
    assert_eq!(independent.len(), 16);
    assert_eq!(expected(2, false, Alphabet::Nested, false).len(), 9);
    assert_eq!(expected(2, false, Alphabet::Duplicate, false).len(), 4);
    let good = single[0].clone();
    let mut missing = good.clone();
    missing.residual.remove(0);
    assert!(!chr_observe::equivalent(
        &good,
        &missing,
        &mut Default::default()
    ));
    let mut broken = good.clone();
    broken.residual[1].args[5] = v(901);
    assert!(!chr_observe::equivalent(
        &good,
        &broken,
        &mut Default::default()
    ));
    let failed_word = expected(1, false, Alphabet::Binary, false).pop().unwrap();
    assert!(
        expected(1, false, Alphabet::Binary, true)
            .iter()
            .all(|a| !chr_observe::equivalent(a, &failed_word, &mut Default::default()))
    );
}
#[test]
fn finite_sibling_is_observed_without_claiming_global_completion() {
    let rules = vec![
        Rule::simplify(
            "start",
            [c("start", [])],
            or(c("loop", []).into(), c("finite", [v(0), v(0)]).into()),
        ),
        Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
    ];
    let mut engine = PreparedRuleset::new(rules)
        .unwrap()
        .start(Query {
            constraints: vec![c("start", [])],
            outputs: vec![],
        })
        .unwrap();
    let expected = Answer {
        outputs: vec![],
        residual: vec![c("finite", [v(900), v(900)])],
    };
    let mut answers = 0;
    for _ in 0..20_000 {
        match engine.tick() {
            Event::Progress => {}
            Event::Answer(a) => {
                assert!(chr_observe::equivalent(
                    &a,
                    &expected,
                    &mut Default::default()
                ));
                answers += 1;
            }
            Event::Exhausted => panic!("recursive sibling cannot exhaust"),
        }
    }
    assert_eq!(answers, 1);
}
