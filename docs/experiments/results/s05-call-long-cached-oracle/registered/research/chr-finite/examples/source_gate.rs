use chr_finite::{
    cnf::{Cnf, Encoding},
    model::{Endpoint, Forbidden, Problem},
    native::Prepared as NativePrepared,
    z3,
};
use std::{
    collections::BTreeSet,
    io::{self, BufRead},
};
fn endpoint(s: &str) -> Endpoint {
    match s.as_bytes()[0] {
        b'v' => Endpoint::Var(s[1..].parse().unwrap()),
        b'a' => Endpoint::Value(s[1..].parse().unwrap()),
        _ => panic!("invalid endpoint"),
    }
}
fn atom(s: &str) -> u8 {
    let Endpoint::Value(a) = endpoint(s) else {
        panic!("atom required")
    };
    assert!(a < 3);
    a
}
#[derive(Default)]
struct Input {
    choose: Vec<Endpoint>,
    given: Vec<(Endpoint, u8)>,
    forbidden: Vec<Forbidden>,
    outputs: Vec<(String, usize)>,
}
fn run(input: Input, mode: &str) {
    let mut vars = BTreeSet::new();
    let mut covered = BTreeSet::new();
    for e in input
        .choose
        .iter()
        .chain(input.given.iter().map(|(e, _)| e))
    {
        match e {
            Endpoint::Var(v) => {
                vars.insert(*v);
                covered.insert(*v);
            }
            Endpoint::Value(a) => assert!(*a < 3),
        }
    }
    for f in &input.forbidden {
        for e in [f.left, f.right] {
            if let Endpoint::Var(v) = e {
                vars.insert(v);
            }
        }
    }
    for (_, v) in &input.outputs {
        vars.insert(*v);
    }
    assert!(vars.is_subset(&covered), "ineligible uncovered variable");
    let count = vars.last().map_or(0, |v| v + 1);
    assert!(
        count <= 8 && vars.len() == count,
        "gate variable bound or noncontiguous identifiers"
    );
    let problem = Problem::new(count, input.forbidden).unwrap();
    let mut answers = vec![];
    let mut assignments = vec![];
    match mode {
        "native" => {
            let prepared = NativePrepared::new(&problem);
            assignments.extend(prepared.start(&input.given).unwrap());
        }
        "support" | "conflict" => {
            let encoding = if mode == "support" {
                Encoding::Support
            } else {
                Encoding::Conflict
            };
            let mut prepared = z3::Prepared::new(&Cnf::encode(&problem, encoding)).unwrap();
            let mut query = prepared
                .start(&problem.domains(&input.given).unwrap())
                .unwrap();
            while let Some(a) = query.next_solution().unwrap() {
                assignments.push(a)
            }
        }
        _ => panic!("invalid backend"),
    }
    for row in &assignments {
        let value = |e| match e {
            Endpoint::Var(v) => row[v],
            Endpoint::Value(a) => a,
        };
        let outputs: Vec<_> = input
            .outputs
            .iter()
            .map(|(n, v)| format!("[{:?},{}]", n, row[*v]))
            .collect();
        let residual: Vec<_> = problem
            .forbidden()
            .iter()
            .map(|f| format!("{:?}", [value(f.left), value(f.right), f.a, f.b]))
            .collect();
        answers.push(format!(
            "{{\"outputs\":[{}],\"residual\":[{}]}}",
            outputs.join(","),
            residual.join(",")
        ));
    }
    println!(
        "{{\"raw\":{},\"exhausted\":true,\"answers\":[{}]}}",
        assignments.len(),
        answers.join(",")
    );
}
fn main() {
    let mode = std::env::args().nth(1).expect("backend required");
    let mut input = Input::default();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let f: Vec<_> = line.split_whitespace().collect();
        match f[0] {
            "end" => run(std::mem::take(&mut input), &mode),
            "choose" => input.choose.push(endpoint(f[1])),
            "given" => input.given.push((endpoint(f[1]), atom(f[2]))),
            "forbid" => input.forbidden.push(Forbidden {
                left: endpoint(f[1]),
                right: endpoint(f[2]),
                a: atom(f[3]),
                b: atom(f[4]),
            }),
            "output" => {
                let Endpoint::Var(v) = endpoint(f[2]) else {
                    panic!("output variable required")
                };
                assert!(!input.outputs.iter().any(|(n, _)| n == f[1]));
                input.outputs.push((f[1].into(), v));
            }
            _ => panic!("invalid statement"),
        }
    }
    assert!(
        input.choose.is_empty()
            && input.given.is_empty()
            && input.forbidden.is_empty()
            && input.outputs.is_empty(),
        "unterminated query"
    );
}
