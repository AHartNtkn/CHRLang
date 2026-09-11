#[allow(dead_code)]
#[path = "../../chr-direct-conditional/tests/runtime_support/mod.rs"]
mod oracle;
#[path = "../experiments/reusable_workers.rs"]
mod workers;
use chr_syntax::{Rule, atom, c, eq, or, v};
fn main() {
    let rules = vec![Rule::simplify(
        "choose",
        [c("p", [v(0)])],
        or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
    )];
    let mut checked = 0;
    for count in [1, 2, 4] {
        let mut pool = workers::Pool::new(rules.clone(), count, 4).unwrap();
        for (generation, n) in [0, 1, 4, 2, 3].into_iter().enumerate() {
            let qs = (0..n)
                .map(|i| {
                    chr_cases::query(
                        vec![c("p", [v(0)]), c(&format!("tag{generation}_{i}"), [])],
                        &[0],
                    )
                })
                .collect::<Vec<_>>();
            let id = pool.begin(qs.clone()).unwrap();
            let mut done = vec![false; n];
            let mut answers = vec![Vec::new(); n];
            while done.iter().any(|x| !*x) {
                let mut outstanding = 0;
                for (region, is_done) in done.iter().enumerate() {
                    if !is_done {
                        pool.submit(&id, region, 1).unwrap();
                        outstanding += 1;
                    }
                }
                for _ in 0..outstanding {
                    let b = pool.receive(&id).unwrap();
                    assert!(!b.cancelled);
                    done[b.region] = b.exhausted;
                    answers[b.region].extend(b.answers);
                    if b.exhausted {
                        assert_eq!(b.raw_completions, 2);
                    }
                }
            }
            for (q, a) in qs.iter().zip(answers) {
                oracle::same_raw(a, oracle::run(&rules, q, 1000));
                checked += 1;
            }
            pool.end(&id).unwrap();
            let cancelled = pool.begin(qs).unwrap();
            for region in 0..n {
                pool.submit(&cancelled, region, 1).unwrap();
            }
            pool.end(&cancelled).unwrap();
        }
        pool.shutdown().unwrap();
    }
    println!(
        "{checked} independent source-query checks pass across persistent worker lifetimes; metrics={}",
        chr_factors::COLLECT_METRICS
    );
}
