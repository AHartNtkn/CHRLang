#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod scalar;
#[allow(dead_code)]
#[path = "../../chr-direct-conditional/examples/support/post_choice_source.rs"]
mod source;
use chr_compiled::resource_count::Program;
use chr_syntax::{Goal, Rule, and, atom, c, or, v};
#[test]
fn insufficient_unknown_absent_and_mismatched_queries_are_rejected() {
    let p = Program::infer(&source::rules("common", 1)).unwrap();
    let q = source::query("common", 1, 4, 0);
    let mut shortage = q.clone();
    let i = shortage
        .constraints
        .iter()
        .position(|c| c.name == "fuel")
        .unwrap();
    shortage.constraints.remove(i);
    assert!(p.lower(&shortage).is_err());
    for depth in [v(999), atom("wrong")] {
        let mut x = q.clone();
        x.constraints
            .iter_mut()
            .find(|c| c.name == "start")
            .unwrap()
            .args[0] = depth;
        assert!(p.lower(&x).is_err());
    }
    let mut x = q.clone();
    x.constraints.retain(|c| c.name != "permit");
    assert!(p.lower(&x).is_err());
    let mut x = q.clone();
    x.constraints.retain(|c| c.name != "start");
    assert!(p.lower(&x).is_err());
    let mut x = q.clone();
    x.constraints.push(
        q.constraints
            .iter()
            .find(|c| c.name == "start")
            .unwrap()
            .clone(),
    );
    assert!(p.lower(&x).is_err());
    let mut x = q.clone();
    x.constraints.push(c("run", [atom("z"), v(999)]));
    assert!(p.lower(&x).is_err());
    let mut x = q.clone();
    x.constraints
        .iter_mut()
        .find(|c| c.name == "start")
        .unwrap()
        .args[2] = v(998);
    assert!(p.lower(&x).is_err());
}
#[test]
fn competing_effects_and_observable_depth_are_rejected_with_counterexample() {
    let base = source::rules("common", 1);
    for extra in [
        Rule::propagate("observe-fuel", [c("fuel", [])], c("seen", []).into()),
        Rule::simplify("take-permit", [c("permit", [])], Goal::True),
        Rule::simplify("observe-run", [c("run", [v(0), v(1)])], Goal::True),
        Rule::simplify("make-fuel", [c("make", [])], c("fuel", []).into()),
        Rule::simplify("compete", base[3].removed.clone(), Goal::True),
    ] {
        let mut r = base.clone();
        r.insert(0, extra);
        assert!(Program::infer(&r).is_err());
    }
    let mut r = base.clone();
    r[3].body = and([c("depth", [v(0)]).into(), r[3].body.clone()]);
    assert!(Program::infer(&r).is_err());
    let mut r = base.clone();
    r[3].body = or(r[3].body.clone(), Goal::True);
    assert!(Program::infer(&r).is_err());
    let mut r = base.clone();
    r[3].body = and([r[3].body.clone(), c("run", [v(0), v(1)]).into()]);
    assert!(Program::infer(&r).is_err());
    let mut observer = base.clone();
    observer.insert(
        0,
        Rule::propagate("observe-fuel", [c("fuel", [])], c("seen", []).into()),
    );
    let q = source::query("common", 1, 4, 0);
    let shortened = Program::infer(&base).unwrap().lower(&q).unwrap();
    let before = scalar::run(&observer, &q, 500000);
    let after = scalar::run(&observer, &shortened, 500000);
    assert_eq!(before.len(), 2);
    assert_eq!(after.len(), 2);
    assert!(
        before
            .iter()
            .all(|a| a.residual.iter().filter(|c| c.name == "seen").count() == 4)
    );
    assert!(
        after
            .iter()
            .all(|a| a.residual.iter().all(|c| c.name != "seen"))
    );
}
#[test]
fn branch_specific_shortage_preserves_suspended_residuals() {
    let r = source::rules("independent", 3);
    let mut q = source::query("independent", 3, 4, 0);
    let mut removed = 0;
    q.constraints.retain(|c| {
        if c.name == "fuel" && removed < 3 {
            removed += 1;
            false
        } else {
            true
        }
    });
    let shorter = Program::infer(&r).unwrap().lower(&q).unwrap();
    let expected = scalar::run(&r, &q, 500000);
    assert_eq!(expected.len(), 8);
    assert!(
        expected
            .iter()
            .any(|a| a.residual.iter().any(|c| c.name == "run"))
    );
    scalar::same_raw(scalar::run(&r, &shorter, 500000), expected);
}

#[test]
fn active_entry_effects_cannot_race_the_shortened_terminal() {
    use chr_syntax::t;
    let base = source::rules("common", 0);
    let shortened = Program::infer(&base)
        .unwrap()
        .lower(&source::query("common", 0, 4, 0))
        .unwrap();
    let mut entry = base[3].clone();
    entry.body = and([c("race", [v(1)]).into(), entry.body]);
    let race = Rule::simplify("race", [c("race", [v(0)])], c("observed", [v(0)]).into());
    // Shortening enables the constructor-specific observer before the generic
    // observer. Without shortening, the generic observer consumes the occurrence.
    let observe = Rule::simplify(
        "observe",
        [c("race", [t("done", [v(0)])])],
        c("observed-done", []).into(),
    );
    let r = vec![
        base[2].clone(),
        observe,
        race,
        base[1].clone(),
        base[0].clone(),
        entry,
    ];
    assert!(Program::infer(&r).is_err());
    let original = scalar::run(&r, &source::query("common", 0, 4, 0), 500000);
    let transformed = scalar::run(&r, &shortened, 500000);
    assert_eq!(original.len(), 1);
    assert_eq!(transformed.len(), 1);
    assert!(original[0].residual.iter().any(|c| c.name == "observed"));
    assert!(
        transformed[0]
            .residual
            .iter()
            .any(|c| c.name == "observed-done")
    );
}
