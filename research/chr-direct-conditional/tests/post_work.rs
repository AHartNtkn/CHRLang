#![cfg(feature = "work-diagnostics")]
#[allow(dead_code)]
mod runtime_support;
#[path = "runtime_support/post_source.rs"]
mod source;
use chr_direct_choice::demand::{Event, Prepared};
#[test]
fn registered_post_work() {
    for kind in source::FAMILIES {
        for size in [8, 32] {
            for reverse in [false, true] {
                for mode in ["birth", "birth-miss", "birth-miss-template"] {
                    let mut p = Prepared::new(source::rules(kind)).unwrap();
                    if mode.contains("-miss") {
                        p = p.with_miss_reuse();
                    }
                    if mode.ends_with("-template") {
                        p = p.with_derivation_templates();
                    }
                    for query in 0..4 {
                        let value = if query % 2 == 0 { "a" } else { "b" };
                        let mut r = p.start(source::query(size, kind, reverse, value)).unwrap();
                        let mut answers = vec![];
                        let mut done = false;
                        let mut ticks = 0;
                        for tick in 1..=2_000_000 {
                            ticks = tick;
                            match r.tick() {
                                Event::Answer(a) => answers.push(a),
                                Event::Exhausted => {
                                    done = true;
                                    break;
                                }
                                Event::Progress => (),
                            }
                        }
                        assert!(done);
                        runtime_support::same_raw(answers, source::expected(size, kind, value));
                        let w = r.work();
                        assert_eq!(
                            w.resource_posts,
                            size as usize * if *kind == "post-duplicate" { 2 } else { 1 }
                        );
                        assert_eq!(
                            w.output_binders,
                            if ["post-input", "post-forward"].contains(kind) {
                                0
                            } else {
                                size as usize
                            }
                        );
                        if *kind == "post-template" && mode.ends_with("-template") {
                            assert!(w.template_hits >= size as usize - 1);
                        }
                        let g = r.retained_graph();
                        let templates = r.retained_derivation_templates();
                        println!(
                            "POST_WORK,{kind},{size},{reverse},{mode},{query},{ticks},{},{},{},{},{},{},{},{},{},{}",
                            w.force_entries,
                            w.match_entries,
                            w.validation_entries,
                            w.resource_candidates,
                            w.resource_posts,
                            w.output_binders,
                            w.template_hits,
                            g.nodes,
                            templates.0,
                            templates.1
                        );
                    }
                }
            }
        }
    }
}
