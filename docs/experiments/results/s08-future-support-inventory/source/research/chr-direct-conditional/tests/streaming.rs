//! Each backend is checked against the source generator's closed-form stream.
//! Identical answers count separately; no cross-backend oracle or timing is used.
mod stream_support;
use chr_direct_conditional::engine::{Engine, Event, PreparedRuleset};
use chr_syntax::Answer;
use stream_support::{Extent, Family, Fixture, fixture, valid_answer};

// Semantic service cutoffs only. Larger prefix/lifetime experiments are separate.
const PREFIXES: [usize; 6] = [1, 2, 4, 8, 16, 32];
const MAX_PROGRESS_PER_ANSWER: usize = 2_000_000;
#[derive(Clone, Copy, Debug)]
enum Consumer {
    Drop,
    Retain,
}
enum Delivery {
    Progress,
    Answer(Answer),
    Exhausted,
}
trait Stream {
    fn tick(&mut self) -> Delivery;
}
impl Stream for Engine {
    fn tick(&mut self) -> Delivery {
        match Engine::tick(self) {
            Event::Progress => Delivery::Progress,
            Event::Answer(a) => Delivery::Answer(a),
            Event::Exhausted => Delivery::Exhausted,
        }
    }
}
fn conditional(fixture: &Fixture) -> Engine {
    PreparedRuleset::new(fixture.rules.clone())
        .unwrap()
        .start(fixture.query.clone())
        .unwrap()
}
fn next(stream: &mut impl Stream) -> Option<Answer> {
    for _ in 0..MAX_PROGRESS_PER_ANSWER {
        match stream.tick() {
            Delivery::Progress => (),
            Delivery::Answer(a) => return Some(a),
            Delivery::Exhausted => return None,
        }
    }
    panic!("stream failed to deliver or exhaust within semantic service bound");
}
fn consume(answer: Answer, family: Family, consumer: Consumer, retained: &mut Vec<Answer>) {
    assert!(
        valid_answer(family, &answer),
        "invalid complete answer: {answer:?}"
    );
    match consumer {
        Consumer::Drop => drop(answer),
        Consumer::Retain => retained.push(answer),
    }
}
fn unbounded(mut stream: impl Stream, fixture: &Fixture, consumer: Consumer) {
    assert_eq!(fixture.finite_answers, None);
    let mut retained = vec![];
    let mut delivered = 0;
    for target in PREFIXES {
        while delivered < target {
            let answer = next(&mut stream).expect("unbounded source exhausted prematurely");
            consume(answer, fixture.family, consumer, &mut retained);
            delivered += 1;
        }
        assert_eq!(delivered, target);
        assert_eq!(
            retained.len(),
            match consumer {
                Consumer::Drop => 0,
                Consumer::Retain => target,
            }
        );
    }
    // Cancel an ongoing source while retaining only consumer-owned answers.
    drop(stream);
    assert!(retained.iter().all(|a| valid_answer(fixture.family, a)));
}
fn finite(mut stream: impl Stream, fixture: &Fixture, consumer: Consumer) {
    let expected = fixture.finite_answers.unwrap();
    let mut delivered = 0;
    let mut retained = vec![];
    while let Some(answer) = next(&mut stream) {
        delivered += 1;
        assert!(
            delivered <= expected,
            "raw answer multiplicity exceeds source count"
        );
        consume(answer, fixture.family, consumer, &mut retained);
    }
    assert_eq!(delivered, expected, "finite source exhausted too early");
    assert!(matches!(stream.tick(), Delivery::Exhausted));
    assert_eq!(
        retained.len(),
        match consumer {
            Consumer::Drop => 0,
            Consumer::Retain => expected,
        }
    );
    drop(stream);
    assert!(retained.iter().all(|a| valid_answer(fixture.family, a)));
}
#[test]
fn conditional_unbounded_prefixes_preserve_duplicates_and_consumer_ownership() {
    for family in [Family::Ground, Family::Aliases] {
        assert_eq!(Family::parse(family.name()), Some(family));
        for consumer in [Consumer::Drop, Consumer::Retain] {
            let fixture = fixture(family, Extent::Unbounded);
            unbounded(conditional(&fixture), &fixture, consumer);
        }
    }
}
#[test]
fn conditional_finite_generator_has_exact_raw_count_and_exhaustion() {
    for family in [Family::Ground, Family::Aliases] {
        for n in [0, 1, 2, 8, 16] {
            for consumer in [Consumer::Drop, Consumer::Retain] {
                let fixture = fixture(family, Extent::Finite(n));
                finite(conditional(&fixture), &fixture, consumer);
            }
        }
    }
}
#[cfg(feature = "experiment")]
mod explicit {
    use super::*;
    use chr_compiled::{Access, Policy, SearchEngine, SearchEvent};
    impl Stream for SearchEngine {
        fn tick(&mut self) -> Delivery {
            match SearchEngine::tick(self) {
                SearchEvent::Complete(mut branch) => Delivery::Answer(
                    branch
                        .engine
                        .observe()
                        .expect("successful completed branch"),
                ),
                SearchEvent::Exhausted => Delivery::Exhausted,
                SearchEvent::Progress | SearchEvent::Split { .. } | SearchEvent::Failed(_) => {
                    Delivery::Progress
                }
            }
        }
    }
    fn specialized(fixture: &Fixture) -> SearchEngine {
        chr_compiled::PreparedRuleset::new(fixture.rules.clone(), None)
            .unwrap()
            .specialize_inferred()
            .start_search(fixture.query.clone(), Policy::Global, Access::Indexed)
            .unwrap()
    }
    #[test]
    fn specialized_unbounded_prefixes_preserve_duplicates_and_consumer_ownership() {
        for family in [Family::Ground, Family::Aliases] {
            for consumer in [Consumer::Drop, Consumer::Retain] {
                let fixture = fixture(family, Extent::Unbounded);
                unbounded(specialized(&fixture), &fixture, consumer);
            }
        }
    }
    #[test]
    fn specialized_finite_generator_has_exact_raw_count_and_exhaustion() {
        for family in [Family::Ground, Family::Aliases] {
            for n in [0, 1, 2, 8, 16] {
                for consumer in [Consumer::Drop, Consumer::Retain] {
                    let fixture = fixture(family, Extent::Finite(n));
                    finite(specialized(&fixture), &fixture, consumer);
                }
            }
        }
    }
}

#[test]
fn closed_form_checker_rejects_wrong_values_multiplicity_and_aliases() {
    use chr_syntax::{Var, atom, c, v};
    let ground = Answer {
        outputs: vec![],
        residual: vec![c("result", [atom("a")])],
    };
    assert!(valid_answer(Family::Ground, &ground));
    let mut wrong_atom = ground.clone();
    wrong_atom.residual[0].args[0] = atom("b");
    assert!(!valid_answer(Family::Ground, &wrong_atom));
    let mut extra = ground.clone();
    extra.residual.push(c("loop", []));
    assert!(!valid_answer(Family::Ground, &extra));
    let mut duplicate = ground.clone();
    duplicate.residual.push(ground.residual[0].clone());
    assert!(!valid_answer(Family::Ground, &duplicate));

    let aliases = Answer {
        outputs: vec![],
        residual: vec![c("result", [v(17)]), c("pair", [v(17), v(17)])],
    };
    assert!(valid_answer(Family::Aliases, &aliases));
    let mut reordered = aliases.clone();
    reordered.residual.reverse();
    assert!(valid_answer(Family::Aliases, &reordered));
    let mut split_alias = aliases.clone();
    split_alias.residual[1].args[1] = v(18);
    assert!(!valid_answer(Family::Aliases, &split_alias));
    let duplicate = Answer {
        outputs: vec![],
        residual: vec![aliases.residual[0].clone(), aliases.residual[0].clone()],
    };
    assert!(!valid_answer(Family::Aliases, &duplicate));
    let grounded = Answer {
        outputs: vec![],
        residual: vec![c("result", [atom("a")]), c("pair", [atom("a"), atom("a")])],
    };
    assert!(!valid_answer(Family::Aliases, &grounded));
    let mut unexpected_output = aliases;
    unexpected_output
        .outputs
        .push(("out".into(), chr_syntax::Term::Var(Var(17))));
    assert!(!valid_answer(Family::Aliases, &unexpected_output));
}
