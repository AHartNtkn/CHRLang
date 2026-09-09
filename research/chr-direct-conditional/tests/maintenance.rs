//! Source outcomes and candidate maintenance work, with no elapsed-time assertions.
use chr_direct_conditional::engine::{Event, PreparedRuleset};
use chr_syntax::{Answer, Query, Rule, Var, atom, c, eq, or, t, v};
fn run(rules: Vec<Rule>, query: Query) -> (Vec<Answer>, u64, u64, u64) {
    let mut engine = PreparedRuleset::new(rules).unwrap().start(query).unwrap();
    let mut answers = vec![];
    for _ in 0..1_000_000 {
        match engine.tick() {
            Event::Progress => (),
            Event::Answer(a) => answers.push(a),
            Event::Exhausted => {
                let s = engine.stats();
                return (
                    answers,
                    s.selected_candidates,
                    s.deferred_candidates,
                    s.discovered_tuples,
                );
            }
        }
    }
    panic!("finite maintenance witness did not terminate");
}
#[test]
fn countdown_candidate_selection_is_linear_in_both_source_orders() {
    let mut results = vec![];
    for reversed in [false, true] {
        for n in [16, 64] {
            let step = Rule::simplify(
                "step",
                [c("work", [t("s", [v(0)])])],
                c("work", [v(0)]).into(),
            );
            let base = Rule::simplify("base", [c("work", [atom("z")])], c("result", []).into());
            let rules = if reversed {
                vec![base, step]
            } else {
                vec![step, base]
            };
            let mut numeral = atom("z");
            for _ in 0..n {
                numeral = t("s", [numeral]);
            }
            let (answers, selected, deferred, discovered) = run(
                rules,
                Query {
                    constraints: vec![c("work", [numeral])],
                    outputs: vec![],
                },
            );
            assert_eq!(
                answers,
                vec![Answer {
                    outputs: vec![],
                    residual: vec![c("result", [])]
                }]
            );
            eprintln!(
                "n={n} reversed={reversed} selected={selected} deferred={deferred} discovered={discovered}"
            );
            results.push((n, selected, deferred, discovered));
        }
    }
    for (n, selected, deferred, discovered) in results {
        if cfg!(feature = "metrics") {
            // Selective discovery keeps only the matching immutable constructor.
            let per_occurrence = if cfg!(feature = "selective-discovery") {
                1
            } else {
                2
            };
            assert_eq!(discovered, per_occurrence * (n + 1));
            assert!(selected <= 4 * n + 8, "n={n}: selected={selected}");
            assert!(deferred <= 2 * n + 2, "n={n}: deferred={deferred}");
        } else {
            assert_eq!((selected, deferred, discovered), (0, 0, 0));
        }
    }
}
#[test]
fn partial_liveness_keeps_genuine_deferred_work_in_both_choice_regions() {
    let rules = vec![
        Rule::simplify(
            "choose",
            [c("start", [v(0)])],
            or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
        ),
        Rule::simplify(
            "first",
            [c("p", [atom("a")])],
            c("used", [atom("a")]).into(),
        ),
        Rule::simplify("independent", [c("q", [v(0)])], c("seen", [v(0)]).into()),
        Rule::simplify("remaining", [c("p", [v(0)])], c("used", [v(0)]).into()),
    ];
    let (mut actual, _, deferred, _) = run(
        rules,
        Query {
            constraints: vec![c("start", [v(0)]), c("p", [v(0)]), c("q", [v(0)])],
            outputs: vec![("x".into(), Var(0))],
        },
    );
    for answer in &mut actual {
        answer.residual.sort();
    }
    actual.sort();
    let mut expected = vec![];
    for name in ["a", "b"] {
        let mut residual = vec![c("used", [atom(name)]), c("seen", [atom(name)])];
        residual.sort();
        expected.push(Answer {
            outputs: vec![("x".into(), atom(name))],
            residual,
        });
    }
    expected.sort();
    assert_eq!(actual, expected);
    if cfg!(feature = "metrics") {
        // Three live tuples wait for choose; q's a region and p's b region
        // each wait once more. Consumed portions of partially live occurrences
        // must not create additional deferrals.
        assert!(
            (1..=5).contains(&deferred),
            "partial-support deferrals: {deferred}"
        );
    } else {
        assert_eq!(deferred, 0);
    }
}
