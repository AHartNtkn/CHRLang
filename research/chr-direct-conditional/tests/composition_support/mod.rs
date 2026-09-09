use chr_syntax::{Answer, Query, Rule};
pub enum Event {
    Progress,
    Answer(Answer),
    Exhausted,
}
// Test-only dispatch: retain each actual owner inline; this gate measures no costs.
#[allow(clippy::large_enum_variant)]
pub enum Engine {
    Compiled(chr_compiled::SearchEngine),
    Contextual(chr_relational::contextual_execute::Engine),
    Conditional(chr_direct_conditional::engine::Engine),
}
impl Engine {
    pub fn new(mode: usize, rules: &[Rule], query: &Query) -> Self {
        match mode {
            0 => Self::Compiled(
                chr_compiled::PreparedRuleset::new(rules.to_vec(), None)
                    .unwrap()
                    .start_search(
                        query.clone(),
                        chr_compiled::Policy::Global,
                        chr_compiled::Access::Scan,
                    )
                    .unwrap(),
            ),
            1..=4 => {
                let p = chr_relational::contextual_execute::Prepared::new(rules).unwrap();
                Self::Contextual(match mode {
                    1 => p.start(query),
                    2 => p.start_shared_deductions(query),
                    3 => p.start_persistent_equality(query, false),
                    _ => p.start_persistent_equality(query, true),
                })
            }
            5 => Self::Conditional(
                chr_direct_conditional::engine::PreparedRuleset::new(rules.to_vec())
                    .unwrap()
                    .start(query.clone())
                    .unwrap(),
            ),
            7 => Self::Contextual(
                chr_relational::contextual_execute::Prepared::new(rules)
                    .unwrap()
                    .start_resumable(query),
            ),
            _ => panic!("unknown candidate"),
        }
    }
    pub fn step(&mut self) -> Event {
        match self {
            Self::Compiled(e) => match e.tick() {
                chr_compiled::SearchEvent::Complete(mut b) => {
                    Event::Answer(b.engine.observe().unwrap())
                }
                chr_compiled::SearchEvent::Exhausted => Event::Exhausted,
                _ => Event::Progress,
            },
            Self::Contextual(e) => match e.advance() {
                chr_relational::contextual_execute::Step::Answer(a) => Event::Answer(a),
                chr_relational::contextual_execute::Step::Exhausted => Event::Exhausted,
                _ => Event::Progress,
            },
            Self::Conditional(e) => match e.tick() {
                chr_direct_conditional::engine::Event::Answer(a) => Event::Answer(a),
                chr_direct_conditional::engine::Event::Exhausted => Event::Exhausted,
                _ => Event::Progress,
            },
        }
    }
    pub fn collect(mut self) -> Vec<Answer> {
        let mut answers = vec![];
        for _ in 0..200000 {
            match self.step() {
                Event::Answer(a) => answers.push(a),
                Event::Exhausted => return answers,
                Event::Progress => (),
            }
        }
        panic!("finite composition cutoff");
    }
}
