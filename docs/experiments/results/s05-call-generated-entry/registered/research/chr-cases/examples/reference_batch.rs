use chr_reference::Search;
fn main() {
    let manifest = std::env::args().nth(1).as_deref() == Some("--manifest");
    let cases = chr_cases::registry();
    if manifest {
        for case in cases {
            println!("{case:#?}");
        }
        return;
    }
    println!(
        "case\tstatus\texhausted\tanswers\traw_answers\tsteps\tapplications\tintroductions\tequations\tunification_pairs\thead_candidates\tsplits\tfailed\tduplicates\tmax_frontier\tpending"
    );
    let mut failed = false;
    for case in cases {
        let mut search = Search::new(case.rules, case.query).unwrap();
        let mut actual = vec![];
        let mut exhausted = false;
        for _ in 0..case.budget {
            let batch = search.advance(1);
            actual.extend(batch.answers);
            exhausted = batch.exhausted;
            if exhausted || case.answer_limit.is_some_and(|n| actual.len() >= n) {
                break;
            }
        }
        actual.sort();
        let mut expected = case.expected;
        expected.sort();
        let s = search.stats();
        let pass = actual == expected
            && exhausted == case.exhausted
            && s.completed_branches == case.raw_answers;
        failed |= !pass;
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            case.id,
            if pass { "pass" } else { "mismatch" },
            exhausted,
            actual.len(),
            s.completed_branches,
            s.steps,
            s.rule_applications,
            s.introductions,
            s.equations,
            s.unification_pairs,
            s.head_candidates,
            s.splits,
            s.failed_branches,
            s.duplicate_answers,
            s.max_frontier,
            search.pending_alternatives()
        );
        if !pass {
            eprintln!("{} actual {actual:?}; expected {expected:?}", case.id);
        }
    }
    if failed {
        std::process::exit(1);
    }
}
