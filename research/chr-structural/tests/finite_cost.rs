#[path = "../examples/support/finite_oracle.rs"]
mod oracle;
#[path = "../examples/support/finite_cost.rs"]
mod runtime;
use runtime::{Event, FAMILIES, MODES, Prepared, Schema};
#[test]
fn all_five_paths_preserve_changed_queries_and_complete_owned_answers() {
    let mut checked = 0;
    for family in FAMILIES {
        for width in [0, 1, 4] {
            for mode in MODES {
                let p = Prepared::new(mode, Schema { width, family });
                for q in 0..4 {
                    let mut want = oracle::expected(p.schema, q);
                    want.sort();
                    let mut run = p.start(p.request(q));
                    let mut actual = vec![];
                    let mut done = false;
                    for _ in 0..100_000 {
                        match run.tick() {
                            Event::Progress => (),
                            Event::Answer(a) => {
                                assert_eq!(a.multiplicity, 1);
                                actual.push(a.term);
                            }
                            Event::Done => {
                                done = true;
                                break;
                            }
                        }
                    }
                    assert!(done);
                    actual.sort();
                    assert_eq!(actual, want, "{mode} {family} {width} {q}");
                    checked += 1;
                }
            }
        }
    }
    assert_eq!(checked, 360);
}
