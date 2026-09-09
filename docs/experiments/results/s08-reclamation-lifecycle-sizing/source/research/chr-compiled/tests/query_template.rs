use chr_compiled::{
    Access, Execution, Policy, PreparedRuleset, SearchEngine, SearchEvent, search_bundled,
    search_fixtures,
};
use chr_syntax::{Answer, Query, Var, atom, c, v};

type Observation = (Answer, Vec<(usize, Vec<u64>)>);
fn answers(mut search: SearchEngine) -> Vec<Observation> {
    search.enable_trace();
    let mut answers = vec![];
    for _ in 0..100_000 {
        match search.tick() {
            SearchEvent::Complete(mut branch) => answers.push((
                branch.engine.observe().unwrap(),
                branch.engine.trace().to_vec(),
            )),
            SearchEvent::Exhausted => {
                answers.sort();
                return answers;
            }
            _ => (),
        }
    }
    panic!("finite search exceeded test bound");
}
#[test]
fn prepared_topology_preserves_changed_givens_raw_answers_and_early_drop() {
    for execution in [Execution::Generic, Execution::Generated] {
        let rules = search_fixtures::finite_rules(0);
        let code = matches!(execution, Execution::Generated)
            .then(|| search_bundled(search_fixtures::FINITE_START));
        let prepared = PreparedRuleset::new(rules, code).unwrap();
        let query = Query {
            constraints: vec![
                c("choose", [v(4)]),
                c("choose", [v(8)]),
                c("marker", [v(4), v(8)]),
                c("marker", [v(4), v(8)]),
            ],
            outputs: vec![("shown".into(), Var(4))],
        };
        let template = prepared
            .prepare_query(query.clone(), Policy::Active, Access::Indexed)
            .unwrap();
        assert_eq!(template.retention().occurrences, 4);
        assert_eq!(template.retention().pending, 0);
        assert_eq!(template.retention().history, 0);
        assert_eq!(template.stats().applications, 0);
        for extra in [
            vec![],
            vec![c("given", [v(4), atom("a1")])],
            vec![
                c("given", [v(4), atom("a0")]),
                c("given", [v(4), atom("a2")]),
            ],
            vec![
                c("given", [v(8), atom("a2")]),
                c("given", [v(8), atom("a2")]),
            ],
        ] {
            let mut fresh = query.clone();
            fresh.constraints.extend(extra.clone());
            let expected = answers(
                prepared
                    .start_search(fresh, Policy::Active, Access::Indexed)
                    .unwrap(),
            );
            let mut abandoned = template.start(extra.clone()).unwrap();
            for _ in 0..4 {
                let _ = abandoned.tick();
            }
            drop(abandoned);
            assert_eq!(answers(template.start(extra).unwrap()), expected);
        }
        // Neither assignment choices nor source simplification ran in preparation.
        assert_eq!(answers(template.start(vec![]).unwrap()).len(), 9);
        assert!(
            template
                .start(vec![c("given", [v(99), atom("a0")])])
                .is_err()
        );
        assert_eq!(answers(template.start(vec![]).unwrap()).len(), 9);
    }
}
