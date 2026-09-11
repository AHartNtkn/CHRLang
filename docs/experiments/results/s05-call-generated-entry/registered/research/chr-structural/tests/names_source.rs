use chr_syntax::{Answer, Query, Rule, Var, atom, c, eq, t, v};
#[test]
fn literal_name_occurrences_wake_after_body_bindings() {
    let mut count = 0;
    for (term, fails) in [
        (atom("x"), false),
        (atom("y"), false),
        (atom("app"), false),
        (t("other", [atom("x")]), false),
        (t("app", [atom("x"), atom("y")]), true),
        (t("lam", [atom("x"), atom("y")]), true),
    ] {
        for duplicate in [false, true] {
            for reverse in [false, true] {
                let bind = Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), term.clone()));
                let mut rules = chr_programs::lambda();
                if reverse {
                    rules.insert(0, bind);
                } else {
                    rules.push(bind);
                }
                let mut constraints = vec![c("var", [v(0)])];
                if duplicate {
                    constraints.push(c("var", [v(0)]));
                }
                constraints.push(c("bind", [v(0)]));
                let mut search = chr_reference::Search::new(
                    rules,
                    Query {
                        constraints,
                        outputs: vec![("out".into(), Var(0))],
                    },
                )
                .unwrap();
                let result = search.advance(1000);
                assert!(result.exhausted);
                if fails {
                    assert!(result.answers.is_empty());
                } else {
                    assert_eq!(
                        result.answers,
                        vec![Answer {
                            outputs: vec![("out".into(), term.clone())],
                            residual: vec![c("var", [term.clone()]); if duplicate { 2 } else { 1 }],
                        }]
                    );
                }
                count += 1;
            }
        }
    }
    assert_eq!(count, 24);
    println!("late_bound_source_queries={count}");
}
#[test]
fn alias_chain_keeps_the_future_name_restriction() {
    let mut rules = chr_programs::lambda();
    rules.push(Rule::simplify(
        "alias",
        [c("alias", [v(0), v(1)])],
        eq(v(0), v(1)),
    ));
    rules.push(Rule::simplify(
        "bind",
        [c("bind", [v(0)])],
        eq(v(0), t("app", [atom("x"), atom("y")])),
    ));
    let mut search = chr_reference::Search::new(
        rules,
        Query {
            constraints: vec![
                c("var", [v(0)]),
                c("alias", [v(0), v(1)]),
                c("bind", [v(1)]),
            ],
            outputs: vec![],
        },
    )
    .unwrap();
    let result = search.advance(1000);
    assert!(result.exhausted && result.answers.is_empty());
}
