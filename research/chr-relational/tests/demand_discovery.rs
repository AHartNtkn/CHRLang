use chr_relational::contextual::Store;
use chr_syntax::{c, v};
#[test]
fn demand_selection_follows_order_and_continues_after_rejection() {
    let mut s = Store::default();
    let key = s.unknown();
    for _ in 0..3 {
        s.post("seed", &[key]);
        s.post("fuel", &[key]);
    }
    for _ in 0..6 {
        s.post("permit", &[key]);
    }
    let heads = vec![
        c("seed", [v(0)]),
        c("fuel", [v(0)]),
        c("permit", [v(0)]),
        c("permit", [v(0)]),
    ];
    let all = s.matches(&[], &heads);
    assert_eq!(all.len(), 270);
    for skip in [0, 1, 269, 270] {
        let mut visited = 0;
        let found = s.find_match(&[], &heads, |_| {
            let accept = visited == skip;
            visited += 1;
            accept
        });
        assert_eq!(found, all.get(skip).cloned());
        assert_eq!(visited, (skip + 1).min(270));
    }
}

#[test]
fn partial_constructor_environments_keep_tuple_then_binding_order() {
    use chr_syntax::t;
    let mut s = Store::default();
    let x = s.unknown();
    let y = s.unknown();
    let fx = s.constructor("f", &[x]);
    let fy = s.constructor("f", &[y]);
    s.post("open", &[fx]);
    s.post("open", &[fy]);
    s.post("other", &[x]);
    s.post("other", &[y]);
    s.equate(fx, fy);
    assert!(s.step());
    assert!(s.pending() > 0);
    let heads = [c("open", [t("f", [v(0)])]), c("other", [v(1)])];
    for settled in [false, true] {
        if settled {
            while s.step() {}
        }
        let all = s.matches(&heads[..1], &heads[1..]);
        assert!(!all.is_empty());
        for skip in 0..=all.len() {
            let mut seen = 0;
            let found = s.find_match(&heads[..1], &heads[1..], |_| {
                let yes = seen == skip;
                seen += 1;
                yes
            });
            assert_eq!(found, all.get(skip).cloned());
        }
    }
}

#[test]
fn history_rejection_exposes_recomputation_cost() {
    let mut s = Store::default();
    let x = s.unknown();
    for _ in 0..16 {
        s.post("item", &[x]);
    }
    let heads = [c("item", [v(0)])];
    let eager = s.matches(&heads, &[]);
    let mut history = std::collections::BTreeSet::new();
    let mut offered = 0;
    while let Some(m) = s.find_match(&heads, &[], |m| {
        offered += 1;
        !history.contains(&m.kept)
    }) {
        history.insert(m.kept);
    }
    assert_eq!(history.len(), eager.len());
    assert_eq!(offered, 152);
}

#[test]
fn source_history_and_late_guard_activation_match_independent_answers() {
    use chr_relational::contextual_execute::{Prepared, Step};
    use chr_syntax::{Answer, Guard, Query, Rule, Var, atom, eq};
    for n in [0, 1, 16] {
        let rules = vec![
            Rule::propagate("history", [c("item", [v(0)])], c("mark", [v(0)]).into()),
            Rule {
                name: "guarded".into(),
                kept: vec![],
                removed: vec![c("wait", [v(0), v(1)])],
                guards: vec![Guard::Equal(v(0), v(1))],
                body: c("done", []).into(),
            },
            Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
        ];
        let mut constraints = vec![c("wait", [v(10), atom("a")]), c("bind", [v(10)])];
        let mut residual = vec![c("done", [])];
        for i in 0..n {
            let a = atom(&format!("i{i}"));
            constraints.push(c("item", [a.clone()]));
            residual.push(c("item", [a.clone()]));
            residual.push(c("mark", [a]));
        }
        let q = Query {
            constraints,
            outputs: vec![("x".into(), Var(10))],
        };
        let expected = Answer {
            outputs: vec![("x".into(), atom("a"))],
            residual,
        };
        let p = Prepared::new(&rules).unwrap();
        for mut e in [p.start(&q), p.start_demand(&q)] {
            let mut answers = vec![];
            let mut exhausted = false;
            for _ in 0..10000 {
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
            assert_eq!(answers.len(), 1);
            assert!(chr_observe::equivalent(
                &answers[0],
                &expected,
                &mut Default::default()
            ));
        }
    }
}
