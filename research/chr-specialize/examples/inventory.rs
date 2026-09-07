fn main() {
    println!(
        "case\teligible\treason\tfinite_reference\tobservations_match\traw_match\texhaustion_match\texpansions\tcontradictions\tidentities\tgenerated_goals"
    );
    for case in chr_cases::registry() {
        match chr_specialize::specialize(&case.rules, &case.query, 8) {
            Err(reason) => println!(
                "{}\tfalse\t{reason}\t{}\tna\tna\tna\t0\t0\t0\t0",
                case.id, case.exhausted
            ),
            Ok(p) => {
                let mut search = chr_reference::Search::new(p.rules, p.query).unwrap();
                let mut answers = vec![];
                let mut exhausted = false;
                for _ in 0..case.budget {
                    let b = search.advance(1);
                    answers.extend(b.answers);
                    exhausted = b.exhausted;
                    if exhausted || case.answer_limit.is_some_and(|n| answers.len() >= n) {
                        break;
                    }
                }
                let equal = answers.len() == case.expected.len()
                    && case.expected.iter().all(|e| {
                        answers.iter().any(|a| {
                            chr_observe::equivalent(a, e, &mut chr_observe::Stats::default())
                        })
                    });
                let raw = search.stats().completed_branches == case.raw_answers;
                println!(
                    "{}\ttrue\tok\t{}\t{equal}\t{raw}\t{}\t{}\t{}\t{}\t{}",
                    case.id,
                    case.exhausted,
                    exhausted == case.exhausted,
                    p.stats.expansions,
                    p.stats.contradictions,
                    p.stats.identities,
                    p.stats.generated_goals
                );
                if case.exhausted {
                    assert!(equal && raw && exhausted, "{}", case.id);
                }
            }
        }
    }
}
