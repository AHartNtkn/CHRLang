use chr_compiled::{Access, Policy, PreparedRuleset, SearchEvent, search_bundled, search_fixtures};
use chr_syntax::{Query, Term, Var, atom, c, v};
use std::io::{self, BufRead};

fn term(s: &str) -> Term {
    match s.as_bytes()[0] {
        b'v' => v(s[1..].parse().unwrap()),
        b'a' => atom(s),
        _ => panic!("unsupported bridge term"),
    }
}

fn ground(t: &Term) -> u8 {
    match t {
        Term::App(name, children) if children.is_empty() => match name.as_str() {
            "a0" => 0,
            "a1" => 1,
            "a2" => 2,
            _ => panic!("unknown output atom"),
        },
        _ => panic!("nonground or structured answer outside fragment"),
    }
}

fn run(query: Query, prepared: &PreparedRuleset, policy: Policy, access: Access) {
    let mut search = prepared.start_search(query, policy, access).unwrap();
    let mut raw = 0;
    let mut splits = 0;
    let mut exhausted = false;
    let mut delivered = vec![];
    for _ in 0..1_000_000 {
        match search.tick() {
            SearchEvent::Complete(mut branch) => {
                raw += 1;
                delivered.push(branch.engine.observe().unwrap());
            }
            SearchEvent::Split { .. } => splits += 1,
            SearchEvent::Exhausted => {
                exhausted = true;
                break;
            }
            SearchEvent::Progress | SearchEvent::Failed(_) => (),
        }
    }
    let answers: Vec<String> = delivered
        .iter()
        .map(|answer| {
            let outputs: Vec<String> = answer
                .outputs
                .iter()
                .map(|(name, t)| format!("[{:?},{}]", name, ground(t)))
                .collect();
            let residual: Vec<String> = answer
                .residual
                .iter()
                .map(|constraint| {
                    assert_eq!(constraint.name, "forbid");
                    assert_eq!(constraint.args.len(), 4);
                    format!(
                        "{:?}",
                        constraint.args.iter().map(ground).collect::<Vec<_>>()
                    )
                })
                .collect();
            format!(
                "{{\"outputs\":[{}],\"residual\":[{}]}}",
                outputs.join(","),
                residual.join(",")
            )
        })
        .collect();
    println!(
        "{{\"metrics\":{},\"kernel_metrics\":{},\"raw\":{},\"splits\":{},\"exhausted\":{},\"answers\":[{}]}}",
        chr_compiled::COLLECT_METRICS,
        chr_persistent::COLLECT_KERNEL_METRICS,
        raw,
        splits,
        exhausted,
        answers.join(",")
    );
}

fn main() {
    let order: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let args: Vec<_> = std::env::args().collect();
    let generated = match args.get(2).map(String::as_str) {
        None | Some("generated") => true,
        Some("generic") => false,
        _ => panic!("invalid execution"),
    };
    let policy = match args.get(3).map(String::as_str) {
        None | Some("active") => Policy::Active,
        Some("global") => Policy::Global,
        _ => panic!("invalid policy"),
    };
    let access = match args.get(4).map(String::as_str) {
        None | Some("indexed") => Access::Indexed,
        Some("scan") => Access::Scan,
        _ => panic!("invalid access"),
    };
    let prepared = PreparedRuleset::new(
        search_fixtures::finite_rules(order),
        generated.then(|| search_bundled(search_fixtures::FINITE_START + order)),
    )
    .unwrap();
    let mut query = Query {
        constraints: vec![],
        outputs: vec![],
    };
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split_whitespace().collect();
        match fields[0] {
            "end" => {
                run(query, &prepared, policy, access);
                query = Query {
                    constraints: vec![],
                    outputs: vec![],
                };
            }
            "output" => {
                let Term::Var(Var(id)) = term(fields[2]) else {
                    panic!("variable output required")
                };
                query.outputs.push((fields[1].into(), Var(id)));
            }
            "choose" | "given" | "forbid" => query.constraints.push(c(
                fields[0],
                fields[1..].iter().map(|s| term(s)).collect::<Vec<_>>(),
            )),
            _ => panic!("unsupported bridge statement"),
        }
    }
    assert!(
        query.constraints.is_empty() && query.outputs.is_empty(),
        "unterminated query"
    );
}
