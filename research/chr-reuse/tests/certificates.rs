use chr_reuse::failure::{Certificate, Stats, discover, verify};
use chr_syntax::{atom, t, v};
#[test]
fn structural_proofs_generalize_only_when_current_assumptions_hold() {
    let proof = discover(
        &t("p", [v(0), atom("a")]),
        &t("p", [v(1), atom("b")]),
        &mut Stats::default(),
    )
    .unwrap();
    assert_eq!(proof, Certificate::Clash { pair: vec![1] });
    assert!(verify(
        &proof,
        &t("p", [atom("z"), atom("c")]),
        &t("p", [atom("z"), atom("d")]),
        &mut Stats::default()
    ));
    assert!(!verify(
        &proof,
        &t("p", [v(0), atom("a")]),
        &t("p", [v(1), atom("a")]),
        &mut Stats::default()
    ));
    assert!(!verify(
        &proof,
        &v(0),
        &t("p", [v(1), atom("b")]),
        &mut Stats::default()
    ));
}
#[test]
fn occurs_proof_preserves_the_alias_and_rejects_empty_paths() {
    let a = v(0);
    let b = t("f", [t("g", [v(0)])]);
    let proof = discover(&a, &b, &mut Stats::default()).unwrap();
    assert!(verify(
        &proof,
        &v(40),
        &t("f", [t("g", [v(40)])]),
        &mut Stats::default()
    ));
    assert!(!verify(
        &proof,
        &v(40),
        &t("f", [t("g", [v(41)])]),
        &mut Stats::default()
    ));
    assert!(!verify(
        &Certificate::Occurs {
            pair: vec![],
            variable_on_left: true,
            occurrence: vec![]
        },
        &v(0),
        &v(0),
        &mut Stats::default()
    ));
}
#[test]
fn structural_gate_explicitly_does_not_claim_late_alias_proofs() {
    assert!(
        discover(
            &t("p", [v(0), v(1)]),
            &t("p", [v(1), t("f", [v(0)])]),
            &mut Stats::default()
        )
        .is_none()
    );
}

#[test]
fn all_small_path_certificates_are_sound_against_independent_execution() {
    use chr_syntax::{Query, Rule, Term, c, eq};
    let base = vec![v(0), v(1), atom("a"), atom("b")];
    let mut terms = base.clone();
    terms.extend(base.iter().map(|x| t("f", [x.clone()])));
    for a in &base {
        for b in &base {
            terms.push(t("p", [a.clone(), b.clone()]));
        }
    }
    let paths = vec![
        vec![],
        vec![0],
        vec![1],
        vec![0, 0],
        vec![0, 1],
        vec![1, 0],
        vec![1, 1],
    ];
    let mut proofs = vec![];
    for path in &paths {
        proofs.push(Certificate::Clash { pair: path.clone() });
        for occurrence in &paths[1..] {
            for variable_on_left in [true, false] {
                proofs.push(Certificate::Occurs {
                    pair: path.clone(),
                    variable_on_left,
                    occurrence: occurrence.clone(),
                });
            }
        }
    }
    assert_eq!(proofs.len(), 91);
    for a in &terms {
        for b in &terms {
            let mut reference = chr_reference::Search::new(
                vec![Rule::simplify(
                    "equation",
                    [c("run", [v(10), v(11)])],
                    eq(v(10), v(11)),
                )],
                Query {
                    constraints: vec![c("run", [a.clone(), b.clone()])],
                    outputs: vec![],
                },
            )
            .unwrap();
            let result = reference.advance(100);
            assert!(result.exhausted);
            let failed = result.answers.is_empty();
            for proof in &proofs {
                if verify(proof, a, b, &mut Stats::default()) {
                    assert!(failed, "{proof:?}: {a:?} = {b:?}");
                }
            }
            if let Some(proof) = discover(a, b, &mut Stats::default()) {
                assert!(failed);
                assert!(verify(&proof, a, b, &mut Stats::default()));
            }
        }
    }
    // A source hole is never treated as a rigid zero-arity constructor.
    assert!(!verify(
        &Certificate::Clash { pair: vec![] },
        &Term::Var(chr_syntax::Var(0)),
        &atom("a"),
        &mut Stats::default()
    ));
}
