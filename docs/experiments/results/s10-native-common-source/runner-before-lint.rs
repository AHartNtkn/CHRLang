use chr_compiled::{Access, Policy, PreparedRuleset};
#[path = "../tests/composition_support/mod.rs"]
#[allow(dead_code)]
mod engines;
use chr_syntax::{Constraint, Goal, Query, Rule, Term, Var, and, atom, c, eq, or, v};
use engines::{Engine, Event};
#[allow(dead_code)]
#[path = "support/finite_bridge.rs"]
mod finite_bridge;
#[allow(dead_code)]
#[path = "../../chr-compiled/experiments/finite_phase.rs"]
mod finite_phase;
use std::io::{self, Read};
fn term(s: &str) -> Term {
    if let Some(n) = s.strip_prefix('v') {
        v(n.parse().unwrap())
    } else {
        atom(s)
    }
}
fn constraint(s: &str) -> Constraint {
    let mut parts = s.split(':');
    c(parts.next().unwrap(), parts.map(term).collect::<Vec<_>>())
}
fn constraints(s: &str) -> Vec<Constraint> {
    if s == "-" {
        vec![]
    } else {
        s.split(',').map(constraint).collect()
    }
}
fn show(t: &Term) -> String {
    match t {
        Term::Var(Var(n)) => format!("v{n}"),
        Term::App(name, args) => {
            assert!(args.is_empty());
            name.clone()
        }
    }
}
fn split_top(s: &str, separator: char) -> Vec<&str> {
    let mut depth = 0;
    let mut start = 0;
    let mut parts = vec![];
    for (i, ch) in s.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if ch == separator && depth == 0 {
            parts.push(&s[start..i]);
            start = i + 1;
        }
    }
    assert_eq!(depth, 0);
    parts.push(&s[start..]);
    parts
}
fn goal(s: &str) -> Goal {
    if s == "-" {
        return and(Vec::<Goal>::new());
    }
    if s == "!" {
        return Goal::Fail;
    }
    let parts = split_top(s, ',');
    if parts.len() > 1 {
        return and(parts.into_iter().map(goal).collect::<Vec<_>>());
    }
    if s.starts_with('(') {
        assert!(s.ends_with(')'));
        let arms = split_top(&s[1..s.len() - 1], '|');
        assert_eq!(arms.len(), 2);
        return or(goal(arms[0]), goal(arms[1]));
    }
    if let Some(rest) = s.strip_prefix("=:") {
        let ts: Vec<_> = rest.split(':').collect();
        assert_eq!(ts.len(), 2);
        eq(term(ts[0]), term(ts[1]))
    } else {
        constraint(s).into()
    }
}
fn main() {
    let mode: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut text = String::new();
    io::stdin().read_to_string(&mut text).unwrap();
    let mut sections = text.split("\nNEXT\n");
    let mut lines = sections.next().unwrap().lines();
    let first_constraints = constraints(lines.next().unwrap());
    let first_outputs = outputs(lines.next().unwrap());
    let rules: Vec<Rule> = lines
        .enumerate()
        .map(|(i, line)| {
            let parts: Vec<_> = line.split(';').collect();
            assert_eq!(parts.len(), 3);
            Rule {
                name: format!("r{i}"),
                kept: self::constraints(parts[0]),
                removed: self::constraints(parts[1]),
                guards: vec![],
                body: goal(parts[2]),
            }
        })
        .collect();

    let mut queries = vec![Query {
        constraints: first_constraints,
        outputs: first_outputs,
    }];
    for section in sections {
        let mut lines = section.lines();
        queries.push(Query {
            constraints: constraints(lines.next().unwrap()),
            outputs: outputs(lines.next().unwrap()),
        });
        assert!(lines.next().is_none());
    }
    let compiled = if mode < 4 {
        let p = PreparedRuleset::new(rules.clone(), None).unwrap();
        Some(if mode >= 2 {
            p.specialize_inferred()
        } else {
            p
        })
    } else {
        None
    };
    let contextual = if (4..=8).contains(&mode) {
        Some(chr_relational::contextual_execute::Prepared::new(&rules).unwrap())
    } else {
        None
    };
    let conditional = if mode == 9 {
        Some(chr_direct_conditional::engine::PreparedRuleset::new(rules.clone()).unwrap())
    } else {
        None
    };
    if mode == 99 {
        for prefix in 1..=rules.len() {
            match finite_phase::Prepared::new(&rules, prefix) {
                Ok(_) => println!("PREFIX {prefix} admitted"),
                Err(e) => println!("PREFIX {prefix} {e:?}"),
            }
        }
        return;
    }
    let prefix = if mode == 10 {
        Some(match chr_compiled::pure_prefix::Program::new(&rules) {
            Ok(p) => p,
            Err(e) => {
                println!("UNSUPPORTED {e}");
                return;
            }
        })
    } else {
        None
    };
    let finite = if mode == 11 {
        let admitted = (1..=rules.len())
            .rev()
            .find_map(|n| finite_phase::Prepared::new(&rules, n).ok().map(|p| (n, p)));
        match admitted {
            Some((n, p)) => Some((p, finite_bridge::Bridge::new(rules[n..].to_vec()))),
            None => {
                println!("UNSUPPORTED no admitted finite phase");
                return;
            }
        }
    } else {
        None
    };
    let mut artifacts = std::collections::BTreeMap::new();
    if let Some(p) = &prefix {
        eprintln!("eliminated_predicates={}", p.eliminated_predicates().len());
    }
    for (index, query) in queries.into_iter().enumerate() {
        if let Some((phase, bridge)) = &finite {
            let report = phase
                .solve(&query, finite_phase::Limits::default())
                .unwrap();
            let mut answers = vec![];
            let mut remaining = 20000;
            for solution in report.solutions {
                let (query, weight) = bridge.transport(solution);
                let mut caller = bridge.start(query, weight);
                loop {
                    assert!(remaining > 0, "finite caller service cutoff");
                    remaining -= 1;
                    match caller.step() {
                        finite_bridge::Event::Answer(a) => answers.push(a),
                        finite_bridge::Event::Exhausted => break,
                        finite_bridge::Event::Progress => (),
                    }
                }
            }
            emit_answers(index, true, answers);
            continue;
        }
        let mut engine = match mode {
            0..=3 => Engine::Compiled(
                compiled
                    .as_ref()
                    .unwrap()
                    .start_search(
                        query,
                        Policy::Global,
                        if mode % 2 == 0 {
                            Access::Scan
                        } else {
                            Access::Indexed
                        },
                    )
                    .unwrap(),
            ),
            4 => Engine::Contextual(contextual.as_ref().unwrap().start(&query)),
            5 => Engine::Contextual(contextual.as_ref().unwrap().start_shared_deductions(&query)),
            6 => Engine::Contextual(
                contextual
                    .as_ref()
                    .unwrap()
                    .start_persistent_equality(&query, false),
            ),
            7 => Engine::Contextual(
                contextual
                    .as_ref()
                    .unwrap()
                    .start_persistent_equality(&query, true),
            ),
            8 => Engine::Contextual(contextual.as_ref().unwrap().start_resumable(&query)),
            9 => Engine::Conditional(conditional.as_ref().unwrap().start(query).unwrap()),
            10 => {
                let shape: Vec<_> = query
                    .constraints
                    .iter()
                    .map(|c| (c.name.clone(), c.args.len()))
                    .collect();
                if !artifacts.contains_key(&shape) {
                    artifacts.insert(
                        shape.clone(),
                        prefix.as_ref().unwrap().prepare_shape(&shape).unwrap(),
                    );
                }
                Engine::Compiled(artifacts[&shape].start(&query, Access::Scan).unwrap())
            }
            _ => panic!("unknown mode"),
        };
        let mut answers = vec![];
        let mut exhausted = false;
        for _ in 0..20000 {
            match engine.step() {
                Event::Answer(a) => answers.push(a),
                Event::Exhausted => {
                    exhausted = true;
                    break;
                }
                Event::Progress => (),
            }
        }
        emit_answers(index, exhausted, answers);
    }
}
fn emit_answers(index: usize, exhausted: bool, answers: Vec<chr_syntax::Answer>) {
    println!("QUERY {index} {exhausted} {}", answers.len());
    for answer in answers {
        println!("ANSWER");
        println!(
            "{}",
            answer
                .outputs
                .iter()
                .map(|(_, t)| show(t))
                .collect::<Vec<_>>()
                .join(",")
        );
        for x in answer.residual {
            println!(
                "{}",
                std::iter::once(x.name)
                    .chain(x.args.iter().map(show))
                    .collect::<Vec<_>>()
                    .join(":")
            );
        }
    }
}

fn outputs(s: &str) -> Vec<(String, Var)> {
    if s == "-" {
        vec![]
    } else {
        s.split(',')
            .enumerate()
            .map(|(i, n)| (format!("o{i}"), Var(n.parse().unwrap())))
            .collect()
    }
}
