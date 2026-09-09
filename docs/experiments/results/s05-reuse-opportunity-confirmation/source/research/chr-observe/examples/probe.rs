use chr_observe::{Stats, equivalent};
use chr_reference::Search;
use chr_syntax::{Answer, Constraint, Goal, Query, Rule, and, c, or, v};
fn main() {
    println!(
        "family\tn\tequivalent\tterm_pairs\tscans\tcandidates\tbacktracks\tref_steps\tref_raw\tref_answers\tref_elapsed_us"
    );
    for family in ["labelled", "symmetric", "disconnected"] {
        let sizes: &[usize] = if family == "disconnected" {
            &[4, 6, 8]
        } else {
            &[0, 2, 4, 6, 8]
        };
        for &n in sizes {
            let build = |right: bool| -> Answer {
                let var = |i: usize| {
                    v(if right {
                        100 + (n - 1 - i) as u64
                    } else {
                        i as u64
                    })
                };
                let mut residual = (0..n)
                    .map(|i| match family {
                        "labelled" => c(&format!("p{i}"), [var(i)]),
                        "symmetric" => c("p", [var(i)]),
                        _ => {
                            let next = if right {
                                let base = (i / (n / 2)) * (n / 2);
                                base + (i + 1) % (n / 2)
                            } else {
                                (i + 1) % n
                            };
                            c("edge", [var(i), var(next)])
                        }
                    })
                    .collect::<Vec<Constraint>>();
                if right {
                    residual.reverse();
                }
                Answer {
                    outputs: vec![],
                    residual,
                }
            };
            let left = build(false);
            let right = build(true);
            let expected = family != "disconnected";
            let mut stats = Stats::default();
            let same = equivalent(&left, &right, &mut stats);
            assert_eq!(same, expected);
            let goals = |a: Answer| {
                and(a
                    .residual
                    .into_iter()
                    .map(Goal::Constraint)
                    .collect::<Vec<_>>())
            };
            let rules = vec![Rule::simplify(
                "start",
                [c("start", [])],
                or(goals(left), goals(right)),
            )];
            let started = std::time::Instant::now();
            let mut reference = Search::new(
                rules,
                Query {
                    constraints: vec![c("start", [])],
                    outputs: vec![],
                },
            )
            .unwrap();
            let result = reference.advance(1000);
            let elapsed = started.elapsed().as_micros();
            assert!(result.exhausted);
            assert_eq!(result.answers.len(), if expected { 1 } else { 2 });
            assert_eq!(reference.stats().completed_branches, 2);
            println!(
                "{family}\t{n}\t{same}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{elapsed}",
                stats.term_pairs,
                stats.occurrence_scans,
                stats.occurrence_candidates,
                stats.backtracks,
                reference.stats().steps,
                reference.stats().completed_branches,
                result.answers.len()
            );
        }
    }
}
