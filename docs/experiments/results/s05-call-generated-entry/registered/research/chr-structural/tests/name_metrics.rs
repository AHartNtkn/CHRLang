use chr_structural::name_disequality::{Formula, Name};
#[test]
fn diagnostic_counts_do_not_change_finite_feasibility() {
    let f = Formula::compile(
        3,
        &[],
        &[
            (Name::Variable(0), Name::Variable(1)),
            (Name::Variable(1), Name::Variable(2)),
            (Name::Variable(0), Name::Variable(2)),
        ],
    )
    .unwrap();
    for (alphabet, want) in [
        (vec!["a".into(), "b".into()], false),
        (vec!["a".into(), "b".into(), "c".into()], true),
    ] {
        let result = f.finite(&alphabet, &Default::default());
        assert_eq!(result.satisfiable, want);
        if cfg!(feature = "metrics") {
            assert!(result.assignment_attempts > 0);
        } else {
            assert_eq!(result.assignment_attempts, 0);
        }
    }
}
