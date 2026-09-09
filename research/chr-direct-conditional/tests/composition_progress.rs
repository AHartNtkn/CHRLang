mod composition_support;
mod runtime_support;
#[path = "../../chr-compiled/examples/support/resource_fusion_source.rs"]
mod source;
use chr_syntax::{Answer, Guard, Query, Rule, Var, and, atom, c, eq, or, t, v};
use composition_support::{Engine, Event};

#[test]
fn finite_mixed_sibling_and_owned_answer_survive_ongoing_work() {
    for reverse in [false, true] {
        for history_first in [false, true] {
            let finite = and([eq(v(0), atom("a")), c("take", [v(0), v(1)]).into()]);
            let ongoing = c("loop", []).into();
            let mut rules = vec![
                Rule::simplify(
                    "start",
                    [c("start", [v(0), v(1)])],
                    if reverse {
                        or(finite, ongoing)
                    } else {
                        or(ongoing, finite)
                    },
                ),
                Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
                Rule {
                    name: "take".into(),
                    kept: vec![c("permit", [])],
                    removed: vec![c("take", [v(0), v(1)]), c("ticket", [t("box", [v(3)])])],
                    guards: vec![Guard::Equal(v(0), v(3))],
                    body: and([
                        eq(v(1), t("done", [v(0), v(2)])),
                        c("fresh", [v(2), v(2)]).into(),
                    ]),
                },
            ];
            let history = Rule::propagate("history", [c("permit", [])], c("mark", []).into());
            if history_first {
                rules.insert(0, history);
            } else {
                rules.push(history);
            }
            let q = Query {
                constraints: vec![
                    c("start", [v(10), v(11)]),
                    c("permit", []),
                    c("ticket", [t("box", [atom("a")])]),
                ],
                outputs: vec![("x".into(), Var(10)), ("result".into(), Var(11))],
            };
            let expected = Answer {
                outputs: vec![
                    ("x".into(), atom("a")),
                    ("result".into(), t("done", [atom("a"), v(100)])),
                ],
                residual: vec![c("permit", []), c("mark", []), c("fresh", [v(100), v(100)])],
            };
            for mode in 0..6 {
                let mut e = Engine::new(mode, &rules, &q);
                let mut found = None;
                for _ in 0..200000 {
                    match e.step() {
                        Event::Answer(a) => {
                            found = Some(a);
                            break;
                        }
                        Event::Exhausted => panic!("lost ongoing work mode {mode}"),
                        Event::Progress => (),
                    }
                }
                let answer = found.unwrap_or_else(|| panic!("finite sibling starved mode {mode}"));
                runtime_support::same_raw(vec![answer.clone()], vec![expected.clone()]);
                for _ in 0..1024 {
                    assert!(
                        matches!(e.step(), Event::Progress),
                        "extra publication or false exhaustion mode {mode}"
                    );
                }
                drop(e);
                runtime_support::same_raw(vec![answer], vec![expected.clone()]);
            }
        }
    }
}

#[test]
fn certified_fusion_and_ineligible_queries_across_organizations() {
    let mut admitted = 0;
    let mut rejected = 0;
    let mut executions = 0;
    for family in ["plain", "choices", "duplicates", "shared", "spare"] {
        for n in 0..=3 {
            let rules = source::rules(family);
            let program = chr_compiled::resource_fusion::Program::infer(&rules).unwrap();
            for changed in 0..3 {
                let mut q = source::query(family, n, changed);
                if changed == 1 {
                    if let Some(i) = q.constraints.iter().position(|c| c.name == "permit") {
                        q.constraints.remove(i);
                    }
                } else if changed == 2 {
                    for c in &mut q.constraints {
                        if matches!(c.name.as_str(), "seed" | "fuel" | "permit") {
                            c.args[0] = v(99999);
                        }
                    }
                }
                let lowered = program.lower(&q);
                let eligible = changed == 0
                    || (changed == 1 && (n == 0 || family == "spare"))
                    || (changed == 2 && n == 0 && family != "spare");
                assert_eq!(
                    lowered.is_ok(),
                    eligible,
                    "eligibility {family} {n} {changed}"
                );
                let selected = match lowered {
                    Ok(r) => {
                        admitted += 1;
                        r
                    }
                    Err(_) => {
                        rejected += 1;
                        &rules
                    }
                };
                let expected = runtime_support::run(&rules, &q, 200000);
                for mode in 0..6 {
                    runtime_support::same_raw(
                        Engine::new(mode, &rules, &q).collect(),
                        expected.clone(),
                    );
                    runtime_support::same_raw(
                        Engine::new(mode, selected, &q).collect(),
                        expected.clone(),
                    );
                    executions += 2;
                }
            }
        }
    }
    println!(
        "certificates admitted={admitted} rejected={rejected}; candidate executions={executions}"
    );
}
