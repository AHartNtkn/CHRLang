mod join_support;
#[allow(dead_code)]
mod search_support;
use chr_compiled::{Access, Policy, PreparedRuleset};
use chr_syntax::{Query, Rule};
fn check(source: Vec<Rule>, input: Query, n: usize, r: usize, expected_apps: usize) {
    for policy in [Policy::Global, Policy::Active] {
        for access in [Access::Scan, Access::Indexed] {
            let prepared = PreparedRuleset::new(source.clone(), None).unwrap();
            let mut engine = prepared.start(input.clone(), policy, access).unwrap();
            engine.enable_trace();
            let status = engine.advance(2_000_000);
            assert!(status.exhausted && !status.failed && !status.pending_split);
            let answer = engine.observe().unwrap();
            assert!(join_support::validate(&answer, n, r));
            let mut replay = search_support::Replay::new(&input, &[]);
            for (rule, ids) in engine.trace() {
                replay.fire(*rule, &source[*rule], ids).unwrap();
            }
            assert!(replay.terminal(&source));
            assert!(join_support::validate(&replay.answer(), n, r));
            if source[0].name == "replace" {
                let replacement_id = input.constraints.len() as u64;
                assert!(engine.trace().iter().any(|(rule, ids)| source[*rule].name == "join" && ids[1] == replacement_id), "the fresh replacement must participate in a join");
            }
            assert_eq!(engine.trace().len(), expected_apps);
            if chr_compiled::COLLECT_METRICS {
                assert_eq!(engine.stats().applications, expected_apps as u64);
            }
        }
    }
}
#[test]
fn full_receipts_aliases_cleanup_and_source_replay() {
    for (n, r) in [(1, 0), (1, 3), (3, 1), (3, 4), (8, 8)] {
        check(
            join_support::rules(),
            join_support::query(n, r),
            n,
            r,
            3 * r + n + 2,
        );
    }
}
#[test]
fn late_binding_wakes_waiting_request() {
    use chr_syntax::{c, eq, v};
    let mut source = join_support::rules();
    source.push(Rule::simplify(
        "bind",
        [c("bind", [v(0)])],
        eq(v(0), join_support::key(0)),
    ));
    let mut input = join_support::query(3, 4);
    input.constraints[0].args[0] = v(100);
    input.constraints[1].args[0] = v(100);
    input.constraints.push(c("bind", [v(100)]));
    check(source, input, 3, 4, 18);
}
#[test]
fn consumed_partner_is_replaced_with_fresh_occurrence() {
    use chr_syntax::{c, v};
    let mut source = join_support::rules();
    source.insert(
        0,
        Rule::simplify(
            "replace",
            [c("right", [v(0), v(1)]), c("replace", [v(0)])],
            c("right", [v(0), v(1)]).into(),
        ),
    );
    let mut input = join_support::query(3, 4);
    input.constraints.push(c("replace", [join_support::key(0)]));
    check(source, input, 3, 4, 18);
}
#[test]
fn oracle_rejects_alias_and_receipt_mutations() {
    use chr_syntax::{Answer, c, v};
    let answer = Answer {
        outputs: vec![],
        residual: vec![
            c(
                "receipt",
                [join_support::key(0), join_support::round(0), v(0), v(0)],
            ),
            c(
                "receipt",
                [join_support::key(2), join_support::round(1), v(1), v(1)],
            ),
            c("done", []),
        ],
    };
    assert!(join_support::validate(&answer, 3, 2));
    for mutation in 0..4 {
        let mut bad = answer.clone();
        match mutation {
            0 => bad.residual[0].args[3] = v(1),
            1 => {
                bad.residual[1].args[2] = v(0);
                bad.residual[1].args[3] = v(0);
            }
            2 => bad.residual[1] = bad.residual[0].clone(),
            _ => {
                bad.residual.pop();
            }
        }
        assert!(!join_support::validate(&bad, 3, 2));
    }
}
