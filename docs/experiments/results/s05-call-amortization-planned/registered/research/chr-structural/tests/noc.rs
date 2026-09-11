use chr_structural::{Datum, Mode, NoC, no_c_rules};
fn n(name: &str) -> Datum {
    Datum::app(name, [])
}
#[test]
fn partial_residuals_keep_multiplicity_and_unknown_constructor_boundaries() {
    let hole = Datum::var(0);
    let input = Datum::app("a", [hole.clone(), hole]);
    for mode in [Mode::Direct, Mode::Memo] {
        let mut solver = NoC::new(&no_c_rules(), mode).unwrap();
        let result = solver.reduce(std::slice::from_ref(&input)).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].export(), chr_syntax::v(0));
        assert_eq!(result[1].export(), chr_syntax::v(0));
        let unknown = Datum::app("unknown", [Datum::app("c", [n("z")])]);
        let result = solver.reduce(std::slice::from_ref(&unknown)).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].export(), unknown.export());
        assert!(solver.reduce(&[Datum::app("c", [n("z")])]).is_none());
    }
}
#[test]
fn exact_identity_memoization_reuses_ground_proof_work() {
    let mut input = n("k");
    for _ in 0..8 {
        input = Datum::app("a", [input.clone(), input]);
    }
    let mut direct = NoC::new(&no_c_rules(), Mode::Direct).unwrap();
    let mut memo = NoC::new(&no_c_rules(), Mode::Memo).unwrap();
    assert!(
        direct
            .reduce(std::slice::from_ref(&input))
            .unwrap()
            .is_empty()
    );
    assert!(memo.reduce(&[input]).unwrap().is_empty());
    assert_eq!(direct.stats().computed, 511);
    assert_eq!(memo.stats().computed, 9);
}
#[test]
fn external_observers_invalidate_the_closed_operation() {
    let mut rules = no_c_rules();
    rules.insert(
        0,
        chr_syntax::Rule::propagate(
            "observe",
            [chr_syntax::c("no_c", [chr_syntax::atom("k")])],
            chr_syntax::c("seen", []).into(),
        ),
    );
    assert!(NoC::new(&rules, Mode::Memo).is_err());
    let mut reference = chr_reference::Search::new(
        rules,
        chr_syntax::Query {
            constraints: vec![chr_syntax::c("no_c", [chr_syntax::atom("k")])],
            outputs: vec![],
        },
    )
    .unwrap();
    let b = reference.advance(100);
    assert!(b.exhausted);
    assert_eq!(b.answers[0].residual, vec![chr_syntax::c("seen", [])]);
}

#[test]
fn bounded_term_denotations_match_the_independent_reference() {
    let leaves = vec![
        n("k"),
        n("s"),
        Datum::app("c", [n("z")]),
        n("z"),
        Datum::app("unknown", [Datum::app("c", [n("z")])]),
        Datum::var(0),
        Datum::app("k", [n("k")]),
    ];
    let mut shallow = leaves.clone();
    for a in &leaves {
        for b in &leaves {
            shallow.push(Datum::app("a", [a.clone(), b.clone()]));
        }
    }
    let mut inputs = shallow.clone();
    for a in &shallow {
        for b in &shallow {
            inputs.push(Datum::app("a", [a.clone(), b.clone()]));
        }
    }
    let mut direct = NoC::new(&no_c_rules(), Mode::Direct).unwrap();
    let mut memo = NoC::new(&no_c_rules(), Mode::Memo).unwrap();
    for input in inputs {
        let mut reference = chr_reference::Search::new(
            no_c_rules(),
            chr_syntax::Query {
                constraints: vec![chr_syntax::c("no_c", [input.export()])],
                outputs: vec![("out0".into(), chr_syntax::Var(0))],
            },
        )
        .unwrap();
        let expected = reference.advance(1000);
        assert!(expected.exhausted);
        for solver in [&mut direct, &mut memo] {
            match solver.reduce(std::slice::from_ref(&input)) {
                None => assert!(expected.answers.is_empty()),
                Some(residual) => {
                    assert_eq!(expected.answers.len(), 1);
                    let answer = chr_syntax::Answer {
                        outputs: vec![("out0".into(), chr_syntax::v(0))],
                        residual: residual
                            .iter()
                            .map(|t| chr_syntax::c("no_c", [t.export()]))
                            .collect(),
                    };
                    assert!(chr_observe::equivalent(
                        &answer,
                        &expected.answers[0],
                        &mut Default::default()
                    ));
                }
            }
        }
    }
}
