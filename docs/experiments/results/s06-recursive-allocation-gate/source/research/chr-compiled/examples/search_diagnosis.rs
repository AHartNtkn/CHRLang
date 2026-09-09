use chr_compiled::{
    Access, Policy, PreparedRuleset, SearchEvent, Stats, search_bundled, search_fixtures,
};
use chr_syntax::{Answer, Query, Var, atom, c, v};
use std::collections::BTreeSet;

#[derive(Default, Debug, PartialEq, Eq)]
struct Counts {
    steps: u64,
    splits: u64,
    failures: u64,
    answers: u64,
    peak: usize,
    first_failure_splits: Option<u64>,
    applications: u64,
    candidates: u64,
    observation: u64,
    max_terminal_terms: usize,
}
impl Counts {
    fn add(&mut self, s: &Stats) {
        self.applications += s.applications;
        self.candidates += s.candidate_visits;
        self.observation += s.observation_visits;
    }
}
#[allow(clippy::assertions_on_constants)]
fn main() {
    assert!(
        chr_compiled::COLLECT_METRICS,
        "diagnostic execution requires metrics"
    );
    let args: Vec<_> = std::env::args().collect();
    let family = args[1].as_str();
    let n: usize = args[2].parse().unwrap();
    assert!((4..=8).contains(&n));
    let mut forbidden = vec![];
    match family {
        "weak" => (),
        "chain" => {
            for x in 0..n - 1 {
                for a in 0..3 {
                    for b in 0..3 {
                        if a != b {
                            forbidden.push((x, x + 1, a, b));
                        }
                    }
                }
            }
        }
        "contradiction" => {
            for x in 0..4 {
                for y in x + 1..4 {
                    for a in 0..3 {
                        forbidden.push((x, y, a, a));
                    }
                }
            }
        }
        _ => panic!("unknown family"),
    }
    let residual = |row: &[usize]| {
        forbidden
            .iter()
            .map(|&(x, y, a, b)| {
                c(
                    "forbid",
                    [
                        atom(&format!("a{}", row[x])),
                        atom(&format!("a{}", row[y])),
                        atom(&format!("a{a}")),
                        atom(&format!("a{b}")),
                    ],
                )
            })
            .collect::<Vec<_>>()
    };
    let mut expected = BTreeSet::new();
    for mut code in 0..3usize.pow(n as u32) {
        let row: Vec<_> = (0..n)
            .map(|_| {
                let a = code % 3;
                code /= 3;
                a
            })
            .collect();
        if forbidden
            .iter()
            .all(|&(x, y, a, b)| row[x] != a || row[y] != b)
        {
            expected.insert(row);
        }
    }
    let mut constraints: Vec<_> = forbidden
        .iter()
        .map(|&(x, y, a, b)| {
            c(
                "forbid",
                [
                    v(x as u64),
                    v(y as u64),
                    atom(&format!("a{a}")),
                    atom(&format!("a{b}")),
                ],
            )
        })
        .collect();
    constraints.extend((0..n).map(|x| c("choose", [v(x as u64)])));
    let query = Query {
        constraints,
        outputs: (0..n).map(|x| (format!("o{x}"), Var(x as u64))).collect(),
    };
    let policy = match args[3].as_str() {
        "global" => Policy::Global,
        "active" => Policy::Active,
        _ => panic!(),
    };
    let access = match args[4].as_str() {
        "scan" => Access::Scan,
        "indexed" => Access::Indexed,
        _ => panic!(),
    };
    let code = match args[5].as_str() {
        "generic" => None,
        "generated" => Some(search_bundled(search_fixtures::FINITE_START + 3)),
        _ => panic!(),
    };
    let prepared = PreparedRuleset::new(search_fixtures::finite_rules(3), code).unwrap();
    let mut search = prepared.start_search(query, policy, access).unwrap();
    let mut counts = Counts {
        peak: 1,
        ..Default::default()
    };
    let mut actual = BTreeSet::new();
    loop {
        counts.steps += 1;
        assert!(counts.steps <= 1_000_000, "service cutoff");
        match search.tick() {
            SearchEvent::Split { work, .. } => {
                counts.splits += 1;
                counts.add(&work.unwrap());
            }
            SearchEvent::Failed(branch) => {
                counts.failures += 1;
                counts.first_failure_splits.get_or_insert(counts.splits);
                counts.max_terminal_terms = counts
                    .max_terminal_terms
                    .max(branch.engine.retention().term_nodes);
                counts.add(branch.engine.stats());
            }
            SearchEvent::Complete(mut branch) => {
                counts.answers += 1;
                let answer = branch.engine.observe().unwrap();
                let row: Vec<_> = answer
                    .outputs
                    .iter()
                    .enumerate()
                    .map(|(x, (name, t))| {
                        assert_eq!(name, &format!("o{x}"));
                        (0..3).find(|a| *t == atom(&format!("a{a}"))).unwrap()
                    })
                    .collect();
                assert_eq!(row.len(), n);
                let mut wanted = Answer {
                    outputs: answer.outputs.clone(),
                    residual: residual(&row),
                };
                wanted.residual.sort();
                let mut got = answer;
                got.residual.sort();
                assert_eq!(got, wanted);
                assert!(expected.contains(&row));
                assert!(actual.insert(row));
                counts.max_terminal_terms = counts
                    .max_terminal_terms
                    .max(branch.engine.retention().term_nodes);
                counts.add(branch.engine.stats());
            }
            SearchEvent::Exhausted => break,
            SearchEvent::Progress => (),
        }
        counts.peak = counts.peak.max(search.pending_branches());
    }
    assert_eq!(actual, expected);
    assert_eq!(counts.splits + 1, counts.failures + counts.answers);
    println!("{counts:?}");
}
