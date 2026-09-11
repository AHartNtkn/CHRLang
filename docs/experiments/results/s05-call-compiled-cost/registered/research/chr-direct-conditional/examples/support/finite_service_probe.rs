#[allow(dead_code)]
#[path = "../../tests/runtime_support/mod.rs"]
mod oracle;
#[allow(dead_code)]
#[path = "finite_cost_source.rs"]
mod source;
use chr_direct_conditional::engine::{Event, HeadAdmission, PreparedRuleset};
fn main() {
    let n: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    assert!(matches!(n, 4 | 6 | 8));
    let s = source::Schema {
        family: "all",
        work: 2,
        payload: 4,
        fail: false,
    };
    let rules = s.rules();
    let q = s.query(n, false);
    let mut e = PreparedRuleset::with_head_contract(rules.clone(), None, HeadAdmission::Optional)
        .unwrap()
        .start(q.clone())
        .unwrap();
    let mut stages = [0usize; 7];
    let mut answers = vec![];
    let mut done = false;
    for calls in 1..=32_000_000 {
        stages[e.allocation_stage()] += 1;
        let event = e.tick();
        match event {
            Event::Answer(a) => answers.push(a),
            Event::Exhausted => done = true,
            Event::Progress => (),
        }
        if done
            || matches!(
                calls,
                2_000_000 | 4_000_000 | 8_000_000 | 16_000_000 | 32_000_000
            )
        {
            println!(
                "depth={n} calls={calls} answers={} done={done} stages={stages:?} supports={} changes={} occurrences={}",
                answers.len(),
                e.supports().node_count(),
                e.store().changes().len(),
                e.resources().occurrences().len()
            );
        }
        if done {
            break;
        }
    }
    assert!(done, "diagnostic service cutoff");
    oracle::same_raw(answers, oracle::run(&rules, &q, 2_000_000));
    println!("independent_answers=pass");
}
