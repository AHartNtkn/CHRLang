use chr_syntax::{Answer, Constraint, Goal, Guard, Term};
fn string(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
fn array(values: impl IntoIterator<Item = String>) -> String {
    format!("[{}]", values.into_iter().collect::<Vec<_>>().join(","))
}
fn term(t: &Term) -> String {
    match t {
        Term::Var(v) => v.0.to_string(),
        Term::App(n, args) => array([string(n), array(args.iter().map(term))]),
    }
}
fn constraint(c: &Constraint) -> String {
    array([string(&c.name), array(c.args.iter().map(term))])
}
fn goal(g: &Goal) -> String {
    match g {
        Goal::Constraint(c) => array([string("post"), constraint(c)]),
        Goal::Unify(a, b) => array([string("eq"), term(a), term(b)]),
        Goal::And(gs) => array(std::iter::once(string("and")).chain(gs.iter().map(goal))),
        Goal::Or(a, b) => array([string("or"), goal(a), goal(b)]),
        Goal::True => array([string("true")]),
        Goal::Fail => array([string("fail")]),
    }
}
fn answer(a: &Answer) -> String {
    format!(
        "{{\"outputs\":{},\"residual\":{}}}",
        array(a.outputs.iter().map(|(_, t)| term(t))),
        array(a.residual.iter().map(constraint))
    )
}
fn main() {
    for case in chr_cases::registry() {
        let mut reference =
            chr_reference::Search::new(case.rules.clone(), case.query.clone()).unwrap();
        let mut answers = vec![];
        for _ in 0..case.budget {
            let batch = reference.advance(1);
            answers.extend(batch.answers);
            if batch.exhausted || case.answer_limit.is_some_and(|n| answers.len() >= n) {
                break;
            }
        }
        let rules = array(case.rules.iter().map(|r| {
            format!(
                "{{\"kept\":{},\"removed\":{},\"body\":{},\"guards\":{}}}",
                array(r.kept.iter().map(constraint)),
                array(r.removed.iter().map(constraint)),
                goal(&r.body),
                array(
                    r.guards
                        .iter()
                        .map(|Guard::Equal(a, b)| array([term(a), term(b)]))
                )
            )
        }));
        println!(
            "{{\"id\":{},\"rules\":{},\"constraints\":{},\"outputs\":{},\"expected\":{},\"reference\":{},\"budget\":{},\"limit\":{},\"exhausted\":{},\"raw\":{}}}",
            string(&case.id),
            rules,
            array(case.query.constraints.iter().map(constraint)),
            array(case.query.outputs.iter().map(|(_, v)| v.0.to_string())),
            array(case.expected.iter().map(answer)),
            array(answers.iter().map(answer)),
            case.budget,
            case.answer_limit.map_or("null".into(), |n| n.to_string()),
            case.exhausted,
            case.raw_answers
        );
    }
}
