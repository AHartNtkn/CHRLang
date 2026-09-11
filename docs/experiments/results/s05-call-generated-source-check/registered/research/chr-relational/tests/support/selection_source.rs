use chr_syntax::{Answer, Constraint, Query, Rule, Term, Var, atom, c, eq, t, v};
use std::collections::BTreeMap;
pub fn rule() -> Rule {
    Rule::simplify(
        "take",
        [c("take", [t("f", [v(0)]), v(1)]), c("token", [])],
        eq(v(1), v(0)),
    )
}
pub fn query(family: &str, n: usize, value: &str) -> Query {
    assert!(n > 0);
    let mut nested = atom(value);
    for _ in 0..n {
        nested = t("f", [nested]);
    }
    let mut constraints = vec![];
    for i in 0..n {
        let input = match family {
            "flat" | "competition" => t("f", [atom(value)]),
            "sparse" => {
                if i + 1 == n {
                    t("f", [atom(value)])
                } else {
                    v((n + i) as u64)
                }
            }
            "chain" => {
                if i == 0 {
                    nested.clone()
                } else {
                    v((i - 1) as u64)
                }
            }
            _ => panic!("unknown selection source"),
        };
        constraints.push(c("take", [input, v(i as u64)]));
    }
    constraints
        .extend((0..if family == "competition" { n / 2 } else { n }).map(|_| c("token", [])));
    Query {
        constraints,
        outputs: (0..n).map(|i| (format!("out{i}"), Var(i as u64))).collect(),
    }
}
fn node(i: usize) -> Term {
    atom(&format!("n{i}"))
}
pub struct Encoded {
    pub query: Query,
    pub nodes: usize,
    pub outputs: Vec<(String, usize)>,
}
pub fn encode(q: &Query) -> Encoded {
    fn intern(
        t: &Term,
        vars: &mut BTreeMap<Var, usize>,
        facts: &mut Vec<Constraint>,
        nodes: &mut usize,
    ) -> usize {
        if let Term::Var(v) = t
            && let Some(i) = vars.get(v)
        {
            return *i;
        }
        let id = *nodes;
        *nodes += 1;
        facts.push(c("root", [node(id)]));
        match t {
            Term::Var(v) => {
                vars.insert(*v, id);
            }
            Term::App(name, args) => {
                assert!(matches!(
                    (name.as_str(), args.len()),
                    ("a" | "b", 0) | ("f", 1)
                ));
                let mut columns = vec![node(id)];
                for a in args {
                    columns.push(node(intern(a, vars, facts, nodes)));
                }
                facts.push(c(&format!("d_{name}"), columns));
            }
        }
        id
    }
    let mut facts = vec![c("epoch", [])];
    let mut nodes = 0;
    let mut vars = BTreeMap::new();
    let mut ticket = 0;
    for fact in &q.constraints {
        match (fact.name.as_str(), fact.args.as_slice()) {
            ("token", []) => facts.push(fact.clone()),
            ("take", [input, output]) => {
                let a = intern(input, &mut vars, &mut facts, &mut nodes);
                let b = intern(output, &mut vars, &mut facts, &mut nodes);
                facts.push(c(
                    "request",
                    [atom(&format!("q{ticket}")), node(a), node(b)],
                ));
                for old in 0..ticket {
                    facts.push(c(
                        "before",
                        [atom(&format!("q{old}")), atom(&format!("q{ticket}"))],
                    ));
                }
                ticket += 1;
            }
            _ => panic!("unsupported source constraint"),
        }
    }
    let outputs = q
        .outputs
        .iter()
        .map(|(label, v)| {
            (
                label.clone(),
                intern(&Term::Var(*v), &mut vars, &mut facts, &mut nodes),
            )
        })
        .collect();
    Encoded {
        query: Query {
            constraints: facts,
            outputs: vec![],
        },
        nodes,
        outputs,
    }
}
pub fn decode(answer: Answer, encoded: &Encoded) -> Answer {
    fn id(t: &Term) -> usize {
        match t {
            Term::App(s, a) if a.is_empty() => s.strip_prefix('n').unwrap().parse().unwrap(),
            _ => panic!("nonground node id"),
        }
    }
    let mut parents = vec![None; encoded.nodes];
    let mut descriptors = BTreeMap::new();
    let mut epoch = 0;
    for fact in &answer.residual {
        match fact.name.as_str() {
            "root" => {
                let i = id(&fact.args[0]);
                assert!(parents[i].replace(i).is_none());
            }
            "edge" => {
                let i = id(&fact.args[0]);
                assert!(parents[i].replace(id(&fact.args[1])).is_none());
            }
            "d_a" | "d_b" | "d_f" => {
                assert!(descriptors.insert(id(&fact.args[0]), fact).is_none());
            }
            "epoch" => epoch += 1,
            "before" | "below" | "request" | "token" => (),
            _ => panic!("unfinished encoded operation: {}", fact.name),
        }
    }
    assert_eq!(epoch, 1);
    fn root(mut i: usize, parents: &[Option<usize>]) -> usize {
        for _ in 0..parents.len() {
            let p = parents[i].unwrap();
            if i == p {
                return i;
            }
            i = p;
        }
        panic!("parent cycle")
    }
    for i in 0..parents.len() {
        root(i, &parents);
    }
    for fact in &answer.residual {
        let columns = match fact.name.as_str() {
            "d_a" | "d_b" | "d_f" | "below" => &fact.args[..],
            "request" => &fact.args[1..],
            _ => &[],
        };
        for term in columns {
            assert_eq!(
                id(term),
                root(id(term), &parents),
                "stale descriptor/request"
            );
        }
    }
    fn term(
        i: usize,
        parents: &[Option<usize>],
        ds: &BTreeMap<usize, &Constraint>,
        fuel: usize,
    ) -> Term {
        assert!(fuel > 0, "constructor cycle");
        let r = root(i, parents);
        match ds.get(&r) {
            None => v(r as u64),
            Some(d) => t(
                &d.name[2..],
                d.args[1..]
                    .iter()
                    .map(|a| term(id(a), parents, ds, fuel - 1))
                    .collect::<Vec<_>>(),
            ),
        }
    }
    // Hidden cycles must be checked even when no output reaches them.
    fn visit(
        i: usize,
        parents: &[Option<usize>],
        ds: &BTreeMap<usize, &Constraint>,
        colors: &mut [u8],
    ) {
        let r = root(i, parents);
        assert_ne!(colors[r], 1, "hidden constructor cycle");
        if colors[r] == 2 {
            return;
        }
        colors[r] = 1;
        if let Some(d) = ds.get(&r) {
            for a in &d.args[1..] {
                visit(id(a), parents, ds, colors);
            }
        }
        colors[r] = 2;
    }
    let mut colors = vec![0; parents.len()];
    for &i in descriptors.keys() {
        visit(i, &parents, &descriptors, &mut colors);
    }
    let outputs = encoded
        .outputs
        .iter()
        .map(|(name, i)| {
            (
                name.clone(),
                term(*i, &parents, &descriptors, parents.len() + 1),
            )
        })
        .collect();
    let residual = answer
        .residual
        .iter()
        .filter_map(|fact| match fact.name.as_str() {
            "token" => Some(fact.clone()),
            "request" => Some(c(
                "take",
                fact.args[1..]
                    .iter()
                    .map(|a| term(id(a), &parents, &descriptors, parents.len() + 1))
                    .collect::<Vec<_>>(),
            )),
            _ => None,
        })
        .collect();
    Answer { outputs, residual }
}
