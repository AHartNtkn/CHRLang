use super::{decode, local, node, oracle, run, selection};
use chr_syntax::{Answer, Query, Rule, Var, atom, c, eq, t, v};
fn source() -> Vec<Rule> {
    vec![Rule::simplify(
        "take",
        [c("take", [t("f", [v(0)]), v(1)]), c("token", [])],
        eq(v(1), v(0)),
    )]
}
fn check_local(query: &Query, expected: &[Answer]) {
    let plan = local::Plan::compile(&source()[0]).unwrap();
    let mut run = plan.start_with_metrics::<false>(query, local::DependencyMode::Endpoint);
    let mut finished = false;
    for _ in 0..200_000 {
        if run.advance(1) {
            finished = true;
            break;
        }
    }
    assert!(finished, "unfinished local control");
    oracle::same_raw(run.observe().into_iter().collect(), expected.to_vec());
}
fn decoded(mut answer: Answer, n: usize) -> Answer {
    assert_eq!(
        answer.residual.iter().filter(|c| c.name == "epoch").count(),
        1
    );
    answer
        .residual
        .retain(|c| c.name != "epoch" && c.name != "before");
    for fact in &mut answer.residual {
        if fact.name == "request" {
            fact.name = "take".into();
            fact.args.remove(0);
        }
    }
    let mut answer = decode(answer, n);
    answer.outputs = (0..3).map(|i| answer.outputs[2 * i + 1].clone()).collect();
    answer
}
fn query(
    order: &[usize; 3],
    reverse: bool,
    repair: bool,
    ready: usize,
    tokens: usize,
    chain: bool,
) -> (Query, Query) {
    let mut ordinary = order
        .iter()
        .map(|&i| {
            c(
                "take",
                [
                    if ready & (1 << i) != 0 {
                        t("f", [atom(if i % 2 == 0 { "a" } else { "b" })])
                    } else {
                        v((2 * i) as u64)
                    },
                    v((2 * i + 1) as u64),
                ],
            )
        })
        .collect::<Vec<_>>();
    // In the chain case request 0's result enables the older suspended request 1.
    if chain {
        for fact in &mut ordinary {
            if fact.args[1] == v(1) {
                fact.args[0] = t("f", [t("f", [atom("a")])]);
            }
            if fact.args[1] == v(3) {
                fact.args[0] = v(1);
            }
        }
    }
    ordinary.extend((0..tokens).map(|_| c("token", [])));
    let ordinary = Query {
        constraints: ordinary,
        outputs: (0..3)
            .map(|i| (format!("v{}", 2 * i + 1), Var((2 * i + 1) as u64)))
            .collect(),
    };
    let mut facts = (0..11).map(|i| c("root", [node(i)])).collect::<Vec<_>>();
    facts.extend([c("epoch", []), c("d_a", [node(6)]), c("d_b", [node(7)])]);
    let descriptors = if reverse { [2, 1, 0] } else { [0, 1, 2] };
    for i in descriptors {
        if ready & (1 << i) != 0 || (chain && i == 0) {
            facts.push(c(
                "d_f",
                [
                    node(2 * i),
                    node(if chain && i == 0 { 9 } else { 6 + i % 2 }),
                ],
            ));
        }
    }
    if chain {
        facts.extend([c("d_f", [node(9), node(6)]), c("union", [node(2), node(1)])]);
    }
    for (ticket, &i) in order.iter().enumerate() {
        facts.push(c(
            "request",
            [atom(&format!("q{ticket}")), node(2 * i), node(2 * i + 1)],
        ));
        for earlier in 0..ticket {
            facts.push(c(
                "before",
                [atom(&format!("q{earlier}")), atom(&format!("q{ticket}"))],
            ));
        }
    }
    if repair {
        facts.push(c("union", [node(2 * order[0]), node(8)]));
    }
    facts.extend((0..tokens).map(|_| c("token", [])));
    (
        ordinary,
        Query {
            constraints: facts,
            outputs: vec![],
        },
    )
}
#[test]
fn stable_tickets_select_eligible_requests_after_repair() {
    let rules = selection::rules(true);
    let mut count = 0;
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        for reverse in [false, true] {
            for repair in [false, true] {
                for ready in [0, 1, 3, 7] {
                    for tokens in 0..=3 {
                        let (ordinary, encoded) =
                            query(&order, reverse, repair, ready, tokens, false);
                        let expected = oracle::run(&source(), &ordinary, 200_000);
                        check_local(&ordinary, &expected);
                        let scalar = oracle::run(&rules, &encoded, 200_000)
                            .into_iter()
                            .map(|a| decoded(a, 11))
                            .collect::<Vec<_>>();
                        oracle::same_raw(scalar, expected.clone());
                        for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                            let actual = run(rules.clone(), encoded.clone(), access)
                                .into_iter()
                                .map(|a| decoded(a, 11))
                                .collect();
                            oracle::same_raw(actual, expected.clone());
                            count += 1;
                            println!(
                                "STABLE_CASE,order={order:?},reverse={reverse},repair={repair},ready={ready},tokens={tokens},access={access:?}"
                            );
                        }
                    }
                }
            }
        }
    }
    assert_eq!(count, 768);
    println!("STABLE_SELECTION,matrix={count}");
}
#[test]
fn renewed_selection_services_newly_ready_older_request() {
    for order in [[1, 0, 2], [2, 1, 0]] {
        for tokens in 0..=3 {
            let (ordinary, encoded) = query(&order, true, true, 5, tokens, true);
            let expected = oracle::run(&source(), &ordinary, 200_000);
            check_local(&ordinary, &expected);
            oracle::same_raw(
                oracle::run(&selection::rules(true), &encoded, 200_000)
                    .into_iter()
                    .map(|a| decoded(a, 11))
                    .collect(),
                expected.clone(),
            );
            for access in [chr_compiled::Access::Scan, chr_compiled::Access::Indexed] {
                println!("STABLE_CHAIN,order={order:?},tokens={tokens},access={access:?}");
                oracle::same_raw(
                    run(selection::rules(true), encoded.clone(), access)
                        .into_iter()
                        .map(|a| decoded(a, 11))
                        .collect(),
                    expected.clone(),
                );
            }
        }
    }
    let (ordinary, encoded) = query(&[0, 1, 2], false, false, 7, 2, false);
    let expected = oracle::run(&source(), &ordinary, 200_000);
    check_local(&ordinary, &expected);
    let mutation = oracle::run(&selection::rules(false), &encoded, 200_000);
    assert_eq!(mutation.len(), 1);
    let actual = decoded(mutation[0].clone(), 11);
    assert!(!chr_observe::equivalent(
        &actual,
        &expected[0],
        &mut Default::default()
    ));
    assert!(actual.residual.iter().any(|c| c.name == "token"));
}
