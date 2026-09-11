//! CHR-owned selection over stable source request tickets.
use chr_syntax::{Goal, Rule, and, c, v};

pub fn rules(renew_epoch: bool) -> Vec<Rule> {
    let mut rules = super::kernel::rules();
    rules.retain(|r| r.name != "take-f" && !r.name.starts_with("repair-take-"));
    for column in [1, 2] {
        let args = vec![v(0), v(1), v(2)];
        let mut updated = args.clone();
        updated[column] = v(3);
        rules.push(Rule {
            name: format!("repair-ticket-{column}"),
            kept: vec![c("edge", [args[column].clone(), v(3)])],
            removed: vec![c("request", args)],
            guards: vec![],
            body: c("request", updated).into(),
        });
    }
    rules.push(Rule::propagate(
        "discover-eligible",
        [
            c("epoch", []),
            c("request", [v(0), v(1), v(2)]),
            c("d_f", [v(1), v(3)]),
            c("token", []),
        ],
        c("eligible", [v(0), v(1), v(2), v(3)]).into(),
    ));
    rules.push(Rule {
        name: "dedup-eligible".into(),
        kept: vec![c("eligible", [v(0), v(1), v(2), v(3)])],
        removed: vec![c("eligible", [v(0), v(1), v(2), v(3)])],
        guards: vec![],
        body: Goal::True,
    });
    rules.push(Rule {
        name: "prefer-earlier".into(),
        kept: vec![
            c("before", [v(0), v(4)]),
            c("eligible", [v(0), v(1), v(2), v(3)]),
        ],
        removed: vec![c("eligible", [v(4), v(5), v(6), v(7)])],
        guards: vec![],
        body: Goal::True,
    });
    let mut commit = Rule::simplify(
        "commit-earliest",
        [
            c("eligible", [v(0), v(1), v(2), v(3)]),
            c("request", [v(0), v(1), v(2)]),
            c("token", []),
        ],
        c("union", [v(2), v(3)]).into(),
    );
    if renew_epoch {
        commit.removed.push(c("epoch", []));
        commit.body = and(vec![commit.body, c("epoch", []).into()]);
    }
    rules.push(commit);
    rules
}

/// Reserve one token before discovering this round's candidates. The opening
/// witness guarantees an eligible request; reservation is committed only by
/// commit-earliest. No source operation can interleave with this protocol here.
pub fn reserved_rules() -> Vec<Rule> {
    let mut result = rules(true);
    let discovery = result
        .iter()
        .position(|r| r.name == "discover-eligible")
        .unwrap();
    let rule = &mut result[discovery];
    rule.kept.retain(|head| head.name != "token");
    rule.kept
        .iter_mut()
        .find(|head| head.name == "epoch")
        .unwrap()
        .name = "round".into();
    let commit = result
        .iter_mut()
        .find(|r| r.name == "commit-earliest")
        .unwrap();
    commit.removed.retain(|head| head.name != "token");
    commit
        .removed
        .iter_mut()
        .find(|head| head.name == "epoch")
        .unwrap()
        .name = "round".into();
    result.insert(
        discovery,
        Rule {
            name: "open-round".into(),
            kept: vec![c("request", [v(0), v(1), v(2)]), c("d_f", [v(1), v(3)])],
            removed: vec![c("epoch", []), c("token", [])],
            guards: vec![],
            body: c("round", []).into(),
        },
    );
    result
}
