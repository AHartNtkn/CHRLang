mod arena_cow_support;
#[allow(dead_code)]
#[path = "state_preservation_support/mod.rs"]
mod fixture;
#[allow(dead_code)]
mod search_support;
use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent};
use chr_syntax::{Answer, Query, Rule, Var, and, atom, c, eq, or, t, v};
fn collect(prepared: &PreparedRuleset, source: &[Rule], query: Query) -> (Vec<Answer>, usize) {
    let mut engine = prepared
        .start_search(query.clone(), Policy::Global, Access::Indexed)
        .unwrap();
    engine.enable_trace();
    let (mut answers, mut failures, mut exhausted) = (vec![], 0, false);
    for _ in 0..200_000 {
        match engine.tick() {
            SearchEvent::Complete(mut branch) | SearchEvent::Failed(mut branch) => {
                let mut replay = search_support::Replay::new(&query, &branch.lineage);
                let mut error = None;
                for (rule, ids) in branch.engine.trace() {
                    assert!(error.is_none());
                    error = replay.fire(*rule, &source[*rule], ids).err();
                }
                assert!(replay.choices_consumed());
                assert_eq!(error.is_some(), branch.engine.status().failed);
                if error.is_some() {
                    assert_eq!(error.as_deref(), Some("constructor clash"));
                    // The candidate rule posted its constructed term before failing.
                    if source.iter().any(|r| r.name == "construct-candidate") {
                        assert!(
                            branch
                                .engine
                                .view()
                                .store
                                .iter()
                                .any(|(_, c)| c.name == "stamp")
                        );
                    }
                    failures += 1;
                } else {
                    assert!(replay.terminal(source));
                    let answer = branch.engine.observe().unwrap();
                    assert!(chr_observe::equivalent(
                        &answer,
                        &replay.answer(),
                        &mut Default::default()
                    ));
                    answers.push(answer);
                }
            }
            SearchEvent::Exhausted => {
                exhausted = true;
                break;
            }
            SearchEvent::Split { .. } | SearchEvent::Progress => (),
        }
    }
    assert!(exhausted);
    (answers, failures)
}
#[test]
fn fired_constructor_insertion_then_failure_isolates_siblings_and_reuses_prepared() {
    let source = arena_cow_support::rules();
    let prepared = PreparedRuleset::new(source.clone(), None)
        .unwrap()
        .specialize_inferred();
    for n in [0, 3, 12] {
        for a in [1, 3, 5] {
            for all in [false, true] {
                let (answers, failed) =
                    collect(&prepared, &source, arena_cow_support::query(n, a, all));
                let mut keys = answers
                    .iter()
                    .map(|a| arena_cow_support::answer_key(a).expect("complete stamp aliases"))
                    .collect::<Vec<_>>();
                keys.sort_unstable();
                assert_eq!(keys, if all { (0..a).collect() } else { vec![0] });
                assert_eq!(failed, if all { 0 } else { a - 1 });
            }
        }
    }
}
#[test]
fn refork_after_insertion_keeps_new_node_ids_and_aliases_valid() {
    let source = vec![
        Rule::simplify(
            "start",
            [c("start", [v(0)])],
            or(c("make", [v(0)]).into(), eq(v(0), atom("other"))),
        ),
        Rule::simplify(
            "make",
            [c("make", [v(0)])],
            and(vec![
                eq(v(0), t("box", [atom("a")])),
                or(c("hit", [v(0)]).into(), c("hit", [v(0)]).into()),
            ]),
        ),
        Rule::simplify("hit", [c("hit", [v(0)])], eq(v(0), t("box", [atom("a")]))),
    ];
    let prepared = PreparedRuleset::new(source.clone(), None)
        .unwrap()
        .specialize_inferred();
    let query = Query {
        constraints: vec![c("start", [v(0)])],
        outputs: vec![
            ("x".into(), Var(0)),
            ("alias".into(), Var(0)),
            ("unused".into(), Var(1)),
        ],
    };
    let (answers, failed) = collect(&prepared, &source, query);
    assert_eq!(failed, 0);
    assert_eq!(answers.len(), 3);
    let mut boxed = 0;
    for a in answers {
        assert!(a.residual.is_empty());
        assert_eq!(a.outputs[0].1, a.outputs[1].1);
        assert!(matches!(a.outputs[2].1, chr_syntax::Term::Var(_)));
        if a.outputs[0].1 == t("box", [atom("a")]) {
            boxed += 1
        } else {
            assert_eq!(a.outputs[0].1, atom("other"))
        }
    }
    assert_eq!(boxed, 2);
}
#[test]
fn insertion_answer_oracle_rejects_missing_or_split_constructed_aliases() {
    let source = arena_cow_support::rules();
    let p = PreparedRuleset::new(source.clone(), None).unwrap();
    let (answers, _) = collect(&p, &source, arena_cow_support::query(1, 1, false));
    let answer = &answers[0];
    assert_eq!(arena_cow_support::answer_key(answer), Some(0));
    let mut missing = answer.clone();
    missing.residual.retain(|c| c.name != "stamp");
    assert!(arena_cow_support::answer_key(&missing).is_none());
    let mut split = answer.clone();
    let stamp = split
        .residual
        .iter_mut()
        .find(|c| c.name == "stamp")
        .unwrap();
    stamp.args[0] = t("new", [atom("z"), v(90), v(91)]);
    assert!(arena_cow_support::answer_key(&split).is_none());
}
